from llama_index.core.query_pipeline import (QueryPipeline as QP, InputComponent, FnComponent)
from llama_index.core.llms import ChatResponse
from llama_index.llms.openai import OpenAI

from llama_index.core.prompts import PromptTemplate
from llama_index.core import VectorStoreIndex, load_index_from_storage, SQLDatabase
from llama_index.core.prompts.default_prompts import DEFAULT_TEXT_TO_SQL_PROMPT
from llama_index.core.bridge.pydantic import BaseModel, Field
from llama_index.core.objects import (
    SQLTableNodeMapping,
    ObjectIndex,
    SQLTableSchema,
)
from llama_index.core.retrievers import SQLRetriever
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference

import os
from pathlib import Path
from typing import List
from sqlalchemy import create_engine
from pyvis.network import Network

from app.config import OPENAPI_KEY, CLINICAL_TRIALS_TABLE_INFO_DIR, POSTGRES_ENGINE, EMBEDDING_MODEL_API, EMBEDDING_MODEL_NAME
from app.services.search_utility import setup_logger

logger = setup_logger('ClinicalTrialText2SQLEngine')

class TableInfo(BaseModel):
    """
    Information regarding a structured table.
    """
    table_name: str = Field(
        ..., description="table name (must be underscores and NO spaces)"
    )
    table_summary: str = Field(
        ..., description="short, concise summary/caption of the table"
    )


class ClinicalTrialText2SQLEngine:
    """
    This class implements the logic to convert the user prompt to sql query.
    Then it executes the sql query in the database and return the result.
    """
    def __init__(self, config):
        self.config = config

        self.llm = OpenAI(model="gpt-3.5-turbo", api_key=str(OPENAPI_KEY))
        self.engine = create_engine(str(POSTGRES_ENGINE))

        self.text2sql_prompt = DEFAULT_TEXT_TO_SQL_PROMPT.partial_format(dialect=self.engine.dialect.name)

        self.sql_database = SQLDatabase(self.engine)
        self.table_node_mapping = SQLTableNodeMapping(self.sql_database)
        self.sql_retriever = SQLRetriever(self.sql_database)

        self.table_schema_objs = [SQLTableSchema(table_name=t.table_name, context_str=t.table_summary) for t in self.get_all_table_info()]
        self.embed_model = TextEmbeddingsInference(base_url=EMBEDDING_MODEL_API, model_name=EMBEDDING_MODEL_NAME)

        self.obj_index = ObjectIndex.from_objects(objects=self.table_schema_objs, object_mapping=self.table_node_mapping, index_cls=VectorStoreIndex, embed_model=self.embed_model)
        self.obj_retriever = self.obj_index.as_retriever(similarity_top_k=3)

        self.response_synthesis_prompt = PromptTemplate(
            "Given an input question, synthesize a response from the query results.\n"
            "Query: {query_str}\n"
            "SQL: {sql_query}\n"
            "SQL Response: {context_str}\n"
            "Response: "
        )

        self.qp = self.build_query_pipeline()

    def _get_table_info_with_index(self, idx: int) -> str:
        results_gen = Path(CLINICAL_TRIALS_TABLE_INFO_DIR).glob(f"{idx}_*")
        results_list = list(results_gen)
        if len(results_list) == 0:
            return None
        elif len(results_list) == 1:
            path = results_list[0]
            return TableInfo.parse_file(path)
        else:
            raise ValueError(
                f"More than one file matching index: {list(results_gen)}"
            )

    def get_all_table_info(self):
        file_counts = len(os.listdir(CLINICAL_TRIALS_TABLE_INFO_DIR))
        table_infos = []

        for i in range(file_counts):
            table_info = self._get_table_info_with_index(i)
            table_infos.append(table_info)
        logger.debug(f"ClinicalTrialText2SQLEngine.get_all_table_info table_infos: {len(table_infos)}")
        return table_infos

    def get_table_context_str(self, table_schema_objs: List[SQLTableSchema]):
        """Get table context string."""
        context_strs = []
        for table_schema_obj in table_schema_objs:
            table_info = self.sql_database.get_single_table_info(table_schema_obj.table_name)
            if table_schema_obj.context_str:
                table_opt_context = " The table description is: "
                table_opt_context += table_schema_obj.context_str
                table_info += table_opt_context
            context_strs.append(table_info)
        return "\n\n".join(context_strs)

    def parse_response_to_sql(self, response: ChatResponse) -> str:
        """Parse response to SQL."""
        response = response.message.content
        sql_query_start = response.find("SQLQuery:")
        if sql_query_start != -1:
            response = response[sql_query_start:]
            # TODO: move to removeprefix after Python 3.9+
            if response.startswith("SQLQuery:"):
                response = response[len("SQLQuery:") :]
        sql_result_start = response.find("SQLResult:")
        if sql_result_start != -1:
            response = response[:sql_result_start]
        logger.debug(f"ClinicalTrialText2SQLEngine.parse_response_to_sql sql: {response}")
        return response.strip().strip("```").strip()

    def get_response_synthesis_prompt(self, query_str, sql_query, context_str) -> PromptTemplate:
        response_synthesis_prompt_str = (
            "Given an input question, synthesize a response from the query results.\n"
            "Query: {query_str}\n"
            "SQL: {sql_query}\n"
            "SQL Response: {context_str}\n"
            "Response: "
        )
        return PromptTemplate(response_synthesis_prompt_str)

    def build_query_pipeline(self):
        qp = QP(
        modules={
            "input": InputComponent(),
            "table_retriever": self.obj_retriever,
            "table_output_parser": FnComponent(fn=self.get_table_context_str),
            "text2sql_prompt": self.text2sql_prompt,
            "text2sql_llm": self.llm,
            "sql_output_parser": FnComponent(fn=self.parse_response_to_sql),
            "sql_retriever": self.sql_retriever,
            "response_synthesis_prompt": self.response_synthesis_prompt,
            "response_synthesis_llm": self.llm,
            },
        verbose=True,
        )

        qp.add_chain(["input", "table_retriever", "table_output_parser"])
        qp.add_link("input", "text2sql_prompt", dest_key="query_str")
        qp.add_link("table_output_parser", "text2sql_prompt", dest_key="schema")
        qp.add_chain(
            ["text2sql_prompt", "text2sql_llm", "sql_output_parser", "sql_retriever"]
        )
        qp.add_link(
            "sql_output_parser", "response_synthesis_prompt", dest_key="sql_query"
        )
        qp.add_link(
            "sql_retriever", "response_synthesis_prompt", dest_key="context_str"
        )
        qp.add_link("input", "response_synthesis_prompt", dest_key="query_str")
        qp.add_link("response_synthesis_prompt", "response_synthesis_llm")

        net = Network(notebook=True, cdn_resources="in_line", directed=True)
        net.from_nx(qp.dag)
        net.show("text2sql_dag.html")

        return qp

    def call_text2sql(self, search_text:str):
        try:
            logger.debug(f"ClinicalTrialText2SQLEngine.call_text2sql search_text: {search_text}")
            response = self.qp.run(query=search_text)
            logger.debug(f"ClinicalTrialText2SQLEngine.call_text2sql response: {str(response)}")

        except Exception as ex:
            logger.exception("ClinicalTrialText2SQLEngine.call_text2sql Exception -", exc_info = ex, stack_info=True)
            raise ex

        return {
            "result" : str(response)
        }

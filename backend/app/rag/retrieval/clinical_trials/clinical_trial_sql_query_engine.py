from llama_index.core.query_pipeline import (
    QueryPipeline as QP,
    InputComponent,
    FnComponent,
)
from llama_index.core import VectorStoreIndex, SQLDatabase
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

from app.config import (
    CLINICAL_TRIALS_TABLE_INFO_DIR,
    POSTGRES_ENGINE,
    EMBEDDING_MODEL_API,
    EMBEDDING_MODEL_NAME,
    CLINICAL_TRIAL_SQL_PROGRAM, 
    CLINICAL_TRIALS_RESPONSE_REFINEMENT_PROGRAM, 
    TOGETHER_KEY
    )

from app.services.search_utility import setup_logger

from app.dspy_integration.clinical_trials_response_refinement import ResponseSynthesizerModule
from app.dspy_integration.clinical_trials_sql import SQL_module
import dspy 
import re


logger = setup_logger("ClinicalTrialText2SQLEngine")


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

        self.nous =dspy.Together(model = "NousResearch/Nous-Hermes-llama-2-7b", api_key=str(TOGETHER_KEY))
        self.llm = dspy.Together(model =  "codellama/CodeLlama-13b-Instruct-hf", api_key=str(TOGETHER_KEY))
        dspy.settings.configure(lm = self.llm)
        
        self.sql_module = SQL_module()
        self.sql_module.load(CLINICAL_TRIAL_SQL_PROGRAM)
        self.response_synthesizer = ResponseSynthesizerModule()
        self.response_synthesizer.load(CLINICAL_TRIALS_RESPONSE_REFINEMENT_PROGRAM)
        self.engine = create_engine(str(POSTGRES_ENGINE))

        self.sql_database = SQLDatabase(self.engine)
        self.table_node_mapping = SQLTableNodeMapping(self.sql_database)
        self.sql_retriever = SQLRetriever(self.sql_database)

        self.table_schema_objs = [
            SQLTableSchema(table_name=t.table_name, context_str=t.table_summary)
            for t in self.get_all_table_info()
        ]
        self.embed_model = TextEmbeddingsInference(
            base_url=EMBEDDING_MODEL_API, model_name=EMBEDDING_MODEL_NAME
        )

        self.obj_index = ObjectIndex.from_objects(
            objects=self.table_schema_objs,
            object_mapping=self.table_node_mapping,
            index_cls=VectorStoreIndex,
            embed_model=self.embed_model,
        )
        self.obj_retriever = self.obj_index.as_retriever(similarity_top_k=3)
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
            raise ValueError(f"More than one file matching index: {list(results_gen)}")

    def get_all_table_info(self):
        file_counts = len(os.listdir(CLINICAL_TRIALS_TABLE_INFO_DIR))
        table_infos = []

        for i in range(file_counts):
            table_info = self._get_table_info_with_index(i)
            table_infos.append(table_info)
        logger.info(
            f"get_all_table_info table_infos: {len(table_infos)}"
        )
        return table_infos

    def get_table_context_str(self, table_schema_objs: List[SQLTableSchema]):
        """Get table context string."""
        context_strs = []
        for table_schema_obj in table_schema_objs:
            table_info = self.sql_database.get_single_table_info(
                table_schema_obj.table_name
            )
            if table_schema_obj.context_str:
                table_opt_context = " The table description is: "
                table_opt_context += table_schema_obj.context_str
                table_info += table_opt_context
            context_strs.append(table_info)
        return "\n\n".join(context_strs)


    def extract_sql(self, llm_response: str) -> str:
        # First try to extract SQL code blocks enclosed in triple backticks
        sql = re.search(r"```(?:sql\n)?(.*?)```", llm_response, re.DOTALL | re.IGNORECASE)
        if sql:
            extracted_sql = sql.group(1).strip()
            logger.info(f"Output from LLM: {llm_response} \nExtracted SQL: {extracted_sql}")
            return extracted_sql

        # If not found, try to extract a plain SQL query
        sql = re.search(r"(select.*?;)", llm_response, re.DOTALL | re.IGNORECASE)
        if sql:
            extracted_sql = sql.group(1).strip()
            logger.info(f"Output from LLM: {llm_response} \nExtracted SQL: {extracted_sql}")
            return extracted_sql
        return llm_response



    def get_sql_query(self, question, context): 
        sql_query = self.sql_module(question = question, context = context).answer
        return sql_query
    
    def get_synthesized_response(self, question, sql, database_output): 
        if len(database_output) > 0:
            database_output = database_output[0].text
        with dspy.context(lm=self.nous):
            response = self.response_synthesizer(question = question, sql = sql, database_output = database_output).answer
        return response

    def build_query_pipeline(self):
        qp = QP(
            modules={
                "input": InputComponent(),
                "table_retriever": self.obj_retriever,
                "table_output_parser": FnComponent(fn=self.get_table_context_str),
                "text2sql_llm": FnComponent(self.get_sql_query),
                "sql_output_parser": FnComponent(self.extract_sql),
                "sql_retriever": self.sql_retriever,
                "response_synthesis_llm": FnComponent(self.get_synthesized_response),
            },
            verbose=True,
        )

        qp.add_chain(["input", "table_retriever", "table_output_parser"])
        qp.add_link("input", "text2sql_llm", dest_key="question") # FIX
        qp.add_link("table_output_parser", "text2sql_llm", dest_key= "context") #FIX
        qp.add_chain(
            ["text2sql_llm", "sql_output_parser", "sql_retriever"]
        )
        qp.add_link("text2sql_llm", "sql_output_parser", dest_key = "llm_response")
        qp.add_link("input", "response_synthesis_llm", dest_key = "question")
        qp.add_link("text2sql_llm", "response_synthesis_llm", dest_key = "sql")
        qp.add_link("sql_retriever", "response_synthesis_llm", dest_key = "database_output")
        net = Network(notebook=True, cdn_resources="in_line", directed=True)
        net.from_nx(qp.dag)
        net.show("text2sql_dag.html")
        return qp


    async def call_text2sql(
        self,
        search_text:str
    ) -> dict[str, str]:
        try:
            logger.info(f"call_text2sql search_text: {search_text}") 
            response = self.qp.run(query=search_text)
            logger.info(f"call_text2sql response: {str(response)}")

        except Exception as ex:
            logger.exception("call_text2sql Exception -", exc_info = ex, stack_info=True)
            raise ex

        return {"result": str(response)}

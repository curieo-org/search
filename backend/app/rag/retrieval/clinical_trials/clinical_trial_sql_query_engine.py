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
from llama_index.core.retrievers import VectorIndexRetriever
from llama_index.core.retrievers import SQLRetriever
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.vector_stores.qdrant import QdrantVectorStore

import os
from pathlib import Path
from typing import List
from sqlalchemy import create_engine
from pyvis.network import Network
from qdrant_client import QdrantClient

from app.config import (
    CLINICAL_TRIALS_TABLE_INFO_DIR,
    POSTGRES_ENGINE,
    EMBEDDING_MODEL_API,
    EMBEDDING_MODEL_NAME,
    CLINICAL_TRIAL_SQL_PROGRAM, 
    CLINICAL_TRIALS_RESPONSE_REFINEMENT_PROGRAM, 
    TOGETHER_KEY, 
    SQL_GENERATION_MODEL, 
    CLINICAL_TRAIL_RESPONSE_SYNTHESIZER_MODEL,
    QDRANT_API_KEY,
    QDRANT_API_URL, 
    QDRANT_API_PORT,
    QDRANT_CLINICAL_TRIAL_COLLECTION_NAME,
    QDRANT_TOP_CLINICAL_TRAIL_K,
    QDRANT_CLINICAL_TRIAL_METADATA_FIELD_NAME,
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

        self.response_llm =dspy.Together(
            model = str(CLINICAL_TRAIL_RESPONSE_SYNTHESIZER_MODEL),
            api_key=str(TOGETHER_KEY),
            max_tokens = 500)
        self.sql_llm = dspy.Together(
            model = str(SQL_GENERATION_MODEL),
            api_key=str(TOGETHER_KEY),
            max_tokens = 500)
        dspy.settings.configure(lm = self.sql_llm)
        
        self.sql_module = SQL_module()
        self.sql_module.load(CLINICAL_TRIAL_SQL_PROGRAM)
        self.response_synthesizer = ResponseSynthesizerModule()
        self.response_synthesizer.load(CLINICAL_TRIALS_RESPONSE_REFINEMENT_PROGRAM)
        self.engine = create_engine(str(POSTGRES_ENGINE))

        self.sql_database = SQLDatabase(self.engine)
        self.table_node_mapping = SQLTableNodeMapping(self.sql_database)
        self.sql_retriever = SQLRetriever(self.sql_database)
        self.embed_model = TextEmbeddingsInference(
            base_url=EMBEDDING_MODEL_API, model_name=EMBEDDING_MODEL_NAME
        )

        self.client = QdrantClient(
            url=QDRANT_API_URL,
            port=QDRANT_API_PORT,
            api_key=str(QDRANT_API_KEY),
            https=False
            )
        
        self.vector_store = QdrantVectorStore(
            client=self.client,
            collection_name=QDRANT_CLINICAL_TRIAL_COLLECTION_NAME
            )
        
        self.retriever = VectorIndexRetriever(
            index=VectorStoreIndex.from_vector_store(vector_store=self.vector_store),
            similarity_top_k=int(QDRANT_TOP_CLINICAL_TRAIL_K),
            embed_model=TextEmbeddingsInference(base_url=EMBEDDING_MODEL_API, model_name="")
        )

        self.table_schema_objs = [
            SQLTableSchema(table_name=t.table_name, context_str=t.table_summary)
            for t in self.get_all_table_info()
        ]
        self.obj_index = ObjectIndex.from_objects(
            objects=self.table_schema_objs,
            object_mapping=self.table_node_mapping,
            index_cls=VectorStoreIndex,
            embed_model=self.embed_model,
        )
        self.obj_retriever = self.obj_index.as_retriever(similarity_top_k=1)
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

    def get_sql_query(self, question, context): 
        sql_query = self.sql_module(question = question, context = context).answer
        return sql_query
    
    def extract_sql(self, question:str, llm_response: str) -> str:
        # First try to extract SQL code blocks enclosed in triple backticks
        # old_pattern = r"```(?:sql\n)?(.*?)```|(select.*?;)|('*\n\n---\n\nQuestion:')"
        pattern = r'(?si)SELECT.*?;'
        sql_match = re.search(pattern, llm_response, re.DOTALL | re.IGNORECASE)

        if sql_match:
            extracted_sql = (sql_match.group(1) or sql_match.group(2)).strip()
            retrieved_sql = self.get_relevant_tiles(question, extracted_sql)

            logger.info(f"Output from LLM: {llm_response} \nExtracted SQL: {retrieved_sql}")
            return retrieved_sql
        else:
            # Handle the case where no SQL pattern is matched
            logger.info(f"No SQL pattern matched in LLM response: {llm_response}")
            # Return an appropriate response, such as an empty list or a message indicating no SQL was found
            return []
    
    def replace_title_value(self, sql_command: str, title_names: list[str]):
        titles_for_sql = '\', \''.join([title.replace("\'", "\"") for title in title_names])
        quoted_titles_for_sql = "'{}'".format(titles_for_sql)
        # Pattern to match the WHERE clause related to the title
        pattern = r"title\s*=\s*'.*?'"
        # Replacement pattern using IN clause
        replacement = f"title IN ({quoted_titles_for_sql})"
        # Perform the substitution
        updated_sql_command = re.sub(pattern, replacement, sql_command)
        return updated_sql_command
    
    def get_relevant_tiles(self, question, sql_query) -> list[str] :
        return self.replace_title_value(
            sql_query,
            [
                node.metadata.get(QDRANT_CLINICAL_TRIAL_METADATA_FIELD_NAME)
                for node in self.retriever.retrieve(question)
                ]
            )
    
    def retrieve_input_title_name(self, question, context): 
        sql_query = self.sql_module(question = question, context = context).answer
        return sql_query
    
    def get_synthesized_response(self, question, sql, database_output): 
        if len(database_output) > 0:
            database_output = database_output[0].text
        with dspy.context(lm=self.response_llm):
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
        qp.add_link("input", "text2sql_llm", dest_key="question") 
        qp.add_link("table_output_parser", "text2sql_llm", dest_key= "context")
        qp.add_link("input", "sql_output_parser", dest_key="question")
        qp.add_link("text2sql_llm", "sql_output_parser", dest_key = "llm_response")
        
        qp.add_chain(["sql_output_parser", "sql_retriever"])
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

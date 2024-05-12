import os
import re
from pathlib import Path

import dspy
from llama_index.core import SQLDatabase, VectorStoreIndex
from llama_index.core.bridge.pydantic import BaseModel, Field
from llama_index.core.indices.struct_store.sql_retriever import SQLRetriever
from llama_index.core.objects import ObjectIndex, SQLTableNodeMapping, SQLTableSchema
from llama_index.core.query_pipeline import FnComponent, InputComponent, QueryPipeline
from llama_index.core.retrievers import VectorIndexRetriever
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference
from llama_index.vector_stores.qdrant import QdrantVectorStore
from pyvis.network import Network
from qdrant_client import QdrantClient
from sqlalchemy import create_engine

from app.dspy_integration.clinical_trials_response_refinement import (
    ResponseSynthesizerModule,
)
from app.dspy_integration.clinical_trials_sql import SqlModule
from app.services.search_utility import setup_logger
from app.settings import Settings

logger = setup_logger("ClinicalTrialText2SQLEngine")


class TableInfo(BaseModel):
    """Information regarding a structured table."""

    table_name: str = Field(
        ...,
        description="table name (must be underscores and NO spaces)",
    )
    table_summary: str = Field(
        ...,
        description="short, concise summary/caption of the table",
    )


class ClinicalTrialText2SQLEngine:
    """Converts user prompt to sql query and executes against db, returns the result."""

    def __init__(self, settings: Settings):
        self.settings = settings

        together = settings.together
        self.response_llm = dspy.Together(
            model=together.model,
            api_key=together.api_key,
            max_tokens=500,
        )
        self.sql_llm = dspy.Together(
            model=settings.ai_models.sql_generation,
            api_key=together.api_key,
            max_tokens=500,
        )

        dspy.settings.configure(lm=self.sql_llm)

        dspy_settings = settings.dspy
        self.sql_module = SqlModule()
        self.sql_module.load(dspy_settings.clinical_trial_sql_program)

        self.response_synthesizer = ResponseSynthesizerModule()
        self.response_synthesizer.load(
            dspy_settings.clinical_trials_response_refinement_program,
        )

        self.engine = create_engine(settings.postgres_engine.get_secret_value())

        self.sql_database = SQLDatabase(self.engine)
        self.table_node_mapping = SQLTableNodeMapping(self.sql_database)
        self.sql_retriever = SQLRetriever(self.sql_database)

        self.embed_model = TextEmbeddingsInference(
            base_url=settings.embedding.api,
            model_name=settings.embedding.model,
        )

        self.client = QdrantClient(
            url=settings.qdrant.api_url,
            port=settings.qdrant.api_port,
            api_key=settings.qdrant.api_key.get_secret_value(),
            https=False,  # TODO: activate https in prod
        )

        self.vector_store = QdrantVectorStore(
            client=self.client,
            collection_name=settings.qdrant.collection_name,
        )

        self.retriever = VectorIndexRetriever(
            index=VectorStoreIndex.from_vector_store(vector_store=self.vector_store),
            similarity_top_k=settings.qdrant.top_k,
            embed_model=TextEmbeddingsInference(
                base_url=settings.qdrant.api_url,
                model_name="",  # TODO: is "" correct here?
            ),
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

    def _get_table_info_with_index(self, idx: int) -> TableInfo | None:
        results_gen = Path(self.settings.table_info_dir.clinical_trials).glob(
            f"{idx}_*",
        )
        results_list = list(results_gen)

        if not results_list:
            return None

        if len(results_list) == 1:
            path = results_list[0]
            with Path(path).open("r") as source:
                return TableInfo.parse_raw(source.read())
        else:
            raise ValueError(f"More than one file matching index: {list(results_gen)}")

    def get_all_table_info(self) -> list[TableInfo]:
        file_counts = len(os.listdir(self.settings.table_info_dir.clinical_trials))
        table_infos = []

        for i in range(file_counts):
            if table_info := self._get_table_info_with_index(i):
                table_infos.append(table_info)
        logger.info(f"get_all_table_info table_infos: {len(table_infos)}")
        return table_infos

    def get_table_context_str(self, table_schema_objs: list[SQLTableSchema]) -> str:
        """Get table context string."""
        context_strs = []
        for table_schema_obj in table_schema_objs:
            table_info = self.sql_database.get_single_table_info(
                table_schema_obj.table_name,
            )
            if table_schema_obj.context_str:
                table_opt_context = " The table description is: "
                table_opt_context += table_schema_obj.context_str
                table_info += table_opt_context
            context_strs.append(table_info)

        return "\n\n".join(context_strs)

    def get_sql_query(self, question, context) -> dspy.Prediction:
        return self.sql_module(question=question, context=context).answer

    def extract_sql(self, question: str, llm_response: str) -> str | None:
        # First try to extract SQL code blocks enclosed in triple backticks
        # Old pattern: r"```(?:sql\n)?(.*?)```|(select.*?;)|('*\n\n---\n\nQuestion:')"
        pattern = r"(?si)SELECT.*?;"
        sql_match = re.search(pattern, llm_response, re.DOTALL | re.IGNORECASE)

        if sql_match:
            extracted_sql = (sql_match.group(1) or sql_match.group(2)).strip()
            retrieved_sql = self.get_relevant_tiles(question, extracted_sql)

            logger.info(
                f"Output from LLM: {llm_response} \nExtracted SQL: {retrieved_sql}",
            )
            return retrieved_sql

        logger.info(f"No SQL pattern matched in LLM response: {llm_response}")
        return None

    @staticmethod
    def replace_title_value(sql_command: str, title_names: list[str]) -> str:
        titles_for_sql = "', '".join([title.replace("'", '"') for title in title_names])
        quoted_titles_for_sql = f"'{titles_for_sql}'"
        # Pattern to match the WHERE clause related to the title
        pattern = r"title\s*=\s*'.*?'"
        # Replacement pattern using IN clause
        replacement = f"title IN ({quoted_titles_for_sql})"
        # Perform the substitution
        return re.sub(pattern, replacement, sql_command)

    def get_relevant_tiles(self, question, sql_query) -> str:
        return self.replace_title_value(
            sql_query,
            [
                node.metadata.get(
                    self.settings.qdrant.clinical_trial_metadata_field_name,
                )
                for node in self.retriever.retrieve(question)
            ],
        )

    def retrieve_input_title_name(self, question, context) -> dspy.OutputField:
        return self.sql_module(question=question, context=context).answer

    def get_synthesized_response(
        self,
        question,
        sql,
        database_output,
    ) -> dspy.OutputField:
        if len(database_output) > 0:
            database_output = database_output[0].text
        with dspy.context(lm=self.response_llm):
            return self.response_synthesizer(
                question=question,
                sql=sql,
                database_output=database_output,
            ).answer

    def build_query_pipeline(self) -> QueryPipeline:
        qp = QueryPipeline(
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
        qp.add_link("table_output_parser", "text2sql_llm", dest_key="context")
        qp.add_link("input", "sql_output_parser", dest_key="question")
        qp.add_link("text2sql_llm", "sql_output_parser", dest_key="llm_response")

        qp.add_chain(["sql_output_parser", "sql_retriever"])
        qp.add_link("input", "response_synthesis_llm", dest_key="question")
        qp.add_link("text2sql_llm", "response_synthesis_llm", dest_key="sql")
        qp.add_link(
            "sql_retriever",
            "response_synthesis_llm",
            dest_key="database_output",
        )

        net = Network(notebook=True, cdn_resources="in_line", directed=True)
        net.from_nx(qp.dag)
        net.show("text2sql_dag.html")
        return qp

    async def call_text2sql(self, search_text: str) -> dict[str, str]:
        try:
            logger.info(f"call_text2sql search_text: {search_text}")
            response = self.qp.run(query=search_text)
            logger.info(f"call_text2sql response: {str(response)}")

        except Exception as ex:
            logger.exception("call_text2sql Exception -", exc_info=ex, stack_info=True)
            raise ex

        return {"result": str(response)}

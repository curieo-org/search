from llama_index.core.query_pipeline import (
    QueryPipeline as QP,
    InputComponent,
    FnComponent,
)
from llama_index.llms.openai import OpenAI

from llama_index.core.prompts import PromptTemplate, PromptType
from llama_index.core import VectorStoreIndex
from llama_index.legacy.query_engine.knowledge_graph_query_engine import (
    DEFAULT_NEBULAGRAPH_NL2CYPHER_PROMPT_TMPL,
)
from llama_index.core.objects import (
    SimpleObjectNodeMapping,
    ObjectIndex,
)
from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference

import os
import sentry_sdk
from pathlib import Path
from typing import List
from app.database.nebula_graph import NebulaGraph

from app.config import (
    OPENAI_API_KEY,
    DRUG_CHEMBL_TABLE_INFO_DIR,
    EMBEDDING_MODEL_API,
    EMBEDDING_MODEL_NAME,
)
from app.services.search_utility import setup_logger

logger = setup_logger("DrugChEMBLText2CypherEngine")


class DrugChEMBLText2CypherEngine:
    """
    This class implements the logic to convert the user prompt to cypher query.
    Then it executes the cypher query in the graph database and return the result.
    """

    def __init__(self, config):
        self.config = config
        self.graph_storage = NebulaGraph()

        self.llm = OpenAI(model="gpt-3.5-turbo", api_key=str(OPENAI_API_KEY))

        self.table_schema_objs = self.get_all_table_info()
        self.embed_model = TextEmbeddingsInference(
            base_url=EMBEDDING_MODEL_API, model_name=EMBEDDING_MODEL_NAME
        )
        self.table_node_mapping = SimpleObjectNodeMapping.from_objects(
            self.table_schema_objs
        )

        self.obj_index = ObjectIndex.from_objects(
            objects=self.table_schema_objs,
            object_mapping=self.table_node_mapping,
            index_cls=VectorStoreIndex,
            embed_model=self.embed_model,
        )
        self.obj_retriever = self.obj_index.as_retriever(similarity_top_k=3)

        self.cypher_query_retriever_prompt = PromptTemplate(
            DEFAULT_NEBULAGRAPH_NL2CYPHER_PROMPT_TMPL,
            prompt_type=PromptType.TEXT_TO_GRAPH_QUERY,
        )

        self.response_synthesis_prompt = PromptTemplate(
            "Given an input question, synthesize a response from the query results.\n"
            "Query: {query_str}\n"
            "Cypher Query: {cypher_query}\n"
            "Cypher Response: {context_str}\n"
            "Response: "
        )

        self.qp = self.build_query_pipeline()

    def execute_graph_query(self, queries):
        logger.info(
            f"DrugChEMBLText2CypherEngine.execute_graph_query queries: {queries}"
        )
        queries = str(queries).strip()
        query_list = []

        # find queries between ``` and ``` and remove them
        if queries.find("```") == -1:
            start_index = queries.find("MATCH")
            query_list.append(queries[start_index:])
        else:
            start_index = queries.find("```")
            while start_index != -1:
                end_index = queries.find("```", start_index + 3)
                if end_index == -1:
                    break

                query_list.append(queries[start_index + 3 : end_index])

                start_index = queries.find("```", end_index + 3)

        logger.info(
            f"DrugChEMBLText2CypherEngine.execute_graph_query query_list: {query_list}"
        )
        results = []

        for query in query_list:
            if query.strip() == "":
                continue

            query = query.strip() + ";"

            result_dict = self.graph_storage.execute_query(query)
            results.append(result_dict)

        logger.info(
            f"DrugChEMBLText2CypherEngine.execute_graph_query results: {results}"
        )

        return results

    def _get_table_info_with_index(self, idx: int) -> dict:
        results_gen = Path(DRUG_CHEMBL_TABLE_INFO_DIR).glob(f"{idx}_*")
        results_list = list(results_gen)

        if len(results_list) == 0:
            return None
        elif len(results_list) == 1:
            path = results_list[0]
            with open(path, "r") as f:
                table_info = f.read()
                table_dict = eval(table_info)
                return table_dict
        else:
            raise ValueError(f"More than one file matching index: {list(results_gen)}")

    def get_all_table_info(self):
        file_counts = len(os.listdir(DRUG_CHEMBL_TABLE_INFO_DIR))
        table_infos = []

        for i in range(file_counts):
            table_info = self._get_table_info_with_index(i)
            table_infos.append(table_info)

        logger.info(
            f"DrugChEMBLText2CypherEngine.get_all_table_info table_infos: {len(table_infos)}"
        )
        return table_infos

    def get_table_context_str(self, table_schema_objs: List[dict[str, str]]) -> str:
        """Get table context string."""
        context_strs = []

        for table_schema_obj in table_schema_objs:
            table_context = ""

            for key, value in table_schema_obj.items():
                table_context += f"{key}: {value}\n"

            context_strs.append(table_context)

        return "\n\n".join(context_strs)

    def get_response_synthesis_prompt(
        self, query_str, sql_query, context_str
    ) -> PromptTemplate:
        response_synthesis_prompt_str = (
            "Given an input question, synthesize a response from the query results.\n"
            "Query: {query_str}\n"
            "Cypher Query: {cypher_query}\n"
            "Cypher Response: {context_str}\n"
            "Response: "
        )
        return PromptTemplate(response_synthesis_prompt_str)

    def cypher_output_parser(self, response: list[dict[str, list]]) -> str:
        response_str = ""

        for record in response:
            record_in_list = []
            for key, value in record.items():
                record_in_list.append(str(key) + " : " + str(value))

            response_str += " ## ".join(record_in_list) + "\n"

        logger.info(
            f"DrugChEMBLText2CypherEngine.cypher_output_parser response_str: {response_str}"
        )
        return response_str

    def build_query_pipeline(self):
        qp = QP(
            modules={
                "input": InputComponent(),
                "table_retriever": self.obj_retriever,
                "table_output_parser": FnComponent(fn=self.get_table_context_str),
                "cypher_query_retriever_prompt": self.cypher_query_retriever_prompt,
                "cypher_query_retriever_llm": self.llm,
                "cypher_output_retriever": FnComponent(fn=self.execute_graph_query),
                "cypher_output_parser": FnComponent(fn=self.cypher_output_parser),
                "response_synthesis_prompt": self.response_synthesis_prompt,
                "response_synthesis_llm": self.llm,
            },
            verbose=True,
        )

        qp.add_chain(["input", "table_retriever", "table_output_parser"])

        qp.add_link("input", "cypher_query_retriever_prompt", dest_key="query_str")
        qp.add_link(
            "table_output_parser", "cypher_query_retriever_prompt", dest_key="schema"
        )

        qp.add_chain(["cypher_query_retriever_prompt", "cypher_query_retriever_llm"])
        qp.add_link("cypher_query_retriever_prompt", "cypher_query_retriever_llm")
        qp.add_link("cypher_query_retriever_llm", "cypher_output_retriever")

        qp.add_chain(["cypher_output_retriever", "cypher_output_parser"])

        qp.add_link("input", "response_synthesis_prompt", dest_key="query_str")
        qp.add_link(
            "cypher_query_retriever_llm",
            "response_synthesis_prompt",
            dest_key="cypher_query",
        )
        qp.add_link(
            "cypher_output_parser", "response_synthesis_prompt", dest_key="context_str"
        )

        qp.add_chain(["response_synthesis_prompt", "response_synthesis_llm"])
        qp.add_link("response_synthesis_prompt", "response_synthesis_llm")

        return qp

    async def call_text2cypher(self, search_text:str) -> str:
        try:
            logger.info(f"DrugChEMBLText2CypherEngine.call_text2cypher search_text: {search_text}")

            response = self.qp.run(query=search_text)

            logger.info(f"DrugChEMBLText2CypherEngine.call_text2cypher response: {str(response)}")

        except Exception as ex:
            logger.exception("DrugChEMBLText2CypherEngine.call_text2cypher Exception -", exc_info = ex, stack_info=True)
            
            raise ex

        return response

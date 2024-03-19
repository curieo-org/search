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
import pandas as pd

from app.config import OPENAPI_KEY, CLINICAL_TRIALS_TABLE_INFO_DIR, POSTGRES_ENGINE, EMBEDDING_MODEL_API, EMBEDDING_MODEL_NAME
from app.services.search_utility import setup_logger

logger = setup_logger('ClinicalTrialText2SQLEngine')


# setup another logger 
import logging

import json
import logging

# a set of standard LogRecord attributes to be used when building JSON logs
LOG_RECORD_ATTRIBUTES = {
    'args', 'asctime', 'created', 'exc_info', 'exc_text', 'filename',
    'funcName', 'levelname', 'levelno', 'lineno', 'module', 'msecs', 'message',
    'msg', 'name', 'pathname', 'process', 'processName', 'relativeCreated',
    'stack_info', 'thread', 'threadName'}


class JSONFormatter(logging.Formatter):
    """
    Simple logging formatter using JSON serialization.
    """
    def __init__(self, fields=None, always_extra=None, datefmt=None):
        """
        Args:
            fields (dict, optional): A dictionary of fields to use in the log.
                The keys in the dictionary are keys that will be used in the
                final log form, and its values are the names of the attributes
                from the log record to use as final log values. Defaults to
                None, which is interpreted as an empty dict.
            always_extra (dict, optional): A dictionary of additional static
                values written to the final log. Defaults to None, which is
                interpreted as an empty dict.
            datefmt (str, optional): strftime date format. For more details
                check logging.Formatter documentation. Defaults to None.
        """
        super().__init__(fmt=None, datefmt=datefmt, style='%')
        self.fields = fields or {}
        self._uses_time = "asctime" in self.fields.values()
        self.always_extra = always_extra or {}

    def usesTime(self):
        """
        Check if the format uses the creation time of the record. For more
        information about the method see logging.Formatter.
        """
        return self._uses_time

    def format(self, record):
        """
        Build a JSON serializable dict starting from `self.always_extra`,
        adding the data from the LogRecord specified in `self.fields`, and
        finally adding the record specific extra data.

        Args:
            record (logging.LogRecord): log record to be converted to string

        Returns:
            string: JSON serialized log record
        """
        # start with always_extra data to prevent overriding log record data
        data = self.always_extra.copy()

        # format non-serializable record values. For more details see method
        # logging.Formatter.format()
        record.message = record.getMessage()
        if self.usesTime():
            record.asctime = self.formatTime(record, self.datefmt)
        if record.exc_info:
            # Cache the traceback text to avoid converting it multiple times
            # (it's constant anyway)
            if not record.exc_text:
                record.exc_text = self.formatException(record.exc_info)
        record.stack_info = self.formatStack(record.stack_info)

        # extract wanted fields from log record
        for key, field in self.fields.items():
            value = record.__dict__.get(field, None)

            # use cached exception traceback
            if field == "exc_info":
                value = record.exc_text

            # skip record fields without data
            if value:
                data[key] = value

        # copy only LogRecord extra
        for field, value in record.__dict__.items():
            # skip all standard fields
            if field in LOG_RECORD_ATTRIBUTES:
                continue
            # skip all internal variables and names
            if field.startswith("_"):
                continue

            data[field] = value

        return json.dumps(data)
def log_clinical_data():
    logger = logging.getLogger("debug_file_clinical_trials")
    logger.setLevel(logging.DEBUG)
    # formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
    formatter = JSONFormatter(fields={"message": "message", "time": "asctime"} )
    handler = logging.FileHandler("debug_file_clinical_trials.json")
    handler.setFormatter(formatter)
    logger.addHandler(handler) 
    return logger

clinical_logger = log_clinical_data()

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
        self.query = ""

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
        self.logging_dict = {}
        self.qp = self.build_query_pipeline()

    def append_to_file(self, statement): 
        clinical_logger.debug(statement)    
        with open(f"./another_one.txt", "a") as f:
            f.write(str(statement) + ",\n")

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
        # self.append_to_file("********************************************")   
        # self.append_to_file("**************************** TABLE CONTEXT STRING ************************************")
        # self.append_to_file({"context_dict" : "\n\n".join(context_strs)})
        self.logging_dict['context_dict'] = "\n\n".join(context_strs)
        # self.append_to_file("********************************************")   
        return "\n\n".join(context_strs)

    def parse_response_to_sql(self, response: ChatResponse) -> str:
        """Parse response to SQL."""
        response = response.message.content
        # # self.append_to_file("********************************************")   
        # self.append_to_file("*********** SQL output response **************")
        # self.append_to_file({"SQL_OUTPUT":response})
        # self.append_to_file("********************************************")   
        self.logging_dict['SQL_OUTPUT'] = response
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
            # # self.append_to_file("********************************************")   
            # self.append_to_file({"search_text" : search_text})
            self.logging_dict['search_text'] = search_text
            # self.append_to_file("********************************************")   
            # logger.debug(f"ClinicalTrialText2SQLEngine.call_text2sql search_text: {search_text}")
            response = self.qp.run(query=search_text)
            # logger.debug(f"ClinicalTrialText2SQLEngine.call_text2sql response: {str(response)}")

        except Exception as ex:
            logger.exception("ClinicalTrialText2SQLEngine.call_text2sql Exception -", exc_info = ex, stack_info=True)
            raise ex
        self.append_to_file(self.logging_dict)
        return {
            "result" : str(response)
        }

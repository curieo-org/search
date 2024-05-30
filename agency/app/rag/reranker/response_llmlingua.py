import re
from typing import Any, Dict, List, Optional

import requests
from llama_index.core.bridge.pydantic import Field, PrivateAttr
from llama_index.core.callbacks import CBEventType, EventPayload
from llama_index.core.instrumentation import get_dispatcher
from llama_index.core.instrumentation.events.rerank import (
    ReRankEndEvent,
    ReRankStartEvent,
)
from llama_index.core.postprocessor.types import BaseNodePostprocessor
from llama_index.core.schema import MetadataMode, NodeWithScore, QueryBundle
from pydantic import SecretStr
from llmlingua import PromptCompressor

from app.settings import RerankingSettings
from app.rag.utils.models import RetrievedResult
from app.utils.logging import setup_logger

dispatcher = get_dispatcher(__name__)

TAG_RE = re.compile(r"<[^>]+>")

logger = setup_logger("LongLLMLinguaPostprocessor")

DEFAULT_INSTRUCTION_STR = "Given the context, please answer the final question"


class LongLLMLinguaPostprocessor(BaseNodePostprocessor):
    """Optimization of nodes.

    Compress using LongLLMLingua paper.

    """

    metadata_mode: MetadataMode = Field(
        default=MetadataMode.ALL, description="Metadata mode."
    )
    instruction_str: str = Field(
        default=DEFAULT_INSTRUCTION_STR, description="Instruction string."
    )
    target_token: int = Field(
        default=300, description="Target number of compressed tokens."
    )
    rank_method: str = Field(default="longllmlingua", description="Ranking method.")
    additional_compress_kwargs: Dict[str, Any] = Field(
        default_factory=dict, description="Additional compress kwargs."
    )
    model: str = Field(
        default="BAAI/bge-reranker-large",
        description="The model to use when calling AI API",
    )
    top_n: int = 2

    _llm_lingua: Any = PrivateAttr()

    @classmethod
    def from_settings(
        cls,
        *,
        settings: RerankingSettings,
    ) -> "LongLLMLinguaPostprocessor":
        return LongLLMLinguaPostprocessor.from_dict(settings.dict())

    def __init__(
        self,
        model_name: str = "microsoft/phi-2",
        device_map: str = "cpu",
        model_config: Optional[dict] = {},
        metadata_mode: MetadataMode = MetadataMode.ALL,
        instruction_str: str = DEFAULT_INSTRUCTION_STR,
        target_token: int = 300,
        rank_method: str = "longllmlingua",
    ):
        """LongLLMLingua Compressor for Node Context."""
        from llmlingua import PromptCompressor

        additional_compress_kwargs = {}

        self._llm_lingua = PromptCompressor(
            model_name=model_name,
            device_map=device_map,
            model_config=model_config,
            use_llmlingua2=True
        )
        super().__init__(
            metadata_mode=metadata_mode,
            instruction_str=instruction_str,
            target_token=target_token,
            rank_method=rank_method,
            additional_compress_kwargs=additional_compress_kwargs,
        )

    @classmethod
    def class_name(cls) -> str:
        return "LongLLMLinguaPostprocessor"

    def _postprocess_nodes(
        self,
        nodes: List[RetrievedResult],
        query_bundle: Optional[QueryBundle] = None,
    ) -> List[RetrievedResult]:
        """Optimize a node text given the query by shortening the node text."""
        if query_bundle is None:
            raise ValueError("Query bundle is required.")
        context_texts = [n.text for n in nodes]
        # split by "\n\n" (recommended by LongLLMLingua authors)
        new_context_texts = [
            c for context in context_texts for c in context.split("\n\n")
        ]

        # You can use it this way, although the question-aware fine-grained compression hasn't been enabled.
        compressed_prompt = self._llm_lingua.compress_prompt(
            new_context_texts,  # ! Replace the previous context_list
            instruction=self.instruction_str,
            question=query_bundle.query_str,
            # target_token=2000,
            target_token=self.target_token,
            rank_method=self.rank_method,
            **self.additional_compress_kwargs,
        )

        compressed_prompt_txt = compressed_prompt["compressed_prompt"]

        # separate out the question and instruction (appended to top and bottom)
        compressed_prompt_txt_list = compressed_prompt_txt.split("\n\n")
        compressed_prompt_txt_list = compressed_prompt_txt_list[1:-1]

        # return nodes for each list
        return [
            compressed_prompt_txt
        ]
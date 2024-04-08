import re
from typing import Any, List, Optional

import requests
from llama_index.core.bridge.pydantic import Field, PrivateAttr
from llama_index.core.callbacks import CBEventType, EventPayload
from llama_index.core.postprocessor.types import BaseNodePostprocessor
from llama_index.core.schema import NodeWithScore, QueryBundle

from app.services.search_utility import setup_logger
from app.settings import RerankSettings

TAG_RE = re.compile(r"<[^>]+>")

logger = setup_logger("TextEmbeddingInferenceRerankEngine")


class TextEmbeddingInferenceRerankEngine(BaseNodePostprocessor):
    """
    The class extends the BaseNodePostprocessor class, aimed at reranking nodes (elements) based on text embedding inference.
    This class is part of a larger framework, likely for processing and analyzing data within a specific domain,
    such as document retrieval or search engine optimization. Here's an overview of the class and its components:
    """

    model: str = Field(
        default="BAAI/bge-reranker-large",
        description="The model to use when calling AI API",
    )
    _session: Any = PrivateAttr()

    def __init__(
        self,
        settings: RerankSettings,
        top_n: int = 2,
        model: str = "BAAI/bge-reranker-large",
    ):
        super().__init__(top_n=top_n, model=model)
        self.api = settings.api
        self.top_count = settings.top_count
        self.model = model
        self._session = requests.Session()

    @classmethod
    def class_name(cls) -> str:
        return "TextEmbeddingInferenceRerankEngine"

    def _postprocess_nodes(
        self,
        nodes: List[NodeWithScore],
        query_bundle: Optional[QueryBundle] = None,
    ) -> List[NodeWithScore]:
        """
        This method takes a list of nodes (each represented by a NodeWithScore object) and an optional QueryBundle object.
        It performs the reranking operation by:
        -- Validating the input.
        -- Extracting text from each node, removing HTML tags.
        -- Sending a request to the specified AI API with the extracted texts and additional query information.
        -- Processing the API's response to update the nodes' scores based on reranking results.
        -- Returning the top N reranked nodes, according to the class's top_n attribute and possibly constrained by a global RERANK_TOP_COUNT.
        """
        if query_bundle is None:
            raise ValueError("Missing query bundle in extra info.")
        if len(nodes) == 0:
            return []

        with self.callback_manager.event(CBEventType.RERANKING) as event:
            logger.info(
                "TextEmbeddingInferenceRerankEngine.postprocess_nodes query: "
                + query_bundle.query_str
            )
            texts = [TAG_RE.sub("", node.get_content()) for node in nodes]
            results = self._session.post(  # type: ignore
                self.api,
                json={
                    "query": query_bundle.query_str,
                    "truncate": True,
                    "texts": texts,
                },
            ).json()

            if len(results) == 0:
                raise RuntimeError(results["detail"])

            new_nodes = []
            for result in results:
                new_node_with_score = NodeWithScore(
                    node=nodes[result["index"]], score=result["score"]
                )
                new_nodes.append(new_node_with_score)
            event.on_end(payload={EventPayload.NODES: new_nodes})

        return new_nodes[: self.top_count]

import re
from typing import Any, Optional

import requests
from llama_index.core.bridge.pydantic import Field, PrivateAttr
from llama_index.core.callbacks import CBEventType, EventPayload
from llama_index.core.postprocessor.types import BaseNodePostprocessor
from llama_index.core.schema import NodeWithScore, QueryBundle
from pydantic import SecretStr

from app.services.search_utility import setup_logger
from app.settings import RerankingSettings

TAG_RE = re.compile(r"<[^>]+>")

logger = setup_logger("TextEmbeddingInferenceRerankEngine")


class TextEmbeddingInferenceRerankEngine(BaseNodePostprocessor):
    """Text-embedding reranking post-processor."""

    model: str = Field(
        default="BAAI/bge-reranker-large",
        description="The model to use when calling AI API",
    )
    api: str
    auth_token: SecretStr
    top_count: int
    _session: Any = PrivateAttr()

    def __init__(
        self,
        *,
        api: str,
        auth_token: SecretStr,
        top_count: int,
        model: str,
        top_n: int = 2,
    ):
        super().__init__(
            top_n=top_n,
            model=model,
            api=api,
            auth_token=auth_token,
            top_count=top_count,
        )
        self.model = model
        self._session = requests.Session()

    @classmethod
    def from_settings(
        cls, *, settings: RerankingSettings,
    ) -> "TextEmbeddingInferenceRerankEngine":
        return cls(
            api=settings.api,
            auth_token=settings.auth_token,
            top_count=settings.top_count,
            model=settings.model,
        )

    @classmethod
    def class_name(cls) -> str:
        return "TextEmbeddingInferenceRerankEngine"

    def _postprocess_nodes(
        self,
        nodes: list[NodeWithScore],
        query_bundle: Optional[QueryBundle] = None,
    ) -> list[NodeWithScore]:
        """Post-processing reranking of response nodes.

        This method takes a list of nodes represented by a NodeWithScore object and an
        optional QueryBundle object.

        It performs the reranking operation by:
        -- Validating the input.
        -- Extracting text from each node, removing HTML tags.
        -- Sending a request to the specified AI API with the extracted texts and
           additional query information.
        -- Processing the APIs response to update the nodes' scores based on reranking
           results.
        -- Returning the top N reranked nodes, according to the class's top_n attribute
           and possibly constrained by top_count.
        """
        if query_bundle is None:
            raise ValueError("Missing query bundle in extra info.")
        if len(nodes) == 0:
            return []

        with self.callback_manager.event(CBEventType.RERANKING) as event:
            logger.info(
                "TextEmbeddingInferenceRerankEngine.postprocess_nodes query: "
                + query_bundle.query_str,
            )
            texts = [TAG_RE.sub("", node.get_content()) for node in nodes]
            results = self._session.post(  # type: ignore
                self.api,
                json={
                    "query": query_bundle.query_str,
                    "truncate": True,
                    "texts": texts,
                },
                headers={
                    "Authorization": f"Bearer {self.auth_token.get_secret_value()}",
                },
            ).json()

            if len(results) == 0:
                raise RuntimeError(results["detail"])

            new_nodes = []
            for result in results:
                new_node_with_score = NodeWithScore(
                    node=nodes[result["index"]], score=result["score"],
                )
                new_nodes.append(new_node_with_score)
            event.on_end(payload={EventPayload.NODES: new_nodes})

        return new_nodes[: self.top_count]

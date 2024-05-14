import re
from typing import Optional

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

from app.settings import RerankingSettings
from app.utils.logging import setup_logger

dispatcher = get_dispatcher(__name__)

TAG_RE = re.compile(r"<[^>]+>")

logger = setup_logger("TextEmbeddingRerankPostprocessor")


class TextEmbeddingRerankPostprocessor(BaseNodePostprocessor):
    """Text-embedding reranking post-processor."""

    model: str = Field(
        default="BAAI/bge-reranker-large",
        description="The model to use when calling AI API",
    )
    api: str
    auth_token: SecretStr
    top_count: int
    top_n: int = 2

    _session: requests.Session = PrivateAttr(default_factory=lambda: requests.Session())

    @classmethod
    def from_settings(
        cls,
        *,
        settings: RerankingSettings,
    ) -> "TextEmbeddingRerankPostprocessor":
        return TextEmbeddingRerankPostprocessor.from_dict(settings.dict())

    @classmethod
    def class_name(cls) -> str:
        return "TextEmbeddingRerankPostprocessor"

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
        dispatch_event = dispatcher.get_dispatch_event()
        dispatch_event(
            ReRankStartEvent(
                query=query_bundle,
                nodes=nodes,
                top_n=self.top_n,
                model_name=self.model,
            ),
        )

        if query_bundle is None:
            raise ValueError("Missing query bundle in extra info.")
        if len(nodes) == 0:
            return []

        with self.callback_manager.event(
            CBEventType.RERANKING,
            payload={
                EventPayload.NODES: nodes,
                EventPayload.MODEL_NAME: self.model,
                EventPayload.QUERY_STR: query_bundle.query_str,
                EventPayload.TOP_K: self.top_n,
            },
        ) as event:
            logger.info(
                "TextEmbeddingInferenceRerankEngine.postprocess_nodes query: "
                + query_bundle.query_str,
            )
            # TODO: Better than? [TAG_RE.sub("", node.get_content()) for node in nodes]
            texts = [
                node.node.get_content(metadata_mode=MetadataMode.EMBED)
                for node in nodes
            ]

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
                index = result["index"]
                node = nodes[index]
                node.score = float(result["score"])
                new_nodes.append(node)
            event.on_end(payload={EventPayload.NODES: new_nodes})

        dispatch_event(ReRankEndEvent(nodes=new_nodes))

        return new_nodes[: self.top_count]

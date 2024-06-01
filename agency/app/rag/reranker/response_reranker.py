import re

import requests
from llama_index.core.instrumentation import get_dispatcher
from llama_index.core.schema import QueryBundle
from pydantic import SecretStr

from app.rag.utils.models import RerankedResult, RetrievedResult
from app.settings import RerankingSettings
from app.utils.logging import setup_logger

dispatcher = get_dispatcher(__name__)

TAG_RE = re.compile(r"<[^>]+>")

logger = setup_logger("TextEmbeddingRerankPostprocessor")


class RerankEngine:
    """Text-embedding reranking post-processor."""

    api: str
    auth_token: SecretStr
    top_count: int
    top_n: int = 2

    _session: requests.Session = requests.Session()

    def __init__(
        self,
        settings: RerankingSettings,
    ):
        self.api = settings.api
        self.auth_token = settings.auth_token
        self.top_count = settings.top_count

    def rerank_nodes(
        self,
        query_bundle: QueryBundle,
        nodes: list[RetrievedResult],
    ) -> RerankedResult:
        """Post-processing reranking of response nodes.

        This method takes a list of nodes represented by a list of RetrievedResult objects and reranks them using the llmlingua model.

        Args:
            query_bundle (QueryBundle): QueryBundle object.
            nodes (list[RetrievedResult]): List of RetrievedResult objects.
            top_count (int): Number of top nodes to return.

        Returns:
            RerankedResult: RerankedResult object.
        """

        try:
            text_list = [node.text[:512] for node in nodes]

            result = self._session.post(
                self.api,
                json={
                    "query": query_bundle.query_str,
                    "target_token": 300,
                    "context_texts_list": text_list,
                },
                headers={
                    "Authorization": f"Bearer {self.auth_token.get_secret_value()}",
                },
            ).json()
            result = result["response"]

            reranked_result = RerankedResult(
                compressed_prompt=result["compressed_prompt"],
                reranked_sources=[
                    nodes[source_index].source for source_index in result["sources"]
                ],
            )

            return reranked_result

        except Exception as e:
            logger.error(f"Error in postprocess_nodes: {e}")
            raise e

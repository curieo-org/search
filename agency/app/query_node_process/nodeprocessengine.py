# ruff: noqa: ERA001, ARG002, D205
import asyncio

from llama_index.embeddings.text_embeddings_inference import TextEmbeddingsInference

from app.settings import Settings
from app.utils.custom_vectorstore import CurieoQueryBundle
from app.utils.logging import setup_logger
from app.utils.splade_embedding import SpladeEmbeddingsInference

logger = setup_logger("QueryProcessorEngine")


class QueryProcessorEngine:
    def __init__(self, settings: Settings):
        self.settings = settings
        self.embed_model = TextEmbeddingsInference(
            model_name="",
            base_url=self.settings.embedding.api_url,
            auth_token=self.settings.embedding.api_key.get_secret_value(),
            timeout=60,
            embed_batch_size=self.settings.embedding.batch_size,
        )

        self.splade_model = SpladeEmbeddingsInference(
            model_name="",
            base_url=self.settings.spladeembedding.api,
            auth_token=self.settings.spladeembedding.api_key.get_secret_value(),
            timeout=60,
            batch_size=self.settings.spladeembedding.batch_size,
        )

    def process_sparse_query_vectors(
        self,
        splade_embeddings: dict,
    ) -> tuple[list[list[int]], list[list[float]]]:
        try:
            indices = [
                [entry.get("index") for entry in sublist]
                for sublist in splade_embeddings
            ]
            vectors = [
                [entry.get("value") for entry in sublist]
                for sublist in splade_embeddings
            ]

            if len(indices) != len(vectors):
                logger.error(
                    "The length of indices and vectors \
                        are not equal in sparse_query_vectors"
                )
                return [], []
            return indices, vectors
        except Exception as e:
            logger.exception("failed to query vectors from the splade model", e)
            return [], []

    async def query_process(self, search_text: str) -> CurieoQueryBundle:
        logger.info(f"query_process. search_text: {search_text}")
        if not len(search_text):
            return None

        try:
            async with asyncio.TaskGroup() as tg:
                extract_dense_embedding_task = tg.create_task(
                    self.embed_model.aget_agg_embedding_from_queries(
                        queries=[search_text]
                    )
                )
                extract_sparse_embedding_task = tg.create_task(
                    self.splade_model.get_text_embedding_batch(texts=[search_text])
                )

            dense_embeddings = extract_dense_embedding_task.result()
            sparse_embeddings = self.process_sparse_query_vectors(
                splade_embeddings=extract_sparse_embedding_task.result()
            )

            return CurieoQueryBundle(
                query_str=search_text,
                embedding=dense_embeddings,
                sparse_embedding=sparse_embeddings,
            )

        except Exception as e:
            logger.exception(
                "Orchestrator.handle_pubmed_web_search failed -",
                exc_info=e,
                stack_info=True,
            )
            return None

import sentry_sdk

from app.grpc_types.agency_pb2 import (
    Double2D,
    Embeddings,
    EmbeddingsOutput,
    Int2D,
    PubmedResponse,
    SearchInput,
)
from app.grpc_types.agency_pb2_grpc import AgencyService
from app.pubmed_retrieval.clusterretrievalengine import ClusterRetrievalEngine
from app.pubmed_retrieval.parentretrievalengine import ParentRetrievalEngine
from app.query_node_process.nodeprocessengine import QueryProcessorEngine
from app.settings import app_settings
from app.utils.custom_vectorstore import CurieoQueryBundle
from app.utils.logging import setup_logger

pubmed_parent_engine = ParentRetrievalEngine(settings=app_settings)
pubmed_cluster_engine = ClusterRetrievalEngine(settings=app_settings)
embedding_query_engine = QueryProcessorEngine(settings=app_settings)

logger = setup_logger("Search_API")


def prepare_query_bundle(request: Embeddings) -> CurieoQueryBundle:
    sparse_indices = list(  # noqa
        list(value for value in index.values)  # noqa
        for index in request.sparse_indices
    )
    sparse_embedding = list(  # noqa
        list(value for value in vector.values)  # noqa
        for vector in request.sparse_embedding
    )
    dense_embedding = list(request.dense_embedding) if request.dense_embedding else []

    return CurieoQueryBundle(
        query_str="",
        embedding=dense_embedding,
        sparse_embedding=(sparse_indices, sparse_embedding),
    )


class Search(AgencyService):
    @staticmethod
    async def pubmed_parent_search(
        request: Embeddings,
        _target,
        _options=(),
        _channel_credentials=None,
        _call_credentials=None,
        _insecure=False,
        _compression=None,
        _wait_for_ready=None,
        _timeout=None,
        _metadata=None,
    ) -> PubmedResponse:
        if trace_transaction := sentry_sdk.Hub.current.scope.transaction:
            trace_transaction.set_tag("title", "pubmed_parent_search")

        logger.info(f"pubmed_parent_search. query: {request}")

        try:
            query = prepare_query_bundle(request)

            if pubmed_sources := await pubmed_parent_engine.retrieve_parent_nodes(
                query,
            ):
                logger.info(
                    f"pubmed_parent_search. result length: {len(pubmed_sources)}"
                )

                return PubmedResponse(
                    status=200,
                    sources=pubmed_sources,
                )

            logger.error("pubmed_parent_search. failed to retrieve search results")

            return PubmedResponse(status=500, sources=[])
        except Exception as e:
            logger.exception(e)

            return PubmedResponse(status=500, sources=[])

    @staticmethod
    async def pubmed_cluster_search(
        request: Embeddings,
        _target,
        _options=(),
        _channel_credentials=None,
        _call_credentials=None,
        _insecure=False,
        _compression=None,
        _wait_for_ready=None,
        _timeout=None,
        _metadata=None,
    ) -> PubmedResponse:
        if trace_transaction := sentry_sdk.Hub.current.scope.transaction:
            trace_transaction.set_tag("title", "pubmed_parent_search")

        logger.info(f"pubmed_cluster_search. query: {request}")
        try:
            query = prepare_query_bundle(request)

            if pubmed_sources := await pubmed_cluster_engine.retrieve_cluster_nodes(
                query,
            ):
                logger.info(
                    f"pubmed_cluster_search. result length: {len(pubmed_sources)}"
                )

                return PubmedResponse(
                    status=200,
                    sources=pubmed_sources,
                )

            logger.error("pubmed_cluster_search. failed to retrieve search results")

            return PubmedResponse(status=500, sources=[])
        except Exception as e:
            logger.exception(e)

            return PubmedResponse(status=500, sources=[])

    @staticmethod
    async def embeddings_compute(
        request: SearchInput,
        _target,
        _options=(),
        _channel_credentials=None,
        _call_credentials=None,
        _insecure=False,
        _compression=None,
        _wait_for_ready=None,
        _timeout=None,
        _metadata=None,
    ) -> EmbeddingsOutput:
        if trace_transaction := sentry_sdk.Hub.current.scope.transaction:
            trace_transaction.set_tag("title", "embeddings_compute")

        query = request.query.strip()

        logger.info(f"embeddings_compute. query: {query}")
        try:
            if embedding_result := await embedding_query_engine.query_process(
                query,
            ):
                logger.info(f"embeddings_compute. result: {embedding_result}")

                dense_embedding = embedding_result.embedding or []
                sparse_embedding = embedding_result.sparse_embedding[1] or []
                sparse_indices = embedding_result.sparse_embedding[0] or []

                return EmbeddingsOutput(
                    status=200,
                    embeddings=Embeddings(
                        dense_embedding=dense_embedding,
                        sparse_embedding=[
                            Double2D(values=vector) for vector in sparse_embedding
                        ],
                        sparse_indices=[
                            Int2D(values=indices) for indices in sparse_indices
                        ],
                    ),
                )

            logger.error("embeddings_compute. failed to retrieve search results")

            return EmbeddingsOutput(
                status=500,
                embeddings=Embeddings(
                    dense_embedding=[], sparse_embedding=[], sparse_indices=[]
                ),
            )
        except Exception as e:
            logger.exception(e)

            return EmbeddingsOutput(
                status=500,
                embeddings=Embeddings(
                    dense_embedding=[], sparse_embedding=[], sparse_indices=[]
                ),
            )

from dataclasses import dataclass
from typing import Any, cast

import llama_index.core.instrumentation as instrument
from llama_index.core import StorageContext, VectorStoreIndex
from llama_index.core.embeddings.utils import EmbedType
from llama_index.core.indices.vector_store.retrievers import VectorIndexRetriever
from llama_index.core.schema import BaseNode, NodeWithScore, QueryBundle
from llama_index.core.utils import iter_batch
from llama_index.core.vector_stores.types import (
    MetadataFilters,
    VectorStoreQueryMode,
    VectorStoreQueryResult,
)
from llama_index.vector_stores.qdrant import QdrantVectorStore
from llama_index.vector_stores.qdrant.utils import (
    HybridFusionCallable,
    relative_score_fusion,
)
from qdrant_client.http import models as rest

from app.utils.custom_basenode import CurieoBaseNode

dispatcher = instrument.get_dispatcher(__name__)


@dataclass
class CurieoQueryBundle(QueryBundle):
    sparse_embedding: tuple[list[list[int]], list[list[float]]] | None = None


class CurieoVectorStore(QdrantVectorStore):
    def __init__(self, collection_name: str, aclient: Any | None = None):
        super().__init__(
            collection_name=collection_name,
            aclient=aclient,
            hybrid_fusion_fn=cast(HybridFusionCallable, relative_score_fusion),
        )

    def node_process_to_metadata_dict(
        self, node: CurieoBaseNode, text_required: bool = True
    ) -> dict[str, Any]:
        """Common logic for saving Node data into metadata dict."""
        node_dict = node.dict()
        metadata: dict[str, Any] = node_dict.get("metadata", {})

        node_dict["embedding"] = None
        node_dict["sparse_embedding"] = None

        metadata["_node_type"] = node.class_name()
        if text_required:
            metadata["text"] = node.text
        metadata["doc_id"] = node.id_

        return metadata

    def _build_points(self, nodes: list[BaseNode]) -> tuple[list[Any], list[str]]:
        ids = []
        points = []
        for node_batch in iter_batch(nodes, self.batch_size):
            node_ids = []
            vectors: list[Any] = []
            payloads = []

            for _, node in enumerate(node_batch):
                assert isinstance(node, CurieoBaseNode)  # noqa
                node_ids.append(node.node_id)

                vectors.append(
                    {
                        "text-sparse": rest.SparseVector(
                            indices=node.get_sparse_embedding().get("indices", []),
                            values=node.get_sparse_embedding().get("vector", []),
                        ),
                        "text-dense": node.get_embedding(),
                    }
                )

                metadata = self.node_process_to_metadata_dict(node, text_required=True)
                payloads.append(metadata)

            points.extend(
                [
                    rest.PointStruct(id=node_id, payload=payload, vector=vector)
                    for node_id, payload, vector in zip(
                        node_ids, payloads, vectors, strict=False
                    )
                ]
            )

            ids.extend(node_ids)

        return points, ids

    async def aquery(self, query_bundle: CurieoQueryBundle) -> VectorStoreQueryResult:
        """Asynchronously query vector store.
        
        NOTE: this is not implemented for all vector stores. If not implemented,
        it will just call query synchronously.
        """
        dense_embedding = cast(list[float], query_bundle.query_embedding)
        sparse_indices, sparse_embedding = query_bundle.sparse_embedding

        response = await self._aclient.search_batch(
            collection_name=self.collection_name,
            requests=[
                rest.SearchRequest(
                    vector=rest.NamedVector(
                        name="text-dense",
                        vector=dense_embedding,
                    ),
                    limit=query_bundle.similarity_top_k,
                    filter=query_bundle.filters,
                    with_payload=True,
                ),
                rest.SearchRequest(
                    vector=rest.NamedSparseVector(
                        name="text-sparse",
                        vector=rest.SparseVector(
                            indices=sparse_indices[0],
                            values=sparse_embedding[0],
                        ),
                    ),
                    limit=query_bundle.sparse_top_k,
                    filter=query_bundle.filters,
                    with_payload=True,
                ),
            ],
        )

        # sanity check
        if len(response) != 2:
            raise ValueError(f"Expected 2 responses, got {len(response)}")

        # flatten the response
        return relative_score_fusion(
            self.parse_to_query_result(response[0]),
            self.parse_to_query_result(response[1]),
            # NOTE: only for hybrid search (0 for sparse search, 1 for dense search)
            alpha=query_bundle.alpha or 0.5,
            # NOTE: use hybrid_top_k if provided, otherwise use similarity_top_k
            top_k=query_bundle.hybrid_top_k or query_bundle.similarity_top_k,
        )


class CurieoVectorStoreQuery:
    def __init__(
        self,
        query_embedding: list[float] | None = None,
        sparse_embedding: list[float] | None = None,
        similarity_top_k: int = 1,
        doc_ids: list[str] | None = None,
        node_ids: list[str] | None = None,
        query_str: str | None = None,
        output_fields: list[str] | None = None,
        embedding_field: str | None = None,
        mode: VectorStoreQueryMode = VectorStoreQueryMode.HYBRID,
        alpha: float | None = None,
        filters: MetadataFilters | None = None,
        mmr_threshold: float | None = None,
        sparse_top_k: int | None = None,
        hybrid_top_k: int | None = None,
    ):
        self.query_embedding = query_embedding
        self.sparse_embedding = sparse_embedding
        self.similarity_top_k = similarity_top_k
        self.doc_ids = doc_ids
        self.node_ids = node_ids
        self.query_str = query_str
        self.output_fields = output_fields
        self.embedding_field = embedding_field
        self.mode = mode
        self.alpha = alpha
        self.filters = filters
        self.mmr_threshold = mmr_threshold
        self.sparse_top_k = sparse_top_k
        self.hybrid_top_k = hybrid_top_k


class CurieoVectorIndexRetriever(VectorIndexRetriever):
    def _build_vector_store_query(
        self, query_bundle: QueryBundle
    ) -> CurieoVectorStoreQuery:
        return CurieoVectorStoreQuery(
            query_embedding=query_bundle.embedding,
            sparse_embedding=query_bundle.sparse_embedding,
            similarity_top_k=self._similarity_top_k,
            node_ids=self._node_ids,
            doc_ids=self._doc_ids,
            query_str=query_bundle.query_str,
            mode=self._vector_store_query_mode,
            alpha=self._alpha,
            filters=self._filters,
            sparse_top_k=self._sparse_top_k,
        )

    @dispatcher.span
    async def _aretrieve(self, query_bundle: CurieoQueryBundle) -> list[NodeWithScore]:
        query = self._build_vector_store_query(query_bundle)
        query_result = await self._vector_store.aquery(query)
        return self._build_node_list_from_query_result(query_result)


class CurieoVectorStoreIndex(VectorStoreIndex):
    def __init__(
        self,
        embed_model: EmbedType | None = None,
        storage_context: StorageContext | None = None,
    ) -> None:
        """Initialize params."""
        self._use_async = True

        super().__init__(
            nodes=[],
            embed_model=embed_model,
            storage_context=storage_context,
        )

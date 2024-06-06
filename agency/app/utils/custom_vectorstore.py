from typing import Any, List, Optional
from dataclasses import dataclass

from llama_index.core.schema import (
    BaseNode,
    QueryBundle
)
from llama_index.core.utils import iter_batch
from llama_index.vector_stores.qdrant import QdrantVectorStore
from llama_index.core.indices.vector_store.retrievers import VectorIndexRetriever
from llama_index.core.vector_stores.types import (
    VectorStoreQueryMode,
    MetadataFilters
)
from qdrant_client.http import models as rest

from app.utils.custom_basenode import CurieoBaseNode


class CurieoVectorStore(QdrantVectorStore):
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


@dataclass
class CurieoVectorStoreQuery:
    """Vector store query."""

    query_embedding: Optional[List[float]] = None
    sparse_embedding = None
    similarity_top_k: int = 1
    doc_ids: Optional[List[str]] = None
    node_ids: Optional[List[str]] = None
    query_str: Optional[str] = None
    output_fields: Optional[List[str]] = None
    embedding_field: Optional[str] = None

    mode: VectorStoreQueryMode = VectorStoreQueryMode.DEFAULT

    # NOTE: only for hybrid search (0 for bm25, 1 for vector search)
    alpha: Optional[float] = None

    # metadata filters
    filters: Optional[MetadataFilters] = None

    # only for mmr
    mmr_threshold: Optional[float] = None

    # NOTE: currently only used by postgres hybrid search
    sparse_top_k: Optional[int] = None
    # NOTE: return top k results from hybrid search. similarity_top_k is used for dense search top k
    hybrid_top_k: Optional[int] = None


@dataclass
class CurieoQueryBundle(QueryBundle):
    sparse_embedding: Optional[List[float]] = None
    

class CurieoVectorIndexRetriever(VectorIndexRetriever):
    def _build_vector_store_query(
        self, query_bundle_with_embeddings: CurieoQueryBundle
    ) -> CurieoVectorStoreQuery:
        return CurieoVectorStoreQuery(
            query_embedding=query_bundle_with_embeddings.embedding,
            sparse_embedding=query_bundle_with_embeddings.sparse_embedding,
            similarity_top_k=self._similarity_top_k,
            node_ids=self._node_ids,
            doc_ids=self._doc_ids,
            query_str=query_bundle_with_embeddings.query_str,
            mode=self._vector_store_query_mode,
            alpha=self._alpha,
            filters=self._filters,
            sparse_top_k=self._sparse_top_k,
        )



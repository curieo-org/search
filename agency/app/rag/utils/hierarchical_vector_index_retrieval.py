"""Base vector store index query."""

from typing import Any, Dict, List

import llama_index.core.instrumentation as instrument
from llama_index.core.base.base_retriever import BaseRetriever
from llama_index.core.base.embeddings.base import BaseEmbedding
from llama_index.core.callbacks.base import CallbackManager
from llama_index.core.callbacks.schema import CBEventType, EventPayload
from llama_index.core.constants import DEFAULT_SIMILARITY_TOP_K
from llama_index.core.data_structs.data_structs import IndexDict
from llama_index.core.indices.utils import log_vector_store_query_result
from llama_index.core.indices.vector_store.base import VectorStoreIndex
from llama_index.core.instrumentation.events.retrieval import (
    RetrievalEndEvent,
    RetrievalStartEvent,
)
from llama_index.core.schema import NodeWithScore, ObjectType, QueryBundle, QueryType
from llama_index.core.vector_stores.types import (
    MetadataFilters,
    VectorStoreQuery,
    VectorStoreQueryMode,
    VectorStoreQueryResult,
)

dispatcher = instrument.get_dispatcher(__name__)


class HierarchialVectorIndexRetriever(BaseRetriever):
    """Vector index retriever.

    Args:
        index (VectorStoreIndex): vector store index.
        similarity_top_k (int): number of top k results to return.
        vector_store_query_mode (str): vector store query mode
            See reference for VectorStoreQueryMode for full list of supported modes.
        filters (Optional[MetadataFilters]): metadata filters, defaults to None
        alpha (float): weight for sparse/dense retrieval, only used for
            hybrid query mode.
        doc_ids (Optional[List[str]]): list of documents to constrain search.
        vector_store_kwargs (dict): Additional vector store specific kwargs to pass
            through to the vector store at query time.

    """

    def __init__(
        self,
        index: VectorStoreIndex,
        similarity_top_k: int = DEFAULT_SIMILARITY_TOP_K,
        vector_store_query_mode: VectorStoreQueryMode = VectorStoreQueryMode.DEFAULT,
        filters: MetadataFilters | None = None,
        alpha: float | None = None,
        node_ids: List[str] | None = None,
        doc_ids: List[str] | None = None,
        sparse_top_k: int | None = None,
        callback_manager: CallbackManager | None = None,
        object_map: dict | None = None,
        embed_model: BaseEmbedding | None = None,
        verbose: bool = False,
        **kwargs: Any,
    ) -> None:
        """Initialize params."""
        self._index = index
        self._vector_store = self._index.vector_store
        self._embed_model = embed_model or self._index._embed_model
        self._docstore = self._index.docstore

        self._similarity_top_k = similarity_top_k
        self._vector_store_query_mode = VectorStoreQueryMode(vector_store_query_mode)
        self._alpha = alpha
        self._node_ids = node_ids
        self._doc_ids = doc_ids
        self._filters = filters
        self._sparse_top_k = sparse_top_k
        self._kwargs: Dict[str, Any] = kwargs.get("vector_store_kwargs", {})

        callback_manager = callback_manager or CallbackManager()
        super().__init__(
            callback_manager=callback_manager,
            object_map=object_map,
            verbose=verbose,
        )

    @property
    def similarity_top_k(self) -> int:
        """Return similarity top k."""
        return self._similarity_top_k

    @similarity_top_k.setter
    def similarity_top_k(self, similarity_top_k: int) -> None:
        """Set similarity top k."""
        self._similarity_top_k = similarity_top_k

    @dispatcher.span
    def retrieve(
        self, str_or_query_bundle: QueryType, filters: MetadataFilters | None = None
    ) -> List[NodeWithScore]:
        """Retrieve nodes given query.

        Args:
            str_or_query_bundle (QueryType): Either a query string or
                a QueryBundle object.

        """
        dispatch_event = dispatcher.get_dispatch_event()

        self._check_callback_manager()
        dispatch_event(
            RetrievalStartEvent(
                str_or_query_bundle=str_or_query_bundle,
            )
        )
        if isinstance(str_or_query_bundle, str):
            query_bundle = QueryBundle(str_or_query_bundle)
        else:
            query_bundle = str_or_query_bundle
        with self.callback_manager.as_trace("query"):
            with self.callback_manager.event(
                CBEventType.RETRIEVE,
                payload={EventPayload.QUERY_STR: query_bundle.query_str},
            ) as retrieve_event:
                nodes = self._retrieve(query_bundle, filters)
                nodes = self._handle_recursive_retrieval(query_bundle, nodes)
                retrieve_event.on_end(
                    payload={EventPayload.NODES: nodes},
                )
        dispatch_event(
            RetrievalEndEvent(
                str_or_query_bundle=str_or_query_bundle,
                nodes=nodes,
            )
        )
        return nodes

    @dispatcher.span
    def _retrieve(
        self, query_bundle: QueryBundle, filters: MetadataFilters | None = None
    ) -> List[NodeWithScore]:
        if self._vector_store.is_embedding_query:
            if query_bundle.embedding is None and len(query_bundle.embedding_strs) > 0:
                query_bundle.embedding = (
                    self._embed_model.get_agg_embedding_from_queries(
                        query_bundle.embedding_strs
                    )
                )
        return self._get_nodes_with_embeddings(query_bundle, filters)

    def _build_vector_store_query(
        self,
        query_bundle_with_embeddings: QueryBundle,
        filters: MetadataFilters | None = None,
    ) -> VectorStoreQuery:
        return VectorStoreQuery(
            query_embedding=query_bundle_with_embeddings.embedding,
            similarity_top_k=self._similarity_top_k,
            node_ids=self._node_ids,
            doc_ids=self._doc_ids,
            query_str=query_bundle_with_embeddings.query_str,
            mode=self._vector_store_query_mode,
            alpha=self._alpha,
            filters=filters,
            sparse_top_k=self._sparse_top_k,
        )

    def _build_node_list_from_query_result(
        self, query_result: VectorStoreQueryResult
    ) -> List[NodeWithScore]:
        if query_result.nodes is None:
            # NOTE: vector store does not keep text and returns node indices.
            # Need to recover all nodes from docstore
            if query_result.ids is None:
                raise ValueError(
                    "Vector store query result should return at "
                    "least one of nodes or ids."
                )
            assert isinstance(self._index.index_struct, IndexDict)
            node_ids = [
                self._index.index_struct.nodes_dict[idx] for idx in query_result.ids
            ]
            nodes = self._docstore.get_nodes(node_ids)
            query_result.nodes = nodes
        else:
            # NOTE: vector store keeps text, returns nodes.
            # Only need to recover image or index nodes from docstore
            for i in range(len(query_result.nodes)):
                source_node = query_result.nodes[i].source_node
                if (not self._vector_store.stores_text) or (
                    source_node is not None and source_node.node_type != ObjectType.TEXT
                ):
                    node_id = query_result.nodes[i].node_id
                    if self._docstore.document_exists(node_id):
                        query_result.nodes[i] = self._docstore.get_node(node_id)  # type: ignore[index]

        log_vector_store_query_result(query_result)

        node_with_scores: List[NodeWithScore] = []
        for ind, node in enumerate(query_result.nodes):
            score: float | None = None
            if query_result.similarities is not None:
                score = query_result.similarities[ind]
            node_with_scores.append(NodeWithScore(node=node, score=score))

        return node_with_scores

    def _get_nodes_with_embeddings(
        self,
        query_bundle_with_embeddings: QueryBundle,
        filters: MetadataFilters | None = None,
    ) -> List[NodeWithScore]:
        query = self._build_vector_store_query(query_bundle_with_embeddings, filters)
        query_result = self._vector_store.query(query, **self._kwargs)
        return self._build_node_list_from_query_result(query_result)
from typing import Any

from llama_index.core.schema import BaseNode
from llama_index.core.utils import iter_batch
from llama_index.vector_stores.qdrant import QdrantVectorStore
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
                assert isinstance(node, CurieoBaseNode)
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

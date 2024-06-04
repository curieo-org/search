import json
import time
from enum import Enum, auto
from typing import Any

from llama_index.core.bridge.pydantic import BaseModel
from llama_index.core.schema import (
    BaseNode,
    RelatedNodeInfo,
    TextNode,
)

DEFAULT_TEXT_NODE_TMPL = "{metadata_str}\n\n{content}"
DEFAULT_METADATA_TMPL = "{key}: {value}"
# NOTE: for pretty printing
TRUNCATE_LENGTH = 350
WRAP_WIDTH = 70

RelatedNodeType = RelatedNodeInfo | list[RelatedNodeInfo]


class MetadataMode(str, Enum):
    ALL = "all"
    EMBED = "embed"
    LLM = "llm"
    NONE = "none"


class ObjectType(str, Enum):
    TEXT = auto()
    IMAGE = auto()
    INDEX = auto()
    DOCUMENT = auto()
    CURIEO_NODE = auto()


class CurieoBaseNode(TextNode):
    sparse_embedding: Any = None

    obj: Any = None

    def dict(self, **kwargs: Any) -> dict[str, Any]:
        from llama_index.core.storage.docstore.utils import doc_to_json

        data = super().dict(**kwargs)

        try:
            if self.obj is None:
                data["obj"] = None
            elif isinstance(self.obj, BaseNode):
                data["obj"] = doc_to_json(self.obj)
            elif isinstance(self.obj, BaseModel):
                data["obj"] = self.obj.dict()
            else:
                data["obj"] = json.dumps(self.obj)
        except Exception:
            raise ValueError("CurieoBaseNode obj is not serializable: " + str(self.obj))

        return data

    @classmethod
    def from_text_node(
        cls,
        node: TextNode,
        node_id_generate: bool = False,
        sparse_embedding: dict = {},
    ) -> "CurieoBaseNode":
        if node_id_generate:
            node.id_ = str(int(time.time() * 1000)) + "_" + node.id_
        return cls(
            **node.dict(),
            sparse_embedding=sparse_embedding,
        )

    @classmethod
    def get_type(cls) -> str:
        return ObjectType.CURIEO_NODE

    @classmethod
    def class_name(cls) -> str:
        return "CURIEO_NODE"

    def set_metadata(self, _key: str, value: Any) -> None:
        """Set the content of the node."""
        self.text = value

    def get_sparse_embedding(self) -> list[float]:
        """Get sparse embedding.

        Errors if embedding is None.

        """
        if self.sparse_embedding is None:
            raise ValueError("Sparse embedding not set.")
        return self.sparse_embedding

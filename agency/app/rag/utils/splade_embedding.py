from collections.abc import Callable
from typing import Any

import llama_index.core.instrumentation as instrument
from llama_index.core.base.embeddings.base import (
    DEFAULT_EMBED_BATCH_SIZE,
    BaseEmbedding,
    Embedding,
)
from llama_index.core.bridge.pydantic import Field
from llama_index.core.callbacks import CallbackManager
from llama_index.core.callbacks.schema import CBEventType, EventPayload
from llama_index.core.instrumentation.events.embedding import EmbeddingStartEvent
from llama_index.core.utils import get_tqdm_iterable
from llama_index.embeddings.huggingface.utils import format_text

dispatcher = instrument.get_dispatcher(__name__)

DEFAULT_URL = "http://127.0.0.1:8080"


class SpladeEmbeddingsInference(BaseEmbedding):
    base_url: str = Field(
        default=DEFAULT_URL,
        description="Base URL for the splade embeddings service.",
    )
    query_instruction: str | None = Field(
        description="Instruction to prepend to query text."
    )
    text_instruction: str | None = Field(description="Instruction to prepend to text.")
    timeout: float = Field(
        default=60.0,
        description="Timeout in seconds for the request.",
    )
    truncate_text: bool = Field(
        default=True,
        description="Whether to truncate text or not when generating embeddings.",
    )
    auth_token: str | Callable[[str], str] | None = Field(
        default=None,
        description="Authentication token or authentication \
            token generating function for authenticated requests",
    )

    def __init__(
        self,
        model_name: str,
        base_url: str = DEFAULT_URL,
        text_instruction: str | None = None,
        query_instruction: str | None = None,
        batch_size: int = DEFAULT_EMBED_BATCH_SIZE,
        timeout: float = 60.0,
        truncate_text: bool = True,
        callback_manager: CallbackManager | None = None,
        auth_token: str | Callable[[str], str] | None = None,
    ):
        super().__init__(
            base_url=base_url,
            model_name=model_name,
            text_instruction=text_instruction,
            query_instruction=query_instruction,
            batch_size=batch_size,
            timeout=timeout,
            truncate_text=truncate_text,
            callback_manager=callback_manager,
            auth_token=auth_token,
        )

    @classmethod
    def class_name(cls) -> str:
        return "SpladeEmbeddingsInference"

    def _call_api(self, texts: list[str]) -> list[list[float]]:
        import httpx

        headers = {"Content-Type": "application/json"}
        if self.auth_token is not None:
            if callable(self.auth_token):
                headers["Authorization"] = self.auth_token(self.base_url)
            else:
                headers["Authorization"] = self.auth_token
        json_data = {"inputs": texts, "truncate": self.truncate_text}

        with httpx.Client() as client:
            response = client.post(
                f"{self.base_url}/embed_sparse",
                headers=headers,
                json=json_data,
                timeout=self.timeout,
            )

        return response.json()

    def _get_text_embedding(self, _text: str):
        return []

    def _get_text_embeddings(self, texts: list[str]):
        """Get text embeddings."""
        texts = [
            format_text(text, self.model_name, self.text_instruction) for text in texts
        ]
        return self._call_api(texts)

    def _get_query_embedding(self, _query: str) -> list[float]:
        return []

    async def _aget_query_embedding(self, _query: str) -> list[float]:
        return []

    def get_text_embedding_batch(
        self,
        texts: list[str],
        show_progress: bool = False,
        **_kwargs: Any,
    ) -> list[Embedding]:
        """Get a list of text embeddings, with batching."""
        cur_batch: list[str] = []
        result_embeddings: list[Embedding] = []

        queue_with_progress = enumerate(
            get_tqdm_iterable(texts, show_progress, "Splade Embeddings")
        )

        for idx, text in queue_with_progress:
            cur_batch.append(text)
            if idx == len(texts) - 1 or len(cur_batch) == self.batch_size:
                # flush
                dispatcher.event(
                    EmbeddingStartEvent(
                        model_dict=self.to_dict(),
                    )
                )
                with self.callback_manager.event(
                    CBEventType.EMBEDDING,
                    payload={EventPayload.SERIALIZED: self.to_dict()},
                ) as event:
                    embeddings = self._get_text_embeddings(cur_batch)
                    result_embeddings.extend(embeddings)
                    event.on_end(
                        payload={
                            EventPayload.CHUNKS: cur_batch,
                            EventPayload.EMBEDDINGS: embeddings,
                        },
                    )
                cur_batch = []
        return result_embeddings

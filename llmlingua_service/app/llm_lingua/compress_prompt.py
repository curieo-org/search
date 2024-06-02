from typing import List
from llmlingua import PromptCompressor
import copy

import numpy as np


class ProxyPromptCompressor(PromptCompressor):
    def __init__(self, model_name):
        self.proxy = PromptCompressor(
            model_name=model_name, device_map="mps", use_llmlingua2=True
        )
        self.instruction_str = "Given the context, please answer the final question"

    def call__get_context_prob(
        self,
        context_list: list,
        token_to_word,
        force_tokens: List[str],
        token_map: dict,
        force_reserve_digit: bool,
    ):
        return self.proxy._PromptCompressor__get_context_prob(
            context_list, token_to_word, force_tokens, token_map, force_reserve_digit
        )

    def call__chunk_context(self, origin_text, chunk_end_tokens):
        return self.proxy._PromptCompressor__chunk_context(
            origin_text, chunk_end_tokens
        )

    def call__compress(
        self,
        context_list: list,
        reduce_rate: float,
        token_to_word: str,
        force_tokens: List[str],
        token_map: dict,
        force_reserve_digit: bool,
        drop_consecutive: bool,
    ):
        return self.proxy._PromptCompressor__compress(
            context_list,
            reduce_rate,
            token_to_word,
            force_tokens,
            token_map,
            force_reserve_digit,
            drop_consecutive,
        )


class LLMLinguaCompressor:

    def __init__(self, model_name):
        self._llm_lingua = ProxyPromptCompressor(model_name=model_name)
        self.instruction_str = "Given the context, please answer the final question"

    async def compress_prompt(
        self,
        query_str,
        context_texts_list,
        rate: float = 0.5,
        target_token: int = -1,
        use_context_level_filter: bool = False,
        use_token_level_filter: bool = True,
        target_context: int = -1,
        context_level_rate: float = 1.0,
        context_level_target_token: int = -1,
        force_context_ids: List[int] = [],
        return_word_label: bool = False,
        word_sep: str = "\t\t|\t\t",
        label_sep: str = " ",
        token_to_word: str = "mean",
        force_tokens: List[str] = [],
        force_reserve_digit: bool = False,
        drop_consecutive: bool = False,
        chunk_end_tokens: List[str] = [".", "\n"],
    ):

        assert len(force_tokens) <= self._llm_lingua.proxy.max_force_token
        token_map = {}
        chunk_end_tokens = copy.deepcopy(chunk_end_tokens)
        for c in chunk_end_tokens:
            if c in token_map:
                chunk_end_tokens.append(token_map[c])
        chunk_end_tokens = set(chunk_end_tokens)

        context = copy.deepcopy(context_texts_list)

        n_original_token = 0
        context_chunked = []
        for i in range(len(context)):
            n_original_token += self._llm_lingua.proxy.get_token_length(
                context[i], use_oai_tokenizer=True
            )
            for ori_token, new_token in token_map.items():
                context[i] = context[i].replace(ori_token, new_token)
            context_chunked.append(
                self._llm_lingua.call__chunk_context(
                    context[i], chunk_end_tokens=chunk_end_tokens
                )
            )

        if (
            target_context <= 0
            and context_level_rate >= 1.0
            and context_level_target_token <= 0
        ):
            if target_token < 0 and rate < 1.0:
                context_level_rate = (
                    (rate + 1.0) / 2 if use_token_level_filter else rate
                )
                if target_token >= 0:
                    context_level_target_token = (
                        target_token * 2 if use_token_level_filter else target_token
                    )

        if target_context >= 0:
            context_level_rate = min(target_context / len(context), 1.0)
            if context_level_target_token >= 0:
                context_level_rate = min(
                    context_level_target_token / n_original_token, 1.0
                )

        context_probs, context_words = self._llm_lingua.call__get_context_prob(
            context_chunked,
            token_to_word=token_to_word,
            force_tokens=force_tokens,
            token_map=token_map,
            force_reserve_digit=force_reserve_digit,
        )

        threshold = np.percentile(context_probs, int(100 * (1 - context_level_rate)))

        reserved_context = []
        sources = []
        context_label = [False] * len(context_probs)
        for i, p in enumerate(context_probs):
            if p >= threshold or (
                force_context_ids is not None and i in force_context_ids
            ):
                reserved_context.append(context_chunked[i])
                sources.append(i)
                context_label[i] = True
        n_reserved_token = 0
        for chunks in reserved_context:
            for c in chunks:
                n_reserved_token += self._llm_lingua.proxy.get_token_length(
                    c, use_oai_tokenizer=True
                )
        if target_token >= 0:
            rate = min(target_token / n_reserved_token, 1.0)

        compressed_context, word_list, word_label_list = (
            self._llm_lingua.call__compress(
                reserved_context,
                reduce_rate=max(0, 1 - rate),
                token_to_word=token_to_word,
                force_tokens=force_tokens,
                token_map=token_map,
                force_reserve_digit=force_reserve_digit,
                drop_consecutive=drop_consecutive,
            )
        )

        n_compressed_token = 0
        for c in compressed_context:
            n_compressed_token += self._llm_lingua.proxy.get_token_length(
                c, use_oai_tokenizer=True
            )
        ratio = 1 if n_compressed_token == 0 else n_original_token / n_compressed_token
        res = {
            "compressed_prompt": "\n\n".join(compressed_context),
            "compressed_prompt_list": compressed_context,
            "origin_tokens": n_original_token,
            "compressed_tokens": n_compressed_token,
            "ratio": f"{ratio:.1f}x",
            "sources": sources,
        }

        return res

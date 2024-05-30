from llmlingua import PromptCompressor


class LLMLinguaCompressor:

    def __init__(self, model_name):
        self._llm_lingua = PromptCompressor(
            model_name=model_name,
            device_map="cuda",
            use_llmlingua2=True
        )
        self.instruction_str = "Given the context, please answer the final question"

    async def compress_prompt(self, query_str, context_texts, target_token, rank_method="longllmlingua"):

        compressed_prompt = self._llm_lingua.compress_prompt(
            context_texts,
            instruction=self.instruction_str,
            question=query_str,
            target_token=target_token,
            rank_method=rank_method
        )
        compressed_prompt_txt = compressed_prompt["compressed_prompt"]

        return compressed_prompt_txt
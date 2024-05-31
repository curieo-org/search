from typing import List
from fastapi import APIRouter, Depends, Header, HTTPException
from fastapi.responses import JSONResponse
from fastapi.logger import logger
from pydantic import BaseModel

from app.api.router.gzip import GzipRoute
from app.llm_lingua.compress_prompt import LLMLinguaCompressor

router = APIRouter()
router.route_class = GzipRoute
compressor = LLMLinguaCompressor(
    model_name="microsoft/llmlingua-2-xlm-roberta-large-meetingbank"
)


class CompressPromptRequest(BaseModel):
    query: str = ""
    context_texts_list: List[str] = ""
    target_token: int = 300


class CompressPromptResponse(BaseModel):
    response: dict = {}


@router.post("/compress_prompt/")
async def compress_prompt(request: CompressPromptRequest) -> JSONResponse:
    try:
        response = await compressor.compress_prompt(
            query_str=request.query,
            context_texts_list=request.context_texts_list,
            target_token=request.target_token,
        )
        return CompressPromptResponse(response=response)
    except Exception as e:
        logger.error(f"Error processing request: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")

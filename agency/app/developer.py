from typing import Union
import uvicorn
from typing import List
from uuid import UUID

from fastapi import (APIRouter, Depends, Header, HTTPException)
from fastapi.responses import JSONResponse
from fastapi.logger import logger
from app.settings import Settings
from app.router.orchestrator import Orchestrator

settings = Settings()
router = APIRouter()
orchestrator = Orchestrator(settings)

@router.get(
    "/Search/",
    summary="List all Search Results",
    description="List all Search Results"
)
async def get_search_results(
    query: str = ""
) -> JSONResponse:
    data = await orchestrator.handle_pubmed_bioxriv_web_search(search_text=query)
    
    return JSONResponse(status_code=200, content=data)

if __name__ == "__main__":
    uvicorn.run(router, host="0.0.0.0", port=8000)
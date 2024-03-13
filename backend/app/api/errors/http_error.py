from fastapi import HTTPException, Request
from fastapi.responses import JSONResponse


HTTP_ERROR_TITLE = "HTTP_ERROR"

async def http_error_handler(_: Request, exc: HTTPException) -> JSONResponse:
    response = {
        "detail": exc.detail,
        "status": exc.status_code,
        "title": HTTP_ERROR_TITLE
    }
    return JSONResponse(response, status_code=exc.status_code)
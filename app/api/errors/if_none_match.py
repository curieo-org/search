from fastapi import HTTPException, Request, status
from fastapi.responses import Response


class IfNoneMatch(HTTPException):
    pass


async def if_none_match_handler(_: Request, exc: IfNoneMatch) -> Response:
    return Response("", 304, headers=exc.headers)
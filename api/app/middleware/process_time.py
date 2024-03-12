import time

from starlette.middleware.base import BaseHTTPMiddleware

HEADER_NAME = "X-Recommender-Process-Time"


def duration_string(start_time, end_time):
    return str(end_time - start_time)


class ProcessTimeHeaderMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request, call_next):
        start_time = time.time()
        response = await call_next(request)
        end_time = time.time()
        response.headers[HEADER_NAME] = duration_string(start_time, end_time)
        return response
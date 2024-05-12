from collections.abc import Mapping, Sequence

import httpx

URLTypes = httpx.URL | str
HeaderTypes = (
    httpx.Headers
    | Mapping[str, str]
    | Mapping[bytes, bytes]
    | Sequence[tuple[str, str]]
    | Sequence[tuple[bytes, bytes]]
)
TimeoutTypes = httpx.Timeout | float | tuple[float, float, float, float]


async def _httpx_infallible_request(
    method: str,
    url: URLTypes,
    *,
    headers: HeaderTypes | None = None,
    timeout: TimeoutTypes | None = None,
) -> httpx.Response | None:
    """Scoped variant of httpx request that will never raise an exception.

    This implementation is not as feature rich as httpx.AsyncClient.request. Look to
    that function definition if you need to add more parameters to this function.

    Same goes for adding additional helper methods like `httpx_get`. If you need POST
    simply add `httpx_post` in a similar fashion.
    """
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                method=method,
                url=url,
                headers=headers,
                timeout=timeout,
            )

        response.raise_for_status()

        return response

    except Exception:  # noqa
        return None


async def httpx_get(
    url: URLTypes,
    *,
    headers: HeaderTypes | None = None,
    timeout: TimeoutTypes | None = None,
) -> httpx.Response | None:
    return await _httpx_infallible_request(
        "get",
        url=url,
        headers=headers,
        timeout=timeout,
    )

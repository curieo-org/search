from starlette.config import Config, environ
from starlette.datastructures import Secret

config = Config(".env")
DEBUG = config('DEBUG', cast=bool, default=False)

DEFAULT_BASE_URL = 'http://127.0.0.1'

PROJECT_NAME: str = config("PROJECT_NAME", default="Curieo Search")
VERSION: str = config("VERSION", default="0.1")
DEBUG: bool = config("DEBUG", cast=bool, default=True)
TESTING: bool = config("TESTING", cast=bool, default=True)
SHOW_REQUEST_PROCESS_TIME_HEADER: bool = config("SHOW_REQUEST_PROCESS_TIME_HEADER", cast=bool, default=TESTING)
PROMPT_LANGUAGE: str = config("PROMPT_LANGUAGE", default="en-US")


# The internal location of the Search Backend.
# Used for doing calls to the Search Backend service.
SEARCH_API_ROOT: str = config("SEARCH_API_ROOT", default=DEFAULT_BASE_URL)

# The public facing url of the Search Backend.
# Used for creating Backend urls to deliver to clients.
SEARCH_API_PUBLIC_URL: str = config("SEARCH_API_PUBLIC_URL", default=DEFAULT_BASE_URL)

# Default max-age of cacheable resources
SEARCH_CACHE_MAX_AGE: str = config("SEARCH_CACHE_MAX_AGE", default='86400')

# Locale to return in Content-Language if none is known
SEARCH_FALLBACK_LOCALE: str = config("SEARCH_FALLBACK_LOCALE", default='en-US')

# Base URL we are publicly exposed as
SEARCH_PUBLIC_BASE_URL: str = config("SEARCH_PUBLIC_BASE_URL", default=DEFAULT_BASE_URL)

# Base URL SEARCH Service is publicly exposed as
SEARCH_PUBLIC_SERVICE_URL: str = config("SEARCH_PUBLIC_BASE_URL", default=SEARCH_PUBLIC_BASE_URL)

##BRAVE SEARCH
BRAVE_SEARCH_API: str = config("BRAVE_SEARCH_API_ROOT", default="https://api.search.brave.com/res/v1/web/search")
BRAVE_SUBSCRIPTION_KEY  = config('BRAVE_SUBSCRIPTION_KEY', cast=Secret)
BRAVE_RESULT_COUNT: int = config("BRAVE_RESULT_COUNT", default=10)

## LLM SERVICE Details
LLM_SERVICE_PROVIDER: str = config("LLM_SERVICE_PROVIDER", default="togetherai")

##TogetherAI API Definition
# TogetherAI API URL
TOGETHER_API: str = config("TOGETHER_API", default="https://api.together.xyz/v1/completions")
TOGETHER_KEY: str  = config('TOGETHER_KEY', cast=Secret)
TOGETHER_MODEL: str = config("TOGETHER_MODEL", default="mistralai/Mistral-7B-Instruct-v0.1")
TOGETHER_PROMPT_CONFIG: dict = config("TOGETHER_PROMPT_CONFIG", default = {
    "max_tokens": 1024,
    "temperature": 0.7,
    "top_p": 0.7,
    "prompt" : {"prompt_token_limit": 4096},
    "stream": True }
    )

##OpenAPI API Definition
OPENAPI_KEY: str  = config('OPENAPI_KEY', cast=Secret)


## Embeddings Details
# Embedding Rerank API URL
EMBEDDING_RERANK_API: str = config("EMBEDDING_RERANK_API", default="http://127.0.0.1:8080/rerank")
EMBEDDING_CHUNK_SIZE: int = config("EMBEDDING_CHUNK_SIZE", default=512)
EMBEDDING_MODEL_API: str = config("EMBEDDING_MODEL_API", default="http://127.0.0.1:8081")
EMBEDDING_MODEL_NAME: str = config("EMBEDDING_MODEL_NAME", default="BAAI/bge-small-en-v1.5")

#RERANK 
RERANK_TOP_COUNT: int = config("RERANK_TOP_COUNT", default=5)

## Clinical Trails Configurations Details
#table info dir
CLINICAL_TRIALS_TABLE_INFO_DIR: str = config("CLINICAL_TRIALS_TABLE_INFO_DIR", default="app/rag/retrieval/clinical_trials/AACTTableQuestions_TableInfo")
POSTGRES_ENGINE: str = config('POSTGRES_ENGINE', cast=Secret)                                           

## REDIS Configuration
#REDIS URL
REDIS_URL: str = config("REDIS_URL", default="redis://127.0.0.1:6379")


## JWT
# JWT_SECRET_KEY key used to validate RS256 signed JWTs.
# Can also be shared secret for HS256 signed JWTs.
JWT_SECRET_KEY: str = config("JWT_SECRET_KEY", cast=Secret)

# Algorithm used to sign JWT. Can be RS256, HS256 and None.
JWT_ALGORITHM: str = config("JWT_ALGORITHM", default='HS256')
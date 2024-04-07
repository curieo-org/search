# Load environment variables from .env file
from pydantic import (
    BaseModel,
    RedisDsn,
    SecretStr,
)

from pydantic_settings import BaseSettings, SettingsConfigDict


class ProjectSettings(BaseSettings):
    name: str = "Curieo Search"
    environment: str = "production"
    version: str = "0.0.1"
    default_base_url: str = "http://127.0.0.1"
    debug: bool = True
    testing: bool = True
    show_request_process_time_header: bool = True
    prompt_language: str = "en-US"


class SearchSettings(BaseSettings):
    api_root: str = "http://127.0.0.1"
    api_public_url: str = "http://127.0.0.1"
    api_base_url: str = "http://127.0.0.1"
    locale: str = "en-US"


class BraveSettings(BaseSettings):
    api_root: str = "https://api.search.brave.com/res/v1/web/search"
    subscription_key: SecretStr
    result_count: int = 10


class TogetherPromptConfig(BaseModel):
    max_tokens: int = 1024
    temperature: float = 0.7
    top_p: float = 0.7
    prompt_token_limit: int = 4096
    stream: bool = True


class TogetherSettings(BaseSettings):
    api_root: str = "https://api.together.xyz/v1/completions"
    api_key: SecretStr
    model: str = "mistralai/Mistral-7B-Instruct-v0.1"
    prompt_config: TogetherPromptConfig = TogetherPromptConfig()


class OpenAISettings(BaseSettings):
    api_key: SecretStr


class EmbeddingSettings(BaseSettings):
    api: str = "http://127.0.0.1:8080"
    model: str = "BAAI/bge-large-en-v1.5"
    revision: str = "refs/pr/5"
    chunk_size: int = 512


class RerankSettings(BaseSettings):
    api: str = "http://127.0.0.1:8081/rerank"
    model: str = "BAAI/bge-reranker-large"
    revision: str = "refs/pr/4"
    chunk_size: int = 512
    top_count: int = 5


class TableInfoDirSettings(BaseSettings):
    clinical_trials: str = "app/rag/retrieval/clinical_trials/AACTTableQuestions_TableInfo"
    drug_chembl: str = "app/rag/retrieval/drug_chembl/ChEMBLTableQuestions_TableInfo"


class NebulaGraphSettings(BaseSettings):
    host: str = "http://127.0.0.1"
    port: int = 9669
    user: SecretStr
    password: SecretStr
    space: str = "chembl"


class RedisSettings(BaseSettings):
    url: RedisDsn
    max_age: int = 86400
    max_sorted_set: int = 100


class WandbSettings(BaseSettings):
    api_key: SecretStr
    project: str = "pe_router"
    entity: str = "curieo"
    note: str = "Curieo Search"


class SentrySettings(BaseSettings):
    dsn: SecretStr
    enable_tracing: bool = False
    environment: str = "development"
    phoenix_api: str = "http://127.0.0.1:6007/v1/traces"


class GroqSettings(BaseSettings):
    api_key: SecretStr


class QdrantSettings(BaseSettings):
    api_key: SecretStr
    api_port: int = 6333
    api_url: str = "localhost"

    collection_name: str = "pubmed_hybrid_vector_db"
    top_k: int = 20
    sparse_top_k: int = 3

    clinical_trial_collection_name: str = "clinical_trials_vector_db"
    clinical_trial_top_k: int = 5
    clinical_trial_metadata_field_name: str = "title"


class LlamaIndexSettings(BaseSettings):
    chat_enabled: bool = False
    relevance_criteria: float = 0.7


class DspySettings(BaseSettings):
    clinical_trial_sql_program: str = (
        "app/dspy_integration/dspy_programs/clinical_trials_sql_generation.json"
    )
    clinical_trials_response_refinement_program: str = (
        "app/dspy_integration/dspy_programs/clinical_trials_response_refinement.json"
    )
    orchestrator_router_prompt_program: str = (
        "app/dspy_integration/dspy_programs/orchestrator_router_prompt.json"
    )


class AIModelsSettings(BaseSettings):
    router: str = "gpt-3.5-turbo"
    sql_generation: str = "codellama/CodeLlama-13b-Instruct-hf"
    clinical_trail_response_synthesizer_model: str = (
        "NousResearch/Nous-Hermes-llama-2-7b"
    )
    pubmed_response_synthesizer_model: str = "mistralai/Mixtral-8x7B-Instruct-v0.1"


class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file='.env', env_file_encoding='utf-8',
                                      env_nested_delimiter='__')

    postgres_engine: SecretStr
    project: ProjectSettings = ProjectSettings()
    search: SearchSettings
    brave: BraveSettings
    together: TogetherSettings
    openai: OpenAISettings
    nebula_graph: NebulaGraphSettings
    redis: RedisSettings
    wandb: WandbSettings
    sentry: SentrySettings
    groq: GroqSettings
    dspy: DspySettings = DspySettings()
    embedding: EmbeddingSettings = EmbeddingSettings()
    reranking: RerankSettings = RerankSettings()
    qdrant: QdrantSettings
    llama_index: LlamaIndexSettings
    table_info_dir: TableInfoDirSettings = TableInfoDirSettings()
    ai_models: AIModelsSettings = AIModelsSettings()

# print(Settings().model_dump())
# settings = Settings()

# Project Settings
# DEBUG: bool = config("DEBUG", cast=bool, default=True)
# DEFAULT_BASE_URL: str = config("DEFAULT_BASE_URL", default="http://127.0.0.1")
# PROJECT_NAME: str = config("PROJECT_NAME", default="Curieo Search")
# VERSION: str = config("VERSION", default="0.0.1")
# TESTING: bool = config("TESTING", cast=bool, default=True)
# SHOW_REQUEST_PROCESS_TIME_HEADER: bool = config(
#    "SHOW_REQUEST_PROCESS_TIME_HEADER", cast=bool, default=TESTING
# )
# PROMPT_LANGUAGE: str = config("PROMPT_LANGUAGE", default="en-US")

# SEARCH API Configuration
## The internal location of the Search Backend.
## Used for doing calls to the Search Backend service.
# SEARCH_API_ROOT: str = config("SEARCH_API_ROOT", default=DEFAULT_BASE_URL)

## The public facing url of the Search Backend.
## Used for creating Backend urls to deliver to clients.
# SEARCH_API_PUBLIC_URL: str = config("SEARCH_API_PUBLIC_URL", default=DEFAULT_BASE_URL)

## Locale to return in Content-Language if none is known
# SEARCH_FALLBACK_LOCALE: str = config("SEARCH_FALLBACK_LOCALE", default="en-US")

## Base URL we are publicly exposed as
# SEARCH_PUBLIC_BASE_URL: str = config("SEARCH_PUBLIC_BASE_URL", default=DEFAULT_BASE_URL)

## Base URL SEARCH Service is publicly exposed as
# SEARCH_PUBLIC_SERVICE_URL: str = config(
#    "SEARCH_PUBLIC_BASE_URL", default=SEARCH_PUBLIC_BASE_URL
# )

# BRAVE SEARCH API Configuration
# BRAVE_SEARCH_API: str = config(
#    "BRAVE_SEARCH_API_ROOT", default="https://api.search.brave.com/res/v1/web/search"
# )
# BRAVE_SUBSCRIPTION_KEY: Secret = config("BRAVE_SUBSCRIPTION_KEY", cast=Secret)
# BRAVE_RESULT_COUNT: int = config("BRAVE_RESULT_COUNT", default=10)
#
## LLM SERVICE Configuration
# LLM_SERVICE_PROVIDER: str = config("LLM_SERVICE_PROVIDER", default="togetherai")
#
### TogetherAI API Configuration
# TOGETHER_API: str = config(
#    "TOGETHER_API", default="https://api.together.xyz/v1/completions"
# )
# TOGETHER_KEY: Secret = config("TOGETHER_KEY", cast=Secret)
# TOGETHER_MODEL: str = config(
#    "TOGETHER_MODEL", default="mistralai/Mistral-7B-Instruct-v0.1"
# )
# TOGETHER_PROMPT_CONFIG: dict = {
#    "max_tokens": 1024,
#    "temperature": 0.7,
#    "top_p": 0.7,
#    "prompt": {"prompt_token_limit": 4096},
#    "stream": True,
# }
#
## OpenAI API Configuration
# OPENAI_API_KEY: Secret = config("OPENAI_API_KEY", cast=Secret)
#
## Embeddings Model Configuration
# EMBEDDING_CHUNK_SIZE: int = config("EMBEDDING_CHUNK_SIZE", default=512)
# EMBEDDING_MODEL_API: str = config("EMBEDDING_MODEL_API",
#                                  default="http://127.0.0.1:8080")
# EMBEDDING_MODEL_NAME: str = config(
#    "EMBEDDING_MODEL_NAME", default="BAAI/bge-large-en-v1.5"
# )
#
## RERANK Model Configuration
# EMBEDDING_RERANK_API: str = config("EMBEDDING_RERANK_API",
#                                   default="http://127.0.0.1:8081/rerank")
# RERANK_TOP_COUNT: int = config("RERANK_TOP_COUNT", default=5)
#
## Clinical Trails Configurations
### table info dir
# CLINICAL_TRIALS_TABLE_INFO_DIR: str = "app/rag/retrieval/clinical_trials/AACTTableQuestions_TableInfo"
# POSTGRES_ENGINE: Secret = config("POSTGRES_ENGINE", cast=Secret)
#
## Drug ChEMBL Configurations
### table info dir
# DRUG_CHEMBL_TABLE_INFO_DIR: str = "app/rag/retrieval/drug_chembl/ChEMBLTableQuestions_TableInfo"
#
### NEBULA GRAPH Configuration
# NEBULA_GRAPH_HOST: str = config("NEBULA_GRAPH_HOST", default="http://127.0.0.1")
# NEBULA_GRAPH_PORT: str = config("NEBULA_GRAPH_PORT", default="9669")
# NEBULA_GRAPH_USER: Secret = config("NEBULA_GRAPH_USER", cast=Secret)
# NEBULA_GRAPH_PASSWORD: Secret = config("NEBULA_GRAPH_PASSWORD", cast=Secret)
# NEBULA_GRAPH_SPACE: str = config("NEBULA_GRAPH_SPACE", default="chembl")
#
## REDIS Configuration
# REDIS_URL: Secret = config("REDIS_URL", cast=Secret)
# CACHE_MAX_AGE: str = config("SEARCH_CACHE_MAX_AGE", default="86400")
# CACHE_MAX_SORTED_SET: int = config("CACHE_MAX_SORTED_SET", default=100)
#
## JWT Configuration
### JWT_SECRET_KEY key used to validate RS256 signed JWTs.
### Can also be shared secret for HS256 signed JWTs.
# JWT_SECRET_KEY: Secret = config("JWT_SECRET_KEY", cast=Secret)
### Algorithm used to sign JWT. Can be RS256, HS256 and None.
# JWT_ALGORITHM: str = config("JWT_ALGORITHM", default="HS256")
#
## WANDB Configuration
# WANDB_API_KEY: Secret = config("WANDB_API_KEY", cast=Secret)
# WANDB_PROJECT: str = config("WANDB_PROJECT", default="pe_router")
# WANDB_ENTITY: str = config("WANDB_ENTITY", default="curieo")
# WANDB_NOTE: str = config("WANDB_NOTE", default="Curieo Search")
#
## Sentry Configuration
# SENTRY_DSN: Secret = config("SENTRY_DSN", cast=Secret)
# SENTRY_ENABLE_TRACING: bool = config("SENTRY_ENABLE_TRACING", cast=bool, default=False)
#
## GROQ API Configuration
# GROQ_API_KEY: Secret = config("GROQ_API_KEY", cast=Secret)
#
## QDRANT API configuration
# QDRANT_API_KEY: Secret | None = config("QDRANT_API_KEY", cast=Secret | None)
# QDRANT_API_PORT: int = config("QDRANT_API_PORT", default=6333)
# if ENVIRONMENT == 'local':
#    QDRANT_API_URL = config("QDRANT_API_URL", default="localhost")
# else:
#    QDRANT_API_URL = config("QDRANT_API_URL",
#                            default="https://ff1f8e90-959e-4cff-9455-03914d8a7002.europe-west3-0.gcp.cloud.qdrant.io")
# QDRANT_COLLECTION_NAME: str = config("QDRANT_COLLECTION_NAME",
#                                     default="pubmed_hybrid_vector_db")
# QDRANT_CLINICAL_TRIAL_COLLECTION_NAME: str = config(
#    "QDRANT_CLINICAL_TRIAL_COLLECTION_NAME", default="clinical_trials_vector_db")
# QDRANT_TOP_K: int = config("QDRANT_TOP_K", default=20)
# QDRANT_SPARSE_TOP_K: int = config("QDRANT_SPARSE_TOP_K", default=3)
# QDRANT_TOP_CLINICAL_TRAIL_K: int = config("QDRANT_TOP_CLINICAL_TRAIL_K", default=5)
# QDRANT_CLINICAL_TRIAL_METADATA_FIELD_NAME: str = config(
#    "QDRANT_CLINICAL_TRIAL_METADATA_FIELD_NAME", default="title")
#
## LLAMA_INDEX Configuration
# CHAT_ENABLED: bool = config("CHAT_ENABLED", default=False)
# PUBMED_RELEVANCE_CRITERIA: float = config("PUBMED_RELEVANCE_CRITERIA", default=0.7)
#
## Dspy Integration Configuration
# CLINICAL_TRIAL_SQL_PROGRAM: str = "app/dspy_integration/dspy_programs/clinical_trials_sql_generation.json"
# CLINICAL_TRIALS_RESPONSE_REFINEMENT_PROGRAM: str = "app/dspy_integration/dspy_programs/clinical_trials_response_refinement.json"
# ORCHESTRATOR_ROUTER_PROMPT_PROGRAM: str = "app/dspy_integration/dspy_programs/orchestrator_router_prompt.json"
#
## Phoenix Configuration
# PHOENIX_API_ENDPOINT: str = config("PHOENIX_API_ENDPOINT",
#                                   default="http://127.0.0.1:6007/v1/traces")
#
## AI models
# ROUTER_MODEL: str = config("ROUTER_MODEL", default="gpt-3.5-turbo")
# SQL_GENERATION_MODEL: str = config("ROUTER_MODEL",
#                                   default="codellama/CodeLlama-13b-Instruct-hf")
# CLINICAL_TRAIL_RESPONSE_SYNTHESIZER_MODEL: str = config(
#    "CLINICAL_TRAIL_RESPONSE_SYNTHESIZER_MODEL",
#    default="NousResearch/Nous-Hermes-llama-2-7b")
# PUBMED_RESPONSE_SYNTHESIZER_MODEL: str = config("PUBMED_RESPONSE_SYNTHESIZER_MODEL",
#                                                default="mistralai/Mixtral-8x7B-Instruct-v0.1")
#

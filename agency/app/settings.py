# Load environment variables from .env file
from pydantic import SecretStr
from pydantic_settings import BaseSettings, SettingsConfigDict


class ProjectSettings(BaseSettings):
    name: str = "Curieo Search"
    environment: str = "production"
    version: str = "0.0.1"
    default_base_url: str = "http://127.0.0.1"
    port: int = 50051
    graceful_shutdown_period: int = 5
    max_grpc_workers: int = 10
    debug: bool = True
    testing: bool = True
    show_request_process_time_header: bool = True
    prompt_language: str = "en-US"


class SearchSettings(BaseSettings):
    api_root: str = "http://127.0.0.1"
    api_public_url: str = "http://127.0.0.1"
    api_base_url: str = "http://127.0.0.1"
    locale: str = "en-US"

class EmbeddingSettings(BaseSettings):
    api_url: str = "http://localhost:8080"
    api_key: SecretStr
    batch_size: int = 4


class SpladeEmbeddingSettings(BaseSettings):
    api: str = "http://localhost:8081"
    api_key: SecretStr
    batch_size: int = 4


class RedisSettings(BaseSettings):
    url: SecretStr
    default_expiry: int = 86400


class TracingSettings(BaseSettings):
    sentry_dsn: SecretStr
    jaeger_endpoint: str = "http://127.0.0.1:4317"
    enable_tracing: bool = False
    environment: str = "development"
    phoenix_api: str = "http://127.0.0.1:4317"
    project_name: str = "Curieo Search"
    service_name: str = "agency-service"


class QdrantSettings(BaseSettings):
    api_port: int = 6333
    api_url: str = "localhost"
    api_key: SecretStr
    collection_name: str = "pubmed_hybrid"
    top_k: int = 10
    sparse_top_k: int = 5
    metadata_field_name: str = "title"


class PubmedRetrievalSettings(BaseSettings):
    parent_relevance_criteria: float = 0.1
    cluster_relevance_criteria: float = 0.1
    url_prefix: str = "https://pubmed.ncbi.nlm.nih.gov"


class PubmedDatabaseSettings(BaseSettings):
    connection: SecretStr
    children_text_table_name: str = "pubmed_text_details"
    record_title_table_name: str = "pubmed_titles"


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        extra="allow",
        env_file=".env",
        env_file_encoding="utf-8",
        env_nested_delimiter="__",
    )

    pubmed_database: PubmedDatabaseSettings
    project: ProjectSettings = ProjectSettings()
    search: SearchSettings = SearchSettings()
    redis: RedisSettings
    tracing: TracingSettings
    embedding: EmbeddingSettings
    spladeembedding: SpladeEmbeddingSettings
    pubmed_parent_qdrant: QdrantSettings
    pubmed_cluster_qdrant: QdrantSettings
    pubmed_retrieval: PubmedRetrievalSettings = PubmedRetrievalSettings()

app_settings = Settings()

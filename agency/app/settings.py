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


class BraveSettings(BaseSettings):
    api_root: str = "https://api.search.brave.com/res/v1/web/search"
    subscription_key: SecretStr
    result_count: int = 10
    goggles_id: SecretStr
    result_filter: list[str] = [
        "discussions",
        "faq",
        "summarizer",
        "infobox",
        "news",
        "query",
        "web",
    ]


class TogetherPromptConfig(BaseSettings):
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


class BioLLMSettings(BaseSettings):
    model_name: str = "ivarflakstad/Llama3-OpenBioLLM-8B"
    api_url: str = "http://localhost:8085"
    temperature: float = 0.1
    max_tokens: int = 1000
    prompt_token_limit: int = 3000


class OpenAISettings(BaseSettings):
    api_key: SecretStr


class EmbeddingSettings(BaseSettings):
    api_url: str = "http://localhost:8080"
    api_key: SecretStr
    batch_size: int = 4


class SpladeEmbeddingSettings(BaseSettings):
    api: str = "http://localhost:8081"
    api_key: SecretStr
    batch_size: int = 4


class PostProcessingSettings(BaseSettings):
    api: str = "http://localhost:8000/compress"
    max_tokens_per_node: int = 512
    compressed_target_token: int = 300
    top_n_sources: int = 10


class TableInfoDirSettings(BaseSettings):
    clinical_trials: str = (
        "app/rag/retrieval/clinical_trials/AACTTableQuestions_TableInfo"
    )
    drug_chembl: str = "app/rag/retrieval/drug_chembl/ChEMBLTableQuestions_TableInfo"


class NebulaGraphSettings(BaseSettings):
    host: str = "http://127.0.0.1"
    port: int = 9669
    user: SecretStr
    password: SecretStr
    space: str = "chembl"


class RedisSettings(BaseSettings):
    url: SecretStr
    default_expiry: int = 86400


class WandbSettings(BaseSettings):
    api_key: SecretStr
    project: str = "pe_router"
    entity: str = "curieo"
    note: str = "Curieo Search"


class TracingSettings(BaseSettings):
    sentry_dsn: SecretStr
    jaeger_endpoint: str = "http://127.0.0.1:4317"
    enable_tracing: bool = False
    environment: str = "development"
    phoenix_api: str = "http://127.0.0.1:6006/v1/traces"
    project_name: str = "Curieo Search"
    service_name: str = "agency-service"


class GroqSettings(BaseSettings):
    api_key: SecretStr


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
    brave: BraveSettings
    together: TogetherSettings
    openai: OpenAISettings
    nebula_graph: NebulaGraphSettings
    redis: RedisSettings
    wandb: WandbSettings | None = None
    tracing: TracingSettings
    groq: GroqSettings
    dspy: DspySettings = DspySettings()
    embedding: EmbeddingSettings
    spladeembedding: SpladeEmbeddingSettings
    post_process: PostProcessingSettings = PostProcessingSettings()
    biollm: BioLLMSettings = BioLLMSettings()
    pubmed_parent_qdrant: QdrantSettings
    pubmed_cluster_qdrant: QdrantSettings
    clinical_trial_qdrant: QdrantSettings
    pubmed_retrieval: PubmedRetrievalSettings = PubmedRetrievalSettings()
    table_info_dir: TableInfoDirSettings = TableInfoDirSettings()
    ai_models: AIModelsSettings = AIModelsSettings()


app_settings = Settings()

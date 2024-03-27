import sentry_sdk
import logging
from sentry_sdk.integrations.logging import LoggingIntegration
from sentry_sdk.integrations.asyncio import AsyncioIntegration
from sentry_sdk.integrations.starlette import StarletteIntegration
from sentry_sdk.integrations.fastapi import FastApiIntegration

from openinference.instrumentation.llama_index import LlamaIndexInstrumentor
from opentelemetry import trace as trace_api
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk import trace as trace_sdk
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace.export import SimpleSpanProcessor

from app.config import SENTRY_DSN, SENTRY_ENABLE_TRACING, PHOENIX_API_ENDPOINT, ENVIRONMENT

def setup_tracing():
    sentry_sdk.init(
        dsn=str(SENTRY_DSN),
        enable_tracing=SENTRY_ENABLE_TRACING,
        integrations=[
            AsyncioIntegration(),
            StarletteIntegration(
                transaction_style="endpoint"
            ),
            FastApiIntegration(
                transaction_style="endpoint"
            ),
            LoggingIntegration(
                level=logging.INFO,
                event_level=logging.WARNING
            ),
        ],
    )


    if ENVIRONMENT == 'production':
        resource = Resource(attributes={})
        tracer_provider = trace_sdk.TracerProvider(resource=resource)
        span_exporter = OTLPSpanExporter(endpoint=PHOENIX_API_ENDPOINT)
        span_processor = SimpleSpanProcessor(span_exporter=span_exporter)
        tracer_provider.add_span_processor(span_processor=span_processor)
        trace_api.set_tracer_provider(tracer_provider=tracer_provider)
        LlamaIndexInstrumentor().instrument()


setup_tracing()
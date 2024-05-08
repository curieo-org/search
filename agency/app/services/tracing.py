import logging

import sentry_sdk
from openinference.instrumentation.llama_index import LlamaIndexInstrumentor
from openinference.semconv.resource import ResourceAttributes
from opentelemetry import trace as trace_api
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk import trace as trace_sdk
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace.export import SimpleSpanProcessor
from sentry_sdk.integrations.asyncio import AsyncioIntegration
from sentry_sdk.integrations.grpc import GRPCIntegration
from sentry_sdk.integrations.logging import LoggingIntegration

from app.settings import SentrySettings


def setup_tracing(settings: SentrySettings):
    sentry_sdk.init(
        dsn=settings.dsn.get_secret_value(),
        enable_tracing=settings.enable_tracing,
        integrations=[
            AsyncioIntegration(),
            GRPCIntegration(),
            LoggingIntegration(level=logging.INFO, event_level=logging.WARNING),
        ],
    )

    if settings.environment == "production":
        resource = Resource(
            attributes={
                ResourceAttributes.PROJECT_NAME: settings.phoenix_project_name,
            }
        )
        tracer_provider = trace_sdk.TracerProvider(resource=resource)
        span_exporter = OTLPSpanExporter(endpoint=settings.phoenix_api)
        span_processor = SimpleSpanProcessor(span_exporter=span_exporter)
        tracer_provider.add_span_processor(span_processor=span_processor)
        trace_api.set_tracer_provider(tracer_provider=tracer_provider)
        LlamaIndexInstrumentor().instrument()
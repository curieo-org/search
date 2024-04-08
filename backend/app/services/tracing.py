import logging

import sentry_sdk
from openinference.instrumentation.llama_index import LlamaIndexInstrumentor
from opentelemetry import trace as trace_api
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk import trace as trace_sdk
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace.export import SimpleSpanProcessor
from sentry_sdk.integrations.asyncio import AsyncioIntegration
from sentry_sdk.integrations.fastapi import FastApiIntegration
from sentry_sdk.integrations.logging import LoggingIntegration
from sentry_sdk.integrations.starlette import StarletteIntegration

from app.settings import SentrySettings


def setup_tracing(settings: SentrySettings):
    sentry_sdk.init(
        dsn=str(settings.dsn),
        enable_tracing=settings.enable_tracing,
        integrations=[
            AsyncioIntegration(),
            StarletteIntegration(transaction_style="endpoint"),
            FastApiIntegration(transaction_style="endpoint"),
            LoggingIntegration(level=logging.INFO, event_level=logging.WARNING),
        ],
    )

    if settings.project.environment == "production":
        resource = Resource(attributes={})
        tracer_provider = trace_sdk.TracerProvider(resource=resource)
        span_exporter = OTLPSpanExporter(endpoint=settings.phoenix_api)
        span_processor = SimpleSpanProcessor(span_exporter=span_exporter)
        tracer_provider.add_span_processor(span_processor=span_processor)
        trace_api.set_tracer_provider(tracer_provider=tracer_provider)
        LlamaIndexInstrumentor().instrument()

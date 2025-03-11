import logging

import sentry_sdk
from openinference.instrumentation.llama_index import LlamaIndexInstrumentor
from openinference.semconv.resource import ResourceAttributes
from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.instrumentation.grpc import GrpcAioInstrumentorServer
from opentelemetry.instrumentation.llamaindex import (
    LlamaIndexInstrumentor as OpentelemetryLlamaIndexInstrumentor,
)
from opentelemetry.propagate import set_global_textmap
from opentelemetry.sdk import trace as trace_sdk
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace.export import BatchSpanProcessor, SimpleSpanProcessor
from opentelemetry.trace.propagation.tracecontext import TraceContextTextMapPropagator
from sentry_sdk.integrations.asyncio import AsyncioIntegration
from sentry_sdk.integrations.grpc import GRPCIntegration
from sentry_sdk.integrations.logging import LoggingIntegration

from app.settings import TracingSettings


def setup_tracing(settings: TracingSettings) -> None:
    if settings.environment == "production":
        resource = Resource(
            attributes={
                ResourceAttributes.PROJECT_NAME: settings.project_name,
                "service.name": settings.service_name,
            },
        )

        # sentry tracing
        sentry_sdk.init(
            dsn=settings.sentry_dsn.get_secret_value(),
            enable_tracing=settings.enable_tracing,
            integrations=[
                AsyncioIntegration(),
                GRPCIntegration(),
                LoggingIntegration(level=logging.INFO, event_level=logging.WARNING),
            ],
        )

        # phoenix tracing
        tracer_provider = trace_sdk.TracerProvider(resource=resource)
        span_exporter = OTLPSpanExporter(endpoint=settings.phoenix_api)
        span_processor = SimpleSpanProcessor(span_exporter=span_exporter)
        tracer_provider.add_span_processor(span_processor=span_processor)
        trace.set_tracer_provider(tracer_provider=tracer_provider)
        LlamaIndexInstrumentor().instrument()

        # jaeger opentelemetry tracing
        trace.get_tracer_provider().add_span_processor(
            BatchSpanProcessor(
                OTLPSpanExporter(
                    endpoint=settings.jaeger_endpoint,
                )
            )
        )
        set_global_textmap(TraceContextTextMapPropagator())
        GrpcAioInstrumentorServer().instrument()
        OpentelemetryLlamaIndexInstrumentor().instrument()

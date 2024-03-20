import sentry_sdk
from opentelemetry import trace
from opentelemetry.propagate import set_global_textmap
from opentelemetry.sdk.trace import TracerProvider
from sentry_sdk.integrations.opentelemetry import SentrySpanProcessor, SentryPropagator
from sentry_sdk.integrations.asyncio import AsyncioIntegration
from opentelemetry.trace.propagation.tracecontext import TraceContextTextMapPropagator

from app.config import SENTRY_DSN

sentry_sdk.init(
    dsn=str(SENTRY_DSN),
    enable_tracing=True,
    integrations=[AsyncioIntegration()],
    instrumenter="otel",
)

provider = TracerProvider()
provider.add_span_processor(SentrySpanProcessor())
trace.set_tracer_provider(provider)
set_global_textmap(SentryPropagator())
trace_client = trace.get_tracer(__name__)


class SentryTracer:
    def __init__(self):
        global trace_client

        if not trace_client:
            provider = TracerProvider()
            provider.add_span_processor(SentrySpanProcessor())
            trace.set_tracer_provider(provider)
            set_global_textmap(SentryPropagator())
            trace_client = trace.get_tracer(__name__)


    async def create_span(self, trace_id: str, operation: str) -> trace.Span:
        global trace_client

        carrier = {'traceparent': trace_id}
        context = TraceContextTextMapPropagator().extract(carrier=carrier)
        span = trace_client.start_span(operation, context=context)

        return span


    async def create_child_span(self, parent_trace_span, operation: str) -> trace.Span:
        global trace_client

        parent_span_context = parent_trace_span.get_span_context()
        traceparent = '00-' + str(hex(parent_span_context.trace_id))[2:] + '-' + str(hex(parent_span_context.span_id))[2:] + '-' + str(hex(parent_span_context.trace_flags))[2:].zfill(2)

        return await self.create_span(traceparent, operation)
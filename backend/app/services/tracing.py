import sentry_sdk
import logging
from sentry_sdk.integrations.logging import LoggingIntegration
from sentry_sdk.integrations.asyncio import AsyncioIntegration
from sentry_sdk.integrations.starlette import StarletteIntegration
from sentry_sdk.integrations.fastapi import FastApiIntegration

from app.config import SENTRY_DSN

def setup_tracing():
    sentry_sdk.init(
        dsn=str(SENTRY_DSN),
        enable_tracing=True,
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
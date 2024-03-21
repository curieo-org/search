from fastapi import Depends, APIRouter, HTTPException, status, Header
from fastapi.security import OAuth2PasswordRequestForm
from authx import AuthX, AuthXConfig

from .models import Token, User
from app.api.router.gzip import GzipRoute
from app.config import JWT_SECRET_KEY, JWT_ALGORITHM
from app.services.search_utility import setup_logger
from app.services.tracing import SentryTracer

router = APIRouter()
router.route_class = GzipRoute

auth_config = AuthXConfig()
auth_config.JWT_ALGORITHM = JWT_ALGORITHM
auth_config.JWT_SECRET_KEY = str(JWT_SECRET_KEY)

security = AuthX(config=auth_config)

logger = setup_logger("auth")


@router.post("/token", response_model=Token)
async def login_for_access_token(form_data: OAuth2PasswordRequestForm = Depends(), traceparent: str = Header(None)):
    trace_span = await SentryTracer().create_span(traceparent, 'api_token')

    with trace_span:
        trace_span.set_attribute('description', f"Login for Access Token. username: {form_data.username}")

        user = await authenticate_user(form_data.username, form_data.password)
        if not user:
            raise HTTPException(
                status_code=401,
                detail="Bad credentials",
                headers={"WWW-Authenticate": "Bearer"},
            )

        token = security.create_access_token(uid=user.username)
        trace_span.set_attribute('result', str(token))

    return Token(access_token=token, token_type="bearer")


async def authenticate_user(username: str, password: str) -> User | None:
    user = await get_user(username)
    if not user:
        return None

    # if not verify_password(password, user.hashed_password):
    #    return None

    return user


# FIXME: Should be obvious
async def get_user(_username: str) -> User | None:
    return User(username="curieo")


# FIXME: Should be obvious
def verify_password(_password: str, _hashed_password: str) -> bool:
    return True

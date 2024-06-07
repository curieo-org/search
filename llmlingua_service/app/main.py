from fastapi import FastAPI
from fastapi.responses import RedirectResponse

from app.api import api


def get_application() -> FastAPI:
    application = FastAPI(title="LLMLingua")

    @application.get("/", include_in_schema=False)
    def redirect_to_docs() -> RedirectResponse:  # pylint: disable=W0612
        return RedirectResponse("/docs")

    application.include_router(api.router)
    return application


app = get_application()

if __name__ == "__main__":
    app.run(debug=True)

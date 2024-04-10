# pull official base image
FROM python:3.11.8-slim

# install system dependencies
RUN apt-get update && apt-get -y install libpq-dev gcc git


# install python dependencies
COPY ./pyproject.toml /
COPY ./poetry.lock /
RUN pip install poetry uvicorn
RUN poetry config virtualenvs.create false
RUN poetry install --compile

COPY ./app /app
COPY .env /.env

EXPOSE 8000

CMD ["uvicorn", "app.main:app", "--host=0.0.0.0", "--port", "80"]

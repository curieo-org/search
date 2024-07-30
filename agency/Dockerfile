# pull official base image
FROM python:3.12.4-slim

# install system dependencies
RUN apt-get update && apt-get -y install libpq-dev gcc git


# install python dependencies
COPY ./pyproject.toml /
COPY ./poetry.lock /
RUN pip install --no-cache-dir --upgrade pip
RUN pip install poetry
RUN poetry config virtualenvs.create false
RUN poetry install --compile

COPY ./app /app

EXPOSE 50051

CMD ["poetry", "run", "app"]

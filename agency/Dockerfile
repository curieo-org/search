# pull official base image
FROM --platform=linux/amd64 python:3.11.8-slim

# install system dependencies
RUN apt-get update && apt-get -y install libpq-dev gcc git


# install python dependencies
COPY ./pyproject.toml /
COPY ./poetry.lock /
RUN pip install poetry
RUN poetry config virtualenvs.create false
RUN poetry install --compile

COPY ./app /app

EXPOSE 50051

CMD ["poetry", "run", "app"]

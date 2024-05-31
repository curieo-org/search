# pull official base image
FROM python:3.12-slim

# set working directory
WORKDIR /usr/src/

# install system dependencies
RUN apt-get update && apt-get -y install libpq-dev gcc

# add app
COPY . /usr/src/.

# install python dependencies
RUN pip install --upgrade pip
RUN pip install poetry uvicorn
RUN poetry install --no-root

EXPOSE 8080

CMD ["poetry", "run", "uvicorn", "app.main:app", "--reload", "--port", "8080"]
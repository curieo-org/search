# pull official base image
FROM nvcr.io/nvidia/tritonserver:22.07-py3

# set working directory
WORKDIR /usr/src/

# install system dependencies
RUN apt-get update && apt-get -y install libpq-dev gcc

# add app
COPY . /usr/src/.

# install python dependencies
RUN pip install --upgrade pip
RUN pip install poetry uvicorn packaging filelock
RUN poetry config virtualenvs.create false
RUN poetry install --compile --no-root

EXPOSE 8080

CMD ["poetry", "run", "uvicorn", "app.main:app", "--reload", "--port", "8080"]
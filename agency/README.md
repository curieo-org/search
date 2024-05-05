# ⚕️🧬🔬Curieo Healthcare Search API 🔍

![Current Version](https://img.shields.io/badge/version-v0.1-blue)
![GitHub Org's stars](https://img.shields.io/github/stars/curieo-org)
![Website](https://img.shields.io/website?url=http%3A%2F%2Fcurieo.org%2F)

Welcome to Curieo Search, the search engine on a mission to revolutionize how you access
healthcare information. Our
goal is simple yet ambitious: to be the best healthcare search engine available,
offering unparalleled access to
accurate, up-to-date, and relevant medical information.

## Table of Contents

- [⚕️🧬🔬Curieo Healthcare Search API 🔍](#️curieo-healthcare-search-api-)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Running the App](#running-the-app)
  - [Deployment](#deployment)

## Installation

- Install Curieo Agency using Docker
    - Make sure Docker is installed.
    - Build the Docker image `docker build -t curieo-agency .`
    - Run the Docker container `docker run -p 50051:50051 -d agency`

- Install Cureio Agency without Docker
    - Make sure `pyenv` is installed.
    - Create a new virtual environment `python -m venv .venv`
    - Activate the env `source .venv/bin/activate`
    - Install poetry and uvicorn `pip install poetry`
    - Install dependencies `poetry install`


- Set up Embedding Server
  from [TEI](https://github.com/huggingface/text-embeddings-inference/tree/main)

    - Setup for Re-rankers models

  ```
  model=BAAI/bge-reranker-large
  revision=refs/pr/4
  volume=$PWD/data # share a volume with the Docker container to avoid downloading weights every run

  docker run --gpus all -p 8080:80 -v $volume:/data --pull always ghcr.io/huggingface/text-embeddings-inference:1.1 --model-id $model --revision $revision
  ```

  The Output:

  ```
  curl 127.0.0.1:8080/rerank \
  -X POST \
  -d '{"query":"What is Deep Learning?", "texts": ["Deep Learning is not...", "Deep learning is..."]}' \
  -H 'Content-Type: application/json'
  ```

    - Using for Text Embeddings

  ```
  model=BAAI/bge-large-en-v1.5
  revision=refs/pr/5
  volume=$PWD/data # share a volume with the Docker container to avoid downloading weights every run

  docker run --gpus all -p 8081:80 -v $volume:/data --pull always ghcr.io/huggingface/text-embeddings-inference:1.1 --model-id $model --revision $revision
  ```

  The Output:

  ```
  curl 127.0.0.1:8080/embed \
  -X POST \
  -d '{"inputs":"What is Deep Learning?"}' \
  -H 'Content-Type: application/json'
  ```

  If you want use without the docker the do the following steps:

  ```
  git clone git@github.com:huggingface/text-embeddings-inference.git
  cd text-embeddings-inference

  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # On x86
  cargo install --path router -F candle -F mkl
  # On M1 or M2
  cargo install --path router -F candle -F metal

  model=BAAI/bge-large-en-v1.5
  revision=refs/pr/5
  text-embeddings-router --model-id $model --revision $revision --port 8081

  model=BAAI/bge-reranker-large
  revision=refs/pr/4
  text-embeddings-router --model-id $model --revision $revision --port 8080
  ```

    - Clone the [Data Digger](https://github.com/curieo-org/data-digger) repository and
      follow the instructions to set
      up the aact postgresql database and chembl nebula graph database.
    - Add local redis engine
    - Alternatively, you can use the existing deployed databases for all of the above if
      they are available.
    - Copy the `.env.template` file to `.env` and update the required values.

## Running the App

Steps and commands for running the app are to be included here

- Use the below command to execute the server:

  ```
  poetry run app
  ```

## Deployment

Not decided yet, we will work on that part.
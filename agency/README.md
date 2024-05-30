# Agency

## Table of Contents
- [Agency](#agency)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Technologies](#technologies)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Deployment](#deployment)

## Introduction
Agency is a collection of multiple RAG modules. Every module is responsible for computing a the search result for a specific data source. All of the modules are exposed via a gRPC server.

## Technologies
1. Python
2. gRPC Server
3. Llama Index
4. Qdrant Vector Database
5. Postgres Database
6. Nebula Graph Database

## Prerequisites
1. Python 3
2. Embedding LLM
3. Splade Embedding LLM
4. Summarization LLM
5. Reranker LLM
6. Clinical Trials
   1. Clinical Trials PostgresDB
   2. Clinical Trials QdrantDB
   3. SQL LLM Model
7. Drug Discovery
   1. Drug Nebula Graphdb
   2. Cypher LLM Model
8. Web Search
   1. Pubmed PostgresDB
   2. Pubmed QdrantDB

## Installation
- Install Curieo Agency using Docker
    - Make sure Docker is installed.
    - Build the Docker image `docker build -t curieo-agency .`
    - Run the Docker container `docker run -p 50051:50051 -d curieo-agency`

- Install Cureio Agency without Docker
    - Make sure `pyenv` is installed.
    - Create a new virtual environment `python3 -m venv .venv`
    - Activate the env `source .venv/bin/activate`
    - Install poetry and uvicorn `pip install poetry`
    - Install dependencies `poetry install`

## Usage
```bash
# configure the environment variables
cp .env.template .env

# start the server
poetry run app
```

## Deployment
```bash
# Update the following line in the Makefile for the TAG
TAG = 

# Build and upload the image to the ECR
make -f Makefile_ECR

# Change the TAG in the helm/values.yaml file in the root directory
image: 698471419283.dkr.ecr.eu-central-1.amazonaws.com/curieo-agency:<TAG>

# Create a new deployment in the Kubernetes from the root directory
helm install agency ./helm -n <NAMESPACE> --values ./helm/values.yaml

# Update the already existing deployment in the Kubernetes from the root directory
helm upgrade agency ./helm -n <NAMESPACE> --values ./helm/values.yaml
```

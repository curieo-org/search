# ⚕️🧬🔬Curieo Healthcare Search Backend Project 🔍

![Current Version](https://img.shields.io/badge/version-v0.1-blue)
![GitHub Org's stars](https://img.shields.io/github/stars/curieo-org)
![Website](https://img.shields.io/website?url=http%3A%2F%2Fcurieo.org%2F)

Welcome to Curieo Search, the search engine on a mission to revolutionize how you access healthcare information. Our goal is simple yet ambitious: to be the best healthcare search engine available, offering unparalleled access to accurate, up-to-date, and relevant medical information.

## Table of Contents

-   [Getting Started](#getting-started)
    -   [Tools Required](#tools-required)
    -   [Installation](#installation)
-   [Running the App](#running-the-app)
-   [Deployment](#deployment)
-   [Versioning](#versioning)

## Getting Started

The project might have multiple branches: `master`, `development`, etc. which can be explained here

-   `master` contains aggregate code of all branches
-   `development` contains code under development

```
	search
	├── README.md
	├── package.json
	├── .gitignore
	├── public
	│   ├── favicon.ico
	│   ├── index.html
	│   └── manifest.json
	├── app
	│   ├── api
	│   │	├── common
	│   │	├── endpoints
	│   │	│	└── search_endpoint.py
	│   │	├── errors
	│   │	│	├──── http_error.py
	│   │	│	└──── if_none_match.py
	│   │	├── router
	│   │	│	└──── api.py
	│   ├── middleware
	│   │		└── process_time.py
	│   ├── rag
	│   │	├── augmentation
	│   │	├── ensemble
	│   │	├── generation
	│   │	│	└──── response_synthesis.py
	│   │	├── reranker
	│   │	│	└──── response_reranker.py
	│   │	├── retrieval
	│   │	│	├──── clinical_trials
	│   │	│	│	├──── AACTTableQuestions_TableInfo
	│   │	│	│	└──── clinical_trial_sql_query_engine.py
	│   │	│	├──── pubmed
	│   │	│	│	└──── pubmedqueryengine.py
	│   │	│	├──── drug
	│   │	│	└──── web
	│   │	│	│	└──── brave_search.py
	│   ├── router
	│   │	└──── orchestrator.py
	│   ├── services
	│   │	└──── search_utility.py
	│   ├── tests
	│   ├── config.py
	│   └── main.py
	├── docker
	│   │	└────Dockerfile.local
	├── requirements
	│   ├── base.txt
	│   └── development.txt
	├── scripts
	│   └── docker-entrypoint.sh
	├── .editorconfig
	├── .gitignore
 	├── docker-compose.yml
   	├── poetry.lock
    	├── pytest.ini
     	└── README.md

```

### Tools Required

-   A text editor or an IDE (VsCode is Preferred)

### Installation

All installation steps go here.

-   Install `poetry` for the dependencies. - `poetry install`

-   Set up Embedding Server from [TEI](https://github.com/huggingface/text-embeddings-inference/tree/main)

    -   Setup for Re-rankers models

    ```
    model=BAAI/bge-large-en-v1.5
    revision=refs/pr/5
    volume=$PWD/data # share a volume with the Docker container to avoid downloading weights every run

    docker run --gpus all -p 8080:80 -v $volume:/data --pull always ghcr.io/huggingface/text-embeddings-inference:1.1 --model-id $model --revision $revision
    ```

    -   Using for Text Embeddings

    ```
    model=BAAI/bge-reranker-large
    revision=refs/pr/4
    volume=$PWD/data # share a volume with the Docker container to avoid downloading weights every run

    docker run --gpus all -p 8080:80 -v $volume:/data --pull always ghcr.io/huggingface/text-embeddings-inference:1.1 --model-id $model --revision $revision
    ```

    -   Steps to complete it

-   Installing another tool

## Running the App

Steps and commands for running the app are to be included here

-   Example steps:
    ```
      Example command
    ```

## Deployment

This section is completely optional. Add additional notes about how to deploy this on a live system

## Versioning

`0.0.1` :

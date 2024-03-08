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

    The Output:

    ```
    curl 127.0.0.1:8080/rerank \
    -X POST \
    -d '{"query":"What is Deep Learning?", "texts": ["Deep Learning is not...", "Deep learning is..."]}' \
    -H 'Content-Type: application/json'
    ```

    -   Using for Text Embeddings

    ```
    model=BAAI/bge-reranker-large
    revision=refs/pr/4
    volume=$PWD/data # share a volume with the Docker container to avoid downloading weights every run

    docker run --gpus all -p 8080:80 -v $volume:/data --pull always ghcr.io/huggingface/text-embeddings-inference:1.1 --model-id $model --revision $revision
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
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # On x86
    cargo install --path router -F candle -F mkl
    # On M1 or M2
    cargo install --path router -F candle -F metal

    model=BAAI/bge-large-en-v1.5
    revision=refs/pr/5

    text-embeddings-router --model-id $model --revision $revision --port 8080
    ```

    -   Add postgresql database and create the engine.
    -   Add local redis engine and change in the config.py file

## Running the App

Steps and commands for running the app are to be included here

-   First complete the login process:

    ```
      curl --location 'http://127.0.0.1:8000/login?username=curieo&password=curieo' --header 'accept: application/json'
    ```

    You will receive the access token.

-   Use the token to retrieve the search results.

    -   Example API for Clinical Trial Request:

        ```
        curl --location 'http://127.0.0.1:8000/Search?query=What is diseases associated with A Natural History Study of Canavan Disease' \
        --header 'Authorization: Bearer <access token>'
        ```

    -   Example API for Search:
        ```
        curl --location 'http://127.0.0.1:8000/Search?query=Are there any recent regulatory updates or guidance documents related to CMC requirements for biopharmaceutical products similar to mRNA vaccine for covid' \
        --header 'Authorization: Bearer <access token>'
        ```

## Deployment

Not decided yet, we will work on that part.

## Versioning

`0.0.1`

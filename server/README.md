# Search Server

## Table of Contents
- [Search Server](#search-server)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Technologies](#technologies)
  - [API Documentation](#api-documentation)
  - [Prerequisites](#prerequisites)
  - [Usage](#usage)
  - [Contribution](#contribution)
  - [Deployment](#deployment)

## Introduction
Search server is responsible for serving all the search APIs and communication with the respective modules to compute the results. It also communicates with different LLM models and database engines to fetch the results.

## Technologies
1. Rust
2. Axum Web Framework
3. Postgres Database
4. Redis Cache
5. gRPC Client

## API Documentation
- [API Documentation](./documentation/search-api.md)

## Prerequisites
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Postgres](https://www.postgresql.org/download/)
3. [Redis](https://redis.io/docs/latest/operate/oss_and_stack/install/install-redis/)
4. Create a database: `CREATE DATABASE curieo_search;`

## Usage

In order for the server to work, you need to configure the services that the services needs. 
These services are (at least) the following. In the `.config` folder you find `.toml` files with the services required.

- the search application (`Agency`)
- llm 
	- prompt compression (LLM lingua)
	- toxicity predictor 
	- query rephraser
- cache (REDIS)

Additionally, in the `.env` file, some services get additional configuration.

```bash
# setup the environment and configure
cp .env.template .env

# Copy the environment variables from the aws secret file
# https://eu-central-1.console.aws.amazon.com/secretsmanager/secret?name=search-server-development-env&region=eu-central-1
# and turn on curieo vpn
```

- database (see below)
- authorization tokens
	- brave
	- toxicity predictor
	- query rephraser 
	- open ai 

```
# use the following url for database migration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/curieo_search
```

To locally compile and run, optionally, you need to create new tables or schemas.

Maintain current ordering in the `migrations` directory; making sure that the order of schema creation is not create error in the database.

```bash
sqlx migrate add schema_name

# configure the database
sqlx migrate run

# start server (production)
cargo run --release

# start server (development)
cargo watch -x run
```

## Contribution
```bash
# Run and push changes after adding new sqlx query.
cargo sqlx prepare
```

## Deployment
```bash
# Update the following line in the Makefile for the TAG
TAG = 

# Build and upload the image to the ECR
make ecr_deploy

# Change the TAG in the helm/values.yaml file in the root directory
image: 698471419283.dkr.ecr.eu-central-1.amazonaws.com/curieo-search-server:<TAG>

# Create a new deployment in the Kubernetes from the root directory
helm install search-server ./helm -n <NAMESPACE> --values ./helm/values.yaml

# Update the already existing deployment in the Kubernetes from the root directory
helm upgrade search-server ./helm -n <NAMESPACE> --values ./helm/values.yaml
```

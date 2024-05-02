# API Documentation

## Table of Contents

- [API Documentation](#api-documentation)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Endpoints](#endpoints)
    - [Authentication](#authentication)
      - [POST Register](#post-register)
    - [Params](#params)
      - [POST Login](#post-login)
    - [Params](#params-1)
      - [GET Search](#get-search)
    - [Params](#params-2)
  - [GET Search History](#get-search-history)
    - [Params](#params-3)
  - [PATCH Update Search Reaction](#patch-update-search-reaction)
    - [Params](#params-4)
  - [Get One Search History](#get-one-search-history)
    - [Params](#params-5)
  - [GET Top Searches](#get-top-searches)
    - [Params](#params-6)

## Introduction

This API documentation provides a detailed description of the API endpoints request and response format, error handling, and rate limiting. This will cover all endpoints of the search project.

## Endpoints

Base URLs:

- <a href="http://127.0.0.1:8000">Local Dev: http://127.0.0.1:8000</a>

### Authentication

#### POST Register

POST /auth/register

### Params

| Name     | Location              | Type   | Required | Description          |
| -------- | --------------------- | ------ | -------- | -------------------- |
| email    | x-www-form-urlencoded | string | yes      | unique email address |
| password | x-www-form-urlencoded | string | yes      | strong password      |

> Response Examples

```json
{
  "user_id": "12b48e74-f596-11ee-85a9-f3d0e85b3083",
  "email": "username@email.com",
}
```

#### POST Login

POST /auth/login

### Params

| Name     | Location              | Type   | Required | Description         |
| -------- | --------------------- | ------ | -------- | ------------------- |
| email | x-www-form-urlencoded | string | yes      | registered email |
| password | x-www-form-urlencoded | string | yes      | correct password    |

> Response Examples

```json

```

#### GET Search

GET /search

### Params

| Name       | Location | Type   | Required | Description               |
| ---------- | -------- | ------ | -------- | ------------------------- |
| query      | query    | string | yes      | search query              |
| session_id | query    | string | no       | current session id if any |

> Response Examples

```json
{
  "search_history_id": "1e07e80c-f596-11ee-85a9-c30d0dd94485",
  "user_id": "12b48e74-f596-11ee-85a9-f3d0e85b3083",
  "session_id": "c4641ed5-8e8d-4e4a-bccb-b11f309cb5db",
  "query": "What is aspirin",
  "result": "Aspirin is a medicine",
  "sources": [
    {
      "url": "https://www.webmd.com/baby/drug-use-and-pregnancy",
      "metadata": {
        "page_age": "2021-08-18T00:00:00"
      }
    },
    {
      "url": "https://www.stanfordchildrens.org/en/topic/default?id=illegal-drug-use-and-pregnancy-85-P01208",
      "metadata": {
        "helper_text": ""
      }
    }
  ],
  "reaction": null,
  "created_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0],
  "updated_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0]
}
```

## GET Search History

GET /search/history

### Params

| Name   | Location | Type   | Required | Description          |
| ------ | -------- | ------ | -------- | -------------------- |
| limit  | query    | string | no       | maximum result       |
| offset | query    | string | no       | offset of the result |

> Response Examples

```json
[
  {
    "search_history_id": "1e07e80c-f596-11ee-85a9-c30d0dd94485",
    "user_id": "12b48e74-f596-11ee-85a9-f3d0e85b3083",
    "session_id": "c4641ed5-8e8d-4e4a-bccb-b11f309cb5db",
    "query": "What is aspirin",
    "result": "Aspirin is a medicine",
    "sources": [
      {
        "url": "https://www.webmd.com/baby/drug-use-and-pregnancy",
        "metadata": {
          "page_age": "2021-08-18T00:00:00"
        }
      },
      {
        "url": "https://www.stanfordchildrens.org/en/topic/default?id=illegal-drug-use-and-pregnancy-85-P01208",
        "metadata": {
          "helper_text": ""
        }
      }
    ],
    "reaction": null,
    "created_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0],
    "updated_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0]
  },
  {
    "search_history_id": "2e07e80c-f596-11ef-85a8-c30d0dd94486",
    "user_id": "12b48e74-f596-11ee-85a9-f3d0e85b3083",
    "session_id": "c4641ed5-8e8d-4e4a-bccb-b11f309cb5db",
    "query": "What is imatinib",
    "result": "Imatinib is a medicine",
    "sources": [
      {
        "url": "https://www.webmd.com/baby/drug-use-and-pregnancy",
        "metadata": {
          "page_age": "2021-08-18T00:00:00"
        }
      },
      {
        "url": "https://www.stanfordchildrens.org/en/topic/default?id=illegal-drug-use-and-pregnancy-85-P01208",
        "metadata": {
          "helper_text": ""
        }
      }
    ],
    "reaction": null,
    "created_at": [2024, 99, 10, 52, 39, 279568001, 0, 0, 0],
    "updated_at": [2024, 99, 10, 52, 39, 279568002, 0, 0, 0]
  }
]
```

## PATCH Update Search Reaction

PATCH /search/reaction

### Params

| Name              | Location | Type    | Required | Description       |
| ----------------- | -------- | ------- | -------- | ----------------- |
| body              | body     | object  | no       | none              |
| search_history_id | body     | string  | yes      | search history id |
| reaction          | body     | boolean | yes      | user reaction     |

> Response Examples

```json
{
  "search_history_id": "1e07e80c-f596-11ee-85a9-c30d0dd94485",
  "user_id": "12b48e74-f596-11ee-85a9-f3d0e85b3083",
  "session_id": "c4641ed5-8e8d-4e4a-bccb-b11f309cb5db",
  "query": "What is aspirin",
  "result": "Aspirin is a medicine",
  "sources": [
    {
      "url": "https://www.webmd.com/baby/drug-use-and-pregnancy",
      "metadata": {
        "page_age": "2021-08-18T00:00:00"
      }
    },
    {
      "url": "https://www.stanfordchildrens.org/en/topic/default?id=illegal-drug-use-and-pregnancy-85-P01208",
      "metadata": {
        "helper_text": ""
      }
    }
  ],
  "reaction": null,
  "created_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0],
  "updated_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0]
}
```

## Get One Search History

GET /search/one

### Params

| Name  | Location | Type   | Required | Description |
| ----- | -------- | ------ | -------- | ----------- |
| search_history_id | body     | string  | yes      | search history id |

> Response Examples

```json
{
  "search_history_id": "1e07e80c-f596-11ee-85a9-c30d0dd94485",
  "user_id": "12b48e74-f596-11ee-85a9-f3d0e85b3083",
  "session_id": "c4641ed5-8e8d-4e4a-bccb-b11f309cb5db",
  "query": "What is aspirin",
  "result": "Aspirin is a medicine",
  "sources": [
    {
      "url": "https://www.webmd.com/baby/drug-use-and-pregnancy",
      "metadata": {
        "page_age": "2021-08-18T00:00:00"
      }
    },
    {
      "url": "https://www.stanfordchildrens.org/en/topic/default?id=illegal-drug-use-and-pregnancy-85-P01208",
      "metadata": {
        "helper_text": ""
      }
    }
  ],
  "reaction": null,
  "created_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0],
  "updated_at": [2024, 99, 10, 52, 39, 279568000, 0, 0, 0]
}
```

## GET Top Searches

GET /search/top

### Params

| Name  | Location | Type   | Required | Description |
| ----- | -------- | ------ | -------- | ----------- |
| limit | query    | string | yes      | none        |

> Response Examples

```json
["What is aspirin", "What is imatinib"]
```

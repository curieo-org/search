-- Creating a table for searches
CREATE TABLE searches
(
    search_id   uuid primary key            default uuid_generate_v1mc(),
    thread_id   uuid            not null    references threads (thread_id),
    query       varchar(255)    not null,
    result      text            not null,
    media_urls  text[]          not null,
    reaction    boolean,
    created_at  timestamptz     not null    default now(),
    updated_at  timestamptz     not null    default now()
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('searches');

-- And creating an index on `thread_id` to make it easier to find all searches for a given thread
CREATE INDEX searches_thread_id ON searches (thread_id);

-- And creating a table for sources
CREATE TABLE sources
(
    source_id       uuid primary key                default uuid_generate_v1mc(),
    url             varchar(255) unique not null,
    title           varchar(255)        not null,
    description     text,
    source_type     integer             not null,
    metadata        jsonb,
    created_at      timestamptz         not null    default now(),
    updated_at      timestamptz         not null    default now()
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('sources');

-- And creating an index on `url` to make it easier to find a source by its URL
CREATE INDEX sources_url ON sources (url);

-- And creating a join table for searches and sources
CREATE TABLE search_sources
(
    search_source_id    uuid primary key        default uuid_generate_v1mc(),
    search_id           uuid        not null    references searches (search_id),
    source_id           uuid        not null    references sources (source_id),
    created_at          timestamptz not null    default now(),
    updated_at          timestamptz not null    default now(),

    unique (search_id, source_id)
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('search_sources');

-- And creating an index on `search_id` and `source_id` to make it easier to find all sources for a given search
CREATE INDEX search_sources_search_id ON search_sources (search_id);
CREATE INDEX search_sources_source_id ON search_sources (source_id);

-- Creating a table for collections
CREATE TABLE collections
(
    collection_id   uuid primary key            default uuid_generate_v1mc(),
    user_id         uuid            not null    references users (user_id),
    name            varchar(255)    not null,
    description     varchar(1000),
    category        integer         not null,
    context         jsonb,
    metadata        jsonb,
    created_at      timestamptz     not null    default now(),
    updated_at      timestamptz     not null    default now()
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('collections');

-- And creating an index on `user_id` to make it easier to find all collections for a given user
CREATE INDEX collections_user_id ON collections (user_id);

-- Creating a join table for collections and sources
CREATE TABLE collection_sources
(
    collection_source_id    uuid primary key        default uuid_generate_v1mc(),
    collection_id           uuid        not null    references collections (collection_id),
    source_id               uuid        not null    references sources (source_id),
    created_at              timestamptz not null    default now(),
    updated_at              timestamptz not null    default now(),

    unique (collection_id, source_id)
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('collection_sources');

-- And creating an index on `collection_id` and `source_id` to make it easier to find all sources for a given collection
CREATE INDEX collection_sources_collection_id ON collection_sources (collection_id);
CREATE INDEX collection_sources_source_id ON collection_sources (source_id);

-- And creating an join table for collections and searches
CREATE TABLE collection_searches
(
    collection_search_id    uuid primary key        default uuid_generate_v1mc(),
    collection_id           uuid        not null    references collections (collection_id),
    search_id               uuid        not null    references searches (search_id),
    created_at              timestamptz not null    default now(),
    updated_at              timestamptz not null    default now(),

    unique (collection_id, search_id)
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('collection_searches');

-- And creating an index on `collection_id` and `search_id` to make it easier to find all searches for a given collection
CREATE INDEX collection_searches_collection_id ON collection_searches (collection_id);
CREATE INDEX collection_searches_search_id ON collection_searches (search_id);

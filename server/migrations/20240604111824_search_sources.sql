-- Creating a join table for searches and sources
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

-- Creating a table for sources
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

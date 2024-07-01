-- Creating a table for searches
CREATE TABLE searches
(
    search_id       uuid primary key            default uuid_generate_v1mc(),
    thread_id       uuid            not null    references threads (thread_id),
    query           varchar(400)    not null,
    rephrased_query varchar(400)    not null,
    result          text            not null,
    media_urls      text[],
    reaction        boolean,
    created_at      timestamptz     not null    default now(),
    updated_at      timestamptz     not null    default now()
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('searches');

-- And creating an index on `thread_id` to make it easier to find all searches for a given thread
CREATE INDEX searches_thread_id ON searches (thread_id);

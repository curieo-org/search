-- Creating a table for threads
CREATE TABLE threads
(
    thread_id   uuid    primary key         default uuid_generate_v1mc(),
    user_id     uuid            not null    references users (user_id),
    title       varchar(255)    not null,
    context     jsonb,
    created_at  timestamptz     not null    default now(),
    updated_at  timestamptz     not null    default now()
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('threads');

-- And creating an index on `user_id` to make it easier to find all threads for a given user
CREATE INDEX threads_user_id ON threads (user_id);

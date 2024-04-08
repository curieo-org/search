create table search_history (
    search_history_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null references users(user_id),
    session_id uuid not null,
    query text not null,
    result text not null,
    sources text[] not null,
    reaction boolean,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('search_history');

-- And creating an index on `session_id` to make it easier to find all searches for a given session
CREATE INDEX search_history_session_id ON search_history(session_id);

create table search_history (
    search_history_id uuid primary key default uuid_generate_v1mc(),
    search_text text not null,
    response_text text not null,
    response_sources text[] not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('search_history');

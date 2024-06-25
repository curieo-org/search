create table users (
    -- Having the table name as part of the primary key column makes it nicer to write joins, e.g.:
    --
    -- select * from users
    -- inner join article using (user_id)
    --
    -- as opposed to `inner join article on article.user_id = user.id`, and makes it easier to keep track of primary
    -- keys as opposed to having all PK columns named "id"
    user_id         uuid primary key                                default uuid_generate_v1mc(),
    username        text collate "case_insensitive" unique not null,
    email           text collate "case_insensitive" unique not null,
    fullname        varchar(50),
    title           varchar(50),
    company         varchar(50),
    user_group      integer                                not null default 0,
    password_hash   text,
    access_token    text,
    created_at      timestamptz                            not null default now(),
    updated_at      timestamptz
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('users');

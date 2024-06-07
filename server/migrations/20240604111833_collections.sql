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

-- Creating a join table for collections and items
CREATE TABLE collection_items
(
    collection_item_id      uuid primary key        default uuid_generate_v1mc(),
    collection_id           uuid        not null    references collections (collection_id),
    item_id                 uuid        not null,
    item_type               integer     not null,

    created_at              timestamptz not null    default now(),
    updated_at              timestamptz not null    default now(),

    unique (collection_id, item_id)
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('collection_items');

-- And creating an index on `collection_id` and `item_id` to make it easier to find all items for a given collection
CREATE INDEX collection_items_collection_id ON collection_items (collection_id);
CREATE INDEX collection_items_collection_id_item_type ON collection_items (collection_id, item_type);



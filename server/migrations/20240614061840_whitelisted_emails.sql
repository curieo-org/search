-- Add migration script here
CREATE TABLE whitelisted_emails
(
    email           text collate "case_insensitive" primary key,
    approved        boolean      not null           default false
);
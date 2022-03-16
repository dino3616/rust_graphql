-- Your SQL goes here
create table users(
    id serial not null primary key,
    name varchar(255) not null,
    profile text,
    created_at timestamp with time zone not null default current_timestamp,
    updated_at timestamp with time zone not null default current_timestamp
);
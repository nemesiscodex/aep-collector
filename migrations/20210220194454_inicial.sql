-- Add migration script here

create table collector (
    id serial primary key,
    created_at timestamp default now(),
    frame varchar(64) not null
);
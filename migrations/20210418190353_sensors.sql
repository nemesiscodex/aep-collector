-- Add migration script here

ALTER TABLE collector RENAME TO collector_activation;

create table collector_sensors (
    id serial primary key,
    created_at timestamp default now(),
    precipitation numeric not null,
    wind_velocity numeric not null,
    wind_direction numeric not null,
    humidity numeric not null,
    exterior_temperature numeric not null,
    interior_temperature numeric not null,
    pressure numeric not null
);

create table collector_frames (
    id serial primary key,
    created_at timestamp default now(),
    satellite varchar(128) not null
);
# AEP Collector
Data collector service.

## Installation
- Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Install dependencies
```
apt-get install -y pkg-config openssl libssl-dev libpq-dev
``` 
- Start postgres database:
    - With docker compose:
    
        Create a docker-compose.yml file:
    ```
    version: "3"
    services:
        postgres:
            image: postgis/postgis:13-3.1-alpine
            restart: always
            environment:
                POSTGRES_PASSWORD: aep
                POSTGRES_USER: aep
                POSTGRES_DB: aep
            ports:
                - 5432:5432
            volumes:
                - pgdata:/var/lib/postgresql/data
    ``` 
    Start postgres 
    ```
    docker-compose up -d postgres
    ``` 
- Install sqlx client
```
cargo install sqlx-cli --no-default-features --features postgres
```  
- Run migrations:
```
export DATABASE_URL=postgres://aep:aep@127.0.0.1:5432/aep
sqlx migrate run
```
- Start the collector:
```
export RUST_LOG=debug 
cargo run --release 
```

## About the project
### Chagas Disease
In Paraguay, as well as in other Latin American countries, [Chagas disease](https://en.wikipedia.org/wiki/Chagas_disease)
is one of the pressing issues in the area of public health.

This disease is caused by the parasite [Trypanosoma cruzi](https://en.wikipedia.org/wiki/Trypanosoma_cruzi), being [Triatoma infestans](https://en.wikipedia.org/wiki/Triatoma_infestans) insect or commonly called vinchuca (kissing bug), the main vector of South America.

Currently, there is a concern in European countries and on the United States, due to the growing eco-tourism to South American countries, where Chagas disease is referred to as an "exotic disease".
Prevention mostly involves eliminating kissing bugs and avoiding their bites. **A vaccine has not been developed as of 2019**.

Treatment options for infected patients are limited.
Early infections are treatable with the medication if given early, but becomes less effective the longer a person has had Chagas disease.

Most people with the disease live in poverty, and do not realize they are infected.

#### Eradication of Chagas Disease with Technology
> A project in conjunction with the [Center for the Development of Scientific Research (CEDIC)](https://www.cedicpy.com/) and the [Polytechnic School of the National University of Asunción](http://www.fpuna.edu.py/).

The monitoring of the vector transmitting Chagas disease will be possible through the implementation of a wireless infrared photoelectric sensor network for remote early detection of kissing bugs infestation in a surveillance zone.
Readings of the sensor network will be sent from a ground station to the research lab located in the Polytechnic School using a [satellite](https://birds4.birds-project.com/).
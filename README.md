# Buckpal

## What is this? 

This is an attempt at writing the example from Get Your Hands Dirty On Clean Architecture. The original examples in the book are in Java, but are here in Rust.
Interfaces are replaced with traits, and most dependencies are injected. The exception is where I experimented with Lazy Static for the database.
Completed:
- Send money `POST /accounts/send/{fromId}/{toId}/{amount}/``
- Account balance `GET /accounts/{id}/balance`

Todo: 
- Create an account `POST /accounts/`
- List accounts `Get /accounts/`
- Get account `Get /accounts/{id}`
- Auth

Will I do these? Who knows!

## Running it

You will need:
1. Rust
2. Sqlite3

To run: 
1. `./init_db.sh` to copy from `init.sql` into `test.db`
2. `cargo run` to run rust server

Then you can check out the completed endpoints:e.g. `GET localhost:8080/accounts/2/balance` or `POST localhost:8080/accounts/send/1/2/300`
There are only accounts of id 1,2,3 -- but you can modify this in the init.sql

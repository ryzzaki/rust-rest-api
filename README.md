## Setup

```bash
# create .env
$ touch .env

# add the config to .env
$ cat .env.example > .env

# use the nightly build of Rust - but this should already be done by rust-toolchain
$ rustup override set nightly
```

## Diesel

```bash
# to up a migration
$ diesel migration run

# to down a migration
$ diesel migration redo

# to generate a new migration
$ diesel migration generate "migration_name"

# if migrations are gone for some reason
$ diesel setup
```

## Compile & Run

```bash
# run
$ cargo run

# build
$ cargo build
```

## Use stable on Rocket

If experimental features make it to stable, then use stable Rust with Rocket:

```bash
$ rustup update stable
```

## Example Requests using cURL

```bash
# create a new user
curl --request POST --header "Content-Type: application/json" --data '{ "username": "mr_anderson", "password": "wick", "first_name": "john" }' http://localhost:8000/api/v1/users

# get all users
curl http://localhost:8000/api/v1/users

# get user by username
curl --request POST --header "Content-Type: application/json" --data '{ "username": "mr_anderson" }' http://localhost:8000/api/v1/users/get
```

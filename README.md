# Market API Server

## Setup

Create `.env` file:

```
DATABASE_URL=sqlite://./db.sqlite
```

Install sqlx-cli:

```
cargo install sqlx-cli --no-default-features --features sqlite
```

Create database and run migrations:

```
sqlx database create
sqlx migrate run
```

## Build

```
cargo build
# or
cargo build --release
```

## Run

```
cargo run
# or
cargo run --release
```

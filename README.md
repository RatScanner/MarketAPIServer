[![Build Status](https://github.com/RatScanner/MarketAPIServer/workflows/test/badge.svg)](https://github.com/RatScanner/MarketAPIServer/actions)
[![dependency status](https://deps.rs/repo/github/RatScanner/MarketAPIServer/status.svg)](https://deps.rs/repo/github/RatScanner/MarketAPIServer)
[![Lines Of Code](https://tokei.rs/b1/github/RatScanner/MarketAPIServer?category=code)](https://github.com/RatScanner/MarketAPIServer)

# Market API Server

## Setup

Create `.env` file:

```
DATABASE_URL=postgresql://<DB_USER>:<DB_USER_PASSWORD>@localhost/<DB_NAME>
AUTH_KEY=<MY_SECURE_KEY>
ENVIRONMENT=<Production|Development>
# SERVICE=false <-- Disable fetch service, enabled by default
```

Install sqlx-cli:

```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
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

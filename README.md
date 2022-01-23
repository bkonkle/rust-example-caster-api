# Rust Async-GraphQL Example: Caster API

[<img alt="Rust" src="https://img.shields.io/badge/rust-2021-a72145?logo=rust&style=flat" />](https://www.rust-lang.org/)

This is an example app for the upcoming Rust video series by [Brandon Konkle](https://github.com/bkonkle). It implements a basic API to support a number of hypothetical frontends for the imaginary "Caster" app, a tool to help podcasters, broadcasters, and streamers coordinate show content with their co-hosts and guests. Limited to just the API to support the front end.

## Local Development

Install Rust with [rustup](https://rustup.rs/).

### Clippy

For helpful linting rools, install [Clippy](https://github.com/rust-lang/rust-clippy) with `rustup`:

```sh
rustup component add clippy
```

Run it with `cargo`:

```sh
cargo clippy --fix
```

Configure the `rust-analyzer` VS Code plugin to use it (in _settings.json_):

```json
{
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

### Cargo Make

To build scripts from the _Makefile.toml_, install Cargo Make:

```sh
cargo install cargo-make
```

### Running Docker

To run the docker-compose formation for the API app:

```sh
cargo make docker-api up -d
```

### SQLx CLI

Install the SQLx CLI for running migrations:

```sh
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Create a database based on the `DATABASE_URL` in the `.env`, if you haven't already:

```sh
cargo make db-create
```

Run migrations:

```sh
cargo make db-migrate
```

If you want to wipe your database and start over:

```sh
cargo make db-reset
```

### Running the Local Server

Use `cargo` to run the server locally:

```sh
cargo run
```

### Update Dependencies

First, install the `outdated` command for `cargo`:

```sh
cargo install --locked cargo-outdated
```

Then, update and check for any major dependency changes:

```sh
cargo update
cargo outdated
```

### Running Integration Tests

To integration test, you need to have the Docker Compose stack with Postgres and Redis running locally, or within your CI pipeline.

NOTE: This is destructive, and will wipe out data within your local database. See below for how to use an alternate test database locally.

To run the integration tests:

```sh
cargo make integration
```

#### Using an Alternate Test Database

Running integration tests is destructive. If you want to preserve your local data, use an alternate database for local integration testing. Create a `config/test.toml` file and customize the `DATABASE_URL`:

```toml
[database]
name = "caster_rust_test"
url = "postgresql://caster:caster@localhost:1701/caster_rust_test"
```

Since the `RUN_MODE` environment variable is set by the `tasks.integration` make task to "test", this file will automatically be picked up by the config reader.

NOTE: To manage this test database with the SQLx CLI, you'll need to temporarily edit your `.env` file to match the values above, and then run the command to reset the test database:

```sh
cargo make db-reset
```

You can restore the original values in your `.env` afterwards.

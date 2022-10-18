# Rust Async-GraphQL Example: Caster API

[<img alt="Rust" src="https://img.shields.io/badge/rust-2021-a72145?logo=rust&style=flat" />](https://www.rust-lang.org)
[<img alt="GraphQL" src="https://img.shields.io/badge/graphql-e10098?logo=graphql&style=flat" />](https://graphql.org)
[<img alt="SeaORM" src="https://img.shields.io/badge/SeaORM-032846?logo=postgresql&style=flat" />](https://github.com/SeaQL/sea-orm)
[<img alt="Tokio" src="https://img.shields.io/badge/tokio-463103?logo=rust&style=flat" />](https://tokio.rs)

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

### Configuration

Configuration is unfortunately stored in two places, with the primary location being the [config](config/) folder. This folder contains hierarchical config files that are read by [Figment](https://github.com/SergioBenitez/Figment).

To set up your local environment, create a `local.toml` file and a `test.toml` file, using [`local.toml.example`](config/local.toml.example) and [`test.toml.example`](config/test.toml.example) as a guide.

The `local.toml` config is loaded by default in every run mode. In addition, an attempt to load a config file with the name of the run mode is also made - for example, `test.toml` when the `run_mode` is "test".

This config is read in as part of a [`lazy_static`](https://docs.rs/lazy_static/latest/lazy_static/) instance that is first initialized when the [`main.rs`](apps/api/src/main.rs) module from the `caster_api` app calls `caster_utils::config::get_config()`.

For CLI tools, however, we have to provide a small `.env` file with a subset of our config values so that tools like `docker-compose` and `sqlx-cli` can read them. Use the `.env.example` as a guide.

### Running Docker

To run the docker-compose formation with just the supporting services needed to run `cargo make dev`:

```sh
cargo make docker up -d
```

To shut it down:

```sh
cargo make docker down
```

To run docker-compose with the API app included:

```sh
cargo make docker-api up -d
```

To shut it down:

```sh
cargo make docker-api down
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

### Running the Local dev server

Use `cargo` to run the dev server locally:

```sh
cargo make dev
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

## Deployment

### Building Docker Containers Locally

To build locally, use Buildkit:

```sh
DOCKER_BUILDKIT=1 docker build -t caster-api -f apps/api/Dockerfile .
```

To clear the build cache:

```sh
docker builder prune --filter type=exec.cachemount
```

To inspect the local filesystem:

```sh
docker run --rm -it --entrypoint=/bin/bash caster-api
```

To inspect the full build context:

```sh
docker image build --no-cache -t build-context -f - . <<EOF
FROM busybox
WORKDIR /build-context
COPY . .
CMD find .
EOF

docker container run --rm build-context
```

And to clean up the build context test image:

```sh
docker image rm build-context
```

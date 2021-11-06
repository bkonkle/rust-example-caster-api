# Rust Async-GraphQL Example: Caster API

[<img alt="Rust" src="https://img.shields.io/badge/rust-2018-a72145?logo=rust&style=flat" />](https://www.rust-lang.org/)

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

### Running the Local Server

Use `cargo` to run the server locally:

```sh
cargo run
```

Welcome to the hidden world of Rust-based Postgres extensions.
We've got cake!


# Pre-requisites

* A relatively recent [Rust](https://rust-lang.org) toolchain.


# Steps

1. Install `cargo-pgx`, with `cargo install cargo-pgx --version 0.4.5`

2. Configure `cargo-pgx`, with `cargo pgx init --pgNN=download`

   Where `NN` is the major version of the Postgres version you want to use as your "primary" test platform.

   Note that you can replace the `--pgNN=download` option with `--pgNN=<path to pg_config>`, if you already have a Postgres installation.
   However, this installation must be writable by your user.

3. Hack away to your heart's content.

4. To run the test suite, run `cargo pgx test`

5. To get a running Postgres with your current extension available for use, run `cargo pgx run`.
   It'll print the port number that your temporary Postgres instance is listening on.

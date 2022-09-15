A Postgres extension to provide Encrypted Query Operations (`enquo`).


# Installation

## Pre-requisites

* A relatively recent [Rust](https://rust-lang.org) toolchain; and

* A Postgres installation, including the server-side development headers (for Debian/Ubuntu, these are in the `postgresql-server-dev-NN` package)


## Steps

1. Install `cargo-pgx`, with `cargo install cargo-pgx`

2. Configure `cargo-pgx`, with `cargo pgx init --pgNN=$(which pg_config)`

   Where `NN` is the major version of your Postgres installation (eg `--pg14=$(which pg_config)` if you're running Postgres 14)

3. Run `cargo pgx install --release` (this command requires write access to the directories specified by `PKGLIBDIR` and `SHAREDIR` in the output of `pg_config`)

If all goes well, you should be able to now enable the extension in any database of your choice with `CREATE EXTENSION pg_enquo;`


# Development

Welcome to the hidden world of Rust-based Postgres extensions.
We've got cake!


## Pre-requisites

* A relatively recent [Rust](https://rust-lang.org) toolchain.


## Steps

1. Install `cargo-pgx`, with `cargo install cargo-pgx`

2. Configure `cargo-pgx`, with `cargo pgx init --pgNN=download`

   Where `NN` is the major version of the Postgres version you want to use as your "primary" test platform.

   Note that you can replace the `--pgNN=download` option with `--pgNN=<path to pg_config>`, if you already have a Postgres installation.
   However, this installation must be writable by your user.

3. Hack away to your heart's content.

4. To run the test suite, run `cargo pgx test`

5. To get a running Postgres with your current extension available for use, run `cargo pgx run`.

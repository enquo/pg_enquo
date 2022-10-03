# Pre-requisites

* A relatively recent [Rust](https://rust-lang.org) toolchain; and

* A Postgres installation, including the server-side development headers (for Debian/Ubuntu, these are in the `postgresql-server-dev-NN` package)


# Steps

1. Install `cargo-pgx`, with `cargo install cargo-pgx --version 0.4.5`

2. Configure `cargo-pgx`, with `cargo pgx init --pgNN=$(which pg_config)`

   Where `NN` is the major version of your Postgres installation (eg `--pg14=$(which pg_config)` if you're running Postgres 14)

3. Run `cargo pgx install --release` (this command requires write access to the directories specified by `PKGLIBDIR` and `SHAREDIR` in the output of `pg_config`)

If all goes well, you should be able to now enable the extension in any database of your choice with `CREATE EXTENSION pg_enquo;`

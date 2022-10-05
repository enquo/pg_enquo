# Pre-requisites

Before you get installing, let's get your environment setup.
Once you have this sorted, you won't have to do it again for upgrades of `pg_enquo`.


## Install Rust

To build `pg_enquo`, you must have Rust 1.59.0 or later installed.
You can check if you already have this installed by running:

```sh
cargo version
```

If it says something like `cargo 1.61.0`, then you're ready to go.
An older version number indicates you'll need to upgrade Rust, which is just a `rustup upgrade` (or via your package manager).
Alternately, if it reports "command not found" or similar, then visit [the Rust project installation instructions](https://www.rust-lang.org/learn/get-started) for the easiest way to get going.


## Ensure `rustfmt` is Available

Part of the build process requires the `rustfmt` code formatting tool.
It is *usually* a part of a standard Rust install, but in certain circumstances may be missing.
Check that it is installed by running:

```sh
rustfmt </dev/null
```

If it is not already installed, you should be prompted to install it with `rustup component add rustfmt`.


## You Will Need PostgreSQL

As a PostgreSQL extension, it's not surprising that `pg_enquo` needs PostgreSQL installed.
You will need to have an installation of PostgreSQL 11 or later in order to build and use `pg_enquo`.

Note that you also need to have the server-side development headers for PostgreSQL installed.
If you are running a packaged version of PostgreSQL, these may be in a package that isn't installed by default.
For example, for Debian/Ubuntu, the server-side development headers are in a package named **`postgresql-server-dev-14`** (switch out the `14` for your PostgreSQL major version, if you running an older version).


## Install and Configure `cargo-pgx`

The `cargo-pgx` package is a tool for building PostgreSQL extensions in Rust.

1. Install `cargo-pgx`, with `cargo install cargo-pgx --version 0.4.5`

2. Configure `cargo-pgx`, with `cargo pgx init --pg14=$(which pg_config)`

   If you're running an older PostgreSQL major version, replace `14` with the major version you're using.


# Build and Install

The extension itself is built directly from the `pg_enquo` git repository.
Start off by cloning that repository to your machine and getting into the checked out source:

```sh
git clone https://github.com/enquo/pg_enquo.git
cd pg_enquo
```

Now you can now build and install `pg_enquo` with this one weird trick:

```sh
cargo pgx install --features pg14 --release
```

If you're running an older PostgreSQL major version, replace `14` with the major version you're using.

This command requires write access to the directories specified by `PKGLIBDIR` and `SHAREDIR` in the output of `pg_config`.
If necessary, run the command with `sudo`.


# Enable the Extension in Your Database

If all has gone well so far, you should be able to now enable the extension in any database of your choice with `CREATE EXTENSION pg_enquo;`.

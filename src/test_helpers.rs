use enquo_core::{key_provider::Static, Field, Root};
use pgx::*;

pub fn field() -> Field {
    Root::new(&Static::new(b"testkey"))
        .unwrap()
        .field(b"foo", b"bar")
        .unwrap()
}

pub fn arg(s: &String) -> (PgOid, Option<pg_sys::Datum>) {
    (PgBuiltInOids::TEXTOID.oid(), s.into_datum())
}

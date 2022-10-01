use pgx::*;
use enquo_core::{Field, Root};

pub fn field() -> Field {
    let k: &[u8] = b"testkey";
    Root::new(&k).unwrap().field(b"foo", b"bar").unwrap()
}

pub fn arg(s: &String) -> (PgOid, Option<pg_sys::Datum>) {
    (PgBuiltInOids::TEXTOID.oid(), s.into_datum())
}

use enquo_core::{key_provider::Static, Field, Root};
use pgrx::*;
use std::sync::Arc;

pub fn field() -> Field {
    let key = [0u8; 32]; // Create a 32-byte key filled with zeros (for testing purposes only)
    let static_provider = Static::new(&key).unwrap();
    let root = Root::new(Arc::new(static_provider)).unwrap();
    root.field(b"foo", b"bar").unwrap()
}

pub fn arg(s: &String) -> (PgOid, Option<pg_sys::Datum>) {
    (PgBuiltInOids::TEXTOID.oid(), s.into_datum())
}

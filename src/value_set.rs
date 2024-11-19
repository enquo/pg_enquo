use enquo_core::ValueSet;
use pgrx::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(
    Serialize, Deserialize, Debug, PostgresType
)]
#[allow(non_camel_case_types)]
pub struct enquo_value_set(ValueSet);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[pg_test]
    fn value_set_type_exists() {
        assert!(Spi::get_one::<bool>(
            "SELECT COUNT(*) = 1 FROM pg_type WHERE typname = 'enquo_value_set'"
        )
        .unwrap()
        .unwrap());
    }
}

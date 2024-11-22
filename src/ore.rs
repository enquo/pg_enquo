use enquo_core::datatype::{Kith, ORE};
use pgrx::*;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    PostgresType,
    PostgresEq,
    PostgresOrd,
)]
#[allow(non_camel_case_types)]
pub struct enquo_ore_32_8(pub ORE<8, 16>);

#[derive(Serialize, Deserialize, Debug, PostgresType)]
#[allow(non_camel_case_types)]
pub struct enquo_kith_ore_32_8(Kith<ORE<8, 16>>);

#[pg_operator(immutable, parallel_safe)]
#[opname(<)]
#[negator(>=)]
fn ore_32_8_lt(left: enquo_ore_32_8, right: enquo_kith_ore_32_8) -> bool {
    left.0
        < right
            .0
            .compatible_member(&left.0)
            .expect("Cannot perform operation on a kith without a compatible member")
}

#[pg_operator(immutable, parallel_safe)]
#[opname(>)]
#[negator(<=)]
fn ore_32_8_gt(left: enquo_ore_32_8, right: enquo_kith_ore_32_8) -> bool {
    left.0
        > right
            .0
            .compatible_member(&left.0)
            .expect("Cannot perform operation on a kith without a compatible member")
}

#[pg_operator(immutable, parallel_safe)]
#[opname(<=)]
#[negator(>)]
fn ore_32_8_le(left: enquo_ore_32_8, right: enquo_kith_ore_32_8) -> bool {
    left.0
        <= right
            .0
            .compatible_member(&left.0)
            .expect("Cannot perform operation on a kith without a compatible member")
}

#[pg_operator(immutable, parallel_safe)]
#[opname(>=)]
#[negator(<)]
fn ore_32_8_ge(left: enquo_ore_32_8, right: enquo_kith_ore_32_8) -> bool {
    left.0
        >= right
            .0
            .compatible_member(&left.0)
            .expect("Cannot perform operation on a kith without a compatible member")
}

use enquo_core::{ValueSet, ORE};
use pgx::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, PostgresType, PostgresEq)]
#[allow(non_camel_case_types)]
pub struct enquo_ore_32_4(pub ORE<8, 16, u32>);

#[derive(Serialize, Deserialize, Debug, PostgresType)]
#[allow(non_camel_case_types)]
pub struct enquo_set_ore_32_4(ValueSet<ORE<8, 16, u32>>);

#[pg_operator(immutable, parallel_safe)]
#[opname(<)]
#[negator(>=)]
fn ore_32_4_lt(left: enquo_ore_32_4, right: enquo_set_ore_32_4) -> bool {
    left.0
        < *right
            .0
            .compatible_value(&left.0)
            .expect("Cannot perform operation on value set without a compatible value")
}

#[pg_operator(immutable, parallel_safe)]
#[opname(>)]
#[negator(<=)]
fn ore_32_4_gt(left: enquo_ore_32_4, right: enquo_set_ore_32_4) -> bool {
    left.0
        > *right
            .0
            .compatible_value(&left.0)
            .expect("Cannot perform operation on value set without a compatible value")
}

#[pg_operator(immutable, parallel_safe)]
#[opname(<=)]
#[negator(>)]
fn ore_32_4_le(left: enquo_ore_32_4, right: enquo_set_ore_32_4) -> bool {
    left.0
        <= *right
            .0
            .compatible_value(&left.0)
            .expect("Cannot perform operation on value set without a compatible value")
}

#[pg_operator(immutable, parallel_safe)]
#[opname(>=)]
#[negator(<)]
fn ore_32_4_ge(left: enquo_ore_32_4, right: enquo_set_ore_32_4) -> bool {
    left.0
        >= *right
            .0
            .compatible_value(&left.0)
            .expect("Cannot perform operation on value set without a compatible value")
}

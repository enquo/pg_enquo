use enquo_core::I64;
use pgx::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PostgresType, PostgresEq, PostgresOrd, Ord, Eq, PartialOrd, PartialEq)]
#[allow(non_camel_case_types)]
pub struct enquo_bigint(I64);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn ensure_enquo_bigint_exists() {
        assert!(Spi::get_one::<bool>("SELECT COUNT(*) = 1 FROM pg_type WHERE typname = 'enquo_bigint'").unwrap());
    }

    #[pg_test]
    fn ensure_enquo_bigint_has_operators() {
        let type_oid_datum = Spi::get_one_with_args::<u32>(
            "SELECT oid FROM pg_type WHERE typname = $1",
            vec![
                (PgBuiltInOids::TEXTOID.oid(), String::from("enquo_bigint").into_datum())
            ]
        ).unwrap().into_datum();

        for op in vec!["=", "<>", "<", ">", "<=", ">="].iter() {
            assert!(
                Spi::get_one_with_args::<bool>(
                    "SELECT COUNT(*) = 1 FROM pg_operator WHERE oprname = $1 AND oprleft = $2 AND oprright = $2",
                    vec![
                        (PgBuiltInOids::TEXTOID.oid(), op.to_string().into_datum()),
                        (PgBuiltInOids::OIDOID.oid(), type_oid_datum)
                    ]
                ).unwrap()
            );
        }
    }
}

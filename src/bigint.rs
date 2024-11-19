use enquo_core::datatype::I64;
use pgrx::*;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    PostgresType,
    PostgresEq,
    PostgresOrd,
)]
#[allow(non_camel_case_types)]
pub struct enquo_bigint(I64);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use enquo_core::datatype::I64;
    use serde_json;
    use pgrx::pg_sys::Oid;

    fn create_test_table() {
        Spi::run("CREATE TABLE bigint_tests (id VARCHAR(255), bi enquo_bigint NOT NULL)").unwrap();
    }

    fn create_test_index() {
        Spi::run("CREATE INDEX bigint_test_idx ON bigint_tests(bi)").unwrap();
    }

    #[pg_test]
    fn type_exists() {
        assert!(Spi::get_one::<bool>(
            "SELECT COUNT(*) = 1 FROM pg_type WHERE typname = 'enquo_bigint'"
        )
        .unwrap()
        .unwrap());
    }

    #[pg_test]
    fn type_has_operators() {
        let type_oid_datum = Spi::get_one_with_args::<Oid>(
            "SELECT oid FROM pg_type WHERE typname = $1",
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                String::from("enquo_bigint").into_datum(),
            )],
        )
        .unwrap()
        .into_datum();

        for op in vec!["=", "<>", "<", ">", "<=", ">="].iter() {
            assert!(
                Spi::get_one_with_args::<bool>(
                    "SELECT COUNT(*) = 1 FROM pg_operator WHERE oprname = $1 AND oprleft = $2 AND oprright = $2",
                    vec![
                        (PgBuiltInOids::TEXTOID.oid(), op.to_string().into_datum()),
                        (PgBuiltInOids::OIDOID.oid(), type_oid_datum)
                    ]
                ).unwrap().unwrap()
            );
        }
    }

    #[pg_test]
    fn data_insertion() {
        create_test_table();

        let value = I64::new(42, b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();

        Spi::run(&format!(
            r#"INSERT INTO bigint_tests VALUES ('42', '{}')"#,
            s
        ))
        .unwrap();
    }

    #[pg_test]
    fn querying_without_left_ciphertexts() {
        create_test_table();

        for i in 0..10 {
            let value = I64::new(i, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO bigint_tests VALUES ('{}', '{}')"#,
                i, s
            ))
            .unwrap();
        }

        let v4 = I64::new_with_unsafe_parts(4, b"test", &field()).unwrap();
        let s4 = serde_json::to_string(&v4).unwrap();

        assert_eq!(
            "4",
            Spi::get_one_with_args::<String>(
                "SELECT id FROM bigint_tests WHERE bi = $1::enquo_bigint",
                vec![arg(&s4)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            4,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM bigint_tests WHERE bi < $1::enquo_bigint",
                vec![arg(&s4)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            5,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM bigint_tests WHERE bi <= $1::enquo_bigint",
                vec![arg(&s4)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            6,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM bigint_tests WHERE bi >= $1::enquo_bigint",
                vec![arg(&s4)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            5,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM bigint_tests WHERE bi > $1::enquo_bigint",
                vec![arg(&s4)]
            )
            .unwrap()
            .unwrap()
        );
    }

    #[pg_test]
    #[should_panic]
    fn indexing_without_left_ciphertexts_fails() {
        create_test_table();
        create_test_index();

        for i in 0..2 {
            let value = I64::new(i, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO bigint_tests VALUES ('{}', '{}')"#,
                0, s
            ))
            .unwrap();
        }
    }

    #[pg_test]
    fn indexing_with_unsafe_parts_succeeds() {
        create_test_table();
        create_test_index();

        let value = I64::new_with_unsafe_parts(0, b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();
        Spi::run(&format!(
            r#"INSERT INTO bigint_tests VALUES ('{}', '{}')"#,
            0, s
        ))
        .unwrap();
    }

    #[pg_test]
    #[should_panic]
    fn order_by_without_left_ciphertexts_fails() {
        create_test_table();

        for i in 0..10 {
            let value = I64::new(i, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO bigint_tests VALUES ('{}', '{}')"#,
                i, s
            ))
            .unwrap();
        }

        Spi::run("SELECT id FROM bigint_tests ORDER BY bi").unwrap();
    }
}

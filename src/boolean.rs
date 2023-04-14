use enquo_core::Boolean;
use pgx::*;
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
pub struct enquo_boolean(Boolean);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use enquo_core::Boolean;
    use serde_json;

    fn create_test_table() {
        Spi::run("CREATE TABLE boolean_tests (id VARCHAR(255), value enquo_boolean NOT NULL)")
            .unwrap();
    }

    fn create_test_index() {
        Spi::run("CREATE INDEX boolean_test_idx ON boolean_tests(value)").unwrap();
    }

    #[pg_test]
    fn boolean_type_exists() {
        assert!(Spi::get_one::<bool>(
            "SELECT COUNT(*) = 1 FROM pg_type WHERE typname = 'enquo_boolean'"
        )
        .unwrap()
        .unwrap());
    }

    #[pg_test]
    fn boolean_type_has_operators() {
        let type_oid_datum = Spi::get_one_with_args::<u32>(
            "SELECT oid FROM pg_type WHERE typname = $1",
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                String::from("enquo_boolean").into_datum(),
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
    fn boolean_data_insertion() {
        create_test_table();

        let value = Boolean::new(true, b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();

        Spi::run(&format!(
            r#"INSERT INTO boolean_tests VALUES ('true', '{}')"#,
            s
        ))
        .unwrap();
    }

    #[pg_test]
    fn querying_booleans_without_left_ciphertexts() {
        create_test_table();

        for i in vec![true, false] {
            let value = Boolean::new(i, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO boolean_tests VALUES ('{}', '{}')"#,
                i, s
            ))
            .unwrap();
        }

        let vt = Boolean::new_with_unsafe_parts(true, b"test", &field()).unwrap();
        let st = serde_json::to_string(&vt).unwrap();

        assert_eq!(
            "true",
            Spi::get_one_with_args::<String>(
                "SELECT id FROM boolean_tests WHERE value = $1::enquo_boolean",
                vec![arg(&st)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            1,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM boolean_tests WHERE value < $1::enquo_boolean",
                vec![arg(&st)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            2,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM boolean_tests WHERE value <= $1::enquo_boolean",
                vec![arg(&st)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            1,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM boolean_tests WHERE value >= $1::enquo_boolean",
                vec![arg(&st)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            0,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM boolean_tests WHERE value > $1::enquo_boolean",
                vec![arg(&st)]
            )
            .unwrap()
            .unwrap()
        );
    }

    #[pg_test]
    #[should_panic]
    fn indexing_booleans_without_left_ciphertexts_fails() {
        create_test_table();
        create_test_index();

        for i in vec![true, false] {
            let value = Boolean::new(i, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO boolean_tests VALUES ('{}', '{}')"#,
                i, s
            ))
            .unwrap();
        }
    }

    #[pg_test]
    fn indexing_booleans_with_unsafe_parts_succeeds() {
        create_test_table();
        create_test_index();

        let value = Boolean::new_with_unsafe_parts(true, b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();
        Spi::run(&format!(
            r#"INSERT INTO boolean_tests VALUES ('{}', '{}')"#,
            0, s
        ))
        .unwrap();
    }

    #[pg_test]
    #[should_panic]
    fn boolean_order_by_without_left_ciphertexts_fails() {
        create_test_table();

        for i in vec![true, false] {
            let value = Boolean::new(i, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO boolean_tests VALUES ('{}', '{}')"#,
                i, s
            ))
            .unwrap();
        }

        Spi::run("SELECT id FROM boolean_tests ORDER BY value").unwrap();
    }
}

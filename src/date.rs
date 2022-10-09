use enquo_core::Date;
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
pub struct enquo_date(Date);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use enquo_core::Date;
    use serde_json;

    fn create_test_table() {
        Spi::run("CREATE TABLE date_tests (id VARCHAR(255), dt enquo_date NOT NULL)");
    }

    fn create_test_index() {
        Spi::run("CREATE INDEX date_test_idx ON date_tests(dt)");
    }

    #[pg_test]
    fn date_type_exists() {
        assert!(Spi::get_one::<bool>(
            "SELECT COUNT(*) = 1 FROM pg_type WHERE typname = 'enquo_date'"
        )
        .unwrap());
    }

    #[pg_test]
    fn date_type_has_operators() {
        let type_oid_datum = Spi::get_one_with_args::<u32>(
            "SELECT oid FROM pg_type WHERE typname = $1",
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                String::from("enquo_date").into_datum(),
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
                ).unwrap()
            );
        }
    }

    #[pg_test]
    fn date_insertion() {
        create_test_table();

        let value = Date::new((1970, 1, 1), b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();

        Spi::run(&format!(
            r#"INSERT INTO date_tests VALUES ('1970-01-01', '{}')"#,
            s
        ));
    }

    #[pg_test]
    fn querying_dates_without_left_ciphertexts() {
        create_test_table();

        for i in 0..10 {
            let value = Date::new((1970 + i, 1, 1), b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO date_tests VALUES ('197{}-01-01', '{}')"#,
                i, s
            ));
        }

        let v4 = Date::new_with_unsafe_parts((1974, 1, 1), b"test", &field()).unwrap();
        let s4 = serde_json::to_string(&v4).unwrap();

        assert_eq!(
            "1974-01-01",
            Spi::get_one_with_args::<String>(
                "SELECT id FROM date_tests WHERE dt = $1::enquo_date",
                vec![arg(&s4)]
            )
            .unwrap()
        );
        assert_eq!(
            4,
            Spi::get_one_with_args::<u32>(
                "SELECT COUNT(id) FROM date_tests WHERE dt < $1::enquo_date",
                vec![arg(&s4)]
            )
            .unwrap()
        );
        assert_eq!(
            5,
            Spi::get_one_with_args::<u32>(
                "SELECT COUNT(id) FROM date_tests WHERE dt <= $1::enquo_date",
                vec![arg(&s4)]
            )
            .unwrap()
        );
        assert_eq!(
            6,
            Spi::get_one_with_args::<u32>(
                "SELECT COUNT(id) FROM date_tests WHERE dt >= $1::enquo_date",
                vec![arg(&s4)]
            )
            .unwrap()
        );
        assert_eq!(
            5,
            Spi::get_one_with_args::<u32>(
                "SELECT COUNT(id) FROM date_tests WHERE dt > $1::enquo_date",
                vec![arg(&s4)]
            )
            .unwrap()
        );
    }

    #[pg_test]
    #[should_panic]
    fn indexing_dates_without_left_ciphertexts_fails() {
        create_test_table();
        create_test_index();

        for i in 0..2 {
            let value = Date::new((1970, 1, i), b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO date_tests VALUES ('{}', '{}')"#,
                0, s
            ));
        }
    }

    #[pg_test]
    fn indexing_date_with_unsafe_parts_succeeds() {
        create_test_table();
        create_test_index();

        let value = Date::new_with_unsafe_parts((1970, 1, 1), b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();
        Spi::run(&format!(
            r#"INSERT INTO date_tests VALUES ('{}', '{}')"#,
            0, s
        ));
    }

    #[pg_test]
    #[should_panic]
    fn order_by_dates_without_left_ciphertexts_fails() {
        create_test_table();

        for i in 0..10 {
            let value = Date::new((1970 + i, 1, 1), b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO date_tests VALUES ('{}', '{}')"#,
                i, s
            ));
        }

        Spi::run("SELECT id FROM date_tests ORDER BY dt");
    }
}

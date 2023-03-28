use enquo_core::Text;
use pgx::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(
    Serialize, Deserialize, Debug, Eq, Hash, PartialEq, PostgresType, PostgresEq, PostgresHash,
)]
#[allow(non_camel_case_types)]
pub struct enquo_text(Text);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use enquo_core::Text;
    use serde_json;

    fn create_test_table() {
        Spi::run("CREATE TABLE text_tests (id VARCHAR(255), txt enquo_text NOT NULL)").unwrap();
    }

    fn create_test_index() {
        Spi::run("CREATE INDEX text_test_idx ON text_tests USING hash (txt)").unwrap();
    }

    #[pg_test]
    fn text_type_exists() {
        assert!(Spi::get_one::<bool>(
            "SELECT COUNT(*) = 1 FROM pg_type WHERE typname = 'enquo_text'"
        )
        .unwrap()
        .unwrap());
    }

    #[pg_test]
    fn text_type_has_operators() {
        let type_oid_datum = Spi::get_one_with_args::<u32>(
            "SELECT oid FROM pg_type WHERE typname = $1",
            vec![(
                PgBuiltInOids::TEXTOID.oid(),
                String::from("enquo_text").into_datum(),
            )],
        )
        .unwrap()
        .into_datum();

        for op in vec!["=", "<>"].iter() {
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
    fn text_insertion() {
        create_test_table();

        let value = Text::new("Hello, Enquo!", b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();

        Spi::run(&format!(
            r#"INSERT INTO text_tests VALUES ('hello', '{}')"#,
            s
        ))
        .unwrap();
    }

    #[pg_test]
    fn querying_text_without_left_ciphertexts() {
        create_test_table();

        for i in 0..10 {
            let value = Text::new(&format!("hello, {} Enquo!", i), b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(
                r#"INSERT INTO text_tests VALUES ('{}', '{}')"#,
                i, s
            ))
            .unwrap();
        }

        let v4 = Text::new_with_unsafe_parts("hello, 4 Enquo!", b"test", &field()).unwrap();
        let s4 = serde_json::to_string(&v4).unwrap();

        assert_eq!(
            "4",
            Spi::get_one_with_args::<String>(
                "SELECT id FROM text_tests WHERE txt = $1::enquo_text",
                vec![arg(&s4)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            9,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM text_tests WHERE txt <> $1::enquo_text",
                vec![arg(&s4)]
            )
            .unwrap()
            .unwrap()
        );
    }

    #[pg_test]
    #[should_panic]
    fn indexing_text_without_hash_value_fails() {
        create_test_table();
        create_test_index();

        let value = Text::new("onoes!", b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();
        Spi::run(&format!(
            r#"INSERT INTO text_tests VALUES ('{}', '{}')"#,
            0, s
        ))
        .unwrap();
    }

    #[pg_test]
    fn indexing_text_with_unsafe_parts_succeeds() {
        create_test_table();
        create_test_index();

        let value = Text::new_with_unsafe_parts("huzzah!", b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();
        Spi::run(&format!(
            r#"INSERT INTO text_tests VALUES ('{}', '{}')"#,
            0, s
        ))
        .unwrap();
    }
}

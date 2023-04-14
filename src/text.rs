use enquo_core::Text;
use pgx::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::enquo_ore_32_4;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    PostgresType,
    PostgresEq,
    PostgresOrd,
    PostgresHash,
)]
#[allow(non_camel_case_types)]
pub struct enquo_text(Text);

#[pg_extern]
fn length(t: enquo_text) -> enquo_ore_32_4 {
    enquo_ore_32_4(t.0.length().expect(
        "Cannot extract length from instance of enquo_text that doesn't provide length information",
    ))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use enquo_core::{Text, ORE};
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

        let v4 = Text::new_with_unsafe_parts("hello, 4 Enquo!", b"test", &field(), None).unwrap();
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

        let value = Text::new_with_unsafe_parts("huzzah!", b"test", &field(), None).unwrap();
        let s = serde_json::to_string(&value).unwrap();
        Spi::run(&format!(
            r#"INSERT INTO text_tests VALUES ('{}', '{}')"#,
            0, s
        ))
        .unwrap();
    }

    #[pg_test]
    fn length_retrieval() {
        create_test_table();

        let value = Text::new("Hello, Enquo!", b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();

        Spi::run(&format!(
            r#"INSERT INTO text_tests VALUES ('hello', '{}')"#,
            s
        ))
        .unwrap();

        let ore_len_str = Spi::get_one::<String>("SELECT length(txt)::text FROM text_tests")
            .unwrap()
            .unwrap();

        let actual_len: ORE<8, 16, u32> = serde_json::from_str(&ore_len_str).unwrap();
        let lengths = Text::query_length(13, &field()).unwrap();
        let expected_len = lengths.compatible_value(&actual_len).unwrap();

        assert_eq!(expected_len, &actual_len);
    }

    #[pg_test]
    fn length_querying() {
        create_test_table();

        for v in vec!["Hello, Enquo!", "Well hello there, young Enquo!", "ohai!"] {
            let value = Text::new(v, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();

            Spi::run(&format!(r#"INSERT INTO text_tests VALUES ('{v}', '{s}')"#)).unwrap();
        }

        let query_value = Text::query_length(10, &field()).unwrap();
        let query_str = serde_json::to_string(&query_value).unwrap();

        assert_eq!(
            2,
            Spi::get_one_with_args::<i64>(
                "SELECT COUNT(id) FROM text_tests WHERE length(txt) > $1::enquo_set_ore_32_4",
                vec![arg(&query_str)]
            )
            .unwrap()
            .unwrap()
        );
        assert_eq!(
            "ohai!",
            Spi::get_one_with_args::<String>(
                "SELECT id FROM text_tests WHERE length(txt) <= $1::enquo_set_ore_32_4",
                vec![arg(&query_str)]
            )
            .unwrap()
            .unwrap()
        );
    }
}

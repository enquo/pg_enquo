use enquo_core::I64;
use pgx::*;
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use backtrace::Backtrace;

use crate::ValueOptions;

#[derive(Serialize, Deserialize, Debug, PostgresType, PostgresEq, PostgresOrd)]
#[filter_datum]
#[allow(non_camel_case_types)]
pub struct enquo_bigint {
    #[serde(rename = "v")]
    value: I64,

    #[serde(rename = "$")]
    options: Option<Vec<ValueOptions>>,
}

impl IntoDatumFilter for enquo_bigint {
    fn filter_for_datum(self) -> Self {
        let mut obj = self;
        match &obj.options {
            Some(opts) => {
                if !opts.contains(&ValueOptions::KeepLeft) {
                    obj.value.clear_left_ciphertexts();
                }
            },
            None => {
                obj.value.clear_left_ciphertexts();
            },
        }

        obj
    }
}

impl Ord for enquo_bigint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for enquo_bigint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for enquo_bigint {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for enquo_bigint {}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;
    use super::*;
    use crate::test_helpers::*;
    use enquo_core::I64;
    use serde_json;

    fn create_test_table() {
        Spi::run("CREATE TABLE bigint_tests (id VARCHAR(255), bi enquo_bigint NOT NULL)");
    }

    fn create_test_index() {
        Spi::run("CREATE INDEX bigint_test_idx ON bigint_tests(bi)");
    }

    #[pg_test]
    fn type_exists() {
        assert!(Spi::get_one::<bool>("SELECT COUNT(*) = 1 FROM pg_type WHERE typname = 'enquo_bigint'").unwrap());
    }

    #[pg_test]
    fn type_has_operators() {
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

    #[pg_test]
    fn data_insertion() {
        create_test_table();

        let value = I64::new(42, b"test", &field()).unwrap();
        let s = serde_json::to_string(&value).unwrap();

        Spi::run(&format!(r#"INSERT INTO bigint_tests VALUES ('42', '{{"v":{}}}')"#, s));
    }

    #[pg_test]
    fn querying() {
        create_test_table();

        for i in 0..10 {
            let value = I64::new(i, b"test", &field()).unwrap();
            let s = serde_json::to_string(&value).unwrap();
            Spi::run(&format!(r#"INSERT INTO bigint_tests VALUES ('{}', '{{"v":{}}}')"#, i, s));
        }

        let v4 = I64::new(4, b"test", &field()).unwrap();
        let s4 = format!(r#"{{"v":{},"$":["KeepLeft"]}}"#, serde_json::to_string(&v4).unwrap());

        assert_eq!("4", Spi::get_one_with_args::<String>("SELECT id FROM bigint_tests WHERE bi = $1::enquo_bigint", vec![arg(&s4)]).unwrap());
        assert_eq!(4, Spi::get_one_with_args::<u32>("SELECT COUNT(id) FROM bigint_tests WHERE bi < $1::enquo_bigint", vec![arg(&s4)]).unwrap());
        assert_eq!(5, Spi::get_one_with_args::<u32>("SELECT COUNT(id) FROM bigint_tests WHERE bi <= $1::enquo_bigint", vec![arg(&s4)]).unwrap());
        assert_eq!(6, Spi::get_one_with_args::<u32>("SELECT COUNT(id) FROM bigint_tests WHERE bi >= $1::enquo_bigint", vec![arg(&s4)]).unwrap());
        assert_eq!(5, Spi::get_one_with_args::<u32>("SELECT COUNT(id) FROM bigint_tests WHERE bi > $1::enquo_bigint", vec![arg(&s4)]).unwrap());
    }

    #[pg_test]
    fn left_ciphertexts_filtered_by_default() {
        create_test_table();

        Spi::run(r#"INSERT INTO bigint_tests VALUES ('', '{"v":{"v1":{"a":{"iv":[],"ct":[]},"o":{"l":[1,2,3,4,5],"r":[]},"k":[]}}}')"#);
        let r = Spi::get_one::<enquo_bigint>("SELECT bi FROM bigint_tests").unwrap();

        match &r.value {
            I64::v1(v) => assert_eq!(None, v.ore_ciphertext.left),
            _ => panic!("How did we get a non-v1 value here?!?"),
        };
    }

    #[pg_test]
    fn left_ciphertexts_kept_if_requested() {
        create_test_table();

        Spi::run(r#"INSERT INTO bigint_tests VALUES ('', '{"v":{"v1":{"a":{"iv":[],"ct":[]},"o":{"l":[1,2,3,4,5],"r":[]},"k":[]}},"$":["KeepLeft"]}')"#);
        let r = Spi::get_one::<enquo_bigint>("SELECT bi FROM bigint_tests").unwrap();

        match &r.value {
            I64::v1(v) => assert!(matches!(v.ore_ciphertext.left, Some(_))),

            _ => panic!("How did we get a non-v1 value here?!?"),
        };
    }
}

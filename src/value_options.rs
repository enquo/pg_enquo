use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum ValueOptions {
    #[serde(rename = "KL", alias = "KeepLeft")]
    KeepLeft,
    #[serde(skip)]
    #[allow(dead_code)]
    Unknown,
}

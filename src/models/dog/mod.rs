use welds::prelude::*;

#[derive(Debug, WeldsModel, PartialEq)]
#[welds(table = "dogs")]
pub(crate) struct Dog {
    #[welds(primary_key)]
    pub id: i64,
    pub name: String,
    pub test_json: serde_json::Value,
}

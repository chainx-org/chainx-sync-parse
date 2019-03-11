use serde_json::Value;

use super::schema::blocks;

#[derive(Insertable)]
#[table_name = "blocks"]
pub struct NewBlock<'a> {
    pub height: i64,
    pub prefix: &'a [u8],
    pub value: &'a Value,
}

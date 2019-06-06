mod models;
mod schema;

use std::collections::HashMap;
use std::env;

use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use serde_json::Value;

use self::models::*;
use self::schema::blocks;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn insert_block(conn: &PgConnection, new_blocks: Vec<NewBlock>) -> usize {
    diesel::insert_into(blocks::table)
        .values(&new_blocks)
        .execute(conn)
        .expect("Error saving new block")
}

pub fn insert_block_into_pgsql(conn: &PgConnection, h: u64, stat: &HashMap<Vec<u8>, Value>) {
    let new_blocks: Vec<NewBlock> = stat
        .iter()
        .map(|(k, v)| NewBlock {
            height: h as i64,
            prefix: k,
            value: v,
        })
        .collect();
    let expect_size = new_blocks.len();
    let actual_size = insert_block(conn, new_blocks);
    if actual_size != expect_size {
        warn!(
            target: "parse",
            "PostgreSQL: insert size (actual/expect) - ({}/{})",
            actual_size, expect_size
        );
    } else {
        info!(
            target: "parse",
            "PostgreSQL: insert all blocks with height #{} successfully",
            h
        );
    }
}

mod models;
mod schema;

use std::collections::HashMap;
use std::env;

use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use log::{info, warn};
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

pub fn insert_block_with_height(conn: &PgConnection, h: u64, stat: &HashMap<String, Value>) {
    let new_blocks: Vec<NewBlock> = stat
        .iter()
        .map(|(k, v)| NewBlock {
            height: h as i64,
            prefix: k,
            value: v,
        })
        .collect();
    let size = insert_block(conn, new_blocks);
    if size != new_blocks.len() {
        warn!(
            "PostgreSQL: insert size (actual/expect) - ({}/{})",
            size,
            new_blocks.len()
        );
    } else {
        info!(
            "PostgreSQL: insert all blocks with height #{} successfully",
            h
        );
    }
}

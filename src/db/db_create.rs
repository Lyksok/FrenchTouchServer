use rusqlite::{Connection};
use std::fs::File;
use std::io::prelude::*;

fn get_schema(schema_path: &str) -> std::io::Result<String> {
    let mut file = File::open(&schema_path).expect("Could not open file");

    let mut res = String::new();
    match file.read_to_string(&mut res) {
        Err(e) => Err(e),
        Ok(_) => Ok(res),
    }
}

pub fn create_db(conn: Connection) -> std::io::Result<()> {
    let schema_file: String = get_schema("schema.sql").expect("Could not retrieve schema");
    let schemas: Vec<String> = schema_file
        .split("\n\n")
        .map(|x| x.replace("\n",""))
        .collect();

    for query in schemas.iter() {
        conn.execute(query, [],).expect("Create query failed");
    }
    Ok(())
}

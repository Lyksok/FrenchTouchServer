use rusqlite::{Connection, Result};
use std::fs::File;
use std::io::prelude::*;

fn get_schema(schema_path: &str) -> String {
    let mut file = match File::open(&schema_path) {
        Err(why) => panic!("Could not open {}: {}", schema_path, why),
        Ok(file) => file,
    };

    let mut res = String::new();
    match file.read_to_string(&mut res) {
        Err(why) => panic!("Could not read file: {}", why),
        Ok(_) => println!("Successfully retrieved file data"),
    };
    res
}

fn aux_create_db(db_path: &str, schema_path: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;
    let schema_file: String = get_schema(schema_path);
    let schemas: Vec<String> = schema_file
        .split("\n\n")
        .map(|x| x.replace("\n",""))
        .collect();

    for query in schemas.iter() {
        conn.execute(query, [],)?;
    }
    Ok(())
}

pub fn create_db(db_path: &str, schema_path: &str) -> std::io::Result<()> {
    match aux_create_db(db_path, schema_path) {
        Err(e) => println!("Failed to create database: {}", e),
        Ok(_) => println!("Database created successfully."),
    }
    Ok(())
}

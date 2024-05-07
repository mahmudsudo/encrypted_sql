use std::fs::read_dir;
use std::path::Path;
use std::collections::HashMap;

use tfhe::integer::ClientKey;

use tfhe::{FheUint8};
use tfhe::prelude::FheEncrypt;
use tfhe::shortint::PBSParameters;
use crate::database_server::Database;

pub mod database_server;

struct EncryptedQuery {
    sql: String,
    conditions: Vec<EncryptedCondition>
}

struct EncryptedCondition {
    left: Vec<u8>, // encrypted value
    op: Vec<u8>, // encrypted operator
    right: Vec<u8>, // encrypted value
}

struct EncryptedResult {
    result: Vec<u8>,
}

impl EncryptedResult {
    // Decrypt the EncryptedResult
    fn decrypt_result(client_key: &ClientKey, result: &EncryptedResult) -> String {
        let decrypted_values: Vec<u8> = result.result.iter()
            .map(|encrypted_value| encrypted_value.decrypt(client_key))
            .collect();
        String::from_utf8_lossy(&decrypted_values).to_string()
    }
}

struct Tables {
    columns: HashMap<String, Column>,
    rows: Vec<Row>,
}

struct Column {
    name: String,
    data_type: DataType,
}

enum DataType {
    Integer(IntegerType),
    Boolean,
    String,
}

enum IntegerType {
    Signed8,
    Unsigned8,
    Signed16,
    Unsigned16,
    Signed32,
    Unsigned32,
    Signed64,
    Unsigned64,
}

struct Row {
    values: Vec<Value>,
}

enum Value {
    Integer(i64),
    Boolean(bool),
    String(String),
}

fn load_tables(path: &Path, db: &mut Database) -> Tables {
    let mut tables = Tables::new();

    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let table_name = file_name.trim_end_matches(".csv");

            // Load the table from CSV file using the Database instance
            db.load_table_from_csv(table_name, &path).unwrap();

            // Assume we have a way to get the loaded table
            let loaded_table = db.get_table(table_name).unwrap();

            // Insert the loaded table into the Tables struct
            tables.insert_row(table_name, loaded_table);
        }
    }

    tables
}

fn default_cpu_parameters() -> PBSParameters {
    todo!()
}

fn encrypt_query(query: sqlparser::ast::Select, user_fhe_secret_key: &ClientKey) -> EncryptedQuery {
    let mut encrypted_query = EncryptedQuery {
        sql: query.to_string(),
        conditions: Vec::new(),
    };

    let sql_bytes = query.to_string().as_bytes();
    let mut encrypted_sql : Vec<u8> = Vec::new();

    for b in sql_bytes {
       let encrypted_b =  FheUint8::encrypt(*b, user_fhe_secret_key);
        let b_u8 = encrypted_b;
    }

    encrypted_query.sql = encrypted_sql;

    encrypted_query
}

fn run_fhe_query(sks: &tfhe::integer::ServerKey, input: &EncryptedQuery, data: &Tables) -> EncryptedResult {
    todo!()
}

fn decrypt_result(clientk_key: &ClientKey, result: &EncryptedResult) -> String {
    todo!()
}

fn main() {

}

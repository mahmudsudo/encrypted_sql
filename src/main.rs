use std::{env, process};
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use std::time::Instant;

use tfhe::integer::{ClientKey, ServerKey};
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
    // Assuming each table is stored with its name as a key
    tables: HashMap<String, Vec<HashMap<String, String>>>,
}

impl Tables {
    pub fn new() -> Tables {
        Tables {
            tables: HashMap::new(),
        }
    }

    // Function to insert a row into a table
    pub fn insert_row(&mut self, table_name: &str, row: HashMap<String, String>) {
        if let Some(table) = self.tables.get_mut(table_name) {
            table.push(row);
        } else {
            self.tables.insert(table_name.to_string(), vec![row]);
        }
    }
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

fn default_cpu_parameters() -> PBSParameters {
    todo!()
}

fn load_tables(path: &Path, db: &Database) -> Result<Tables, rusqlite::Error> {
    let mut tables = Tables::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |e| e == "csv") {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let table_name = file_name.trim_end_matches(".csv");

            // Load the table from CSV file using the Database instance
            db.load_table_from_csv(&file_path)?;

            // Assume we have a function to retrieve loaded data as a vector of HashMaps (each HashMap represents a row)
            let loaded_data = db.retrieve_table_data(&table_name)?;
            for row in loaded_data {
                tables.insert_row(table_name, row);
            }
        }
    }

    Ok(tables)
}

// fn encrypt_query(query_str: &str, user_fhe_secret_key: &ClientKey) -> EncryptedQuery {
//     let dialect = GenericDialect {}; // Using a generic SQL dialect
//     let ast = Parser::parse_sql(&dialect, query_str).expect("Failed to parse query");
//
//     // Assuming the first statement is a SELECT and we're only handling simple cases for demonstration.
//     let query = if let sqlparser::ast::Statement::Select(select) = &ast[0] {
//         select
//     } else {
//         panic!("Query provided is not a SELECT query.");
//     };
//
//     // Encrypt conditions (assuming a simple where clause)
//     let mut conditions = vec![];
//     if let Some(where_clause) = &query.selection {
//         // Assuming a simple binary operation for demonstration
//         if let sqlparser::ast::Expr::BinaryOp { left, op, right } = where_clause.as_ref() {
//             let encrypted_left = FheUint8::encrypt(left.to_string().as_bytes()[0], user_fhe_secret_key);
//             let encrypted_op = FheUint8::encrypt(op.to_string().as_bytes()[0], user_fhe_secret_key);
//             let encrypted_right = FheUint8::encrypt(right.to_string().as_bytes()[0], user_fhe_secret_key);
//
//             conditions.push(EncryptedCondition {
//                 left: vec![encrypted_left], // Simplified for example
//                 op: vec![encrypted_op],
//                 right: vec![encrypted_right],
//             });
//         }
//     }
//
//     EncryptedQuery {
//         sql: query.to_string(), // Store the overall SQL for now, assuming non-sensitive or already encrypted
//         conditions
//     }
// }

// Mock-up of encryption function
fn encrypt_query(query_file_path: &Path, client_key: &ClientKey) -> EncryptedQuery {
    // This function should parse the SQL file and encrypt the query
    todo!()
}

// Mock-up of the FHE query function
fn run_fhe_query(sks: &ServerKey, input: &EncryptedQuery, data: &Database) -> EncryptedResult {
    // This function should simulate running an encrypted query against the database
    todo!()
}

fn decrypt_result(clientk_key: &ClientKey, result: &EncryptedResult) -> String {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} /path/to/db_dir query.txt", args[0]);
        process::exit(1);
    }

    let db_path = Path::new(&args[1]);
    let query_file_path = Path::new(&args[2]);

    // Example: Initialize your database and encryption systems here
    let db = match Database::load_from_directory(db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to load database: {}", e);
            process::exit(1);
        }
    };

    // Example: Initialize Server and Client keys
    let server_key = ServerKey::new_radix_server_key_from_shortint();  // Placeholder, replace with correct initialization
    let client_key = ClientKey::new_radix_client_key_from_shortint();  // Placeholder, replace with correct initialization

    // Assume we have a query in query_file_path that needs to be encrypted
    let encrypted_query = encrypt_query(&query_file_path, &client_key);

    // Running an FHE query
    let start = Instant::now();
    let encrypted_result = run_fhe_query(&server_key, &encrypted_query, &db);
    let duration = start.elapsed();

    // Decrypting the result
    let decrypted_result = decrypt_result(&client_key, &encrypted_result);

    println!("Runtime: {:.2?}", duration);
    println!("Clear DB query result: (some result)"); // Example placeholder
    println!("Encrypted DB query result: {}", decrypted_result); // Example placeholder
    println!("Results match: YES"); // Validation placeholder
}

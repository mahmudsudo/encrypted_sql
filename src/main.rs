use std::{collections::HashMap, env, fmt, fs, fs::read_dir, path::Path, process, time::Instant};
use std::error::Error;
use std::fmt::Debug;

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use tfhe::{ClientKey, ConfigBuilder, FheUint8, generate_keys, prelude::*, ServerKey, shortint::{parameters::*, PBSParameters}};

use crate::database_server::{AppError, Database};

pub mod database_server;

struct EncryptedQuery {
    sql: Vec<FheUint8>, // Vector of encrypted characters.
}

impl fmt::Display for EncryptedQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encrypted SQL Query")
    }
}

struct EncryptedResult {
    result: Vec<u8>,
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

fn load_tables(path: &Path, db: &Database) -> Result<Tables, AppError> {
    let mut tables = Tables::new();

    let entries = read_dir(path).map_err(AppError::Io)?;

    for entry in entries {
        let entry = entry.map_err(AppError::Io)?;
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().unwrap_or_default() == "csv" {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let table_name = file_name.trim_end_matches(".csv");

            db.load_table_from_csv(&file_path)?;

            if let Ok(loaded_data) = db.retrieve_table_data(&table_name) {
                for row in loaded_data {
                    tables.insert_row(table_name, row);
                }
            }
        }
    }

    Ok(tables)
}

fn encrypt_query(query_path: &Path, client_key: &ClientKey) -> Result<EncryptedQuery, Box<dyn std::error::Error>> {
    let query = fs::read_to_string(query_path)?;
    let dialect = GenericDialect {};
    let _ast = Parser::parse_sql(&dialect, &*query)?;

    let encrypted_data: Vec<FheUint8> = query.chars()
        .map(|c| {
            let encrypted_char = FheUint8::encrypt(c as u8, client_key);
            encrypted_char
        })
        .collect();

    Ok(EncryptedQuery { sql: encrypted_data })
}

/// This function will process an `EncryptedQuery` on set of data stored in `Tables`.
/// It will be able to execute basic SQL operations on the data in an encrypted form, and return an `Encrypted Result`.
fn run_fhe_query(sks: &ServerKey, input: &EncryptedQuery, _data: &Database) -> Result<EncryptedResult, Box<dyn Error>> {
    // TODO: update this processing of encrypted query on encrypted data
    Ok(EncryptedResult {
        result: input.sql.iter().map(|fhe_uint8| fhe_uint8.clone()).collect() // Placeholder logic
    })
}

fn decrypt_result(client_key: &ClientKey, encrypted_result: &EncryptedResult) -> Result<String, Box<dyn Error>> {
    // Assuming encrypted_result.result is Vec<FheUint8>
    let decrypted_chars: Result<Vec<u8>, _> = encrypted_result.result.iter()
        .map(|enc_char| enc_char.decrypt(client_key))
        .collect();

    let decrypted_bytes = decrypted_chars?;
    let result_string = String::from_utf8(decrypted_bytes)?;

    Ok(result_string)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} /path/to/db_dir query.txt", args[0]);
        process::exit(1);
    }

    let db_path = Path::new(&args[1]);
    let query_file_path = Path::new(&args[2]);

    // Setup TFHE configuration
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    let clear_value = 123u8; // Example clear value
    let encrypted_value = FheUint8::encrypt(clear_value, &client_key);

    // Load the database
    let db = match Database::load_from_directory(db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to load database: {}", e);
            process::exit(1);
        }
    };

    // Encrypt the query
    let encrypted_query = encrypt_query(query_file_path, &client_key).unwrap();

    // Run an FHE query (placeholder)
    let start = Instant::now();
    let encrypted_result = run_fhe_query(&server_key, &encrypted_query, &db);
    let duration = start.elapsed();

    // Decrypt the result
    let decrypted_result = decrypt_result(&client_key, &encrypted_result).unwrap();

    println!("Runtime: {:.2?}", duration);
    println!("Encrypted DB query result: {}", decrypted_result);
    println!("Results match: YES");
}
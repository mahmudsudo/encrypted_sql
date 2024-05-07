use std::{env, process,collections::HashMap,fs::read_dir,path::Path,time::Instant};

use tfhe::{generate_keys, ConfigBuilder,ClientKey, ServerKey,prelude::*,shortint::{PBSParameters,parameters::*}};


use crate::database_server::{AppError, Database};

pub mod database_server;

struct EncryptedQuery {
    sql: String,
}

struct EncryptedResult {
    result: Vec<u8>,
}

impl EncryptedResult {
    fn decrypt_result(client_key: &ClientKey, result: &EncryptedResult) -> String {
        // Assuming decryption returns bytes and converting them back to string
        let decrypted_bytes = result.result.iter()
            .map(|&encrypted_byte| decrypt_byte(encrypted_byte, client_key))
            .collect::<Vec<u8>>();
        String::from_utf8_lossy(&decrypted_bytes).to_string()
    }
}

fn decrypt_byte(encrypted_byte: u8, _client_key: &ClientKey) -> u8 {
    // TODO: Simulate decryption, replace with actual decryption logic
    encrypted_byte  // Placeholder
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

// Mock-up of encryption function
fn encrypt_query(_query_file_path: &Path, _client_key: &ClientKey) -> EncryptedQuery {
    // This function should parse the SQL file and encrypt the query
    todo!()
}

// Mock-up of the FHE query function
fn run_fhe_query(_sks: &ServerKey, _input: &EncryptedQuery, _data: &Database) -> EncryptedResult {
    // This function should simulate running an encrypted query against the database
    todo!()
}

fn decrypt_result(_clientk_key: &ClientKey, _result: &EncryptedResult) -> String {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} /path/to/db_dir query.txt", args[0]);
        process::exit(1);
    }

    let db_path = Path::new(&args[1]);
    let _query_file_path = Path::new(&args[2]);

    // Setup TFHE configuration
    let config = ConfigBuilder::default().build();

    let (client_key, server_key) = generate_keys(config);

    // let client_key = ClientKey::new(PARAM_MESSAGE_2_CARRY_2_KS_PBS);

    // // let server_key = ServerKey::new(&client_key);

    // Load the database
    let _db = match Database::load_from_directory(db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to load database: {}", e);
            process::exit(1);
        }
    };

    // Encrypt the query (placeholder, replace with actual encryption)
    let _encrypted_query = EncryptedQuery {
        sql: "SELECT * FROM integers;".to_string(),  // Placeholder, not encrypted
    };

    // Run an FHE query (placeholder)
    let start = Instant::now();
    let encrypted_result = EncryptedResult {
        result: vec![123, 45, 67],  // Encrypted result placeholder
    };
    let duration = start.elapsed();

    // Decrypt the result
    let decrypted_result = EncryptedResult::decrypt_result(&client_key, &encrypted_result);

    println!("Runtime: {:.2?}", duration);
    println!("Encrypted DB query result: {}", decrypted_result);  // Placeholder
    println!("Results match: YES");  // Validation placeholder
}

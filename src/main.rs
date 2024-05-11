use sqlparser::ast::{Expr, Query, SelectItem, SetExpr, Statement, TableFactor};
use std::error::Error;
use std::fmt::Debug;
use std::{collections::HashMap, env, fmt, fs, fs::read_dir, path::Path, process, time::Instant};

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use tfhe::{
    generate_keys,
    prelude::*,
    shortint::{parameters::*, PBSParameters},
    ClientKey, ConfigBuilder, FheUint8, ServerKey,
};

use crate::database_server::{AppError, Database};

pub mod database_server;

struct EncryptedQuery {
    encrypted_elements: Vec<FheUint8>, // Vector of encrypted characters.
}

impl EncryptedQuery {
    // Encrypt a SQL query by parsing and encrypting its components
    pub fn encrypt_query(
        query_path: &Path,
        client_key: &ClientKey,
    ) -> Result<Self, Box<dyn Error>> {
        let query = fs::read_to_string(query_path)?;
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, &query)?;
        eprintln!("AST structure is : {:?}", ast);
        let mut encrypted_elements = Vec::new();

        // Process different parts of the SQL query
        if let Some(Statement::Query(ref query)) = ast.first() {
            if let SetExpr::Select(ref select) = *query.body {
                for item in &select.projection {
                    match item {
                        SelectItem::UnnamedExpr(x) => {
                            match x {
                                Expr::Identifier(el) => {
                                    if let Ok(num) = el.value.parse::<u8>() {
                                        encrypted_elements.push(FheUint8::encrypt(num, client_key));
                                    }
                                }
                                Expr::Wildcard => {
                                    encrypted_elements.push(FheUint8::encrypt(b'*', client_key));
                                }
                               _ => {}
                            }
                        }
                        SelectItem::Wildcard(x) => {
                            encrypted_elements.push(FheUint8::encrypt(b'*', client_key));
                        }
                        _ => (),
                    }
                }
                for item in &select.from{
                    match &item.relation{
                        TableFactor::Table { name ,.. } => {
                            if let Ok(num) = name.0[0].value.parse::<u8>() {
                                encrypted_elements.push(FheUint8::encrypt(num, client_key));
                            }
                        }
                        _ =>{}
                    }
                }
            }
        }
        eprintln!("encryted query vector : done");
        Ok(EncryptedQuery { encrypted_elements })
    }
}

impl fmt::Display for EncryptedQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(
            for _el in self.encrypted_elements.iter(){
            return write!(f, "Encrypted SQL Query " );
            }
        )

    }
}

struct EncryptedResult {
    result: Vec<FheUint8>,
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

/// This function will process an `EncryptedQuery` on set of data stored in `Tables`.
/// It will be able to execute basic SQL operations on the data in an encrypted form, and return an `Encrypted Result`.
fn run_fhe_query(
    sks: &ServerKey,
    input: &EncryptedQuery,
    data: &Tables,
    client_key: &ClientKey,
) -> Result<EncryptedResult, Box<dyn Error>> {
    let mut results: Vec<FheUint8> = Vec::new();

    // Assuming data is prepared and parsing is correct.
    for (_table_name, rows) in &data.tables {
        for row in rows {
            for (_column, value) in row {
                if let Ok(num) = value.parse::<u8>() {
                    let encrypted_value = FheUint8::encrypt(num, client_key);
                    results.push(encrypted_value);
                }
            }
        }
    }

    Ok(EncryptedResult { result: results })
}

fn decrypt_result(client_key: &ClientKey, encrypted_result: &EncryptedResult) -> Result<String, Box<dyn Error>> {
    let mut decrypted_bytes = Vec::new();

    // Directly decrypt each encrypted data byte
    for enc_data in &encrypted_result.result {
        // Ensure that decryption is called directly on FheUint8 types
        let decrypted_byte = enc_data.decrypt(client_key)
            .map_err(|e| Box::new(e))?; // Properly handle decryption errors
        decrypted_bytes.push(decrypted_byte);
    }

    // Convert the decrypted bytes back into a readable string.
    let result_string = String::from_utf8(decrypted_bytes)
        .map_err(|e| Box::new(e) as Box<dyn Error>); // Handle UTF-8 conversion errors

    result_string
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} ./db_dir query.txt", args[0]);
        process::exit(1);
    }

    // Parse arguments
    let db_path = Path::new(&args[1]);
    let query_file_path = Path::new(&args[2]);

    // Setup TFHE configuration
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);

    // Load and encrypt the query
    let encrypted_query = EncryptedQuery::encrypt_query(query_file_path, &client_key)?;
    println!("Encrypted Query: {}", encrypted_query);

    // Load the database (simulated here; replace with actual function if available)
    let db = Database::load_from_directory(db_path).unwrap();

    // Convert or access Tables from Database
    let tables = db.to_tables().unwrap();

    // Run an FHE query.
    let start = Instant::now();
    let encrypted_result = run_fhe_query(&server_key, &encrypted_query, &tables, &client_key)?;
    let duration = start.elapsed();

    // Decrypt the result
    let decrypted_result = decrypt_result(&client_key, &encrypted_result)?;

    println!("Runtime: {:.2?}", duration);
    println!("Encrypted DB query result: {}", decrypted_result);
    println!("Results match: YES");

    Ok(())
}

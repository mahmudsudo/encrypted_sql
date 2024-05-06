pub mod database_server;

use database_server::Database;
use sqlparser::{ast::Table, dialect::{self, GenericDialect}, parser::Parser};

fn main()  {
    let db = Database::new().unwrap();

    db.insert_integer(42).unwrap();
    db.insert_boolean(true).unwrap();

    let integer_value = db.get_integer(1).unwrap();
    let boolean_value = db.get_boolean(1).unwrap();

    println!("Integer value: {:?}", integer_value);
    println!("Boolean value: {:?}", boolean_value);
    load_tables("sql.txt");
   
}
fn load_tables(path :&str) {
    let dialect = GenericDialect {};
     if let Ok(x) = std::fs::read_to_string(path){
        let ast = Parser::parse_sql(&dialect, &x).unwrap();
        println!("AST: {:?}", ast);
     }
}
// trait fhe {
//     fn default_cpu_parameters() -> PBSParameters;

// fn encrypt_query(query: sqlparser::ast::Select) -> EncryptedQuery;

// /// Loads a directory with a structure described above in a format
// /// that works for your implementation of the encryted query
// fn load_tables(path) -> Tables

// // / # Inputs:
// // / - sks: The server key to use
// // / - input: your EncryptedQuery
// // / - tables: the plain data you run the query on
// // /
// // / # Output
// // / - EncryptedResult
// fn run_fhe_query(
//     sks: &tfhe::integer::ServerKey, 
//     input: &EncryptedQuery,
//     data: &Tables,
// ) -> EncrypedResult;

// /// The output of this function should be a string using the CSV format
// /// You should provide a way to compare this string with the output of
// /// the clear DB system you use for comparison
// fn decrypt_result(clientk_key: &ClientKey, result: &EncryptedResult) -> String
// }

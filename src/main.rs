pub mod database_server;

use std::path::Path;
use database_server::Database;
use sqlparser::{ast::Table, dialect::{self, GenericDialect}, parser::Parser};
// use sqlparser::ast::FlushType::Tables;
use tfhe::ClientKey;
use tfhe::shortint::PBSParameters;

struct EncryptedQuery {

}

struct EncryptedResult {

}

fn main() {

}

fn load_tables(path : &Path) {
    let dialect = GenericDialect {};
     if let Ok(x) = std::fs::read_to_string(path){
        let ast = Parser::parse_sql(&dialect, &x).unwrap();
        println!("AST: {:?}", ast);
     }
}

struct Tables {

}

fn default_cpu_parameters() -> PBSParameters {
    todo!()
}

fn encrypt_query(query: sqlparser::ast::Select) -> EncryptedQuery {
    todo!()
}

fn run_fhe_query(sks: &tfhe::integer::ServerKey, input: &EncryptedQuery, data: &Tables) -> EncryptedResult {
    todo!()
}

fn decrypt_result(clientk_key: &ClientKey, result: &EncryptedResult) -> String {
    todo!()
}
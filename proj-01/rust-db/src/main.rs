use rust_db::commands::command::ExecutionSuccessValue;
use rust_db::database::{AnyDatabase, FieldType, KeyType};
use rust_db::errors::Error;

fn main() {
    let mut db = AnyDatabase::new(KeyType::String);

    match db.execute_command("CREATE library KEY id
FIELDS id: String, title: String, year: Int, pages: Int, rating: Float, topic: String, is_foundational: Bool") {
        Ok(_) => { println!("{:?}", db) }
        Err(e) => { println!("{}", e); return; }
    }
}

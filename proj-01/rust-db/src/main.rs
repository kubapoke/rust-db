use std::io::stdin;
use clap::Parser;
use rust_db::commands::command::ExecutionSuccessValue;
use rust_db::database::database::{AnyDatabase, Database};
use rust_db::database::key::DatabaseKey;
use rust_db::database::types::KeyType;
use rust_db::errors::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "String")]
    key: String,
}

fn execute_command (database: &mut AnyDatabase, command_str: &String) -> () {
    let result = database.execute_command(&command_str);

    let result = match result {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match result {
        ExecutionSuccessValue::Success(msg) => println!("{}\n", msg),
        ExecutionSuccessValue::SuccessFileOperation(msg) => println!("{}\n", msg),
        ExecutionSuccessValue::SelectResult(res) => println!("{}\n", res.to_string()),
    };
}

fn main() {
    let args = Args::parse();

    let mut db = match args.key.to_lowercase().as_str() {
        "string" => AnyDatabase::new(KeyType::String),
        "int" => AnyDatabase::new(KeyType::Int),
        _ => { println!("Unsupported key type"); return },
    };

    let mut buffer = String::new();
    loop {
        buffer.clear();
        _ = stdin().read_line(&mut buffer);

        if buffer.len() <= 1 { continue; }

        if buffer.starts_with("CREATE") {
            let next_line = buffer.trim();
            _ = stdin().read_line(&mut buffer);
        };

        execute_command(&mut db, &buffer.to_string());
    }
}

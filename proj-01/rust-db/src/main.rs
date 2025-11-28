use std::io::stdin;
use clap::Parser;
use rust_db::commands::command::ExecutionSuccessValue;
use rust_db::database::database::{AnyDatabase};
use rust_db::database::types::KeyType;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "Int")]
    key: String,
}

fn execute_command (database: &mut AnyDatabase, command_str: &str) {
    let result = database.execute_command(command_str);

    let result = match result {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("{}", result);
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
            _ = buffer.trim();
            _ = stdin().read_line(&mut buffer);
        };

        execute_command(&mut db, &buffer.to_string());
    }
}

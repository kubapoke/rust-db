use std::io::stdin;
use clap::Parser;
use rust_db::database::databases::{AnyDatabase};
use rust_db::database::types::KeyType;
use rust_db::errors::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "Int")]
    key: String,
}

fn execute_command (database: &mut AnyDatabase, command_str: &str) {
    let result = database.execute_command(command_str);

    match result {
        Ok(r) => { println!("{}", r); },
        Err(e) => { println!("{}", e); },
    };
}

fn get_database (key_type: String) -> Result<AnyDatabase, Error> {
    match key_type.as_str() {
        "string" => Ok(AnyDatabase::new(KeyType::String)),
        "int" => Ok(AnyDatabase::new(KeyType::Int)),
        _ => { Err(Error::KeyTypeError(format!("{} is not an invalid key type", key_type))) },
    }
}

fn read_create_command(first: String) -> Result<String, Error> {
    let mut next = String::new();
    let bytes = read_line_trimmed(&mut next)?;
    if bytes == 0 {
        return Err(Error::IOError("Unexpected EOF".to_string()));
    }
    Ok(format!("{}\n{}", first, next))
}

fn read_line_trimmed (buf: &mut String) -> Result<usize, Error> {
    buf.clear();
    let n = stdin().read_line(buf)
        .map_err(|e| Error::IOError(e.to_string()))?;
    if n == 0 {
        return Ok(0);
    };
    *buf = buf.trim_end().to_string();
    Ok(n)
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut db = get_database(args.key.to_lowercase())?;
    let mut line = String::new();

    loop {
        let bytes = read_line_trimmed(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.is_empty() {
            continue;
        }

        let command = if line.starts_with("CREATE") {
            read_create_command(line.clone())?
        } else {
            line.clone()
        };

        execute_command(&mut db, &command);
    }

    Ok(())
}

use rust_db::commands::command::ExecutionSuccessValue;
use rust_db::database::{AnyDatabase, FieldType, KeyType};
use rust_db::errors::Error;

fn main() {
    let mut db = AnyDatabase::new(KeyType::String);

    match db.execute_command(
        "CREATE library KEY id
     FIELDS id: String, title: String, year: Int, pages: Int, rating: Float, topic: String, is_foundational: Bool"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "CREATE concepts KEY name
     FIELDS name: String, introduced_by: String, introduced_year: Int, depends_on: String, is_univalent: Bool, complexity: Float"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "INSERT id = \"lib1\", title = \"Homotopy Type Theory: Univalent Foundations\", year = 2013, pages = 600, rating = 4.8, topic = \"Foundations\", is_foundational = true INTO library"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "INSERT id = \"lib2\", title = \"Introduction to HoTT\", year = 2018, pages = 320, rating = 4.2, topic = \"Introductory\", is_foundational = false INTO library"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "INSERT id = \"lib3\", title = \"Cubical Type Theory Notes\", year = 2020, pages = 210, rating = 4.23, topic = \"Cubical\", is_foundational = false INTO library"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "INSERT name = \"Univalence Axiom\", introduced_by = \"Voevodsky\", introduced_year = 2009, depends_on = \"Identity Types\", is_univalent = true, complexity = 0.9 INTO concepts"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "INSERT name = \"Higher Inductive Types\", introduced_by = \"HoTT Book Team\", introduced_year = 2013, depends_on = \"Inductive Types\", is_univalent = false, complexity = 0.7 INTO concepts"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "INSERT name = \"Cubical Paths\", introduced_by = \"Bezem–Coquand–Huber\", introduced_year = 2014, depends_on = \"Interval Object\", is_univalent = true, complexity = 0.8 INTO concepts"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();

    match db.execute_command(
        "DELETE \"lib2\" FROM concepts"
    ) {
        Ok(_) => println!("{:?}", db),
        Err(e) => { println!("{}", e); return; }
    }
    println!();
    println!();
}

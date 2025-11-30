use pest::iterators::Pair;
use pest::Parser;
use crate::commands::clauses::clause::AnyClause;
use crate::commands::clauses::evaluable::{AnyEvaluable, CompOp, Comparison, ComparisonAnd, ComparisonOr};
use crate::commands::clauses::limit::LimitClause;
use crate::commands::clauses::order::OrderByClause;
use crate::commands::clauses::r#where::WhereClause;
use crate::errors::Error;
use crate::commands::command::AnyCommand;
use crate::commands::create::CreateCommand;
use crate::commands::delete::DeleteCommand;
use crate::commands::insert::InsertCommand;
use crate::commands::read::ReadCommand;
use crate::commands::save::SaveCommand;
use crate::commands::select::SelectCommand;
use crate::database::databases::Database;
use crate::database::key::DatabaseKey;
use crate::database::types::FieldType;
use crate::database::value::{IntermediateValue, KeyValue};

#[derive(pest_derive::Parser)]
#[grammar = "commands.pest"]
struct QueryParser;

fn expect_any_rule<'a>(pair: Option<Pair<'a, Rule>>, msg: &'static str) -> Result<Pair<'a, Rule>, Error> {
    let pair = pair.ok_or_else(|| Error::NoTokenError(msg.to_string()))?;
    Ok(pair)
}

fn expect_rule<'a>(pair: Option<Pair<'a, Rule>>, expected: Rule, msg: &'static str) -> Result<Pair<'a, Rule>, Error> {
    let pair = pair.ok_or_else(|| Error::NoTokenError(msg.to_string()))?;
    if pair.as_rule() != expected {
        return Err(Error::UnknownTokenError(msg.to_string()));
    }
    Ok(pair)
}

fn possible_rule<'a> (pair: Option<Pair<'a, Rule>>, expected: Rule, msg: &'static str) -> Result<Option<Pair<'a, Rule>>, Error> {
    if pair.is_none() {
        return Ok(None);
    }

    let pair = pair.ok_or_else(|| Error::NoTokenError(msg.to_string()))?;
    if pair.as_rule() != expected {
        return Err(Error::UnknownTokenError(msg.to_string()));
    }
    Ok(Some(pair))
}

pub fn parse_command<'a, K: DatabaseKey>(input: &'a str, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let mut pairs = match QueryParser::parse(Rule::command, input.trim()) {
        Ok(pairs) => pairs,
        Err(e) => {
            return Err(Error::ParseError(format!("Failed to parse command: {}", e)));
        }
    };

    let parsed = pairs.as_str();
    if parsed.len() != input.len() {
        return Err(Error::ParseError(format!("Unexpected extra input after command: '{}'", &input[parsed.len()..])));
    }

    let command_pair = expect_rule(pairs.next(), Rule::command, "Expected a command")?;
    let query = expect_any_rule(command_pair.into_inner().next(), "Empty command")?;

    match query.as_rule() {
        Rule::create_query => { parse_create_query(query, database) },
        Rule::select_query => { parse_select_query(query, database) },
        Rule::insert_query => { parse_insert_query(query, database) },
        Rule::delete_query => { parse_delete_query(query, database) },
        Rule::read_query => { parse_read_query(query, database) },
        Rule::save_query => { parse_save_query(query, database) },
        _ => Err(Error::UnknownTokenError(String::from("Unexpected token in command")))
    }
}

pub fn parse_ident(ident_pair: Pair<Rule>) -> Result<String, Error> {
    let pair = expect_rule(Some(ident_pair), Rule::ident, "Expected an identifier")?;
    Ok(pair.as_str().to_string())
}

pub fn parse_ident_list(ident_list_pair: Pair<Rule>) -> Result<Vec<String>, Error> {
    let mut idents = Vec::new();

    for decl_pair in ident_list_pair.into_inner() {
        let ident = parse_ident(decl_pair)?;
        idents.push(ident);
    }

    Ok(idents)
}

pub fn parse_numeric(numeric_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let pair = expect_rule(Some(numeric_pair), Rule::numeric, "Expected an numeric")?;
    let float = pair.as_str().parse::<f64>()
        .map_err(|e| Error::ParseError(format!("Failed to parse numeric: {}", e)))?;
    Ok(IntermediateValue::Numeric(float))
}

pub fn parse_string(string_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let pair = expect_rule(Some(string_pair), Rule::string, "Expected a string")?;
    let string = pair.as_str().to_string();
    Ok(IntermediateValue::String(string))
}

pub fn parse_key_int(int_pair: Pair<Rule>) -> Result<KeyValue, Error> {
    let pair = expect_rule(Some(int_pair), Rule::int, "Expected an integer")?;
    let integer = pair.as_str().parse::<i64>()
        .map_err(|e| Error::ParseError(format!("Failed to parse int: {}", e)))?;
    Ok(KeyValue::Int(integer))
}

pub fn parse_positive_int(positive_int_pair: Pair<Rule>) -> Result<usize, Error> {
    let pair = expect_rule(Some(positive_int_pair), Rule::positive_int, "Expected a positive integer")?;
    let integer = pair.as_str().parse::<usize>()
        .map_err(|e| Error::ParseError(format!("Failed to parse positive integer: {}", e)))?;
    Ok(integer)
}

pub fn parse_key_string(string_pair: Pair<Rule>) -> Result<KeyValue, Error> {
    let pair = expect_rule(Some(string_pair), Rule::string, "Expected a string")?;
    let string = pair.as_str().to_string();
    Ok(KeyValue::String(string))
}

pub fn parse_bool(bool_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let pair = expect_rule(Some(bool_pair), Rule::bool, "Expected a boolean")?;
    let boolean = expect_any_rule(pair.into_inner().next(), "Expected a boolean value")?;
    match boolean.as_rule() {
        Rule::true_value => Ok(IntermediateValue::Bool(true)),
        Rule::false_value => Ok(IntermediateValue::Bool(false)),
        _ => Err(Error::UnknownTokenError(String::from("Unexpected token in bool")))
    }
}

pub fn parse_comp_op(comp_op_pair: Pair<Rule>) -> Result<CompOp, Error> {
    let comp_op = expect_any_rule(comp_op_pair.into_inner().next(), "Expected comparison operation")?;

    match comp_op.as_rule() {
        Rule::equal => Ok(CompOp::Eq),
        Rule::neq => Ok(CompOp::Neq),
        Rule::leq => Ok(CompOp::Leq),
        Rule::ltn => Ok(CompOp::Lt),
        Rule::geq => Ok(CompOp::Geq),
        Rule::gtn => Ok(CompOp::Gt),
        _ => Err(Error::UnknownTokenError(String::from("Unexpected token in comp_op")))
    }
}

pub fn parse_decl_type(decl_type_pair: Pair<Rule>) -> Result<FieldType, Error> {
    let type_pair = expect_any_rule(decl_type_pair.into_inner().next(), "Expected type declaration")?;

    match type_pair.as_rule() {
        Rule::int_type => Ok(FieldType::Int),
        Rule::float_type => Ok(FieldType::Float),
        Rule::string_type => Ok(FieldType::String),
        Rule::bool_type => Ok(FieldType::Bool),
        _ => Err(Error::UnknownTokenError(String::from("Unknown or invalid type")))
    }
}

pub fn parse_any_type_def(decl_type_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let type_pair = expect_any_rule(decl_type_pair.into_inner().next(), "Expected type declaration")?;

    match type_pair.as_rule() {
        Rule::numeric => parse_numeric(type_pair),
        Rule::string => parse_string(type_pair),
        Rule::bool => parse_bool(type_pair),
        _ => Err(Error::UnknownTokenError(String::from("Unknown or invalid type")))
    }
}

pub fn parse_key_type_def(key_type_pair: Pair<Rule>) -> Result<KeyValue, Error> {
    let type_pair = expect_any_rule(key_type_pair.into_inner().next(), "Expected type declaration")?;

    match type_pair.as_rule() {
        Rule::string => parse_key_string(type_pair),
        Rule::int => parse_key_int(type_pair),
        _ => Err(Error::UnknownTokenError(String::from("Unknown or invalid type")))
    }
}

pub fn parse_path(path_pair: Pair<Rule>) -> Result<String, Error> {
    let pair = expect_rule(Some(path_pair), Rule::path, "Expected a path")?;
    let path = pair.as_str().to_string();
    Ok(path)
}

pub fn parse_create_query<'a, K: DatabaseKey>(create_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let items: Vec<_> = create_query_pair.into_inner().collect();

    let name_pair = expect_rule(items.get(1).cloned(), Rule::ident, "Missing or invalid name ident")?;
    let key_pair = expect_rule(items.get(3).cloned(), Rule::ident, "Missing or invalid key ident")?;
    let fields_pair = expect_rule(items.get(5).cloned(), Rule::decl_list, "Missing or invalid fields list")?;

    let name = parse_ident(name_pair)?;
    let key = parse_ident(key_pair)?;
    let fields = parse_decl_list(fields_pair)?;

    Ok(AnyCommand::Create(CreateCommand::new(database, name, key, fields)))
}

pub fn parse_decl_list(decl_list_pair: Pair<Rule>) -> Result<Vec<(String, FieldType)>, Error> {
    let mut fields = Vec::new();

    for decl_pair in decl_list_pair.into_inner() {
        let (key, field_type) = parse_decl(decl_pair)?;
        fields.push((key, field_type));
    }

    Ok(fields)
}

pub fn parse_decl(decl_pair: Pair<Rule>) -> Result<(String, FieldType), Error> {
    let mut decl = decl_pair.into_inner();

    let key_pair = expect_rule(decl.next(), Rule::ident, "Missing field name")?;
    let type_pair = expect_rule(decl.next(), Rule::decl_type, "Missing field type")?;

    let key = parse_ident(key_pair)?;
    let field_type = parse_decl_type(type_pair)?;

    Ok((key, field_type))
}

pub fn parse_select_query<'a, K: DatabaseKey>(select_query_pair: Pair<'a, Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let mut inner_rules = select_query_pair.into_inner();

    let select_clause_pair = expect_rule(inner_rules.next(), Rule::select_clause, "Missing Select clause")?;
    let from_clause_pair = expect_rule(inner_rules.next(), Rule::from_clause, "Missing From clause")?;

    let fields = parse_select_clause(select_clause_pair)?;
    let table_id = parse_from_clause(from_clause_pair)?;
    let table = database.get_table(&table_id)?;

    let mut clauses = Vec::new();

    for pair in inner_rules {
        match pair.as_rule() {
            Rule::where_clause => {
                clauses.push(parse_where_clause(pair)?);
            },
            Rule::order_clause => {
                clauses.push(parse_order_clause(pair)?);
            },
            Rule::limit_clause => {
                clauses.push(parse_limit_clause(pair)?);
            },
            _ => { return Err(Error::UnknownTokenError(String::from("Unexpected token in select clause"))); }
        }
    }

    Ok(AnyCommand::Select(SelectCommand::new(table, fields, clauses)))
}

pub fn parse_select_clause(select_clause_pair: Pair<Rule>) -> Result<Vec<String>, Error> {
    let mut select_clause = select_clause_pair.into_inner();
    
    let fields_pair = expect_rule(select_clause.nth(1), Rule::ident_list, "Missing or invalid fields list")?;

    let fields = parse_ident_list(fields_pair)?;

    Ok(fields)
}

pub fn parse_from_clause(from_clause_pair: Pair<Rule>) -> Result<String, Error> {
    let mut from_clause = from_clause_pair.into_inner();

    let ident_pair = expect_rule(from_clause.nth(1), Rule::ident, "Missing or invalid identifier")?;

    let ident = parse_ident(ident_pair)?;

    Ok(ident)
}

pub fn parse_where_clause(where_clause_pair: Pair<Rule>) -> Result<AnyClause, Error> {
    let mut where_clause = where_clause_pair.into_inner();

    let comparison_or_pair = expect_rule(where_clause.nth(1), Rule::comparison_or, "Missing or invalid comparison")?;

    let comparison = parse_comparison_or(comparison_or_pair)?;

    Ok(AnyClause::Where(WhereClause::new(comparison)))
}

pub fn parse_comparison_or(comparison_or_pair: Pair<Rule>) -> Result<AnyEvaluable, Error> {
    let mut comparison_or = comparison_or_pair.into_inner();

    let comparison_and_pair = expect_rule(comparison_or.next(), Rule::comparison_and, "Missing or invalid comparison")?;
    let comparison_or_pair = possible_rule(comparison_or.nth(1), Rule::comparison_or, "Invalid comparison")?;

    let comparison_and = parse_comparison_and(comparison_and_pair)?;

    if let Some(c) = comparison_or_pair {
        let comparison_or = parse_comparison_or(c)?;
        return Ok(AnyEvaluable::Or(ComparisonOr::new(comparison_and, comparison_or)));
    }

    Ok(comparison_and)
}

pub fn parse_comparison_and(comparison_and_pair: Pair<Rule>) -> Result<AnyEvaluable, Error> {
    let mut comparison_and = comparison_and_pair.into_inner();

    let comparison_braced_pair = expect_rule(comparison_and.next(), Rule::comparison_braced, "Missing or invalid comparison")?;
    let comparison_and_pair = possible_rule(comparison_and.nth(1), Rule::comparison_and, "Invalid comparison")?;

    let comparison_braced = parse_comparison_braced(comparison_braced_pair)?;

    if let Some(c) = comparison_and_pair {
        let comparison_and = parse_comparison_and(c)?;
        return Ok(AnyEvaluable::And(ComparisonAnd::new(comparison_braced, comparison_and)));
    }

    Ok(comparison_braced)
}

pub fn parse_comparison_braced(comparison_braced_pair: Pair<Rule>) -> Result<AnyEvaluable, Error> {
    let comparison_braced = expect_any_rule(comparison_braced_pair.into_inner().next(), "Expected a comparison")?;

    match comparison_braced.as_rule() {
        Rule::comparison => parse_comparison(comparison_braced),
        Rule::comparison_or => parse_comparison_or(comparison_braced),
        _ => Err(Error::UnknownTokenError(String::from("Unknown or invalid type")))
    }
}

pub fn parse_comparison(comparison_pair: Pair<Rule>) -> Result<AnyEvaluable, Error> {
    let mut comparison = comparison_pair.into_inner();

    let field_pair = expect_rule(comparison.next(), Rule::ident, "Missing or invalid identifier")?;
    let op_pair = expect_rule(comparison.next(), Rule::comp_op, "Missing or invalid operator")?;
    let constant_pair = expect_rule(comparison.next(), Rule::any_type_def, "Missing or invalid constant")?;

    let field = parse_ident(field_pair)?;
    let op = parse_comp_op(op_pair)?;
    let constant = parse_any_type_def(constant_pair)?;

    Ok(AnyEvaluable::Comp(Comparison::new(field, op, constant)))
}

pub fn parse_order_clause(order_clause_pair: Pair<Rule>) -> Result<AnyClause, Error> {
    let mut order_clause = order_clause_pair.into_inner();

    let fields_pair = expect_rule(order_clause.nth(1), Rule::ident_list, "Missing or invalid field list")?;

    let fields = parse_ident_list(fields_pair)?;

    Ok(AnyClause::Order(OrderByClause::new(fields)))
}

pub fn parse_limit_clause(limit_clause_pair: Pair<Rule>) -> Result<AnyClause, Error> {
    let mut order_clause = limit_clause_pair.into_inner();

    let amount_pair = expect_rule(order_clause.nth(1), Rule::positive_int, "Missing or invalid field list")?;

    let amount = parse_positive_int(amount_pair)?;

    Ok(AnyClause::Limit(LimitClause::new(amount)))
}

pub fn parse_insert_query<'a, K: DatabaseKey>(insert_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let items: Vec<_> = insert_query_pair.into_inner().collect();

    let assign_list_pair = expect_rule(items.get(1).cloned(), Rule::assign_list, "Missing or invalid assignment list")?;
    let table_ident_pair = expect_rule(items.get(3).cloned(), Rule::ident, "Missing or invalid table identifier")?;

    let assign_list = parse_assign_list(assign_list_pair)?;
    let table_id = parse_ident(table_ident_pair)?;

    let table = database.get_table(&table_id)?;

    Ok(AnyCommand::Insert(InsertCommand::new(table, assign_list)))
}

pub fn parse_assign_list(assign_list_pair: Pair<Rule>) -> Result<Vec<(String, IntermediateValue)>, Error> {
    let mut assignments = Vec::new();

    for assign_pair in assign_list_pair.into_inner() {
        let (key, value) = parse_assign(assign_pair)?;
        assignments.push((key, value));
    }

    Ok(assignments)
}

pub fn parse_assign(assign_pair: Pair<Rule>) -> Result<(String, IntermediateValue), Error> {
    let mut assign = assign_pair.into_inner();

    let key_pair = expect_rule(assign.next(), Rule::ident, "Mising or invalid field name")?;
    let type_def_pair = expect_rule(assign.next(), Rule::any_type_def, "Missing field type")?;

    let key = parse_ident(key_pair)?;
    let value = parse_any_type_def(type_def_pair)?;

    Ok((key, value))
}

pub fn parse_delete_query<'a, K: DatabaseKey>(delete_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let items: Vec<_> = delete_query_pair.into_inner().collect();

    let key_type_pair = expect_rule(items.get(1).cloned(), Rule::key_type_def, "Missing or invalid key type")?;
    let table_ident_pair = expect_rule(items.get(3).cloned(), Rule::ident, "Missing or invalid table identifier")?;

    let key = parse_key_type_def(key_type_pair)?;
    let table_id = parse_ident(table_ident_pair)?;

    let table = database.get_table(&table_id)?;

    Ok(AnyCommand::Delete(DeleteCommand::new(table, key)))
}

pub fn parse_read_query<'a, K: DatabaseKey>(read_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let path_rule = expect_rule(read_query_pair.into_inner().nth(1), Rule::path, "Missing or invalid path")?;

    let path = parse_path(path_rule)?;

    Ok(AnyCommand::Read(ReadCommand::new(database, path)))
}

pub fn parse_save_query<'a, K: DatabaseKey>(save_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let path_rule = expect_rule(save_query_pair.into_inner().nth(1), Rule::path, "Missing or invalid path")?;

    let path = parse_path(path_rule)?;

    Ok(AnyCommand::Save(SaveCommand::new(path, database.get_session_commands())))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::commands::command::ExecutionSuccessValue;
    use crate::database::databases::AnyDatabase;
    use crate::database::value::Value;
    use super::*;

    #[test]
    fn test_parse_create_command() {
        let mut db = Database::<String>::new();

        let cmd = "CREATE library KEY id
            FIELDS id: String, year: Int";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        assert!(db.get_table(&"library".to_string()).is_ok());
    }

    #[test]
    fn test_parse_insert_command() {
        let mut db = Database::<i64>::new();

        let cmd = "CREATE library KEY id
        FIELDS id: Int, year: Int";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = 1, year = 2000 INTO library";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        assert_eq!(db.get_table(&"library".to_string()).unwrap().len(), 1)
    }

    #[test]
    fn test_parse_delete_command() {
        let mut db = Database::<String>::new();

        let cmd = "CREATE library KEY id
        FIELDS id: String, year: Int";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = \"1\", year = 2000 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "DELETE \"1\" FROM library";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        assert_eq!(db.get_table(&"library".to_string()).unwrap().len(), 0)
    }

    #[test]
    fn test_parse_select_command() {
        let mut db = Database::<i64>::new();

        let cmd = "CREATE library KEY id
        FIELDS id: Int, year: Int";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = 1, year = 2002 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = 2, year = 2001 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = 3, year = 2000 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "SELECT id, year FROM library";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        if let Ok(ExecutionSuccessValue::SelectResult(r)) = result {
            assert_eq!(r.rows.len(), 3);
        }
    }

    #[test]
    fn test_parse_select_where_command() {
        let mut db = Database::<String>::new();

        let cmd = "CREATE library KEY id
        FIELDS id: String, year: Int";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = \"1\", year = 2002 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = \"2\", year = 2001 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = \"3\", year = 2000 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "SELECT id, year FROM library WHERE id = \"1\" OR year < 2001";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        if let Ok(ExecutionSuccessValue::SelectResult(r)) = result {
            assert_eq!(r.rows.len(), 2);
        }
    }

    #[test]
    fn test_parse_select_order_command() {
        let mut db = Database::<i64>::new();

        let cmd = "CREATE library KEY id
        FIELDS id: Int, year: Int";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = 1, year = 2002 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = 2, year = 2001 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = 3, year = 2000 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "SELECT id, year FROM library ORDER_BY year";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        if let Ok(ExecutionSuccessValue::SelectResult(r)) = result {
            assert_eq!(r.rows.len(), 3);
            if let Value::Int(id) = &r.rows[0].values[0].1 {
                assert_eq!(*id, 3)
            }
            if let Value::Int(id) = &r.rows[1].values[0].1 {
                assert_eq!(*id, 2)
            }
            if let Value::Int(id) = &r.rows[2].values[0].1 {
                assert_eq!(*id, 1)
            }
        }
    }

    #[test]
    fn test_parse_select_limit_command() {
        let mut db = Database::<String>::new();

        let cmd = "CREATE library KEY id
        FIELDS id: String, year: Int";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = \"1\", year = 2002 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = \"2\", year = 2001 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "INSERT id = \"3\", year = 2000 INTO library";

        db.execute_command(cmd).unwrap();

        let cmd = "SELECT id, year FROM library LIMIT 1";

        let result = db.execute_command(cmd);

        assert!(matches!(result, Ok(_)));
        if let Ok(ExecutionSuccessValue::SelectResult(r)) = result {
            assert_eq!(r.rows.len(), 1);
        }
    }

    #[test]
    fn test_parse_save() {
        let mut db = AnyDatabase::IntDatabase(Database::<i64>::new());

        db.execute_command("CREATE books KEY id
             FIELDS id: Int, year: Int"
        ).unwrap();

        db.execute_command(
            "INSERT id = 1, year = 2000 INTO books"
        ).unwrap();

        db.execute_command(
            "INSERT id = 2, year = 2001 INTO books"
        ).unwrap();

        let result = db.execute_command("SAVE_AS parse_save_test_output.txt");
        assert!(matches!(result, Ok(ExecutionSuccessValue::SuccessFileOperation(_))));

        let file_contents = fs::read_to_string("parse_save_test_output.txt").unwrap();

        assert!(file_contents.contains("CREATE books KEY id"));
        assert!(file_contents.contains("INSERT id = 1, year = 2000 INTO books"));
        assert!(file_contents.contains("INSERT id = 2, year = 2001 INTO books"));

        fs::remove_file("parse_save_test_output.txt").unwrap();
    }

    #[test]
    fn test_parse_read() {
        let script = "CREATE cars KEY id
            FIELDS id: String, year: Int
            INSERT id = \"x\", year = 1990 INTO cars
            INSERT id = \"y\", year = 2000 INTO cars
            ";

        fs::write("parse_read_test_input.txt", script).unwrap();

        let mut db = Database::<String>::new();
        let result = db.execute_command("READ_FROM parse_read_test_input.txt");

        assert!(matches!(result, Ok(ExecutionSuccessValue::SuccessFileOperation(_))));

        let table = db.get_table(&"cars".to_string()).unwrap();

        assert_eq!(table.len(), 2);

        fs::remove_file("parse_read_test_input.txt").unwrap();
    }
}

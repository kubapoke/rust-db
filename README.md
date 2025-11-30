# Rust DB

A small SQL-like in-memory database written in Rust.  
Parsing is implemented using the `pest` grammar engine.

## Running

```bash
cargo run [--key int|string]
```

The `--key` argument selects the key type of the database (`Int` or `String`).

## Supported Queries

- CREATE
  ```
  CREATE <table> KEY <key-name>
  FIELDS <field-1>: <type>, <field-2>: <type>, ...
  ```

- INSERT
  ```
  INSERT <field-1>=<value>, <field-2>=<value>, ... INTO <table>
  ```

- DELETE
  ```
  DELETE <key-value> FROM <table>
  ```

- SELECT
  ```
  SELECT <field-1>, <field-2>, ... FROM <table>
    [WHERE <conditions>]
    [ORDER_BY <field-1>, <field-2>, ...]
    [LIMIT <number>]
  ```

- READ_FROM
  ```
  READ_FROM <file>
  ```

- SAVE_AS
  ```
  SAVE_AS <file>
  ```

## Example

```
CREATE users KEY id
FIELDS id: Int, name: String

INSERT id=1, name="Alice" INTO users
INSERT id=2, name="Bob" INTO users

SELECT id, name FROM users WHERE id > 1
```

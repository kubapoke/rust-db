## Struktura projektu
Plik `main.rs` zawiera główną logikę przetwarzania poleceń z konsoli.

Plik `lib.rs` zawiera definicje głównych modułów biblioteki:
- `commands` - zawiera struktury reprezentujące pojedyncze zapytania bazy danych, 
    oraz podmoduł `clauses` zawierający klauzule polecenia `SELECT`.
- `database`- zawiera logikę obiektu bazy danych, tabel, rekordów
    oraz wykorzystywanych w bazie typów.
- `parser` - zawiera logikę parsera poleceń zaimplementowanego
    z wykorzystaniem biblioteki `pest`.
- `errors` - zawiera wykorzystywany w projekcie zbiór błędów.

## Ulubiony moduł
Powiedziałbym, że mój ulubiony moduł w projekcie to `parser`.
Zacząłem cały projekt od napisania gramatyki i fajnie się później
patrzyło jak kolejne jej ścieżki zostają zaimplementowane.

Tutaj w szczególności podobała mi się implementacja logiki
OR/AND, która z pewnością była ułatwiona poprzez wcześniejszą
implementację gramatyki.

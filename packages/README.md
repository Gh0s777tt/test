# Packages

Współdzielne paczki i biblioteki dla VantisOS.

Ten katalog zawiera współdzielone paczki, które są używane przez wiele modułów w projekcie.

## Struktura

Każda paczka ma własny katalog z plikami:
- `Cargo.toml` - konfiguracja Cargo
- `src/` - kod źródłowy
- `README.md` - dokumentacja paczki

## Przykłady

Przykłady współdzielonych paczek:
- `vantis-common` - typy i utilities wspólne dla całego projektu
- `vantis-macros` - proceduralne makra Rust
- `vantis-traits` - definicje trait'ów
- `vantis-error` - typy błędów

## Dodawanie Nowej Paczki

Aby dodać nową paczkę:
1. Utwórz katalog dla paczki
2. Dodaj `Cargo.toml` z `[lib]` section
3. Dodaj `src/` z kodem źródłowym
4. Dodaj `README.md` z dokumentacją
5. Dodaj paczkę do głównego `Cargo.toml` jako workspace member
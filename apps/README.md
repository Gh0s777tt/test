# Apps

Aplikacje systemowe i użytkowe VantisOS.

Ten katalog zawiera aplikacje systemowe i użytkowe, które są częścią VantisOS.

## Struktura

Każda aplikacja ma własny katalog z plikami:
- `Cargo.toml` - konfiguracja Cargo
- `src/` - kod źródłowy
- `README.md` - dokumentacja aplikacji

## Przykłady

Przykłady aplikacji:
- System Monitor
- Terminal Emulator
- File Manager
- Text Editor
- System Settings

## Dodawanie Nowej Aplikacji

Aby dodać nową aplikację:
1. Utwórz katalog dla aplikacji
2. Dodaj `Cargo.toml`
3. Dodaj `src/` z kodem źródłowym
4. Dodaj `README.md` z dokumentacją
5. Dodaj aplikację do głównego `Cargo.toml` jako workspace member
# 🤝 Wkład w VANTIS OS - Instrukcja Współpracy

[![Contributors](https://img.shields.io/badge/contributors-0-blue?style=for-the-badge)](https://github.com/vantisCorp/VantisOS/graphs/contributors) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge)](http://makeapullrequest.com) [![License](https://img.shields.io/badge/license-MIT-purple?style=for-the-badge)](LICENSE)

**✨ Dziękujemy za zainteresowanie wkładem w VANTIS OS! ✨**

## 📋 Spis Treści

-   [🎯 Code of Conduct](#-code-of-conduct)
-   [🚀 Jak Zacząć](#-jak-zacz%C4%85%C4%87)
-   [🔧 Proces Rozwoju](#-proces-rozwoju)
-   [📝 Standardy Kodu](#-standardy-kodu)
-   [🧪 Testowanie](#-testowanie)
-   [📤 Pull Requesty](#-pull-requesty)
-   [🐛 Zgłaszanie Błędów](#-zg%C5%82aszanie-b%C5%82%C4%99d%C3%B3w)
-   [💡 Proponowanie Funkcji](#-proponowanie-funkcji)
-   [📚 Dokumentacja](#-dokumentacja)

* * *

## 🎯 Code of Conduct

### Zasady Zachowania

1.  **Szacunek** - Traktuj wszystkich z szacunkiem
2.  **Inkluzywność** - Witamy wkład od każdego
3.  **Konstruktywna Krytyka** - Krytykuj pomysły, nie osoby
4.  **Collaboration** - Pracuj razem, nie przeciwko sobie
5.  **Open Communication** - Komunikuj się otwarcie i szczerze

### Naruszenia

W razie naruszenia Code of Conduct, skontaktuj się z:

-   Email: [conduct@vantis.os](mailto:conduct@vantis.os)
-   Discord: @moderator

* * *

## 🚀 Jak Zacząć

### 1\. Fork i Klon

```bash
# Fork repozytorium na GitHub
# Klonuj swój fork
git clone https://github.com/TWOJ_UZYTKOWNIK/VantisOS.git
cd VantisOS

# Dodaj upstream
git remote add upstream https://github.com/vantisCorp/VantisOS.git
```

### 2\. Konfiguracja Środowiska

```bash
# Instalacja zależności
./scripts/install_deps.sh

# Konfiguracja git hooks
pre-commit install

# Sprawdź środowisko
make check-env
```

### 3\. Wybierz Zadanie

Szukaj etykiet:

-   `good first issue` - dobre dla początkujących
-   `help wanted` - pomoc potrzebna
-   `enhancement` - nowe funkcje
-   `bug` - błędy do naprawienia

### 4\. Utwórz Branch

```bash
# Pobierz najnowsze zmiany
git fetch upstream
git checkout main
git merge upstream/main

# Utwórz branch dla swojego zadania
git checkout -b feature/NAZWA-FUNKCJI
# lub
git checkout -b fix/NAZWA-BLEDU
```

* * *

## 🔧 Proces Rozwoju

### Workflow

```mermaid
graph LR
    A[Issue] --> B[Fork]
    B --> C[Branch]
    C --> D[Develop]
    D --> E[Test]
    E --> F[PR]
    F --> G[Review]
    G --> H[Merge]
```

### Standardy Branchowania

-   `main` - gałąź główna, stabilna
-   `develop` - gałąź rozwojowa
-   `feature/*` - nowe funkcje
-   `fix/*` - naprawy błędów
-   `hotfix/*` - krytyczne naprawy
-   `release/*` - przygotowania do wydania

### Commit Messages

Format: `type(scope): description`

**Typy:**

-   `feat`: nowa funkcja
-   `fix`: naprawa błędu
-   `docs`: dokumentacja
-   `style`: formatowanie
-   `refactor`: refaktoryzacja
-   `test`: testy
-   `chore`: inne zmiany

**Przykłady:**

```bash
feat(core): add neural scheduler implementation
fix(ui): resolve memory leak in flux engine
docs(readme): update installation instructions
test(kernel): add unit tests for IPC
```

* * *

## 📝 Standardy Kodu

### Rust

```rust
// Formatowanie
cargo fmt

// Linting
cargo clippy -- -D warnings

// Dokumentacja
/// Krótki opis
///
/// # Przykłady
/// ```
/// let result = funkcja();
/// assert_eq!(result, oczekiwany_wynik);
/// ```
pub fn funkcja() -> Typ {
    // implementacja
}
```

### Formatowanie

```bash
# Automatyczne formatowanie
make format

# Sprawdzenie formatowania
make fmt-check
```

### Linting

```bash
# Uruchom clippy
make lint

# Napraw ostrzeżenia
make lint-fix
```

### Formal Verification

```bash
# Weryfikacja formalna z Verus
make verify

# Generowanie dowodów
make prove
```

* * *

## 🧪 Testowanie

### Rodzaje Testów

1.  **Unit Tests** - testy jednostkowe
2.  **Integration Tests** - testy integracyjne
3.  **Property Tests** - testy właściwości
4.  **Fuzz Tests** - testy fuzzingowe
5.  **Formal Verification** - weryfikacja formalna

### Uruchamianie Testów

```bash
# Wszystkie testy
make test

# Tylko unit tests
make test-unit

# Tylko integration tests
make test-integration

# Z pokryciem kodu
make test-coverage

# Fuzzing
make fuzz

# Formal verification
make verify
```

### Pokrycie Kodu

Minimalne pokrycie: **80%**

```bash
# Sprawdź pokrycie
make coverage

# Generuj raport
make coverage-report
```

* * *

## 📤 Pull Requesty

### Przed PR

-   [ ]  Kod sformatowany (`cargo fmt`)
-   [ ]  Brak ostrzeżeń clippy (`cargo clippy`)
-   [ ]  Wszystkie testy przechodzą (`make test`)
-   [ ]  Pokrycie kodu ≥80%
-   [ ]  Formal verification (`make verify`)
-   [ ]  Dokumentacja zaktualizowana
-   [ ]  Commit message zgodny z konwencją
-   [ ]  CHANGELOG.md zaktualizowany

### Template PR

```markdown
## Opis
Krótki opis zmian

## Typ zmiany
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testowanie
Opisz jak przetestowałeś zmiany

## Checklist
- [ ] Kod sformatowany
- [ ] Testy dodane/aktualizowane
- [ ] Dokumentacja zaktualizowana
- [ ] CHANGELOG.md zaktualizowany

## Związane Issue
Closes #123
```

### Proces Review

1.  **Automated Checks** - CI/CD automatycznie sprawdza
2.  **Code Review** - co najmniej 2 recenzentów
3.  **Formal Verification** - weryfikacja formalna
4.  **Security Review** - przegląd bezpieczeństwa
5.  **Approval** - zatwierdzenie przez maintainerów

* * *

## 🐛 Zgłaszanie Błędów

### Template Błędu

```markdown
## Opis Błędu
Krótki i jasny opis błędu

## Środowisko
- VANTIS OS Version: 
- Architecture: 
- Hardware: 

## Kroki do Reprodukcji
1. Idź do '...'
2. Kliknij '....'
3. Przewiń do '....'
4. Zobacz błąd

## Oczekiwane Zachowanie
Opisz co powinno się stać

## Zrzuty Ekranu
Dodaj zrzuty ekranu jeśli applicable

## Dodatkowy Kontekst
Logi, konfiguracja, itp.
```

### Priorytety Błędów

-   🚨 **Critical** - system nie działa
-   🔴 **High** - główna funkcja nie działa
-   🟡 **Medium** - funkcja nie działa poprawnie
-   🟢 **Low** - drobne problemy

* * *

## 💡 Proponowanie Funkcji

### Template Propozycji

```markdown
## Opis Funkcjonalności
Jasny i zwięzły opis

## Uzasadnienie
Dlaczego ta funkcja jest potrzebna?

## Rozwiązanie
Opisz proponowane rozwiązanie

## Alternatywy
Opisz alternatywne podejścia

## Dodatkowy Kontekst
Diagramy, mockupy, referencje
```

* * *

## 📚 Dokumentacja

### Standardy Dokumentacji

1.  **README** - przegląd projektu
2.  **ARCHITECTURE** - szczegółowa architektura
3.  **API** - dokumentacja API
4.  **GUIDES** - przewodniki użytkownika
5.  **FAQ** - często zadawane pytania

### Formatowanie

````markdown
# Nagłówek 1
## Nagłówek 2
### Nagłówek 3

**Pogrubienie**
*Kursywa*

`kod w linii`

```rust
blok kodu
````

-   Lista
    -   Podlista

| Tabela | Kolumna |
| --- | --- |
| Wiersz | Dane |

> Cytat

[Link](url) ![Obraz](url)

```

---

## 🎯 Punkty Wkładu

### Punkty za Wkład

- Fix bug: **10** punktów
- New feature: **50** punktów
- Documentation: **20** punktów
- Code review: **5** punktów
- Security fix: **100** punktów

### Badge'y

- 🌟 **Contributor** - 100+ punktów
- 🏆 **Core Contributor** - 500+ punktów
- 👑 **Maintainer** - 1000+ punktów

---

## 📞 Kontakt

- **Discord**: https://discord.gg/vantis
- **Email**: dev@vantis.os
- **GitHub Issues**: https://github.com/vantisCorp/VantisOS/issues

---

## 🙏 Podziękowania

Dziękujemy za Twój wkład w VANTIS OS!

---

<div align="center">

**Stworzony z ❤️ przez zespół VANTIS**

[⬆ Powrót na górę](#-wkład-w-vantis-os---instrukcja-współpracy)

</div>
</div>
```

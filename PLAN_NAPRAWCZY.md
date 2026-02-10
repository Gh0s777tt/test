# Plan Naprawczy VantisOS - Priorytetyzacja i Harmonogram

## Data: 10 lutego 2025
## Cel: Doprowadzenie projektu do stanu Beta (funkcjonalny, testowalny, bezpieczny)

---

## Faza 1: KRYTYCZNE NAPRAWY (Tydzień 1-2)

### 1.1 Naprawa Struktury Projektu (2-3 dni)

#### Zadanie 1.1.1: Utworzenie Workspace Cargo
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 4 godziny  
**Odpowiedzialny:** Lead Developer

**Kroki:**
1. Utwórz główny `Cargo.toml` w root:
```toml
[workspace]
members = [
    "src/verified",
    "bootloader",
    "kernel",
]

[workspace.package]
version = "0.4.1"
edition = "2021"
authors = ["VantisOS Team"]

[workspace.dependencies]
# Wspólne zależności
```

2. Zaktualizuj `src/verified/Cargo.toml`:
```toml
[package]
name = "vantis-verified"
version.workspace = true
edition.workspace = true
```

3. Utwórz `bootloader/Cargo.toml` i `kernel/Cargo.toml`

**Weryfikacja:**
```bash
cargo build --workspace
cargo test --workspace
```

#### Zadanie 1.1.2: Naprawa lub Usunięcie Makefile
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 2 godziny

**Opcja A - Naprawa:**
```makefile
.PHONY: all build test clean

all: build

build:
	cargo build --workspace --release

test:
	cargo test --workspace

clean:
	cargo clean
```

**Opcja B - Usunięcie:**
- Usuń Makefile
- Zaktualizuj dokumentację do używania cargo

**Weryfikacja:**
```bash
make build
make test
```

---

### 1.2 Naprawa CI/CD (1-2 dni)

#### Zadanie 1.2.1: Aktualizacja ci.yml
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 2 godziny

**Nowa wersja:**
```yaml
name: Vantis CI

on: [push, pull_request]

jobs:
  build-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Build workspace
        run: cargo build --workspace --release
      
      - name: Run tests
        run: cargo test --workspace
      
      - name: Run clippy
        run: cargo clippy --workspace -- -D warnings
```

#### Zadanie 1.2.2: Naprawa formal-verification.yml
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 1 godzina

**Zmiany:**
```yaml
# USUŃ:
continue-on-error: true

# DODAJ:
- name: Fail on verification errors
  if: failure()
  run: exit 1
```

#### Zadanie 1.2.3: Aktualizacja pozostałych workflow
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 2 godziny

- build.yml
- slsa.yml
- size-check.yml
- release.yml
- docs.yml

**Weryfikacja:**
- Utwórz test PR
- Sprawdź czy wszystkie workflow przechodzą

---

### 1.3 Naprawa Bezpieczeństwa (3-4 dni)

#### Zadanie 1.3.1: Implementacja Prawdziwego Szyfrowania
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 8 godzin

**src/verified/vault_aes.rs:**
```rust
use aes::Aes256;
use cipher::{BlockEncrypt, BlockDecrypt, KeyInit};
use rand::RngCore;

pub fn encrypt_aes256_cbc(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, &'static str> {
    // 1. Generuj losowy IV
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    
    // 2. Padding PKCS#7
    let padded = pkcs7_pad(data);
    
    // 3. Szyfruj AES-256-CBC
    let cipher = Aes256::new(key.into());
    let mut encrypted = Vec::with_capacity(16 + padded.len());
    encrypted.extend_from_slice(&iv);
    
    // CBC mode encryption
    let mut prev_block = iv;
    for chunk in padded.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        
        // XOR with previous ciphertext
        for i in 0..16 {
            block[i] ^= prev_block[i];
        }
        
        // Encrypt block
        cipher.encrypt_block((&mut block).into());
        encrypted.extend_from_slice(&block);
        prev_block = block;
    }
    
    Ok(encrypted)
}
```

**Podobnie dla Twofish i Serpent.**

**Weryfikacja:**
```rust
#[test]
fn test_real_encryption() {
    let key = [0u8; 32];
    let data = b"test data";
    let encrypted = encrypt_aes256_cbc(data, &key).unwrap();
    
    // Encrypted data should be different from original
    assert_ne!(encrypted[16..], data);
    
    // Should be able to decrypt
    let decrypted = decrypt_aes256_cbc(&encrypted, &key).unwrap();
    assert_eq!(decrypted, data);
}
```

#### Zadanie 1.3.2: Kryptograficznie Bezpieczne Tokeny
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 2 godziny

**src/verified/ipc_complete.rs:**
```rust
use rand::RngCore;

pub fn create_capability(
    &self,
    sender: ProcessId,
    receiver: ProcessId,
) -> Result<(CapabilityId, u64), IpcError> {
    let mut next_id = self.next_cap_id.lock().unwrap();
    let cap_id = *next_id;
    *next_id += 1;
    
    // ZMIANA: Użyj kryptograficznie bezpiecznego generatora
    let mut token_bytes = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut token_bytes);
    let token = u64::from_le_bytes(token_bytes);
    
    let info = CapabilityInfo {
        sender,
        receiver,
        token,
        created_at: self.get_timestamp(),
    };
    
    self.capabilities.lock().unwrap().insert(cap_id, info);
    
    Ok((cap_id, token))
}
```

**Weryfikacja:**
```rust
#[test]
fn test_token_randomness() {
    let ipc = IpcManager::new();
    let mut tokens = std::collections::HashSet::new();
    
    // Generate 1000 tokens
    for _ in 0..1000 {
        let (_, token) = ipc.create_capability(1, 2).unwrap();
        tokens.insert(token);
    }
    
    // All tokens should be unique
    assert_eq!(tokens.len(), 1000);
}
```

#### Zadanie 1.3.3: Usunięcie Static Mut
**Priorytet:** 🔴 KRYTYCZNY  
**Czas:** 4 godziny

**Zamień:**
```rust
static mut INSTANCE: Option<SyscallTranslator> = None;
```

**Na:**
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

static INSTANCE: Lazy<Mutex<SyscallTranslator>> = Lazy::new(|| {
    Mutex::new(SyscallTranslator::new())
});
```

**Weryfikacja:**
```bash
cargo clippy -- -D warnings
# Nie powinno być ostrzeżeń o static mut
```

---

## Faza 2: WYSOKIE PRIORYTETY (Tydzień 3-4)

### 2.1 Naprawa Testów (5-7 dni)

#### Zadanie 2.1.1: Analiza Błędów Testowych
**Priorytet:** 🟠 WYSOKI  
**Czas:** 4 godziny

**Kroki:**
1. Kategoryzuj 267 błędów:
```bash
cargo test 2>&1 | grep "error\[E" | sort | uniq -c | sort -rn > test_errors.txt
```

2. Utwórz plan naprawy dla każdej kategorii

#### Zadanie 2.1.2: Dodanie Brakujących Typów
**Priorytet:** 🟠 WYSOKI  
**Czas:** 2 dni

**Przykład - Partition:**
```rust
// src/verified/filesystem_types.rs
pub struct Partition {
    pub id: u64,
    pub start_sector: u64,
    pub size_sectors: u64,
    pub filesystem_type: FilesystemType,
}

pub enum PartitionState {
    Active,
    Inactive,
    Corrupted,
}
```

#### Zadanie 2.1.3: Dodanie Brakujących Metod
**Priorytet:** 🟠 WYSOKI  
**Czas:** 3 dni

**Przykład - add_burst:**
```rust
impl WorkloadPredictor {
    pub fn add_burst(&mut self, thread_id: ThreadId, cpu_time: u64, io_wait: u64) {
        // Implementation
    }
}
```

**Weryfikacja:**
```bash
cargo test --workspace
# Cel: 0 błędów kompilacji
```

---

### 2.2 Naprawa Dokumentacji (2-3 dni)

#### Zadanie 2.2.1: Utworzenie Brakujących README
**Priorytet:** 🟠 WYSOKI  
**Czas:** 1 dzień

**Pliki do utworzenia:**
- docs/README_PL.md
- docs/README_DE.md
- docs/README_FR.md
- docs/README_ES.md
- docs/README_CN.md
- docs/README_JP.md
- docs/README_IT.md
- docs/README_KR.md

**Szablon:**
```markdown
# VantisOS - [Język]

[Tłumaczenie głównego README.md]

## Instalacja
[Instrukcje w danym języku]

## Dokumentacja
[Linki do dokumentacji]
```

#### Zadanie 2.2.2: Utworzenie scripts/install_deps.sh
**Priorytet:** 🟠 WYSOKI  
**Czas:** 2 godziny

```bash
#!/bin/bash
set -e

echo "Installing VantisOS dependencies..."

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build tools
sudo apt-get update
sudo apt-get install -y build-essential

# QEMU for testing
sudo apt-get install -y qemu-system-x86

echo "Dependencies installed successfully!"
```

#### Zadanie 2.2.3: Aktualizacja README do Rzeczywistości
**Priorytet:** 🟠 WYSOKI  
**Czas:** 4 godziny

**Zmiany:**
- Usuń "production-ready" → "early development"
- Zaktualizuj status weryfikacji
- Dodaj sekcję "Known Issues"
- Zaktualizuj instrukcje instalacji

---

## Faza 3: ŚREDNIE PRIORYTETY (Tydzień 5-6)

### 3.1 Poprawa Jakości Kodu (3-4 dni)

#### Zadanie 3.1.1: Rozwiązanie Błędów Clippy
**Priorytet:** 🟡 ŚREDNI  
**Czas:** 2 dni

**Proces:**
```bash
# 1. Generuj raport
cargo clippy --workspace -- -D warnings > clippy_errors.txt

# 2. Kategoryzuj błędy
cat clippy_errors.txt | grep "warning:" | sort | uniq -c

# 3. Napraw kategoria po kategorii
```

#### Zadanie 3.1.2: Usunięcie Ostrzeżeń Kompilatora
**Priorytet:** 🟡 ŚREDNI  
**Czas:** 1 dzień

**Główne kategorie:**
- Unused variables → Dodaj prefix `_`
- Unused imports → Usuń
- Dead code → Usuń lub dodaj `#[allow(dead_code)]`

#### Zadanie 3.1.3: Dodanie Dokumentacji
**Priorytet:** 🟡 ŚREDNI  
**Czas:** 2 dni

**Cel:** 100% funkcji publicznych z dokumentacją

```rust
/// Encrypts data using AES-256-CBC
///
/// # Arguments
/// * `data` - Data to encrypt
/// * `key` - 256-bit encryption key
///
/// # Returns
/// Encrypted data with IV prepended
///
/// # Errors
/// Returns error if data is too large
pub fn encrypt_aes256_cbc(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, &'static str>
```

---

### 3.2 Implementacja Pełnej Detekcji Deadlock (2-3 dni)

#### Zadanie 3.2.1: Graf Zależności Procesów
**Priorytet:** 🟡 ŚREDNI  
**Czas:** 1 dzień

```rust
pub struct DependencyGraph {
    edges: HashMap<ProcessId, Vec<ProcessId>>,
}

impl DependencyGraph {
    pub fn add_dependency(&mut self, waiter: ProcessId, holder: ProcessId) {
        self.edges.entry(waiter).or_insert_with(Vec::new).push(holder);
    }
    
    pub fn remove_dependency(&mut self, waiter: ProcessId, holder: ProcessId) {
        if let Some(deps) = self.edges.get_mut(&waiter) {
            deps.retain(|&id| id != holder);
        }
    }
}
```

#### Zadanie 3.2.2: Algorytm Wykrywania Cykli
**Priorytet:** 🟡 ŚREDNI  
**Czas:** 1 dzień

```rust
impl DependencyGraph {
    pub fn detect_cycle(&self) -> Option<Vec<ProcessId>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for &node in self.edges.keys() {
            if let Some(cycle) = self.dfs_cycle(node, &mut visited, &mut rec_stack) {
                return Some(cycle);
            }
        }
        
        None
    }
    
    fn dfs_cycle(
        &self,
        node: ProcessId,
        visited: &mut HashSet<ProcessId>,
        rec_stack: &mut HashSet<ProcessId>,
    ) -> Option<Vec<ProcessId>> {
        // DFS implementation
    }
}
```

#### Zadanie 3.2.3: Testy Deadlock
**Priorytet:** 🟡 ŚREDNI  
**Czas:** 1 dzień

```rust
#[test]
fn test_deadlock_detection() {
    let mut graph = DependencyGraph::new();
    
    // Create cycle: A -> B -> C -> A
    graph.add_dependency(1, 2);
    graph.add_dependency(2, 3);
    graph.add_dependency(3, 1);
    
    let cycle = graph.detect_cycle();
    assert!(cycle.is_some());
    assert_eq!(cycle.unwrap(), vec![1, 2, 3, 1]);
}
```

---

## Harmonogram Realizacji

### Tydzień 1-2: KRYTYCZNE
```
Dzień 1-2:   Struktura projektu (Cargo workspace)
Dzień 3-4:   CI/CD (wszystkie workflow)
Dzień 5-7:   Bezpieczeństwo (encryption, tokeny)
Dzień 8-10:  Bezpieczeństwo (static mut, testy)
```

### Tydzień 3-4: WYSOKIE
```
Dzień 11-15: Naprawa testów (267 błędów)
Dzień 16-18: Dokumentacja (README, skrypty)
```

### Tydzień 5-6: ŚREDNIE
```
Dzień 19-22: Jakość kodu (clippy, ostrzeżenia)
Dzień 23-25: Deadlock detection
Dzień 26-28: Testy integracyjne
```

---

## Metryki Sukcesu

### Po Fazie 1 (Tydzień 2):
- ✅ `cargo build --workspace` działa
- ✅ Wszystkie workflow CI/CD przechodzą
- ✅ Prawdziwe szyfrowanie zaimplementowane
- ✅ Brak static mut warnings
- ✅ Tokeny są kryptograficznie bezpieczne

### Po Fazie 2 (Tydzień 4):
- ✅ `cargo test --workspace` działa (0 błędów kompilacji)
- ✅ Wszystkie README_*.md istnieją
- ✅ scripts/install_deps.sh działa
- ✅ Dokumentacja jest spójna z kodem

### Po Fazie 3 (Tydzień 6):
- ✅ `cargo clippy -- -D warnings` przechodzi
- ✅ 0 ostrzeżeń kompilatora
- ✅ Pełna detekcja deadlock działa
- ✅ 100% funkcji publicznych ma dokumentację

---

## Zasoby Wymagane

### Zespół:
- 1 Lead Developer (full-time)
- 1 Security Engineer (part-time, Faza 1)
- 1 DevOps Engineer (part-time, Faza 1)
- 1 QA Engineer (part-time, Faza 2-3)

### Narzędzia:
- GitHub Actions (CI/CD)
- Cargo (build system)
- Clippy (linting)
- Rustfmt (formatting)

### Czas:
- **Minimum:** 6 tygodni (1.5 miesiąca)
- **Realistycznie:** 8-10 tygodni (2-2.5 miesiąca)
- **Z buforem:** 12 tygodni (3 miesiące)

---

## Ryzyka i Mitygacje

### Ryzyko 1: Brak zasobów
**Mitygacja:** Priorytetyzacja - zrób najpierw Fazę 1

### Ryzyko 2: Nowe błędy podczas naprawy
**Mitygacja:** Testy regresji po każdej zmianie

### Ryzyko 3: Zależności zewnętrzne
**Mitygacja:** Użyj stabilnych wersji crates

### Ryzyko 4: Scope creep
**Mitygacja:** Trzymaj się planu, nie dodawaj nowych features

---

## Wnioski

Ten plan naprawczy jest **realny i wykonalny** w ciągu 6-12 tygodni przy odpowiednich zasobach.

**Kluczowe zasady:**
1. **Priorytetyzacja** - Najpierw krytyczne, potem reszta
2. **Weryfikacja** - Każda zmiana musi być przetestowana
3. **Dokumentacja** - Każda naprawa musi być udokumentowana
4. **Komunikacja** - Regularne statusy i code review

**Po zakończeniu tego planu, VantisOS będzie:**
- ✅ Kompilowalny jako całość
- ✅ Testowalny (wszystkie testy działają)
- ✅ Bezpieczny (prawdziwe szyfrowanie)
- ✅ Udokumentowany (spójna dokumentacja)
- ✅ Gotowy do dalszego rozwoju

---

**Plan przygotowany przez:** SuperNinja AI Agent  
**Data:** 10 lutego 2025  
**Status:** GOTOWY DO REALIZACJI
</file_path>
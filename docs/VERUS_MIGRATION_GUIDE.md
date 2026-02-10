# 🔄 Przewodnik Migracji Składni Verus - VantisOS IPC

## 📋 Informacje Ogólne

**Data**: 10 lutego 2025  
**Wersja Verus**: 0.2026.02.06.4a2b93e  
**Pliki do Migracji**: 11 plików IPC  
**Szacowany Czas**: 1-2 godziny  
**Status**: 🟡 **W TRAKCIE**

---

## 🎯 Cel Migracji

Migracja kodu IPC ze starej składni Verus (używającej `#[cfg(feature = "verus")]` i `builtin::*`) do nowej składni Verus (używającej makra `verus! { ... }`).

---

## 📊 Różnice w Składni

### 1. Struktura Pliku

#### ❌ Stara Składnia
```rust
#[cfg(feature = "verus")]
use builtin::*;
#[cfg(feature = "verus")]
use builtin_macros::*;
#[cfg(feature = "verus")]
use vstd::prelude::*;

use super::process::Pid;
use std::collections::HashMap;

#[cfg(feature = "verus")]
pub spec fn wf(&self) -> bool {
    self.checksum == compute_checksum_spec(self.data)
}

#[cfg(feature = "verus")]
pub proof fn theorem_message_integrity_preserved()
    ensures(
        forall|msg: IntegrityMessage| 
            msg.wf() ==> msg.verify_integrity()
    )
{
    // Proof
}
```

#### ✅ Nowa Składnia
```rust
use vstd::prelude::*;

use super::process::Pid;
use std::collections::HashMap;

verus! {

pub spec fn wf(&self) -> bool {
    self.checksum == compute_checksum_spec(self.data)
}

pub proof fn theorem_message_integrity_preserved()
    ensures
        forall|msg: IntegrityMessage| 
            msg.wf() ==> msg.verify_integrity()
{
    // Proof
}

} // verus!
```

### 2. Importy

#### ❌ Stara Składnia
```rust
#[cfg(feature = "verus")]
use builtin::*;
#[cfg(feature = "verus")]
use builtin_macros::*;
#[cfg(feature = "verus")]
use vstd::prelude::*;
```

#### ✅ Nowa Składnia
```rust
use vstd::prelude::*;
```

### 3. Requires i Ensures

#### ❌ Stara Składnia
```rust
pub fn new(sender: Pid, receiver: Pid, data: Vec<u8>) -> Result<Self, &'static str>
    requires(data.len() <= MAX_MESSAGE_SIZE)
    ensures(|result: Result<BoundedMessage, &'static str>| match result {
        Ok(msg) => msg.wf() && msg.size() == data.len(),
        Err(_) => true,
    })
{
    // Implementation
}
```

#### ✅ Nowa Składnia
```rust
pub fn new(sender: Pid, receiver: Pid, data: Vec<u8>) -> Result<Self, &'static str>
    requires
        data.len() <= MAX_MESSAGE_SIZE,
    ensures
        |result: Result<BoundedMessage, &'static str>| match result {
            Ok(msg) => msg.wf() && msg.size() == data.len(),
            Err(_) => true,
        }
{
    // Implementation
}
```

### 4. Funkcje Spec i Proof

#### ❌ Stara Składnia
```rust
#[cfg(feature = "verus")]
pub spec fn data(&self) -> Seq<u8> {
    self.data
}

#[cfg(feature = "verus")]
pub proof fn theorem_data_immutability()
    ensures(
        forall|msg1: IntegrityMessage, msg2: IntegrityMessage|
            msg1.wf() && msg2.wf() && 
            msg1.data() == msg2.data() ==>
            msg1.checksum() == msg2.checksum()
    )
{
    // Proof
}
```

#### ✅ Nowa Składnia
```rust
pub spec fn data(&self) -> Seq<u8> {
    self.data
}

pub proof fn theorem_data_immutability()
    ensures
        forall|msg1: IntegrityMessage, msg2: IntegrityMessage|
            msg1.wf() && msg2.wf() && 
            msg1.data() == msg2.data() ==>
            msg1.checksum() == msg2.checksum()
{
    // Proof
}
```

### 5. Atrybuty Verus

#### ❌ Stara Składnia
```rust
#[cfg(feature = "verus")]
#[verifier::external_body]
pub fn compute_checksum(data: &[u8]) -> u32 {
    // Implementation
}
```

#### ✅ Nowa Składnia
```rust
#[verifier::external_body]
pub fn compute_checksum(data: &[u8]) -> u32 {
    // Implementation
}
```

**Uwaga**: Atrybuty `#[verifier::...]` pozostają bez zmian, ale są umieszczone wewnątrz bloku `verus! { ... }`.

---

## 🔧 Skrypt Automatycznej Migracji

### migrate_verus_syntax.py

```python
#!/usr/bin/env python3
"""
Skrypt do automatycznej migracji składni Verus
z starej wersji (builtin::*) do nowej (verus! macro)
"""

import re
import sys
from pathlib import Path

def migrate_file(file_path):
    """Migruje pojedynczy plik do nowej składni Verus"""
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Backup oryginalnego pliku
    backup_path = file_path.with_suffix('.rs.backup')
    with open(backup_path, 'w') as f:
        f.write(content)
    
    # 1. Usuń stare importy Verus
    content = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\nuse builtin::\*;\s*\n',
        '',
        content
    )
    content = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\nuse builtin_macros::\*;\s*\n',
        '',
        content
    )
    content = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\nuse vstd::prelude::\*;\s*\n',
        '',
        content
    )
    
    # 2. Dodaj nowy import na początku (po komentarzach)
    lines = content.split('\n')
    insert_index = 0
    for i, line in enumerate(lines):
        if not line.startswith('//') and line.strip():
            insert_index = i
            break
    
    if 'use vstd::prelude::*;' not in content:
        lines.insert(insert_index, 'use vstd::prelude::*;\n')
    
    content = '\n'.join(lines)
    
    # 3. Usuń #[cfg(feature = "verus")] przed funkcjami spec/proof
    content = re.sub(
        r'#\[cfg\(feature = "verus"\)\]\s*\n(?=pub (spec|proof) fn)',
        '',
        content
    )
    
    # 4. Zmień requires(condition) na requires condition
    content = re.sub(
        r'requires\(([^)]+)\)',
        r'requires \1',
        content
    )
    
    # 5. Zmień ensures(condition) na ensures condition
    # To jest bardziej skomplikowane, bo ensures może mieć zagnieżdżone nawiasy
    # Zostawiamy to do ręcznej edycji lub bardziej zaawansowanego parsera
    
    # 6. Znajdź pierwszą funkcję spec/proof i dodaj verus! {
    verus_start = re.search(r'(pub (spec|proof) fn)', content)
    if verus_start:
        insert_pos = verus_start.start()
        content = content[:insert_pos] + 'verus! {\n\n' + content[insert_pos:]
    
    # 7. Dodaj } // verus! na końcu pliku
    content = content.rstrip() + '\n\n} // verus!\n'
    
    # Zapisz zmigrowany plik
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"✅ Zmigrowano: {file_path}")
    print(f"   Backup: {backup_path}")

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 migrate_verus_syntax.py <file1.rs> [file2.rs ...]")
        sys.exit(1)
    
    for file_path in sys.argv[1:]:
        path = Path(file_path)
        if not path.exists():
            print(f"❌ Plik nie istnieje: {file_path}")
            continue
        
        try:
            migrate_file(path)
        except Exception as e:
            print(f"❌ Błąd podczas migracji {file_path}: {e}")

if __name__ == '__main__':
    main()
```

---

## 📝 Proces Migracji Krok po Kroku

### Krok 1: Przygotowanie

```bash
cd /workspace/VantisOS/src/verified

# Utwórz katalog na backupy
mkdir -p backup_ipc

# Skopiuj wszystkie pliki IPC do backupu
cp ipc*.rs backup_ipc/

# Stwórz skrypt migracji
cat > migrate_verus_syntax.py << 'EOF'
[zawartość skryptu powyżej]
EOF

chmod +x migrate_verus_syntax.py
```

### Krok 2: Migracja Plików (Kolejność)

**Zalecana kolejność** (od najprostszego do najbardziej złożonego):

1. `ipc_message_integrity.rs` (616 linii, 31 requires/ensures)
2. `ipc_capability_correctness.rs` (710 linii, 6 requires/ensures)
3. `ipc_deadlock_freedom.rs` (682 linie, 4 requires/ensures)
4. `ipc_information_leakage.rs` (780 linii, 30 requires/ensures)
5. `ipc_resource_bounds.rs` (689 linii, 58 requires/ensures)
6. `ipc_verified.rs` (829 linii, 23 requires/ensures)
7. `ipc_integrated.rs` (742 linie, 10 requires/ensures)
8. `ipc_complete.rs` (722 linie)
9. `ipc_inline.rs` (734 linie)
10. `ipc_complete_tests.rs` (544 linie)
11. `ipc.rs` (745 linii) - ostatni, bo bazowy

**Migracja pojedynczego pliku**:

```bash
# Migracja
python3 migrate_verus_syntax.py ipc_message_integrity.rs

# Weryfikacja składni
/workspace/verus-x86-linux/verus --no-verify ipc_message_integrity.rs

# Jeśli OK, commit
git add ipc_message_integrity.rs
git commit -m "feat: migrate ipc_message_integrity.rs to new Verus syntax"
```

### Krok 3: Ręczna Korekta

Po automatycznej migracji, sprawdź i popraw ręcznie:

1. **Ensures z zagnieżdżonymi nawiasami**:
   ```rust
   // Może wymagać ręcznej korekty
   ensures(|result| match result { ... })
   // na:
   ensures |result| match result { ... }
   ```

2. **Requires z wieloma warunkami**:
   ```rust
   requires([cond1, cond2, cond3])
   // na:
   requires
       cond1,
       cond2,
       cond3,
   ```

3. **Funkcje exec wewnątrz verus!**:
   - Sprawdź, czy wszystkie funkcje exec są poprawnie umieszczone

### Krok 4: Weryfikacja

```bash
# Weryfikacja składni (bez weryfikacji formalnej)
/workspace/verus-x86-linux/verus --no-verify ipc_message_integrity.rs

# Kompilacja z Cargo
cd /workspace/VantisOS/src/verified
cargo build --features verus

# Testy
cargo test --features verus
```

### Krok 5: Commit i Push

```bash
# Po migracji wszystkich plików
git add ipc*.rs
git commit -m "feat: migrate all IPC files to new Verus syntax (verus! macro)"
git push origin fix/test-compilation-errors
```

---

## ⚠️ Typowe Problemy i Rozwiązania

### Problem 1: Błąd "unexpected token"

**Objaw**:
```
error: unexpected token: `(`
  --> ipc_message_integrity.rs:150:14
   |
150 |     ensures(|result| ...)
   |              ^
```

**Rozwiązanie**:
```rust
// Zmień:
ensures(|result| ...)

// Na:
ensures |result| ...
```

### Problem 2: Błąd "requires expects a boolean"

**Objaw**:
```
error: requires expects a boolean expression
  --> ipc_resource_bounds.rs:68:14
   |
68  |     requires([cond1, cond2])
   |              ^^^^^^^^^^^^^^
```

**Rozwiązanie**:
```rust
// Zmień:
requires([cond1, cond2])

// Na:
requires
    cond1,
    cond2,
```

### Problem 3: Funkcje poza blokiem verus!

**Objaw**:
```
error: spec function must be inside verus! macro
```

**Rozwiązanie**:
- Upewnij się, że wszystkie funkcje `spec`, `proof` i `exec` są wewnątrz bloku `verus! { ... }`
- Funkcje pomocnicze (bez Verus) mogą być poza blokiem

### Problem 4: Importy wewnątrz verus!

**Objaw**:
```
error: use statements not allowed inside verus! macro
```

**Rozwiązanie**:
- Wszystkie importy (`use`) muszą być PRZED blokiem `verus! { ... }`
- Tylko kod Verus (spec, proof, exec) wewnątrz bloku

---

## ✅ Checklist Migracji

### Przed Migracją
- [ ] Backup wszystkich plików IPC
- [ ] Verus zainstalowany i działający
- [ ] Skrypt migracji gotowy
- [ ] Git branch utworzony

### Podczas Migracji
- [ ] Migracja pliku 1: ipc_message_integrity.rs
- [ ] Migracja pliku 2: ipc_capability_correctness.rs
- [ ] Migracja pliku 3: ipc_deadlock_freedom.rs
- [ ] Migracja pliku 4: ipc_information_leakage.rs
- [ ] Migracja pliku 5: ipc_resource_bounds.rs
- [ ] Migracja pliku 6: ipc_verified.rs
- [ ] Migracja pliku 7: ipc_integrated.rs
- [ ] Migracja pliku 8: ipc_complete.rs
- [ ] Migracja pliku 9: ipc_inline.rs
- [ ] Migracja pliku 10: ipc_complete_tests.rs
- [ ] Migracja pliku 11: ipc.rs

### Po Migracji
- [ ] Weryfikacja składni wszystkich plików
- [ ] Kompilacja z Cargo
- [ ] Testy jednostkowe przechodzą
- [ ] Commit i push do GitHub
- [ ] Dokumentacja zaktualizowana

---

## 📊 Metryki Migracji

| Metryka | Wartość |
|---------|---------|
| Pliki do migracji | 11 |
| Łączna liczba linii | 7,793 |
| Funkcje spec | 66 |
| Funkcje proof | 66 |
| Requires/Ensures | 162 |
| Szacowany czas | 1-2 godz |
| Pliki zmigrowane | 0/11 |
| Status | 🟡 W trakcie |

---

## 🚀 Następne Kroki Po Migracji

### 1. Weryfikacja Formalna (4 tygodnie)

**Tydzień 1**: Message Integrity + Capability Correctness
```bash
/workspace/verus-x86-linux/verus ipc_message_integrity.rs
/workspace/verus-x86-linux/verus ipc_capability_correctness.rs
```

**Tydzień 2**: Deadlock Freedom + Resource Bounds
```bash
/workspace/verus-x86-linux/verus ipc_deadlock_freedom.rs
/workspace/verus-x86-linux/verus ipc_resource_bounds.rs
```

**Tydzień 3**: Information Leakage
```bash
/workspace/verus-x86-linux/verus ipc_information_leakage.rs
```

**Tydzień 4**: Integracja i testy
```bash
/workspace/verus-x86-linux/verus ipc_verified.rs
/workspace/verus-x86-linux/verus ipc_integrated.rs
```

### 2. Dokumentacja

- [ ] Aktualizacja README.md
- [ ] Dokumentacja API
- [ ] Przykłady użycia
- [ ] Przewodnik weryfikacji

### 3. Testy

- [ ] Testy jednostkowe
- [ ] Testy integracyjne
- [ ] Benchmarki wydajności
- [ ] Testy bezpieczeństwa

---

## 📞 Zasoby

### Dokumentacja Verus
- **Oficjalna dokumentacja**: https://verus-lang.github.io/verus/
- **GitHub**: https://github.com/verus-lang/verus
- **Przykłady**: https://github.com/verus-lang/verus/tree/main/source/rust_verify_test/tests

### Narzędzia
- **Verus Binary**: `/workspace/verus-x86-linux/verus`
- **Wersja**: 0.2026.02.06.4a2b93e
- **Z3 SMT Solver**: Included

### Wsparcie
- **GitHub Issues**: https://github.com/verus-lang/verus/issues
- **Zulip Chat**: https://verus-lang.zulipchat.com/

---

## 💡 Wskazówki

1. **Testuj Często**: Weryfikuj każdy plik po migracji
2. **Małe Commity**: Commituj po każdym zmigrowanym pliku
3. **Czytaj Błędy**: Verus daje bardzo szczegółowe komunikaty błędów
4. **Używaj --no-verify**: Do sprawdzania składni bez pełnej weryfikacji
5. **Backup**: Zawsze rób backup przed migracją

---

**Status**: 📝 **PRZEWODNIK GOTOWY**  
**Następna Akcja**: Rozpoczęcie migracji plików  
**Priorytet**: 🔴 **KRYTYCZNY**

---

*Dokument stworzony: 10 lutego 2025*  
*Autor: SuperNinja AI Agent*  
*Wersja: 1.0*
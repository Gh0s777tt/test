# Plan Porządkowania Repozytorium VantisOS
## Analiza i Strategia Czyszczenia

## 1. ANALIZA OBECNEGO STANU

### Pliki w Katalogu Głównym (Root)
Obecnie w katalogu głównym znajduje się **90+ plików dokumentacji**, co znacznie utrudnia nawigację.

### Kategorie Plików do Uporządkowania:

#### A. Dokumenty Sesyjne/Tymczasowe (DO ARCHIWIZACJI)
```
WEEK_*.md (10 plików)
SESSION_*.md (2 pliki)
DAILY_SUMMARY_*.md (1 plik)
DAY_5_PATH_CACHING.md
COMPLETE_FINAL_SUMMARY_FEB_11_2025.md
FINAL_STATUS_REPORT*.md (2 pliki)
GITHUB_ACTIONS_COMPLETE_FEB_11_2025.md
```
**Akcja:** Przenieść do `docs/archive/sessions/`

#### B. Raporty Analityczne (DO UPORZĄDKOWANIA)
```
COMPREHENSIVE_REPOSITORY_ANALYSIS_FEB_11_2025.md
DETAILED_FUNCTION_ANALYSIS_FEB_11_2025.md
BRANCH_ANALYSIS_REPORT.md
FINAL_ANALYSIS_REPORT.md
PERFORMANCE_BASELINE_RESULTS.md
```
**Akcja:** Przenieść do `docs/reports/`

#### C. Plany i Strategie (DO UPORZĄDKOWANIA)
```
FULL_BUILD_PLAN.md
OPTION_2_ALPINE_ANALYSIS.md
REALISTIC_BUILD_OPTIONS.md
BUILD_OPTIONS_SUMMARY.md
IMMEDIATE_ACTION_PLAN.md
QUICK_BUILD_ISO_GUIDE.md
PLAN_NAPRAWCZY.md
DETAILED_COMPLETION_PLAN_PL.md
```
**Akcja:** Przenieść do `docs/plans/`

#### D. Dokumenty Polskojęzyczne (DO UPORZĄDKOWANIA)
```
ANALIZA_WERYFIKACJA.md
COMPREHENSIVE_ANALYSIS_PL.md
DETAILED_COMPLETION_PLAN_PL.md
EXECUTIVE_SUMMARY_PL.md
NOWA_ANALIZA_2025_02_10.md
PLAN_NAPRAWCZY.md
PODSUMOWANIE_PL.md
PODSUMOWANIE_WIELOBRANCH_PL.md
PROJECT_VISUAL_MAP_PL.md
STATUS_ISO_INSTALACJI_PL.md
SZCZEGOLOWA_ANALIZA_I_PLAN.md
```
**Akcja:** Przenieść do `docs/polish/`

#### E. Dokumenty Rekrutacyjne
```
RECRUITMENT_POSTING_GUIDE.md
RECRUITMENT_TRACKING.md
QUICK_RECRUITMENT_POSTS.md
```
**Akcja:** Przenieść do `docs/recruitment/`

#### F. Dokumenty Weryfikacji
```
VERIFICATION_STATUS.md
IPC_VERIFICATION_SESSION_*.md (3 pliki)
IPC_INTEGRATION_SESSION.md
CICD_VERUS_SETUP_COMPLETE.md
```
**Akcja:** Przenieść do `docs/verification/`

#### G. Duplikaty i Przestarzałe (DO USUNIĘCIA)
```
C (plik bez rozszerzenia - prawdopodobnie śmieć)
SUMMARY.MD (duplikat?)
README_CLEANUP_PROJECT.md (przestarzały)
README_UPDATE_COMPLETE_FEB_11_2025.md (przestarzały)
README_VERIFICATION_SECTION.md (zintegrowany z README)
CLEANUP_PROJECT_INDEX.md (przestarzały)
CLEANUP_SUMMARY.md (przestarzały)
STRUCTURE_PROPAGATION_REPORT*.md (przestarzałe)
PUSH_INSTRUCTIONS.md (przestarzały)
BUILD_REPORT.md (przestarzały - mamy nowsze)
MULTI_BRANCH_CLEANUP_SUMMARY.md (przestarzały)
PR_28_MERGE_SUMMARY.md (przestarzały)
```
**Akcja:** Usunąć

#### H. Pliki Konfiguracyjne (POZOSTAWIĆ)
```
.EditorConfig
.gitignore
.gitattributes
.gitmodules
.pre-commit-config.yaml
.releaserc.json
.travis.yml
.all-contributorsrc
Cargo.toml.verification
Makefile
Makefile.verification
book.toml
cliff.toml
deny.toml
filesystem.toml
flake.nix
gitlab-ci.yml
gitpod.yml
initfs.toml
initfs_live.toml
rust-toolchain
sonar-project.properties
bochs.x86_64
```
**Akcja:** Pozostawić w root

#### I. Dokumenty Główne (POZOSTAWIĆ W ROOT)
```
README.md
CHANGELOG.md
CONTRIBUTING.md
LICENSE
SECURITY.MD
CITATION.cff
todo.md
ROADMAP_2026_2027.md
```
**Akcja:** Pozostawić w root

#### J. Skrypty (POZOSTAWIĆ)
```
bootstrap.sh
deploy_production_crypto.sh
test_direct_metal.sh
```
**Akcja:** Pozostawić w root

#### K. Najnowsze Raporty (POZOSTAWIĆ W ROOT)
```
ALL_FIXES_COMPLETE_FINAL.md (najnowszy raport)
FIXES_COMPLETE_SUMMARY.md (najnowszy raport)
PROBLEM_ANALYSIS_AND_FIXES.md (najnowszy raport)
DEVELOPMENT_WORKFLOW.md (aktywny dokument)
DOCUMENTATION_INDEX.md (aktywny indeks)
```
**Akcja:** Pozostawić w root (są aktualne i ważne)

## 2. STRUKTURA DOCELOWA

```
VantisOS/
├── README.md                          # Główny README
├── CHANGELOG.md                       # Historia zmian
├── CONTRIBUTING.md                    # Wytyczne dla kontrybutorów
├── LICENSE                            # Licencja
├── SECURITY.MD                        # Polityka bezpieczeństwa
├── CITATION.cff                       # Cytowanie projektu
├── todo.md                            # Aktualne zadania
├── ROADMAP_2026_2027.md              # Roadmapa projektu
├── ALL_FIXES_COMPLETE_FINAL.md       # Najnowszy raport
├── FIXES_COMPLETE_SUMMARY.md         # Podsumowanie napraw
├── PROBLEM_ANALYSIS_AND_FIXES.md     # Analiza problemów
├── DEVELOPMENT_WORKFLOW.md           # Workflow deweloperski
├── DOCUMENTATION_INDEX.md            # Indeks dokumentacji
│
├── docs/                              # Dokumentacja
│   ├── archive/                       # Archiwum
│   │   └── sessions/                  # Sesje robocze
│   ├── reports/                       # Raporty analityczne
│   ├── plans/                         # Plany i strategie
│   ├── polish/                        # Dokumenty PL
│   ├── recruitment/                   # Rekrutacja
│   └── verification/                  # Weryfikacja formalna
│
├── src/                               # Kod źródłowy
├── scripts/                           # Skrypty
├── .github/                           # GitHub workflows
└── [inne katalogi projektu]
```

## 3. AKTUALIZACJA .gitignore

Dodać do .gitignore:
```
# Temporary files
*.tmp
*.bak
*~
*.swp
*.swo

# Build artifacts
target/
debug/
release/
*.rlib
*.so
*.dylib
*.dll

# IDE
.idea/
.vscode/
*.iml
.project
.classpath
.settings/

# OS
.DS_Store
Thumbs.db
desktop.ini

# Logs
*.log
logs/

# Documentation build
docs/_build/
docs/.doctrees/

# Test outputs
test_output/
benchmark_results/
```

## 4. PLAN WYKONANIA

### Krok 1: Utworzenie Struktury Katalogów
```bash
mkdir -p docs/archive/sessions
mkdir -p docs/reports
mkdir -p docs/plans
mkdir -p docs/polish
mkdir -p docs/recruitment
mkdir -p docs/verification
```

### Krok 2: Przeniesienie Plików
- Przenieść pliki sesyjne do `docs/archive/sessions/`
- Przenieść raporty do `docs/reports/`
- Przenieść plany do `docs/plans/`
- Przenieść dokumenty PL do `docs/polish/`
- Przenieść dokumenty rekrutacyjne do `docs/recruitment/`
- Przenieść dokumenty weryfikacji do `docs/verification/`

### Krok 3: Usunięcie Przestarzałych Plików
- Usunąć duplikaty i przestarzałe dokumenty

### Krok 4: Aktualizacja .gitignore
- Dodać nowe reguły ignorowania

### Krok 5: Aktualizacja DOCUMENTATION_INDEX.md
- Zaktualizować ścieżki do dokumentów

### Krok 6: Commit i Push
- Commit zmian z opisem "refactor: reorganize repository structure"
- Push do GitHub

## 5. OCZEKIWANE REZULTATY

### Przed:
- 90+ plików w katalogu głównym
- Trudna nawigacja
- Brak jasnej struktury

### Po:
- ~15 plików w katalogu głównym (tylko najważniejsze)
- Czysta, logiczna struktura
- Łatwa nawigacja
- Profesjonalny wygląd repozytorium

## 6. BEZPIECZEŃSTWO

Przed usunięciem jakichkolwiek plików:
1. Sprawdzić czy nie są używane w CI/CD
2. Sprawdzić czy nie są linkowane w README
3. Utworzyć backup (git commit przed zmianami)
4. Możliwość rollback w razie problemów

## 7. NASTĘPNE KROKI

Po uporządkowaniu:
1. Zaktualizować README z nową strukturą
2. Zaktualizować CONTRIBUTING.md z wytycznymi
3. Dodać docs/README.md z opisem struktury dokumentacji
4. Rozważyć użycie mdBook lub Sphinx dla dokumentacji
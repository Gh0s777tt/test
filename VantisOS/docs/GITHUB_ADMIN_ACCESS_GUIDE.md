# Przewodnik: Jak nadać dostęp administratora do VantisOS

## Problem Obecny

GitHub Actions nie działają w repozytorium `vantisCorp/VantisOS` ponieważ:
- Obecny token jest app token (nie ma pełnych uprawnień administratora)
- Brak możliwości włączenia GitHub Actions na poziomie organizacji
- Wszystkie workflow kończą się niepowodzeniem przed wykonaniem

## Rozwiązania

### 🎯 Rozwiązanie 1: Stwórz Personal Access Token (PAT) - ZALECANE

#### Krok 1: Zaloguj się jako administrator organizacji

1. Otwórz https://github.com i zaloguj się na konto z uprawnieniami **owner** organizacji `vantisCorp`
2. Upewnij się, że jesteś członkiem organizacji z rolą "Owner"

#### Krok 2: Wygeneruj Personal Access Token

1. Kliknij na swoją ikonę profilu → **Settings**
2. W lewym panelu: **Developer settings** → **Personal access tokens** → **Tokens (classic)**
3. Kliknij **Generate new token** → **Generate new token (classic)**

#### Krok 3: Skonfiguruj token

**Note:** `VantisOS Admin Token - February 2025`

**Expiration:** Wybierz `90 days` (lub `No expiration` jeśli planujesz długoterminowe używanie)

**Select scopes (wszystkie potrzebne):**
```
✅ repo                    # Pełny dostęp do repozytoriów
✅ repo:status             # Status commitów
✅ repo_deployment         # Deployment status
✅ public_repo             # Dostęp do publicznych repozytoriów
✅ admin:org               # Zarządzanie organizacją
✅ admin:org_hook          # Hooki organizacji
✅ admin:public_key       # Klucze publiczne
✅ admin:repo_hook         # Repozytorium hooki
✅ delete_repo             # Usuwanie repozytoriów
✅ workflow                # GitHub Actions workflow
✅ write:packages          # Pisanie do paczek
✅ read:packages           # Czytanie paczek
✅ delete:packages         # Usuwanie paczek
✅ user                    # Informacje o użytkowniku
✅ user:email              # Email użytkownika
✅ user:follow             # Obserwowanie użytkowników
✅ read:gpg_key            # Klucze GPG
✅ read:org                # Czytanie organizacji
✅ read:public_key         # Klucze publiczne
```

4. Kliknij **Generate token**
5. **SKOPIUJ TOKEN** (będzie widoczny tylko raz!)

#### Krok 4: Zaktualizuj token w środowisku

W terminalu lub środowisku developmentowym:

```bash
# Jeśli używasz środowiska Linux/Mac
export GITHUB_TOKEN="twoj_nowy_tutaj_wklej"

# Jeśli używasz Windows PowerShell
$env:GITHUB_TOKEN="twoj_nowy_tutaj_wklej"

# Sprawdź czy działa
gh auth status
```

#### Krok 5: Weryfikacja uprawnień

```bash
# Sprawdź czy masz uprawnienia
gh repo view vantisCorp/VantisOS --json viewerPermission

# Powinno pokazać: "viewerPermission": "admin" lub "maintain"

# Sprawdź dostęp do organizacji
gh api orgs/vantisCorp --jq '.login, .description'

# Sprawdź ustawienia Actions
gh api repos/vantisCorp/VantisOS/actions/permissions/workflow
```

---

### 🔄 Rozwiązanie 2: Dodaj użytkownika jako Owner Organizacji

#### Jeśli jesteś już członkiem organizacji:

1. **Obecny Owner organizacji powinien:**
   - Przejdź do: https://github.com/organizations/vantisCorp
   - Kliknij na **People** w górnym menu
   - Kliknij **Add member**
   - Wpisz nazwę użytkownika GitHub
   - Wybierz rolę: **Owner**
   - Kliknij **Add member**

#### Jeśli nie jesteś członkiem:

1. Poproś o zaproszenie do organizacji
2. Akceptuj zaproszenie przez email lub link
3. Zażądaj roli "Owner"

---

### 🏢 Rozwiązanie 3: Włącz GitHub Actions na poziomie organizacji

#### Krok 1: Przejdź do ustawień organizacji

Jako administrator organizacji:

1. Przejdź do: https://github.com/organizations/vantisCorp/settings
2. W lewym panelu kliknij **Actions** → **General**

#### Krok 2: Skonfiguruj Actions

**Actions permissions:**
- Wybierz: **Allow all actions and reusable workflows**

**Workflow permissions:**
- Wybierz: **Read and write permissions**

**Fork pull request workflows from outside collaborators:**
- Wybierz: **Allow**
- zaznacz: **Require approval for all outside collaborators**

**Allow GitHub Actions to create and approve pull requests:**
- zaznacz: **Allow GitHub Actions to create and approve pull requests**

3. Kliknij **Save**

---

### 💼 Rozwiązanie 4: Sprawdź i aktualizuj plan GitHub

#### Dlaczego to ważne?

GitHub Actions w prywatnych repozytoriach wymagają:
- GitHub Pro (dla pojedynczych użytkowników)
- GitHub Team (dla małych zespołów)
- GitHub Enterprise (dla dużych organizacji)

#### Jak sprawdzić plan:

1. Zaloguj się jako owner organizacji
2. Przejdź do: https://github.com/organizations/vantisCorp/settings/billing
3. Sprawdź czy aktywny plan to **GitHub Team** lub **GitHub Enterprise**

#### Jeśli plan to GitHub Free:

1. Kliknij **Upgrade**
2. Wybierz **GitHub Team** ($4/user/miesiąc)
3. Skonfiguruj płatność
4. Po aktywacji GitHub Actions powinny działać

---

### 📋 Rozwiązanie 5: Przenieś repozytorium do konta osobistego

**Tylko jeśli:**
- Jesteś jedynym użytkownikiem
- Nie potrzebujesz organizacji
- Chcesz pełnej kontroli

#### Kroki:

1. Przejdź do repozytorium: https://github.com/vantisCorp/VantisOS
2. Kliknij **Settings**
3. Przewiń do sekcji **Danger Zone**
4. Kliknij **Transfer repository**
5. Wybierz swoje konto osobiste jako docelowe
6. Potwierdź transfer wpisując nazwę repozytorium

---

## Jak zweryfikować czy wszystko działa

Po zastosowaniu jednego z rozwiązań:

### Test 1: Sprawdź uprawnienia
```bash
gh repo view vantisCorp/VantisOS --json viewerPermission
# Powinno pokazać: "admin" lub "maintain"
```

### Test 2: Sprawdź Actions
```bash
gh api repos/vantisCorp/VantisOS/actions/permissions/workflow
# Powinno pokazać uprawnienia bez błędów
```

### Test 3: Uruchom testowy workflow
```bash
cd VantisOS
gh workflow run test-simple.yml
sleep 15
gh run list --workflow=test-simple.yml --limit 1
# Powinno pokazać "completed" z "success"
```

### Test 4: Sprawdź szczegóły workflow
```bash
gh run view LATEST_RUN_ID --log
# Powinno pokazać szczegółowe logi z wykonania
```

---

## Problemy i Rozwiązania

### ❌ Problem: "Repository not found"

**Rozwiązanie:**
- Sprawdź czy token ma scope `repo`
- Zaloguj się ponownie: `gh auth login --with-token <<< "$GITHUB_TOKEN"`

### ❌ Problem: "Resource not accessible by integration"

**Rozwiązanie:**
- Użyj Personal Access Token zamiast app token
- Dodaj scope `admin:org` do tokena

### ❌ Problem: "Upgrade to GitHub Pro or make this repository public"

**Rozwiązanie:**
- Zaktualizuj plan GitHub na Team/Enterprise
- Lub zmień repozytorium na publiczne (jeśli to bezpieczne)

### ❌ Problem: "Actions are disabled"

**Rozwiązanie:**
- Przejdź do ustawień organizacji → Actions
- Włącz Actions
- Sprawdź ustawienia billing

---

## Najlepsza praktyka

**Zalecana ścieżka:**

1. ✅ Stwórz Personal Access Token (PAT) z pełnymi uprawnieniami
2. ✅ Zaktualizuj token w środowisku
3. ✅ Sprawdź czy masz uprawnienia administratora
4. ✅ Włącz GitHub Actions w ustawieniach organizacji
5. ✅ Zaktualizuj plan GitHub na Team (jeśli to darmowe)
6. ✅ Przetestuj workflow
7. ✅ Ciesz się działającym CI/CD!

---

## Pomoc Techniczna

Jeśli żadne z powyższych rozwiązań nie działa:

1. Sprawdź dokumentację GitHub Actions: https://docs.github.com/en/actions
2. Skontaktuj z supportem GitHub: https://support.github.com
3. Sprawdź status GitHub: https://www.githubstatus.com

---

## Podsumowanie

**Kluczowe kroki:**
1. Stwórz PAT z pełnymi uprawnieniami `repo`, `admin:org`, `workflow`
2. Zaloguj się z tym tokenem
3. Włącz Actions w ustawieniach organizacji
4. Zaktualizuj plan GitHub na Team/Enterprise
5. Przetestuj workflow

**Po tych krokach GitHub Actions powinny działać prawidłowo!**
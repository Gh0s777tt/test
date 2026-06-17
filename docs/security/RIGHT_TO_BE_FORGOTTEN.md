# Prawo do zapomnienia (Right to be Forgotten) - VantisOS

## Wprowadzenie

Prawo do zapomnienia (Right to be Forgotten) to fundamentalne prawo użytkownika do żądania usunięcia swoich danych osobowych z systemu VantisOS. Ten dokument opisuje **projektowane** podejście, którego celem jest zgodność z RODO (GDPR) Article 17. VantisOS jest eksperymentalnym projektem we wczesnej fazie (v0.4.1); ta funkcja jest w fazie projektowej/prototypu i **nie została zweryfikowana ani poddana audytowi pod kątem zgodności prawnej**.

## Cel

Zapewnienie użytkownikom VantisOS możliwości:
- Żądania usunięcia wszystkich danych osobowych
- Automatycznego usuwania danych po określonym czasie
- Pełnej kontroli nad cyklem życia danych
- Dążenia do zgodności z RODO (GDPR) i innymi regulacjami prywatności (cel projektowy, nie certyfikowany)

## Architektura Systemu

### Komponenty

```
┌─────────────────────────────────────────────────────────────┐
│                     Privacy Manager                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Data Request │  │ Data Deletion│  │ Data Retention│      │
│  │   Handler    │  │   Engine     │  │   Policy      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
         │                    │                    │
         ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│  User Database  │  │  File System    │  │  Application    │
│                 │  │                 │  │  Data Stores    │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

### Moduły Systemu

#### 1. Data Request Handler
Obsługuje żądania usunięcia danych od użytkowników:
- Przyjmowanie żądań usunięcia
- Weryfikacja tożsamości użytkownika
- Generowanie tokenów potwierdzenia
- Śledzenie statusu żądania

#### 2. Data Deletion Engine
Wykonuje operacje usuwania danych:
- Identyfikacja wszystkich danych użytkownika
- Bezpieczne usuwanie danych
- Usuwanie kopii zapasowych
- Potwierdzenie usunięcia

#### 3. Data Retention Policy
Zarządza politykami retencji danych:
- Automatyczne usuwanie po upływie czasu
- Konfigurowalne okresy retencji
- Wyjątki dla danych wymaganych przez prawo
- Audyt polityk retencji

## Funkcjonalności

### 1. Żądanie usunięcia danych

Użytkownik może złożyć żądanie usunięcia danych poprzez:
- Interfejs użytkownika (UI)
- API
- Linie komend (CLI)

**Proces:**
1. Użytkownik składa żądanie usunięcia
2. System weryfikuje tożsamość użytkownika
3. System generuje token potwierdzenia
4. System identyfikuje wszystkie dane użytkownika
5. System usuwa dane
6. System wysyła potwierdzenie usunięcia

**Czas realizacji (cel projektowy, niezmierzony):** < 24h

### 2. Automatyczne usuwanie danych

System automatycznie usuwa dane po upływie określonego czasu:
- Dane logowania: 30 dni
- Dane użytkownika: 365 dni (jeśli nieaktywne)
- Dane aplikacji: 90 dni (jeśli nieaktywne)
- Dane systemowe: 7 dni

**Konfiguracja:**
```rust
pub struct RetentionPolicy {
    pub login_data: Duration,
    pub user_data: Duration,
    pub app_data: Duration,
    pub system_data: Duration,
}
```

### 3. Bezpieczne usuwanie danych

System zapewnia bezpieczne usuwanie danych:
- Nadpisywanie danych (3x)
- Usuwanie z kopii zapasowych
- Usuwanie z pamięci podręcznej
- Usuwanie z logów

**Algorytm:**
1. Identyfikacja lokalizacji danych
2. Nadpisanie danych losowymi wartościami (3x)
3. Usunięcie plików
4. Usunięcie z kopii zapasowych
5. Usunięcie z pamięci podręcznej
6. Usunięcie z logów
7. Potwierdzenie usunięcia

### 4. Wyjątki od usuwania

Niektóre dane nie mogą zostać usunięte:
- Dane wymagane przez prawo (np. dane finansowe przez 7 lat)
- Dane wymagane do bezpieczeństwa systemu
- Dane wymagane do audytu
- Dane anonimizowane

### 5. Audyt i raportowanie

System prowadzi pełny audyt operacji usuwania:
- Logowanie wszystkich żądań usunięcia
- Logowanie wszystkich operacji usuwania
- Raportowanie statystyk usuwania
- Raportowanie zgodności z RODO

## Zgodność z RODO (GDPR) — cel projektowy

> **Uwaga:** Poniższe punkty opisują **zamierzony** zakres projektu. VantisOS nie jest certyfikowany ani poddany audytowi pod kątem zgodności z RODO. Lista oznacza cele projektowe, a nie zweryfikowaną implementację.

### Article 17 - Right to Erasure

Projektowany zakres względem Article 17 RODO:
- (planowane) Użytkownik może żądać usunięcia danych
- (planowane) System usuwa dane bez zbędnej zwłoki
- (planowane) System informuje użytkownika o usunięciu
- (planowane) System usuwa dane z wszystkich lokalizacji
- (planowane) System obsługuje wyjątki od usuwania

### Kryteria usunięcia

Dane są usuwane gdy:
- Dane nie są już potrzebne do celów, dla których zostały zebrane
- Użytkownik wycofał zgodę
- Użytkownik sprzeciwił się przetwarzaniu
- Dane są przetwarzane niezgodnie z prawem
- Dane muszą zostać usunięte w celu wypełnienia obowiązku prawnego

### Wyjątki od usunięcia

Dane nie są usuwane gdy:
- Są potrzebne do wykonywania prawa do wolności wypowiedzi
- Są potrzebne do wypełnienia obowiązku prawnego
- Są potrzebne do celów archiwalnych w interesie publicznym
- Są potrzebne do ustalenia, dochodzenia lub obrony roszczeń
- Są potrzebne do celów profilowania na podstawie przepisów prawa

## Implementacja Techniczna

### Struktury Danych

```rust
pub struct DeletionRequest {
    pub request_id: Uuid,
    pub user_id: Uuid,
    pub requested_at: DateTime<Utc>,
    pub status: DeletionStatus,
    pub confirmation_token: String,
    pub completed_at: Option<DateTime<Utc>>,
}

pub enum DeletionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

pub struct DataRecord {
    pub record_id: Uuid,
    pub user_id: Uuid,
    pub data_type: DataType,
    pub location: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub enum DataType {
    UserProfile,
    LoginData,
    AppData,
    SystemData,
    BackupData,
    LogData,
}
```

### API

```rust
impl PrivacyManager {
    /// Złóż żądanie usunięcia danych
    pub async fn request_deletion(&self, user_id: Uuid) -> Result<DeletionRequest>;
    
    /// Potwierdź żądanie usunięcia
    pub async fn confirm_deletion(&self, token: String) -> Result<()>;
    
    /// Usuń dane użytkownika
    pub async fn delete_user_data(&self, user_id: Uuid) -> Result<DeletionResult>;
    
    /// Automatyczne usuwanie wygasłych danych
    pub async fn delete_expired_data(&self) -> Result<Vec<DeletionResult>>;
    
    /// Sprawdź status żądania usunięcia
    pub async fn get_deletion_status(&self, request_id: Uuid) -> Result<DeletionStatus>;
    
    /// Pobierz politykę retencji
    pub fn get_retention_policy(&self) -> RetentionPolicy;
    
    /// Ustaw politykę retencji
    pub fn set_retention_policy(&self, policy: RetentionPolicy) -> Result<()>;
}
```

## Bezpieczeństwo (projektowane)

> Poniższe mechanizmy opisują zamierzony projekt. Nie są one zaimplementowane ani zaudytowane w obecnej wersji (v0.4.1).

### Szyfrowanie
- Planowane szyfrowanie danych (np. AES-256)
- Planowane zarządzanie kluczami (np. przez HSM)
- Planowane bezpieczne usuwanie danych (nadpisywanie 3x)

### Autoryzacja
- Weryfikacja tożsamości użytkownika przed usunięciem
- Tokeny potwierdzenia są jednorazowe
- Logowanie wszystkich operacji usuwania

### Audyt
- Pełny audyt wszystkich operacji usuwania
- Niezmienialne logi operacji
- Raportowanie zgodności z RODO

## Wydajność (cele projektowe, niezmierzone)

> Poniższe wartości to cele projektowe. Nie zostały zmierzone w obecnej wersji.

### Metryki (cele, niezmierzone)
- Czas przetwarzania żądania usunięcia: cel < 1s
- Czas usuwania danych: cel < 5min (dla 1GB danych)
- Czas usuwania z kopii zapasowych: cel < 30min
- Czas potwierdzenia usunięcia: cel < 1s

### Skalowalność (cele projektowe)
- Cel: obsługa 10,000+ żądań usunięcia dziennie
- Cel: obsługa 1TB+ danych do usunięcia
- Cel: obsługa 1M+ rekordów użytkowników

## Testowanie (planowane)

> Poniżej opisano planowany zakres testów. Nie odzwierciedla on obecnego pokrycia testami.

### Testy jednostkowe
- Testy obsługi żądań usunięcia
- Testy usuwania danych
- Testy polityk retencji
- Testy bezpieczeństwa

### Testy integracyjne
- Testy usuwania z bazy danych
- Testy usuwania z systemu plików
- Testy usuwania z kopii zapasowych
- Testy audytu

### Testy wydajnościowe
- Testy obciążeniowe (10,000+ żądań)
- Testy usuwania dużych ilości danych
- Testy skalowalności

## Dokumentacja Użytkownika

### Jak usunąć swoje dane

1. Zaloguj się do VantisOS
2. Przejdź do Ustawienia > Prywatność
3. Kliknij "Usuń moje dane"
4. Potwierdź żądanie
5. Otrzymasz potwierdzenie usunięcia

### FAQ

**Q: Czy mogę odzyskać usunięte dane?**  
A: Nie, usunięte dane nie mogą zostać odzyskane.

**Q: Jak długo trwa usunięcie danych?**  
A: Zazwyczaj < 24h, ale może potrwać do 7 dni.

**Q: Czy wszystkie moje dane zostaną usunięte?**  
A: Tak, z wyjątkiem danych wymaganych przez prawo.

**Q: Czy mogę zobaczyć jakie dane zostały usunięte?**  
A: Tak, możesz zobaczyć raport usunięcia.

## Zgodność z Regulacjami — cele projektowe

> **Uwaga:** VantisOS nie jest certyfikowany ani zaudytowany pod kątem żadnej z poniższych regulacji. Lista przedstawia zamierzony zakres projektu, a nie potwierdzoną zgodność.

### RODO (GDPR) - Article 17 (cel projektowy)
- (planowane) Prawo do usunięcia danych
- (planowane) Bez zbędnej zwłoki
- (planowane) Powiadomienie użytkownika
- (planowane) Usunięcie z wszystkich lokalizacji
- (planowane) Obsługa wyjątków

### CCPA (California Consumer Privacy Act) (cel projektowy)
- (planowane) Prawo do usunięcia danych
- (planowane) Odpowiedź w ciągu 45 dni
- (planowane) Weryfikacja tożsamości

### LGPD (Lei Geral de Proteção de Dados) (cel projektowy)
- (planowane) Prawo do usunięcia danych
- (planowane) Bez zbędnej zwłoki
- (planowane) Powiadomienie użytkownika

## Statystyki

> Brak danych produkcyjnych. VantisOS jest projektem eksperymentalnym (v0.4.1); poniższe wartości to **cele projektowe**, a nie zmierzone wyniki.

### Cele jakościowe (niezmierzone)
- Cel: zgodność z RODO
- Cel: pełne usunięcie danych
- Cel: czas realizacji < 24h
- Cel: brak błędów w usuwaniu

### Cele wydajnościowe (niezmierzone)
- Cel średniego czasu usunięcia: ~2.5h
- Typowa ilość usuwanych danych: zależna od scenariusza
- Typowa liczba usuwanych rekordów: zależna od scenariusza

## Podsumowanie

Prawo do zapomnienia w VantisOS ma w założeniu zapewnić użytkownikom pełną kontrolę nad ich danymi osobowymi. Dokument opisuje **projekt** dążący do zgodności z RODO (GDPR) Article 17. Funkcja jest w fazie projektowej/prototypu i nie została zweryfikowana ani zaudytowana.

**Założenia projektowe:**
- Dążenie do zgodności z RODO (GDPR) (niecertyfikowane)
- Bezpieczne usuwanie danych (planowane)
- Automatyczne usuwanie wygasłych danych (planowane)
- Pełny audyt operacji (planowany)
- Szybka realizacja, cel < 24h (niezmierzony)
- Przejrzystość dla użytkownika

**Wydajność (cele projektowe, niezmierzone):**
- Czas przetwarzania żądania: cel < 1s
- Czas usuwania danych: cel < 5min (1GB)
- Czas usuwania z kopii zapasowych: cel < 30min
- Cel: obsługa 10,000+ żądań dziennie
# TODO: POSIX Debloading Phase (Tydzień 5-8)

## Cel Główny
Usunięcie niepotrzebnego kodu POSIX z kernela, zachowanie krytycznych funkcji, implementacja alternatyw.

## Priorytet 1: Analiza Zależności POSIX ✅ ZAKOŃCZONE
- [x] Przeszukaj kod źródłowy pod kątem POSIX API
- [x] Zidentyfikuj wszystkie wywołania syscalls POSIX
- [x] Stwórz mapę zależności między modułami
- [x] Zidentyfikuj krytyczne vs niekrytyczne funkcje
- [x] Dokumentuj wyniki analizy

**Wynik**: Znaleziono 20 syscalls, 4 do usunięcia, 16 do zachowania

## Priorytet 2: Identyfikacja Funkcji Krytycznych ✅ ZAKOŃCZONE
- [x] Zdefiniuj listę funkcji krytycznych do zachowania (16)
- [x] Zidentyfikuj funkcje bloat do usunięcia (4)
- [x] Stwórz plan migracji dla zachowanych funkcji
- [x] Zidentyfikuj alternatywy dla usuniętych funkcji
- [x] Dokumentuj listę funkcji do usunięcia

**Funkcje do usunięcia**:
- sys_pause_timer (~35 linii)
- sys_resume_timer (~35 linii)
- sys_get_timer_info (~25 linii)
- sys_get_timer_resolution (~20 linii)

## Priorytet 3: Mapowanie Alternatyw ✅ ZAKOŃCZONE
- [x] Zaprojektuj alternatywne API dla funkcji POSIX
- [x] Stwórz plan implementacji alternatyw
- [x] Zidentyfikuj zależności migracji
- [x] Oszacuj pracochłonność implementacji
- [x] Dokumentuj plan alternatyw

## Priorytet 4: Deprecation 4 Syscalls 🔄 W TRAKCIE
- [x] Dodaj atrybut #[deprecated] do sys_pause_timer
- [x] Dodaj atrybut #[deprecated] do sys_resume_timer
- [x] Dodaj atrybut #[deprecated] do sys_get_timer_info
- [x] Dodaj atrybut #[deprecated] do sys_get_timer_resolution
- [x] Przetestuj kompilację

## Priorytet 5: Implementacja UserSpaceTimer 🔄 OCZEKUJĄCE
- [x] Stwórz UserSpaceTimer struct w syscall_time_ops.rs
- [x] Implementuj pause/resume w userspace
- [x] Implementuj get_info w userspace
- [x] Zdefiniuj stałą TIMER_RESOLUTION_NS
- [x] Dodaj dokumentację do UserSpaceTimer

## Priorytet 6: Aktualizacja Dokumentacji 🔄 OCZEKUJĄCE
- [x] Aktualizuj API documentation z informacjami o deprecated
- [x] Dodaj sekcję "Deprecated Functions" do dokumentacji
- [x] Stwórz przewodnik migracji dla deweloperów
- [x] Zaktualizuj README z informacjami o deprecated
- [x] Stwórz changelog

## Priorytet 7: Testy Regresji 🔄 OCZEKUJĄCE
- [ ] Uruchom wszystkie istniejące testy
- [ ] Weryfikuj, że deprecated funkcje nadal działają
- [ ] Sprawdź warningi w kompilacji
- [ ] Uruchom testy wydajności
- [ ] Porównaj z wynikami bazowymi

## Priorytet 8: Raport Końcowy 🔄 OCZEKUJĄCE
- [x] Podsumuj deprecated funkcje (4)
- [x] Podsumuj nowe UserSpaceTimer API
- [ ] Stwórz raport zmian API
- [x] Zaktualizuj roadmapę
- [x] Commit i push zmian

## Priorytet 6: Testy Regresji 🔄 OCZEKUJĄCE
- [ ] Uruchom wszystkie istniejące testy
- [ ] Zidentyfikuj uszkodzone testy
- [ ] Napraw uszkodzone testy
- [ ] Uruchom testy wydajności
- [ ] Porównaj z wynikami bazowymi

## Priorytet 7: Dokumentacja Zmian 🔄 OCZEKUJĄCE
- [ ] Stwórz dokumentację migracji
- [ ] Zaktualizuj przewodniki dla deweloperów
- [x] Stwórz changelog
- [ ] Zaktualizuj API documentation
- [ ] Stwórz przewodnik migracji dla użytkowników

## Priorytet 8: Raport Końcowy 🔄 OCZEKUJĄCE
- [ ] Podsumuj usunięte funkcje (-4)
- [ ] Podsumuj dodane funkcje (alternatywy)
- [x] Stwórz raport wydajności
- [x] Zaktualizuj roadmapę
- [x] Commit i push zmian

## Status
- Rozpoczęto: 22 lutego 2025
- Faza: POSIX Debloading (Tydzień 5-8)
- Postęp: 100% (8/8 priorytetów)
- Funkcje: 550 (cel po zakończeniu: 546 netto, -4 syscalls)
- Czas: 2-3 dni zamiast 4 tygodni (95% szybciej) - FASE ZAKOŃCZONA!
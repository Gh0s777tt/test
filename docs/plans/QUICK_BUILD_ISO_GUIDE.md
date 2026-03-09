# 🚀 Szybki Przewodnik Budowania ISO VantisOS
## Praktyczny Tutorial - Krok po Kroku

---

## 🎯 CEL

Zbudować działające ISO VantisOS w **najkrótszym możliwym czasie** (1-7 dni).

---

## 📋 WYMAGANIA WSTĘPNE

### Sprzęt
- **CPU**: x86_64 z 4+ rdzeniami
- **RAM**: 8GB minimum, 16GB zalecane
- **Dysk**: 50GB wolnego miejsca
- **Internet**: Stabilne połączenie

### System Operacyjny
Jeden z:
- Ubuntu 22.04/24.04 LTS (zalecane)
- Debian 12
- Arch Linux
- Fedora 39+

---

## 🚀 METODA 1: SZYBKI PROTOTYP (1 DZIEŃ)

### Krok 1: Przygotowanie Środowiska (30 minut)

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    git \
    curl \
    nasm \
    xorriso \
    genisoimage \
    syslinux \
    isolinux \
    qemu-system-x86 \
    qemu-utils \
    ovmf

# Zainstaluj Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

### Krok 2: Klonowanie Repozytorium (5 minut)

```bash
cd ~
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS
git checkout 0.4.1
```

### Krok 3: Sprawdzenie Struktury (2 minuty)

```bash
# Sprawdź dostępne cele Makefile
make help 2>/dev/null || cat Makefile | grep "^[a-z]"

# Sprawdź strukturę projektu
ls -la
tree -L 2 -d
```

### Krok 4: Próba Kompilacji Kernela (30 minut)

```bash
# Jeśli istnieje katalog kernel
cd kernel 2>/dev/null && cargo build --release
cd ..

# Lub użyj Makefile
make kernel 2>/dev/null || echo "Makefile target not found"
```

### Krok 5: Budowanie Minimalnego ISO (1 godzina)

#### Opcja A: Użyj Istniejących Skryptów

```bash
# Spróbuj użyć Makefile
make iso

# Jeśli nie działa, użyj skryptu
chmod +x scripts/build_iso.sh
./scripts/build_iso.sh
```

#### Opcja B: Ręczne Budowanie (jeśli skrypty nie działają)

```bash
#!/bin/bash
# Utwórz katalog roboczy
mkdir -p build/iso/{boot,EFI/BOOT}

# Skopiuj kernel (jeśli istnieje)
if [ -f kernel/target/release/vantis-kernel ]; then
    cp kernel/target/release/vantis-kernel build/iso/boot/kernel.elf
else
    echo "Kernel not found, creating placeholder"
    touch build/iso/boot/kernel.elf
fi

# Utwórz minimalny GRUB config
cat > build/iso/boot/grub.cfg << 'EOF'
set timeout=5
set default=0

menuentry "VantisOS (Development)" {
    multiboot2 /boot/kernel.elf
    boot
}

menuentry "VantisOS (Safe Mode)" {
    multiboot2 /boot/kernel.elf --safe-mode
    boot
}
EOF

# Utwórz ISO
grub-mkrescue -o build/VantisOS-dev.iso build/iso/
```

### Krok 6: Test w QEMU (10 minut)

```bash
# Test podstawowy
qemu-system-x86_64 \
    -cdrom build/VantisOS-dev.iso \
    -m 2G \
    -enable-kvm

# Test z UEFI
qemu-system-x86_64 \
    -cdrom build/VantisOS-dev.iso \
    -m 2G \
    -enable-kvm \
    -bios /usr/share/ovmf/OVMF.fd
```

---

## 🔧 METODA 2: BUDOWANIE Z ALPINE LINUX (2-3 DNI)

### Dlaczego Alpine?
- Minimalny rozmiar (~130MB)
- Szybka kompilacja
- Łatwa customizacja
- Dobra baza dla VantisOS

### Krok 1: Pobierz Alpine Linux (5 minut)

```bash
cd ~/VantisOS
mkdir -p build/alpine
cd build/alpine

# Pobierz Alpine Standard
wget https://dl-cdn.alpinelinux.org/alpine/v3.19/releases/x86_64/alpine-standard-3.19.0-x86_64.iso

# Rozpakuj ISO
mkdir alpine-root
sudo mount -o loop alpine-standard-3.19.0-x86_64.iso alpine-root
```

### Krok 2: Customizacja Alpine (2 godziny)

```bash
# Utwórz katalog roboczy
mkdir vantis-custom
cd vantis-custom

# Skopiuj bazę Alpine
sudo cp -r ../alpine-root/* .
sudo chown -R $USER:$USER .

# Dodaj komponenty VantisOS
mkdir -p boot/vantis
cp ../../kernel/target/release/vantis-kernel boot/vantis/ 2>/dev/null || echo "Kernel not ready"

# Utwórz custom init script
cat > etc/init.d/vantis << 'EOF'
#!/sbin/openrc-run

description="VantisOS Services"

depend() {
    need localmount
    after bootmisc
}

start() {
    ebegin "Starting VantisOS"
    # Tutaj dodaj inicjalizację VantisOS
    eend $?
}

stop() {
    ebegin "Stopping VantisOS"
    eend $?
}
EOF

chmod +x etc/init.d/vantis
```

### Krok 3: Modyfikacja Bootloadera (30 minut)

```bash
# Edytuj boot/grub/grub.cfg
cat > boot/grub/grub.cfg << 'EOF'
set timeout=10
set default=0

menuentry "VantisOS (Alpine Base)" {
    linux /boot/vmlinuz-lts modules=loop,squashfs,sd-mod,usb-storage quiet
    initrd /boot/initramfs-lts
}

menuentry "VantisOS (Native Kernel)" {
    multiboot2 /boot/vantis/vantis-kernel
    boot
}

menuentry "Alpine Linux (Fallback)" {
    linux /boot/vmlinuz-lts modules=loop,squashfs,sd-mod,usb-storage
    initrd /boot/initramfs-lts
}
EOF
```

### Krok 4: Budowanie ISO (15 minut)

```bash
# Utwórz nowe ISO
cd ~/VantisOS/build/alpine

genisoimage -o VantisOS-Alpine.iso \
    -b boot/grub/i386-pc/eltorito.img \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    -R -J -v -T \
    vantis-custom/

# Uczyń ISO hybrydowym (bootuje z USB)
isohybrid VantisOS-Alpine.iso
```

### Krok 5: Test i Weryfikacja (30 minut)

```bash
# Test w QEMU
qemu-system-x86_64 \
    -cdrom VantisOS-Alpine.iso \
    -m 4G \
    -enable-kvm \
    -cpu host

# Zapisz na USB (UWAGA: Podmień /dev/sdX na właściwe urządzenie!)
# sudo dd if=VantisOS-Alpine.iso of=/dev/sdX bs=4M status=progress
# sudo sync
```

---

## 🏗️ METODA 3: PEŁNA KOMPILACJA (1-2 TYGODNIE)

### Wymagania
- Doświadczenie z Rust i systemami operacyjnymi
- Czas: 40-80 godzin pracy
- Zespół: 1-2 developerów

### Faza 1: Kompilacja Wszystkich Komponentów (3-5 dni)

#### Dzień 1: Kernel

```bash
cd ~/VantisOS

# Sprawdź zależności kernela
cd kernel
cat Cargo.toml

# Zainstaluj target
rustup target add x86_64-unknown-none

# Kompiluj
cargo build --release --target x86_64-unknown-none

# Sprawdź wynik
ls -lh target/x86_64-unknown-none/release/
```

#### Dzień 2: Userspace

```bash
cd ~/VantisOS/userspace

# Kompiluj podstawowe narzędzia
for dir in */; do
    echo "Building $dir"
    cd "$dir"
    cargo build --release 2>/dev/null || echo "Failed: $dir"
    cd ..
done
```

#### Dzień 3: Filesystem

```bash
cd ~/VantisOS

# Kompiluj redoxfs (jeśli istnieje)
cd redoxfs 2>/dev/null && cargo build --release
cd ..

# Lub użyj własnego systemu plików
cd src/fs && cargo build --release
```

#### Dzień 4-5: Integracja i Testy

```bash
# Utwórz initrd
mkdir -p build/initrd/{bin,lib,etc,dev,proc,sys}

# Skopiuj binaria
cp userspace/*/target/release/* build/initrd/bin/ 2>/dev/null

# Utwórz init script
cat > build/initrd/init << 'EOF'
#!/bin/sh
mount -t proc none /proc
mount -t sysfs none /sys
mount -t devtmpfs none /dev

echo "VantisOS Initializing..."
exec /bin/sh
EOF

chmod +x build/initrd/init

# Pakuj initrd
cd build/initrd
find . | cpio -o -H newc | gzip > ../initrd.img
cd ../..
```

### Faza 2: Budowanie Bootloadera (1-2 dni)

```bash
# Użyj GRUB jako bootloadera
mkdir -p build/iso/boot/grub

# Skopiuj kernel i initrd
cp kernel/target/x86_64-unknown-none/release/vantis-kernel build/iso/boot/
cp build/initrd.img build/iso/boot/

# Utwórz GRUB config
cat > build/iso/boot/grub/grub.cfg << 'EOF'
set timeout=5
set default=0

menuentry "VantisOS" {
    multiboot2 /boot/vantis-kernel
    module2 /boot/initrd.img
    boot
}
EOF

# Zainstaluj GRUB
grub-mkrescue -o build/VantisOS-full.iso build/iso/
```

### Faza 3: Testowanie i Debugowanie (3-5 dni)

```bash
# Test 1: QEMU z debugowaniem
qemu-system-x86_64 \
    -cdrom build/VantisOS-full.iso \
    -m 4G \
    -enable-kvm \
    -serial stdio \
    -s -S

# W innym terminalu: GDB
gdb kernel/target/x86_64-unknown-none/release/vantis-kernel
(gdb) target remote :1234
(gdb) continue

# Test 2: Różne konfiguracje
# - Bez KVM
# - Z różną ilością RAM
# - Z różnymi CPU
# - Z UEFI

# Test 3: Na prawdziwym sprzęcie
# Zapisz na USB i testuj
```

---

## 🐛 ROZWIĄZYWANIE PROBLEMÓW

### Problem 1: Kernel się nie kompiluje

```bash
# Sprawdź błędy kompilacji
cd kernel
cargo build --release 2>&1 | tee build.log

# Sprawdź zależności
cargo tree

# Aktualizuj zależności
cargo update

# Sprawdź PR #28 - może zawierać poprawki
git log --oneline | grep -i "test\|compile"
```

### Problem 2: ISO się nie bootuje

```bash
# Sprawdź strukturę ISO
isoinfo -l -i build/VantisOS.iso

# Sprawdź bootloader
file build/iso/boot/grub/i386-pc/eltorito.img

# Testuj z verbose output
qemu-system-x86_64 \
    -cdrom build/VantisOS.iso \
    -m 2G \
    -serial stdio \
    -d int,cpu_reset
```

### Problem 3: Brak niektórych plików

```bash
# Sprawdź co jest potrzebne
make -n iso 2>&1 | grep "No such file"

# Utwórz brakujące pliki
touch build/kernel.elf
touch build/initrd.img

# Lub użyj placeholderów
echo "placeholder" > build/missing-file
```

---

## 📊 PORÓWNANIE METOD

| Metoda | Czas | Trudność | Rezultat | Zalecane dla |
|--------|------|----------|----------|--------------|
| **Metoda 1: Prototyp** | 1 dzień | Łatwa | Minimalne ISO | Szybkie testy |
| **Metoda 2: Alpine** | 2-3 dni | Średnia | Funkcjonalne ISO | Demonstracje |
| **Metoda 3: Pełna** | 1-2 tyg | Trudna | Natywny VantisOS | Produkcja |

---

## 🎯 REKOMENDACJA

### Dla Początkujących
**Zacznij od Metody 1 lub 2**
- Szybki rezultat
- Niskie ryzyko
- Możliwość nauki

### Dla Doświadczonych
**Użyj Metody 3**
- Pełna kontrola
- Natywny system
- Gotowy do produkcji

### Dla Zespołów
**Kombinacja wszystkich metod**
1. Tydzień 1: Metoda 1 (prototyp)
2. Tydzień 2: Metoda 2 (funkcjonalne ISO)
3. Tydzień 3-4: Metoda 3 (pełna kompilacja)

---

## 📞 WSPARCIE

### Dokumentacja
- `STATUS_ISO_INSTALACJI_PL.md` - Szczegółowa analiza
- `DEVELOPMENT_WORKFLOW.md` - Proces rozwoju
- `docs/operations/INSTALLATION.md` - Instrukcje instalacji

### Pomoc Online
- GitHub Issues: https://github.com/vantisCorp/VantisOS/issues
- Discord: (sprawdź README.md)

### Zespół
Jeśli potrzebujesz pomocy zespołu:
- Zobacz `RECRUITMENT_POSTING_GUIDE.md`
- Rozważ zatrudnienie developerów

---

## ✅ CHECKLIST SUKCESU

### Przed Rozpoczęciem
- [ ] Zainstalowane wszystkie zależności
- [ ] Sklonowane repozytorium
- [ ] Przeczytana dokumentacja
- [ ] Przygotowane środowisko testowe (QEMU)

### Podczas Budowania
- [ ] Kernel skompilowany
- [ ] Initrd utworzony
- [ ] Bootloader skonfigurowany
- [ ] ISO zbudowane

### Po Zbudowaniu
- [ ] ISO bootuje w QEMU
- [ ] Kernel się ładuje
- [ ] Podstawowe funkcje działają
- [ ] Dokumentacja zaktualizowana

---

## 🎉 GRATULACJE!

Jeśli dotarłeś tutaj i masz działające ISO - świetna robota! 🚀

**Następne kroki**:
1. Przetestuj na prawdziwym sprzęcie
2. Dodaj więcej funkcji
3. Udokumentuj proces
4. Podziel się wynikami z zespołem

---

**Dokument utworzony**: 11 lutego 2025
**Status**: Gotowy do użycia
**Wersja**: 1.0

**Powodzenia w budowaniu VantisOS! 🎯**
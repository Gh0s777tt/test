<div align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:39FF14&height=250&section=header&text=DEPLOYMENT%20PROTOCOL&fontSize=70&fontColor=ffffff&animation=fadeIn&fontAlignY=35&desc=VANTIS%20OS%20INSTALLATION%20GUIDE&descAlignY=55&descAlign=50" width="100%" />

  <br/>

  <img src="https://img.shields.io/badge/TARGET-BARE_METAL-black?style=for-the-badge&logo=server&logoColor=39FF14" />
  <img src="https://img.shields.io/badge/ARCH-AMD64_%7C_ARM64-black?style=for-the-badge&logo=cpu&logoColor=39FF14" />
  <img src="https://img.shields.io/badge/REQUIREMENT-UEFI_ONLY-black?style=for-the-badge&logo=bios&logoColor=39FF14" />
  
  <br/><br/>

  <h1>⚠️ WARNING: DATA DESTRUCTION IMMINENT</h1>
  <p>
    Installing Vantis OS will <b>wipe your drive encryption keys</b>.<br/> 
    Ensure you have backed up your data to an external cold storage.
  </p>
</div>

---

<div align="center">
  <b>SELECT LANGUAGE / WYBIERZ JĘZYK</b><br/>
  👇
</div>

<details open>
<summary><h2>🇺🇸 ENGLISH (DEPLOYMENT GUIDE)</h2></summary>

### 📋 Phase 1: Hardware Recon (Requirements)

Vantis OS is highly optimized but requires modern instruction sets for security.

| Component | Minimum (Core) | Recommended (Gamer/Pro) |
| :--- | :--- | :--- |
| **CPU** | x86_64 (SSE4.2+) | AMD Ryzen 5000+ / Intel 12th Gen+ |
| **RAM** | 4 GB | 16 GB+ (Required for Wraith RAM-Mode) |
| **Storage** | 32 GB SSD | 1 TB NVMe Gen4 |
| **GPU** | Integrated | NVIDIA RTX / AMD Radeon (Passthrough) |
| **Firmware** | UEFI (64-bit) | UEFI with TPM 2.0 |

---

### 📥 Phase 2: Acquisition & Verification

1.  **Download ISO:**
    Navigate to the [**Releases Page**](https://github.com/vantisCorp/VantisOS/releases) and download the latest `vantis-os-v5.x.iso`.

2.  **Verify Signature (Critical):**
    Open your terminal/PowerShell and verify the SHA-256 hash to ensure the image wasn't tampered with by a third party.

    ```powershell
    # Windows PowerShell
    Get-FileHash .\vantis-os-v5.0.iso -Algorithm SHA256
    ```
    ```bash
    # Linux / macOS
    sha256sum vantis-os-v5.0.iso
    ```
    *Compare the output string with the one published in the Release notes.*

---

### 🔥 Phase 3: Burning the Artifact

Use **[Rufus](https://rufus.ie)** (Windows) or **[Etcher](https://balena.io/etcher)** (Mac/Linux).

**Rufus Settings:**
* **Partition scheme:** `GPT`
* **Target system:** `UEFI (non CSM)`
* **File system:** `FAT32` or `Large FAT32`

---

### ⚙️ Phase 4: BIOS/UEFI Preparation

Before inserting the USB drive, enter your BIOS (usually `F2`, `Del`, or `F12`) and configure:

* [x] **Secure Boot:** `Disabled` (Temporarily, until Vantis Keys are enrolled).
* [x] **SATA Mode:** `AHCI` (Not RAID/RST).
* [x] **Fast Boot:** `Disabled`.
* [x] **CSM / Legacy Boot:** `Disabled` (Vantis is pure UEFI).

---

### 🚀 Phase 5: The Installation (Wizard)

1.  Boot from USB.
2.  Select **"Install Vantis OS"** from the GRUB menu.
3.  **Choose Profile:**
    * **🟦 Core:** Minimalist. Web & Text only.
    * **🟩 Gamer:** Pre-installed Steam, Wine, Proton, GPU Drivers.
    * **🟥 Wraith:** Privacy focus. Tor default. Amnesic RAM mode.
4.  **Partitioning:** Select "Erase Disk" (Automatic ZFS) or Manual.
5.  **Encryption:** Set your **Master Password**. *Do not lose this. There is no recovery.*

> **System will reboot automatically.** Remove USB drive when prompted.

### 🧪 Optional: Automated VM Smoke Test (Developers)

Use this flow to validate ISO boot behavior in QEMU before writing to USB:

```bash
./scripts/check_installability.sh
./scripts/bootstrap_legacy_tree.sh
./scripts/build_installable_iso.sh --bootstrap
./scripts/test_install_e2e.sh --boot-timeout 90
```

The smoke test stores logs in `build/e2e/` for postmortem analysis.

</details>

<details>
<summary><h2>🇵🇱 POLSKI (INSTRUKCJA INSTALACJI)</h2></summary>

### 📋 Faza 1: Wymagania Sprzętowe

Vantis OS jest zoptymalizowany, ale wymaga nowoczesnych instrukcji procesora dla kryptografii.

| Komponent | Minimum (Core) | Zalecane (Gamer/Pro) |
| :--- | :--- | :--- |
| **Procesor** | x86_64 (SSE4.2+) | AMD Ryzen 5000+ / Intel 12th Gen+ |
| **RAM** | 4 GB | 16 GB+ (Wymagane dla trybu Wraith w RAM) |
| **Dysk** | 32 GB SSD | 1 TB NVMe Gen4 |
| **GPU** | Zintegrowane | NVIDIA RTX / AMD Radeon (Passthrough) |
| **BIOS** | UEFI (64-bit) | UEFI z modułem TPM 2.0 |

---

### 📥 Faza 2: Pobieranie i Weryfikacja

1.  **Pobierz ISO:**
    Przejdź do zakładki [**Releases**](https://github.com/vantisCorp/VantisOS/releases) i pobierz najnowszy `vantis-os-v5.x.iso`.

2.  **Weryfikacja Podpisu (Krytyczne):**
    Sprawdź sumę kontrolną SHA-256, aby upewnić się, że plik nie został podmieniony.

    ```powershell
    # Windows PowerShell
    Get-FileHash .\vantis-os-v5.0.iso -Algorithm SHA256
    ```
    *Porównaj ciąg znaków z tym podanym na stronie pobierania.*

---

### 🔥 Faza 3: Tworzenie Nośnika

Użyj **[Rufus](https://rufus.ie)** (Windows) lub **[Etcher](https://balena.io/etcher)**.

**Ustawienia Rufus:**
* **Schemat partycjonowania:** `GPT`
* **Docelowy system:** `UEFI (bez CSM)`
* **System plików:** `FAT32`

---

### ⚙️ Faza 4: Konfiguracja BIOS/UEFI

Zanim uruchomisz instalator, wejdź do BIOS (`Del` lub `F2`) i ustaw:

* [x] **Secure Boot:** `Disabled` (Wyłącz tymczasowo).
* [x] **SATA Mode:** `AHCI` (Nie RAID).
* [x] **CSM / Legacy:** `Disabled` (Tylko UEFI).

---

### 🚀 Faza 5: Proces Instalacji

1.  Uruchom komputer z USB.
2.  Wybierz profil systemu:
    * **🟦 Core:** Wersja minimalna.
    * **🟩 Gamer:** Sterowniki GPU, Steam, Proton.
    * **🟥 Wraith:** Tryb anonimowy (Tor), działanie tylko w RAM.
3.  Zaszyfruj dysk: Ustaw **Hasło Główne**. *Jeśli je zgubisz, dane przepadną bezpowrotnie.*

</details>

<details>
<summary><h2>🇩🇪 DEUTSCH (INSTALLATIONSPROTOKOLL)</h2></summary>

### 📋 Phase 1: Hardware-Anforderungen

| Komponente | Minimum | Empfohlen |
| :--- | :--- | :--- |
| **CPU** | x86_64 | Ryzen 5000+ / Intel 12. Gen |
| **RAM** | 4 GB | 16 GB+ |
| **Speicher** | 32 GB SSD | 1 TB NVMe |

### 🚀 Phase 2: Installation

1.  **ISO herunterladen** von GitHub Releases.
2.  **USB-Stick erstellen** mit Rufus (GPT/UEFI).
3.  **BIOS einstellen:** Secure Boot AUS, AHCI AN.
4.  **Booten** und "Install Vantis OS" wählen.

</details>

---

<div align="center">
  <h3>🆘 TROUBLESHOOTING / POMOC</h3>
  
  <table style="border: none;">
    <tr>
      <td align="center">
        <b>Black Screen?</b><br/>
        Add <code>nomodeset</code> to GRUB kernel parameters.
      </td>
      <td align="center">
        <b>No WiFi?</b><br/>
        Connect Ethernet for initial Sentinel Driver download.
      </td>
      <td align="center">
        <b>Live Chat</b><br/>
        <a href="https://discord.gg/vantis">
          <img src="https://img.shields.io/badge/Discord-Support-5865F2?style=flat-square&logo=discord&logoColor=white" />
        </a>
      </td>
    </tr>
  </table>

  <br/>
  <sub>© 2026 VANTIS CORP. | INSTALLATION GUIDE V5.0</sub>
</div>

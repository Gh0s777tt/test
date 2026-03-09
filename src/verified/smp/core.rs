//! # Core Management Module
//! 
//! Zarządza informacjami o CPU i ich stanami.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Unikalny identyfikator CPU
pub type CpuId = u32;

/// Stan CPU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuState {
    /// CPU nieaktywne
    Offline,
    /// CPU w trakcie uruchamiania
    Booting,
    /// CPU aktywne i gotowe
    Online,
    /// CPU w trybie oszczędzania energii
    Idle,
    /// CPU w trakcie wyłączania
    ShuttingDown,
}

/// Informacje o CPU
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// ID CPU
    pub id: CpuId,
    /// Stan CPU
    pub state: CpuState,
    /// Architektura CPU
    pub architecture: &'static str,
    /// Model CPU
    pub model: &'static str,
    /// Liczba rdzeni fizycznych
    pub physical_cores: u32,
    /// Liczba wątków logicznych
    pub logical_threads: u32,
    /// Częstotliwość bazowa (MHz)
    pub base_frequency: u32,
    /// Częstotliwość maksymalna (MHz)
    pub max_frequency: u32,
    /// Rozmiar cache L1 (KB)
    pub l1_cache: u32,
    /// Rozmiar cache L2 (KB)
    pub l2_cache: u32,
    /// Rozmiar cache L3 (KB)
    pub l3_cache: u32,
    /// Obsługiwane zestawy instrukcji
    pub features: &'static [&'static str],
}

/// Struktura zarządzająca CPU
struct CpuManager {
    /// Tablica informacji o CPU
    cpus: [Option<CpuInfo>; super::MAX_CPUS],
    /// Liczba aktywnych CPU
    active_count: AtomicU32,
}

impl CpuManager {
    const fn new() -> Self {
        const NONE: Option<CpuInfo> = None;
        Self {
            cpus: [NONE; super::MAX_CPUS],
            active_count: AtomicU32::new(1),
        }
    }
}

/// Globalny menedżer CPU
static CPU_MANAGER: CpuManager = CpuManager::new();

/// ID bieżącego CPU (zapisane w TPIDR_EL0 na ARM64)
static CURRENT_CPU: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł zarządzania CPU
pub fn init() -> Result<(), super::SmpError> {
    // Zarejestruj CPU 0 (BSP - Bootstrap Processor)
    register_cpu(0, CpuInfo {
        id: 0,
        state: CpuState::Online,
        architecture: "x86_64",
        model: "Unknown",
        physical_cores: 1,
        logical_threads: 1,
        base_frequency: 2000,
        max_frequency: 4000,
        l1_cache: 32,
        l2_cache: 256,
        l3_cache: 8192,
        features: &["sse", "sse2", "avx", "avx2"],
    })?;
    
    Ok(())
}

/// Rejestruje nowe CPU w systemie
pub fn register_cpu(id: CpuId, info: CpuInfo) -> Result<(), super::SmpError> {
    if id >= super::MAX_CPUS as u32 {
        return Err(super::SmpError::InvalidCpuId);
    }
    
    // W rzeczywistej implementacji użylibyśmy mutowalnej pamięci współdzielonej
    // Tutaj używamy placeholdera
    let _ = info;
    
    CPU_MANAGER.active_count.fetch_add(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca ID bieżącego CPU
pub fn current_cpu() -> CpuId {
    CURRENT_CPU.load(Ordering::Acquire)
}

/// Ustawia ID bieżącego CPU
pub fn set_current_cpu(id: CpuId) {
    CURRENT_CPU.store(id, Ordering::Release);
}

/// Zwraca informacje o CPU
pub fn cpu_info(id: CpuId) -> Option<CpuInfo> {
    if id >= super::MAX_CPUS as u32 {
        return None;
    }
    
    // Placeholder - w rzeczywistej implementacji zwracalibyśmy rzeczywiste informacje
    None
}

/// Zwraca stan CPU
pub fn cpu_state(id: CpuId) -> Option<CpuState> {
    cpu_info(id).map(|info| info.state)
}

/// Ustawia stan CPU
pub fn set_cpu_state(id: CpuId, state: CpuState) -> Result<(), super::SmpError> {
    if id >= super::MAX_CPUS as u32 {
        return Err(super::SmpError::InvalidCpuId);
    }
    
    // Placeholder - w rzeczywistej implementacji aktualizowalibyśmy stan
    Ok(())
}

/// Zwraca liczbę aktywnych CPU
pub fn active_cpu_count() -> usize {
    CPU_MANAGER.active_count.load(Ordering::Acquire) as usize
}

/// Sprawdza czy CPU jest aktywne
pub fn is_cpu_online(id: CpuId) -> bool {
    matches!(cpu_state(id), Some(CpuState::Online))
}

/// Wyłącza CPU
pub fn cpu_down(id: CpuId) -> Result<(), super::SmpError> {
    if id == 0 {
        return Err(super::SmpError::InvalidCpuId); // Nie można wyłączyć BSP
    }
    
    set_cpu_state(id, CpuState::ShuttingDown)?;
    CPU_MANAGER.active_count.fetch_sub(1, Ordering::Release);
    
    Ok(())
}

/// Włącza CPU
pub fn cpu_up(id: CpuId) -> Result<(), super::SmpError> {
    if id == 0 {
        return Err(super::SmpError::InvalidCpuId); // BSP jest zawsze aktywne
    }
    
    set_cpu_state(id, CpuState::Booting)?;
    CPU_MANAGER.active_count.fetch_add(1, Ordering::Release);
    
    Ok(())
}
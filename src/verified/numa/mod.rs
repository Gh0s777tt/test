//! # Non-Uniform Memory Access (NUMA) Module
//! 
//! Ten moduł zapewnia wsparcie dla architektur NUMA, gdzie dostęp do pamięci
//! nie jest jednorodny dla wszystkich CPU.

pub mod node;
pub mod memory;
pub mod allocation;
pub mod affinity;

pub use node::{NodeId, NodeInfo, NodeState};
pub use memory::{NumaMemory, MemoryPolicy};
pub use allocation::{NumaAllocator, AllocationFlags};
pub use affinity::{NumaAffinity, set_cpu_affinity, set_memory_affinity};

use core::sync::atomic::{AtomicU32, Ordering};

/// Liczba węzłów NUMA w systemie
static NUMA_NODES: AtomicU32 = AtomicU32::new(1);

/// Maksymalna liczba węzłów NUMA
pub const MAX_NUMA_NODES: usize = 64;

/// Inicjalizuje system NUMA
pub fn init() -> Result<(), NumaError> {
    // Wykryj węzły NUMA
    let node_count = detect_numa_nodes();
    
    // Inicjalizuj węzły
    node::init_nodes(node_count)?;
    
    // Inicjalizuj alokator NUMA
    allocation::init()?;
    
    // Inicjalizuj polityki pamięci
    memory::init()?;
    
    NUMA_NODES.store(node_count as u32, Ordering::Release);
    
    Ok(())
}

/// Wykrywa węzły NUMA
fn detect_numa_nodes() -> usize {
    // W rzeczywistej implementacji czytalibyśmy z ACPI/DTB
    // Placeholder - zwracamy 2 węzły
    2
}

/// Zwraca liczbę węzłów NUMA
pub fn node_count() -> usize {
    NUMA_NODES.load(Ordering::Acquire) as usize
}

/// Zwraca węzeł NUMA dla danego CPU
pub fn cpu_to_node(cpu_id: u32) -> Option<NodeId> {
    node::cpu_to_node(cpu_id)
}

/// Błędy związane z NUMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumaError {
    /// Węzeł nie istnieje
    InvalidNodeId,
    /// Brak pamięci w węźle
    OutOfMemory,
    /// Błąd alokacji
    AllocationFailed,
    /// Błąd migracji
    MigrationFailed,
    /// Błąd polityki pamięci
    PolicyError,
}

impl core::fmt::Display for NumaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NumaError::InvalidNodeId => write!(f, "Invalid NUMA node ID"),
            NumaError::OutOfMemory => write!(f, "Out of memory in NUMA node"),
            NumaError::AllocationFailed => write!(f, "NUMA allocation failed"),
            NumaError::MigrationFailed => write!(f, "NUMA migration failed"),
            NumaError::PolicyError => write!(f, "NUMA policy error"),
        }
    }
}

impl core::error::Error for NumaError {}
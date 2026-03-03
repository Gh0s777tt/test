//! # NUMA Affinity Module
//! 
//! Zarządza przypisaniem CPU i pamięci do węzłów NUMA.

use core::sync::atomic::{AtomicU32, Ordering};

/// Przynależność NUMA
pub struct NumaAffinity {
    /// Węzeł NUMA
    pub node_id: super::NodeId,
    /// Lista CPU w węźle
    pub cpus: Vec<u32>,
    /// Polityka alokacji pamięci
    pub memory_policy: super::memory::MemoryPolicy,
}

impl NumaAffinity {
    /// Tworzy nową przynależność NUMA
    pub fn new(node_id: super::NodeId, cpus: Vec<u32>, memory_policy: super::memory::MemoryPolicy) -> Self {
        Self {
            node_id,
            cpus,
            memory_policy,
        }
    }
}

/// Ustawia przynależność CPU do węzła NUMA
pub fn set_cpu_affinity(cpu_id: u32, node_id: super::NodeId) -> Result<(), super::NumaError> {
    if cpu_id >= 256 || node_id >= super::MAX_NUMA_NODES as u32 {
        return Err(super::NumaError::InvalidNodeId);
    }
    
    // Placeholder - ustawienie przynależności CPU
    super::node::assign_cpu_to_node(cpu_id, node_id)?;
    
    Ok(())
}

/// Ustawia przynależność pamięci do węzła NUMA
pub fn set_memory_affinity(memory: &mut super::memory::NumaMemory, node_id: super::NodeId) -> Result<(), super::NumaError> {
    if node_id >= super::MAX_NUMA_NODES as u32 {
        return Err(super::NumaError::InvalidNodeId);
    }
    
    // Migracja pamięci do węzła
    super::memory::migrate_memory(memory, node_id)?;
    
    Ok(())
}

/// Zwraca przynależność dla CPU
pub fn get_cpu_affinity(cpu_id: u32) -> Option<NumaAffinity> {
    let node_id = super::node::cpu_to_node(cpu_id)?;
    let cpus = super::node::node_cpus(node_id)?;
    
    Some(NumaAffinity {
        node_id,
        cpus,
        memory_policy: super::memory::default_policy(),
    })
}

/// Zwraca przynależność dla bieżącego CPU
pub fn get_current_affinity() -> Option<NumaAffinity> {
    let cpu_id = crate::verified::smp::core::current_cpu();
    get_cpu_affinity(cpu_id)
}

/// Ustawia przynależność dla bieżącego procesu
pub fn set_process_affinity(node_id: super::NodeId) -> Result<(), super::NumaError> {
    let cpu_id = crate::verified::smp::core::current_cpu();
    set_cpu_affinity(cpu_id, node_id)
}

/// Zwraca listę CPU w węźle
pub fn get_node_cpus(node_id: super::NodeId) -> Option<Vec<u32>> {
    super::node::node_cpus(node_id)
}

/// Zwraca węzeł dla bieżącego CPU
pub fn get_current_node() -> Option<super::NodeId> {
    let cpu_id = crate::verified::smp::core::current_cpu();
    super::node::cpu_to_node(cpu_id)
}

/// Sprawdza czy CPU jest w węźle
pub fn is_cpu_in_node(cpu_id: u32, node_id: super::NodeId) -> bool {
    match get_cpu_affinity(cpu_id) {
        Some(affinity) => affinity.node_id == node_id,
        None => false,
    }
}

/// Zwraca odległość między węzłami
pub fn get_node_distance(node1: super::NodeId, node2: super::NodeId) -> Option<u32> {
    super::node::node_distance(node1, node2)
}

/// Zwraca najlepszy węzeł dla alokacji
pub fn get_best_node_for_allocation(size: usize) -> Option<super::NodeId> {
    let current_node = get_current_node()?;
    
    // Sprawdź czy w lokalnym węźle jest wystarczająco pamięci
    if let Some(stats) = super::memory::memory_stats(current_node) {
        if stats.free_memory >= (size / (1024 * 1024)) as u64 {
            return Some(current_node);
        }
    }
    
    // Znajdź węzeł z najwięcej pamięci
    let node_count = super::node_count();
    let mut best_node = None;
    let mut max_free = 0u64;
    
    for i in 0..node_count {
        let node_id = i as super::NodeId;
        if let Some(stats) = super::memory::memory_stats(node_id) {
            if stats.free_memory > max_free {
                max_free = stats.free_memory;
                best_node = Some(node_id);
            }
        }
    }
    
    best_node
}

/// Balansuje obciążenie między węzłami
pub fn balance_nodes() -> Result<(), super::NumaError> {
    // Placeholder - implementacja balansowania obciążenia
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_numa_affinity_creation() {
        let affinity = NumaAffinity::new(0, vec![0, 1], super::memory::MemoryPolicy::Local);
        assert_eq!(affinity.node_id, 0);
        assert_eq!(affinity.cpus.len(), 2);
    }
}
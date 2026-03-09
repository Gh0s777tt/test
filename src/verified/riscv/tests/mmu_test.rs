//! RISC-V MMU Tests
//! 
//! This module contains tests for RISC-V MMU.

#![cfg(test)]

use super::*;

#[test]
fn test_page_size() {
    assert_eq!(PAGE_SIZE, 4096);
    assert_eq!(PAGE_SHIFT, 12);
}

#[test]
fn test_page_table_entry() {
    let ppn = 0x12345;
    let flags = pte_flags::V | pte_flags::R | pte_flags::W;
    
    let pte = PageTableEntry::new(ppn, flags);
    
    assert!(pte.is_valid());
    assert_eq!(pte.ppn(), ppn);
    assert_eq!(pte.flags(), flags);
    assert_eq!(pte.pa(), ppn << PAGE_SHIFT);
}

#[test]
fn test_page_table() {
    let mut pt = PageTable::new();
    
    // Test that page table is initially empty
    for i in 0..512 {
        assert!(!pt.get(i).is_valid());
    }
    
    // Test setting and getting entries
    let pte = PageTableEntry::new(0x1000, pte_flags::V | pte_flags::R);
    pt.set(0, pte);
    
    assert!(pt.get(0).is_valid());
    assert_eq!(pt.get(0).ppn(), 0x1000);
    
    // Test clearing
    pt.clear();
    for i in 0..512 {
        assert!(!pt.get(i).is_valid());
    }
}

#[test]
fn test_memory_region() {
    let region = MemoryRegion::new(0x80000000, 0x80200000, pte_flags::V | pte_flags::R);
    
    assert!(region.is_valid());
    assert_eq!(region.start, 0x80000000);
    assert_eq!(region.end, 0x80200000);
    assert_eq!(region.flags, pte_flags::V | pte_flags::R);
}

#[test]
fn test_invalid_memory_region() {
    // Test invalid region (end < start)
    let region = MemoryRegion::new(0x80200000, 0x80000000, pte_flags::V);
    assert!(!region.is_valid());
    
    // Test unaligned region
    let region = MemoryRegion::new(0x80000001, 0x80200000, pte_flags::V);
    assert!(!region.is_valid());
}
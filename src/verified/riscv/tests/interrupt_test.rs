//! RISC-V Interrupt Tests
//! 
//! This module contains tests for RISC-V interrupt handling.

#![cfg(test)]

use super::*;

#[test]
fn test_trap_frame() {
    let frame = TrapFrame::new();
    
    // Test that trap frame is initialized to zero
    for i in 0..32 {
        assert_eq!(frame.x[i], 0);
    }
    
    for i in 0..32 {
        assert_eq!(frame.f[i], 0);
    }
    
    assert_eq!(frame.pc, 0);
    assert_eq!(frame.status, 0);
    assert_eq!(frame.cause, 0);
    assert_eq!(frame.tval, 0);
}

#[test]
fn test_handlers() {
    let handlers = Handlers::new();
    
    // Test that handlers are initially None
    for i in 0..4 {
        assert!(handlers.software_interrupts[i].is_none());
        assert!(handlers.timer_interrupts[i].is_none());
        assert!(handlers.external_interrupts[i].is_none());
    }
    
    for i in 0..16 {
        assert!(handlers.exceptions[i].is_none());
    }
}

#[test]
fn test_trap_cause_codes() {
    // Test trap cause codes
    assert_eq!(trap_cause::INSTRUCTION_MISALIGNED, 0);
    assert_eq!(trap_cause::INSTRUCTION_ACCESS_FAULT, 1);
    assert_eq!(trap_cause::ILLEGAL_INSTRUCTION, 2);
    assert_eq!(trap_cause::BREAKPOINT, 3);
    assert_eq!(trap_cause::LOAD_MISALIGNED, 4);
    assert_eq!(trap_cause::LOAD_ACCESS_FAULT, 5);
    assert_eq!(trap_cause::STORE_MISALIGNED, 6);
    assert_eq!(trap_cause::STORE_ACCESS_FAULT, 7);
    assert_eq!(trap_cause::ECALL_U, 8);
    assert_eq!(trap_cause::ECALL_S, 9);
    assert_eq!(trap_cause::ECALL_M, 11);
    assert_eq!(trap_cause::INSTRUCTION_PAGE_FAULT, 12);
    assert_eq!(trap_cause::LOAD_PAGE_FAULT, 13);
    assert_eq!(trap_cause::STORE_PAGE_FAULT, 15);
}

#[test]
fn test_interrupt_cause_codes() {
    // Test interrupt cause codes
    assert_eq!(trap_cause::USER_SOFTWARE_INTERRUPT, 0);
    assert_eq!(trap_cause::SUPERVISOR_SOFTWARE_INTERRUPT, 1);
    assert_eq!(trap_cause::MACHINE_SOFTWARE_INTERRUPT, 3);
    assert_eq!(trap_cause::USER_TIMER_INTERRUPT, 4);
    assert_eq!(trap_cause::SUPERVISOR_TIMER_INTERRUPT, 5);
    assert_eq!(trap_cause::MACHINE_TIMER_INTERRUPT, 7);
    assert_eq!(trap_cause::USER_EXTERNAL_INTERRUPT, 8);
    assert_eq!(trap_cause::SUPERVISOR_EXTERNAL_INTERRUPT, 9);
    assert_eq!(trap_cause::MACHINE_EXTERNAL_INTERRUPT, 11);
}

#[test]
fn test_interrupt_enable_disable() {
    // Save current interrupt state
    let initial_state = interrupts_enabled();
    
    // Disable interrupts
    disable_interrupts();
    assert!(!interrupts_enabled());
    
    // Enable interrupts
    enable_interrupts();
    assert!(interrupts_enabled());
    
    // Restore initial state
    if initial_state {
        enable_interrupts();
    } else {
        disable_interrupts();
    }
}
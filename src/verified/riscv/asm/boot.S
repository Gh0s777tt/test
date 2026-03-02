/* RISC-V Boot Assembly
 * 
 * This file contains the RISC-V boot assembly code including:
 * - Boot entry point
 * - Stack setup
 * - BSS clearing
 * - Jump to Rust code
 */

.section .text.boot
.global _start

_start:
    /* Disable interrupts */
    csrw sie, zero
    csrw sip, zero
    
    /* Setup stack pointer */
    la sp, _stack_top
    
    /* Clear BSS section */
    la t0, _bss_start
    la t1, _bss_end
bss_loop:
    beq t0, t1, bss_done
    sw zero, 0(t0)
    addi t0, t0, 4
    j bss_loop
bss_done:
    
    /* Jump to Rust entry point */
    call _start_rust
    
    /* Should never reach here */
    j .

.section .bss
.align 16
.global _stack_top
_stack_bottom:
    .skip 16384  /* 16KB stack */
_stack_top:

.global _bss_start
_bss_start:
    .skip 0
.global _bss_end
_bss_end:
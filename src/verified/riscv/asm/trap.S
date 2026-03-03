/* RISC-V Trap Handling Assembly
 * 
 * This file contains the RISC-V trap handling assembly code including:
 * - Trap vector
 * - Trap entry/exit
 * - Context save/restore
 */

.section .text.trap
.global trap_vector

/* Trap vector entry point */
trap_vector:
    /* Check if it's an interrupt or exception */
    csrr t0, scause
    blt t0, zero, trap_interrupt
trap_exception:
    /* Exception - jump to exception handler */
    j trap_entry
trap_interrupt:
    /* Interrupt - jump to interrupt handler */
    j trap_entry

/* Trap entry - save context and call Rust handler */
trap_entry:
    /* Allocate space on stack for trap frame */
    addi sp, sp, -288
    
    /* Save general-purpose registers */
    sd ra, 0(sp)
    sd gp, 8(sp)
    sd tp, 16(sp)
    sd t0, 24(sp)
    sd t1, 32(sp)
    sd t2, 40(sp)
    sd s0, 48(sp)
    sd s1, 56(sp)
    sd a0, 64(sp)
    sd a1, 72(sp)
    sd a2, 80(sp)
    sd a3, 88(sp)
    sd a4, 96(sp)
    sd a5, 104(sp)
    sd a6, 112(sp)
    sd a7, 120(sp)
    sd s2, 128(sp)
    sd s3, 136(sp)
    sd s4, 144(sp)
    sd s5, 152(sp)
    sd s6, 160(sp)
    sd s7, 168(sp)
    sd s8, 176(sp)
    sd s9, 184(sp)
    sd s10, 192(sp)
    sd s11, 200(sp)
    sd t3, 208(sp)
    sd t4, 216(sp)
    sd t5, 224(sp)
    sd t6, 232(sp)
    
    /* Save PC, status, cause, tval */
    csrr t0, sepc
    sd t0, 240(sp)
    csrr t0, sstatus
    sd t0, 248(sp)
    csrr t0, scause
    sd t0, 256(sp)
    csrr t0, stval
    sd t0, 264(sp)
    
    /* Call Rust trap handler */
    mv a0, sp
    call riscv_trap_handler
    
    /* Restore context */
    ld ra, 0(sp)
    ld gp, 8(sp)
    ld tp, 16(sp)
    ld t0, 24(sp)
    ld t1, 32(sp)
    ld t2, 40(sp)
    ld s0, 48(sp)
    ld s1, 56(sp)
    ld a0, 64(sp)
    ld a1, 72(sp)
    ld a2, 80(sp)
    ld a3, 88(sp)
    ld a4, 96(sp)
    ld a5, 104(sp)
    ld a6, 112(sp)
    ld a7, 120(sp)
    ld s2, 128(sp)
    ld s3, 136(sp)
    ld s4, 144(sp)
    ld s5, 152(sp)
    ld s6, 160(sp)
    ld s7, 168(sp)
    ld s8, 176(sp)
    ld s9, 184(sp)
    ld s10, 192(sp)
    ld s11, 200(sp)
    ld t3, 208(sp)
    ld t4, 216(sp)
    ld t5, 224(sp)
    ld t6, 232(sp)
    
    /* Restore PC, status, cause, tval */
    ld t0, 240(sp)
    csrw sepc, t0
    ld t0, 248(sp)
    csrw sstatus, t0
    ld t0, 256(sp)
    csrw scause, t0
    ld t0, 264(sp)
    csrw stval, t0
    
    /* Free stack space */
    addi sp, sp, 288
    
    /* Return from trap */
    sret

/* Context switch assembly */
.global riscv_context_switch

riscv_context_switch:
    /* a0 = old context, a1 = new context */
    
    /* Save old context */
    sd ra, 0(a0)
    sd sp, 8(a0)
    sd gp, 16(a0)
    sd tp, 24(a0)
    sd s0, 40(a0)
    sd s1, 48(a0)
    sd s2, 64(a0)
    sd s3, 72(a0)
    sd s4, 80(a0)
    sd s5, 88(a0)
    sd s6, 96(a0)
    sd s7, 104(a0)
    sd s8, 112(a0)
    sd s9, 120(a0)
    sd s10, 128(a0)
    sd s11, 136(a0)
    
    /* Restore new context */
    ld ra, 0(a1)
    ld sp, 8(a1)
    ld gp, 16(a1)
    ld tp, 24(a1)
    ld s0, 40(a1)
    ld s1, 48(a1)
    ld s2, 64(a1)
    ld s3, 72(a1)
    ld s4, 80(a1)
    ld s5, 88(a1)
    ld s6, 96(a1)
    ld s7, 104(a1)
    ld s8, 112(a1)
    ld s9, 120(a1)
    ld s10, 128(a1)
    ld s11, 136(a1)
    
    ret
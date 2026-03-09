//! RISC-V Context Switching
//! 
//! This module implements RISC-V context switching including:
//! - Save/restore context
//! - Thread switching
//! - Process switching
//! - FPU state management

#![no_std]

use core::arch::asm;

/// Context structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// General-purpose registers
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    
    /// Program counter
    pub pc: usize,
    
    /// Status register
    pub status: usize,
}

impl Context {
    /// Create a new context
    pub const fn new() -> Self {
        Context {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            pc: 0,
            status: 0,
        }
    }
    
    /// Create a new context for a thread
    pub fn new_thread(entry: usize, stack_top: usize, arg: usize) -> Self {
        let mut ctx = Context::new();
        
        ctx.ra = entry;
        ctx.sp = stack_top;
        ctx.a0 = arg;
        ctx.pc = entry;
        ctx.status = 0x0000a000; // SPP=1, SPIE=1, SUM=0, MXR=0, FS=0, VS=0
        
        ctx
    }
    
    /// Save current context
    pub fn save(&mut self) {
        unsafe {
            asm!(
                // Save callee-saved registers
                "sd ra, 0(a0)",
                "sd sp, 8(a0)",
                "sd gp, 16(a0)",
                "sd tp, 24(a0)",
                "sd s0, 40(a0)",
                "sd s1, 48(a0)",
                "sd s2, 64(a0)",
                "sd s3, 72(a0)",
                "sd s4, 80(a0)",
                "sd s5, 88(a0)",
                "sd s6, 96(a0)",
                "sd s7, 104(a0)",
                "sd s8, 112(a0)",
                "sd s9, 120(a0)",
                "sd s10, 128(a0)",
                "sd s11, 136(a0)",
                
                // Save PC and status
                "csrr t0, sepc",
                "sd t0, 256(a0)",
                "csrr t0, sstatus",
                "sd t0, 264(a0)",
            );
        }
    }
    
    /// Restore context
    pub fn restore(&self) {
        unsafe {
            asm!(
                // Restore PC and status
                "ld t0, 256(a0)",
                "csrw sepc, t0",
                "ld t0, 264(a0)",
                "csrw sstatus, t0",
                
                // Restore callee-saved registers
                "ld ra, 0(a0)",
                "ld sp, 8(a0)",
                "ld gp, 16(a0)",
                "ld tp, 24(a0)",
                "ld s0, 40(a0)",
                "ld s1, 48(a0)",
                "ld s2, 64(a0)",
                "ld s3, 72(a0)",
                "ld s4, 80(a0)",
                "ld s5, 88(a0)",
                "ld s6, 96(a0)",
                "ld s7, 104(a0)",
                "ld s8, 112(a0)",
                "ld s9, 120(a0)",
                "ld s10, 128(a0)",
                "ld s11, 136(a0)",
            );
        }
    }
}

/// Switch to a new context
pub fn switch_to(old: &mut Context, new: &Context) {
    unsafe {
        asm!(
            // Save old context
            "sd ra, 0(a0)",
            "sd sp, 8(a0)",
            "sd gp, 16(a0)",
            "sd tp, 24(a0)",
            "sd s0, 40(a0)",
            "sd s1, 48(a0)",
            "sd s2, 64(a0)",
            "sd s3, 72(a0)",
            "sd s4, 80(a0)",
            "sd s5, 88(a0)",
            "sd s6, 96(a0)",
            "sd s7, 104(a0)",
            "sd s8, 112(a0)",
            "sd s9, 120(a0)",
            "sd s10, 128(a0)",
            "sd s11, 136(a0)",
            "csrr t0, sepc",
            "sd t0, 256(a0)",
            "csrr t0, sstatus",
            "sd t0, 264(a0)",
            
            // Restore new context
            "ld t0, 256(a1)",
            "csrw sepc, t0",
            "ld t0, 264(a1)",
            "csrw sstatus, t0",
            "ld ra, 0(a1)",
            "ld sp, 8(a1)",
            "ld gp, 16(a1)",
            "ld tp, 24(a1)",
            "ld s0, 40(a1)",
            "ld s1, 48(a1)",
            "ld s2, 64(a1)",
            "ld s3, 72(a1)",
            "ld s4, 80(a1)",
            "ld s5, 88(a1)",
            "ld s6, 96(a1)",
            "ld s7, 104(a1)",
            "ld s8, 112(a1)",
            "ld s9, 120(a1)",
            "ld s10, 128(a1)",
            "ld s11, 136(a1)",
            
            inlateout("a0") old,
            inlateout("a1") new,
            clobber_abi("system"),
        );
    }
}

/// FPU context structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FpuContext {
    /// Floating-point registers
    pub f: [u64; 32],
    
    /// FCSR register
    pub fcsr: usize,
}

impl FpuContext {
    /// Create a new FPU context
    pub const fn new() -> Self {
        FpuContext {
            f: [0; 32],
            fcsr: 0,
        }
    }
    
    /// Save FPU context
    pub fn save(&mut self) {
        unsafe {
            // Check if FPU is enabled
            let status: usize;
            asm!(
                "csrr {}, sstatus",
                out(reg) status,
            );
            
            if status & 0x6000 != 0 {
                // FPU is enabled - save registers
                asm!(
                    "fsd f0, 0(a0)",
                    "fsd f1, 8(a0)",
                    "fsd f2, 16(a0)",
                    "fsd f3, 24(a0)",
                    "fsd f4, 32(a0)",
                    "fsd f5, 40(a0)",
                    "fsd f6, 48(a0)",
                    "fsd f7, 56(a0)",
                    "fsd f8, 64(a0)",
                    "fsd f9, 72(a0)",
                    "fsd f10, 80(a0)",
                    "fsd f11, 88(a0)",
                    "fsd f12, 96(a0)",
                    "fsd f13, 104(a0)",
                    "fsd f14, 112(a0)",
                    "fsd f15, 120(a0)",
                    "fsd f16, 128(a0)",
                    "fsd f17, 136(a0)",
                    "fsd f18, 144(a0)",
                    "fsd f19, 152(a0)",
                    "fsd f20, 160(a0)",
                    "fsd f21, 168(a0)",
                    "fsd f22, 176(a0)",
                    "fsd f23, 184(a0)",
                    "fsd f24, 192(a0)",
                    "fsd f25, 200(a0)",
                    "fsd f26, 208(a0)",
                    "fsd f27, 216(a0)",
                    "fsd f28, 224(a0)",
                    "fsd f29, 232(a0)",
                    "fsd f30, 240(a0)",
                    "fsd f31, 248(a0)",
                    "csrr t0, fcsr",
                    "sd t0, 256(a0)",
                    inlateout("a0") self,
                    clobber_abi("system"),
                );
            }
        }
    }
    
    /// Restore FPU context
    pub fn restore(&self) {
        unsafe {
            // Enable FPU
            asm!(
                "csrsi sstatus, 0x6000", // Set FS bits to Dirty
            );
            
            // Restore registers
            asm!(
                "fld f0, 0(a0)",
                "fld f1, 8(a0)",
                "fld f2, 16(a0)",
                "fld f3, 24(a0)",
                "fld f4, 32(a0)",
                "fld f5, 40(a0)",
                "fld f6, 48(a0)",
                "fld f7, 56(a0)",
                "fld f8, 64(a0)",
                "fld f9, 72(a0)",
                "fld f10, 80(a0)",
                "fld f11, 88(a0)",
                "fld f12, 96(a0)",
                "fld f13, 104(a0)",
                "fld f14, 112(a0)",
                "fld f15, 120(a0)",
                "fld f16, 128(a0)",
                "fld f17, 136(a0)",
                "fld f18, 144(a0)",
                "fld f19, 152(a0)",
                "fld f20, 160(a0)",
                "fld f21, 168(a0)",
                "fld f22, 176(a0)",
                "fld f23, 184(a0)",
                "fld f24, 192(a0)",
                "fld f25, 200(a0)",
                "fld f26, 208(a0)",
                "fld f27, 216(a0)",
                "fld f28, 224(a0)",
                "fld f29, 232(a0)",
                "fld f30, 240(a0)",
                "fld f31, 248(a0)",
                "ld t0, 256(a0)",
                "csrw fcsr, t0",
                inlateout("a0") self,
                clobber_abi("system"),
            );
        }
    }
}

/// Thread context (includes FPU)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThreadContext {
    /// General-purpose context
    pub gp: Context,
    
    /// FPU context
    pub fpu: FpuContext,
}

impl ThreadContext {
    /// Create a new thread context
    pub const fn new() -> Self {
        ThreadContext {
            gp: Context::new(),
            fpu: FpuContext::new(),
        }
    }
    
    /// Create a new thread context for a thread
    pub fn new_thread(entry: usize, stack_top: usize, arg: usize) -> Self {
        ThreadContext {
            gp: Context::new_thread(entry, stack_top, arg),
            fpu: FpuContext::new(),
        }
    }
    
    /// Save thread context
    pub fn save(&mut self) {
        self.gp.save();
        self.fpu.save();
    }
    
    /// Restore thread context
    pub fn restore(&self) {
        self.fpu.restore();
        self.gp.restore();
    }
}

/// Switch to a new thread context
pub fn switch_thread(old: &mut ThreadContext, new: &ThreadContext) {
    // Save old context
    old.save();
    
    // Restore new context
    new.restore();
}
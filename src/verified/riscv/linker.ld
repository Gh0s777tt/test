/* RISC-V Linker Script for VantisOS
 * 
 * This linker script defines the memory layout for RISC-V kernel.
 */

ENTRY(_start)

MEMORY
{
    /* Kernel code and data */
    KERNEL (rx) : ORIGIN = 0x80000000, LENGTH = 2M
    
    /* Kernel heap */
    HEAP (rw) : ORIGIN = 0x80200000, LENGTH = 62M
    
    /* User space */
    USER (rw) : ORIGIN = 0x84000000, LENGTH = 64M
}

SECTIONS
{
    /* Boot code section */
    .text.boot : {
        *(.text.boot)
    } > KERNEL
    
    /* Kernel code section */
    .text : {
        *(.text .text.*)
    } > KERNEL
    
    /* Read-only data section */
    .rodata : {
        *(.rodata .rodata.*)
    } > KERNEL
    
    /* Data section */
    .data : {
        *(.data .data.*)
    } > KERNEL
    
    /* BSS section */
    .bss : {
        _bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        _bss_end = .;
    } > KERNEL
    
    /* Stack section */
    .stack (NOLOAD) : {
        _stack_bottom = .;
        . = . + 16K;
        _stack_top = .;
    } > KERNEL
    
    /* Heap section */
    .heap (NOLOAD) : {
        _heap_start = .;
        . = . + 62M;
        _heap_end = .;
    } > HEAP
    
    /* Discard sections */
    /DISCARD/ : {
        *(.comment)
        *(.note.*)
        *(.eh_frame*)
    }
}
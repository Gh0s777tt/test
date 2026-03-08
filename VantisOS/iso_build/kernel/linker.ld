/* Linker script for VantisOS kernel */

ENTRY(_start)

SECTIONS {
    /* Start at 1 MB */
    . = 1M;
    
    /* Multiboot header must be early in the file */
    .multiboot ALIGN(4K) :
    {
        *(.multiboot)
    }
    
    /* Text section */
    .text ALIGN(4K) :
    {
        *(.text)
        *(.text.*)
    }
    
    /* Read-only data */
    .rodata ALIGN(4K) :
    {
        *(.rodata)
        *(.rodata.*)
    }
    
    /* Read-write data */
    .data ALIGN(4K) :
    {
        *(.data)
        *(.data.*)
    }
    
    /* BSS (uninitialized data) */
    .bss ALIGN(4K) :
    {
        *(COMMON)
        *(.bss)
        *(.bss.*)
    }
    
    /* Discard unwanted sections */
    /DISCARD/ :
    {
        *(.comment)
        *(.note.*)
        *(.eh_frame)
    }
}
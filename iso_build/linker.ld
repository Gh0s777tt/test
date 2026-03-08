/* VantisOS Kernel Linker Script */
/* Links kernel at 1MB for multiboot compatibility */

ENTRY(_start)

SECTIONS
{
    /* Start at 1MB - standard for bootloaders */
    . = 1M;
    
    /* Multiboot header must be first */
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
    
    /* Initialized data */
    .data ALIGN(4K) :
    {
        *(.data)
        *(.data.*)
    }
    
    /* Uninitialized data (BSS) */
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
        *(.note*)
        *(.eh_frame*)
    }
}
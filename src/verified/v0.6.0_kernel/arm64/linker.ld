OUTPUT_FORMAT(elf64-littleaarch64)
OUTPUT_ARCH(aarch64)

ENTRY(arm64_kernel_entry)

SECTIONS
{
    . = 0x40000000;
    
    .text : {
        *(.text.boot)
        *(.text*)
    }
    
    .rodata : {
        *(.rodata*)
    }
    
    .data : {
        *(.data*)
    }
    
    .bss : {
        *(COMMON)
        *(.bss*)
    }
    
    .stack (NOLOAD) : {
        . = ALIGN(16384);
        __stack_start = .;
        . = . + 8388608;
        __stack_end = .;
    }
    
    .heap (NOLOAD) : {
        __heap_start = .;
        . = . + 16777216;
        __heap_end = .;
    }
}

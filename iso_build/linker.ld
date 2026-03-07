ENTRY(_start)

SECTIONS
{
    . = 1M;
    
    .multiboot ALIGN(4K) :
    {
        *(.multiboot)
    }
    
    .text ALIGN(4K) :
    {
        *(.text)
    }
    
    .rodata ALIGN(4K) :
    {
        *(.rodata)
    }
    
    .bss ALIGN(4K) :
    {
        *(.bss)
    }
}

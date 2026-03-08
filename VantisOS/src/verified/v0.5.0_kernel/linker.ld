ENTRY(_start)

SECTIONS
{
    /* Place multiboot header at the very beginning */
    . = 0;
    .multiboot : {
        *(.multiboot)
    }

    /* Start kernel at 1MB */
    . = 1M;

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(COMMON)
        *(.bss)
    }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.gnu.build-id)
    }
}
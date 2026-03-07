; Minimal Multiboot kernel
BITS 32

section .multiboot
align 4
    dd 0x1BADB002          ; Magic
    dd 0x00000003          ; Flags
    dd -(0x1BADB002 + 3)   ; Checksum

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:

section .text
global _start
extern kernel_main

_start:
    mov esp, stack_top
    push ebx
    call kernel_main
.hang:
    cli
    hlt
    jmp .hang

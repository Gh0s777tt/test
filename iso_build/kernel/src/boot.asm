; Multiboot header for VantisOS
; Compatible with GRUB bootloader

section .multiboot
align 4
    dd 0x1BADB002           ; Magic number
    dd 0x00000003           ; Flags (page align, memory info)
    dd -(0x1BADB002 + 3)    ; Checksum

section .bss
align 16
stack_bottom:
    resb 65536              ; 64 KB stack
stack_top:

section .text
global _start
extern kernel_main

_start:
    ; Set up stack
    mov esp, stack_top
    
    ; Save multiboot info pointer
    push ebx
    
    ; Clear direction flag
    cld
    
    ; Call kernel (will be replaced by Rust _start)
    call kernel_main
    
    ; If kernel returns, hang
.hang:
    cli
    hlt
    jmp .hang
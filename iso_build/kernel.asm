; VantisOS Kernel Entry Point
; Multiboot compliant kernel header

BITS 32

; Multiboot header constants
MBOOT_PAGE_ALIGN    equ 1 << 0
MBOOT_MEM_INFO      equ 1 << 1
MBOOT_MAGIC         equ 0x1BADB002
MBOOT_FLAGS         equ MBOOT_PAGE_ALIGN | MBOOT_MEM_INFO
MBOOT_CHECKSUM      equ -(MBOOT_MAGIC + MBOOT_FLAGS)

; Multiboot header - must be in first 8KB of kernel
section .multiboot
align 4
    dd MBOOT_MAGIC
    dd MBOOT_FLAGS
    dd MBOOT_CHECKSUM

; Stack
section .bss
align 16
stack_bottom:
    resb 16384      ; 16 KB stack
stack_top:

; Entry point
section .text
global _start
extern kernel_main

_start:
    ; Disable interrupts
    cli
    
    ; Set up stack
    mov esp, stack_top
    
    ; Save multiboot info
    push ebx        ; Multiboot info pointer
    push eax        ; Multiboot magic
    
    ; Clear direction flag
    cld
    
    ; Check for multiboot
    cmp eax, 0x2BADB002
    jne .no_multiboot
    
    ; Call kernel main (if available)
    ; For pure assembly kernel, we do everything here
    ; call kernel_main
    
    ; Initialize VGA
    call vga_init
    
    ; Print welcome banner
    call print_banner
    
    ; Main loop
.main_loop:
    hlt
    jmp .main_loop
    
.no_multiboot:
    ; Print error message
    mov dword [0xB8000], 0x4F524F45  ; "ER"
    mov dword [0xB8004], 0x4F3A4F52  ; "R:"
    mov dword [0xB8008], 0x4F424F4D  ; "MB"
    mov dword [0xB800C], 0x4F544F4F  ; "OT"
    jmp .main_loop

; Initialize VGA
vga_init:
    ; Set VGA mode (text mode 80x25)
    mov ax, 0x0003
    int 0x10
    
    ; Set cursor position to 0,0
    mov dx, 0x3D4
    mov al, 0x0E
    out dx, al
    inc dx
    mov al, 0x00
    out dx, al
    
    dec dx
    mov al, 0x0F
    out dx, al
    inc dx
    out dx, al
    
    ret

; Print welcome banner
print_banner:
    ; Print to VGA memory at 0xB8000
    mov edi, 0xB8000
    
    ; Set color (light cyan on blue)
    mov ah, 0x1F
    
    ; Print top border
    mov al, 0xDA
    mov [edi], ax
    add edi, 2
    mov ecx, 78
.top_border:
    mov al, 0xC4
    mov [edi], ax
    add edi, 2
    loop .top_border
    mov al, 0xBF
    mov [edi], ax
    add edi, 2
    
    ; Next line
    add edi, 2
    
    ; Print empty line with borders
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    mov ecx, 78
    mov al, ' '
.empty_line:
    mov [edi], ax
    add edi, 2
    loop .empty_line
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    
    ; Next line
    add edi, 2
    
    ; Print title line
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    
    ; Center the title
    mov ecx, 20
.space_before:
    mov al, ' '
    mov [edi], ax
    add edi, 2
    loop .space_before
    
    ; Print title
    mov esi, title_str
.print_title:
    lodsb
    test al, al
    jz .title_done
    mov [edi], ax
    add edi, 2
    jmp .print_title
.title_done:
    
    ; Fill rest of line
    mov ecx, 32
.space_after:
    mov al, ' '
    mov [edi], ax
    add edi, 2
    loop .space_after
    
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    
    ; Next line
    add edi, 2
    
    ; Print subtitle line
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    
    mov ecx, 14
.space_before2:
    mov al, ' '
    mov [edi], ax
    add edi, 2
    loop .space_before2
    
    mov esi, subtitle_str
.print_subtitle:
    lodsb
    test al, al
    jz .subtitle_done
    mov [edi], ax
    add edi, 2
    jmp .print_subtitle
.subtitle_done:
    
    mov ecx, 18
.space_after2:
    mov al, ' '
    mov [edi], ax
    add edi, 2
    loop .space_after2
    
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    
    ; Print empty lines
    mov ecx, 3
.empty_lines:
    push ecx
    add edi, 2
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    mov ecx, 78
    mov al, ' '
.fill_empty:
    mov [edi], ax
    add edi, 2
    loop .fill_empty
    mov al, 0xB3
    mov [edi], ax
    pop ecx
    loop .empty_lines
    
    ; Print success message
    add edi, 2
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    
    mov ecx, 10
.space_before3:
    mov al, ' '
    mov [edi], ax
    add edi, 2
    loop .space_before3
    
    ; Set green color
    mov ah, 0x0A
    mov al, 0xFE    ; checkmark
    mov [edi], ax
    add edi, 2
    
    mov ah, 0x1F
    mov esi, success_str
.print_success:
    lodsb
    test al, al
    jz .success_done
    mov [edi], ax
    add edi, 2
    jmp .print_success
.success_done:
    
    ; Fill rest
    mov ecx, 35
.space_after3:
    mov al, ' '
    mov [edi], ax
    add edi, 2
    loop .space_after3
    
    mov al, 0xB3
    mov [edi], ax
    add edi, 2
    
    ; Bottom border
    add edi, 2
    mov al, 0xC0
    mov [edi], ax
    add edi, 2
    mov ecx, 78
.bottom_border:
    mov al, 0xC4
    mov [edi], ax
    add edi, 2
    loop .bottom_border
    mov al, 0xD9
    mov [edi], ax
    
    ret

; Strings
title_str:      db "VantisOS v1.5.0 'Quantum Ready'", 0
subtitle_str:   db "Quantum-Ready Operating System", 0
success_str:    db " Kernel initialized successfully!", 0

; GDT (Global Descriptor Table)
section .rodata
gdt_start:
    ; Null descriptor
    dq 0
    
gdt_code:
    ; Code segment descriptor (selector = 0x08)
    ; Base = 0, Limit = 0xFFFFF
    ; Access: Present, Ring 0, Code, Executable, Readable
    dw 0xFFFF       ; Limit (bits 0-15)
    dw 0x0000       ; Base (bits 0-15)
    db 0x00         ; Base (bits 16-23)
    db 10011010b    ; Access byte
    db 11001111b    ; Flags + Limit (bits 16-19)
    db 0x00         ; Base (bits 24-31)
    
gdt_data:
    ; Data segment descriptor (selector = 0x10)
    dw 0xFFFF       ; Limit (bits 0-15)
    dw 0x0000       ; Base (bits 0-15)
    db 0x00         ; Base (bits 16-23)
    db 10010010b    ; Access byte
    db 11001111b    ; Flags + Limit (bits 16-19)
    db 0x00         ; Base (bits 24-31)
    
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1   ; Size
    dd gdt_start                  ; Address

; Code and data selectors
CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start
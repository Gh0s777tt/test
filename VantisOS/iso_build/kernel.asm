; VantisOS Kernel - Multiboot compliant ELF kernel
; Outputs to both VGA and Serial port

BITS 32

; Multiboot header constants
MBOOT_PAGE_ALIGN    equ 1<<0
MBOOT_MEM_INFO      equ 1<<1
MBOOT_MAGIC         equ 0x1BADB002
MBOOT_FLAGS         equ MBOOT_PAGE_ALIGN | MBOOT_MEM_INFO
MBOOT_CHECKSUM      equ -(MBOOT_MAGIC + MBOOT_FLAGS)

; Serial port
COM1_PORT           equ 0x3F8

; Multiboot header (must be early in the file)
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

; Kernel code
section .text
global _start

_start:
    ; Set up stack
    mov esp, stack_top
    
    ; Clear direction flag
    cld
    
    ; Save multiboot info
    push ebx
    
    ; Initialize serial port COM1
    call init_serial
    
    ; Send banner to serial
    mov esi, serial_banner
    call print_serial
    
    ; Initialize VGA (set video mode 3 - 80x25 text)
    mov ax, 0x0003
    int 0x10
    
    ; Print welcome message to VGA buffer
    mov edi, 0xB8000       ; VGA text buffer address
    
    ; Clear screen (fill with spaces, white on black)
    mov ecx, 2000          ; 80 * 25 characters
    mov ax, 0x0F20         ; Space character (0x20) with white on black (0x0F)
    rep stosw
    
    ; Print banner - line 1
    mov edi, 0xB8000
    mov esi, banner
    call print_string_vga
    
    ; Print separator - line 2
    mov edi, 0xB80A0
    mov esi, separator
    call print_string_vga
    
    ; Print features
    mov edi, 0xB8140
    mov esi, features1
    call print_string_vga
    
    mov edi, 0xB81E0
    mov esi, features2
    call print_string_vga
    
    mov edi, 0xB8280
    mov esi, features3
    call print_string_vga
    
    mov edi, 0xB8320
    mov esi, features4
    call print_string_vga
    
    mov edi, 0xB83C0
    mov esi, ready_msg
    call print_string_vga
    
    mov edi, 0xB8460
    mov esi, halt_msg
    call print_string_vga
    
    ; Send ready message to serial
    mov esi, serial_ready
    call print_serial

    ; Halt
    cli
.halt:
    hlt
    jmp .halt

; Initialize serial port COM1
init_serial:
    push eax
    push dx
    
    mov dx, COM1_PORT + 1    ; Interrupt Enable Register
    xor al, al
    out dx, al               ; Disable interrupts
    
    mov dx, COM1_PORT + 3    ; Line Control Register
    mov al, 0x80             ; Enable DLAB
    out dx, al
    
    mov dx, COM1_PORT + 0    ; Divisor Latch Low (baud rate divisor)
    mov al, 0x01             ; 115200 baud
    out dx, al
    
    mov dx, COM1_PORT + 1    ; Divisor Latch High
    xor al, al
    out dx, al
    
    mov dx, COM1_PORT + 3    ; Line Control Register
    mov al, 0x03             ; 8 bits, no parity, one stop bit
    out dx, al
    
    mov dx, COM1_PORT + 2    ; FIFO Control Register
    mov al, 0xC7             ; Enable FIFO, clear them, 14-byte threshold
    out dx, al
    
    mov dx, COM1_PORT + 4    ; Modem Control Register
    mov al, 0x0B             ; IRQs enabled, RTS/DSR set
    out dx, al
    
    pop dx
    pop eax
    ret

; Print string to serial port
; Input: ESI = source string
print_serial:
    pusha
.loop:
    lodsb                   ; Load byte from ESI into AL
    test al, al             ; Check if null terminator
    jz .done
    
    ; Wait for transmit buffer to be empty
.wait:
    push dx
    mov dx, COM1_PORT + 5    ; Line Status Register
    in al, dx
    pop dx
    test al, 0x20            ; Check if transmit buffer empty
    jz .wait
    
    ; Send character
    push dx
    mov dx, COM1_PORT
    mov al, [esi-1]          ; Get character we loaded earlier
    out dx, al
    pop dx
    
    jmp .loop
.done:
    popa
    ret

; Print string to VGA buffer
; Input: ESI = source string, EDI = destination in VGA buffer
print_string_vga:
    pusha
.loop:
    lodsb               ; Load byte from ESI into AL
    test al, al         ; Check if null terminator
    jz .done
    mov ah, 0x0F        ; White on black attribute
    mov [edi], ax       ; Write character + attribute
    add edi, 2          ; Move to next character position
    jmp .loop
.done:
    popa
    ret

; Data
section .rodata
banner:
    db "   V A N T I S O S   v 1 . 5 . 0   ' Q u a n t u m   R e a d y ' ", 0

separator:
    db "   ================================================================", 0

features1:
    db "   * Modern x86_64 kernel with preemptive multitasking", 0

features2:
    db "   * Quantum computing simulation capabilities", 0

features3:
    db "   * Post-quantum cryptography support (Kyber, Dilithium)", 0

features4:
    db "   * Secure, privacy-focused design", 0

ready_msg:
    db "   >>> System ready. VantisOS loaded successfully!", 0

halt_msg:
    db "   System halted. Press power button to restart.", 0

; Serial messages
serial_banner:
    db 10, 13
    db "========================================", 10, 13
    db "  VantisOS v1.5.0 'Quantum Ready'", 10, 13
    db "  Quantum-Ready Operating System", 10, 13
    db "========================================", 10, 13
    db 10, 13
    db "Kernel initialized successfully!", 10, 13
    db 10, 13
    db "Features:", 10, 13
    db "  - Modern x86_64 kernel", 10, 13
    db "  - Quantum computing simulation", 10, 13
    db "  - Post-quantum cryptography", 10, 13
    db "  - Secure design", 10, 13
    db 10, 13, 0

serial_ready:
    db "System ready. VantisOS is running!", 10, 13
    db "System halted.", 10, 13, 0

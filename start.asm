section .text
global start, _start

start:
_start:
    jmp multiboot_entry

align 8
section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number
    dd 0                         ; architektur protected mode code
    dd header_end - header_start ; header length

    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
section .text
multiboot_entry:
    mov eax,0xb8000
l1:
    mov [eax],al
    inc eax
    jmp l1

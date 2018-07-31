	extern _start

global start

section .text
bits 32
start:
    mov esp, stack_top
    
    cld
    xor eax,eax
    mov	edi,0x1000 ; Direct Mapped PML4 auf 0x1000
    mov	ecx, 2*1024; 2*1024*4 = 8 kbyte auf 0 setzen
    rep stosd
    mov	eax,0x2013 ; PDP@ 0x2000 + PageCacheDisabled + Writeable + Present
    mov [0x1000],eax
    mov eax, 0x1013
    mov [0x1FF8],eax; Selbsreferenzierung

	mov	ecx, 512; 512 Einträge
	mov eax, 0x193 ; Global + 1 + PageCacheDisabled + WriteThrough + Present
	xor	edx,edx
	mov	edi,0x2000
fillPDP:
	mov	[edi],eax
	mov	[edi+4],edx
	add	edi,8
	add	eax,0x40000000
	adc edx,0
    loop fillPDP
    
    mov edi, ebx       ; move Multiboot info pointer to edi
    
    ; move page table address to cr3
    mov eax, 0x1000
    mov cr3, eax

    ; enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31
    or eax, 1 << 16
    mov cr0, eax

    ; load the gdt
    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax
	jmp gdt64.code:longmodeActive

bits 64
longmodeActive:


	mov	rcx, APStartblock_End
	mov	rbx, resetVector
	mov	rdx,0
APStartcodeCopy:
	mov	al, byte [rbx]
	
	mov byte [rdx],al
	inc rdx
	inc rbx
	cmp rcx,rbx
	jne APStartcodeCopy

    ; jump to long mode!
    jmp _start

    mov word [0xb8000], 0x0248 ; H
    mov word [0xb8002], 0x0265 ; e
    mov word [0xb8004], 0x026c ; l
    mov word [0xb8006], 0x026c ; l
    mov word [0xb8008], 0x026f ; o
    mov word [0xb800a], 0x022c ; ,
    mov word [0xb800c], 0x0220 ;
    mov word [0xb800e], 0x0277 ; w
    mov word [0xb8010], 0x026f ; o
    mov word [0xb8012], 0x0272 ; r
    mov word [0xb8014], 0x026c ; l
    mov word [0xb8016], 0x0264 ; d
    mov word [0xb8018], 0x0221 ; !
    mov word [0xb801a], 0x0221 ; !
    hlt

section .bss ;block started by symbol - entries automatically set to zero by linker

align 4096 ;makes sure that adresses are aligned correct

p4_table: 
    resb 4096 ;directive to reservate bytes
p3_table:
    resb 4096 ;directive to reservate bytes
p2_table:
    resb 4096 ;directive to reservate bytes
stack_bottom:
    resb 4096 * 4
stack_top:

section .rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)
.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64

resetVector:
incbin "src/asm/APReset"
APStartblock_End:


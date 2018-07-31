org 0

bits 16
resetVector:

	mov	ax,0
	mov	ss,ax
	mov ds,ax
	mov es,ax
	lgdt [0xFB0] ; GDTPTR
	mov eax,0x0011
	mov	cr0,eax
	jmp 0x18:protectModeVector
bits 32
protectModeVector:
	mov ax,0x20
	mov ds,ax
	mov es,ax
	mov ss,ax
	
	mov eax,0x1000
	mov	cr3,eax
	
	mov	eax, cr4
	bts	eax,5
	mov	cr4,eax
	
	mov	ecx,0xc0000080
	rdmsr
	bts	eax,8
	wrmsr
	
	mov	eax,cr0
	bts	eax,31
	bts	eax,16
	mov	cr0,eax
	
	lgdt [0xFB0]

    ; update selectors
    
	mov eax, 0x10
    mov ss, ax
    mov ds, ax
    mov es, ax
	
	jmp 0x08:longmodeVector
bits 64 
longmodeVector:
	;Stackpointer laden perXCHG. RAX (0) wird mit DEST vertauscht. Danach testen
	xor rax,rax
	lock xchg rax,[0xCF0]
	cmp	rax,0
	je	longmodeVector
	mov	rsp,rax
	mov	rax,[0xCF8]
	jmp rax
APStartblock_End:
	

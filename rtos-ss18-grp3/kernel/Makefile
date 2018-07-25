default: build

build: target/kernel.bin

.PHONY: clean cargo

debug: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso -monitor telnet:127.0.0.1:1235,server,nowait -s -S -device isa-debug-exit,iobase=0xf4,iosize=0x04 -D qemu.loig&
	objdump -d target/kernel.bin > kernelDisasm
	(echo "disconnect") | gdb --init-command ./gdbscript1
	gdb --init-command ./gdbscript2

cargo:
	cargo xbuild --target x86_64-blog_os.json

run: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso -device isa-debug-exit,iobase=0xf4,iosize=0x04

target/multiboot_header.o: src/asm/multiboot_header.asm
	mkdir -p target
	nasm -f elf64 src/asm/multiboot_header.asm -o target/multiboot_header.o

target/boot.o: src/asm/boot.asm
	mkdir -p target
	nasm -f elf64 src/asm/boot.asm -o target/boot.o

target/kernel.bin: target/multiboot_header.o target/boot.o src/asm/linker.ld cargo
	$(LD) --nmagic --gc-sections --output=target/kernel.bin --script=src/asm/linker.ld target/multiboot_header.o target/boot.o target/x86_64-blog_os/debug/libkernel.a

target/os.iso: target/kernel.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot/
	grub-mkrescue -o target/os.iso target/isofiles

clean:
	cargo clean

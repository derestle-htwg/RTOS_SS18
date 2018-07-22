UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    LD = ld
    MKRESCUE = grub-mkrescue
endif
ifeq ($(UNAME_S),Darwin)
    LD = /Users/chris/opt/bin/x86_64-pc-elf-ld
    MKRESCUE = /Users/chris/opt/bin/grub-mkrescue
endif

default: build

build: target/kernel.bin

.PHONY: clean cargo

cargo: 
	xargo build --release --target x86_64-unknown-intermezzos-gnu

run: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso

target/multiboot_header.o: src/asm/multiboot_header.asm
	mkdir -p target
	nasm -f elf64 src/asm/multiboot_header.asm -o target/multiboot_header.o

target/boot.o: src/asm/boot.asm
	mkdir -p target
	nasm -f elf64 src/asm/boot.asm -o target/boot.o

target/kernel.bin: target/multiboot_header.o target/boot.o src/asm/linker.ld cargo
	$(LD) --nmagic --gc-sections --output=target/kernel.bin --script=src/asm/linker.ld target/multiboot_header.o target/boot.o target/x86_64-unknown-intermezzos-gnu/release/librust_kernel.a

target/os.iso: target/kernel.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot/
	$(MKRESCUE) -o target/os.iso target/isofiles

clean:
	cargo clean

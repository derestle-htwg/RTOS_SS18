all:
	bootimage build

run:	all
	qemu-system-x86_64 bootimage.bin -monitor telnet:127.0.0.1:1234,server,nowait

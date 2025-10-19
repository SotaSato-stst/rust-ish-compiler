exec_asm:
	nasm -f macho64 misc/hello.asm -o hello.o
	cc -arch x86_64 hello.o -o hello -Wl
	arch -x86_64 ./hello
	rm hello hello.o
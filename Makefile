exec_asm:
	nasm -f macho64 misc/hello.asm -o hello.o
	cc -arch x86_64 hello.o -o hello -Wl
	arch -x86_64 ./hello
	rm hello hello.o

exec_output:
	nasm -f macho64 misc/output.asm -o output.o
	cc -arch x86_64 output.o -o output -Wl
	arch -x86_64 ./output > output.txt
	cat output.txt
	rm output output.o output.txt
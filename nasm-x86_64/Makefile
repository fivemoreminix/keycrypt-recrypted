NASM_FLAGS=-felf64
SOURCES=main.s

run: build
	./keycrypt

build:
	nasm $(NASM_FLAGS) -o keycrypt.o $(SOURCES) && gcc -o keycrypt keycrypt.o

clean:
	rm keycrypt keycrypt.o

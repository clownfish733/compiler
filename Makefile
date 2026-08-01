all: run

main.o: main.s
	as main.s -o main.o

prog: main.o
	gcc -o main main.o -nostdlib -static

run: prog
	@./main; echo "exit code ", $$?
.PHONY: all run

clean:
	rm main.o main.s

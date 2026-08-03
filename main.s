.intel_syntax noprefix
	.set SYS_WRITE, 1
	.set SYS_EXIT, 60
	.set STDOUT, 1
	.set ZERO, 48
	.section .text
	.global _start
print:
	push rbp
	mov rbp, rsp
	sub rsp, 32
	mov r9, -1
	mov byte ptr [rbp - 1], 10
	mov eax, edi
.Lprint_loop:
	xor edx, edx
	mov ecx, 10
	div ecx
	dec r9
	add dl, ZERO
	mov byte ptr [rbp + r9], dl
	test eax, eax
	jnz .Lprint_loop
Lprint_end:
	lea rsi, [rbp + r9]
	neg r9
	mov rdx, r9
	mov rdi, STDOUT
	mov rax, SYS_WRITE
	syscall
	leave
	ret
fib:
	push rbp
	mov rbp, rsp
	sub rsp, 80
	mov dword ptr [rbp - 4], edi
	mov eax, dword ptr [rbp - 4]
	mov dword ptr [rbp - 8], eax
	mov dword ptr [rbp - 12], 0
	mov eax, dword ptr [rbp - 8]
	mov ebx, dword ptr [rbp - 12]
	mov ecx, eax
	xor eax, eax
	cmp ecx, ebx
	sete al
	mov dword ptr [rbp - 8], eax
	mov eax, dword ptr [rbp - 4]
	mov dword ptr [rbp - 12], eax
	mov dword ptr [rbp - 16], 1
	mov eax, dword ptr [rbp - 12]
	mov ebx, dword ptr [rbp - 16]
	mov ecx, eax
	xor eax, eax
	cmp ecx, ebx
	sete al
	mov dword ptr [rbp - 12], eax
	mov eax, dword ptr [rbp - 8]
	mov ebx, dword ptr [rbp - 12]
	or eax, ebx
	mov dword ptr [rbp - 8], eax
	cmp dword ptr [rbp - 8], 0
	jz .Lfib_Else0
	mov eax, dword ptr [rbp - 4]
	mov dword ptr [rbp - 8], eax
	mov eax, dword ptr [rbp - 8]
	jmp .Lfib_If_End0
.Lfib_Else0:
	mov eax, dword ptr [rbp - 4]
	mov dword ptr [rbp - 8], eax
	mov dword ptr [rbp - 12], 1
	mov eax, dword ptr [rbp - 8]
	mov ebx, dword ptr [rbp - 12]
	sub eax, ebx
	mov dword ptr [rbp - 8], eax
	mov edi, dword ptr [rbp - 8]
	call fib
	mov dword ptr [rbp - 8], eax
	mov eax, dword ptr [rbp - 4]
	mov dword ptr [rbp - 12], eax
	mov dword ptr [rbp - 16], 2
	mov eax, dword ptr [rbp - 12]
	mov ebx, dword ptr [rbp - 16]
	sub eax, ebx
	mov dword ptr [rbp - 12], eax
	mov edi, dword ptr [rbp - 12]
	call fib
	mov dword ptr [rbp - 12], eax
	mov eax, dword ptr [rbp - 8]
	mov ebx, dword ptr [rbp - 12]
	add eax, ebx
	mov dword ptr [rbp - 8], eax
	mov eax, dword ptr [rbp - 8]
.Lfib_If_End0:
	leave
	ret
main:
	push rbp
	mov rbp, rsp
	sub rsp, 96
	mov dword ptr [rbp - 4], 10
	mov edi, dword ptr [rbp - 4]
	call fib
	mov dword ptr [rbp - 4], eax
	mov dword ptr [rbp - 8], 10
	mov dword ptr [rbp - 12], 1
.Lmain_While_Loop0:
	mov eax, dword ptr [rbp - 8]
	mov dword ptr [rbp - 16], eax
	mov dword ptr [rbp - 20], 0
	mov eax, dword ptr [rbp - 16]
	mov ebx, dword ptr [rbp - 20]
	mov ecx, eax
	xor eax, eax
	cmp ecx, ebx
	setne al
	mov dword ptr [rbp - 16], eax
	cmp dword ptr [rbp - 16], 0
	jz .Lmain_While_End0
	mov eax, dword ptr [rbp - 12]
	mov dword ptr [rbp - 16], eax
	mov dword ptr [rbp - 20], 2
	mov eax, dword ptr [rbp - 16]
	mov ebx, dword ptr [rbp - 20]
	mul ebx
	mov dword ptr [rbp - 16], eax
	mov eax, dword ptr [rbp - 16]
	mov dword ptr [rbp - 12], eax
	mov eax, dword ptr [rbp - 8]
	mov dword ptr [rbp - 16], eax
	dec dword ptr [rbp - 16]
	mov eax, dword ptr [rbp - 16]
	mov dword ptr [rbp - 8], eax
	jmp .Lmain_While_Loop0
.Lmain_While_End0:
	mov dword ptr [rbp - 16], 10
	mov dword ptr [rbp - 20], 4
	mov dword ptr [rbp - 24], 8
	mov dword ptr [rbp - 28], 2
	mov eax, dword ptr [rbp - 24]
	mov ebx, dword ptr [rbp - 28]
	sub eax, ebx
	mov dword ptr [rbp - 24], eax
	mov edi, dword ptr [rbp - 24]
	call fib
	mov dword ptr [rbp - 24], eax
	mov dword ptr [rbp - 28], 3
	mov eax, dword ptr [rbp - 24]
	mov ebx, dword ptr [rbp - 28]
	add eax, ebx
	mov dword ptr [rbp - 24], eax
	mov eax, dword ptr [rbp - 20]
	mov ebx, dword ptr [rbp - 24]
	mul ebx
	mov dword ptr [rbp - 20], eax
	mov eax, dword ptr [rbp - 16]
	mov ebx, dword ptr [rbp - 20]
	add eax, ebx
	mov dword ptr [rbp - 16], eax
	mov eax, dword ptr [rbp - 4]
	mov dword ptr [rbp - 20], eax
	mov eax, dword ptr [rbp - 12]
	mov dword ptr [rbp - 24], eax
	mov eax, dword ptr [rbp - 16]
	mov dword ptr [rbp - 28], eax
	mov eax, dword ptr [rbp - 24]
	mov ebx, dword ptr [rbp - 28]
	add eax, ebx
	mov dword ptr [rbp - 24], eax
	mov eax, dword ptr [rbp - 20]
	mov ebx, dword ptr [rbp - 24]
	add eax, ebx
	mov dword ptr [rbp - 20], eax
	mov edi, dword ptr [rbp - 20]
	call print
	mov dword ptr [rbp - 20], 3
	mov eax, dword ptr [rbp - 20]
	mov dword ptr [rbp - 12], eax
	mov eax, dword ptr [rbp - 12]
	mov dword ptr [rbp - 20], eax
	mov edi, dword ptr [rbp - 20]
	call print
	leave
	ret
_start:
	push rbp
	mov rbp, rsp
	call main
	mov rdi, 8
	mov rax, SYS_EXIT
	syscall
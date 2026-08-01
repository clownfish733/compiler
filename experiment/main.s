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
.Lprint_loop:   
    xor edx, edx
    mov ecx, 10
    div ecx
    dec r9
    add dl, ZERO
    mov byte ptr [rbp + r9], dl
    test eax, eax
    jnz .Lprint_loop
.Lprint_end:
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
    sub rsp, 32
    mov dword ptr [rbp - 4], eax
    cmp eax, 0
    je .Lfib_if_case
    cmp eax, 1
    je .Lfib_if_case
    jmp .Lfib_else_case

.Lfib_if_case:
    leave
    ret

.Lfib_else_case:
   sub eax, 1
   call fib
   mov dword ptr [rbp - 8], eax
   mov eax, dword ptr [rbp - 4]
   sub eax, 2
   call fib
   add eax, dword ptr [rbp - 8]
   leave
   ret

main:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov eax, 4
    call fib
    mov dword ptr [rbp - 4], eax
    mov dword ptr [rbp - 8], 1
    mov dword ptr [rbp - 12], 10

.Lmain_loop:
    cmp dword ptr [rbp - 12], 0
    je .Lmain_end
    mov eax, dword ptr [rbp - 8]
    imul eax, 2
    mov dword ptr [rbp - 8], eax
    dec dword ptr [rbp - 12]
    jmp .Lmain_loop

.Lmain_end:
    mov dword ptr [rbp - 16], 7
    mov eax, dword ptr [rbp - 16]
    mov edx, dword ptr [rbp - 8]
    imul eax, edx
    add eax, 2
    mov dword ptr [rbp - 16], eax
    mov eax, dword ptr [rbp - 4]
    add eax, dword ptr [rbp - 8]
    add eax, dword ptr [rbp - 16]
    call print
    leave 
    ret

_start:
    push rbp
    mov rbp, rsp
    call main
    mov rdi, 0
    mov rax, SYS_EXIT
    syscall


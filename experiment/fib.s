.intel_syntax noprefix

    .set SYS_WRITE, 1
    .set SYS_EXIT, 60
    .set STDOUT, 1

    .section .text
    .global _start

fib:
    push rbp
    mov rbp, rsp
    sub rsp, 16

    mov rax, [rbp+16]
    cmp rax, 0
    je .Lfib_base
    cmp rax, 1
    je .Lfib_base
    mov rax, [rbp+16]
    cmp rax, 1
    cmp rax, 1
    je .Lfib_base
    jmp .Lfib_rec

.Lfib_base:
    mov  rax, [rbp + 16]
    mov [rbp+24], rax
    jmp .Lfib_end

.Lfib_rec:
    sub rsp, 8
    mov rax, [rbp+16]
    sub rax, 1
    push rax
    call fib
    add rsp, 8
    pop rax
    mov [rbp-8], rax

    sub rsp, 8
    mov rax, [rbp+16]
    sub rax, 2
    push rax
    call fib
    add rsp, 8
    pop rax
    mov [rbp-16], rax

    mov rax, [rbp-8]
    add rax, [rbp-16]
    mov [rbp+24], rax

.Lfib_end:
    mov rsp, rbp
    pop rbp
    ret


println_usize:
    push rbp
    mov rbp, rsp
    sub rsp, 48

    lea rax, [rbp-32]
    add rax, 32
    mov [rbp-40], rax

    mov rax, [rbp-40]
    sub rax, 1
    mov [rbp-40], rax
    mov byte ptr [rax], 10

.Lpr_loop:
    mov rax, [rbp+16]
    xor rdx, rdx
    mov rcx, 10
    div rcx

    mov [rbp+16], rax
    add rdx, 48
    mov rax, [rbp-40]
    sub rax, 1
    mov [rbp-40], rax
    mov [rax], dl
    cmp qword ptr [rbp+16], 0
    jne .Lpr_loop

    lea rax, [rbp-32]
    add rax, 32
    sub rax, [rbp-40]
    mov rdx, rax
    mov rsi, [rbp-40]
    mov rdi, STDOUT
    mov rax, SYS_WRITE
    syscall

    mov rsp, rbp
    pop rbp
    ret

_start:
    sub rsp, 8
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov qword ptr [rbp-8], 10

    sub rsp, 8
    mov rax, [rbp-8]
    push rax
    call fib
    add rsp, 8
    pop rax

    push rax
    call println_usize
    add rsp, 8
    mov rdi, 0
    mov rax, SYS_EXIT
    syscall









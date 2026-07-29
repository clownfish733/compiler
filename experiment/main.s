
    .text
    .global _start
_start:
    mov edi, 4
    call fib
    mov edi, eax
    mov eax, 60
    syscall

fib:
    cmp edi, 1
    jle .Lbase
    push rbx
    push rbp
    sub rsp, 8
    mov ebx, edi

    lea edi, [rbx-1]
    call fib
    mov ebp, eax

    lea edi, [rbx-2]
    call fib

    add eax, ebp
    
    add rsp, 8
    pop rbp
    pop rbx
    ret

.Lbase:
    mov eax, 1
    ret

    .size fib, .-fib



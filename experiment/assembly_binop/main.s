.intel_syntax noprefix
    .set SYS_WRITE, 1
    .set STDOUT, 1
    .set SYS_EXIT, 60
    .set ZERO, 48

.global .text
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
.Lprint_end:
    lea rsi, [rbp + r9]
    neg r9
    mov rdx, r9
    mov rdi, STDOUT
    mov rax, SYS_WRITE
    syscall 
    leave
    ret
    

_start:
    push rbp
    mov rbp, rsp
    sub rsp, 32
#+
    mov eax, 10
    mov ebx, 11
    add eax, ebx
    mov edi, eax
    call print
#-
    mov eax, 7
    mov ebx, 4
    sub eax, ebx
    mov edi, eax
    call print
#*
    mov eax, 7
    mov ebx, 4
    mul ebx
    mov edi, eax
    call print

#/
    xor edx, edx
    mov ebx, 3
    mov eax, 11
    div ebx
    mov edi, eax
    call print
#%
    xor edx, edx
    mov ebx, 3
    mov eax, 11
    div ebx
    mov eax, edx
    mov edi, eax
    call print
#|
    mov eax, 3
    mov ebx, 11
    or eax, ebx
    mov edi, eax
    call print
#&
    mov eax, 3
    mov ebx, 11
    and eax, ebx
    mov edi, eax
    call print
# ==
    mov eax, 12
    mov ebx, 10
    mov ecx, eax
    xor eax, eax
    cmp ebx, ecx
    sete al
    mov edi, eax
    call print

# != 
    mov eax, 12
    mov ebx, 10
    mov ecx, eax
    xor eax, eax
    cmp ebx, ecx
    setne al
    mov edi, eax
    call print

# <
    mov eax, 12
    mov ebx, 10
    mov ecx, eax
    xor eax, eax
    cmp ecx, ebx
    setb al
    mov edi, eax
    call print
# <= 
    mov eax, 12
    mov ebx, 10
    mov ecx, eax
    xor eax, eax
    cmp ecx, ebx
    setbe al
    mov edi, eax
    call print

# > 
    mov eax, 12
    mov ebx, 10
    mov ecx, eax
    xor eax, eax
    cmp ecx, ebx
    setg al
    mov edi, eax
    call print

# <
    mov eax, 12
    mov ebx, 10
    mov ecx, eax
    xor eax, eax
    cmp ecx, ebx
    cmp ecx, ebx
    setge al
    mov edi, eax
    call print

# && = &
# || = |
    mov dword ptr [rbp - 4], 10
    inc dword ptr [rbp - 4]
    mov edi, dword ptr [rbp - 4]
    call print



#exit
    mov rax, SYS_EXIT
    mov rdi, 0
    syscall


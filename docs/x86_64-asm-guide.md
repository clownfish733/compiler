# x86-64 Assembly Reference — Intel syntax (NASM), Linux

Covers syntax, registers, the System V AMD64 ABI, and the Linux syscall ABI.
Primary flavour is NASM; GAS `.intel_syntax noprefix` differences are called out at the end.

---

## 1. Syntax basics

```nasm
    mnemonic  dest, src        ; destination first — opposite of AT&T
```

- No `%` on registers, no `$` on immediates.
- Comments start with `;`.
- Labels: `name:` — a trailing colon is optional in NASM but write it anyway.
- Local labels start with `.` and are scoped to the previous non-local label:

```nasm
loop_a:
.top:               ; really loop_a.top
    dec rcx
    jnz .top
```

### Memory operands

General form:

```
[ base + index*scale + displacement ]
```

- `base`, `index`: any 64-bit GPR (`rsp` cannot be an index).
- `scale`: 1, 2, 4, or 8 only.
- `displacement`: signed 32-bit constant.

```nasm
mov rax, [rbx]                 ; load qword at rbx
mov rax, [rbx + 8]
mov rax, [rbx + rcx*8]         ; array[i] for 8-byte elements
mov rax, [rbx + rcx*8 + 16]
mov rax, [rbp - 24]            ; local variable
```

### Size specifiers

Needed whenever the width is ambiguous — i.e. when neither operand is a register.

```nasm
mov qword [rax], 1             ; 8 bytes
mov dword [rax], 1             ; 4 bytes
mov byte  [rax], 1             ; 1 byte
mov rax, [rbx]                 ; unambiguous, rax is 64-bit
```

`byte` / `word` (2) / `dword` (4) / `qword` (8) / `oword` (16).

### Sections & data

```nasm
section .rodata
msg:    db "hello", 10         ; 10 = '\n'
msglen: equ $ - msg            ; $ = current address; $$ = section start

section .data
counter: dq 0

section .bss
buf:    resb 4096              ; reserve, zero-initialised
arr:    resq 100

section .text
global main
```

Emit: `db dw dd dq dt` (1/2/4/8/10 bytes). Reserve: `resb resw resd resq`.
`times 64 db 0` repeats.

---

## 2. Registers

### General purpose (16)

| 64 | 32 | 16 | 8 low | 8 high |
|---|---|---|---|---|
| rax | eax | ax | al | ah |
| rbx | ebx | bx | bl | bh |
| rcx | ecx | cx | cl | ch |
| rdx | edx | dx | dl | dh |
| rsi | esi | si | sil | — |
| rdi | edi | di | dil | — |
| rbp | ebp | bp | bpl | — |
| rsp | esp | sp | spl | — |
| r8–r15 | r8d–r15d | r8w–r15w | r8b–r15b | — |

**The rule everyone trips on:** writing to a **32-bit** sub-register **zero-extends** into the full 64-bit register. Writing to 8- or 16-bit sub-registers leaves the upper bits **unchanged**.

```nasm
mov rax, 0xFFFFFFFFFFFFFFFF
mov eax, 5          ; rax is now 5      — upper 32 bits cleared
mov ax,  5          ; rax = 0xFFFFFFFFFFFF0005
mov al,  5          ; rax = 0xFFFFFFFFFFFFFF05
```

Consequences:
- `xor eax, eax` is the canonical zeroing idiom — 2 bytes instead of 3, clears all 64 bits, and is recognised as a dependency-break by the CPU.
- `mov eax, ecx` is a valid, shorter zero-extending 32→64 move. There is no `movzx r64, r/m32` because you don't need one.
- The partial writes (`al`, `ax`) create a false dependency on the old value. Prefer `movzx`/`movsx`.

`ah`/`bh`/`ch`/`dh` cannot be encoded in an instruction that also uses `r8`–`r15`, `sil`, `dil`, `spl`, or `bpl` — they need a REX prefix, which repurposes those encodings. Mostly just avoid the high-byte registers.

### Special roles

- `rsp` — stack pointer. Grows **downward**. `push` subtracts 8 then stores.
- `rbp` — frame pointer by convention only; free as a GPR if you omit frames.
- `rip` — instruction pointer. Not directly readable, but addressable:

```nasm
default rel                    ; put near top of file: bare [sym] becomes RIP-relative
lea rdi, [msg]                 ; RIP-relative address of msg
lea rdi, [rel msg]             ; explicit, works without `default rel`
```

Use RIP-relative addressing by default on x86-64. Absolute 64-bit addresses are larger and break position-independent executables.

### Flags (`rflags`)

| Flag | Meaning |
|---|---|
| ZF | result was zero |
| SF | sign bit of result |
| CF | unsigned overflow / borrow |
| OF | signed overflow |
| PF | parity of low byte |
| DF | direction for string ops (`cld` forward, `std` backward) |

### SIMD / FP

- `xmm0`–`xmm15` (128-bit), `ymm0`–`ymm15` (256, AVX), `zmm0`–`zmm31` (512, AVX-512).
- `float` and `double` live in the low bits of xmm registers, not x87.
- x87 (`st0`–`st7`) is only used for `long double` on Linux.

---

## 3. System V AMD64 ABI (Linux, macOS, BSD)

### Argument passing

| Purpose | Registers |
|---|---|
| Integer / pointer args 1–6 | `rdi, rsi, rdx, rcx, r8, r9` |
| Floating-point args 1–8 | `xmm0`–`xmm7` |
| Further args | pushed on stack, **right to left** |
| Integer return | `rax` (`rax:rdx` for 128-bit) |
| FP return | `xmm0` (`xmm0:xmm1` for two-field structs) |

Mnemonic for the integer order: **Di**ane's **Si**lk **D**ress **C**osts **8**9 → di, si, d, c, 8, 9.

### Register preservation

| Callee-saved (you must restore) | Caller-saved (assume destroyed by any `call`) |
|---|---|
| `rbx, rbp, rsp, r12, r13, r14, r15` | `rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11` |
| | all `xmm`/`ymm`/`zmm` |

`r10` and `r11` are scratch — `r11` in particular is guaranteed dead after `syscall`.

### Stack alignment — the classic segfault

`rsp` must be **16-byte aligned at the point of the `call` instruction**. Because `call` pushes an 8-byte return address, on entry to your function `rsp ≡ 8 (mod 16)`.

So on entry:
- `push rbp` brings you back to 16-aligned.
- If you don't push anything, subtract 8 (or an odd multiple of 8) before calling out.

Ignore this and libc functions that use SSE (`printf` with floats, `memset`, ...) will fault on an unaligned `movaps`.

### Red zone

The 128 bytes **below** `rsp` are guaranteed untouched by signal handlers. A leaf function (one that calls nothing) can use `[rsp - 8]` … `[rsp - 128]` as scratch without adjusting `rsp` at all. Not available in kernel code or interrupt handlers.

### Variadic functions

Before calling a variadic function, `al` must hold the number of vector registers used (an upper bound is fine):

```nasm
lea  rdi, [fmt]
mov  rsi, 42
xor  eax, eax        ; 0 vector registers — required for printf!
call printf wrt ..plt
```

Forgetting `xor eax, eax` before `printf` is one of the most common crashes.

### Struct passing (summary)

Structs ≤ 16 bytes get split into two 8-byte "eightbytes", each classified INTEGER or SSE and passed in the corresponding register class. Anything larger, or containing unaligned fields, goes in memory. Structs returned in memory: the caller passes a hidden pointer in `rdi` (shifting all other args right by one) and the callee returns that pointer in `rax`.

### Prologue / epilogue

```nasm
my_func:
    push rbp
    mov  rbp, rsp
    sub  rsp, 32          ; locals; keep total a multiple of 16
    push rbx              ; save any callee-saved regs you clobber
    ; ...
    pop  rbx
    leave                 ; == mov rsp, rbp ; pop rbp
    ret
```

Frame-pointer omission is standard at `-O1`+; then locals are addressed off `rsp` and `rbp` is a free GPR. Keep frames while learning — they make `gdb` backtraces work.

---

## 4. Linux syscall ABI — *different from the function ABI*

| | |
|---|---|
| Syscall number | `rax` |
| Args 1–6 | `rdi, rsi, rdx, **r10**, r8, r9` |
| Instruction | `syscall` |
| Return | `rax` |
| Clobbered | `rcx` (holds return addr), `r11` (holds rflags) |

**`r10`, not `rcx`,** for the fourth argument — the `syscall` instruction overwrites `rcx`.

Errors come back as a negative errno in `rax` (range `-4095` … `-1`); there is no `errno` variable at this level. Everything else is preserved.

Numbers are in `/usr/include/asm/unistd_64.h`. A few: `read` 0, `write` 1, `open` 2, `close` 3, `mmap` 9, `exit` 60, `exit_group` 231.

```nasm
    mov rax, 1          ; write
    mov rdi, 1          ; fd = stdout
    lea rsi, [msg]
    mov rdx, msglen
    syscall
```

---

## 5. Instruction cheat sheet

### Movement

```nasm
mov   rax, rbx
movzx rax, bl        ; zero-extend 8/16 → wider
movsx rax, bl        ; sign-extend 8/16 → wider
movsxd rax, ecx      ; sign-extend 32 → 64
lea   rax, [rbx + rcx*4 + 8]   ; compute address; no memory access
xchg  rax, rbx
```

`lea` is the general-purpose "do arithmetic without touching flags" instruction. `lea rax, [rdi + rdi*2]` is `rax = rdi * 3` in one uop.

### Arithmetic

```nasm
add rax, rbx
sub rax, rbx
inc rax / dec rax          ; do not write CF — can cause partial-flag stalls
neg rax
imul rax, rbx              ; 2-operand signed multiply
imul rax, rbx, 10          ; 3-operand
mul  rbx                   ; 1-operand: rdx:rax = rax * rbx (unsigned)
```

Division needs the dividend in `rdx:rax`, so **you must set `rdx` first**:

```nasm
; unsigned: rax / rbx
xor edx, edx
div rbx                    ; rax = quotient, rdx = remainder

; signed: rax / rbx
cqo                        ; sign-extend rax into rdx (cdq for eax→edx)
idiv rbx
```

Skipping `xor edx, edx` / `cqo` gives a `#DE` fault or garbage. Division is also ~20–40× slower than multiplication — multiply by a magic reciprocal if it's hot.

### Logic and shifts

```nasm
and / or / xor / not
shl rax, 3        ; logical left  (= *8)
shr rax, 3        ; logical right (zero-fill)
sar rax, 3        ; arithmetic right (sign-fill) — this is signed /8
rol / ror
```

Variable shift counts must be in `cl`: `shl rax, cl`.

### Comparison and branching

```nasm
cmp rax, rbx      ; sets flags from rax - rbx, discards result
test rax, rax     ; sets flags from AND; the idiom for "is rax zero?"
```

After `cmp a, b`:

| Signed | Unsigned | Meaning |
|---|---|---|
| `je` / `jz` | `je` | a == b |
| `jne` | `jne` | a != b |
| `jg` / `jnle` | `ja` | a > b |
| `jge` | `jae` | a ≥ b |
| `jl` | `jb` | a < b |
| `jle` | `jbe` | a ≤ b |

Signed uses **g/l** (greater/less), unsigned uses **a/b** (above/below). Getting these mixed up is a real bug source when comparing pointers or lengths.

```nasm
setg  al          ; store 0/1 in a byte based on condition
cmovg rax, rbx    ; conditional move — branchless, no misprediction
```

### Stack and control flow

```nasm
push rax / pop rax        ; always 8 bytes in 64-bit mode
call func                 ; push return address, jump
ret                       ; pop into rip
jmp  label
```

### String ops

```nasm
cld                       ; DF = 0, forward
mov rcx, 100
mov rsi, src
mov rdi, dst
rep movsb                 ; copy rcx bytes rsi→rdi
rep stosb                 ; fill rcx bytes at rdi with al
```

`rep movsb`/`rep stosb` are genuinely fast on modern Intel/AMD (ERMSB / FSRM) — don't assume a hand-rolled loop beats them.

---

## 6. Full example

```nasm
; sum.asm — sums an array, prints result via libc
; nasm -f elf64 -g -F dwarf sum.asm -o sum.o
; gcc -no-pie sum.o -o sum

default rel

section .rodata
fmt:    db "sum = %ld", 10, 0

section .data
arr:    dq 1, 2, 3, 4, 5
arrlen: equ ($ - arr) / 8

section .text
global main
extern printf

; long sum(const long *a, size_t n)
;   rdi = a, rsi = n  ->  rax
sum:
    xor eax, eax
    xor ecx, ecx
.loop:
    cmp rcx, rsi
    jae .done
    add rax, [rdi + rcx*8]
    inc rcx
    jmp .loop
.done:
    ret                       ; leaf function: no frame needed

main:
    push rbp
    mov  rbp, rsp             ; rsp now 16-aligned

    lea  rdi, [arr]
    mov  rsi, arrlen
    call sum

    lea  rdi, [fmt]
    mov  rsi, rax
    xor  eax, eax             ; 0 vector args for printf
    call printf wrt ..plt

    xor  eax, eax             ; return 0
    leave
    ret
```

Freestanding version, no libc:

```nasm
; nasm -f elf64 hello.asm -o hello.o && ld hello.o -o hello
section .rodata
msg:    db "hello", 10
msglen: equ $ - msg

section .text
global _start
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, msglen
    syscall

    mov rax, 60               ; exit
    xor edi, edi
    syscall
```

Note the entry point differs: `_start` when linking with `ld` directly (and you must exit via syscall — there's no runtime to return to), `main` when linking with `gcc`.

**PIE:** modern GCC defaults to position-independent executables. Either link with `-no-pie`, or use RIP-relative addressing everywhere plus `call sym wrt ..plt` for external calls.

---

## 7. GAS `.intel_syntax noprefix` differences

If you're using `as`/`gcc` rather than NASM, the syntax is Intel but the *semantics of symbols* follow MASM, not NASM. This is the main gotcha:

| | NASM | GAS Intel |
|---|---|---|
| Address of symbol | `mov rax, sym` | `mov rax, OFFSET sym` |
| Contents of symbol | `mov rax, [sym]` | `mov rax, sym` or `mov rax, [sym]` |
| Size override | `mov qword [rax], 1` | `mov qword ptr [rax], 1` |
| RIP-relative | `[rel sym]` or `default rel` | `[rip + sym]` |
| Comment | `;` | `#`, `//`, `/* */` |
| Sections | `section .text` | `.section .text` or `.text` |
| Data | `db / dw / dd / dq` | `.byte / .word / .long / .quad` |
| Reserve | `resb 64` | `.space 64` / `.zero 64` |
| Constant | `equ` | `.equ` / `.set` |
| Globals | `global main` | `.globl main` |

Enable with `.intel_syntax noprefix` at the top of the `.s` file, or compile C with `gcc -masm=intel -S`. Note that GCC will emit `.intel_syntax noprefix` itself in that mode.

In GAS Intel syntax a bare symbol name is a **memory reference**. `mov eax, printf` is an error, not a way to load a function pointer.

---

## 8. Debugging

```
gdb ./prog
set disassembly-flavor intel
layout asm            # or: layout regs
starti                # stop at the first instruction
si / ni               # step / next instruction
info registers
p/x $rax
x/16xb $rsp           # examine memory
x/8i $rip             # next 8 instructions
```

`objdump -d -M intel prog` for a static disassembly.
`gcc -O2 -S -masm=intel` (or godbolt.org) to see what the compiler does — the fastest way to learn idiomatic codegen.

---

## 9. Gotcha checklist

1. `rsp` must be 16-aligned **at the `call`**, so `≡ 8 mod 16` on function entry.
2. `xor eax, eax` before calling `printf` or any variadic function.
3. Syscall arg 4 is `r10`, not `rcx`. `syscall` clobbers `rcx` and `r11`.
4. `cqo`/`xor edx, edx` before `idiv`/`div`.
5. Signed `jg/jl` vs unsigned `ja/jb`.
6. Writing `eax` zeroes the top half of `rax`; writing `al` does not.
7. Restore `rbx`, `rbp`, `r12`–`r15` if you touch them.
8. `mov` immediates are limited to 32 bits sign-extended, except `mov r64, imm64`.
9. Displacements are 32-bit signed — you can't `mov rax, [rbx + 0x100000000]`.
10. `[rsp]` as an index register is not encodable; `rsp` can only be a base.

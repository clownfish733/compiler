	.intel_syntax noprefix
	.file	"c6op54kichmh949sgafezgjty"
	.section	.text._RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm,"ax",@progbits
	.hidden	_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm
	.globl	_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm
	.p2align	4
	.type	_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm,@function
_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm:
.Lfunc_begin0:
	.file	1 "/home/clownfish73/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src" "rt.rs"
	.loc	1 199 0
	.cfi_startproc
	sub	rsp, 40
	.cfi_def_cfa_offset 48
	mov	eax, ecx
	mov	rcx, rdx
	mov	rdx, rsi
	mov	qword ptr [rsp + 8], rdi
	mov	qword ptr [rsp + 16], rdx
	mov	qword ptr [rsp + 24], rcx
	mov	byte ptr [rsp + 39], al
.Ltmp0:
	.loc	1 206 10 prologue_end
	mov	qword ptr [rsp], rdi
	.loc	1 205 5
	mov	rdi, rsp
	lea	rsi, [rip + .Lanon.1068aae783be348f63a596d68b563339.0]
	movzx	r8d, al
	call	qword ptr [rip + _RNvNtCsgczF5crJ4sT_3std2rt19lang_start_internal@GOTPCREL]
	.loc	1 211 2 epilogue_begin
	add	rsp, 40
	.cfi_def_cfa_offset 8
	ret
.Ltmp1:
.Lfunc_end0:
	.size	_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm, .Lfunc_end0-_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm
	.cfi_endproc

	.section	.text._RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm,"ax",@progbits
	.p2align	4
	.type	_RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm,@function
_RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm:
.Lfunc_begin1:
	.file	2 "/home/clownfish73/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys" "backtrace.rs"
	.loc	2 162 0
	.cfi_startproc
	sub	rsp, 24
	.cfi_def_cfa_offset 32
	mov	qword ptr [rsp + 8], rdi
.Ltmp2:
	.loc	2 166 18 prologue_end
	call	_RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm
.Ltmp3:
	.file	3 "/home/clownfish73/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src" "hint.rs"
	.loc	3 491 5
	#APP
	#NO_APP
.Ltmp4:
	.loc	2 172 2 epilogue_begin
	add	rsp, 24
	.cfi_def_cfa_offset 8
	ret
.Ltmp5:
.Lfunc_end1:
	.size	_RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm, .Lfunc_end1-_RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm
	.cfi_endproc

	.section	.text._RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm,"ax",@progbits
	.p2align	4
	.type	_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm,@function
_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm:
.Lfunc_begin2:
	.loc	1 206 0
	.cfi_startproc
	sub	rsp, 24
	.cfi_def_cfa_offset 32
	mov	qword ptr [rsp + 8], rdi
.Ltmp6:
	.loc	1 206 70 prologue_end
	mov	rdi, qword ptr [rdi]
	.loc	1 206 18 is_stmt 0
	call	_RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm
	.loc	1 206 76
	call	_RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm
	mov	byte ptr [rsp + 23], al
.Ltmp7:
	.file	4 "/home/clownfish73/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/process/unix" "common.rs"
	.loc	4 592 9 is_stmt 1
	movzx	eax, al
.Ltmp8:
	.loc	1 206 93 epilogue_begin
	add	rsp, 24
	.cfi_def_cfa_offset 8
	ret
.Ltmp9:
.Lfunc_end2:
	.size	_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm, .Lfunc_end2-_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm
	.cfi_endproc
	.file	5 "/home/clownfish73/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src" "process.rs"

	.section	.text._RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm,"ax",@progbits
	.p2align	4
	.type	_RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm,@function
_RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm:
.Lfunc_begin3:
	.file	6 "/home/clownfish73/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops" "function.rs"
	.loc	6 250 0
	.cfi_startproc
	sub	rsp, 24
	.cfi_def_cfa_offset 32
	mov	qword ptr [rsp + 16], rdi
.Ltmp10:
	.loc	6 250 5 prologue_end
	mov	rdi, qword ptr [rdi]
	call	_RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm
	.loc	6 250 5 epilogue_begin is_stmt 0
	add	rsp, 24
	.cfi_def_cfa_offset 8
	ret
.Ltmp11:
.Lfunc_end3:
	.size	_RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm, .Lfunc_end3-_RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm
	.cfi_endproc

	.section	.text._RNvCs3eeMCulCqJh_7rustasm4main,"ax",@progbits
	.hidden	_RNvCs3eeMCulCqJh_7rustasm4main
	.globl	_RNvCs3eeMCulCqJh_7rustasm4main
	.p2align	4
	.type	_RNvCs3eeMCulCqJh_7rustasm4main,@function
_RNvCs3eeMCulCqJh_7rustasm4main:
.Lfunc_begin4:
	.cfi_startproc
	.file	7 "/home/clownfish73/holiday_projects/compiler/experiment/rustasm" "src/main.rs"
	.loc	7 2 9 prologue_end is_stmt 1
	mov	dword ptr [rsp - 4], 9
.Ltmp12:
	.loc	7 3 2
	ret
.Ltmp13:
.Lfunc_end4:
	.size	_RNvCs3eeMCulCqJh_7rustasm4main, .Lfunc_end4-_RNvCs3eeMCulCqJh_7rustasm4main
	.cfi_endproc

	.section	.text._RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm,"ax",@progbits
	.p2align	4
	.type	_RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm,@function
_RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm:
.Lfunc_begin5:
	.cfi_startproc
	.loc	5 2659 6 prologue_end
	xor	eax, eax
	ret
.Ltmp14:
.Lfunc_end5:
	.size	_RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm, .Lfunc_end5-_RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm
	.cfi_endproc

	.section	.text._RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm,"ax",@progbits
	.p2align	4
	.type	_RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm,@function
_RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm:
.Lfunc_begin6:
	.loc	6 250 0
	.cfi_startproc
	sub	rsp, 24
	.cfi_def_cfa_offset 32
	mov	qword ptr [rsp + 16], rdi
.Ltmp15:
	.loc	6 250 5 prologue_end
	call	rdi
	.loc	6 250 5 epilogue_begin is_stmt 0
	add	rsp, 24
	.cfi_def_cfa_offset 8
	ret
.Ltmp16:
.Lfunc_end6:
	.size	_RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm, .Lfunc_end6-_RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm
	.cfi_endproc

	.section	.text._RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm,"ax",@progbits
	.p2align	4
	.type	_RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm,@function
_RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm:
.Lfunc_begin7:
	.loc	6 250 0 is_stmt 1
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	sub	rsp, 40
	.cfi_def_cfa_offset 48
	mov	qword ptr [rsp + 8], rdi
.Ltmp17:
	lea	rdi, [rsp + 8]
.Ltmp20:
	.loc	6 250 5 prologue_end
	call	_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm
.Ltmp18:
	mov	dword ptr [rsp + 4], eax
	jmp	.LBB7_3
.LBB7_1:
	.loc	6 250 5
	mov	rdi, qword ptr [rsp + 24]
	call	_Unwind_Resume@PLT
.LBB7_2:
.Ltmp19:
	.loc	6 0 5 is_stmt 0
	mov	rcx, rax
	mov	eax, edx
	mov	qword ptr [rsp + 24], rcx
	mov	dword ptr [rsp + 32], eax
	jmp	.LBB7_1
.LBB7_3:
	mov	eax, dword ptr [rsp + 4]
	.loc	6 250 5 epilogue_begin
	add	rsp, 40
	.cfi_def_cfa_offset 8
	ret
.Ltmp21:
.Lfunc_end7:
	.size	_RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm, .Lfunc_end7-_RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm
	.cfi_endproc
	.section	.gcc_except_table._RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm,"a",@progbits
	.p2align	2, 0x0
GCC_except_table7:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp17-.Lfunc_begin7
	.uleb128 .Ltmp18-.Ltmp17
	.uleb128 .Ltmp19-.Lfunc_begin7
	.byte	0
	.uleb128 .Ltmp18-.Lfunc_begin7
	.uleb128 .Lfunc_end7-.Ltmp18
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4
	.type	main,@function
main:
.Lfunc_begin8:
	.cfi_startproc
	push	rax
	.cfi_def_cfa_offset 16
	mov	rdx, rsi
	mov	rax, qword ptr [rip + __rustc_debug_gdb_scripts_section__@GOTPCREL]
	mov	al, byte ptr [rax]
	movsxd	rsi, edi
	lea	rdi, [rip + _RNvCs3eeMCulCqJh_7rustasm4main]
	xor	ecx, ecx
	call	_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm
	pop	rcx
	.cfi_def_cfa_offset 8
	ret
.Lfunc_end8:
	.size	main, .Lfunc_end8-main
	.cfi_endproc

	.type	.Lanon.1068aae783be348f63a596d68b563339.0,@object
	.section	.data.rel.ro..Lanon.1068aae783be348f63a596d68b563339.0,"aw",@progbits
	.p2align	3, 0x0
.Lanon.1068aae783be348f63a596d68b563339.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm
	.quad	_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm
	.quad	_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm
	.size	.Lanon.1068aae783be348f63a596d68b563339.0, 48

	.type	__rustc_debug_gdb_scripts_section__,@object
	.section	.debug_gdb_scripts,"aMS",@progbits,1,unique,1
	.weak	__rustc_debug_gdb_scripts_section__
__rustc_debug_gdb_scripts_section__:
	.asciz	"\001gdb_load_rust_pretty_printers.py"
	.size	__rustc_debug_gdb_scripts_section__, 34

	.section	.debug_abbrev,"",@progbits
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	23
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	85
	.byte	23
	.byte	0
	.byte	0
	.byte	2
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	2
	.byte	24
	.byte	0
	.byte	0
	.byte	3
	.byte	19
	.byte	1
	.byte	29
	.byte	19
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	4
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	5
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	6
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	7
	.byte	57
	.byte	1
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	8
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	9
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	10
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	11
	.byte	29
	.byte	1
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	88
	.byte	11
	.byte	89
	.byte	11
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	12
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	49
	.byte	19
	.byte	0
	.byte	0
	.byte	13
	.byte	29
	.byte	0
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	88
	.byte	11
	.byte	89
	.byte	5
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	14
	.byte	47
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	15
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	16
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	0
	.byte	0
	.byte	17
	.byte	11
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	18
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	49
	.byte	19
	.byte	0
	.byte	0
	.byte	19
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	20
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	50
	.byte	11
	.byte	0
	.byte	0
	.byte	21
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	60
	.byte	25
	.byte	0
	.byte	0
	.byte	22
	.byte	5
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	23
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	24
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	25
	.byte	21
	.byte	0
	.byte	0
	.byte	0
	.byte	26
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	27
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	28
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	29
	.byte	46
	.byte	0
	.byte	71
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	30
	.byte	46
	.byte	1
	.byte	71
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	31
	.byte	5
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	32
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	106
	.byte	25
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_info,"",@progbits
.Lcu_begin0:
	.long	.Ldebug_info_end0-.Ldebug_info_start0
.Ldebug_info_start0:
	.short	4
	.long	.debug_abbrev
	.byte	8
	.byte	1
	.long	.Linfo_string0
	.short	28
	.long	.Linfo_string1
	.long	.Lline_table_start0
	.long	.Linfo_string2
	.quad	0
	.long	.Ldebug_ranges0
	.byte	2
	.long	.Linfo_string3
	.long	61
	.byte	9
	.byte	3
	.quad	.Lanon.1068aae783be348f63a596d68b563339.0
	.byte	3
	.long	181
	.long	.Linfo_string19
	.byte	48
	.byte	8
	.byte	4
	.long	.Linfo_string4
	.long	139
	.byte	8
	.byte	0
	.byte	4
	.long	.Linfo_string7
	.long	159
	.byte	8
	.byte	8
	.byte	4
	.long	.Linfo_string9
	.long	159
	.byte	8
	.byte	16
	.byte	4
	.long	.Linfo_string10
	.long	139
	.byte	8
	.byte	24
	.byte	4
	.long	.Linfo_string11
	.long	139
	.byte	8
	.byte	32
	.byte	4
	.long	.Linfo_string12
	.long	139
	.byte	8
	.byte	40
	.byte	0
	.byte	5
	.long	152
	.long	.Linfo_string6
	.long	0
	.byte	6
	.long	.Linfo_string5
	.byte	7
	.byte	0
	.byte	6
	.long	.Linfo_string8
	.byte	7
	.byte	8
	.byte	7
	.long	.Linfo_string13
	.byte	7
	.long	.Linfo_string14
	.byte	7
	.long	.Linfo_string15
	.byte	8
	.long	.Linfo_string18
	.byte	8
	.byte	8
	.byte	4
	.long	.Linfo_string16
	.long	689
	.byte	8
	.byte	0
	.byte	0
	.byte	9
	.quad	.Lfunc_begin2
	.long	.Lfunc_end2-.Lfunc_begin2
	.byte	1
	.byte	87
	.long	.Linfo_string52
	.long	.Linfo_string53
	.byte	1
	.byte	206
	.long	976
	.byte	10
	.byte	3
	.byte	145
	.byte	8
	.byte	6
	.long	.Linfo_string16
	.byte	8
	.byte	1
	.byte	200
	.long	689
	.byte	11
	.long	1002
	.quad	.Ltmp7
	.long	.Ltmp8-.Ltmp7
	.byte	1
	.byte	206
	.byte	85
	.byte	12
	.byte	2
	.byte	145
	.byte	23
	.long	1008
	.byte	13
	.long	996
	.quad	.Ltmp7
	.long	.Ltmp8-.Ltmp7
	.byte	5
	.short	2256
	.byte	16
	.byte	0
	.byte	14
	.long	152
	.long	.Linfo_string22
	.byte	0
	.byte	0
	.byte	9
	.quad	.Lfunc_begin0
	.long	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	87
	.long	.Linfo_string46
	.long	.Linfo_string47
	.byte	1
	.byte	199
	.long	1082
	.byte	15
	.byte	2
	.byte	145
	.byte	8
	.long	.Linfo_string16
	.byte	1
	.byte	200
	.long	689
	.byte	15
	.byte	2
	.byte	145
	.byte	16
	.long	.Linfo_string64
	.byte	1
	.byte	201
	.long	1082
	.byte	15
	.byte	2
	.byte	145
	.byte	24
	.long	.Linfo_string65
	.byte	1
	.byte	202
	.long	1089
	.byte	15
	.byte	2
	.byte	145
	.byte	39
	.long	.Linfo_string68
	.byte	1
	.byte	203
	.long	969
	.byte	14
	.long	152
	.long	.Linfo_string22
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string26
	.byte	7
	.long	.Linfo_string27
	.byte	16
	.quad	.Lfunc_begin1
	.long	.Lfunc_end1-.Lfunc_begin1
	.byte	1
	.byte	87
	.long	.Linfo_string50
	.long	.Linfo_string51
	.byte	2
	.byte	162
	.byte	15
	.byte	2
	.byte	145
	.byte	8
	.long	.Linfo_string70
	.byte	2
	.byte	162
	.long	689
	.byte	17
	.quad	.Ltmp3
	.long	.Ltmp4-.Ltmp3
	.byte	10
	.byte	2
	.byte	145
	.byte	7
	.long	.Linfo_string69
	.byte	1
	.byte	2
	.byte	166
	.long	152
	.byte	11
	.long	713
	.quad	.Ltmp3
	.long	.Ltmp4-.Ltmp3
	.byte	2
	.byte	169
	.byte	5
	.byte	18
	.byte	2
	.byte	145
	.byte	23
	.long	735
	.byte	0
	.byte	0
	.byte	14
	.long	689
	.long	.Linfo_string49
	.byte	14
	.long	152
	.long	.Linfo_string22
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string28
	.byte	7
	.long	.Linfo_string29
	.byte	7
	.long	.Linfo_string30
	.byte	19
	.long	.Linfo_string33
	.byte	1
	.byte	1
	.byte	1
	.byte	20
	.long	.Linfo_string31
	.long	969
	.byte	1
	.byte	0
	.byte	3
	.byte	21
	.long	.Linfo_string34
	.long	.Linfo_string35
	.byte	4
	.short	591
	.long	976

	.byte	22
	.long	983
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string28
	.byte	19
	.long	.Linfo_string33
	.byte	1
	.byte	1
	.byte	1
	.byte	20
	.long	.Linfo_string31
	.long	544
	.byte	1
	.byte	0
	.byte	3
	.byte	21
	.long	.Linfo_string38
	.long	.Linfo_string39
	.byte	5
	.short	2255
	.long	976

	.byte	22
	.long	596
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string45
	.byte	23
	.quad	.Lfunc_begin5
	.long	.Lfunc_end5-.Lfunc_begin5
	.byte	1
	.byte	87
	.long	.Linfo_string59
	.long	.Linfo_string60
	.byte	5
	.short	2657
	.long	596
	.byte	24
	.byte	2
	.byte	145
	.byte	127
	.byte	5
	.short	2657
	.long	152
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	702
	.long	.Linfo_string17
	.long	0
	.byte	25
	.byte	7
	.long	.Linfo_string20
	.byte	7
	.long	.Linfo_string21
	.byte	26
	.long	.Linfo_string23
	.long	.Linfo_string24
	.byte	3
	.short	490
	.byte	1
	.byte	14
	.long	152
	.long	.Linfo_string22
	.byte	27
	.long	.Linfo_string25
	.byte	1
	.byte	3
	.short	490
	.long	152
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string41
	.byte	7
	.long	.Linfo_string42
	.byte	7
	.long	.Linfo_string43
	.byte	9
	.quad	.Lfunc_begin3
	.long	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	87
	.long	.Linfo_string56
	.long	.Linfo_string57
	.byte	6
	.byte	250
	.long	976
	.byte	28
	.byte	2
	.byte	145
	.byte	16
	.byte	6
	.byte	250
	.long	1115
	.byte	28
	.byte	2
	.byte	145
	.byte	15
	.byte	6
	.byte	250
	.long	152
	.byte	14
	.long	181
	.long	.Linfo_string54
	.byte	14
	.long	152
	.long	.Linfo_string55
	.byte	0
	.byte	16
	.quad	.Lfunc_begin6
	.long	.Lfunc_end6-.Lfunc_begin6
	.byte	1
	.byte	87
	.long	.Linfo_string61
	.long	.Linfo_string62
	.byte	6
	.byte	250
	.byte	28
	.byte	2
	.byte	145
	.byte	16
	.byte	6
	.byte	250
	.long	689
	.byte	28
	.byte	2
	.byte	145
	.byte	15
	.byte	6
	.byte	250
	.long	152
	.byte	14
	.long	689
	.long	.Linfo_string54
	.byte	14
	.long	152
	.long	.Linfo_string55
	.byte	0
	.byte	9
	.quad	.Lfunc_begin7
	.long	.Lfunc_end7-.Lfunc_begin7
	.byte	1
	.byte	87
	.long	.Linfo_string63
	.long	.Linfo_string57
	.byte	6
	.byte	250
	.long	976
	.byte	28
	.byte	2
	.byte	145
	.byte	8
	.byte	6
	.byte	250
	.long	181
	.byte	28
	.byte	2
	.byte	145
	.byte	23
	.byte	6
	.byte	250
	.long	152
	.byte	14
	.long	181
	.long	.Linfo_string54
	.byte	14
	.long	152
	.long	.Linfo_string55
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	6
	.long	.Linfo_string32
	.byte	7
	.byte	1
	.byte	6
	.long	.Linfo_string36
	.byte	5
	.byte	4
	.byte	5
	.long	544
	.long	.Linfo_string37
	.long	0
	.byte	29
	.long	564
	.byte	1
	.byte	30
	.long	616
	.byte	1
	.byte	31
	.long	.Linfo_string40
	.byte	5
	.short	2255
	.long	596
	.byte	0
	.byte	7
	.long	.Linfo_string44
	.byte	32
	.quad	.Lfunc_begin4
	.long	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	87
	.long	.Linfo_string58
	.long	.Linfo_string16
	.byte	7
	.byte	1

	.byte	17
	.quad	.Lfunc_begin4
	.long	.Ltmp12-.Lfunc_begin4
	.byte	10
	.byte	2
	.byte	145
	.byte	124
	.long	.Linfo_string72
	.byte	4
	.byte	7
	.byte	2
	.long	976
	.byte	0
	.byte	0
	.byte	0
	.byte	6
	.long	.Linfo_string48
	.byte	5
	.byte	8
	.byte	5
	.long	1102
	.long	.Linfo_string67
	.long	0
	.byte	5
	.long	969
	.long	.Linfo_string66
	.long	0
	.byte	5
	.long	181
	.long	.Linfo_string71
	.long	0
	.byte	0
.Ldebug_info_end0:
	.section	.data.rel.ro..Lanon.1068aae783be348f63a596d68b563339.0,"aw",@progbits
.Lsec_end0:
	.section	.text._RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm,"ax",@progbits
.Lsec_end1:
	.section	.text._RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm,"ax",@progbits
.Lsec_end2:
	.section	.text._RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm,"ax",@progbits
.Lsec_end3:
	.section	.text._RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm,"ax",@progbits
.Lsec_end4:
	.section	.text._RNvCs3eeMCulCqJh_7rustasm4main,"ax",@progbits
.Lsec_end5:
	.section	.text._RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm,"ax",@progbits
.Lsec_end6:
	.section	.text._RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm,"ax",@progbits
.Lsec_end7:
	.section	.text._RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm,"ax",@progbits
.Lsec_end8:
	.section	.debug_aranges,"",@progbits
	.long	172
	.short	2
	.long	.Lcu_begin0
	.byte	8
	.byte	0
	.zero	4,255
	.quad	.Lanon.1068aae783be348f63a596d68b563339.0
	.quad	.Lsec_end0-.Lanon.1068aae783be348f63a596d68b563339.0
	.quad	.Lfunc_begin0
	.quad	.Lsec_end1-.Lfunc_begin0
	.quad	.Lfunc_begin1
	.quad	.Lsec_end2-.Lfunc_begin1
	.quad	.Lfunc_begin2
	.quad	.Lsec_end3-.Lfunc_begin2
	.quad	.Lfunc_begin3
	.quad	.Lsec_end4-.Lfunc_begin3
	.quad	.Lfunc_begin4
	.quad	.Lsec_end5-.Lfunc_begin4
	.quad	.Lfunc_begin5
	.quad	.Lsec_end6-.Lfunc_begin5
	.quad	.Lfunc_begin6
	.quad	.Lsec_end7-.Lfunc_begin6
	.quad	.Lfunc_begin7
	.quad	.Lsec_end8-.Lfunc_begin7
	.quad	0
	.quad	0
	.section	.debug_ranges,"",@progbits
.Ldebug_ranges0:
	.quad	.Lfunc_begin0
	.quad	.Lfunc_end0
	.quad	.Lfunc_begin1
	.quad	.Lfunc_end1
	.quad	.Lfunc_begin2
	.quad	.Lfunc_end2
	.quad	.Lfunc_begin3
	.quad	.Lfunc_end3
	.quad	.Lfunc_begin4
	.quad	.Lfunc_end4
	.quad	.Lfunc_begin5
	.quad	.Lfunc_end5
	.quad	.Lfunc_begin6
	.quad	.Lfunc_end6
	.quad	.Lfunc_begin7
	.quad	.Lfunc_end7
	.quad	0
	.quad	0
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"clang LLVM (rustc version 1.97.0 (2d8144b78 2026-07-07))"
.Linfo_string1:
	.asciz	"src/main.rs/@/c6op54kichmh949sgafezgjty"
.Linfo_string2:
	.asciz	"/home/clownfish73/holiday_projects/compiler/experiment/rustasm"
.Linfo_string3:
	.asciz	"<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}"
.Linfo_string4:
	.asciz	"drop_in_place"
.Linfo_string5:
	.asciz	"()"
.Linfo_string6:
	.asciz	"*const ()"
.Linfo_string7:
	.asciz	"size"
.Linfo_string8:
	.asciz	"usize"
.Linfo_string9:
	.asciz	"align"
.Linfo_string10:
	.asciz	"__method3"
.Linfo_string11:
	.asciz	"__method4"
.Linfo_string12:
	.asciz	"__method5"
.Linfo_string13:
	.asciz	"std"
.Linfo_string14:
	.asciz	"rt"
.Linfo_string15:
	.asciz	"lang_start"
.Linfo_string16:
	.asciz	"main"
.Linfo_string17:
	.asciz	"fn()"
.Linfo_string18:
	.asciz	"{closure_env#0}<()>"
.Linfo_string19:
	.asciz	"<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}"
.Linfo_string20:
	.asciz	"core"
.Linfo_string21:
	.asciz	"hint"
.Linfo_string22:
	.asciz	"T"
.Linfo_string23:
	.asciz	"_RINvNtCscI6d9CVNmLh_4core4hint9black_boxuECs3eeMCulCqJh_7rustasm"
.Linfo_string24:
	.asciz	"black_box<()>"
.Linfo_string25:
	.asciz	"dummy"
.Linfo_string26:
	.asciz	"sys"
.Linfo_string27:
	.asciz	"backtrace"
.Linfo_string28:
	.asciz	"process"
.Linfo_string29:
	.asciz	"unix"
.Linfo_string30:
	.asciz	"common"
.Linfo_string31:
	.asciz	"__0"
.Linfo_string32:
	.asciz	"u8"
.Linfo_string33:
	.asciz	"ExitCode"
.Linfo_string34:
	.asciz	"_RNvMs8_NtNtNtNtCsgczF5crJ4sT_3std3sys7process4unix6commonNtB5_8ExitCode6as_i32Cs3eeMCulCqJh_7rustasm"
.Linfo_string35:
	.asciz	"as_i32"
.Linfo_string36:
	.asciz	"i32"
.Linfo_string37:
	.asciz	"&std::sys::process::unix::common::ExitCode"
.Linfo_string38:
	.asciz	"_RNvMsT_NtCsgczF5crJ4sT_3std7processNtB5_8ExitCode6to_i32Cs3eeMCulCqJh_7rustasm"
.Linfo_string39:
	.asciz	"to_i32"
.Linfo_string40:
	.asciz	"self"
.Linfo_string41:
	.asciz	"ops"
.Linfo_string42:
	.asciz	"function"
.Linfo_string43:
	.asciz	"FnOnce"
.Linfo_string44:
	.asciz	"rustasm"
.Linfo_string45:
	.asciz	"{impl#63}"
.Linfo_string46:
	.asciz	"_RINvNtCsgczF5crJ4sT_3std2rt10lang_startuECs3eeMCulCqJh_7rustasm"
.Linfo_string47:
	.asciz	"lang_start<()>"
.Linfo_string48:
	.asciz	"isize"
.Linfo_string49:
	.asciz	"F"
.Linfo_string50:
	.asciz	"_RINvNtNtCsgczF5crJ4sT_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs3eeMCulCqJh_7rustasm"
.Linfo_string51:
	.asciz	"__rust_begin_short_backtrace<fn(), ()>"
.Linfo_string52:
	.asciz	"_RNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0Cs3eeMCulCqJh_7rustasm"
.Linfo_string53:
	.asciz	"{closure#0}<()>"
.Linfo_string54:
	.asciz	"Self"
.Linfo_string55:
	.asciz	"Args"
.Linfo_string56:
	.asciz	"_RNSNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_once6vtableCs3eeMCulCqJh_7rustasm"
.Linfo_string57:
	.asciz	"call_once<std::rt::lang_start::{closure_env#0}<()>, ()>"
.Linfo_string58:
	.asciz	"_RNvCs3eeMCulCqJh_7rustasm4main"
.Linfo_string59:
	.asciz	"_RNvXsZ_NtCsgczF5crJ4sT_3std7processuNtB5_11Termination6reportCs3eeMCulCqJh_7rustasm"
.Linfo_string60:
	.asciz	"report"
.Linfo_string61:
	.asciz	"_RNvYFEuINtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm"
.Linfo_string62:
	.asciz	"call_once<fn(), ()>"
.Linfo_string63:
	.asciz	"_RNvYNCINvNtCsgczF5crJ4sT_3std2rt10lang_startuE0INtNtNtCscI6d9CVNmLh_4core3ops8function6FnOnceuE9call_onceCs3eeMCulCqJh_7rustasm"
.Linfo_string64:
	.asciz	"argc"
.Linfo_string65:
	.asciz	"argv"
.Linfo_string66:
	.asciz	"*const u8"
.Linfo_string67:
	.asciz	"*const *const u8"
.Linfo_string68:
	.asciz	"sigpipe"
.Linfo_string69:
	.asciz	"result"
.Linfo_string70:
	.asciz	"f"
.Linfo_string71:
	.asciz	"*mut std::rt::lang_start::{closure_env#0}<()>"
.Linfo_string72:
	.asciz	"x"
	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.97.0 (2d8144b78 2026-07-07)"
	.section	".note.GNU-stack","",@progbits
	.section	.debug_line,"",@progbits
.Lline_table_start0:

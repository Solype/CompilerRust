	.file	"main.c"
	.text
	.globl	main
	.type	main, @function
main:
.LFB0:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pxor	%xmm0, %xmm0
	movss	%xmm0, -4(%rbp)
	pxor	%xmm1, %xmm1
	cvtss2sd	-4(%rbp), %xmm1
	movsd	.LC1(%rip), %xmm0
	addsd	%xmm1, %xmm0
	cvtsd2ss	%xmm0, %xmm0
	movss	%xmm0, -4(%rbp)
	pxor	%xmm1, %xmm1
	cvtss2sd	-4(%rbp), %xmm1
	movsd	.LC2(%rip), %xmm0
	addsd	%xmm1, %xmm0
	cvtsd2ss	%xmm0, %xmm0
	movss	%xmm0, -4(%rbp)
	movl	$0, -8(%rbp)
	addl	$1, -8(%rbp)
	movl	$0, %eax
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.section	.rodata
	.align 8
.LC1:
	.long	-1717986918
	.long	1069128089
	.align 8
.LC2:
	.long	-1717986918
	.long	1070176665
	.ident	"GCC: (GNU) 16.1.1 20260501 (Red Hat 16.1.1-1)"
	.section	.note.GNU-stack,"",@progbits

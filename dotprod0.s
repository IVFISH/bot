.section .text,"xr",one_only,bot::dotproduct::dot_prod_simd_0
	.globl	bot::dotproduct::dot_prod_simd_0
	.p2align	4, 0x90
bot::dotproduct::dot_prod_simd_0:
	.cv_func_id 1048
.seh_proc _ZN3bot10dotproduct15dot_prod_simd_017hc56f0bcf5e0da9b1E
	sub rsp, 104
	.seh_stackalloc 104
	.seh_endprologue
	mov qword ptr [rsp + 40], rdx
	mov qword ptr [rsp + 48], r9
	cmp rdx, r9
	jne .LBB15_5
	.cv_inline_site_id 1049 within 1048 inlined_at 50 46 0
	.cv_inline_site_id 1050 within 1049 inlined_at 10 3577 0
	.cv_inline_site_id 1051 within 1050 inlined_at 51 106 0
	.cv_inline_site_id 1052 within 1051 inlined_at 5 128 0
	.cv_inline_site_id 1053 within 1052 inlined_at 6 104 0
	.cv_inline_site_id 1054 within 1053 inlined_at 6 283 0
	.cv_inline_site_id 1055 within 1054 inlined_at 41 843 0
	.cv_inline_site_id 1056 within 1055 inlined_at 41 752 0
	xorps xmm0, xmm0
	cmp rdx, 4
	jb .LBB15_4
	shr rdx, 2
	xorps xmm1, xmm1
	xor eax, eax
	.p2align	4, 0x90
.LBB15_3:
	.cv_inline_site_id 1057 within 1053 inlined_at 6 288 0
	.cv_inline_site_id 1058 within 1057 inlined_at 6 273 0
	movups xmm2, xmmword ptr [rcx + rax]
	.cv_inline_site_id 1059 within 1057 inlined_at 6 273 0
	movups xmm3, xmmword ptr [r8 + rax]
	.cv_inline_site_id 1060 within 1053 inlined_at 6 288 0
	.cv_inline_site_id 1061 within 1060 inlined_at 5 88 0
	.cv_inline_site_id 1062 within 1061 inlined_at 50 45 0
	mulps xmm3, xmm2
	movaps xmm2, xmm3
	addss xmm2, xmm1
	movshdup xmm4, xmm3
	addss xmm4, xmm2
	movaps xmm2, xmm3
	unpckhpd xmm2, xmm3
	addss xmm2, xmm4
	shufps xmm3, xmm3, 255
	addss xmm3, xmm2
	.cv_inline_site_id 1063 within 1060 inlined_at 5 88 0
	addss xmm0, xmm3
	add rax, 16
	dec rdx
	jne .LBB15_3
.LBB15_4:
	add rsp, 104
	ret
.LBB15_5:
	mov qword ptr [rsp + 56], 0
	lea r9, [rip + __unnamed_17]
	lea rcx, [rsp + 40]
	lea rdx, [rsp + 48]
	lea r8, [rsp + 56]
	call core::panicking::assert_failed
	int3

.section .text,"xr",one_only,bot::simd::search_simd_smol
	.globl	bot::simd::search_simd_smol
	.p2align	4, 0x90
bot::simd::search_simd_smol:
	.cv_func_id 309
.seh_proc _ZN3bot4simd16search_simd_smol17h55489519486be4b6E
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push rsi
	.seh_pushreg rsi
	push rdi
	.seh_pushreg rdi
	push rbx
	.seh_pushreg rbx
	sub rsp, 80
	.seh_stackalloc 80
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movdqa xmmword ptr [rsp + 48], xmm7
	.seh_savexmm xmm7, 48
	movdqa xmmword ptr [rsp + 32], xmm6
	.seh_savexmm xmm6, 32
	.seh_endprologue
	mov rsi, rcx
	.cv_inline_site_id 310 within 309 inlined_at 12 23 0
	.cv_inline_site_id 311 within 310 inlined_at 10 2000 0
	.cv_inline_site_id 312 within 311 inlined_at 37 2986 0
	.cv_inline_site_id 313 within 312 inlined_at 38 33 0
	.cv_inline_site_id 314 within 313 inlined_at 39 52 0
	.cv_inline_site_id 315 within 314 inlined_at 37 480 0
	.cv_inline_site_id 316 within 315 inlined_at 37 698 0
	.cv_inline_site_id 317 within 316 inlined_at 22 157 0
	mov r15, r8
	and r15, -8
	je .LBB10_1
	.cv_inline_site_id 318 within 317 inlined_at 22 219 0
	.cv_inline_site_id 319 within 318 inlined_at 35 437 0
	mov rdi, r8
	shr rdi, 3
	shl rdi, 4
	shr r8, 62
	jne .LBB10_3
	.cv_inline_site_id 320 within 317 inlined_at 22 229 0
	.cv_inline_site_id 321 within 320 inlined_at 24 241 0
	.cv_inline_site_id 322 within 321 inlined_at 24 181 0
	.cv_inline_site_id 323 within 322 inlined_at 24 96 0
	mov rbx, rdx
	movzx eax, byte ptr [rip + __rust_no_alloc_shim_is_unstable]
	mov r14d, 2
	mov edx, 2
	mov rcx, rdi
	call __rust_alloc
	test rax, rax
	je .LBB10_4
	xor ecx, ecx
	pcmpeqd xmm0, xmm0

    ;; store xmm1-5 into memory (128 bit)
	movdqa xmm1, xmmword ptr [rip + __xmm@55555555555555555555555555555555] ;; 0x5 = 0b0101
	movdqa xmm2, xmmword ptr [rip + __xmm@33333333333333333333333333333333] ;; 0x3 = 0b0010
	movdqa xmm3, xmmword ptr [rip + __xmm@0f000f000f000f000f000f000f000f00] ;; 
	movdqa xmm4, xmmword ptr [rip + __xmm@000f000f000f000f000f000f000f000f]
	movdqa xmm5, xmmword ptr [rip + __xmm@3f8000003f8000003f8000003f800000]
	.p2align	4, 0x90
.LBB10_7:
	.cv_inline_site_id 324 within 313 inlined_at 39 60 0
	.cv_inline_site_id 325 within 324 inlined_at 40 26 0
	.cv_inline_site_id 326 within 325 inlined_at 37 3122 0
	.cv_inline_site_id 327 within 326 inlined_at 10 813 0
	.cv_inline_site_id 328 within 327 inlined_at 1 87 0
	.cv_inline_site_id 329 within 328 inlined_at 1 666 0
	.cv_inline_site_id 330 within 329 inlined_at 1 480 0
	.cv_inline_site_id 331 within 330 inlined_at 3 98 0
	.cv_inline_site_id 332 within 331 inlined_at 5 128 0
	.cv_inline_site_id 333 within 332 inlined_at 5 128 0
	.cv_inline_site_id 334 within 333 inlined_at 5 128 0
	movdqu xmm6, xmmword ptr [rbx + 2*rcx]
	.cv_inline_site_id 335 within 334 inlined_at 10 2583 0
	.cv_inline_site_id 336 within 335 inlined_at 5 88 0
	.cv_inline_site_id 337 within 336 inlined_at 5 88 0
	.cv_inline_site_id 338 within 337 inlined_at 12 19 0
    ;; sixteen - x.leading_zeros() - p_bitmask[0].trailing_zeros()
	movdqa xmm7, xmm6
	psrlw xmm7, 1
	por xmm7, xmm6
	movdqa xmm8, xmm7
	psrlw xmm8, 2
	por xmm8, xmm7
	movdqa xmm7, xmm8
	psrlw xmm7, 4
	por xmm7, xmm8
	movdqa xmm8, xmm7
	psrlw xmm8, 8
	por xmm8, xmm7
	pxor xmm8, xmm0
	movdqa xmm7, xmm8
	psrlw xmm7, 1
	pand xmm7, xmm1
	psubb xmm8, xmm7
	movdqa xmm7, xmm8
	pand xmm7, xmm2
	psrlw xmm8, 2
	pand xmm8, xmm2
	paddb xmm8, xmm7
	movdqa xmm7, xmm8
	psrlw xmm7, 4
	paddb xmm7, xmm8
	movdqa xmm8, xmm7
	psllw xmm8, 8
	pand xmm8, xmm3
	paddb xmm8, xmm7
	psrlw xmm8, 8
	movdqa xmm7, xmm4
	psubw xmm7, xmm8
	.cv_inline_site_id 339 within 337 inlined_at 12 20 0
	.cv_inline_site_id 340 within 339 inlined_at 15 60 0
    ;; saves xmm registers back into memory
	pand xmm7, xmm4
	movdqa xmm8, xmm7
	punpckhwd xmm8, xmm8
	pslld xmm8, 23
	paddd xmm8, xmm5
	cvttps2dq xmm8, xmm8
	pslld xmm8, 16
	psrad xmm8, 16
	punpcklwd xmm7, xmm7
	pslld xmm7, 23
	paddd xmm7, xmm5
	cvttps2dq xmm7, xmm7
	pslld xmm7, 16
	psrad xmm7, 16
	packssdw xmm7, xmm8
	paddw xmm7, xmm7
	por xmm7, xmm6
	.cv_inline_site_id 341 within 336 inlined_at 5 88 0
	.cv_inline_site_id 342 within 341 inlined_at 5 88 0
	.cv_inline_site_id 343 within 342 inlined_at 1 473 0
	.cv_inline_site_id 344 within 343 inlined_at 1 663 0
	.cv_inline_site_id 345 within 344 inlined_at 26 265 0
	.cv_inline_site_id 346 within 345 inlined_at 41 45 0
	.cv_inline_site_id 347 within 346 inlined_at 10 2405 0
	.cv_inline_site_id 348 within 347 inlined_at 42 392 0
	.cv_inline_site_id 349 within 348 inlined_at 26 269 0
	.cv_inline_site_id 350 within 349 inlined_at 13 294 0
	.cv_inline_site_id 351 within 350 inlined_at 10 810 0
	.cv_inline_site_id 352 within 351 inlined_at 37 3123 0
	movdqu xmmword ptr [rax + 2*rcx], xmm7
	.cv_inline_site_id 353 within 346 inlined_at 10 2404 0
	add rcx, 8
	.cv_inline_site_id 354 within 334 inlined_at 10 2582 0
	.cv_inline_site_id 355 within 354 inlined_at 7 2364 0
	.cv_inline_site_id 356 within 355 inlined_at 8 44 0
	add rdi, -16
	jne .LBB10_7
	jmp .LBB10_8
.LBB10_1:
	xor ecx, ecx
	mov eax, 2
.LBB10_8:
	mov qword ptr [rsi], r15
	mov qword ptr [rsi + 8], rax
	mov qword ptr [rsi + 16], rcx
	mov rax, rsi
	movaps xmm6, xmmword ptr [rsp + 32]
	movaps xmm7, xmmword ptr [rsp + 48]
	movaps xmm8, xmmword ptr [rsp + 64]
	add rsp, 80
	pop rbx
	pop rdi
	pop rsi
	pop r14
	pop r15
	ret
.LBB10_3:
	xor r14d, r14d
.LBB10_4:
	mov rcx, r14
	mov rdx, rdi
	call alloc::raw_vec::handle_error
	int3

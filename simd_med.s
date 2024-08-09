.section .text,"xr",one_only,bot::simd::search_simd_med
	.globl	bot::simd::search_simd_med
	.p2align	4, 0x90
bot::simd::search_simd_med:
	.cv_func_id 294
.seh_proc _ZN3bot4simd15search_simd_med17h64c1b4911cbfcd37E
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r12
	.seh_pushreg r12
	push rsi
	.seh_pushreg rsi
	push rdi
	.seh_pushreg rdi
	push rbx
	.seh_pushreg rbx
	sub rsp, 120
	.seh_stackalloc 120
	movdqa xmmword ptr [rsp + 96], xmm10
	.seh_savexmm xmm10, 96
	movdqa xmmword ptr [rsp + 80], xmm9
	.seh_savexmm xmm9, 80
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movdqa xmmword ptr [rsp + 48], xmm7
	.seh_savexmm xmm7, 48
	movdqa xmmword ptr [rsp + 32], xmm6
	.seh_savexmm xmm6, 32
	.seh_endprologue
	mov rsi, rcx
	mov r12, qword ptr [rsp + 224]
	mov rax, qword ptr [rsp + 208]
	.cv_inline_site_id 295 within 294 inlined_at 12 23 0
	.cv_inline_site_id 296 within 295 inlined_at 10 600 0
	.cv_inline_site_id 297 within 296 inlined_at 6 25 0
	.cv_inline_site_id 298 within 297 inlined_at 6 302 0
	.cv_inline_site_id 299 within 298 inlined_at 6 599 0
	.cv_inline_site_id 300 within 299 inlined_at 5 112 0
	.cv_inline_site_id 301 within 300 inlined_at 7 2369 0
	.cv_inline_site_id 302 within 301 inlined_at 8 57 0
	.cv_inline_site_id 303 within 302 inlined_at 4 902 0
	shr r8, 3
	.cv_inline_site_id 304 within 297 inlined_at 6 303 0
	.cv_inline_site_id 305 within 304 inlined_at 6 599 0
	.cv_inline_site_id 306 within 305 inlined_at 5 112 0
	.cv_inline_site_id 307 within 306 inlined_at 7 2369 0
	.cv_inline_site_id 308 within 307 inlined_at 8 57 0
	.cv_inline_site_id 309 within 308 inlined_at 4 902 0
	shr rax, 3
	cmp r8, rax
	cmovb rax, r8
	.cv_inline_site_id 310 within 294 inlined_at 12 23 0
	.cv_inline_site_id 311 within 310 inlined_at 10 600 0
	.cv_inline_site_id 312 within 311 inlined_at 6 25 0
	.cv_inline_site_id 313 within 312 inlined_at 6 303 0
	.cv_inline_site_id 314 within 313 inlined_at 6 599 0
	.cv_inline_site_id 315 within 314 inlined_at 5 112 0
	.cv_inline_site_id 316 within 315 inlined_at 7 2369 0
	.cv_inline_site_id 317 within 316 inlined_at 8 57 0
	.cv_inline_site_id 318 within 317 inlined_at 4 902 0
	shr r12, 3
	cmp rax, r12
	cmovb r12, rax
	.cv_inline_site_id 319 within 294 inlined_at 12 30 0
	.cv_inline_site_id 320 within 319 inlined_at 10 2000 0
	.cv_inline_site_id 321 within 320 inlined_at 37 2986 0
	.cv_inline_site_id 322 within 321 inlined_at 38 33 0
	.cv_inline_site_id 323 within 322 inlined_at 39 52 0
	.cv_inline_site_id 324 within 323 inlined_at 37 480 0
	.cv_inline_site_id 325 within 324 inlined_at 37 698 0
	.cv_inline_site_id 326 within 325 inlined_at 22 157 0
	test r12, r12
	je .LBB9_1
	.cv_inline_site_id 327 within 326 inlined_at 22 219 0
	.cv_inline_site_id 328 within 327 inlined_at 35 437 0
	mov rax, r12
	shr rax, 59
	mov r14, r12
	shl r14, 4
	test rax, rax
	jne .LBB9_3
	.cv_inline_site_id 329 within 326 inlined_at 22 229 0
	.cv_inline_site_id 330 within 329 inlined_at 24 241 0
	.cv_inline_site_id 331 within 330 inlined_at 24 181 0
	.cv_inline_site_id 332 within 331 inlined_at 24 96 0
	mov rdi, r9
	mov rbx, rdx
	movzx eax, byte ptr [rip + __rust_no_alloc_shim_is_unstable]
	mov r15d, 2
	mov edx, 2
	mov rcx, r14
	call __rust_alloc
	test rax, rax
	je .LBB9_4
	mov rdx, qword ptr [rsp + 216]
	xor ecx, ecx
	pcmpeqd xmm0, xmm0
	movdqa xmm1, xmmword ptr [rip + __xmm@55555555555555555555555555555555]
	movdqa xmm2, xmmword ptr [rip + __xmm@33333333333333333333333333333333]
	movdqa xmm3, xmmword ptr [rip + __xmm@0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f]
	movdqa xmm4, xmmword ptr [rip + __xmm@000f000f000f000f000f000f000f000f]
	movdqa xmm5, xmmword ptr [rip + __xmm@00100010001000100010001000100010]
	mov r8, r12
	.p2align	4, 0x90
.LBB9_7:
	.cv_inline_site_id 333 within 322 inlined_at 39 60 0
	.cv_inline_site_id 334 within 333 inlined_at 40 26 0
	.cv_inline_site_id 335 within 334 inlined_at 37 3122 0
	.cv_inline_site_id 336 within 335 inlined_at 10 813 0
	.cv_inline_site_id 337 within 336 inlined_at 1 87 0
	.cv_inline_site_id 338 within 337 inlined_at 1 666 0
	.cv_inline_site_id 339 within 338 inlined_at 1 480 0
	.cv_inline_site_id 340 within 339 inlined_at 3 98 0
	.cv_inline_site_id 341 within 340 inlined_at 5 128 0
	.cv_inline_site_id 342 within 341 inlined_at 5 128 0
	.cv_inline_site_id 343 within 342 inlined_at 6 104 0
	.cv_inline_site_id 344 within 343 inlined_at 6 288 0
	.cv_inline_site_id 345 within 344 inlined_at 6 273 0
	.cv_inline_site_id 346 within 345 inlined_at 6 114 0
	.cv_inline_site_id 347 within 346 inlined_at 6 273 0
	movdqu xmm6, xmmword ptr [rbx + 2*rcx]
	.cv_inline_site_id 348 within 346 inlined_at 6 273 0
	movdqu xmm8, xmmword ptr [rdi + 2*rcx]
	.cv_inline_site_id 349 within 344 inlined_at 6 273 0
	movdqu xmm7, xmmword ptr [rdx + 2*rcx]
	.cv_inline_site_id 350 within 343 inlined_at 6 288 0
	.cv_inline_site_id 351 within 350 inlined_at 5 88 0
	.cv_inline_site_id 352 within 351 inlined_at 12 24 0
	movdqa xmm9, xmm6
	psrlw xmm9, 1
	por xmm9, xmm6
	movdqa xmm6, xmm9
	psrlw xmm6, 2
	por xmm6, xmm9
	movdqa xmm9, xmm6
	psrlw xmm9, 4
	por xmm9, xmm6
	movdqa xmm6, xmm9
	psrlw xmm6, 8
	por xmm6, xmm9
	pxor xmm6, xmm0
	movdqa xmm9, xmm6
	psrlw xmm9, 1
	pand xmm9, xmm1
	psubb xmm6, xmm9
	movdqa xmm9, xmm6
	pand xmm9, xmm2
	psrlw xmm6, 2
	pand xmm6, xmm2
	paddb xmm6, xmm9
	movdqa xmm9, xmm6
	psrlw xmm9, 4
	paddb xmm9, xmm6
	pand xmm9, xmm3
	movdqa xmm10, xmm9
	psllw xmm10, 8
	paddb xmm10, xmm9
	psrlw xmm10, 8
	movdqa xmm6, xmm4
	psubw xmm6, xmm10
	.cv_inline_site_id 353 within 351 inlined_at 12 25 0
	movdqa xmm9, xmm8
	psrlw xmm9, 1
	por xmm9, xmm8
	movdqa xmm8, xmm9
	psrlw xmm8, 2
	por xmm8, xmm9
	movdqa xmm9, xmm8
	psrlw xmm9, 4
	por xmm9, xmm8
	movdqa xmm8, xmm9
	psrlw xmm8, 8
	por xmm8, xmm9
	pxor xmm8, xmm0
	movdqa xmm9, xmm8
	psrlw xmm9, 1
	pand xmm9, xmm1
	psubb xmm8, xmm9
	movdqa xmm9, xmm8
	pand xmm9, xmm2
	psrlw xmm8, 2
	pand xmm8, xmm2
	paddb xmm8, xmm9
	movdqa xmm9, xmm8
	psrlw xmm9, 4
	paddb xmm9, xmm8
	pand xmm9, xmm3
	movdqa xmm10, xmm9
	psllw xmm10, 8
	paddb xmm10, xmm9
	psrlw xmm10, 8
	movdqa xmm8, xmm5
	psubw xmm8, xmm10
	.cv_inline_site_id 354 within 351 inlined_at 12 26 0
	movdqa xmm9, xmm7
	psrlw xmm9, 1
	por xmm9, xmm7
	movdqa xmm7, xmm9
	psrlw xmm7, 2
	por xmm7, xmm9
	movdqa xmm9, xmm7
	psrlw xmm9, 4
	por xmm9, xmm7
	movdqa xmm7, xmm9
	psrlw xmm7, 8
	por xmm7, xmm9
	pxor xmm7, xmm0
	movdqa xmm9, xmm7
	psrlw xmm9, 1
	pand xmm9, xmm1
	psubb xmm7, xmm9
	movdqa xmm9, xmm7
	pand xmm9, xmm2
	psrlw xmm7, 2
	pand xmm7, xmm2
	paddb xmm7, xmm9
	movdqa xmm9, xmm7
	psrlw xmm9, 4
	paddb xmm9, xmm7
	pand xmm9, xmm3
	movdqa xmm7, xmm9
	psllw xmm7, 8
	paddb xmm7, xmm9
	psrlw xmm7, 8
	movdqa xmm9, xmm4
	psubw xmm9, xmm7
	.cv_inline_site_id 355 within 351 inlined_at 12 27 0
	.cv_inline_site_id 356 within 355 inlined_at 17 90 0
	psubusw xmm8, xmm6
	paddw xmm8, xmm6
	.cv_inline_site_id 357 within 355 inlined_at 17 90 0
	psubusw xmm9, xmm8
	paddw xmm9, xmm8
	.cv_inline_site_id 358 within 350 inlined_at 5 88 0
	.cv_inline_site_id 359 within 358 inlined_at 5 88 0
	.cv_inline_site_id 360 within 359 inlined_at 1 473 0
	.cv_inline_site_id 361 within 360 inlined_at 1 663 0
	.cv_inline_site_id 362 within 361 inlined_at 26 265 0
	.cv_inline_site_id 363 within 362 inlined_at 41 45 0
	.cv_inline_site_id 364 within 363 inlined_at 10 2405 0
	.cv_inline_site_id 365 within 364 inlined_at 42 392 0
	.cv_inline_site_id 366 within 365 inlined_at 26 269 0
	.cv_inline_site_id 367 within 366 inlined_at 13 294 0
	.cv_inline_site_id 368 within 367 inlined_at 10 810 0
	.cv_inline_site_id 369 within 368 inlined_at 37 3123 0
	movdqu xmmword ptr [rax + 2*rcx], xmm9
	.cv_inline_site_id 370 within 363 inlined_at 10 2404 0
	add rcx, 8
	.cv_inline_site_id 371 within 343 inlined_at 6 283 0
	.cv_inline_site_id 372 within 371 inlined_at 43 843 0
	.cv_inline_site_id 373 within 372 inlined_at 43 752 0
	dec r8
	jne .LBB9_7
	jmp .LBB9_8
.LBB9_1:
	mov eax, 2
	xor ecx, ecx
.LBB9_8:
	.cv_inline_site_id 374 within 322 inlined_at 39 51 0
	.cv_inline_site_id 375 within 374 inlined_at 1 69 0
	.cv_inline_site_id 376 within 375 inlined_at 1 626 0
	.cv_inline_site_id 377 within 376 inlined_at 23 847 0
	shl r12, 3
	mov qword ptr [rsi], r12
	mov qword ptr [rsi + 8], rax
	mov qword ptr [rsi + 16], rcx
	mov rax, rsi
	movaps xmm6, xmmword ptr [rsp + 32]
	movaps xmm7, xmmword ptr [rsp + 48]
	movaps xmm8, xmmword ptr [rsp + 64]
	movaps xmm9, xmmword ptr [rsp + 80]
	movaps xmm10, xmmword ptr [rsp + 96]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r14
	pop r15
	ret
.LBB9_3:
	xor r15d, r15d
.LBB9_4:
	mov rcx, r15
	mov rdx, r14
	call alloc::raw_vec::handle_error
	int3

.section .text,"xr",one_only,bot::simd::search_simd
	.globl	bot::simd::search_simd
	.p2align	4, 0x90
bot::simd::search_simd:
	.cv_func_id 309
.seh_proc _ZN3bot4simd11search_simd17h255b4551b38913c5E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
	push r12
	.seh_pushreg r12
	push rsi
	.seh_pushreg rsi
	push rdi
	.seh_pushreg rdi
	push rbx
	.seh_pushreg rbx
	sub rsp, 2952
	.seh_stackalloc 2952
	lea rbp, [rsp + 128]
	.seh_setframe rbp, 128
	movdqa xmmword ptr [rbp + 2800], xmm10
	.seh_savexmm xmm10, 2928
	movdqa xmmword ptr [rbp + 2784], xmm9
	.seh_savexmm xmm9, 2912
	movdqa xmmword ptr [rbp + 2768], xmm8
	.seh_savexmm xmm8, 2896
	movaps xmmword ptr [rbp + 2752], xmm7
	.seh_savexmm xmm7, 2880
	movaps xmmword ptr [rbp + 2736], xmm6
	.seh_savexmm xmm6, 2864
	.seh_endprologue
	mov qword ptr [rbp + 2728], -2
	mov r14, rcx
	.cv_inline_site_id 310 within 309 inlined_at 12 12 0
	mov qword ptr [rbp + 2648], 0
	xorps xmm0, xmm0
	movups xmmword ptr [rbp + 2664], xmm0
	mov qword ptr [rbp + 2656], 8
	mov rax, qword ptr [rdx]
	.cv_inline_site_id 311 within 309 inlined_at 12 14 0
	mov r9d, eax
	and r9d, 15
	shr rax, 4
	movdqu xmm0, xmmword ptr [rdx]
	movdqu xmm1, xmmword ptr [rdx + 16]
	movdqa xmmword ptr [rbp - 64], xmm1
	movdqa xmmword ptr [rbp - 80], xmm0
	mov qword ptr [rsp + 32], rax
	lea rcx, [rbp + 2528]
	lea rdx, [rbp - 80]
	mov r8d, 1
	call bot::scalar::process_scalar
	mov rdi, qword ptr [rbp + 2536]
	mov r15, qword ptr [rbp + 2544]
	.cv_inline_site_id 312 within 309 inlined_at 12 17 0
	.cv_inline_site_id 313 within 312 inlined_at 38 2852 0
	.cv_inline_site_id 314 within 313 inlined_at 39 461 0
	.cv_inline_site_id 315 within 314 inlined_at 39 110 0
	.cv_inline_site_id 316 within 315 inlined_at 39 161 0
	.cv_inline_site_id 317 within 316 inlined_at 38 698 0
	.cv_inline_site_id 318 within 317 inlined_at 22 157 0
	test r15, r15
	mov qword ptr [rbp + 2704], r15
	je .LBB10_2
	.cv_inline_site_id 319 within 318 inlined_at 22 219 0
	.cv_inline_site_id 320 within 319 inlined_at 35 437 0
	mov rax, r15
	shr rax, 58
	mov rsi, r15
	shl rsi, 5
	test rax, rax
	jne .LBB10_10
	.cv_inline_site_id 321 within 318 inlined_at 22 229 0
	.cv_inline_site_id 322 within 321 inlined_at 24 241 0
	.cv_inline_site_id 323 within 322 inlined_at 24 181 0
	.cv_inline_site_id 324 within 323 inlined_at 24 96 0
	movzx eax, byte ptr [rip + __rust_no_alloc_shim_is_unstable]
	mov ebx, 8
	mov edx, 8
	mov rcx, rsi
	call __rust_alloc
	test rax, rax
	jne .LBB10_3
	jmp .LBB10_12
.LBB10_2:
	mov eax, 8
	xor esi, esi
.LBB10_3:
	.cv_inline_site_id 325 within 315 inlined_at 39 166 0
	.cv_inline_site_id 326 within 325 inlined_at 29 1268 0
	mov qword ptr [rbp + 2520], r14
	mov rcx, rax
	mov qword ptr [rbp + 2696], rax
	mov rdx, rdi
	mov r8, rsi
	call memcpy
	.cv_inline_site_id 327 within 309 inlined_at 12 17 0
	lea rcx, [rbp + 2648]
	call alloc::collections::vec_deque::VecDeque<T,A>::grow
	.cv_inline_site_id 328 within 327 inlined_at 21 1751 0
	.cv_inline_site_id 329 within 328 inlined_at 21 187 0
	.cv_inline_site_id 330 within 329 inlined_at 21 160 0
	mov rax, qword ptr [rbp + 2656]
	mov rcx, qword ptr [rbp + 2664]
	.cv_inline_site_id 331 within 327 inlined_at 21 1747 0
	.cv_inline_site_id 332 within 331 inlined_at 21 222 0
	dec rcx
	mov rdx, qword ptr [rbp + 2648]
	add rdx, rcx
	cmovae rdx, rcx
	mov qword ptr [rbp + 2664], rdx
	inc qword ptr [rbp + 2672]
	lea rcx, [rdx + 2*rdx]
	mov rdx, qword ptr [rbp + 2704]
	mov qword ptr [rax + 8*rcx], rdx
	mov r8, qword ptr [rbp + 2696]
	mov qword ptr [rax + 8*rcx + 8], r8
	mov qword ptr [rax + 8*rcx + 16], rdx
	lea rsi, [rbp + 1184]
	lea r15, [rbp + 1624]
	lea rdi, [rbp + 2024]
	.cv_inline_site_id 333 within 309 inlined_at 12 20 0
	mov r14, qword ptr [rbp + 2672]
	xor ebx, ebx
	movaps xmm6, xmmword ptr [rip + __xmm@00100010001000100010001000100010]
	xorps xmm7, xmm7
	jmp .LBB10_5
	.p2align	4, 0x90
.LBB10_28:
	.cv_inline_site_id 334 within 309 inlined_at 12 105 0
	.cv_inline_site_id 335 within 334 inlined_at 21 1774 0
	.cv_inline_site_id 336 within 335 inlined_at 21 215 0
	.cv_inline_site_id 337 within 336 inlined_at 21 210 0
	.cv_inline_site_id 338 within 337 inlined_at 21 766 0
	mov rax, qword ptr [rbp + 2648]
	mov r14, qword ptr [rbp + 2672]
	mov rcx, qword ptr [rbp + 2640]
	mov r13, qword ptr [rbp + 2720]
.LBB10_29:
	mov r8, qword ptr [rbp + 2664]
	add r8, r14
	cmp r8, rax
	mov edx, 0
	cmovae rdx, rax
	sub r8, rdx
	.cv_inline_site_id 339 within 335 inlined_at 21 187 0
	.cv_inline_site_id 340 within 339 inlined_at 21 160 0
	mov rax, qword ptr [rbp + 2656]
	lea rdx, [r8 + 2*r8]
	mov qword ptr [rax + 8*rdx], r13
	mov r8, qword ptr [rbp + 2712]
	mov qword ptr [rax + 8*rdx + 8], r8
	mov qword ptr [rax + 8*rdx + 16], rbx
	inc r14
	mov qword ptr [rbp + 2672], r14
	mov rdx, qword ptr [rbp + 2632]
	.cv_inline_site_id 341 within 309 inlined_at 12 106 0
	.cv_inline_site_id 342 within 341 inlined_at 20 536 0
	.cv_inline_site_id 343 within 342 inlined_at 20 536 0
	.cv_inline_site_id 344 within 343 inlined_at 22 598 0
	test rdx, rdx
	mov ebx, 0
	je .LBB10_5
.LBB10_35:
	.cv_inline_site_id 345 within 309 inlined_at 12 106 0
	.cv_inline_site_id 346 within 345 inlined_at 20 536 0
	.cv_inline_site_id 347 within 346 inlined_at 20 536 0
	.cv_inline_site_id 348 within 347 inlined_at 22 598 0
	.cv_inline_site_id 349 within 348 inlined_at 22 309 0
	shl rdx, 5
	.cv_inline_site_id 350 within 347 inlined_at 22 599 0
	.cv_inline_site_id 351 within 350 inlined_at 24 254 0
	mov r8d, 8
	call __rust_dealloc
.LBB10_5:
	test r14, r14
	je .LBB10_6
	.cv_inline_site_id 352 within 309 inlined_at 12 23 0
	mov rcx, qword ptr [rbp + 2664]
	.cv_inline_site_id 353 within 352 inlined_at 21 1693 0
	.cv_inline_site_id 354 within 353 inlined_at 21 215 0
	.cv_inline_site_id 355 within 354 inlined_at 21 210 0
	lea rdx, [rcx + 1]
	.cv_inline_site_id 356 within 355 inlined_at 21 766 0
	mov r8, qword ptr [rbp + 2648]
	.cv_inline_site_id 357 within 352 inlined_at 21 1697 0
	.cv_inline_site_id 358 within 357 inlined_at 21 180 0
	.cv_inline_site_id 359 within 358 inlined_at 21 160 0
	mov rax, qword ptr [rbp + 2656]
	cmp rdx, r8
	cmovb r8, rbx
	neg r8
	lea rdx, [rcx + r8]
	inc rdx
	mov qword ptr [rbp + 2664], rdx
	dec r14
	mov qword ptr [rbp + 2672], r14
	lea r9, [rcx + 2*rcx]
	mov rdx, qword ptr [rax + 8*r9]
	.cv_inline_site_id 360 within 309 inlined_at 12 23 0
	mov r8, rdx
	neg r8
	jo .LBB10_21
	.cv_inline_site_id 361 within 309 inlined_at 12 24 0
	.cv_inline_site_id 362 within 361 inlined_at 38 2921 0
	.cv_inline_site_id 363 within 362 inlined_at 40 16 0
	mov rcx, qword ptr [rax + 8*r9 + 8]
	mov rax, qword ptr [rax + 8*r9 + 16]
	test rax, rax
	je .LBB10_30
	mov r8, rax
	mov r9, qword ptr [rcx]
	mov rax, r9
	shr rax, 4
	mov qword ptr [rbp + 2456], rax
	and r9, 15
	je .LBB10_34
	mov qword ptr [rbp + 2632], rdx
	mov qword ptr [rbp + 2512], r9
	lea rax, [r9 - 1]
	mov qword ptr [rbp + 2552], rax
	mov rdx, r8
	shl rdx, 5
	lea r12, [rcx + rdx]
	mov r9, r8
	movabs rax, 576460752303423480
	and r9, rax
	shl r9, 5
	add r9, rcx
	mov qword ptr [rbp + 2488], r9
	and r8d, 7
	mov qword ptr [rbp + 2472], r8
	and rdx, -256
	add rdx, rcx
	mov qword ptr [rbp + 2480], rdx
	mov eax, 8
	mov qword ptr [rbp + 2712], rax
	xor r13d, r13d
	xor ebx, ebx
	xor r9d, r9d
	mov qword ptr [rbp + 2640], rcx
	jmp .LBB10_25
	.p2align	4, 0x90
.LBB10_43:
	mov r9, qword ptr [rbp + 2496]
.LBB10_25:
	.cv_inline_site_id 364 within 309 inlined_at 12 33 0
	.cv_inline_site_id 365 within 364 inlined_at 41 843 0
	.cv_inline_site_id 366 within 365 inlined_at 41 752 0
	cmp r9, 4
	mov qword ptr [rbp + 2504], r9
	jae .LBB10_26
	cmp dword ptr [rbp + 2512], 4
	jae .LBB10_40
	mov rax, qword ptr [rbp + 2552]
	lea rax, [rax + 2*rax]
	lea rdx, [rip + __unnamed_8]
	lea rax, [rdx + 8*rax]
	lea r8, [r9 + 2*r9]
	.cv_inline_site_id 367 within 309 inlined_at 12 37 0
	movzx edx, word ptr [rax + 2*r8]
	movd xmm0, edx
	or edx, 65536
	rep bsf	edx, edx
	.cv_inline_site_id 368 within 367 inlined_at 43 154 0
	movd xmm1, edx
	pshuflw xmm1, xmm1, 0
	pshufd xmm1, xmm1, 0
	movdqa xmmword ptr [rbp + 2304], xmm1
	.cv_inline_site_id 369 within 309 inlined_at 12 38 0
	movzx edx, word ptr [rax + 2*r8 + 2]
	movd xmm1, edx
	or edx, 65536
	rep bsf	edx, edx
	.cv_inline_site_id 370 within 369 inlined_at 43 154 0
	movd xmm2, edx
	pshuflw xmm2, xmm2, 0
	pshufd xmm2, xmm2, 0
	movdqa xmmword ptr [rbp + 2288], xmm2
	.cv_inline_site_id 371 within 309 inlined_at 12 39 0
	movzx eax, word ptr [rax + 2*r8 + 4]
	movd xmm2, eax
	or eax, 65536
	rep bsf	eax, eax
	.cv_inline_site_id 372 within 371 inlined_at 43 154 0
	movd xmm3, eax
	pshuflw xmm3, xmm3, 0
	pshufd xmm3, xmm3, 0
	movdqa xmmword ptr [rbp + 2272], xmm3
	lea rax, [r9 + 1]
	mov qword ptr [rbp + 2496], rax
	pshuflw xmm0, xmm0, 0
	pshufd xmm8, xmm0, 0
	pshuflw xmm0, xmm1, 0
	pshufd xmm9, xmm0, 0
	pshuflw xmm0, xmm2, 0
	pshufd xmm10, xmm0, 0
	mov r9d, 1
	jmp .LBB10_42
	.p2align	4, 0x90
.LBB10_110:
	mov r9, qword ptr [rbp + 2464]
	inc r9
	mov rbx, r13
	mov rcx, qword ptr [rbp + 2640]
	mov r13, qword ptr [rbp + 2720]
.LBB10_42:
	.cv_inline_site_id 373 within 309 inlined_at 12 42 0
	.cv_inline_site_id 374 within 373 inlined_at 41 843 0
	.cv_inline_site_id 375 within 374 inlined_at 41 752 0
	cmp r9, 9
	jae .LBB10_43
	.cv_inline_site_id 376 within 309 inlined_at 12 65 0
	.cv_inline_site_id 377 within 376 inlined_at 10 600 0
	.cv_inline_site_id 378 within 377 inlined_at 6 25 0
	movaps xmmword ptr [rbp + 2384], xmm7
	mov qword ptr [rbp + 2400], 0
	.cv_inline_site_id 379 within 309 inlined_at 12 65 0
	.cv_inline_site_id 380 within 379 inlined_at 10 600 0
	.cv_inline_site_id 381 within 380 inlined_at 6 25 0
	movaps xmmword ptr [rbp + 2416], xmm7
	mov qword ptr [rbp + 2432], 0
	.cv_inline_site_id 382 within 309 inlined_at 12 76 0
	.cv_inline_site_id 383 within 382 inlined_at 10 600 0
	.cv_inline_site_id 384 within 383 inlined_at 6 25 0
	movaps xmmword ptr [rbp - 80], xmm7
	mov qword ptr [rbp - 64], 0
	.cv_inline_site_id 385 within 309 inlined_at 12 76 0
	.cv_inline_site_id 386 within 385 inlined_at 10 600 0
	.cv_inline_site_id 387 within 386 inlined_at 6 25 0
	movaps xmmword ptr [rbp + 2016], xmm7
	mov qword ptr [rbp + 2032], 0
	.cv_inline_site_id 388 within 309 inlined_at 12 76 0
	.cv_inline_site_id 389 within 388 inlined_at 10 600 0
	.cv_inline_site_id 390 within 389 inlined_at 6 25 0
	movaps xmmword ptr [rbp + 2560], xmm7
	mov qword ptr [rbp + 2576], 0
	mov rax, qword ptr [rbp + 2400]
	mov qword ptr [rbp + 2608], rax
	movaps xmm0, xmmword ptr [rbp + 2384]
	movaps xmmword ptr [rbp + 2592], xmm0
	mov rax, qword ptr [rbp + 2432]
	mov qword ptr [rbp + 2336], rax
	movaps xmm0, xmmword ptr [rbp + 2416]
	movaps xmmword ptr [rbp + 2320], xmm0
	.cv_inline_site_id 391 within 309 inlined_at 12 84 0
	.cv_inline_site_id 392 within 391 inlined_at 10 600 0
	.cv_inline_site_id 393 within 392 inlined_at 6 25 0
	mov qword ptr [rbp + 2368], 0
	.cv_inline_site_id 394 within 309 inlined_at 12 84 0
	.cv_inline_site_id 395 within 394 inlined_at 10 1459 0
	.cv_inline_site_id 396 within 395 inlined_at 1 23 0
	.cv_inline_site_id 397 within 396 inlined_at 1 452 0
	.cv_inline_site_id 398 within 397 inlined_at 10 1766 0
	mov rax, qword ptr [rbp - 64]
	mov qword ptr [rsi - 72], rax
	movaps xmm0, xmmword ptr [rbp - 80]
	movups xmmword ptr [rsi - 88], xmm0
	mov rax, qword ptr [rbp + 2032]
	mov qword ptr [rsi + 16], rax
	movaps xmm0, xmmword ptr [rbp + 2016]
	movups xmmword ptr [rsi], xmm0
	mov rax, qword ptr [rbp + 2576]
	mov qword ptr [rsi + 328], rax
	movaps xmm0, xmmword ptr [rbp + 2560]
	movups xmmword ptr [rsi + 312], xmm0
	mov qword ptr [rbp + 968], 0
	mov qword ptr [rbp + 1008], rcx
	mov qword ptr [rbp + 1016], r12
	lea rdx, [rbp + 2448]
	mov qword ptr [rbp + 1024], rdx
	mov qword ptr [rbp + 1032], 0
	mov qword ptr [rbp + 1072], rcx
	mov qword ptr [rbp + 1080], r12
	mov qword ptr [rbp + 1088], rdx
	mov qword ptr [rbp + 1120], 0
	mov qword ptr [rbp + 1160], rcx
	mov qword ptr [rbp + 1168], r12
	mov qword ptr [rbp + 1176], rdx
	mov qword ptr [rbp + 1208], 0
	mov qword ptr [rbp + 1248], rcx
	mov qword ptr [rbp + 1256], r12
	mov qword ptr [rbp + 1264], rdx
	lea r8, [rbp + 2256]
	mov qword ptr [rbp + 1272], r8
	mov qword ptr [rbp + 1280], 0
	mov qword ptr [rbp + 1320], rcx
	mov qword ptr [rbp + 1328], r12
	mov qword ptr [rbp + 1336], rdx
	mov qword ptr [rbp + 1344], r8
	mov rax, qword ptr [rbp + 2608]
	mov qword ptr [rsi + 184], rax
	movaps xmm0, xmmword ptr [rbp + 2592]
	movups xmmword ptr [rsi + 168], xmm0
	mov qword ptr [rbp + 1376], 0
	mov qword ptr [rbp + 1416], rcx
	mov qword ptr [rbp + 1424], r12
	mov qword ptr [rbp + 1432], rdx
	mov qword ptr [rbp + 1440], r8
	mov rax, qword ptr [rbp + 2336]
	mov qword ptr [rsi + 280], rax
	movaps xmm0, xmmword ptr [rbp + 2320]
	movups xmmword ptr [rsi + 264], xmm0
	lea rax, [rbp + 2304]
	mov qword ptr [rbp + 1472], rax
	lea rax, [rbp + 2288]
	mov qword ptr [rbp + 1480], rax
	lea rax, [rbp + 2272]
	mov qword ptr [rbp + 1488], rax
	lea rax, [rbp + 2240]
	mov qword ptr [rbp + 1520], rax
	lea rax, [rbp + 2224]
	mov qword ptr [rbp + 1528], rax
	lea rax, [rbp + 2208]
	mov qword ptr [rbp + 1536], rax
	mov qword ptr [rbp + 1544], rcx
	mov rax, qword ptr [rbp + 2480]
	mov qword ptr [rbp + 1552], rax
	mov rax, qword ptr [rbp + 2488]
	mov qword ptr [rbp + 1560], rax
	mov rax, qword ptr [rbp + 2472]
	mov qword ptr [rbp + 1568], rax
	mov rax, qword ptr [rbp + 2368]
	mov qword ptr [rsi + 408], rax
	mov qword ptr [rbp + 2464], r9
	mov qword ptr [rbp + 2448], r9
	.cv_inline_site_id 399 within 309 inlined_at 12 59 0
	.cv_inline_site_id 400 within 399 inlined_at 43 154 0
	movaps xmmword ptr [rbp + 2256], xmm6
	.cv_inline_site_id 401 within 309 inlined_at 12 72 0
	.cv_inline_site_id 402 within 401 inlined_at 43 154 0
	movdqa xmmword ptr [rbp + 2240], xmm8
	.cv_inline_site_id 403 within 309 inlined_at 12 73 0
	.cv_inline_site_id 404 within 403 inlined_at 43 154 0
	movdqa xmmword ptr [rbp + 2224], xmm9
	.cv_inline_site_id 405 within 309 inlined_at 12 74 0
	.cv_inline_site_id 406 within 405 inlined_at 43 154 0
	movdqa xmmword ptr [rbp + 2208], xmm10
	movaps xmmword ptr [rbp + 2352], xmm7
	movdqa xmm0, xmmword ptr [rbp + 2352]
	movdqu xmmword ptr [rsi + 392], xmm0
	mov qword ptr [rbp + 1600], rdx
	lea rax, [rbp + 2456]
	mov qword ptr [rbp + 1608], rax
	mov qword ptr [rbp + 1616], 0
	mov r8d, 192
	mov rcx, r15
	lea r14, [rbp - 80]
	mov rdx, r14
	call memcpy
	mov qword ptr [rbp + 1816], 0
	mov r8d, 192
	lea rcx, [rbp + 1824]
	lea rdx, [rbp + 2016]
	call memcpy
	mov qword ptr [rbp + 2688], r13
	mov rax, qword ptr [rbp + 2712]
	mov qword ptr [rbp + 2680], rax
	.cv_inline_site_id 407 within 309 inlined_at 12 100 0
	.cv_inline_site_id 408 within 407 inlined_at 10 2000 0
	.cv_inline_site_id 409 within 408 inlined_at 38 2986 0
	.cv_inline_site_id 410 within 409 inlined_at 45 33 0
	mov rcx, r14
	lea rdx, [rbp + 968]
	call <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
	test byte ptr [rbp - 80], 1
	mov qword ptr [rbp + 2720], r13
	je .LBB10_46
	.cv_inline_site_id 411 within 410 inlined_at 44 27 0
	.cv_inline_site_id 412 within 411 inlined_at 1 69 0
	.cv_inline_site_id 413 within 412 inlined_at 1 617 0
	mov rax, qword ptr [rbp + 1616]
	test rax, rax
	.cv_inline_site_id 414 within 412 inlined_at 1 617 0
	je .LBB10_49
	.cv_inline_site_id 415 within 414 inlined_at 2 1166 0
	.cv_inline_site_id 416 within 415 inlined_at 13 250 0
	.cv_inline_site_id 417 within 416 inlined_at 5 112 0
	.cv_inline_site_id 418 within 417 inlined_at 6 91 0
	.cv_inline_site_id 419 within 418 inlined_at 6 264 0
	.cv_inline_site_id 420 within 419 inlined_at 6 599 0
	.cv_inline_site_id 421 within 420 inlined_at 6 91 0
	.cv_inline_site_id 422 within 421 inlined_at 6 264 0
	.cv_inline_site_id 423 within 422 inlined_at 6 599 0
	.cv_inline_site_id 424 within 423 inlined_at 6 91 0
	mov rcx, qword ptr [rbp + 1624]
	mov rdx, qword ptr [rbp + 1656]
	.cv_inline_site_id 425 within 424 inlined_at 6 264 0
	.cv_inline_site_id 426 within 425 inlined_at 6 599 0
	.cv_inline_site_id 427 within 426 inlined_at 8 57 0
	.cv_inline_site_id 428 within 427 inlined_at 4 902 0
	sub rcx, rax
	.cv_inline_site_id 429 within 424 inlined_at 6 264 0
	.cv_inline_site_id 430 within 429 inlined_at 6 599 0
	.cv_inline_site_id 431 within 430 inlined_at 26 255 0
	.cv_inline_site_id 432 within 431 inlined_at 26 365 0
	.cv_inline_site_id 433 within 432 inlined_at 25 48 0
	sub rdx, qword ptr [rbp + 1648]
	shr rcx, 5
	cmp rcx, rdx
	cmovb rdx, rcx
	mov rax, qword ptr [rbp + 1712]
	.cv_inline_site_id 434 within 421 inlined_at 6 264 0
	.cv_inline_site_id 435 within 434 inlined_at 6 599 0
	.cv_inline_site_id 436 within 435 inlined_at 26 255 0
	.cv_inline_site_id 437 within 436 inlined_at 26 365 0
	.cv_inline_site_id 438 within 437 inlined_at 25 48 0
	sub rax, qword ptr [rbp + 1704]
	cmp rdx, rax
	cmovb rax, rdx
	mov rcx, qword ptr [rbp + 1768]
	.cv_inline_site_id 439 within 418 inlined_at 6 264 0
	.cv_inline_site_id 440 within 439 inlined_at 6 599 0
	.cv_inline_site_id 441 within 440 inlined_at 26 255 0
	.cv_inline_site_id 442 within 441 inlined_at 26 365 0
	.cv_inline_site_id 443 within 442 inlined_at 25 48 0
	sub rcx, qword ptr [rbp + 1760]
	cmp rax, rcx
	cmovb rcx, rax
	.cv_inline_site_id 444 within 412 inlined_at 1 618 0
	mov rax, qword ptr [rbp + 1816]
	test rax, rax
	.cv_inline_site_id 445 within 412 inlined_at 1 618 0
	je .LBB10_52
.LBB10_53:
	.cv_inline_site_id 446 within 445 inlined_at 2 1166 0
	.cv_inline_site_id 447 within 446 inlined_at 13 250 0
	.cv_inline_site_id 448 within 447 inlined_at 5 112 0
	.cv_inline_site_id 449 within 448 inlined_at 6 91 0
	.cv_inline_site_id 450 within 449 inlined_at 6 264 0
	.cv_inline_site_id 451 within 450 inlined_at 6 599 0
	.cv_inline_site_id 452 within 451 inlined_at 6 91 0
	.cv_inline_site_id 453 within 452 inlined_at 6 264 0
	.cv_inline_site_id 454 within 453 inlined_at 6 599 0
	.cv_inline_site_id 455 within 454 inlined_at 6 91 0
	mov rdx, qword ptr [rbp + 1824]
	mov r8, qword ptr [rbp + 1856]
	.cv_inline_site_id 456 within 455 inlined_at 6 264 0
	.cv_inline_site_id 457 within 456 inlined_at 6 599 0
	.cv_inline_site_id 458 within 457 inlined_at 8 57 0
	.cv_inline_site_id 459 within 458 inlined_at 4 902 0
	sub rdx, rax
	.cv_inline_site_id 460 within 455 inlined_at 6 264 0
	.cv_inline_site_id 461 within 460 inlined_at 6 599 0
	.cv_inline_site_id 462 within 461 inlined_at 26 255 0
	.cv_inline_site_id 463 within 462 inlined_at 26 365 0
	.cv_inline_site_id 464 within 463 inlined_at 25 48 0
	sub r8, qword ptr [rbp + 1848]
	shr rdx, 5
	cmp rdx, r8
	cmovb r8, rdx
	mov rdx, qword ptr [rbp + 1912]
	.cv_inline_site_id 465 within 452 inlined_at 6 264 0
	.cv_inline_site_id 466 within 465 inlined_at 6 599 0
	.cv_inline_site_id 467 within 466 inlined_at 26 255 0
	.cv_inline_site_id 468 within 467 inlined_at 26 365 0
	.cv_inline_site_id 469 within 468 inlined_at 25 48 0
	sub rdx, qword ptr [rbp + 1904]
	cmp r8, rdx
	cmovb rdx, r8
	mov rax, qword ptr [rbp + 1968]
	.cv_inline_site_id 470 within 449 inlined_at 6 264 0
	.cv_inline_site_id 471 within 470 inlined_at 6 599 0
	.cv_inline_site_id 472 within 471 inlined_at 26 255 0
	.cv_inline_site_id 473 within 472 inlined_at 26 365 0
	.cv_inline_site_id 474 within 473 inlined_at 25 48 0
	sub rax, qword ptr [rbp + 1960]
	cmp rdx, rax
	cmovb rax, rdx
	.cv_inline_site_id 475 within 412 inlined_at 1 619 0
	add rax, rcx
	.cv_inline_site_id 476 within 412 inlined_at 1 631 0
	cmp dword ptr [rbp + 968], 2
	jne .LBB10_55
	jmp .LBB10_57
	.p2align	4, 0x90
.LBB10_46:
	.cv_inline_site_id 477 within 309 inlined_at 12 101 0
	.cv_inline_site_id 478 within 477 inlined_at 38 3054 0
	.cv_inline_site_id 479 within 478 inlined_at 46 33 0
	mov eax, 8
	mov qword ptr [rbp + 2704], rax
	xor r14d, r14d
	xor r13d, r13d
	mov qword ptr [rbp + 2696], 0
	add r13, rbx
	.cv_inline_site_id 480 within 479 inlined_at 38 2156 0
	.cv_inline_site_id 481 within 480 inlined_at 38 973 0
	jmp .LBB10_108
.LBB10_49:
	xor ecx, ecx
	mov rax, qword ptr [rbp + 1816]
	test rax, rax
	jne .LBB10_53
.LBB10_52:
	xor eax, eax
	add rax, rcx
	cmp dword ptr [rbp + 968], 2
	je .LBB10_57
.LBB10_55:
	.cv_inline_site_id 482 within 476 inlined_at 3 77 0
	.cv_inline_site_id 483 within 482 inlined_at 5 112 0
	.cv_inline_site_id 484 within 483 inlined_at 6 91 0
	mov rcx, qword ptr [rbp + 1552]
	.cv_inline_site_id 485 within 484 inlined_at 6 223 0
	.cv_inline_site_id 486 within 485 inlined_at 7 2369 0
	.cv_inline_site_id 487 within 486 inlined_at 8 57 0
	.cv_inline_site_id 488 within 487 inlined_at 4 902 0
	sub rcx, qword ptr [rbp + 1544]
	shr rcx, 8
	.cv_inline_site_id 489 within 484 inlined_at 6 224 0
	.cv_inline_site_id 490 within 489 inlined_at 5 112 0
	.cv_inline_site_id 491 within 490 inlined_at 6 91 0
	.cv_inline_site_id 492 within 491 inlined_at 6 223 0
	.cv_inline_site_id 493 within 492 inlined_at 6 91 0
	.cv_inline_site_id 494 within 493 inlined_at 6 223 0
	.cv_inline_site_id 495 within 494 inlined_at 6 91 0
	mov rdx, qword ptr [rbp + 1016]
	.cv_inline_site_id 496 within 495 inlined_at 6 223 0
	.cv_inline_site_id 497 within 496 inlined_at 5 112 0
	.cv_inline_site_id 498 within 497 inlined_at 9 71 0
	.cv_inline_site_id 499 within 498 inlined_at 5 112 0
	.cv_inline_site_id 500 within 499 inlined_at 8 57 0
	.cv_inline_site_id 501 within 500 inlined_at 4 902 0
	sub rdx, qword ptr [rbp + 1008]
	mov r8, qword ptr [rbp + 1080]
	shr rdx, 8
	.cv_inline_site_id 502 within 495 inlined_at 6 224 0
	.cv_inline_site_id 503 within 502 inlined_at 5 112 0
	.cv_inline_site_id 504 within 503 inlined_at 9 71 0
	.cv_inline_site_id 505 within 504 inlined_at 5 112 0
	.cv_inline_site_id 506 within 505 inlined_at 8 57 0
	.cv_inline_site_id 507 within 506 inlined_at 4 902 0
	sub r8, qword ptr [rbp + 1072]
	shr r8, 8
	cmp rdx, r8
	cmovb r8, rdx
	mov rdx, qword ptr [rbp + 1168]
	.cv_inline_site_id 508 within 493 inlined_at 6 224 0
	.cv_inline_site_id 509 within 508 inlined_at 5 112 0
	.cv_inline_site_id 510 within 509 inlined_at 9 71 0
	.cv_inline_site_id 511 within 510 inlined_at 5 112 0
	.cv_inline_site_id 512 within 511 inlined_at 8 57 0
	.cv_inline_site_id 513 within 512 inlined_at 4 902 0
	sub rdx, qword ptr [rbp + 1160]
	shr rdx, 8
	cmp r8, rdx
	cmovb rdx, r8
	.cv_inline_site_id 514 within 491 inlined_at 6 224 0
	.cv_inline_site_id 515 within 514 inlined_at 5 112 0
	.cv_inline_site_id 516 within 515 inlined_at 6 91 0
	.cv_inline_site_id 517 within 516 inlined_at 6 223 0
	.cv_inline_site_id 518 within 517 inlined_at 6 91 0
	mov r8, qword ptr [rbp + 1256]
	.cv_inline_site_id 519 within 518 inlined_at 6 223 0
	.cv_inline_site_id 520 within 519 inlined_at 5 112 0
	.cv_inline_site_id 521 within 520 inlined_at 5 112 0
	.cv_inline_site_id 522 within 521 inlined_at 9 71 0
	.cv_inline_site_id 523 within 522 inlined_at 5 112 0
	.cv_inline_site_id 524 within 523 inlined_at 8 57 0
	.cv_inline_site_id 525 within 524 inlined_at 4 902 0
	sub r8, qword ptr [rbp + 1248]
	shr r8, 8
	mov r9, qword ptr [rbp + 1328]
	.cv_inline_site_id 526 within 518 inlined_at 6 224 0
	.cv_inline_site_id 527 within 526 inlined_at 5 112 0
	.cv_inline_site_id 528 within 527 inlined_at 5 112 0
	.cv_inline_site_id 529 within 528 inlined_at 9 71 0
	.cv_inline_site_id 530 within 529 inlined_at 5 112 0
	.cv_inline_site_id 531 within 530 inlined_at 8 57 0
	.cv_inline_site_id 532 within 531 inlined_at 4 902 0
	sub r9, qword ptr [rbp + 1320]
	shr r9, 8
	cmp r8, r9
	cmovb r9, r8
	mov r8, qword ptr [rbp + 1424]
	.cv_inline_site_id 533 within 516 inlined_at 6 224 0
	.cv_inline_site_id 534 within 533 inlined_at 5 112 0
	.cv_inline_site_id 535 within 534 inlined_at 5 112 0
	.cv_inline_site_id 536 within 535 inlined_at 9 71 0
	.cv_inline_site_id 537 within 536 inlined_at 5 112 0
	.cv_inline_site_id 538 within 537 inlined_at 8 57 0
	.cv_inline_site_id 539 within 538 inlined_at 4 902 0
	sub r8, qword ptr [rbp + 1416]
	shr r8, 8
	cmp r9, r8
	cmovb r8, r9
	cmp rdx, r8
	cmovb r8, rdx
	cmp rcx, r8
	cmovb r8, rcx
	test r8, r8
	je .LBB10_57
	lea rcx, [rbp + 2016]
	xor edx, edx
	jmp .LBB10_58
	.p2align	4, 0x90
.LBB10_57:
	lea rcx, [rbp + 2560]
	mov rdx, rax
.LBB10_58:
	.cv_inline_site_id 540 within 410 inlined_at 44 30 0
	.cv_inline_site_id 541 within 540 inlined_at 38 480 0
	.cv_inline_site_id 542 within 541 inlined_at 38 698 0
	.cv_inline_site_id 543 within 542 inlined_at 22 157 0
	.cv_inline_site_id 544 within 543 inlined_at 22 219 0
	.cv_inline_site_id 545 within 544 inlined_at 35 437 0
	mov qword ptr [rcx], rdx
	cmp rax, 4
	mov r14d, 3
	cmovae r14, rax
	inc r14
	mov qword ptr [rbp + 2696], r14
	shl r14, 5
	movabs rcx, 288230376151711742
	cmp rax, rcx
	ja .LBB10_59
	.cv_inline_site_id 546 within 543 inlined_at 22 229 0
	.cv_inline_site_id 547 within 546 inlined_at 24 241 0
	.cv_inline_site_id 548 within 547 inlined_at 24 181 0
	.cv_inline_site_id 549 within 548 inlined_at 24 96 0
	mov r15, rsi
	mov rsi, rbx
	mov rbx, rdi
	mov rdi, r12
	movzx eax, byte ptr [rip + __rust_no_alloc_shim_is_unstable]
	mov r13d, 8
	mov edx, 8
	mov rcx, r14
	call __rust_alloc
	test rax, rax
	je .LBB10_60
	.cv_inline_site_id 550 within 410 inlined_at 44 33 0
	mov r12, rax
	lea rax, [rbp - 72]
	movdqu xmm0, xmmword ptr [rax]
	movdqu xmm1, xmmword ptr [rax + 16]
	movdqu xmmword ptr [r12 + 16], xmm1
	movdqu xmmword ptr [r12], xmm0
	mov r8d, 1048
	lea r14, [rbp - 80]
	mov rcx, r14
	lea rdx, [rbp + 968]
	call memcpy
	mov rax, qword ptr [rbp + 2696]
	mov qword ptr [rbp + 2624], rax
	mov qword ptr [rbp + 2704], r12
	mov qword ptr [rbp + 2616], r12
	.cv_inline_site_id 551 within 410 inlined_at 44 41 0
	.cv_inline_site_id 552 within 551 inlined_at 46 17 0
	lea rcx, [rbp + 2016]
	mov rdx, r14
	call <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
	mov r12, rdi
	mov rdi, rbx
	mov rbx, rsi
	mov rsi, r15
	mov r13d, 1
	cmp dword ptr [rbp + 2016], 1
	jne .LBB10_64
	mov r15, qword ptr [rbp + 2696]
	.p2align	4, 0x90
.LBB10_66:
	cmp r13, r15
	jne .LBB10_67
	.cv_inline_site_id 553 within 552 inlined_at 38 3093 0
	.cv_inline_site_id 554 within 553 inlined_at 1 69 0
	.cv_inline_site_id 555 within 554 inlined_at 1 617 0
	mov rax, qword ptr [rbp + 568]
	test rax, rax
	.cv_inline_site_id 556 within 554 inlined_at 1 617 0
	je .LBB10_69
	.cv_inline_site_id 557 within 556 inlined_at 2 1166 0
	.cv_inline_site_id 558 within 557 inlined_at 13 250 0
	.cv_inline_site_id 559 within 558 inlined_at 5 112 0
	.cv_inline_site_id 560 within 559 inlined_at 6 91 0
	.cv_inline_site_id 561 within 560 inlined_at 6 264 0
	.cv_inline_site_id 562 within 561 inlined_at 6 599 0
	.cv_inline_site_id 563 within 562 inlined_at 6 91 0
	.cv_inline_site_id 564 within 563 inlined_at 6 264 0
	.cv_inline_site_id 565 within 564 inlined_at 6 599 0
	.cv_inline_site_id 566 within 565 inlined_at 6 91 0
	mov rcx, qword ptr [rbp + 576]
	mov rdx, qword ptr [rbp + 608]
	.cv_inline_site_id 567 within 566 inlined_at 6 264 0
	.cv_inline_site_id 568 within 567 inlined_at 6 599 0
	.cv_inline_site_id 569 within 568 inlined_at 8 57 0
	.cv_inline_site_id 570 within 569 inlined_at 4 902 0
	sub rcx, rax
	.cv_inline_site_id 571 within 566 inlined_at 6 264 0
	.cv_inline_site_id 572 within 571 inlined_at 6 599 0
	.cv_inline_site_id 573 within 572 inlined_at 26 255 0
	.cv_inline_site_id 574 within 573 inlined_at 26 365 0
	.cv_inline_site_id 575 within 574 inlined_at 25 48 0
	sub rdx, qword ptr [rbp + 600]
	shr rcx, 5
	cmp rcx, rdx
	cmovb rdx, rcx
	mov rcx, qword ptr [rbp + 664]
	.cv_inline_site_id 576 within 563 inlined_at 6 264 0
	.cv_inline_site_id 577 within 576 inlined_at 6 599 0
	.cv_inline_site_id 578 within 577 inlined_at 26 255 0
	.cv_inline_site_id 579 within 578 inlined_at 26 365 0
	.cv_inline_site_id 580 within 579 inlined_at 25 48 0
	sub rcx, qword ptr [rbp + 656]
	cmp rdx, rcx
	cmovb rcx, rdx
	mov rax, qword ptr [rbp + 720]
	.cv_inline_site_id 581 within 560 inlined_at 6 264 0
	.cv_inline_site_id 582 within 581 inlined_at 6 599 0
	.cv_inline_site_id 583 within 582 inlined_at 26 255 0
	.cv_inline_site_id 584 within 583 inlined_at 26 365 0
	.cv_inline_site_id 585 within 584 inlined_at 25 48 0
	sub rax, qword ptr [rbp + 712]
	cmp rcx, rax
	cmovb rax, rcx
	.cv_inline_site_id 586 within 554 inlined_at 1 618 0
	mov rcx, qword ptr [rbp + 768]
	test rcx, rcx
	.cv_inline_site_id 587 within 554 inlined_at 1 618 0
	je .LBB10_72
.LBB10_73:
	.cv_inline_site_id 588 within 587 inlined_at 2 1166 0
	.cv_inline_site_id 589 within 588 inlined_at 13 250 0
	.cv_inline_site_id 590 within 589 inlined_at 5 112 0
	.cv_inline_site_id 591 within 590 inlined_at 6 91 0
	.cv_inline_site_id 592 within 591 inlined_at 6 264 0
	.cv_inline_site_id 593 within 592 inlined_at 6 599 0
	.cv_inline_site_id 594 within 593 inlined_at 6 91 0
	.cv_inline_site_id 595 within 594 inlined_at 6 264 0
	.cv_inline_site_id 596 within 595 inlined_at 6 599 0
	.cv_inline_site_id 597 within 596 inlined_at 6 91 0
	mov rdx, qword ptr [rbp + 776]
	mov r8, qword ptr [rbp + 808]
	.cv_inline_site_id 598 within 597 inlined_at 6 264 0
	.cv_inline_site_id 599 within 598 inlined_at 6 599 0
	.cv_inline_site_id 600 within 599 inlined_at 8 57 0
	.cv_inline_site_id 601 within 600 inlined_at 4 902 0
	sub rdx, rcx
	.cv_inline_site_id 602 within 597 inlined_at 6 264 0
	.cv_inline_site_id 603 within 602 inlined_at 6 599 0
	.cv_inline_site_id 604 within 603 inlined_at 26 255 0
	.cv_inline_site_id 605 within 604 inlined_at 26 365 0
	.cv_inline_site_id 606 within 605 inlined_at 25 48 0
	sub r8, qword ptr [rbp + 800]
	shr rdx, 5
	cmp rdx, r8
	cmovb r8, rdx
	mov rcx, qword ptr [rbp + 864]
	.cv_inline_site_id 607 within 594 inlined_at 6 264 0
	.cv_inline_site_id 608 within 607 inlined_at 6 599 0
	.cv_inline_site_id 609 within 608 inlined_at 26 255 0
	.cv_inline_site_id 610 within 609 inlined_at 26 365 0
	.cv_inline_site_id 611 within 610 inlined_at 25 48 0
	sub rcx, qword ptr [rbp + 856]
	cmp r8, rcx
	cmovb rcx, r8
	mov r14, qword ptr [rbp + 920]
	.cv_inline_site_id 612 within 591 inlined_at 6 264 0
	.cv_inline_site_id 613 within 612 inlined_at 6 599 0
	.cv_inline_site_id 614 within 613 inlined_at 26 255 0
	.cv_inline_site_id 615 within 614 inlined_at 26 365 0
	.cv_inline_site_id 616 within 615 inlined_at 25 48 0
	sub r14, qword ptr [rbp + 912]
	cmp rcx, r14
	cmovb r14, rcx
	.cv_inline_site_id 617 within 554 inlined_at 1 619 0
	add r14, rax
	.cv_inline_site_id 618 within 554 inlined_at 1 631 0
	cmp dword ptr [rbp - 80], 2
	jne .LBB10_75
	jmp .LBB10_77
	.p2align	4, 0x90
.LBB10_67:
	mov rcx, qword ptr [rbp + 2704]
	jmp .LBB10_89
.LBB10_69:
	xor eax, eax
	mov rcx, qword ptr [rbp + 768]
	test rcx, rcx
	jne .LBB10_73
.LBB10_72:
	xor r14d, r14d
	add r14, rax
	cmp dword ptr [rbp - 80], 2
	je .LBB10_77
.LBB10_75:
	.cv_inline_site_id 619 within 618 inlined_at 3 77 0
	.cv_inline_site_id 620 within 619 inlined_at 5 112 0
	.cv_inline_site_id 621 within 620 inlined_at 6 91 0
	mov rax, qword ptr [rbp + 504]
	.cv_inline_site_id 622 within 621 inlined_at 6 223 0
	.cv_inline_site_id 623 within 622 inlined_at 7 2369 0
	.cv_inline_site_id 624 within 623 inlined_at 8 57 0
	.cv_inline_site_id 625 within 624 inlined_at 4 902 0
	sub rax, qword ptr [rbp + 496]
	shr rax, 8
	.cv_inline_site_id 626 within 621 inlined_at 6 224 0
	.cv_inline_site_id 627 within 626 inlined_at 5 112 0
	.cv_inline_site_id 628 within 627 inlined_at 6 91 0
	.cv_inline_site_id 629 within 628 inlined_at 6 223 0
	.cv_inline_site_id 630 within 629 inlined_at 6 91 0
	.cv_inline_site_id 631 within 630 inlined_at 6 223 0
	.cv_inline_site_id 632 within 631 inlined_at 6 91 0
	mov rcx, qword ptr [rbp - 32]
	.cv_inline_site_id 633 within 632 inlined_at 6 223 0
	.cv_inline_site_id 634 within 633 inlined_at 5 112 0
	.cv_inline_site_id 635 within 634 inlined_at 9 71 0
	.cv_inline_site_id 636 within 635 inlined_at 5 112 0
	.cv_inline_site_id 637 within 636 inlined_at 8 57 0
	.cv_inline_site_id 638 within 637 inlined_at 4 902 0
	sub rcx, qword ptr [rbp - 40]
	mov rdx, qword ptr [rbp + 32]
	shr rcx, 8
	.cv_inline_site_id 639 within 632 inlined_at 6 224 0
	.cv_inline_site_id 640 within 639 inlined_at 5 112 0
	.cv_inline_site_id 641 within 640 inlined_at 9 71 0
	.cv_inline_site_id 642 within 641 inlined_at 5 112 0
	.cv_inline_site_id 643 within 642 inlined_at 8 57 0
	.cv_inline_site_id 644 within 643 inlined_at 4 902 0
	sub rdx, qword ptr [rbp + 24]
	shr rdx, 8
	cmp rcx, rdx
	cmovb rdx, rcx
	mov rcx, qword ptr [rbp + 120]
	.cv_inline_site_id 645 within 630 inlined_at 6 224 0
	.cv_inline_site_id 646 within 645 inlined_at 5 112 0
	.cv_inline_site_id 647 within 646 inlined_at 9 71 0
	.cv_inline_site_id 648 within 647 inlined_at 5 112 0
	.cv_inline_site_id 649 within 648 inlined_at 8 57 0
	.cv_inline_site_id 650 within 649 inlined_at 4 902 0
	sub rcx, qword ptr [rbp + 112]
	shr rcx, 8
	cmp rdx, rcx
	cmovb rcx, rdx
	.cv_inline_site_id 651 within 628 inlined_at 6 224 0
	.cv_inline_site_id 652 within 651 inlined_at 5 112 0
	.cv_inline_site_id 653 within 652 inlined_at 6 91 0
	.cv_inline_site_id 654 within 653 inlined_at 6 223 0
	.cv_inline_site_id 655 within 654 inlined_at 6 91 0
	mov rdx, qword ptr [rbp + 208]
	.cv_inline_site_id 656 within 655 inlined_at 6 223 0
	.cv_inline_site_id 657 within 656 inlined_at 5 112 0
	.cv_inline_site_id 658 within 657 inlined_at 5 112 0
	.cv_inline_site_id 659 within 658 inlined_at 9 71 0
	.cv_inline_site_id 660 within 659 inlined_at 5 112 0
	.cv_inline_site_id 661 within 660 inlined_at 8 57 0
	.cv_inline_site_id 662 within 661 inlined_at 4 902 0
	sub rdx, qword ptr [rbp + 200]
	shr rdx, 8
	mov r8, qword ptr [rbp + 280]
	.cv_inline_site_id 663 within 655 inlined_at 6 224 0
	.cv_inline_site_id 664 within 663 inlined_at 5 112 0
	.cv_inline_site_id 665 within 664 inlined_at 5 112 0
	.cv_inline_site_id 666 within 665 inlined_at 9 71 0
	.cv_inline_site_id 667 within 666 inlined_at 5 112 0
	.cv_inline_site_id 668 within 667 inlined_at 8 57 0
	.cv_inline_site_id 669 within 668 inlined_at 4 902 0
	sub r8, qword ptr [rbp + 272]
	shr r8, 8
	cmp rdx, r8
	cmovb r8, rdx
	mov rdx, qword ptr [rbp + 376]
	.cv_inline_site_id 670 within 653 inlined_at 6 224 0
	.cv_inline_site_id 671 within 670 inlined_at 5 112 0
	.cv_inline_site_id 672 within 671 inlined_at 5 112 0
	.cv_inline_site_id 673 within 672 inlined_at 9 71 0
	.cv_inline_site_id 674 within 673 inlined_at 5 112 0
	.cv_inline_site_id 675 within 674 inlined_at 8 57 0
	.cv_inline_site_id 676 within 675 inlined_at 4 902 0
	sub rdx, qword ptr [rbp + 368]
	shr rdx, 8
	cmp r8, rdx
	cmovb rdx, r8
	cmp rcx, rdx
	cmovb rdx, rcx
	cmp rax, rdx
	cmovb rdx, rax
	test rdx, rdx
	je .LBB10_77
	lea rax, [rbp + 2560]
	xor ecx, ecx
	jmp .LBB10_78
	.p2align	4, 0x90
.LBB10_77:
	lea rax, [rbp + 2592]
	mov rcx, r14
.LBB10_78:
	.cv_inline_site_id 677 within 552 inlined_at 38 3094 0
	mov qword ptr [rax], rcx
	inc r14
	.cv_inline_site_id 678 within 677 inlined_at 38 973 0
	.cv_inline_site_id 679 within 678 inlined_at 22 354 0
	.cv_inline_site_id 680 within 679 inlined_at 22 348 0
	.cv_inline_site_id 681 within 680 inlined_at 22 475 0
	add r14, r15
	jb .LBB10_79
	mov rcx, r15
	lea rax, [r15 + r15]
	cmp rax, r14
	cmova r14, rax
	xor edx, edx
	mov rax, r14
	shr rax, 58
	sete al
	cmp r14, 5
	jae .LBB10_82
	mov r14d, 4
.LBB10_82:
	.cv_inline_site_id 682 within 680 inlined_at 22 485 0
	test rcx, rcx
	je .LBB10_83
	.cv_inline_site_id 683 within 682 inlined_at 22 309 0
	shl rcx, 5
	mov r8, qword ptr [rbp + 2704]
	mov qword ptr [rbp + 2560], r8
	mov qword ptr [rbp + 2576], rcx
	mov ecx, 8
	jmp .LBB10_85
.LBB10_83:
	xor ecx, ecx
.LBB10_85:
	mov r8, r14
	shl r8, 5
	mov dl, al
	shl edx, 3
	mov qword ptr [rbp + 2568], rcx
	lea rcx, [rbp + 2592]
	lea r9, [rbp + 2560]
	call alloc::raw_vec::finish_grow
	.cv_inline_site_id 684 within 680 inlined_at 22 485 0
	cmp dword ptr [rbp + 2592], 1
	je .LBB10_86
	mov rcx, qword ptr [rbp + 2600]
	mov r15, r14
.LBB10_89:
	.cv_inline_site_id 685 within 552 inlined_at 38 3097 0
	mov rax, r13
	shl rax, 5
	movdqu xmm0, xmmword ptr [rdi]
	movdqu xmm1, xmmword ptr [rdi + 16]
	movdqu xmmword ptr [rcx + rax + 16], xmm1
	movdqu xmmword ptr [rcx + rax], xmm0
	mov r14, r15
	mov qword ptr [rbp + 2624], r15
	mov qword ptr [rbp + 2704], rcx
	mov qword ptr [rbp + 2616], rcx
	lea rcx, [rbp + 2016]
	lea rdx, [rbp - 80]
	call <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
	inc r13
	test byte ptr [rbp + 2016], 1
	mov r15, r14
	jne .LBB10_66
	jmp .LBB10_91
	.p2align	4, 0x90
.LBB10_64:
	mov r15, qword ptr [rbp + 2696]
.LBB10_91:
	.cv_inline_site_id 686 within 478 inlined_at 46 33 0
	.cv_inline_site_id 687 within 686 inlined_at 47 84 0
	.cv_inline_site_id 688 within 687 inlined_at 27 117 0
	.cv_inline_site_id 689 within 688 inlined_at 47 233 0
	.cv_inline_site_id 690 within 689 inlined_at 4 902 0
	mov r14, r13
	shl r14, 5
	mov rax, qword ptr [rbp + 2720]
	.cv_inline_site_id 691 within 481 inlined_at 22 353 0
	.cv_inline_site_id 692 within 691 inlined_at 22 443 0
	sub rax, rbx
	cmp rax, r13
	mov qword ptr [rbp + 2696], r15
	jae .LBB10_92
	.cv_inline_site_id 693 within 481 inlined_at 22 354 0
	.cv_inline_site_id 694 within 693 inlined_at 22 348 0
	.cv_inline_site_id 695 within 694 inlined_at 22 475 0
	add r13, rbx
	jb .LBB10_100
	lea rsi, [rbp + 1624]
	mov r8, qword ptr [rbp + 2720]
	lea r15, [r8 + r8]
	cmp r15, r13
	cmovbe r15, r13
	xor edx, edx
	mov rax, r15
	shr rax, 58
	sete al
	cmp r15, 5
	mov ecx, 4
	cmovb r15, rcx
	.cv_inline_site_id 696 within 694 inlined_at 22 485 0
	test r8, r8
	je .LBB10_102
	.cv_inline_site_id 697 within 696 inlined_at 22 309 0
	mov rcx, r8
	shl rcx, 5
	mov r8, qword ptr [rbp + 2712]
	mov qword ptr [rbp - 80], r8
	mov qword ptr [rbp - 64], rcx
	mov ecx, 8
	jmp .LBB10_104
	.p2align	4, 0x90
.LBB10_92:
	lea r15, [rbp + 1624]
	add r13, rbx
	jmp .LBB10_108
.LBB10_102:
	xor ecx, ecx
.LBB10_104:
	mov r8, r15
	shl r8, 5
	mov dl, al
	shl edx, 3
	mov qword ptr [rbp - 72], rcx
	lea rcx, [rbp + 2016]
	lea r9, [rbp - 80]
	call alloc::raw_vec::finish_grow
	.cv_inline_site_id 698 within 694 inlined_at 22 485 0
	cmp dword ptr [rbp + 2016], 1
	je .LBB10_105
	mov rax, qword ptr [rbp + 2024]
	mov qword ptr [rbp + 2712], rax
	mov qword ptr [rbp + 2720], r15
	mov r15, rsi
	lea rsi, [rbp + 1184]
.LBB10_108:
	.cv_inline_site_id 699 within 479 inlined_at 38 2158 0
	shl rbx, 5
	add rbx, qword ptr [rbp + 2712]
	mov rcx, rbx
	mov rbx, qword ptr [rbp + 2704]
	mov rdx, rbx
	mov r8, r14
	call memcpy
	mov rdx, qword ptr [rbp + 2696]
	.cv_inline_site_id 700 within 478 inlined_at 46 36 0
	.cv_inline_site_id 701 within 700 inlined_at 20 536 0
	.cv_inline_site_id 702 within 701 inlined_at 47 507 0
	.cv_inline_site_id 703 within 702 inlined_at 20 536 0
	.cv_inline_site_id 704 within 703 inlined_at 47 496 0
	.cv_inline_site_id 705 within 704 inlined_at 20 536 0
	.cv_inline_site_id 706 within 705 inlined_at 22 598 0
	test rdx, rdx
	je .LBB10_110
	.cv_inline_site_id 707 within 706 inlined_at 22 309 0
	shl rdx, 5
	.cv_inline_site_id 708 within 705 inlined_at 22 599 0
	.cv_inline_site_id 709 within 708 inlined_at 24 254 0
	mov r8d, 8
	mov rcx, rbx
	call __rust_dealloc
	jmp .LBB10_110
	.p2align	4, 0x90
.LBB10_26:
	.cv_inline_site_id 710 within 334 inlined_at 21 1770 0
	.cv_inline_site_id 711 within 710 inlined_at 21 203 0
	.cv_inline_site_id 712 within 711 inlined_at 21 766 0
	mov rax, qword ptr [rbp + 2648]
	mov r14, qword ptr [rbp + 2672]
	cmp r14, rax
	jne .LBB10_29
	mov qword ptr [rbp + 2720], r13
	lea rcx, [rbp + 2648]
	call alloc::collections::vec_deque::VecDeque<T,A>::grow
	jmp .LBB10_28
	.p2align	4, 0x90
.LBB10_34:
	.cv_inline_site_id 713 within 309 inlined_at 12 106 0
	.cv_inline_site_id 714 within 713 inlined_at 20 536 0
	.cv_inline_site_id 715 within 714 inlined_at 20 536 0
	.cv_inline_site_id 716 within 715 inlined_at 22 598 0
	test rdx, rdx
	jne .LBB10_35
	jmp .LBB10_5
.LBB10_6:
	mov rsi, qword ptr [rbp + 2520]
	mov qword ptr [rsi], 0
	mov qword ptr [rsi + 8], 8
	mov qword ptr [rsi + 16], 0
	mov rdx, qword ptr [rbp + 2528]
	.cv_inline_site_id 717 within 309 inlined_at 12 109 0
	.cv_inline_site_id 718 within 717 inlined_at 20 536 0
	.cv_inline_site_id 719 within 718 inlined_at 20 536 0
	.cv_inline_site_id 720 within 719 inlined_at 22 598 0
	test rdx, rdx
	je .LBB10_8
	mov rcx, qword ptr [rbp + 2536]
	.cv_inline_site_id 721 within 720 inlined_at 22 309 0
	shl rdx, 5
	.cv_inline_site_id 722 within 719 inlined_at 22 599 0
	.cv_inline_site_id 723 within 722 inlined_at 24 254 0
	mov r8d, 8
	call __rust_dealloc
.LBB10_8:
	lea rcx, [rbp + 2648]
	call core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<alloc::vec::Vec<bot::game::Game>>>
	mov rax, rsi
	movaps xmm6, xmmword ptr [rbp + 2736]
	movaps xmm7, xmmword ptr [rbp + 2752]
	movaps xmm8, xmmword ptr [rbp + 2768]
	movaps xmm9, xmmword ptr [rbp + 2784]
	movaps xmm10, xmmword ptr [rbp + 2800]
	add rsp, 2952
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
.LBB10_79:
	xor ecx, ecx
	jmp .LBB10_87
.LBB10_86:
	mov rcx, qword ptr [rbp + 2600]
	mov rdx, qword ptr [rbp + 2608]
.LBB10_87:
	mov qword ptr [rbp + 2624], r15
	mov rax, qword ptr [rbp + 2704]
	mov qword ptr [rbp + 2616], rax
	call alloc::raw_vec::handle_error
	jmp .LBB10_13
.LBB10_59:
	xor r13d, r13d
.LBB10_60:
	mov rax, qword ptr [rbp + 2720]
	mov qword ptr [rbp + 2688], rax
	mov rax, qword ptr [rbp + 2712]
	mov qword ptr [rbp + 2680], rax
	mov rcx, r13
	mov rdx, r14
	call alloc::raw_vec::handle_error
	jmp .LBB10_13
.LBB10_100:
	xor ecx, ecx
	jmp .LBB10_106
.LBB10_105:
	mov rcx, qword ptr [rbp + 2024]
	mov rdx, qword ptr [rbp + 2032]
.LBB10_106:
	call alloc::raw_vec::handle_error
	jmp .LBB10_13
.LBB10_40:
	mov qword ptr [rbp + 2688], r13
	mov rax, qword ptr [rbp + 2712]
	mov qword ptr [rbp + 2680], rax
	lea r8, [rip + __unnamed_9]
	mov edx, 3
	mov rcx, qword ptr [rbp + 2552]
	call core::panicking::panic_bounds_check
	jmp .LBB10_13
.LBB10_21:
	lea rcx, [rip + __unnamed_10]
	call core::option::unwrap_failed
	jmp .LBB10_13
.LBB10_30:
	mov qword ptr [rbp + 2640], rcx
	mov qword ptr [rbp + 2632], rdx
	lea r8, [rip + __unnamed_11]
	xor ecx, ecx
	xor edx, edx
	call core::panicking::panic_bounds_check
	jmp .LBB10_13
.LBB10_10:
	xor ebx, ebx
.LBB10_12:
	mov rcx, rbx
	mov rdx, rsi
	call alloc::raw_vec::handle_error
.LBB10_13:
	ud2
	.seh_handlerdata
	.long	($cppxdata$bot::simd::search_simd)@IMGREL
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$14@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$14@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$14@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	mov rdx, qword ptr [rbp + 2528]
	.cv_inline_site_id 724 within 309 inlined_at 12 109 0
	.cv_inline_site_id 725 within 724 inlined_at 20 536 0
	.cv_inline_site_id 726 within 725 inlined_at 20 536 0
	.cv_inline_site_id 727 within 726 inlined_at 22 598 0
	test rdx, rdx
	je .LBB10_16
	mov rcx, qword ptr [rbp + 2536]
	.cv_inline_site_id 728 within 727 inlined_at 22 309 0
	shl rdx, 5
	.cv_inline_site_id 729 within 726 inlined_at 22 599 0
	.cv_inline_site_id 730 within 729 inlined_at 24 254 0
	mov r8d, 8
	call __rust_dealloc
.LBB10_16:
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movdqa xmm8, xmmword ptr [rsp + 64]
	movdqa xmm9, xmmword ptr [rsp + 48]
	movdqa xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
	.seh_handlerdata
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$17@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$17@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$17@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	mov rcx, qword ptr [rbp + 2696]
	cmp qword ptr [rbp + 2704], 0
	.cv_inline_site_id 731 within 327 inlined_at 21 1753 0
	.cv_inline_site_id 732 within 731 inlined_at 20 536 0
	.cv_inline_site_id 733 within 732 inlined_at 20 536 0
	.cv_inline_site_id 734 within 733 inlined_at 22 598 0
	je .LBB10_19
	.cv_inline_site_id 735 within 734 inlined_at 22 309 0
	mov rdx, qword ptr [rbp + 2704]
	shl rdx, 5
	.cv_inline_site_id 736 within 733 inlined_at 22 599 0
	.cv_inline_site_id 737 within 736 inlined_at 24 254 0
	mov r8d, 8
	call __rust_dealloc
.LBB10_19:
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movdqa xmm8, xmmword ptr [rsp + 64]
	movdqa xmm9, xmmword ptr [rsp + 48]
	movdqa xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
	.seh_handlerdata
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$31@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$31@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$31@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	mov rdx, qword ptr [rbp + 2632]
	.cv_inline_site_id 738 within 309 inlined_at 12 106 0
	.cv_inline_site_id 739 within 738 inlined_at 20 536 0
	.cv_inline_site_id 740 within 739 inlined_at 20 536 0
	.cv_inline_site_id 741 within 740 inlined_at 22 598 0
	test rdx, rdx
	je .LBB10_33
	.cv_inline_site_id 742 within 741 inlined_at 22 309 0
	shl rdx, 5
	.cv_inline_site_id 743 within 740 inlined_at 22 599 0
	.cv_inline_site_id 744 within 743 inlined_at 24 254 0
	mov r8d, 8
	mov rcx, qword ptr [rbp + 2640]
	call __rust_dealloc
.LBB10_33:
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movdqa xmm8, xmmword ptr [rsp + 64]
	movdqa xmm9, xmmword ptr [rsp + 48]
	movdqa xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
	.seh_handlerdata
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$36@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$36@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$36@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	.cv_inline_site_id 745 within 334 inlined_at 21 1776 0
	.cv_inline_site_id 746 within 745 inlined_at 20 536 0
	.cv_inline_site_id 747 within 746 inlined_at 20 536 0
	.cv_inline_site_id 748 within 747 inlined_at 22 598 0
	cmp qword ptr [rbp + 2720], 0
	je .LBB10_38
	.cv_inline_site_id 749 within 748 inlined_at 22 309 0
	mov rdx, qword ptr [rbp + 2720]
	shl rdx, 5
	.cv_inline_site_id 750 within 747 inlined_at 22 599 0
	.cv_inline_site_id 751 within 750 inlined_at 24 254 0
	mov r8d, 8
	mov rcx, qword ptr [rbp + 2712]
	call __rust_dealloc
.LBB10_38:
	mov rax, qword ptr [rbp + 2720]
	mov qword ptr [rbp + 2688], rax
	mov rax, qword ptr [rbp + 2712]
	mov qword ptr [rbp + 2680], rax
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movdqa xmm8, xmmword ptr [rsp + 64]
	movdqa xmm9, xmmword ptr [rsp + 48]
	movdqa xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
	.seh_handlerdata
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$93@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$93@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$93@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	mov rdx, qword ptr [rbp + 2624]
	.cv_inline_site_id 752 within 410 inlined_at 44 43 0
	.cv_inline_site_id 753 within 752 inlined_at 20 536 0
	.cv_inline_site_id 754 within 753 inlined_at 20 536 0
	.cv_inline_site_id 755 within 754 inlined_at 22 598 0
	test rdx, rdx
	je .LBB10_95
	.cv_inline_site_id 756 within 755 inlined_at 22 309 0
	mov rcx, qword ptr [rbp + 2616]
	shl rdx, 5
	.cv_inline_site_id 757 within 754 inlined_at 22 599 0
	.cv_inline_site_id 758 within 757 inlined_at 24 254 0
	mov r8d, 8
	call __rust_dealloc
.LBB10_95:
	mov rax, qword ptr [rbp + 2720]
	mov qword ptr [rbp + 2688], rax
	mov rax, qword ptr [rbp + 2712]
	mov qword ptr [rbp + 2680], rax
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movdqa xmm8, xmmword ptr [rsp + 64]
	movdqa xmm9, xmmword ptr [rsp + 48]
	movdqa xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
	.seh_handlerdata
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$96@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$96@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$96@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	mov rdx, qword ptr [rbp + 2696]
	.cv_inline_site_id 759 within 478 inlined_at 46 36 0
	.cv_inline_site_id 760 within 759 inlined_at 20 536 0
	.cv_inline_site_id 761 within 760 inlined_at 47 507 0
	.cv_inline_site_id 762 within 761 inlined_at 20 536 0
	.cv_inline_site_id 763 within 762 inlined_at 47 496 0
	.cv_inline_site_id 764 within 763 inlined_at 20 536 0
	.cv_inline_site_id 765 within 764 inlined_at 22 598 0
	test rdx, rdx
	je .LBB10_98
	.cv_inline_site_id 766 within 765 inlined_at 22 309 0
	shl rdx, 5
	.cv_inline_site_id 767 within 764 inlined_at 22 599 0
	.cv_inline_site_id 768 within 767 inlined_at 24 254 0
	mov r8d, 8
	mov rcx, qword ptr [rbp + 2704]
	call __rust_dealloc
.LBB10_98:
	mov rax, qword ptr [rbp + 2720]
	mov qword ptr [rbp + 2688], rax
	mov rax, qword ptr [rbp + 2712]
	mov qword ptr [rbp + 2680], rax
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movdqa xmm8, xmmword ptr [rsp + 64]
	movdqa xmm9, xmmword ptr [rsp + 48]
	movdqa xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
	.seh_handlerdata
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$111@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$111@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$111@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	cmp qword ptr [rbp + 2504], 4
	jae .LBB10_112
	.cv_inline_site_id 769 within 309 inlined_at 12 106 0
	.cv_inline_site_id 770 within 769 inlined_at 20 536 0
	.cv_inline_site_id 771 within 770 inlined_at 20 536 0
	.cv_inline_site_id 772 within 771 inlined_at 22 598 0
	mov rdx, qword ptr [rbp + 2688]
	test rdx, rdx
	je .LBB10_115
	.cv_inline_site_id 773 within 772 inlined_at 22 309 0
	mov rcx, qword ptr [rbp + 2680]
	shl rdx, 5
	.cv_inline_site_id 774 within 771 inlined_at 22 599 0
	.cv_inline_site_id 775 within 774 inlined_at 24 254 0
	mov r8d, 8
	call __rust_dealloc
.LBB10_115:
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movaps xmm8, xmmword ptr [rsp + 64]
	movaps xmm9, xmmword ptr [rsp + 48]
	movaps xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
.LBB10_112:
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movaps xmm8, xmmword ptr [rsp + 64]
	movaps xmm9, xmmword ptr [rsp + 48]
	movaps xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
	.seh_handlerdata
.section .text,"xr",one_only,bot::simd::search_simd
	.seh_endproc
	.def	"?dtor$116@?0?bot::simd::search_simd@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$116@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA":
.seh_proc "?dtor$116@?0?_ZN3bot4simd11search_simd17h255b4551b38913c5E@4HA"
	mov qword ptr [rsp + 16], rdx
	push rbp
	.seh_pushreg rbp
	push r15
	.seh_pushreg r15
	push r14
	.seh_pushreg r14
	push r13
	.seh_pushreg r13
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
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm10
	.seh_savexmm xmm10, 32
	movdqa xmmword ptr [rsp + 48], xmm9
	.seh_savexmm xmm9, 48
	movdqa xmmword ptr [rsp + 64], xmm8
	.seh_savexmm xmm8, 64
	movaps xmmword ptr [rsp + 80], xmm7
	.seh_savexmm xmm7, 80
	movaps xmmword ptr [rsp + 96], xmm6
	.seh_savexmm xmm6, 96
	.seh_endprologue
	lea rcx, [rbp + 2648]
	call core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<alloc::vec::Vec<bot::game::Game>>>
	movaps xmm6, xmmword ptr [rsp + 96]
	movaps xmm7, xmmword ptr [rsp + 80]
	movaps xmm8, xmmword ptr [rsp + 64]
	movaps xmm9, xmmword ptr [rsp + 48]
	movaps xmm10, xmmword ptr [rsp + 32]
	add rsp, 120
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret

.section .text,"xr",one_only,bot::simd::search_simd_med
	.globl	bot::simd::search_simd_med
	.p2align	4, 0x90
bot::simd::search_simd_med:
	.cv_func_id 332
.seh_proc _ZN3bot4simd15search_simd_med17h996d00c0cda4bd46E
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
	sub rsp, 264
	.seh_stackalloc 264
	lea rbp, [rsp + 128]
	.seh_setframe rbp, 128
	movdqa xmmword ptr [rbp + 112], xmm13
	.seh_savexmm xmm13, 240
	movdqa xmmword ptr [rbp + 96], xmm12
	.seh_savexmm xmm12, 224
	movdqa xmmword ptr [rbp + 80], xmm11
	.seh_savexmm xmm11, 208
	movdqa xmmword ptr [rbp + 64], xmm10
	.seh_savexmm xmm10, 192
	movdqa xmmword ptr [rbp + 48], xmm9
	.seh_savexmm xmm9, 176
	movdqa xmmword ptr [rbp + 32], xmm8
	.seh_savexmm xmm8, 160
	movdqa xmmword ptr [rbp + 16], xmm7
	.seh_savexmm xmm7, 144
	movdqa xmmword ptr [rbp], xmm6
	.seh_savexmm xmm6, 128
	.seh_endprologue
	mov qword ptr [rbp - 8], -2
	mov rsi, rcx
	mov r14, qword ptr [rbp + 256]
	mov rax, qword ptr [rbp + 240]
	.cv_inline_site_id 333 within 332 inlined_at 12 26 0
	.cv_inline_site_id 334 within 333 inlined_at 10 600 0
	.cv_inline_site_id 335 within 334 inlined_at 6 25 0
	.cv_inline_site_id 336 within 335 inlined_at 6 302 0
	.cv_inline_site_id 337 within 336 inlined_at 6 599 0
	.cv_inline_site_id 338 within 337 inlined_at 5 112 0
	.cv_inline_site_id 339 within 338 inlined_at 7 2369 0
	.cv_inline_site_id 340 within 339 inlined_at 8 57 0
	.cv_inline_site_id 341 within 340 inlined_at 4 902 0
	shr r8, 3
	.cv_inline_site_id 342 within 335 inlined_at 6 303 0
	.cv_inline_site_id 343 within 342 inlined_at 6 599 0
	.cv_inline_site_id 344 within 343 inlined_at 5 112 0
	.cv_inline_site_id 345 within 344 inlined_at 7 2369 0
	.cv_inline_site_id 346 within 345 inlined_at 8 57 0
	.cv_inline_site_id 347 within 346 inlined_at 4 902 0
	shr rax, 3
	cmp r8, rax
	cmovb rax, r8
	.cv_inline_site_id 348 within 332 inlined_at 12 26 0
	.cv_inline_site_id 349 within 348 inlined_at 10 600 0
	.cv_inline_site_id 350 within 349 inlined_at 6 25 0
	.cv_inline_site_id 351 within 350 inlined_at 6 303 0
	.cv_inline_site_id 352 within 351 inlined_at 6 599 0
	.cv_inline_site_id 353 within 352 inlined_at 5 112 0
	.cv_inline_site_id 354 within 353 inlined_at 7 2369 0
	.cv_inline_site_id 355 within 354 inlined_at 8 57 0
	.cv_inline_site_id 356 within 355 inlined_at 4 902 0
	shr r14, 3
	cmp rax, r14
	cmovb r14, rax
	.cv_inline_site_id 357 within 332 inlined_at 12 35 0
	.cv_inline_site_id 358 within 357 inlined_at 10 3390 0
	.cv_inline_site_id 359 within 358 inlined_at 37 120 0
	pxor xmm0, xmm0
	movdqu xmmword ptr [rbp - 40], xmm0
	mov qword ptr [rbp - 80], 0
	mov qword ptr [rbp - 72], 2
	movdqu xmmword ptr [rbp - 64], xmm0
	mov qword ptr [rbp - 48], 2
	mov qword ptr [rbp - 24], 2
	mov qword ptr [rbp - 16], 0
	.cv_inline_site_id 360 within 357 inlined_at 10 3391 0
	.cv_inline_site_id 361 within 360 inlined_at 38 525 0
	test r14, r14
	je .LBB11_1
	.cv_inline_site_id 362 within 361 inlined_at 38 617 0
	.cv_inline_site_id 363 within 362 inlined_at 39 3064 0
	.cv_inline_site_id 364 within 363 inlined_at 39 973 0
	mov rdi, r9
	mov rbx, rdx
	lea rcx, [rbp - 80]
	xor edx, edx
	mov r8, r14
	call alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
	.cv_inline_site_id 365 within 361 inlined_at 38 618 0
	.cv_inline_site_id 366 within 365 inlined_at 38 534 0
	.cv_inline_site_id 367 within 366 inlined_at 39 3064 0
	.cv_inline_site_id 368 within 367 inlined_at 39 973 0
	.cv_inline_site_id 369 within 368 inlined_at 22 353 0
	.cv_inline_site_id 370 within 369 inlined_at 22 443 0
	mov rax, qword ptr [rbp - 56]
	mov rdx, qword ptr [rbp - 40]
	sub rax, rdx
	cmp rax, r14
	jae .LBB11_9
	lea rcx, [rbp - 56]
	mov r8, r14
	call alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
.LBB11_9:
	.cv_inline_site_id 371 within 365 inlined_at 38 535 0
	.cv_inline_site_id 372 within 371 inlined_at 39 3064 0
	.cv_inline_site_id 373 within 372 inlined_at 39 973 0
	.cv_inline_site_id 374 within 373 inlined_at 22 353 0
	.cv_inline_site_id 375 within 374 inlined_at 22 443 0
	mov rax, qword ptr [rbp - 32]
	mov rdx, qword ptr [rbp - 16]
	sub rax, rdx
	cmp rax, r14
	jae .LBB11_2
	lea rcx, [rbp - 32]
	mov r8, r14
	call alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
	mov rdx, qword ptr [rbp - 16]
.LBB11_2:
	.cv_inline_site_id 376 within 361 inlined_at 38 621 0
	.cv_inline_site_id 377 within 376 inlined_at 5 128 0
	.cv_inline_site_id 378 within 377 inlined_at 6 104 0
	.cv_inline_site_id 379 within 378 inlined_at 6 283 0
	.cv_inline_site_id 380 within 379 inlined_at 40 843 0
	mov r11, qword ptr [rbp + 248]
	mov r8, qword ptr [rbp - 72]
	mov r9, qword ptr [rbp - 64]
	mov rcx, qword ptr [rbp - 48]
	mov rax, qword ptr [rbp - 24]
	mov r10, qword ptr [rbp - 40]
	mov r15, rdx
	shl r15, 4
	add r15, rax
	mov r13, r10
	shl r13, 4
	add r13, rcx
	mov qword ptr [rbp - 88], r9
	shl r9, 4
	add r9, r8
	xor r12d, r12d
	pcmpeqd xmm0, xmm0
	movdqa xmm1, xmmword ptr [rip + __xmm@55555555555555555555555555555555]
	movdqa xmm2, xmmword ptr [rip + __xmm@33333333333333333333333333333333]
	movdqa xmm3, xmmword ptr [rip + __xmm@0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f]
	movdqa xmm4, xmmword ptr [rip + __xmm@000f000f000f000f000f000f000f000f]
	movdqa xmm5, xmmword ptr [rip + __xmm@00100010001000100010001000100010]
	movdqa xmm6, xmmword ptr [rip + __xmm@3f8000003f8000003f8000003f800000]
	.p2align	4, 0x90
.LBB11_3:
	.cv_inline_site_id 381 within 378 inlined_at 6 288 0
	.cv_inline_site_id 382 within 381 inlined_at 6 273 0
	.cv_inline_site_id 383 within 382 inlined_at 6 114 0
	.cv_inline_site_id 384 within 383 inlined_at 6 273 0
	movdqu xmm7, xmmword ptr [rbx + 2*r12]
	.cv_inline_site_id 385 within 383 inlined_at 6 273 0
	movdqu xmm9, xmmword ptr [rdi + 2*r12]
	.cv_inline_site_id 386 within 381 inlined_at 6 273 0
	movdqu xmm8, xmmword ptr [r11 + 2*r12]
	.cv_inline_site_id 387 within 378 inlined_at 6 288 0
	.cv_inline_site_id 388 within 387 inlined_at 5 88 0
	.cv_inline_site_id 389 within 388 inlined_at 12 27 0
	movdqa xmm10, xmm7
	psrlw xmm10, 1
	por xmm10, xmm7
	movdqa xmm11, xmm10
	psrlw xmm11, 2
	por xmm11, xmm10
	movdqa xmm10, xmm11
	psrlw xmm10, 4
	por xmm10, xmm11
	movdqa xmm11, xmm10
	psrlw xmm11, 8
	por xmm11, xmm10
	pxor xmm11, xmm0
	movdqa xmm10, xmm11
	psrlw xmm10, 1
	pand xmm10, xmm1
	psubb xmm11, xmm10
	movdqa xmm10, xmm11
	pand xmm10, xmm2
	psrlw xmm11, 2
	pand xmm11, xmm2
	paddb xmm11, xmm10
	movdqa xmm10, xmm11
	psrlw xmm10, 4
	paddb xmm10, xmm11
	pand xmm10, xmm3
	movdqa xmm11, xmm10
	psllw xmm11, 8
	paddb xmm11, xmm10
	psrlw xmm11, 8
	movdqa xmm10, xmm4
	psubw xmm10, xmm11
	.cv_inline_site_id 390 within 388 inlined_at 12 28 0
	movdqa xmm11, xmm9
	psrlw xmm11, 1
	por xmm11, xmm9
	movdqa xmm12, xmm11
	psrlw xmm12, 2
	por xmm12, xmm11
	movdqa xmm11, xmm12
	psrlw xmm11, 4
	por xmm11, xmm12
	movdqa xmm12, xmm11
	psrlw xmm12, 8
	por xmm12, xmm11
	pxor xmm12, xmm0
	movdqa xmm11, xmm12
	psrlw xmm11, 1
	pand xmm11, xmm1
	psubb xmm12, xmm11
	movdqa xmm11, xmm12
	pand xmm11, xmm2
	psrlw xmm12, 2
	pand xmm12, xmm2
	paddb xmm12, xmm11
	movdqa xmm11, xmm12
	psrlw xmm11, 4
	paddb xmm11, xmm12
	pand xmm11, xmm3
	movdqa xmm12, xmm11
	psllw xmm12, 8
	paddb xmm12, xmm11
	psrlw xmm12, 8
	movdqa xmm11, xmm5
	psubw xmm11, xmm12
	.cv_inline_site_id 391 within 388 inlined_at 12 29 0
	movdqa xmm12, xmm8
	psrlw xmm12, 1
	por xmm12, xmm8
	movdqa xmm13, xmm12
	psrlw xmm13, 2
	por xmm13, xmm12
	movdqa xmm12, xmm13
	psrlw xmm12, 4
	por xmm12, xmm13
	movdqa xmm13, xmm12
	psrlw xmm13, 8
	por xmm13, xmm12
	pxor xmm13, xmm0
	movdqa xmm12, xmm13
	psrlw xmm12, 1
	pand xmm12, xmm1
	psubb xmm13, xmm12
	movdqa xmm12, xmm13
	pand xmm12, xmm2
	psrlw xmm13, 2
	pand xmm13, xmm2
	paddb xmm13, xmm12
	movdqa xmm12, xmm13
	psrlw xmm12, 4
	paddb xmm12, xmm13
	pand xmm12, xmm3
	movdqa xmm13, xmm12
	psllw xmm13, 8
	paddb xmm13, xmm12
	psrlw xmm13, 8
	movdqa xmm12, xmm4
	psubw xmm12, xmm13
	.cv_inline_site_id 392 within 388 inlined_at 12 30 0
	.cv_inline_site_id 393 within 392 inlined_at 17 90 0
	psubusw xmm11, xmm10
	paddw xmm11, xmm10
	.cv_inline_site_id 394 within 392 inlined_at 17 90 0
	psubusw xmm12, xmm11
	paddw xmm12, xmm11
	.cv_inline_site_id 395 within 388 inlined_at 12 32 0
	.cv_inline_site_id 396 within 395 inlined_at 15 60 0
	pand xmm12, xmm4
	movdqa xmm10, xmm12
	punpckhwd xmm10, xmm10
	pslld xmm10, 23
	paddd xmm10, xmm6
	cvttps2dq xmm10, xmm10
	pslld xmm10, 16
	psrad xmm10, 16
	punpcklwd xmm12, xmm12
	pslld xmm12, 23
	paddd xmm12, xmm6
	cvttps2dq xmm11, xmm12
	pslld xmm11, 16
	psrad xmm11, 16
	packssdw xmm11, xmm10
	movdqa xmm10, xmm11
	paddw xmm10, xmm11
	por xmm7, xmm10
	paddw xmm11, xmm10
	por xmm11, xmm9
	por xmm10, xmm8
	.cv_inline_site_id 397 within 387 inlined_at 5 88 0
	.cv_inline_site_id 398 within 397 inlined_at 38 603 0
	.cv_inline_site_id 399 within 398 inlined_at 39 3072 0
	movdqu xmmword ptr [r9 + 2*r12], xmm7
	.cv_inline_site_id 400 within 397 inlined_at 38 604 0
	.cv_inline_site_id 401 within 400 inlined_at 38 541 0
	.cv_inline_site_id 402 within 401 inlined_at 39 3072 0
	movdqu xmmword ptr [r13 + 2*r12], xmm11
	.cv_inline_site_id 403 within 400 inlined_at 38 542 0
	.cv_inline_site_id 404 within 403 inlined_at 39 3072 0
	movdqu xmmword ptr [r15 + 2*r12], xmm10
	.cv_inline_site_id 405 within 380 inlined_at 40 752 0
	add r12, 8
	dec r14
	jne .LBB11_3
	mov rbx, qword ptr [rbp - 80]
	mov r11, qword ptr [rbp - 56]
	mov r9, qword ptr [rbp - 32]
	mov rdi, qword ptr [rbp - 88]
	.cv_inline_site_id 406 within 332 inlined_at 12 36 0
	.cv_inline_site_id 407 within 406 inlined_at 39 2679 0
	lea rdi, [r12 + 8*rdi]
	shl rbx, 3
	.cv_inline_site_id 408 within 332 inlined_at 12 36 0
	.cv_inline_site_id 409 within 408 inlined_at 39 2679 0
	lea r10, [r12 + 8*r10]
	shl r11, 3
	.cv_inline_site_id 410 within 332 inlined_at 12 36 0
	.cv_inline_site_id 411 within 410 inlined_at 39 2679 0
	lea rdx, [r12 + 8*rdx]
	shl r9, 3
	jmp .LBB11_5
.LBB11_1:
	mov eax, 2
	xor edx, edx
	xor r9d, r9d
	xor r10d, r10d
	mov ecx, 2
	xor r11d, r11d
	xor edi, edi
	mov r8d, 2
	xor ebx, ebx
.LBB11_5:
	mov qword ptr [rsi], rbx
	mov qword ptr [rsi + 8], r8
	mov qword ptr [rsi + 16], rdi
	mov qword ptr [rsi + 24], r11
	mov qword ptr [rsi + 32], rcx
	mov qword ptr [rsi + 40], r10
	mov qword ptr [rsi + 48], r9
	mov qword ptr [rsi + 56], rax
	mov qword ptr [rsi + 64], rdx
	mov rax, rsi
	movaps xmm6, xmmword ptr [rbp]
	movaps xmm7, xmmword ptr [rbp + 16]
	movaps xmm8, xmmword ptr [rbp + 32]
	movaps xmm9, xmmword ptr [rbp + 48]
	movaps xmm10, xmmword ptr [rbp + 64]
	movaps xmm11, xmmword ptr [rbp + 80]
	movaps xmm12, xmmword ptr [rbp + 96]
	movaps xmm13, xmmword ptr [rbp + 112]
	add rsp, 264
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
	.long	($cppxdata$bot::simd::search_simd_med)@IMGREL
.section .text,"xr",one_only,bot::simd::search_simd_med
	.seh_endproc
	.def	"?dtor$12@?0?bot::simd::search_simd_med@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$12@?0?_ZN3bot4simd15search_simd_med17h996d00c0cda4bd46E@4HA":
.seh_proc "?dtor$12@?0?_ZN3bot4simd15search_simd_med17h996d00c0cda4bd46E@4HA"
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
	sub rsp, 168
	.seh_stackalloc 168
	lea rbp, [rdx + 128]
	movdqa xmmword ptr [rsp + 32], xmm13
	.seh_savexmm xmm13, 32
	movdqa xmmword ptr [rsp + 48], xmm12
	.seh_savexmm xmm12, 48
	movdqa xmmword ptr [rsp + 64], xmm11
	.seh_savexmm xmm11, 64
	movdqa xmmword ptr [rsp + 80], xmm10
	.seh_savexmm xmm10, 80
	movdqa xmmword ptr [rsp + 96], xmm9
	.seh_savexmm xmm9, 96
	movdqa xmmword ptr [rsp + 112], xmm8
	.seh_savexmm xmm8, 112
	movdqa xmmword ptr [rsp + 128], xmm7
	.seh_savexmm xmm7, 128
	movdqa xmmword ptr [rsp + 144], xmm6
	.seh_savexmm xmm6, 144
	.seh_endprologue
	lea rcx, [rbp - 80]
	call core::ptr::drop_in_place<(alloc::vec::Vec<[u16; 8]>,(alloc::vec::Vec<[u16; 8]>,alloc::vec::Vec<[u16; 8]>))>
	movaps xmm6, xmmword ptr [rsp + 144]
	movaps xmm7, xmmword ptr [rsp + 128]
	movaps xmm8, xmmword ptr [rsp + 112]
	movaps xmm9, xmmword ptr [rsp + 96]
	movaps xmm10, xmmword ptr [rsp + 80]
	movaps xmm11, xmmword ptr [rsp + 64]
	movaps xmm12, xmmword ptr [rsp + 48]
	movaps xmm13, xmmword ptr [rsp + 32]
	add rsp, 168
	pop rbx
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret

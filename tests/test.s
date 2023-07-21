	.option nopic
	.text
	.align	1
	.global	sort
	.type	sort, @function
sort:
sort._entry:
	addi	sp, sp, -24
	sd	a0, 16(sp)
	sw	a1, 12(sp)
	li	t2, 0
	sw	t2, 8(sp)
sort.while_entry_0:
	lw	t2, 8(sp)
	sext.w	t2, t2
	lw	t3, 12(sp)
	sext.w	t3, t3
	addi	t3, t3, -1
	slt	t2, t2, t3
	beq	t2, zero, sort.while_end_0
sort.while_body_0:
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, 1
	sw	t2, 4(sp)
sort.while_entry_1:
	lw	t2, 4(sp)
	sext.w	t2, t2
	lw	t3, 12(sp)
	sext.w	t3, t3
	slt	t2, t2, t3
	beq	t2, zero, sort.while_end_1
sort.while_body_1:
	ld	t2, 16(sp)
	lw	t3, 8(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	ld	t3, 16(sp)
	lw	t4, 4(sp)
	sext.w	t4, t4
	slli	t4, t4, 2
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	slt	t2, t2, t3
	beq	t2, zero, sort.if_end_0
sort.if_then_0:
	ld	t2, 16(sp)
	lw	t3, 8(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	sw	t2, 0(sp)
	ld	t2, 16(sp)
	lw	t3, 8(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	ld	t3, 16(sp)
	lw	t4, 4(sp)
	sext.w	t4, t4
	slli	t4, t4, 2
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	sw	t3, 0(t2)
	ld	t2, 16(sp)
	lw	t3, 4(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t3, 0(sp)
	sext.w	t3, t3
	sw	t3, 0(t2)
sort.if_end_0:
	lw	t2, 4(sp)
	sext.w	t2, t2
	addi	t2, t2, 1
	sw	t2, 4(sp)
	j	sort.while_entry_1
sort.while_end_1:
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, 1
	sw	t2, 8(sp)
	j	sort.while_entry_0
sort.while_end_0:
	addi	sp, sp, 24
	ret
	.text
	.align	1
	.global	param32_rec
	.type	param32_rec, @function
param32_rec:
param32_rec._entry:
	addi	sp, sp, -424
	sd	ra, 0(sp)
	sd	s2, 8(sp)
	sd	s3, 16(sp)
	sd	s1, 24(sp)
	sd	s5, 32(sp)
	sd	s4, 40(sp)
	sd	s0, 48(sp)
	sd	s10, 56(sp)
	sd	s11, 64(sp)
	sd	s6, 72(sp)
	sd	s9, 80(sp)
	sd	s8, 88(sp)
	sd	s7, 96(sp)
	sw	a0, 324(sp)
	sw	a1, 320(sp)
	sw	a2, 316(sp)
	sw	a3, 312(sp)
	sw	a4, 308(sp)
	sw	a5, 304(sp)
	sw	a6, 300(sp)
	sw	a7, 296(sp)
	lw	t2, 420(sp)
	sw	t2, 292(sp)
	lw	t2, 416(sp)
	sw	t2, 288(sp)
	lw	t2, 412(sp)
	sw	t2, 284(sp)
	lw	t2, 408(sp)
	sw	t2, 280(sp)
	lw	t2, 404(sp)
	sw	t2, 276(sp)
	lw	t2, 400(sp)
	sw	t2, 272(sp)
	lw	t2, 396(sp)
	sw	t2, 268(sp)
	lw	t2, 392(sp)
	sw	t2, 264(sp)
	lw	t2, 388(sp)
	sw	t2, 260(sp)
	lw	t2, 384(sp)
	sw	t2, 256(sp)
	lw	t2, 380(sp)
	sw	t2, 252(sp)
	lw	t2, 376(sp)
	sw	t2, 248(sp)
	lw	t2, 372(sp)
	sw	t2, 244(sp)
	lw	t2, 368(sp)
	sw	t2, 240(sp)
	lw	t2, 364(sp)
	sw	t2, 236(sp)
	lw	t2, 360(sp)
	sw	t2, 232(sp)
	lw	t2, 356(sp)
	sw	t2, 228(sp)
	lw	t2, 352(sp)
	sw	t2, 224(sp)
	lw	t2, 348(sp)
	sw	t2, 220(sp)
	lw	t2, 344(sp)
	sw	t2, 216(sp)
	lw	t2, 340(sp)
	sw	t2, 212(sp)
	lw	t2, 336(sp)
	sw	t2, 208(sp)
	lw	t2, 332(sp)
	sw	t2, 204(sp)
	lw	t2, 328(sp)
	sw	t2, 200(sp)
	lw	t2, 324(sp)
	sext.w	t2, t2
	addi	t2, t2, -0
	seqz	t2, t2
	beq	t2, zero, param32_rec.if_else_0
param32_rec.if_then_0:
	lw	t2, 320(sp)
	sext.w	t2, t2
	mv	a0, t2
	ld	ra, 0(sp)
	ld	s2, 8(sp)
	ld	s3, 16(sp)
	ld	s1, 24(sp)
	ld	s5, 32(sp)
	ld	s4, 40(sp)
	ld	s0, 48(sp)
	ld	s10, 56(sp)
	ld	s11, 64(sp)
	ld	s6, 72(sp)
	ld	s9, 80(sp)
	ld	s8, 88(sp)
	ld	s7, 96(sp)
	addi	sp, sp, 424
	ret
param32_rec.if_else_0:
	lw	t2, 324(sp)
	sext.w	t2, t2
	addi	t2, t2, -1
	lw	t3, 320(sp)
	sext.w	t3, t3
	lw	t4, 316(sp)
	sext.w	t4, t4
	add	t3, t3, t4
	li	t4, 998244353
	remw	t3, t3, t4
	lw	t4, 312(sp)
	sext.w	t4, t4
	lw	t5, 308(sp)
	sext.w	t5, t5
	lw	t6, 304(sp)
	sext.w	t6, t6
	lw	a0, 300(sp)
	sext.w	a0, a0
	lw	a1, 296(sp)
	sext.w	a1, a1
	lw	a2, 292(sp)
	sext.w	a2, a2
	lw	a3, 288(sp)
	sext.w	a3, a3
	lw	a4, 284(sp)
	sext.w	a4, a4
	lw	a5, 280(sp)
	sext.w	a5, a5
	lw	a6, 276(sp)
	sext.w	a6, a6
	lw	a7, 272(sp)
	sext.w	a7, a7
	lw	s0, 268(sp)
	sext.w	s0, s0
	lw	s1, 264(sp)
	sext.w	s1, s1
	lw	s2, 260(sp)
	sext.w	s2, s2
	lw	s3, 256(sp)
	sext.w	s3, s3
	lw	s4, 252(sp)
	sext.w	s4, s4
	lw	s5, 248(sp)
	sext.w	s5, s5
	lw	s6, 244(sp)
	sext.w	s6, s6
	lw	s7, 240(sp)
	sext.w	s7, s7
	lw	s8, 236(sp)
	sext.w	s8, s8
	lw	s9, 232(sp)
	sext.w	s9, s9
	lw	s10, 228(sp)
	sext.w	s10, s10
	lw	s11, 224(sp)
	sext.w	s11, s11
	lw	t0, 220(sp)
	sd	t0, 104(sp)
	sext.w	t0, t0
	sd	t0, 192(sp)
	lw	t0, 216(sp)
	sd	t0, 112(sp)
	sext.w	t0, t0
	sd	t0, 184(sp)
	lw	t0, 212(sp)
	sd	t0, 120(sp)
	sext.w	t0, t0
	sd	t0, 176(sp)
	lw	t0, 208(sp)
	sd	t0, 128(sp)
	sext.w	t0, t0
	sd	t0, 168(sp)
	lw	t0, 204(sp)
	sd	t0, 136(sp)
	sext.w	t0, t0
	sd	t0, 160(sp)
	lw	t0, 200(sp)
	sd	t0, 144(sp)
	sext.w	t0, t0
	sd	t0, 152(sp)
	li	t1, 0
	sw	t1, -96(sp)
	ld	t0, 152(sp)
	sw	t0, -92(sp)
	ld	t0, 160(sp)
	sw	t0, -88(sp)
	ld	t0, 168(sp)
	sw	t0, -84(sp)
	ld	t0, 176(sp)
	sw	t0, -80(sp)
	ld	t0, 184(sp)
	sw	t0, -76(sp)
	ld	t0, 192(sp)
	sw	t0, -72(sp)
	sw	s11, -68(sp)
	sw	s10, -64(sp)
	sw	s9, -60(sp)
	sw	s8, -56(sp)
	sw	s7, -52(sp)
	sw	s6, -48(sp)
	sw	s5, -44(sp)
	sw	s4, -40(sp)
	sw	s3, -36(sp)
	sw	s2, -32(sp)
	sw	s1, -28(sp)
	sw	s0, -24(sp)
	sw	a7, -20(sp)
	sw	a6, -16(sp)
	sw	a5, -12(sp)
	sw	a4, -8(sp)
	sw	a3, -4(sp)
	mv	a7, a2
	mv	a6, a1
	mv	a5, a0
	mv	a4, t6
	mv	a3, t5
	mv	a2, t4
	mv	a1, t3
	mv	a0, t2
	call	param32_rec
	mv	t2, a0
	mv	a0, t2
	ld	ra, 0(sp)
	ld	s2, 8(sp)
	ld	s3, 16(sp)
	ld	s1, 24(sp)
	ld	s5, 32(sp)
	ld	s4, 40(sp)
	ld	s0, 48(sp)
	ld	s10, 56(sp)
	ld	s11, 64(sp)
	ld	s6, 72(sp)
	ld	s9, 80(sp)
	ld	s8, 88(sp)
	ld	s7, 96(sp)
	addi	sp, sp, 424
	ret
	.text
	.align	1
	.global	param32_arr
	.type	param32_arr, @function
param32_arr:
param32_arr._entry:
	addi	sp, sp, -452
	sd	a0, 252(sp)
	sd	a1, 244(sp)
	sd	a2, 236(sp)
	sd	a3, 228(sp)
	sd	a4, 220(sp)
	sd	a5, 212(sp)
	sd	a6, 204(sp)
	sd	a7, 196(sp)
	ld	t2, 444(sp)
	sd	t2, 188(sp)
	ld	t2, 436(sp)
	sd	t2, 180(sp)
	ld	t2, 428(sp)
	sd	t2, 172(sp)
	ld	t2, 420(sp)
	sd	t2, 164(sp)
	ld	t2, 412(sp)
	sd	t2, 156(sp)
	ld	t2, 404(sp)
	sd	t2, 148(sp)
	ld	t2, 396(sp)
	sd	t2, 140(sp)
	ld	t2, 388(sp)
	sd	t2, 132(sp)
	ld	t2, 380(sp)
	sd	t2, 124(sp)
	ld	t2, 372(sp)
	sd	t2, 116(sp)
	ld	t2, 364(sp)
	sd	t2, 108(sp)
	ld	t2, 356(sp)
	sd	t2, 100(sp)
	ld	t2, 348(sp)
	sd	t2, 92(sp)
	ld	t2, 340(sp)
	sd	t2, 84(sp)
	ld	t2, 332(sp)
	sd	t2, 76(sp)
	ld	t2, 324(sp)
	sd	t2, 68(sp)
	ld	t2, 316(sp)
	sd	t2, 60(sp)
	ld	t2, 308(sp)
	sd	t2, 52(sp)
	ld	t2, 300(sp)
	sd	t2, 44(sp)
	ld	t2, 292(sp)
	sd	t2, 36(sp)
	ld	t2, 284(sp)
	sd	t2, 28(sp)
	ld	t2, 276(sp)
	sd	t2, 20(sp)
	ld	t2, 268(sp)
	sd	t2, 12(sp)
	ld	t2, 260(sp)
	sd	t2, 4(sp)
	ld	t2, 252(sp)
	lw	t2, 0(t2)
	sext.w	t2, t2
	ld	t3, 252(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 244(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 244(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 236(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 236(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 228(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 228(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 220(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 220(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 212(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 212(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 204(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 204(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 196(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 196(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 188(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 188(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 180(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 180(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 172(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 172(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 164(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 164(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 156(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 156(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 148(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 148(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 140(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 140(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 132(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 132(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 124(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 124(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 116(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 116(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 108(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 108(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 100(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 100(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 92(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 92(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 84(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 84(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 76(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 76(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 68(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 68(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 60(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 60(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 52(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 52(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 44(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 44(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 36(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 36(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 28(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 28(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 20(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 20(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 12(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 12(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	ld	t3, 4(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	ld	t3, 4(sp)
	li	t4, 4
	add	t3, t3, t4
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	sw	t2, 0(sp)
	sext.w	t2, t2
	mv	a0, t2
	addi	sp, sp, 452
	ret
	.text
	.align	1
	.global	param16
	.type	param16, @function
param16:
param16._entry:
	addi	sp, sp, -408
	sd	s2, 0(sp)
	sd	s3, 8(sp)
	sd	s1, 16(sp)
	sd	s8, 24(sp)
	sd	s11, 32(sp)
	sd	s9, 40(sp)
	sd	ra, 48(sp)
	sd	s7, 56(sp)
	sd	s5, 64(sp)
	sd	s0, 72(sp)
	sd	s6, 80(sp)
	sd	s4, 88(sp)
	sd	s10, 96(sp)
	sw	a0, 372(sp)
	sw	a1, 368(sp)
	sw	a2, 364(sp)
	sw	a3, 360(sp)
	sw	a4, 356(sp)
	sw	a5, 352(sp)
	sw	a6, 348(sp)
	sw	a7, 344(sp)
	lw	t2, 404(sp)
	sw	t2, 340(sp)
	lw	t2, 400(sp)
	sw	t2, 336(sp)
	lw	t2, 396(sp)
	sw	t2, 332(sp)
	lw	t2, 392(sp)
	sw	t2, 328(sp)
	lw	t2, 388(sp)
	sw	t2, 324(sp)
	lw	t2, 384(sp)
	sw	t2, 320(sp)
	lw	t2, 380(sp)
	sw	t2, 316(sp)
	lw	t2, 376(sp)
	sw	t2, 312(sp)
	lw	t2, 372(sp)
	sext.w	s0, t2
	lw	t2, 368(sp)
	sext.w	s1, t2
	lw	t2, 364(sp)
	sext.w	s2, t2
	lw	t2, 360(sp)
	sext.w	s3, t2
	lw	t2, 356(sp)
	sext.w	s4, t2
	lw	t2, 352(sp)
	sext.w	s5, t2
	lw	t2, 348(sp)
	sext.w	s6, t2
	lw	t2, 344(sp)
	sext.w	s7, t2
	lw	t2, 340(sp)
	sext.w	s8, t2
	lw	t2, 336(sp)
	sext.w	s9, t2
	lw	t2, 332(sp)
	sext.w	s10, t2
	lw	t2, 328(sp)
	sext.w	s11, t2
	lw	t2, 324(sp)
	sext.w	t0, t2
	sd	t0, 104(sp)
	lw	t2, 320(sp)
	sext.w	t0, t2
	sd	t0, 112(sp)
	lw	t2, 316(sp)
	sext.w	t0, t2
	sd	t0, 120(sp)
	lw	t2, 312(sp)
	sext.w	t0, t2
	sd	t0, 128(sp)
	addi	t2, sp, 248
	sd	zero, 0(t2)
	sd	zero, 8(t2)
	sd	zero, 16(t2)
	sd	zero, 24(t2)
	sd	zero, 32(t2)
	sd	zero, 40(t2)
	sd	zero, 48(t2)
	sd	zero, 56(t2)
	addi	t2, sp, 248
	sw	s0, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 4
	sw	s1, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 8
	sw	s2, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 12
	sw	s3, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 16
	sw	s4, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 20
	sw	s5, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 24
	sw	s6, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 28
	sw	s7, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 32
	sw	s8, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 36
	sw	s9, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 40
	sw	s10, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 44
	sw	s11, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 48
	ld	t0, 104(sp)
	sw	t0, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 52
	ld	t0, 112(sp)
	sw	t0, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 56
	ld	t0, 120(sp)
	sw	t0, 0(t2)
	addi	t2, sp, 248
	addi	t2, t2, 60
	ld	t0, 128(sp)
	sw	t0, 0(t2)
	addi	t2, sp, 248
	li	a1, 16
	mv	a0, t2
	call	sort
	addi	t2, sp, 248
	lw	t2, 0(t2)
	sext.w	t2, t2
	addi	t3, sp, 248
	addi	t3, t3, 4
	lw	t3, 0(t3)
	sext.w	t3, t3
	addi	t4, sp, 248
	addi	t4, t4, 8
	lw	t4, 0(t4)
	sext.w	t4, t4
	addi	t5, sp, 248
	addi	t5, t5, 12
	lw	t5, 0(t5)
	sext.w	t5, t5
	addi	t6, sp, 248
	addi	t6, t6, 16
	lw	t6, 0(t6)
	sext.w	t6, t6
	addi	a0, sp, 248
	addi	a0, a0, 20
	lw	a0, 0(a0)
	sext.w	a0, a0
	addi	a1, sp, 248
	addi	a1, a1, 24
	lw	a1, 0(a1)
	sext.w	a1, a1
	addi	a2, sp, 248
	addi	a2, a2, 28
	lw	a2, 0(a2)
	sext.w	a2, a2
	addi	a3, sp, 248
	addi	a3, a3, 32
	lw	a3, 0(a3)
	sext.w	a3, a3
	addi	a4, sp, 248
	addi	a4, a4, 36
	lw	a4, 0(a4)
	sext.w	a4, a4
	addi	a5, sp, 248
	addi	a5, a5, 40
	lw	a5, 0(a5)
	sext.w	a5, a5
	addi	a6, sp, 248
	addi	a6, a6, 44
	lw	a6, 0(a6)
	sext.w	a6, a6
	addi	a7, sp, 248
	addi	a7, a7, 48
	lw	a7, 0(a7)
	sext.w	a7, a7
	addi	s11, sp, 248
	addi	s11, s11, 52
	lw	s11, 0(s11)
	sext.w	s11, s11
	addi	s10, sp, 248
	addi	s10, s10, 56
	lw	s10, 0(s10)
	sext.w	s10, s10
	addi	s9, sp, 248
	addi	s9, s9, 60
	lw	s9, 0(s9)
	sext.w	s9, s9
	lw	s8, 372(sp)
	sext.w	s8, s8
	lw	s7, 368(sp)
	sext.w	s7, s7
	lw	s6, 364(sp)
	sext.w	s6, s6
	lw	s5, 360(sp)
	sext.w	s5, s5
	lw	s4, 356(sp)
	sext.w	s4, s4
	lw	s3, 352(sp)
	sext.w	s3, s3
	lw	s2, 348(sp)
	sext.w	s2, s2
	lw	s1, 344(sp)
	sext.w	s1, s1
	lw	s0, 340(sp)
	sext.w	s0, s0
	lw	t0, 336(sp)
	sd	t0, 136(sp)
	sext.w	t0, t0
	sd	t0, 240(sp)
	lw	t0, 332(sp)
	sd	t0, 144(sp)
	sext.w	t0, t0
	sd	t0, 232(sp)
	lw	t0, 328(sp)
	sd	t0, 152(sp)
	sext.w	t0, t0
	sd	t0, 224(sp)
	lw	t0, 324(sp)
	sd	t0, 160(sp)
	sext.w	t0, t0
	sd	t0, 216(sp)
	lw	t0, 320(sp)
	sd	t0, 168(sp)
	sext.w	t0, t0
	sd	t0, 208(sp)
	lw	t0, 316(sp)
	sd	t0, 176(sp)
	sext.w	t0, t0
	sd	t0, 200(sp)
	lw	t0, 312(sp)
	sd	t0, 184(sp)
	sext.w	t0, t0
	sd	t0, 192(sp)
	sw	t0, -96(sp)
	ld	t0, 200(sp)
	sw	t0, -92(sp)
	ld	t0, 208(sp)
	sw	t0, -88(sp)
	ld	t0, 216(sp)
	sw	t0, -84(sp)
	ld	t0, 224(sp)
	sw	t0, -80(sp)
	ld	t0, 232(sp)
	sw	t0, -76(sp)
	ld	t0, 240(sp)
	sw	t0, -72(sp)
	sw	s0, -68(sp)
	sw	s1, -64(sp)
	sw	s2, -60(sp)
	sw	s3, -56(sp)
	sw	s4, -52(sp)
	sw	s5, -48(sp)
	sw	s6, -44(sp)
	sw	s7, -40(sp)
	sw	s8, -36(sp)
	sw	s9, -32(sp)
	sw	s10, -28(sp)
	sw	s11, -24(sp)
	sw	a7, -20(sp)
	sw	a6, -16(sp)
	sw	a5, -12(sp)
	sw	a4, -8(sp)
	sw	a3, -4(sp)
	mv	a7, a2
	mv	a6, a1
	mv	a5, a0
	mv	a4, t6
	mv	a3, t5
	mv	a2, t4
	mv	a1, t3
	mv	a0, t2
	call	param32_rec
	mv	t2, a0
	mv	a0, t2
	ld	s2, 0(sp)
	ld	s3, 8(sp)
	ld	s1, 16(sp)
	ld	s8, 24(sp)
	ld	s11, 32(sp)
	ld	s9, 40(sp)
	ld	ra, 48(sp)
	ld	s7, 56(sp)
	ld	s5, 64(sp)
	ld	s0, 72(sp)
	ld	s6, 80(sp)
	ld	s4, 88(sp)
	ld	s10, 96(sp)
	addi	sp, sp, 408
	ret
	.text
	.align	1
	.global	main
	.type	main, @function
main:
main._entry:
	addi	sp, sp, -556
	sd	s1, 0(sp)
	sd	s2, 8(sp)
	sd	s6, 16(sp)
	sd	s7, 24(sp)
	sd	s9, 32(sp)
	sd	s11, 40(sp)
	sd	s3, 48(sp)
	sd	s4, 56(sp)
	sd	s8, 64(sp)
	sd	s10, 72(sp)
	sd	s5, 80(sp)
	sd	ra, 88(sp)
	sd	s0, 96(sp)
	call	getint
	mv	s0, a0
	call	getint
	mv	s1, a0
	call	getint
	mv	s2, a0
	call	getint
	mv	s3, a0
	call	getint
	mv	s4, a0
	call	getint
	mv	s5, a0
	call	getint
	mv	s6, a0
	call	getint
	mv	s7, a0
	call	getint
	mv	s8, a0
	call	getint
	mv	s9, a0
	call	getint
	mv	s10, a0
	call	getint
	mv	s11, a0
	call	getint
	mv	t0, a0
	sd	t0, 288(sp)
	call	getint
	mv	t0, a0
	sd	t0, 280(sp)
	call	getint
	mv	t0, a0
	sd	t0, 272(sp)
	call	getint
	mv	t2, a0
	sw	t2, -32(sp)
	ld	t0, 272(sp)
	sw	t0, -28(sp)
	ld	t0, 280(sp)
	sw	t0, -24(sp)
	ld	t0, 288(sp)
	sw	t0, -20(sp)
	sw	s11, -16(sp)
	sw	s10, -12(sp)
	sw	s9, -8(sp)
	sw	s8, -4(sp)
	mv	a7, s7
	mv	a6, s6
	mv	a5, s5
	mv	a4, s4
	mv	a3, s3
	mv	a2, s2
	mv	a1, s1
	mv	a0, s0
	call	param16
	mv	s0, a0
	addi	t2, sp, 300
	sd	zero, 0(t2)
	sd	zero, 8(t2)
	sd	zero, 16(t2)
	sd	zero, 24(t2)
	sd	zero, 32(t2)
	sd	zero, 40(t2)
	sd	zero, 48(t2)
	sd	zero, 56(t2)
	sd	zero, 64(t2)
	sd	zero, 72(t2)
	sd	zero, 80(t2)
	sd	zero, 88(t2)
	sd	zero, 96(t2)
	sd	zero, 104(t2)
	sd	zero, 112(t2)
	sd	zero, 120(t2)
	sd	zero, 128(t2)
	sd	zero, 136(t2)
	sd	zero, 144(t2)
	sd	zero, 152(t2)
	sd	zero, 160(t2)
	sd	zero, 168(t2)
	sd	zero, 176(t2)
	sd	zero, 184(t2)
	sd	zero, 192(t2)
	sd	zero, 200(t2)
	sd	zero, 208(t2)
	sd	zero, 216(t2)
	sd	zero, 224(t2)
	sd	zero, 232(t2)
	sd	zero, 240(t2)
	sd	zero, 248(t2)
	addi	t2, sp, 300
	mv	t3, t2
	sw	s0, 0(t3)
	addi	t2, t2, 4
	li	t3, 8848
	sw	t3, 0(t2)
	li	t2, 1
	sw	t2, 296(sp)
main.while_entry_0:
	lw	t2, 296(sp)
	sext.w	t2, t2
	slti	t2, t2, 32
	beq	t2, zero, main.while_end_0
main.while_body_0:
	lw	t2, 296(sp)
	sext.w	t2, t2
	addi	t3, sp, 300
	slli	t2, t2, 3
	add	t3, t3, t2
	lw	t2, 296(sp)
	sext.w	t2, t2
	addi	t2, t2, -1
	addi	t4, sp, 300
	slli	t2, t2, 3
	add	t4, t4, t2
	addi	t4, t4, 4
	lw	t4, 0(t4)
	sext.w	t4, t4
	addi	t4, t4, -1
	sw	t4, 0(t3)
	lw	t3, 296(sp)
	sext.w	t3, t3
	addi	t4, sp, 300
	slli	t3, t3, 3
	add	t4, t4, t3
	addi	t4, t4, 4
	lw	t3, 296(sp)
	sext.w	t3, t3
	addi	t3, t3, -1
	addi	t2, sp, 300
	slli	t3, t3, 3
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	addi	t2, t2, -2
	sw	t2, 0(t4)
	lw	t4, 296(sp)
	sext.w	t4, t4
	addi	t4, t4, 1
	sw	t4, 296(sp)
	j	main.while_entry_0
main.while_end_0:
	addi	t4, sp, 300
	addi	t2, sp, 300
	addi	t2, t2, 8
	addi	t3, sp, 300
	addi	t3, t3, 16
	addi	t5, sp, 300
	addi	t5, t5, 24
	addi	t6, sp, 300
	addi	t6, t6, 32
	addi	a0, sp, 300
	addi	a0, a0, 40
	addi	a1, sp, 300
	addi	a1, a1, 48
	addi	a2, sp, 300
	addi	a2, a2, 56
	addi	a3, sp, 300
	addi	a3, a3, 64
	addi	a4, sp, 300
	addi	a4, a4, 72
	addi	a5, sp, 300
	addi	a5, a5, 80
	addi	a6, sp, 300
	addi	a6, a6, 88
	addi	a7, sp, 300
	addi	a7, a7, 96
	addi	s0, sp, 300
	addi	s0, s0, 104
	addi	s1, sp, 300
	addi	s1, s1, 112
	addi	s2, sp, 300
	addi	s2, s2, 120
	addi	s3, sp, 300
	addi	s3, s3, 128
	addi	s4, sp, 300
	addi	s4, s4, 136
	addi	s5, sp, 300
	addi	s5, s5, 144
	addi	s6, sp, 300
	addi	s6, s6, 152
	addi	s7, sp, 300
	addi	s7, s7, 160
	addi	s8, sp, 300
	addi	s8, s8, 168
	addi	s9, sp, 300
	addi	s9, s9, 176
	addi	s10, sp, 300
	addi	s10, s10, 184
	addi	s11, sp, 300
	addi	s11, s11, 192
	addi	t0, sp, 300
	sd	t0, 104(sp)
	addi	t0, t0, 200
	sd	t0, 112(sp)
	sd	t0, 264(sp)
	addi	t0, sp, 300
	sd	t0, 120(sp)
	addi	t0, t0, 208
	sd	t0, 128(sp)
	sd	t0, 256(sp)
	addi	t0, sp, 300
	sd	t0, 136(sp)
	addi	t0, t0, 216
	sd	t0, 144(sp)
	sd	t0, 248(sp)
	addi	t0, sp, 300
	sd	t0, 152(sp)
	addi	t0, t0, 224
	sd	t0, 160(sp)
	sd	t0, 240(sp)
	addi	t0, sp, 300
	sd	t0, 168(sp)
	addi	t0, t0, 232
	sd	t0, 176(sp)
	sd	t0, 232(sp)
	addi	t0, sp, 300
	sd	t0, 184(sp)
	addi	t0, t0, 240
	sd	t0, 192(sp)
	sd	t0, 224(sp)
	addi	t0, sp, 300
	sd	t0, 200(sp)
	addi	t0, t0, 248
	sd	t0, 208(sp)
	sd	t0, 216(sp)
	sd	t0, -192(sp)
	ld	t0, 224(sp)
	sd	t0, -184(sp)
	ld	t0, 232(sp)
	sd	t0, -176(sp)
	ld	t0, 240(sp)
	sd	t0, -168(sp)
	ld	t0, 248(sp)
	sd	t0, -160(sp)
	ld	t0, 256(sp)
	sd	t0, -152(sp)
	ld	t0, 264(sp)
	sd	t0, -144(sp)
	sd	s11, -136(sp)
	sd	s10, -128(sp)
	sd	s9, -120(sp)
	sd	s8, -112(sp)
	sd	s7, -104(sp)
	sd	s6, -96(sp)
	sd	s5, -88(sp)
	sd	s4, -80(sp)
	sd	s3, -72(sp)
	sd	s2, -64(sp)
	sd	s1, -56(sp)
	sd	s0, -48(sp)
	sd	a7, -40(sp)
	sd	a6, -32(sp)
	sd	a5, -24(sp)
	sd	a4, -16(sp)
	sd	a3, -8(sp)
	mv	a7, a2
	mv	a6, a1
	mv	a5, a0
	mv	a4, t6
	mv	a3, t5
	mv	a2, t3
	mv	a1, t2
	mv	a0, t4
	call	param32_arr
	mv	t4, a0
	mv	a0, t4
	call	putint
	li	a0, 10
	call	putch
	li	a0, 0
	ld	s1, 0(sp)
	ld	s2, 8(sp)
	ld	s6, 16(sp)
	ld	s7, 24(sp)
	ld	s9, 32(sp)
	ld	s11, 40(sp)
	ld	s3, 48(sp)
	ld	s4, 56(sp)
	ld	s8, 64(sp)
	ld	s10, 72(sp)
	ld	s5, 80(sp)
	ld	ra, 88(sp)
	ld	s0, 96(sp)
	addi	sp, sp, 556
	ret

	.option nopic
	.text
	.section	.data
	.globl	last_char
	.align	2
	.type	last_char, @object
	.size	last_char, 4
last_char:
	.word	32
	.globl	num
	.align	2
	.type	num, @object
	.size	num, 4
num:
	.zero	4
	.globl	other
	.align	2
	.type	other, @object
	.size	other, 4
other:
	.zero	4
	.globl	cur_token
	.align	2
	.type	cur_token, @object
	.size	cur_token, 4
cur_token:
	.zero	4
	.text
	.align	1
	.global	next_char
	.type	next_char, @function
next_char:
	addi	sp, sp, -8
	sd	ra, 0(sp)
	call	getch
	mv	t2, a0
	la	t3, last_char
	sw	t2, 0(t3)
	la	t2, last_char
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	ld	ra, 0(sp)
	addi	sp, sp, 8
	ret
	.global	is_space
	.type	is_space, @function
is_space:
	addi	sp, sp, -8
	sw	a0, 0(sp)
	mv	t2, a0
	sext.w	t2, t2
	addi	t2, t2, -32
	seqz	t2, t2
	sw	t2, 4(sp)
	bne	t2, zero, is_space.or_end_0
is_space.or_false_0:
	lw	t2, 0(sp)
	sext.w	t2, t2
	addi	t2, t2, -10
	seqz	t2, t2
	sw	t2, 4(sp)
is_space.or_end_0:
	lw	t2, 4(sp)
	sext.w	t2, t2
	beq	t2, zero, is_space.if_else_0
is_space.if_then_0:
	li	a0, 1
	addi	sp, sp, 8
	ret
is_space.if_else_0:
	li	a0, 0
	addi	sp, sp, 8
	ret
	.global	is_num
	.type	is_num, @function
is_num:
	addi	sp, sp, -8
	sw	a0, 0(sp)
	mv	t2, a0
	sext.w	t2, t2
	li	t3, 48
	slt	t2, t2, t3
	xori	t2, t2, 1
	sw	t2, 4(sp)
	beq	t2, zero, is_num.and_end_0
is_num.and_true_0:
	lw	t2, 0(sp)
	sext.w	t2, t2
	li	t3, 57
	sgt	t2, t2, t3
	xori	t2, t2, 1
	sw	t2, 4(sp)
is_num.and_end_0:
	lw	t2, 4(sp)
	sext.w	t2, t2
	beq	t2, zero, is_num.if_else_0
is_num.if_then_0:
	li	a0, 1
	addi	sp, sp, 8
	ret
is_num.if_else_0:
	li	a0, 0
	addi	sp, sp, 8
	ret
	.global	next_token
	.type	next_token, @function
next_token:
	addi	sp, sp, -8
	sd	ra, 0(sp)
next_token.while_entry_0:
	la	t2, last_char
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	call	is_space
	mv	t2, a0
	snez	t2, t2
	beq	t2, zero, next_token.while_end_0
next_token.while_body_0:
	call	next_char
	mv	t2, a0
	j	next_token.while_entry_0
next_token.while_end_0:
	la	t2, last_char
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	call	is_num
	mv	t2, a0
	snez	t2, t2
	beq	t2, zero, next_token.if_else_0
next_token.if_then_0:
	la	t2, last_char
	lw	t2, 0(t2)
	sext.w	t2, t2
	addi	t2, t2, -48
	la	t3, num
	sw	t2, 0(t3)
next_token.while_entry_1:
	call	next_char
	mv	t2, a0
	mv	a0, t2
	call	is_num
	mv	t2, a0
	snez	t2, t2
	beq	t2, zero, next_token.while_end_1
next_token.while_body_1:
	la	t2, num
	lw	t2, 0(t2)
	sext.w	t2, t2
	li	t3, 10
	mulw	t2, t2, t3
	la	t3, last_char
	lw	t3, 0(t3)
	sext.w	t3, t3
	add	t2, t2, t3
	addi	t2, t2, -48
	la	t3, num
	sw	t2, 0(t3)
	j	next_token.while_entry_1
next_token.while_end_1:
	li	t2, 0
	la	t3, cur_token
	sw	t2, 0(t3)
	j	next_token.if_end_0
next_token.if_else_0:
	la	t2, last_char
	lw	t2, 0(t2)
	sext.w	t2, t2
	la	t3, other
	sw	t2, 0(t3)
	call	next_char
	mv	t2, a0
	li	t2, 1
	la	t3, cur_token
	sw	t2, 0(t3)
next_token.if_end_0:
	la	t2, cur_token
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	ld	ra, 0(sp)
	addi	sp, sp, 8
	ret
	.global	panic
	.type	panic, @function
panic:
	addi	sp, sp, -8
	sd	ra, 0(sp)
	li	a0, 112
	call	putch
	li	a0, 97
	call	putch
	li	a0, 110
	call	putch
	li	a0, 105
	call	putch
	li	a0, 99
	call	putch
	li	a0, 33
	call	putch
	li	a0, 10
	call	putch
	li	a0, -1
	ld	ra, 0(sp)
	addi	sp, sp, 8
	ret
	.global	get_op_prec
	.type	get_op_prec, @function
get_op_prec:
	addi	sp, sp, -8
	sw	a0, 0(sp)
	mv	t2, a0
	sext.w	t2, t2
	addi	t2, t2, -43
	seqz	t2, t2
	sw	t2, 4(sp)
	bne	t2, zero, get_op_prec.or_end_0
get_op_prec.or_false_0:
	lw	t2, 0(sp)
	sext.w	t2, t2
	addi	t2, t2, -45
	seqz	t2, t2
	sw	t2, 4(sp)
get_op_prec.or_end_0:
	lw	t2, 4(sp)
	sext.w	t2, t2
	beq	t2, zero, get_op_prec.if_end_0
get_op_prec.if_then_0:
	li	a0, 10
	addi	sp, sp, 8
	ret
get_op_prec.if_end_0:
	lw	t2, 0(sp)
	sext.w	t2, t2
	addi	t2, t2, -42
	seqz	t2, t2
	sw	t2, 4(sp)
	bne	t2, zero, get_op_prec.or_end_1
get_op_prec.or_false_1:
	lw	t2, 0(sp)
	sext.w	t2, t2
	addi	t2, t2, -47
	seqz	t2, t2
	sw	t2, 4(sp)
get_op_prec.or_end_1:
	lw	t2, 4(sp)
	sext.w	t2, t2
	sw	t2, 4(sp)
	bne	t2, zero, get_op_prec.or_end_2
get_op_prec.or_false_2:
	lw	t2, 0(sp)
	sext.w	t2, t2
	addi	t2, t2, -37
	seqz	t2, t2
	sw	t2, 4(sp)
get_op_prec.or_end_2:
	lw	t2, 4(sp)
	sext.w	t2, t2
	beq	t2, zero, get_op_prec.if_end_1
get_op_prec.if_then_1:
	li	a0, 20
	addi	sp, sp, 8
	ret
get_op_prec.if_end_1:
	li	a0, 0
	addi	sp, sp, 8
	ret
	.global	stack_push
	.type	stack_push, @function
stack_push:
	addi	sp, sp, -12
	sd	a0, 4(sp)
	sw	a1, 0(sp)
	ld	t2, 4(sp)
	ld	t3, 4(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	addi	t3, t3, 1
	sw	t3, 0(t2)
	ld	t2, 4(sp)
	ld	t3, 4(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t3, 0(sp)
	sext.w	t3, t3
	sw	t3, 0(t2)
	addi	sp, sp, 12
	ret
	.global	stack_pop
	.type	stack_pop, @function
stack_pop:
	addi	sp, sp, -12
	sd	a0, 4(sp)
	mv	t2, a0
	ld	t3, 4(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	sw	t2, 0(sp)
	ld	t2, 4(sp)
	ld	t3, 4(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	addi	t3, t3, -1
	sw	t3, 0(t2)
	lw	t2, 0(sp)
	sext.w	t2, t2
	mv	a0, t2
	addi	sp, sp, 12
	ret
	.global	stack_peek
	.type	stack_peek, @function
stack_peek:
	addi	sp, sp, -8
	sd	a0, 0(sp)
	mv	t2, a0
	ld	t3, 0(sp)
	lw	t3, 0(t3)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	addi	sp, sp, 8
	ret
	.global	stack_size
	.type	stack_size, @function
stack_size:
	addi	sp, sp, -8
	sd	a0, 0(sp)
	mv	t2, a0
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	addi	sp, sp, 8
	ret
	.global	eval_op
	.type	eval_op, @function
eval_op:
	addi	sp, sp, -12
	sw	a0, 8(sp)
	sw	a1, 4(sp)
	sw	a2, 0(sp)
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, -43
	seqz	t2, t2
	beq	t2, zero, eval_op.if_end_0
eval_op.if_then_0:
	lw	t2, 4(sp)
	sext.w	t2, t2
	lw	t3, 0(sp)
	sext.w	t3, t3
	add	t2, t2, t3
	mv	a0, t2
	addi	sp, sp, 12
	ret
eval_op.if_end_0:
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, -45
	seqz	t2, t2
	beq	t2, zero, eval_op.if_end_1
eval_op.if_then_1:
	lw	t2, 4(sp)
	sext.w	t2, t2
	lw	t3, 0(sp)
	sext.w	t3, t3
	sub	t2, t2, t3
	mv	a0, t2
	addi	sp, sp, 12
	ret
eval_op.if_end_1:
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, -42
	seqz	t2, t2
	beq	t2, zero, eval_op.if_end_2
eval_op.if_then_2:
	lw	t2, 4(sp)
	sext.w	t2, t2
	lw	t3, 0(sp)
	sext.w	t3, t3
	mulw	t2, t2, t3
	mv	a0, t2
	addi	sp, sp, 12
	ret
eval_op.if_end_2:
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, -47
	seqz	t2, t2
	beq	t2, zero, eval_op.if_end_3
eval_op.if_then_3:
	lw	t2, 4(sp)
	sext.w	t2, t2
	lw	t3, 0(sp)
	sext.w	t3, t3
	divw	t2, t2, t3
	mv	a0, t2
	addi	sp, sp, 12
	ret
eval_op.if_end_3:
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, -37
	seqz	t2, t2
	beq	t2, zero, eval_op.if_end_4
eval_op.if_then_4:
	lw	t2, 4(sp)
	sext.w	t2, t2
	lw	t3, 0(sp)
	sext.w	t3, t3
	remw	t2, t2, t3
	mv	a0, t2
	addi	sp, sp, 12
	ret
eval_op.if_end_4:
	li	a0, 0
	addi	sp, sp, 12
	ret
	.global	eval
	.type	eval, @function
eval:
	li	t0, -2096
	add	sp, sp, t0
	sd	s0, 0(sp)
	sd	ra, 8(sp)
	addi	t2, sp, 1068
	li	a2, 1024
	li	a1, 0
	mv	a0, t2
	call	memset
	addi	t2, sp, 44
	li	a2, 1024
	li	a1, 0
	mv	a0, t2
	call	memset
	la	t2, cur_token
	lw	t2, 0(t2)
	sext.w	t2, t2
	addi	t2, t2, -0
	snez	t2, t2
	beq	t2, zero, eval.if_end_0
eval.if_then_0:
	call	panic
	mv	t2, a0
	mv	a0, t2
	ld	s0, 0(sp)
	ld	ra, 8(sp)
	li	a0, 2096
	add	sp, sp, a0
	ret
eval.if_end_0:
	addi	t2, sp, 1068
	la	t3, num
	lw	t3, 0(t3)
	sext.w	t3, t3
	mv	a1, t3
	mv	a0, t2
	call	stack_push
	call	next_token
	mv	t2, a0
eval.while_entry_0:
	la	t2, cur_token
	lw	t2, 0(t2)
	sext.w	t2, t2
	addi	t2, t2, -1
	seqz	t2, t2
	beq	t2, zero, eval.while_end_0
eval.while_body_0:
	la	t2, other
	lw	t2, 0(t2)
	sext.w	t2, t2
	sw	t2, 40(sp)
	sext.w	t2, t2
	mv	a0, t2
	call	get_op_prec
	mv	t2, a0
	seqz	t2, t2
	beq	t2, zero, eval.if_end_1
eval.if_then_1:
	j	eval.while_end_0
eval.if_end_1:
	call	next_token
	mv	t2, a0
eval.while_entry_1:
	addi	t2, sp, 44
	mv	a0, t2
	call	stack_size
	mv	t2, a0
	snez	t2, t2
	li	t0, 2092
	add	t0, t0, sp
	sw	t2, 0(t0)
	beq	t2, zero, eval.and_end_0
eval.and_true_0:
	addi	t2, sp, 44
	mv	a0, t2
	call	stack_peek
	mv	t2, a0
	mv	a0, t2
	call	get_op_prec
	mv	s0, a0
	lw	t2, 40(sp)
	sext.w	t2, t2
	mv	a0, t2
	call	get_op_prec
	mv	t2, a0
	slt	t2, s0, t2
	xori	t2, t2, 1
	li	t0, 2092
	add	t0, t0, sp
	sw	t2, 0(t0)
eval.and_end_0:
	li	t0, 2092
	add	t0, t0, sp
	lw	t2, 0(t0)
	sext.w	t2, t2
	beq	t2, zero, eval.while_end_1
eval.while_body_1:
	addi	t2, sp, 44
	mv	a0, t2
	call	stack_pop
	mv	t2, a0
	sw	t2, 36(sp)
	addi	t2, sp, 1068
	mv	a0, t2
	call	stack_pop
	mv	t2, a0
	sw	t2, 32(sp)
	addi	t2, sp, 1068
	mv	a0, t2
	call	stack_pop
	mv	t2, a0
	sw	t2, 28(sp)
	addi	t2, sp, 1068
	mv	s0, t2
	lw	t2, 36(sp)
	sext.w	t2, t2
	lw	t3, 28(sp)
	sext.w	t3, t3
	lw	t4, 32(sp)
	sext.w	t4, t4
	mv	a2, t4
	mv	a1, t3
	mv	a0, t2
	call	eval_op
	mv	t2, a0
	mv	a1, t2
	mv	a0, s0
	call	stack_push
	j	eval.while_entry_1
eval.while_end_1:
	addi	t2, sp, 44
	lw	t3, 40(sp)
	sext.w	t3, t3
	mv	a1, t3
	mv	a0, t2
	call	stack_push
	la	t2, cur_token
	lw	t2, 0(t2)
	sext.w	t2, t2
	addi	t2, t2, -0
	snez	t2, t2
	beq	t2, zero, eval.if_end_2
eval.if_then_2:
	call	panic
	mv	t2, a0
	mv	a0, t2
	ld	s0, 0(sp)
	ld	ra, 8(sp)
	li	a0, 2096
	add	sp, sp, a0
	ret
eval.if_end_2:
	addi	t2, sp, 1068
	la	t3, num
	lw	t3, 0(t3)
	sext.w	t3, t3
	mv	a1, t3
	mv	a0, t2
	call	stack_push
	call	next_token
	mv	t2, a0
	j	eval.while_entry_0
eval.while_end_0:
	call	next_token
	mv	t2, a0
eval.while_entry_2:
	addi	t2, sp, 44
	mv	a0, t2
	call	stack_size
	mv	t2, a0
	snez	t2, t2
	beq	t2, zero, eval.while_end_2
eval.while_body_2:
	addi	t2, sp, 44
	mv	a0, t2
	call	stack_pop
	mv	t2, a0
	sw	t2, 24(sp)
	addi	t2, sp, 1068
	mv	a0, t2
	call	stack_pop
	mv	t2, a0
	sw	t2, 20(sp)
	addi	t2, sp, 1068
	mv	a0, t2
	call	stack_pop
	mv	t2, a0
	sw	t2, 16(sp)
	addi	t2, sp, 1068
	mv	s0, t2
	lw	t2, 24(sp)
	sext.w	t2, t2
	lw	t3, 16(sp)
	sext.w	t3, t3
	lw	t4, 20(sp)
	sext.w	t4, t4
	mv	a2, t4
	mv	a1, t3
	mv	a0, t2
	call	eval_op
	mv	t2, a0
	mv	a1, t2
	mv	a0, s0
	call	stack_push
	j	eval.while_entry_2
eval.while_end_2:
	addi	t2, sp, 1068
	mv	a0, t2
	call	stack_peek
	mv	t2, a0
	mv	a0, t2
	ld	s0, 0(sp)
	ld	ra, 8(sp)
	li	a0, 2096
	add	sp, sp, a0
	ret
	.global	main
	.type	main, @function
main:
	addi	sp, sp, -12
	sd	ra, 0(sp)
	call	getint
	mv	t2, a0
	sw	t2, 8(sp)
	call	getch
	mv	t2, a0
	call	next_token
	mv	t2, a0
main.while_entry_0:
	lw	t2, 8(sp)
	sext.w	t2, t2
	snez	t2, t2
	beq	t2, zero, main.while_end_0
main.while_body_0:
	call	eval
	mv	t2, a0
	mv	a0, t2
	call	putint
	li	a0, 10
	call	putch
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, -1
	sw	t2, 8(sp)
	j	main.while_entry_0
main.while_end_0:
	li	a0, 0
	ld	ra, 0(sp)
	addi	sp, sp, 12
	ret


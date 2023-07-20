	.option nopic
	.text
	.align	1
	.global	defn
	.type	defn, @function
defn:
	addi	sp, sp, -4
	li	a0, 4
	addi	sp, sp, 4
	ret
defn.ret_then_0:
	addi	sp, sp, 4
	ret
	.global	main
	.type	main, @function
main:
	addi	sp, sp, -16
	sd	ra, 0(sp)
	call	defn
	mv	t2, a0
	sw	t2, 8(sp)
	sext.w	t2, t2
	mv	a0, t2
	addi	sp, sp, 16
	ret
main.ret_then_0:
	addi	sp, sp, 16
	ret

	.option nopic
	.text
	.align	1
	.global	func
	.type	func, @function
func:
func._entry:
	addi	sp, sp, -64
	sd	ra, 0(sp)
	sw	a0, 56(sp)
	sd	a1, 48(sp)
	sw	a2, 44(sp)
	sd	a3, 36(sp)
	sw	a4, 32(sp)
	sw	a5, 28(sp)
	sd	a6, 20(sp)
	sw	a7, 16(sp)
	lw	t2, 60(sp)
	sw	t2, 12(sp)
	li	t2, 0
	sw	t2, 8(sp)
func.while_entry_0:
	lw	t2, 8(sp)
	sext.w	t2, t2
	slti	t2, t2, 10
	beq	t2, zero, func.while_end_0
func.while_body_0:
	ld	t2, 48(sp)
	lw	t3, 56(sp)
	sext.w	t3, t3
	li	t4, 236
	mul	t3, t3, t4
	add	t2, t2, t3
	lw	t3, 8(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	call	putint
	lw	t2, 8(sp)
	sext.w	t2, t2
	addi	t2, t2, 1
	sw	t2, 8(sp)
	j	func.while_entry_0
func.while_end_0:
	li	a0, 10
	call	putch
	ld	t2, 36(sp)
	lw	t3, 44(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	call	putint
	li	a0, 10
	call	putch
func.while_entry_1:
	lw	t2, 12(sp)
	sext.w	t2, t2
	slti	t2, t2, 10
	beq	t2, zero, func.while_end_1
func.while_body_1:
	ld	t2, 20(sp)
	lw	t3, 12(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t3, 16(sp)
	sext.w	t3, t3
	li	t4, 128875
	mulw	t3, t3, t4
	li	t4, 3724
	remw	t3, t3, t4
	sw	t3, 0(t2)
	lw	t2, 12(sp)
	sext.w	t2, t2
	addi	t2, t2, 1
	sw	t2, 12(sp)
	lw	t2, 16(sp)
	sext.w	t2, t2
	addi	t2, t2, 7
	sw	t2, 16(sp)
	j	func.while_entry_1
func.while_end_1:
	lw	t2, 32(sp)
	sext.w	t2, t2
	lw	t3, 28(sp)
	sext.w	t3, t3
	add	t2, t2, t3
	mv	a0, t2
	ld	ra, 0(sp)
	addi	sp, sp, 64
	ret
	.text
	.align	1
	.global	main
	.type	main, @function
main:
main._entry:
	li	t0, -28876
	add	sp, sp, t0
	sd	s0, 0(sp)
	sd	ra, 8(sp)
	li	t0, 12528
	add	t3, sp, t0
	sd	zero, 0(t3)
	sd	zero, 8(t3)
	sd	zero, 16(t3)
	sd	zero, 24(t3)
	sd	zero, 32(t3)
	sd	zero, 40(t3)
	sd	zero, 48(t3)
	sd	zero, 56(t3)
	sd	zero, 64(t3)
	sd	zero, 72(t3)
	sd	zero, 80(t3)
	sd	zero, 88(t3)
	sd	zero, 96(t3)
	sd	zero, 104(t3)
	sd	zero, 112(t3)
	sd	zero, 120(t3)
	sd	zero, 128(t3)
	sd	zero, 136(t3)
	sd	zero, 144(t3)
	sd	zero, 152(t3)
	sd	zero, 160(t3)
	sd	zero, 168(t3)
	sd	zero, 176(t3)
	sd	zero, 184(t3)
	sd	zero, 192(t3)
	sd	zero, 200(t3)
	sd	zero, 208(t3)
	sd	zero, 216(t3)
	sd	zero, 224(t3)
	sd	zero, 232(t3)
	sd	zero, 240(t3)
	sd	zero, 248(t3)
	sd	zero, 256(t3)
	sd	zero, 264(t3)
	sd	zero, 272(t3)
	sd	zero, 280(t3)
	sd	zero, 288(t3)
	sd	zero, 296(t3)
	sd	zero, 304(t3)
	sd	zero, 312(t3)
	sd	zero, 320(t3)
	sd	zero, 328(t3)
	sd	zero, 336(t3)
	sd	zero, 344(t3)
	sd	zero, 352(t3)
	sd	zero, 360(t3)
	sd	zero, 368(t3)
	sd	zero, 376(t3)
	sd	zero, 384(t3)
	sd	zero, 392(t3)
	sd	zero, 400(t3)
	sd	zero, 408(t3)
	sd	zero, 416(t3)
	sd	zero, 424(t3)
	sd	zero, 432(t3)
	sd	zero, 440(t3)
	sd	zero, 448(t3)
	sd	zero, 456(t3)
	sd	zero, 464(t3)
	sd	zero, 472(t3)
	sd	zero, 480(t3)
	sd	zero, 488(t3)
	sd	zero, 496(t3)
	sd	zero, 504(t3)
	sd	zero, 512(t3)
	sd	zero, 520(t3)
	sd	zero, 528(t3)
	sd	zero, 536(t3)
	sd	zero, 544(t3)
	sd	zero, 552(t3)
	sd	zero, 560(t3)
	sd	zero, 568(t3)
	sd	zero, 576(t3)
	sd	zero, 584(t3)
	sd	zero, 592(t3)
	sd	zero, 600(t3)
	sd	zero, 608(t3)
	sd	zero, 616(t3)
	sd	zero, 624(t3)
	sd	zero, 632(t3)
	sd	zero, 640(t3)
	sd	zero, 648(t3)
	sd	zero, 656(t3)
	sd	zero, 664(t3)
	sd	zero, 672(t3)
	sd	zero, 680(t3)
	sd	zero, 688(t3)
	sd	zero, 696(t3)
	sd	zero, 704(t3)
	sd	zero, 712(t3)
	sd	zero, 720(t3)
	sd	zero, 728(t3)
	sd	zero, 736(t3)
	sd	zero, 744(t3)
	sd	zero, 752(t3)
	sd	zero, 760(t3)
	sd	zero, 768(t3)
	sd	zero, 776(t3)
	sd	zero, 784(t3)
	sd	zero, 792(t3)
	sd	zero, 800(t3)
	sd	zero, 808(t3)
	sd	zero, 816(t3)
	sd	zero, 824(t3)
	sd	zero, 832(t3)
	sd	zero, 840(t3)
	sd	zero, 848(t3)
	sd	zero, 856(t3)
	sd	zero, 864(t3)
	sd	zero, 872(t3)
	sd	zero, 880(t3)
	sd	zero, 888(t3)
	sd	zero, 896(t3)
	sd	zero, 904(t3)
	sd	zero, 912(t3)
	sd	zero, 920(t3)
	sd	zero, 928(t3)
	sd	zero, 936(t3)
	sd	zero, 944(t3)
	sd	zero, 952(t3)
	sd	zero, 960(t3)
	sd	zero, 968(t3)
	sd	zero, 976(t3)
	sd	zero, 984(t3)
	sd	zero, 992(t3)
	sd	zero, 1000(t3)
	sd	zero, 1008(t3)
	sd	zero, 1016(t3)
	sd	zero, 1024(t3)
	sd	zero, 1032(t3)
	sd	zero, 1040(t3)
	sd	zero, 1048(t3)
	sd	zero, 1056(t3)
	sd	zero, 1064(t3)
	sd	zero, 1072(t3)
	sd	zero, 1080(t3)
	sd	zero, 1088(t3)
	sd	zero, 1096(t3)
	sd	zero, 1104(t3)
	sd	zero, 1112(t3)
	sd	zero, 1120(t3)
	sd	zero, 1128(t3)
	sd	zero, 1136(t3)
	sd	zero, 1144(t3)
	sd	zero, 1152(t3)
	sd	zero, 1160(t3)
	sd	zero, 1168(t3)
	sd	zero, 1176(t3)
	sd	zero, 1184(t3)
	sd	zero, 1192(t3)
	sd	zero, 1200(t3)
	sd	zero, 1208(t3)
	sd	zero, 1216(t3)
	sd	zero, 1224(t3)
	sd	zero, 1232(t3)
	sd	zero, 1240(t3)
	sd	zero, 1248(t3)
	sd	zero, 1256(t3)
	sd	zero, 1264(t3)
	sd	zero, 1272(t3)
	sd	zero, 1280(t3)
	sd	zero, 1288(t3)
	sd	zero, 1296(t3)
	sd	zero, 1304(t3)
	sd	zero, 1312(t3)
	sd	zero, 1320(t3)
	sd	zero, 1328(t3)
	sd	zero, 1336(t3)
	sd	zero, 1344(t3)
	sd	zero, 1352(t3)
	sd	zero, 1360(t3)
	sd	zero, 1368(t3)
	sd	zero, 1376(t3)
	sd	zero, 1384(t3)
	sd	zero, 1392(t3)
	sd	zero, 1400(t3)
	sd	zero, 1408(t3)
	sd	zero, 1416(t3)
	sd	zero, 1424(t3)
	sd	zero, 1432(t3)
	sd	zero, 1440(t3)
	sd	zero, 1448(t3)
	sd	zero, 1456(t3)
	sd	zero, 1464(t3)
	sd	zero, 1472(t3)
	sd	zero, 1480(t3)
	sd	zero, 1488(t3)
	sd	zero, 1496(t3)
	sd	zero, 1504(t3)
	sd	zero, 1512(t3)
	sd	zero, 1520(t3)
	sd	zero, 1528(t3)
	sd	zero, 1536(t3)
	sd	zero, 1544(t3)
	sd	zero, 1552(t3)
	sd	zero, 1560(t3)
	sd	zero, 1568(t3)
	sd	zero, 1576(t3)
	sd	zero, 1584(t3)
	sd	zero, 1592(t3)
	sd	zero, 1600(t3)
	sd	zero, 1608(t3)
	sd	zero, 1616(t3)
	sd	zero, 1624(t3)
	sd	zero, 1632(t3)
	sd	zero, 1640(t3)
	sd	zero, 1648(t3)
	sd	zero, 1656(t3)
	sd	zero, 1664(t3)
	sd	zero, 1672(t3)
	sd	zero, 1680(t3)
	sd	zero, 1688(t3)
	sd	zero, 1696(t3)
	sd	zero, 1704(t3)
	sd	zero, 1712(t3)
	sd	zero, 1720(t3)
	sd	zero, 1728(t3)
	sd	zero, 1736(t3)
	sd	zero, 1744(t3)
	sd	zero, 1752(t3)
	sd	zero, 1760(t3)
	sd	zero, 1768(t3)
	sd	zero, 1776(t3)
	sd	zero, 1784(t3)
	sd	zero, 1792(t3)
	sd	zero, 1800(t3)
	sd	zero, 1808(t3)
	sd	zero, 1816(t3)
	sd	zero, 1824(t3)
	sd	zero, 1832(t3)
	sd	zero, 1840(t3)
	sd	zero, 1848(t3)
	sd	zero, 1856(t3)
	sd	zero, 1864(t3)
	sd	zero, 1872(t3)
	sd	zero, 1880(t3)
	sd	zero, 1888(t3)
	sd	zero, 1896(t3)
	sd	zero, 1904(t3)
	sd	zero, 1912(t3)
	sd	zero, 1920(t3)
	sd	zero, 1928(t3)
	sd	zero, 1936(t3)
	sd	zero, 1944(t3)
	sd	zero, 1952(t3)
	sd	zero, 1960(t3)
	sd	zero, 1968(t3)
	sd	zero, 1976(t3)
	sd	zero, 1984(t3)
	sd	zero, 1992(t3)
	sd	zero, 2000(t3)
	sd	zero, 2008(t3)
	sd	zero, 2016(t3)
	sd	zero, 2024(t3)
	sd	zero, 2032(t3)
	sd	zero, 2040(t3)
	li	t2, 2048
	add	t2, t3, t2
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	addi	t2, t2, 8
	sd	zero, 0(t2)
	li	t2, 16344
	add	t2, t3, t2
	sw	zero, 0(t2)
	addi	t2, sp, 20
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
	sd	zero, 256(t2)
	sd	zero, 264(t2)
	sd	zero, 272(t2)
	sd	zero, 280(t2)
	sd	zero, 288(t2)
	sd	zero, 296(t2)
	sd	zero, 304(t2)
	sd	zero, 312(t2)
	sd	zero, 320(t2)
	sd	zero, 328(t2)
	sd	zero, 336(t2)
	sd	zero, 344(t2)
	sd	zero, 352(t2)
	sd	zero, 360(t2)
	sd	zero, 368(t2)
	sd	zero, 376(t2)
	sd	zero, 384(t2)
	sd	zero, 392(t2)
	sd	zero, 400(t2)
	sd	zero, 408(t2)
	sd	zero, 416(t2)
	sd	zero, 424(t2)
	sd	zero, 432(t2)
	sd	zero, 440(t2)
	sd	zero, 448(t2)
	sd	zero, 456(t2)
	sd	zero, 464(t2)
	sd	zero, 472(t2)
	sd	zero, 480(t2)
	sd	zero, 488(t2)
	sd	zero, 496(t2)
	sd	zero, 504(t2)
	sd	zero, 512(t2)
	sd	zero, 520(t2)
	sd	zero, 528(t2)
	sd	zero, 536(t2)
	sd	zero, 544(t2)
	sd	zero, 552(t2)
	sd	zero, 560(t2)
	sd	zero, 568(t2)
	sd	zero, 576(t2)
	sd	zero, 584(t2)
	sd	zero, 592(t2)
	sd	zero, 600(t2)
	sd	zero, 608(t2)
	sd	zero, 616(t2)
	sd	zero, 624(t2)
	sd	zero, 632(t2)
	sd	zero, 640(t2)
	sd	zero, 648(t2)
	sd	zero, 656(t2)
	sd	zero, 664(t2)
	sd	zero, 672(t2)
	sd	zero, 680(t2)
	sd	zero, 688(t2)
	sd	zero, 696(t2)
	sd	zero, 704(t2)
	sd	zero, 712(t2)
	sd	zero, 720(t2)
	sd	zero, 728(t2)
	sd	zero, 736(t2)
	sd	zero, 744(t2)
	sd	zero, 752(t2)
	sd	zero, 760(t2)
	sd	zero, 768(t2)
	sd	zero, 776(t2)
	sd	zero, 784(t2)
	sd	zero, 792(t2)
	sd	zero, 800(t2)
	sd	zero, 808(t2)
	sd	zero, 816(t2)
	sd	zero, 824(t2)
	sd	zero, 832(t2)
	sd	zero, 840(t2)
	sd	zero, 848(t2)
	sd	zero, 856(t2)
	sd	zero, 864(t2)
	sd	zero, 872(t2)
	sd	zero, 880(t2)
	sd	zero, 888(t2)
	sd	zero, 896(t2)
	sd	zero, 904(t2)
	sd	zero, 912(t2)
	sd	zero, 920(t2)
	sd	zero, 928(t2)
	sd	zero, 936(t2)
	sd	zero, 944(t2)
	sd	zero, 952(t2)
	sd	zero, 960(t2)
	sd	zero, 968(t2)
	sd	zero, 976(t2)
	sd	zero, 984(t2)
	sd	zero, 992(t2)
	sd	zero, 1000(t2)
	sd	zero, 1008(t2)
	sd	zero, 1016(t2)
	sd	zero, 1024(t2)
	sd	zero, 1032(t2)
	sd	zero, 1040(t2)
	sd	zero, 1048(t2)
	sd	zero, 1056(t2)
	sd	zero, 1064(t2)
	sd	zero, 1072(t2)
	sd	zero, 1080(t2)
	sd	zero, 1088(t2)
	sd	zero, 1096(t2)
	sd	zero, 1104(t2)
	sd	zero, 1112(t2)
	sd	zero, 1120(t2)
	sd	zero, 1128(t2)
	sd	zero, 1136(t2)
	sd	zero, 1144(t2)
	sd	zero, 1152(t2)
	sd	zero, 1160(t2)
	sd	zero, 1168(t2)
	sd	zero, 1176(t2)
	sd	zero, 1184(t2)
	sd	zero, 1192(t2)
	sd	zero, 1200(t2)
	sd	zero, 1208(t2)
	sd	zero, 1216(t2)
	sd	zero, 1224(t2)
	sd	zero, 1232(t2)
	sd	zero, 1240(t2)
	sd	zero, 1248(t2)
	sd	zero, 1256(t2)
	sd	zero, 1264(t2)
	sd	zero, 1272(t2)
	sd	zero, 1280(t2)
	sd	zero, 1288(t2)
	sd	zero, 1296(t2)
	sd	zero, 1304(t2)
	sd	zero, 1312(t2)
	sd	zero, 1320(t2)
	sd	zero, 1328(t2)
	sd	zero, 1336(t2)
	sd	zero, 1344(t2)
	sd	zero, 1352(t2)
	sd	zero, 1360(t2)
	sd	zero, 1368(t2)
	sd	zero, 1376(t2)
	sd	zero, 1384(t2)
	sd	zero, 1392(t2)
	sd	zero, 1400(t2)
	sd	zero, 1408(t2)
	sd	zero, 1416(t2)
	sd	zero, 1424(t2)
	sd	zero, 1432(t2)
	sd	zero, 1440(t2)
	sd	zero, 1448(t2)
	sd	zero, 1456(t2)
	sd	zero, 1464(t2)
	sd	zero, 1472(t2)
	sd	zero, 1480(t2)
	sd	zero, 1488(t2)
	sd	zero, 1496(t2)
	sd	zero, 1504(t2)
	sd	zero, 1512(t2)
	sd	zero, 1520(t2)
	sd	zero, 1528(t2)
	sd	zero, 1536(t2)
	sd	zero, 1544(t2)
	sd	zero, 1552(t2)
	sd	zero, 1560(t2)
	sd	zero, 1568(t2)
	sd	zero, 1576(t2)
	sd	zero, 1584(t2)
	sd	zero, 1592(t2)
	sd	zero, 1600(t2)
	sd	zero, 1608(t2)
	sd	zero, 1616(t2)
	sd	zero, 1624(t2)
	sd	zero, 1632(t2)
	sd	zero, 1640(t2)
	sd	zero, 1648(t2)
	sd	zero, 1656(t2)
	sd	zero, 1664(t2)
	sd	zero, 1672(t2)
	sd	zero, 1680(t2)
	sd	zero, 1688(t2)
	sd	zero, 1696(t2)
	sd	zero, 1704(t2)
	sd	zero, 1712(t2)
	sd	zero, 1720(t2)
	sd	zero, 1728(t2)
	sd	zero, 1736(t2)
	sd	zero, 1744(t2)
	sd	zero, 1752(t2)
	sd	zero, 1760(t2)
	sd	zero, 1768(t2)
	sd	zero, 1776(t2)
	sd	zero, 1784(t2)
	sd	zero, 1792(t2)
	sd	zero, 1800(t2)
	sd	zero, 1808(t2)
	sd	zero, 1816(t2)
	sd	zero, 1824(t2)
	sd	zero, 1832(t2)
	sd	zero, 1840(t2)
	sd	zero, 1848(t2)
	sd	zero, 1856(t2)
	sd	zero, 1864(t2)
	sd	zero, 1872(t2)
	sd	zero, 1880(t2)
	sd	zero, 1888(t2)
	sd	zero, 1896(t2)
	sd	zero, 1904(t2)
	sd	zero, 1912(t2)
	sd	zero, 1920(t2)
	sd	zero, 1928(t2)
	sd	zero, 1936(t2)
	sd	zero, 1944(t2)
	sd	zero, 1952(t2)
	sd	zero, 1960(t2)
	sd	zero, 1968(t2)
	sd	zero, 1976(t2)
	sd	zero, 1984(t2)
	sd	zero, 1992(t2)
	sd	zero, 2000(t2)
	sd	zero, 2008(t2)
	sd	zero, 2016(t2)
	sd	zero, 2024(t2)
	sd	zero, 2032(t2)
	sd	zero, 2040(t2)
	li	s0, 2048
	add	s0, t2, s0
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	addi	s0, s0, 8
	sd	zero, 0(s0)
	li	s0, 12504
	add	s0, t2, s0
	sw	zero, 0(s0)
	li	t0, 12528
	add	t2, sp, t0
	li	t3, 4556
	add	t2, t2, t3
	addi	t2, t2, 4
	li	t3, 6
	sw	t3, 0(t2)
	li	t0, 12528
	add	t2, sp, t0
	li	t3, 4556
	add	t2, t2, t3
	addi	t2, t2, 12
	li	t3, 7
	sw	t3, 0(t2)
	li	t0, 12528
	add	t2, sp, t0
	li	t3, 4556
	add	t2, t2, t3
	addi	t2, t2, 16
	li	t3, 4
	sw	t3, 0(t2)
	li	t0, 12528
	add	t2, sp, t0
	li	t3, 4556
	add	t2, t2, t3
	addi	t2, t2, 28
	li	t3, 9
	sw	t3, 0(t2)
	li	t0, 12528
	add	t2, sp, t0
	li	t3, 4556
	add	t2, t2, t3
	addi	t2, t2, 44
	li	t3, 11
	sw	t3, 0(t2)
	addi	t2, sp, 20
	addi	t2, t2, 1416
	addi	t2, t2, 4
	li	t3, 1
	sw	t3, 0(t2)
	addi	t2, sp, 20
	addi	t2, t2, 1416
	addi	t2, t2, 8
	li	t3, 2
	sw	t3, 0(t2)
	addi	t2, sp, 20
	addi	t2, t2, 1416
	addi	t2, t2, 12
	li	t3, 3
	sw	t3, 0(t2)
	addi	t2, sp, 20
	addi	t2, t2, 1416
	addi	t2, t2, 36
	li	t3, 9
	sw	t3, 0(t2)
	li	t0, 12528
	add	t2, sp, t0
	li	t3, 4556
	add	t2, t2, t3
	addi	t2, t2, 4
	lw	t2, 0(t2)
	sext.w	t2, t2
	addi	t3, sp, 20
	li	t0, 12528
	add	t4, sp, t0
	li	t5, 4556
	add	t4, t4, t5
	addi	t4, t4, 12
	lw	t4, 0(t4)
	sext.w	t4, t4
	li	t0, 12528
	add	t5, sp, t0
	li	t6, 4556
	add	t5, t5, t6
	addi	t6, sp, 20
	addi	t6, t6, 1416
	addi	t6, t6, 12
	lw	t6, 0(t6)
	sext.w	t6, t6
	addi	a0, sp, 20
	addi	a0, a0, 1416
	lw	a0, 0(a0)
	sext.w	a0, a0
	addi	a1, sp, 20
	addi	a1, a1, 1416
	addi	a2, sp, 20
	li	a3, 8024
	add	a2, a2, a3
	addi	a2, a2, 16
	lw	a2, 0(a2)
	sext.w	a2, a2
	addi	a3, sp, 20
	li	a4, 12036
	add	a3, a3, a4
	addi	a3, a3, 72
	lw	a3, 0(a3)
	sext.w	a3, a3
	sw	a3, -4(sp)
	mv	a7, a2
	mv	a6, a1
	mv	a5, a0
	mv	a4, t6
	mv	a3, t5
	mv	a2, t4
	mv	a1, t3
	mv	a0, t2
	call	func
	mv	t2, a0
	li	t3, 3
	mulw	t2, t2, t3
	sw	t2, 16(sp)
main.while_entry_0:
	lw	t2, 16(sp)
	sext.w	t2, t2
	li	t3, 0
	slt	t2, t2, t3
	xori	t2, t2, 1
	beq	t2, zero, main.while_end_0
main.while_body_0:
	addi	t2, sp, 20
	addi	t2, t2, 1416
	lw	t3, 16(sp)
	sext.w	t3, t3
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t2, 0(t2)
	sext.w	t2, t2
	mv	a0, t2
	call	putint
	li	a0, 32
	call	putch
	lw	t2, 16(sp)
	sext.w	t2, t2
	addi	t2, t2, -1
	sw	t2, 16(sp)
	j	main.while_entry_0
main.while_end_0:
	li	a0, 10
	call	putch
	li	a0, 0
	ld	s0, 0(sp)
	ld	ra, 8(sp)
	li	t0, 28876
	add	sp, sp, t0
	ret

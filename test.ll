@A = global[3 x [2 x [5 x i32]]]  zeroinitializer
@a = global i32 0, align 4
@b = global i32 10000, align 4
@c = global float 0x3FF0000000000000, align 4
@d = global float 0x0000000000000000, align 4

declare i32 @getint()
declare i32 @getch()
declare i32 @getarray(i32* noundef)
declare float @getfloat()
declare i32 @getfarray(float* noundef)
declare void @putint(i32 noundef)
declare void @putch(i32 noundef)
declare void @putarray(i32 noundef, i32* noundef)
declare void @putfloat(float noundef)
declare void @putfarray(i32 noundef, float* noundef)
declare void @starttime()
declare void @stoptime()
declare void @llvm.memset.p018.i64(i8* noundef, i8 noundef, i64 noundef, i1 noundef)

define float @add(float noundef %a_0, float noundef %b_0) {
_entry:
  %replace_phi_0 = alloca i1, align 1
  %a_1 = alloca float, align 4
  %b_1 = alloca float, align 4
  %x_0 = alloca float, align 4

  store float %a_0, float* %a_1, align 4
  store float %b_0, float* %b_1, align 4
; init x
  store float 0x3FE0000000000000, float* %x_0, align 4

  %0 = load float, float* %a_1, align 4
  %1 = load float, float* %b_1, align 4
  %2 = fadd float %0, %1
  %3 = load float, float* %x_0, align 4
  %4 = fadd float %2, %3
  ret float %4

}

define i32 @main() {
_entry:
  %replace_phi_0 = alloca i1, align 1
  %arr_0 = alloca [4 x i32], align 16
  %f1_0 = alloca float, align 4
  %f2_0 = alloca float, align 4
  %f_0 = alloca float, align 4

; init arr
  %0 = bitcast [4 x i32]* %arr_0 to i8*
  call void @llvm.memset.p018.i64(i8* noundef %0, i8 noundef 0, i64 noundef 16, i1 noundef false)
  %1 = getelementptr inbounds [4 x i32], [4 x i32]* %arr_0, i32 0, i32 0
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [4 x i32], [4 x i32]* %arr_0, i32 0, i32 1
  store i32 2, i32* %2, align 4

; init f1
  store float 0x400921FB60000000, float* %f1_0, align 4

; init f2
  store float 0x4003333340000000, float* %f2_0, align 4

; init f
  store float 0x4000000000000000, float* %f_0, align 4

  %3 = getelementptr inbounds [3 x [2 x [5 x i32]]], [3 x [2 x [5 x i32]]]* @A, i32 0, i32 2
  %4 = getelementptr inbounds [2 x [5 x i32]], [2 x [5 x i32]]* %3, i32 0, i32 1
  %5 = getelementptr inbounds [5 x i32], [5 x i32]* %4, i32 0, i32 2
  %6 = getelementptr inbounds [3 x [2 x [5 x i32]]], [3 x [2 x [5 x i32]]]* @A, i32 0, i32 1
  %7 = getelementptr inbounds [2 x [5 x i32]], [2 x [5 x i32]]* %6, i32 0, i32 2
  %8 = getelementptr inbounds [5 x i32], [5 x i32]* %7, i32 0, i32 2
  %9 = load i32, i32* %8, align 4
  %10 = add i32 188, %9
  store i32 %10, i32* %5, align 4
  %11 = load i32, i32* @a, align 4
  %12 = mul i32 %11, 4
  call void @putint(i32 noundef %12)
  call void @putch(i32 noundef 10)
  %13 = load i32, i32* @b, align 4
  %14 = sdiv i32 %13, 4
  call void @putint(i32 noundef %14)
  call void @putch(i32 noundef 10)
  %15 = load float, float* %f1_0, align 4
  call void @putfloat(float noundef %15)
  call void @putch(i32 noundef 10)
  %16 = load float, float* %f2_0, align 4
  call void @putfloat(float noundef %16)
  call void @putch(i32 noundef 10)
  %17 = getelementptr inbounds [3 x [2 x [5 x i32]]], [3 x [2 x [5 x i32]]]* @A, i32 0, i32 2
  %18 = getelementptr inbounds [2 x [5 x i32]], [2 x [5 x i32]]* %17, i32 0, i32 1
  %19 = getelementptr inbounds [5 x i32], [5 x i32]* %18, i32 0, i32 2
  %20 = load i32, i32* %19, align 4
  call void @putint(i32 noundef %20)
  call void @putch(i32 noundef 10)
  ret i32 0

}


@A = global[3 x [2 x [5 x i32]]]  zeroinitializer

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

  %0 = getelementptr inbounds [3 x [2 x [5 x i32]]], [3 x [2 x [5 x i32]]]* @A, i32 0, i32 2
  %1 = getelementptr inbounds [2 x [5 x i32]], [2 x [5 x i32]]* %0, i32 0, i32 1
  %2 = getelementptr inbounds [5 x i32], [5 x i32]* %1, i32 0, i32 2
  %3 = getelementptr inbounds [3 x [2 x [5 x i32]]], [3 x [2 x [5 x i32]]]* @A, i32 0, i32 1
  %4 = getelementptr inbounds [2 x [5 x i32]], [2 x [5 x i32]]* %3, i32 0, i32 2
  %5 = getelementptr inbounds [5 x i32], [5 x i32]* %4, i32 0, i32 2
  %6 = load i32, i32* %5, align 4
  %7 = add i32 188, %6
  store i32 %7, i32* %2, align 4
  ret i32 0

}


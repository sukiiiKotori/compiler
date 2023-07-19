
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
declare void @llvm.memset.p0i8.i64(i8* noundef, i8 noundef, i64 noundef, i1 noundef)

define float @add(float noundef %a_0, float noundef %b_0) {
_entry:
  %a_1 = alloca float, align 4
  %b_1 = alloca float, align 4
  %t_0 = alloca float, align 4

  store float %a_0, float* %a_1, align 4
  store float %b_0, float* %b_1, align 4
  store float 0x3FE0000000000000, float* %t_0, align 4
  %0 = load float, float* %t_0, align 4
  %1 = load float, float* %a_1, align 4
  %2 = fadd float %0, %1
  %3 = load float, float* %b_1, align 4
  %4 = fadd float %2, %3
  ret float %4
}

define i32 @main() {
_entry:
  %f_0 = alloca float, align 4
  %c2_0 = alloca float, align 4

  store float 0x3FF0000000000000, float* %f_0, align 4
  store float 0x4000000000000000, float* %c2_0, align 4
  %0 = load float, float* %f_0, align 4
  %1 = load float, float* %c2_0, align 4
  %2 = call float @add(float noundef %0, float noundef %1)
  ret i32 0
}



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

define i32 @main() {
_entry:
  %replace_phi_0 = alloca i1, align 1
  %a_0 = alloca i32, align 4
  %b_0 = alloca i32, align 4
  %c_0 = alloca i32, align 4

; init a
  store i32 4, i32* %a_0, align 4

; init b
  %0 = load i32, i32* %a_0, align 4
  %1 = sitofp i32 %0 to float
  %2 = fmul float %1, 0xC020000000000000
  %3 = fptosi float %2 to i32
  store i32 %3, i32* %b_0, align 4

; init c
  store i32 2147483647, i32* %c_0, align 4

  ret i32 0

}


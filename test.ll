
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
  %d_0 = alloca i32, align 4

  store i32 10, i32* %a_0, align 4
  store i32 4, i32* %b_0, align 4
  store i32 2, i32* %c_0, align 4
  store i32 2, i32* %d_0, align 4
  %0 = load i32, i32* %c_0, align 4
  %1 = load i32, i32* %a_0, align 4
  %2 = add i32 %0, %1
  %3 = load i32, i32* %b_0, align 4
  %4 = load i32, i32* %d_0, align 4
  %5 = sub i32 %3, %4
  %6 = mul i32 %2, %5
  ret i32 %6

}


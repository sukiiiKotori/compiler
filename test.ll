
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
  %arr_0 = alloca [4 x i32], align 16

; init arr
  %0 = bitcast [4 x i32]* %arr_0 to i8*
  call void @llvm.memset.p018.i64(i8* noundef %0, i8 noundef 0, i64 noundef 16, i1 noundef false)
  %1 = getelementptr inbounds [4 x i32], [4 x i32]* %arr_0, i32 0, i32 0
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [4 x i32], [4 x i32]* %arr_0, i32 0, i32 1
  store i32 2, i32* %2, align 4

  ret i32 0

}


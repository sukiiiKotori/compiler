@A = global[5 x [6 x [7 x i32]]]  zeroinitializer

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

define i32 @main() {
_entry:
  %0 = getelementptr inbounds [5 x [6 x [7 x i32]]], [5 x [6 x [7 x i32]]]* @A, i32 0, i32 1
  %1 = getelementptr inbounds [6 x [7 x i32]], [6 x [7 x i32]]* %0, i32 0, i32 2
  %2 = getelementptr inbounds [7 x i32], [7 x i32]* %1, i32 0, i32 3
  store i32 5, i32* %2, align 4
  ret i32 0
}


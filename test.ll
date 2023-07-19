@A = global[5 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4]
@B = global[4 x i32]  zeroinitializer
@a = global float 0x3FF0000000000000, align 4
@b = global float 0x4004000000000000, align 4

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
  %replace_phi_0 = alloca i1, align 1
  %a_0 = alloca i32, align 4

; init a
  %0 = getelementptr inbounds [5 x i32], [5 x i32]* @A, i32 0, i32 3
  %1 = load i32, i32* %0, align 4
  store i32 %1, i32* %a_0, align 4

  ret i32 0

}


@A = global[5 x i32]  [i32 0, i32 1, i32 2, i32 3, i32 4]
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
  %farr_0 = alloca [5 x float], align 16

; init a
  %0 = getelementptr inbounds [5 x i32], [5 x i32]* @A, i32 0, i32 3
  %1 = load i32, i32* %0, align 4
  store i32 %1, i32* %a_0, align 4

; init farr
  %2 = bitcast [5 x float]* %farr_0 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef %2, i8 noundef 0, i64 noundef 20, i1 noundef false)
  %3 = getelementptr inbounds [5 x float], [5 x float]* %farr_0, i32 0, i32 0
  store float 0x3FB99999A0000000, float* %3, align 4
  %4 = getelementptr inbounds [5 x float], [5 x float]* %farr_0, i32 0, i32 1
  store float 0x3FC99999A0000000, float* %4, align 4
  %5 = getelementptr inbounds [5 x float], [5 x float]* %farr_0, i32 0, i32 2
  store float 0x3FD3333340000000, float* %5, align 4
  %6 = getelementptr inbounds [5 x float], [5 x float]* %farr_0, i32 0, i32 3
  store float 0x3FD99999A0000000, float* %6, align 4
  %7 = getelementptr inbounds [5 x float], [5 x float]* %farr_0, i32 0, i32 4
  store float 0x3FE0000000000000, float* %7, align 4

  ret i32 0

}


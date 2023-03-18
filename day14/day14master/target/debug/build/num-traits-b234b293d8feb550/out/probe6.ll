; ModuleID = 'probe6.aee1cd56-cgu.0'
source_filename = "probe6.aee1cd56-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc_f20b8b6500101404d6ee98fdd20700b9 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/13afbdaa0655dda23d7129e59ac48f1ec88b2084/library/core/src/num/mod.rs" }>, align 1
@alloc_2dc552aee949b42ddc24bdd71208bb59 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_f20b8b6500101404d6ee98fdd20700b9, [16 x i8] c"K\00\00\00\00\00\00\00/\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe65probe17h5a9a634f066a030eE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h95bfd5f21b4c018bE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h54e8b1f0022d1156E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_2dc552aee949b42ddc24bdd71208bb59) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h95bfd5f21b4c018bE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h54e8b1f0022d1156E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}

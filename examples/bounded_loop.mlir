func.func @compute_sum(%n: i64) {
      %v1 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
      %v2 = llvm.alloca 1 x i64 : (!llvm.ptr)
      llvm.store %v1, %v2 : !llvm.ptr
      %v3 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
      %v4 = llvm.alloca 1 x i64 : (!llvm.ptr)
      llvm.store %v3, %v4 : !llvm.ptr
    scf.while (  %v12 = arith.cmpi sle, %v11, %v0 : i64) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
        %v5 = llvm.load %v2 : !llvm.ptr -> i64
        %v6 = llvm.load %v4 : !llvm.ptr -> i64
        %v7 = arith.addi %v5, %v6 : i64
        llvm.store %v7, %v2 : !llvm.ptr
        %v8 = llvm.load %v4 : !llvm.ptr -> i64
        %v9 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
        %v10 = arith.addi %v8, %v9 : i64
        llvm.store %v10, %v4 : !llvm.ptr
      scf.yield
  }
    func.return   %v13 = llvm.load %v2 : !llvm.ptr -> i64
    func.return
}
func.call @compute_sum(  %v14 = pirtm.operator_atom 10 {receipt = "lit"} : !pirtm.stratum) : () -> ()
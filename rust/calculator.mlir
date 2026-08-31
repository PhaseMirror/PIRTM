func @main() {
  pirtm.import "std::option::Option"
  pirtm.import "std::result::Result"
func.func private @printf(llvm.struct_str, llvm.struct_i64) -> llvm.struct_i1 attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @puts(llvm.struct_str) -> llvm.struct_i1 attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @panic(llvm.struct_str) -> llvm.struct_never attributes {llvm.linkage = #llvm.linkage<external>}
!llvm.enum_Op = type { i32, i64 }
func.func @calculate(%a: i64, %b: i64, %op: i64) {
    scf.switch   %v0 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
        %v1 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      scf.yield
    }
    case 1 {
        %v2 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      scf.yield
    }
    case 2 {
        %v3 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      scf.yield
    }
    case 3 {
      scf.if   %v8 = pirtm.binary_eq %v5, %v7 {receipt = "bin"} : (!pirtm.stratum, !pirtm.stratum) -> !pirtm.stratum {
      %v9 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
  } else {
      %v10 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
  }
      scf.yield
    }
    
  }
    return
}
func.func @main() {
      %v11 = pirtm.operator_atom 10 {receipt = "lit"} : !pirtm.stratum
      %v12 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
      %v13 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    scf.switch   %v13 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
        %v14 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v15 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      scf.yield
    }
    case 1 {
        %v16 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v17 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
      scf.yield
    }
    
  }
    return
}
  %v18 = pirtm.operator_atom 0 {receipt = "ret"} : !pirtm.stratum
}
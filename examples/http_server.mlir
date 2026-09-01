  pirtm.import "std::net::Listener"
  pirtm.import "std::net::Connection"
  pirtm.import "std::net::get_spectral_rho"
  pirtm.import "std::net::log_audit_request"
  pirtm.import "std::string::String"
  pirtm.import "std::io::print"
func.func @respond(%conn: i64, %status_code: i64, %body: i64) {
    func.call @String::from_str(  %v3 = llvm.mlir.constant("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n") : !llvm.ptr) : () -> ()
      %v4 = llvm.alloca 1 x i64 : (!llvm.ptr)
      llvm.store %v_unknown, %v4 : !llvm.ptr
      %v5 = llvm.load %v4 : !llvm.ptr -> i64
      %v2 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v6 = func.call @string_concat(%v5, %v2) : () -> !pirtm.stratum
      llvm.store %v6, %v4 : !llvm.ptr
      %v0 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v7 = llvm.load %v4 : !llvm.ptr -> i64
      %v8 = func.call @string_to_str(%v7) : () -> !pirtm.stratum
      %v9 = func.call @write(%v0, %v8) : () -> !pirtm.stratum
    func.return
}
func.func @handle_request(%conn: i64, %request: i64) {
    func.call @String::from_str(  %v12 = llvm.mlir.constant("{\"status\":\"UP\",\"governance\":\"LOCKED\",\"spectral_rho\":0.0}") : !llvm.ptr) : () -> ()
      %v13 = llvm.alloca 1 x i64 : (!llvm.ptr)
      llvm.store %v_unknown, %v13 : !llvm.ptr
    func.call @respond(  %v10 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum,   %v14 = pirtm.operator_atom 200 {receipt = "lit"} : !pirtm.stratum,   %v15 = llvm.load %v13 : !llvm.ptr -> i64) : () -> ()
    func.call @log_audit_request(  %v16 = llvm.mlir.constant("/status") : !llvm.ptr,   %v17 = pirtm.operator_atom 200 {receipt = "lit"} : !pirtm.stratum) : () -> ()
    func.return
}
func.func @main() {
    func.call @Listener::listen(  %v18 = pirtm.operator_atom 8080 {receipt = "lit"} : !pirtm.stratum) : () -> ()
    scf.switch   %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
      func.call @print(  %v19 = llvm.mlir.constant("PIRTM Governed HTTP/1.1 Micro-Server listening on port 8080") : !llvm.ptr) : () -> ()
        %v20 = arith.constant 1 : i1
        %v21 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v20, %v21 : !llvm.ptr
        %v22 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
        %v23 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v22, %v23 : !llvm.ptr
      scf.while (  %v42 = scf.if %v38 -> (i1) {
    scf.yield %v41 : i1
  } else {
    %c0_v42 = arith.constant false
    scf.yield %c0_v42 : i1
  }) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
        %v24 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v25 = func.call @accept(%v24) : () -> !pirtm.stratum
      scf.switch   %v25 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
        %v26 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v27 = func.call @read(%v26) : () -> !pirtm.stratum
      scf.switch   %v27 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
      func.call @handle_request(  %v28 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum,   %v29 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
        %v30 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v31 = func.call @close(%v30) : () -> !pirtm.stratum
        %v32 = llvm.load %v23 : !llvm.ptr -> i64
        %v33 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
        %v34 = arith.addi %v32, %v33 : i64
        llvm.store %v34, %v23 : !llvm.ptr
      scf.yield
    }
    case 1 {
        %v35 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v36 = func.call @close(%v35) : () -> !pirtm.stratum
      scf.yield
    }
    
  }
      scf.yield
    }
    case 1 {
        %v37 = arith.constant 0 : i1
        llvm.store %v37, %v21 : !llvm.ptr
      scf.yield
    }
    
  }
      scf.yield
  }
        %v43 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v44 = func.call @close(%v43) : () -> !pirtm.stratum
      func.return   %v45 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
      scf.yield
    }
    case 1 {
      func.call @print(  %v46 = llvm.mlir.constant("Failed to bind listener on port 8080") : !llvm.ptr) : () -> ()
      func.return   %v47 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
      scf.yield
    }
    
  }
    func.return
}
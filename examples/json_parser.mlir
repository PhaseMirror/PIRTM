  pirtm.import "std::option::Option"
  pirtm.import "std::result::Result"
  pirtm.import "std::vec::Vec"
  pirtm.import "std::string::String"
  pirtm.import "std::map::Map"
  pirtm.import "std::char"
  pirtm.import "std::str"
  pirtm.import "std::io::read_file"
  pirtm.import "std::io::print"
  pirtm.import "std::convert::parse_f64"
  pirtm.import "std::convert::f64_to_string"
func.func private @parser_new(llvm.struct_ptr) -> llvm.struct_ptr attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @parser_peek(llvm.struct_ptr) -> llvm.struct_i64 attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @parser_skip_whitespace(llvm.struct_ptr) attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @parser_consume_literal(llvm.struct_ptr, llvm.struct_ptr) -> llvm.struct_bool attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @parser_parse_string(llvm.struct_ptr) -> llvm.struct_ptr attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @parser_parse_number(llvm.struct_ptr) -> f64 attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @string_from_cstr(llvm.struct_ptr) -> llvm.struct_ptr attributes {llvm.linkage = #llvm.linkage<external>}
func.func private @string_concat(llvm.struct_ptr, llvm.struct_ptr) -> llvm.struct_ptr attributes {llvm.linkage = #llvm.linkage<external>}
!llvm.enum_JsonValue = type { i32, i64 }
func.func @parse_value(%p: i64) {
      %v0 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      %v1 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    scf.if   %v5 = pirtm.binary_eq %v2, %v4 {receipt = "bin"} : (!pirtm.stratum, !pirtm.stratum) -> !pirtm.stratum {
      %v6 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v7 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    func.return
  }
    scf.if   %v11 = pirtm.binary_eq %v8, %v10 {receipt = "bin"} : (!pirtm.stratum, !pirtm.stratum) -> !pirtm.stratum {
      %v12 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v13 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    func.return
  }
    scf.if   %v17 = pirtm.binary_eq %v14, %v16 {receipt = "bin"} : (!pirtm.stratum, !pirtm.stratum) -> !pirtm.stratum {
      %v18 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v19 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    func.return
  }
    scf.if   %v23 = pirtm.binary_eq %v20, %v22 {receipt = "bin"} : (!pirtm.stratum, !pirtm.stratum) -> !pirtm.stratum {
      %v24 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      %v24 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
    %undef_init_llvm.struct_String = llvm.undef : !llvm.struct_String
    %ins_0_llvm.struct_String = llvm.insertvalue %v0, %undef_init_llvm.struct_String[0] : !llvm.struct_String

        %v25 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    func.return
  }
      %v26 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      %v27 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    return
}
func.func @json_to_string(%val: i64) {
    scf.switch   %v28 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
        %v29 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    %undef_init_llvm.struct_String = llvm.undef : !llvm.struct_String
    %ins_0_llvm.struct_String = llvm.insertvalue %v0, %undef_init_llvm.struct_String[0] : !llvm.struct_String

      scf.yield
    }
    case 1 {
        %v30 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v31 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    %undef_init_llvm.struct_String = llvm.undef : !llvm.struct_String
    %ins_0_llvm.struct_String = llvm.insertvalue %v0, %undef_init_llvm.struct_String[0] : !llvm.struct_String

        %v32 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    %undef_init_llvm.struct_String = llvm.undef : !llvm.struct_String
    %ins_0_llvm.struct_String = llvm.insertvalue %v0, %undef_init_llvm.struct_String[0] : !llvm.struct_String

        %v33 = pirtm.operator_atom 0 {receipt = "if"} : !pirtm.stratum
      scf.yield
    }
    case 2 {
        %v34 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    %undef_init_llvm.struct_String = llvm.undef : !llvm.struct_String
    %ins_0_llvm.struct_String = llvm.insertvalue %v0, %undef_init_llvm.struct_String[0] : !llvm.struct_String

      scf.yield
    }
    case 3 {
        %v35 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v36 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v37 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v37 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
    %undef_init_llvm.struct_String = llvm.undef : !llvm.struct_String
    %ins_0_llvm.struct_String = llvm.insertvalue %v0, %undef_init_llvm.struct_String[0] : !llvm.struct_String

      scf.yield
    }
    
  }
    return
}
func.func @main() {
      %v38 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      %v39 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
      %v40 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
    scf.switch   %v40 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
        %v41 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v42 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v43 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
      scf.yield
    }
    case 1 {
        %v44 = pirtm.operator_atom 0 {receipt = "call"} : !pirtm.stratum
        %v45 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
      scf.yield
    }
    
  }
    return
}
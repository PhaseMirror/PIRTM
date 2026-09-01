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
!llvm.enum_JsonValue = type { i32, i64 }
!llvm.struct_Parser = type { !llvm.ptr, i64 }
func.func @new(%input: i64) {
    %undef_init = llvm.undef : !llvm.struct_Parser
    %ins_0 = llvm.insertvalue   %v0 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, %undef_init[0] : !llvm.struct_Parser
    %ins_1 = llvm.insertvalue   %v1 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum, %undef_init[1] : !llvm.struct_Parser
    func.return
}
func.func @peek(%self: i64) {
    scf.if   %v4 = arith.cmpi sge, %v_unknown, %v3 : i64 {
      %v5 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
  } else {
    %ext_field_input = llvm.extractvalue   %v2 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
    %ext_field_pos = llvm.extractvalue   %v2 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
      %v6 = func.call @string_char_at(%v_unknown, %v_unknown) : () -> !pirtm.stratum
  }
    func.return
}
func.func @advance(%self: i64) {
      %v8 = llvm.alloca 1 x i64 : (!llvm.ptr)
    %ext_field_pos = llvm.extractvalue   %v7 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
      %v9 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
      %v10 = arith.addi %v_unknown, %v9 : i64
      llvm.store %v10, %v8 : !llvm.ptr
    func.return
}
func.func @skip_whitespace(%self: i64) {
    scf.while (  %v16 = arith.cmpi eq, %v14, %v15 : i64) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
      scf.if func.call @char::is_whitespace(  %v12 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> () {
      %v11 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v13 = func.call @advance(%v11) : () -> !pirtm.stratum
  } else {
      pirtm.yield %break : !pirtm.stratum
  }
      scf.yield
  }
    func.return
}
func.func @consume_literal(%self: i64, %lit: i64) {
    func.call @str::len(  %v18 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
    scf.if   %v21 = arith.cmpi sgt, %v19, %v20 : i64 {
    func.return   %v22 = arith.constant 0 : i1
  }
    %ext_field_input = llvm.extractvalue   %v17 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
    %ext_field_pos = llvm.extractvalue   %v17 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
    %ext_field_pos = llvm.extractvalue   %v17 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
      %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v23 = arith.addi %v_unknown, %v_unknown : i64
      %v24 = func.call @string_slice(%v_unknown, %v_unknown, %v23) : () -> !pirtm.stratum
    scf.if   %v26 = arith.cmpi eq, %v25, %v18 : i64 {
      %v27 = llvm.alloca 1 x i64 : (!llvm.ptr)
    %ext_field_pos = llvm.extractvalue   %v17 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
      %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v28 = arith.addi %v_unknown, %v_unknown : i64
      llvm.store %v28, %v27 : !llvm.ptr
      %v29 = arith.constant 1 : i1
  } else {
      %v30 = arith.constant 0 : i1
  }
    func.return
}
func.func @parse_string(%self: i64) {
      %v31 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v32 = func.call @advance(%v31) : () -> !pirtm.stratum
    %ext_field_pos = llvm.extractvalue   %v31 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
    scf.while (  %v43 = arith.cmpi eq, %v41, %v42 : i64) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
      scf.if   %v35 = arith.cmpi eq, %v33, %v34 : i64 {
      pirtm.yield %break : !pirtm.stratum
  }
      scf.if   %v38 = arith.cmpi eq, %v36, %v37 : i64 {
      %v31 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v39 = func.call @advance(%v31) : () -> !pirtm.stratum
  }
        %v31 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v40 = func.call @advance(%v31) : () -> !pirtm.stratum
      scf.yield
  }
    scf.if   %v46 = arith.cmpi ne, %v44, %v_unknown : i64 {
    func.return func.call @Result::Err(  %v47 = llvm.mlir.constant("unterminated string") : !llvm.ptr) : () -> ()
  }
    %ext_field_pos = llvm.extractvalue   %v31 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
      %v31 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v48 = func.call @advance(%v31) : () -> !pirtm.stratum
    %ext_field_input = llvm.extractvalue   %v31 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
      %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v49 = func.call @string_slice(%v_unknown, %v_unknown, %v_unknown) : () -> !pirtm.stratum
    func.return
}
func.func @parse_number(%self: i64) {
    %ext_field_pos = llvm.extractvalue   %v50 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
    scf.if   %v53 = arith.cmpi eq, %v51, %v_unknown : i64 {
      %v50 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v54 = func.call @advance(%v50) : () -> !pirtm.stratum
  }
    scf.while (  %v63 = arith.cmpi eq, %v61, %v62 : i64) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
      scf.if   %v59 = scf.if %v_unknown -> (i1) {
    %c1_v59 = arith.constant true
    scf.yield %c1_v59 : i1
  } else {
    scf.yield %v58 : i1
  } {
      %v50 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v60 = func.call @advance(%v50) : () -> !pirtm.stratum
  } else {
      pirtm.yield %break : !pirtm.stratum
  }
      scf.yield
  }
    %ext_field_pos = llvm.extractvalue   %v50 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
    %ext_field_input = llvm.extractvalue   %v50 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum, 0
      %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v64 = func.call @string_slice(%v_unknown, %v_unknown, %v_unknown) : () -> !pirtm.stratum
    func.call @parse_f64(  %v65 = func.call @string_to_str(%v64) : () -> !pirtm.stratum) : () -> ()
    func.return
}
func.func @parse_value(%p: i64) {
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v67 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v68 = func.call @peek(%v66) : () -> !pirtm.stratum
    scf.switch   %v68 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
      scf.if   %c_true_v71 = arith.constant true
  %v71 = arith.xori %v70, %c_true_v71 : i1 {
    func.return func.call @Result::Err(  %v72 = llvm.mlir.constant("expected null") : !llvm.ptr) : () -> ()
  }
        %v73 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      scf.yield
    }
    case 1 {
      scf.if   %c_true_v76 = arith.constant true
  %v76 = arith.xori %v75, %c_true_v76 : i1 {
    func.return func.call @Result::Err(  %v77 = llvm.mlir.constant("expected true") : !llvm.ptr) : () -> ()
  }
      func.call @JsonValue::Bool(  %v78 = arith.constant 1 : i1) : () -> ()
      scf.yield
    }
    case 2 {
      scf.if   %c_true_v81 = arith.constant true
  %v81 = arith.xori %v80, %c_true_v81 : i1 {
    func.return func.call @Result::Err(  %v82 = llvm.mlir.constant("expected false") : !llvm.ptr) : () -> ()
  }
      func.call @JsonValue::Bool(  %v83 = arith.constant 0 : i1) : () -> ()
      scf.yield
    }
    case 3 {
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v84 = func.call @parse_string(%v66) : () -> !pirtm.stratum
        %v85 = func.call @option_unwrap(%v84) : () -> !pirtm.stratum
      func.call @JsonValue::String(  %v85 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
      scf.yield
    }
    case 4 {
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v86 = func.call @advance(%v66) : () -> !pirtm.stratum
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v87 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
      func.call @Vec::new() : () -> ()
        %v88 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v_unknown, %v88 : !llvm.ptr
      scf.if   %v91 = arith.cmpi eq, %v89, %v_unknown : i64 {
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v92 = func.call @advance(%v66) : () -> !pirtm.stratum
    func.return func.call @JsonValue::Array(  %v93 = llvm.load %v88 : !llvm.ptr -> i64) : () -> ()
  }
      scf.while (  %1 = arith.constant 1 : i1) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
      func.call @parse_value(  %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
        %v94 = func.call @option_unwrap(%v_unknown) : () -> !pirtm.stratum
        %v95 = llvm.load %v88 : !llvm.ptr -> i64
        %v94 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v96 = func.call @vec_push(%v95, %v94) : () -> !pirtm.stratum
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v97 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
      scf.if   %v100 = arith.cmpi eq, %v98, %v_unknown : i64 {
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v101 = func.call @advance(%v66) : () -> !pirtm.stratum
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v102 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
  } else {
    scf.if   %v105 = arith.cmpi eq, %v103, %v_unknown : i64 {
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v106 = func.call @advance(%v66) : () -> !pirtm.stratum
      pirtm.yield %break : !pirtm.stratum
  } else {
    func.return func.call @Result::Err(  %v107 = llvm.mlir.constant("expected ',' or ']'") : !llvm.ptr) : () -> ()
  }
  }
      scf.yield
  }
      func.call @JsonValue::Array(  %v108 = llvm.load %v88 : !llvm.ptr -> i64) : () -> ()
      scf.yield
    }
    case 5 {
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v109 = func.call @advance(%v66) : () -> !pirtm.stratum
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v110 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
      func.call @Map::new() : () -> ()
        %v111 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v_unknown, %v111 : !llvm.ptr
      scf.if   %v114 = arith.cmpi eq, %v112, %v_unknown : i64 {
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v115 = func.call @advance(%v66) : () -> !pirtm.stratum
    func.return func.call @JsonValue::Object(  %v116 = llvm.load %v111 : !llvm.ptr -> i64) : () -> ()
  }
      scf.while (  %1 = arith.constant 1 : i1) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v117 = func.call @parse_string(%v66) : () -> !pirtm.stratum
        %v118 = func.call @option_unwrap(%v117) : () -> !pirtm.stratum
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v119 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
      scf.if   %v122 = arith.cmpi ne, %v120, %v_unknown : i64 {
    func.return func.call @Result::Err(  %v123 = llvm.mlir.constant("expected ':'") : !llvm.ptr) : () -> ()
  }
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v124 = func.call @advance(%v66) : () -> !pirtm.stratum
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v125 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
      func.call @parse_value(  %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
        %v126 = func.call @option_unwrap(%v_unknown) : () -> !pirtm.stratum
        %v127 = llvm.load %v111 : !llvm.ptr -> i64
        %v118 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v126 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v128 = func.call @map_insert(%v127, %v118, %v126) : () -> !pirtm.stratum
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v129 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
      scf.if   %v132 = arith.cmpi eq, %v130, %v_unknown : i64 {
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v133 = func.call @advance(%v66) : () -> !pirtm.stratum
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v134 = func.call @skip_whitespace(%v66) : () -> !pirtm.stratum
  } else {
    scf.if   %v137 = arith.cmpi eq, %v135, %v_unknown : i64 {
      %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      %v138 = func.call @advance(%v66) : () -> !pirtm.stratum
      pirtm.yield %break : !pirtm.stratum
  } else {
    func.return func.call @Result::Err(  %v139 = llvm.mlir.constant("expected ',' or '}'") : !llvm.ptr) : () -> ()
  }
  }
      scf.yield
  }
      func.call @JsonValue::Object(  %v140 = llvm.load %v111 : !llvm.ptr -> i64) : () -> ()
      scf.yield
    }
    case 6 {
        %v66 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v141 = func.call @parse_number(%v66) : () -> !pirtm.stratum
        %v142 = func.call @option_unwrap(%v141) : () -> !pirtm.stratum
      func.call @JsonValue::Number(  %v142 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
      scf.yield
    }
    case 7 {
      func.call @Result::Err(  %v143 = llvm.mlir.constant("unexpected token") : !llvm.ptr) : () -> ()
      scf.yield
    }
    
  }
    func.return
}
func.func @json_to_string(%val: i64) {
    scf.switch   %v144 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
      func.call @String::from_str(  %v145 = llvm.mlir.constant("null") : !llvm.ptr) : () -> ()
      scf.yield
    }
    case 1 {
        %v146 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
      func.call @String::from_str(  %v147 = llvm.mlir.constant("true") : !llvm.ptr) : () -> ()
      func.call @String::from_str(  %v148 = llvm.mlir.constant("false") : !llvm.ptr) : () -> ()
        %v149 = pirtm.operator_atom 0 {receipt = "if"} : !pirtm.stratum
      scf.yield
    }
    case 2 {
      func.call @f64_to_string(  %v150 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
      scf.yield
    }
    case 3 {
      func.call @String::from_str(  %v151 = llvm.mlir.constant("\"") : !llvm.ptr) : () -> ()
        %v152 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v_unknown, %v152 : !llvm.ptr
        %v153 = llvm.load %v152 : !llvm.ptr -> i64
        %v154 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v155 = func.call @string_concat(%v153, %v154) : () -> !pirtm.stratum
        llvm.store %v155, %v152 : !llvm.ptr
        %v156 = llvm.load %v152 : !llvm.ptr -> i64
      func.call @String::from_str(  %v157 = llvm.mlir.constant("\"") : !llvm.ptr) : () -> ()
        %v158 = func.call @string_concat(%v156, %v_unknown) : () -> !pirtm.stratum
        llvm.store %v158, %v152 : !llvm.ptr
        %v159 = llvm.load %v152 : !llvm.ptr -> i64
      scf.yield
    }
    case 4 {
      func.call @String::from_str(  %v160 = llvm.mlir.constant("[") : !llvm.ptr) : () -> ()
        %v161 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v_unknown, %v161 : !llvm.ptr
        %v162 = arith.constant 1 : i1
        %v163 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v162, %v163 : !llvm.ptr
        %v164 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum
        %v165 = func.call @string_len(%v164) : () -> !pirtm.stratum
        %v166 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
        %v167 = llvm.alloca 1 x i64 : (!llvm.ptr)
        llvm.store %v166, %v167 : !llvm.ptr
      scf.while (  %v184 = arith.cmpi slt, %v183, %v165 : i64) : (i1) -> () {
    ^bb0(%arg0: i1):
      scf.condition(%arg0)
  } do {
    ^bb0:
      scf.if   %c_true_v169 = arith.constant true
  %v169 = arith.xori %v168, %c_true_v169 : i1 {
      %v170 = llvm.load %v161 : !llvm.ptr -> i64
    func.call @String::from_str(  %v171 = llvm.mlir.constant(", ") : !llvm.ptr) : () -> ()
      %v172 = func.call @string_concat(%v170, %v_unknown) : () -> !pirtm.stratum
      llvm.store %v172, %v161 : !llvm.ptr
  }
        %v173 = llvm.load %v161 : !llvm.ptr -> i64
      func.call @json_to_string(  %v177 = func.call @option_unwrap(%v176) : () -> !pirtm.stratum) : () -> ()
        %v178 = func.call @string_concat(%v173, %v_unknown) : () -> !pirtm.stratum
        llvm.store %v178, %v161 : !llvm.ptr
        %v179 = arith.constant 0 : i1
        llvm.store %v179, %v163 : !llvm.ptr
        %v180 = llvm.load %v167 : !llvm.ptr -> i64
        %v181 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
        %v182 = arith.addi %v180, %v181 : i64
        llvm.store %v182, %v167 : !llvm.ptr
      scf.yield
  }
        %v185 = llvm.load %v161 : !llvm.ptr -> i64
      func.call @String::from_str(  %v186 = llvm.mlir.constant("]") : !llvm.ptr) : () -> ()
        %v187 = func.call @string_concat(%v185, %v_unknown) : () -> !pirtm.stratum
      scf.yield
    }
    case 5 {
      func.call @String::from_str(  %v188 = llvm.mlir.constant("{...}") : !llvm.ptr) : () -> ()
      scf.yield
    }
    
  }
    func.return
}
func.func @main() {
    func.call @read_file(  %v189 = llvm.mlir.constant("input.json") : !llvm.ptr) : () -> ()
    func.call @Parser::new(  %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
      %v190 = llvm.alloca 1 x i64 : (!llvm.ptr)
      llvm.store %v_unknown, %v190 : !llvm.ptr
    func.call @parse_value(  %v191 = llvm.load %v190 : !llvm.ptr -> i64) : () -> ()
    scf.switch   %v_unknown = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum {
    case 0 {
      func.call @json_to_string(  %v192 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
      func.call @print(  %v193 = func.call @string_to_str(%v_unknown) : () -> !pirtm.stratum) : () -> ()
        %v194 = pirtm.operator_atom 0 {receipt = "lit"} : !pirtm.stratum
      scf.yield
    }
    case 1 {
      func.call @print(  %v195 = llvm.mlir.constant("Parse error: ") : !llvm.ptr) : () -> ()
      func.call @print(  %v196 = pirtm.operator_atom 2 {receipt = "ident"} : !pirtm.stratum) : () -> ()
        %v197 = pirtm.operator_atom 1 {receipt = "lit"} : !pirtm.stratum
      scf.yield
    }
    
  }
    func.return
}
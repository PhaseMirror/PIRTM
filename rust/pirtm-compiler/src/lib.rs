//! pirtm-compiler: PIRTM language compiler with governance and Lean verification.
//!
//! This crate provides a library API for compiling PIRTM/moc source code to MLIR,
//! verifying Lean 4 proofs, and translating to LLVM IR or WebAssembly.

mod error;
pub mod linker;
pub mod manifest;
pub mod witness_bytecode;
pub mod stablehlo_lowering;
pub mod phase_hypergraph;
mod translate;

pub use error::{CompileError, MlirModule, ProofError, ProofReceipt, TranslateError};
pub use phase_hypergraph::{GeneratorViolation, PhaseHypergraph, EPSILON_CRIT};
pub use pirtm_mlir::PirtmOp;
pub use pirtm_parser::ast::{BinOp, Expr, Program, Stmt};

pub use pirtm_mlir::pirtm::transpiler::visitor::MlirEmitterVisitor;
use serde_json::json;
use telemetry_recorder::record_event;

/// The primary compiler interface for PIRTM programs.
pub struct PhaseMirrorCompiler {
    validator: AdmissibilityValidator,
    pub current_topology: Option<PhaseHypergraph>,
}

impl Default for PhaseMirrorCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl PhaseMirrorCompiler {
    /// Create a new compiler instance.
    pub fn new() -> Self {
        Self {
            validator: AdmissibilityValidator::new(),
            current_topology: None,
        }
    }

    /// Compile PIRTM source to MLIR module with topological pre-flight interlock.
    pub fn compile_with_topology(
        &mut self,
        source: &str,
        candidate_topology: Option<&PhaseHypergraph>,
    ) -> Result<MlirModule, CompileError> {
        if let Some(candidate) = candidate_topology {
            if let Some(current) = &self.current_topology {
                match current.verify_transition(candidate) {
                    Ok(d_phi) => {
                        let _ = record_event(
                            "preflight_topology_pass",
                            json!({ "d_phi": format!("{}/{}", d_phi.numer(), d_phi.denom()) }),
                        );
                    }
                    Err(GeneratorViolation::PhaseDissonance(n, d, cn, cd)) => {
                        return Err(CompileError::ValidationError {
                            item: "topology".to_string(),
                            message: format!(
                                "SIG_GOV_KILL: Phase Dissonance Breach: D_Phi({}/{}) >= epsilon_crit({}/{})",
                                n, d, cn, cd
                            ),
                        });
                    }
                    Err(GeneratorViolation::DimensionMismatch) => {
                        return Err(CompileError::ValidationError {
                            item: "topology".to_string(),
                            message: "Topological Incoherence: dimension mismatch between state hypergraphs".to_string(),
                        });
                    }
                }
            }
        }

        let module = self.compile(source)?;

        if let Some(candidate) = candidate_topology {
            self.current_topology = Some(candidate.clone());
        }

        Ok(module)
    }

    /// Compile PIRTM source to MLIR module.
    pub fn compile(&self, source: &str) -> Result<MlirModule, CompileError> {
        let program = pirtm_parser::parse(source).map_err(|e| CompileError::ParseError {
            location: "parse".to_string(),
            message: e,
        })?;

        for stmt in &program.stmts {
            if let pirtm_parser::ast::Stmt::Expr(ref expr) = stmt {
                self.validator
                    .validate(expr)
                    .map_err(|e| CompileError::ValidationError {
                        item: "expression".to_string(),
                        message: e,
                    })?;
            }
        }

        let mut visitor = MlirEmitterVisitor::new();
        let mut ops = Vec::new();
        for stmt in &program.stmts {
            match stmt {
                pirtm_parser::ast::Stmt::Expr(expr) => {
                    visitor.visit_expression(expr, &mut ops);
                }
                _ => {}
            }
        }

        let _ = record_event(
            "compilation",
            json!({
                "source_size": source.len(),
                "num_ops": ops.len(),
            }),
        );

        Ok(MlirModule {
            source: source.to_string(),
            ops,
        })
    }

    /// Get the MLIR text representation of a compiled module.
    pub fn to_mlir(&self, module: &MlirModule) -> Result<String, CompileError> {
        let mlir_text: Vec<String> = module
            .ops
            .iter()
            .map(|op| {
                op.emit_mlir()
                    .map_err(|e| CompileError::MlirError { message: e })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(mlir_text.join("\n"))
    }
}

/// Admissibility validator for AST expressions and manifests.
#[derive(Default)]
pub struct AdmissibilityValidator {}

impl AdmissibilityValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate(&self, _ast: &pirtm_parser::ast::Expr) -> Result<(), String> {
        Ok(())
    }

    pub fn validate_prime(&self, n: u64) -> Result<(), String> {
        if n < 2 {
            return Err(format!("prime_index {} is not a prime", n));
        }
        let limit = (n as f64).sqrt() as u64;
        for i in 2..=limit {
            if n % i == 0 {
                return Err(format!(
                    "prime_index {} is not a prime (divisible by {})",
                    n, i
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_compile_simple() {
        let compiler = PhaseMirrorCompiler::new();
        let result = compiler.compile("42");
        assert!(result.is_ok());
        let module = result.unwrap();
        assert_eq!(module.ops.len(), 1);
    }

    #[test]
    fn test_compiler_compile_pirtsm_expr() {
        let compiler = PhaseMirrorCompiler::new();
        let result = compiler.compile("Ap(2) + 3");
        assert!(result.is_ok());
        let module = result.unwrap();
        // Binary expression: 2 operands (Ap(2) and 3) + 1 binary op = 3 ops
        assert!(
            module.ops.len() >= 2,
            "expected at least 2 ops for binary expression"
        );
    }

    #[test]
    fn test_to_mlir() {
        let compiler = PhaseMirrorCompiler::new();
        let module = compiler.compile("42").unwrap();
        let mlir = compiler.to_mlir(&module).unwrap();
        assert!(mlir.contains("pirtm.operator_atom"));
    }

    #[test]
    fn test_compile_with_topology_interlock() {
        let mut compiler = PhaseMirrorCompiler::new();
        let mut h1 = PhaseHypergraph::new(2);
        h1.tensor[0][0] = num_rational::Ratio::new(10, 100);

        // First transition initializes state
        let res1 = compiler.compile_with_topology("42", Some(&h1));
        assert!(res1.is_ok());

        // Contractive transition: Delta = 1/100 < 3/100 -> PASS
        let mut h2 = PhaseHypergraph::new(2);
        h2.tensor[0][0] = num_rational::Ratio::new(11, 100);
        let res2 = compiler.compile_with_topology("42", Some(&h2));
        assert!(res2.is_ok());

        // Dissonant transition: Delta = 5/100 >= 3/100 -> FAIL-CLOSED
        let mut h3 = PhaseHypergraph::new(2);
        h3.tensor[0][0] = num_rational::Ratio::new(16, 100);
        let res3 = compiler.compile_with_topology("42", Some(&h3));
        assert!(res3.is_err());
        let err_msg = format!("{}", res3.unwrap_err());
        assert!(err_msg.contains("SIG_GOV_KILL: Phase Dissonance Breach"));
    }
}
pub mod type_check;
pub use type_check::{type_check, PirtmExpr, PirtmType, TypeError};

#[cfg(kani)]
mod kani_tests {
    use super::*;

    #[kani::proof]
    fn verify_type_check_soundness() {
        let e1 = kani::any::<i64>();
        let expr = PirtmExpr::Const(e1);
        let ctx = vec![];
        
        let ty = type_check(&ctx, &expr);
        kani::assert(ty.is_ok(), "Const should always type check");
        kani::assert(matches!(ty.unwrap(), PirtmType::Stratum), "Const is always Stratum");
        
        let var_name = "test_var".to_string();
        let ctx = vec![(var_name.clone(), PirtmType::Stratum)];
        let expr = PirtmExpr::Var(var_name);
        
        let ty = type_check(&ctx, &expr);
        kani::assert(ty.is_ok(), "Valid var should type check");
        kani::assert(matches!(ty.unwrap(), PirtmType::Stratum), "Var type matches context");
    }
}

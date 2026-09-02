use crate::manifest::EnsembleManifest;
use pirtm_engine::spectral::{self, Ensemble};
use std::collections::HashMap;

#[derive(Debug)]
pub enum LinkerError {
    DependencyNotFound(String),
    SpectralBudgetExceeded {
        ensemble: String,
        total: f64,
        limit: f64,
    },
    IncompatiblePrime(u64, u64),
    MissingTheoremAnchor {
        ensemble: String,
    },
    NormContractivityViolation {
        ensemble: String,
        norm_1: (u64, u64),
    },
}

/// The SpectralGovernor oversees the linking of multiple ensembles.
/// It constructs the interconnection matrix A and gains λ,
/// and certifies ||G||_1 < 1 in Q via validate_and_certify.
pub struct SpectralGovernor {
    registry: HashMap<String, EnsembleManifest>,
}

impl SpectralGovernor {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn register(&mut self, manifest: EnsembleManifest) {
        self.registry
            .insert(manifest.ensemble.name.clone(), manifest);
    }

    pub fn link(&self, root_name: &str) -> Result<(), LinkerError> {
        let root_manifest = self
            .registry
            .get(root_name)
            .ok_or_else(|| LinkerError::DependencyNotFound(root_name.to_string()))?;

        let mut nodes = Vec::new();
        let mut node_indices = HashMap::new();
        self.collect_dependencies(root_manifest, &mut nodes, &mut node_indices)?;

        let n = nodes.len();
        if n == 0 {
            return Ok(());
        }

        let mut adjacency = vec![vec![0.0; n]; n];
        let mut lambdas = vec![0.0; n];

        for (i, manifest) in nodes.iter().enumerate() {
            lambdas[i] = manifest.governance.spectral_radius;

            if let Some(deps) = &manifest.dependencies {
                for (dep_name, dep_meta) in deps {
                    let dep_manifest = self
                        .registry
                        .get(dep_name)
                        .ok_or_else(|| LinkerError::DependencyNotFound(dep_name.to_string()))?;

                    if let Some(expected_prime) = dep_meta.prime_index {
                        if expected_prime != dep_manifest.ensemble.prime_index {
                            return Err(LinkerError::IncompatiblePrime(
                                expected_prime,
                                dep_manifest.ensemble.prime_index,
                            ));
                        }
                    }

                    if let Some(&j) = node_indices.get(dep_name) {
                        adjacency[i][j] = dep_meta.spectral_max;
                    }
                }
            }
        }

        let ensemble = Ensemble::new(root_name, adjacency, lambdas)
            .with_theorem_name(root_manifest.governance.theorem_name.clone());
        match spectral::validate_and_certify(&ensemble, 0.0) {
            Ok(receipt) => {
                if !receipt.is_norm_contractive {
                    return Err(LinkerError::NormContractivityViolation {
                        ensemble: root_name.to_string(),
                        norm_1: receipt.exact_rational_norm_1,
                    });
                }
                Ok(())
            }
            Err(e) if e.contains("MissingTheoremAnchor") => {
                Err(LinkerError::MissingTheoremAnchor {
                    ensemble: root_name.to_string(),
                })
            }
            Err(e) if e.contains("NormContractivityViolation") => {
                Err(LinkerError::NormContractivityViolation {
                    ensemble: root_name.to_string(),
                    norm_1: (1, 1),
                })
            }
            Err(_) => Err(LinkerError::SpectralBudgetExceeded {
                ensemble: root_name.to_string(),
                total: 1.0,
                limit: 1.0,
            }),
        }
    }

    fn collect_dependencies<'a>(
        &'a self,
        manifest: &'a EnsembleManifest,
        nodes: &mut Vec<&'a EnsembleManifest>,
        node_indices: &mut HashMap<String, usize>,
    ) -> Result<(), LinkerError> {
        if node_indices.contains_key(&manifest.ensemble.name) {
            return Ok(());
        }

        let idx = nodes.len();
        node_indices.insert(manifest.ensemble.name.clone(), idx);
        nodes.push(manifest);

        if let Some(deps) = &manifest.dependencies {
            for (dep_name, _) in deps {
                let dep_manifest = self
                    .registry
                    .get(dep_name)
                    .ok_or_else(|| LinkerError::DependencyNotFound(dep_name.to_string()))?;
                self.collect_dependencies(dep_manifest, nodes, node_indices)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{DependencyMeta, EnsembleMeta, GovernanceMeta};

    fn gov(radius: f64, receipt: &str, theorem_name: &str) -> GovernanceMeta {
        GovernanceMeta {
            spectral_radius: radius,
            epsilon: None,
            contractivity_receipt: receipt.to_string(),
            ledger_anchor: None,
            theorem_name: theorem_name.to_string(),
        }
    }

    #[test]
    fn test_linker_composition_success() {
        let mut governor = SpectralGovernor::new();

        let mut deps = HashMap::new();
        deps.insert(
            "tensor-ops".to_string(),
            DependencyMeta {
                version: "0.1".to_string(),
                spectral_max: 0.3,
                prime_index: Some(17),
            },
        );

        let main = EnsembleManifest {
            ensemble: EnsembleMeta {
                name: "main-app".to_string(),
                version: "1.0".to_string(),
                prime_index: 2,
                description: None,
                authors: None,
            },
            governance: gov(0.5, "hash1", "author_declared_lambda"),
            dependencies: Some(deps),
        };

        let dep = EnsembleManifest {
            ensemble: EnsembleMeta {
                name: "tensor-ops".to_string(),
                version: "0.1".to_string(),
                prime_index: 17,
                description: None,
                authors: None,
            },
            governance: gov(0.2, "hash2", "author_declared_lambda"),
            dependencies: None,
        };

        governor.register(main);
        governor.register(dep);

        assert!(governor.link("main-app").is_ok());
    }

    #[test]
    fn test_linker_rejects_missing_theorem_name() {
        let mut governor = SpectralGovernor::new();

        let main = EnsembleManifest {
            ensemble: EnsembleMeta {
                name: "main-app".to_string(),
                version: "1.0".to_string(),
                prime_index: 2,
                description: None,
                authors: None,
            },
            governance: gov(0.5, "hash1", ""),
            dependencies: None,
        };
        governor.register(main);

        match governor.link("main-app") {
            Err(LinkerError::MissingTheoremAnchor { ensemble }) => {
                assert_eq!(ensemble, "main-app");
            }
            other => panic!("expected MissingTheoremAnchor, got {:?}", other),
        }
    }

    #[test]
    fn test_linker_composition_failure() {
        let mut governor = SpectralGovernor::new();

        let mut deps_a = HashMap::new();
        deps_a.insert(
            "node_b".to_string(),
            DependencyMeta {
                version: "1.0".to_string(),
                spectral_max: 1.5,
                prime_index: None,
            },
        );

        let mut deps_b = HashMap::new();
        deps_b.insert(
            "node_a".to_string(),
            DependencyMeta {
                version: "1.0".to_string(),
                spectral_max: 1.5,
                prime_index: None,
            },
        );

        let node_a = EnsembleManifest {
            ensemble: EnsembleMeta {
                name: "node_a".to_string(),
                version: "1.0".to_string(),
                prime_index: 2,
                description: None,
                authors: None,
            },
            governance: gov(0.9, "hash_a", "author_declared_lambda"),
            dependencies: Some(deps_a),
        };

        let node_b = EnsembleManifest {
            ensemble: EnsembleMeta {
                name: "node_b".to_string(),
                version: "1.0".to_string(),
                prime_index: 3,
                description: None,
                authors: None,
            },
            governance: gov(0.9, "hash_b", "author_declared_lambda"),
            dependencies: Some(deps_b),
        };

        governor.register(node_a);
        governor.register(node_b);

        assert!(governor.link("node_a").is_err());
    }
}

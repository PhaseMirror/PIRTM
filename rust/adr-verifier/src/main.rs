use adr_verifier::{
    append_certificate_to_ledger, check_extension, get_ledger_head_announcement, load_ledger,
    resolve_ledger_path, sync_ledger_with_peer, verify_ledger_chain, AdrRegistry,
    ExtensionCheckRequest, GossipSyncResult, KernelBoundaryConfig, LexicalAnchorTable,
    PhaseMirrorRegistry, GENESIS_PREV_HASH,
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for CNL subcommands
    if let Some(cmd) = args.get(1) {
        if cmd == "cnl" {
            handle_cnl_command(&args);
            return;
        }
    }

    let registry_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("docs/adr/registry.json");

    let contents = match fs::read_to_string(registry_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "[ERROR] Failed to read registry file at {}: {}",
                registry_path, e
            );
            process::exit(1);
        }
    };

    // Detect whether this is a PML runtime registry (v2.0) or a structural ADR registry.
    let is_pml = serde_json::from_str::<serde_json::Value>(&contents)
        .ok()
        .and_then(|v| {
            v.get("version")
                .and_then(|v| v.as_str())
                .map(|s| s.starts_with("2."))
        })
        .unwrap_or(false);

    if is_pml {
        run_boundary_guard(&contents, registry_path);
    } else {
        run_structural_guard(&contents, registry_path);
    }
}

fn handle_cnl_command(args: &[String]) {
    let subcmd = args.get(2).map(|s| s.as_str()).unwrap_or("");
    match subcmd {
        "check-extension" => {
            let token = match args.get(3) {
                Some(t) if !t.starts_with("--") => t.clone(),
                _ => {
                    eprintln!("[FATAL] Usage: adr-verifier cnl check-extension <token> [--prime <P>] [--class <CLASS>] [--proof-path <PATH>] [--canon-hash <HASH>]");
                    process::exit(1);
                }
            };

            let mut proposed_prime: Option<u64> = None;
            let mut proposed_class: Option<String> = None;
            let mut proof_path: Option<PathBuf> = None;
            let mut override_canon_hash: Option<String> = None;
            let mut custom_ledger_path: Option<PathBuf> = None;
            let mut no_ledger = false;

            let mut idx = 4;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--prime" => {
                        if let Some(val) = args.get(idx + 1) {
                            match val.parse::<u64>() {
                                Ok(p) => proposed_prime = Some(p),
                                Err(e) => {
                                    eprintln!("[FATAL] Invalid --prime '{}': {}", val, e);
                                    process::exit(1);
                                }
                            }
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --prime requires an argument");
                            process::exit(1);
                        }
                    }
                    "--class" => {
                        if let Some(val) = args.get(idx + 1) {
                            proposed_class = Some(val.clone());
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --class requires an argument");
                            process::exit(1);
                        }
                    }
                    "--proof-path" => {
                        if let Some(val) = args.get(idx + 1) {
                            proof_path = Some(PathBuf::from(val));
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --proof-path requires an argument");
                            process::exit(1);
                        }
                    }
                    "--canon-hash" => {
                        if let Some(val) = args.get(idx + 1) {
                            override_canon_hash = Some(val.clone());
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --canon-hash requires an argument");
                            process::exit(1);
                        }
                    }
                    "--ledger" => {
                        if let Some(val) = args.get(idx + 1) {
                            custom_ledger_path = Some(PathBuf::from(val));
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --ledger requires an argument");
                            process::exit(1);
                        }
                    }
                    "--no-ledger" => {
                        no_ledger = true;
                        idx += 1;
                    }
                    other => {
                        eprintln!("[FATAL] Unknown flag '{}'", other);
                        process::exit(1);
                    }
                }
            }

            let req = ExtensionCheckRequest {
                token,
                proposed_prime,
                proposed_class,
                proof_path,
                override_canon_hash,
            };

            match check_extension(&req) {
                Ok(cert) => {
                    println!(
                        "[OK] CNL Lexicon Extension Permitted: token='{}', prime={}, class={}",
                        cert.token, cert.prime_index, cert.class
                    );
                    println!("Certificate Hash: {}", cert.certificate_hash);
                    if let Ok(json_str) = serde_json::to_string_pretty(&cert) {
                        println!("{}", json_str);
                    }

                    if !no_ledger {
                        let ledger_path = resolve_ledger_path(custom_ledger_path.as_deref());
                        match append_certificate_to_ledger(&ledger_path, &cert, None) {
                            Ok(entry) => {
                                println!(
                                    "[LEDGER] Appended certificate to '{}' (seq={}, entry_hash={})",
                                    ledger_path.display(),
                                    entry.sequence,
                                    entry.entry_hash
                                );
                            }
                            Err(e) => {
                                eprintln!(
                                    "[FATAL] Failed to append certificate to ledger '{}': {}",
                                    ledger_path.display(),
                                    e
                                );
                                process::exit(1);
                            }
                        }
                    }

                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("[FATAL] CNL Lexicon Extension Refused: {}", e);
                    process::exit(1);
                }
            }
        }
        "list" => {
            let table = LexicalAnchorTable::canonical();
            println!(
                "[OK] Canonical CNL Lexicon ({} tokens):",
                adr_verifier::CANONICAL_TOKENS.len()
            );
            let mut entries = Vec::new();
            for &tok in &adr_verifier::CANONICAL_TOKENS {
                if let Some(entry) = table.lookup(tok) {
                    entries.push((tok, entry.prime, entry.class, entry.is_alias));
                }
            }
            entries.sort_by_key(|e| (e.1, e.0));
            for (tok, prime, class, is_alias) in entries {
                println!(
                    "  {:15} prime={:<4} class={:<10} alias={}",
                    tok,
                    prime,
                    format!("{:?}", class),
                    is_alias
                );
            }
            process::exit(0);
        }
        "verify-hash" => {
            let hash = match args.get(3) {
                Some(h) => h,
                None => {
                    eprintln!("[FATAL] Usage: adr-verifier cnl verify-hash <hash>");
                    process::exit(1);
                }
            };
            match LexicalAnchorTable::validate_lawful_recursion_hash(hash) {
                Ok(()) => {
                    println!("[OK] LawfulRecursionHash validated: {}", hash);
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("[FATAL] LawfulRecursionHash verification failed: {}", e);
                    process::exit(1);
                }
            }
        }
        "verify-ledger" => {
            let mut ledger_path: Option<PathBuf> = None;
            let mut idx = 3;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--ledger" => {
                        if let Some(val) = args.get(idx + 1) {
                            ledger_path = Some(PathBuf::from(val));
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --ledger requires an argument");
                            process::exit(1);
                        }
                    }
                    other => {
                        eprintln!("[FATAL] Unknown flag '{}'", other);
                        process::exit(1);
                    }
                }
            }

            let path = resolve_ledger_path(ledger_path.as_deref());
            match load_ledger(&path) {
                Ok(entries) => {
                    match verify_ledger_chain(&entries) {
                        Ok(()) => {
                            let head_hash = entries
                                .last()
                                .map(|e| e.entry_hash.as_str())
                                .unwrap_or(GENESIS_PREV_HASH);
                            println!(
                                "[OK] CNL Extension Ledger verified: {} entries, chain valid, head={}",
                                entries.len(),
                                head_hash
                            );
                            process::exit(0);
                        }
                        Err(e) => {
                            eprintln!(
                                "[FATAL] Ledger verification failed for '{}': {}",
                                path.display(),
                                e
                            );
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[FATAL] Failed to load ledger '{}': {}", path.display(), e);
                    process::exit(1);
                }
            }
        }
        "ledger-head" => {
            let mut ledger_path: Option<PathBuf> = None;
            let mut idx = 3;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--ledger" => {
                        if let Some(val) = args.get(idx + 1) {
                            ledger_path = Some(PathBuf::from(val));
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --ledger requires an argument");
                            process::exit(1);
                        }
                    }
                    other => {
                        eprintln!("[FATAL] Unknown flag '{}'", other);
                        process::exit(1);
                    }
                }
            }

            let path = resolve_ledger_path(ledger_path.as_deref());
            match load_ledger(&path) {
                Ok(entries) => {
                    if let Some(last) = entries.last() {
                        println!(
                            "[OK] Ledger Head: seq={}, hash={}, token='{}', prime={}",
                            last.sequence,
                            last.entry_hash,
                            last.certificate.token,
                            last.certificate.prime_index
                        );
                        if let Ok(json_str) = serde_json::to_string_pretty(last) {
                            println!("{}", json_str);
                        }
                    } else {
                        println!("[OK] Ledger at '{}' is empty", path.display());
                    }
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("[FATAL] Failed to read ledger '{}': {}", path.display(), e);
                    process::exit(1);
                }
            }
        }
        "gossip-head" => {
            let mut ledger_path: Option<PathBuf> = None;
            let mut idx = 3;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--ledger" => {
                        if let Some(val) = args.get(idx + 1) {
                            ledger_path = Some(PathBuf::from(val));
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --ledger requires an argument");
                            process::exit(1);
                        }
                    }
                    other => {
                        eprintln!("[FATAL] Unknown flag '{}'", other);
                        process::exit(1);
                    }
                }
            }

            let path = resolve_ledger_path(ledger_path.as_deref());
            match load_ledger(&path) {
                Ok(entries) => {
                    let announcement = get_ledger_head_announcement(&entries);
                    println!(
                        "[OK] Gossip Ledger Head: seq={}, hash={}, count={}, ts={}",
                        announcement.sequence,
                        announcement.head_hash,
                        announcement.entry_count,
                        announcement.timestamp_utc
                    );
                    if let Ok(json_str) = serde_json::to_string_pretty(&announcement) {
                        println!("{}", json_str);
                    }
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!(
                        "[FATAL] Failed to load ledger '{}': {}",
                        path.display(),
                        e
                    );
                    process::exit(1);
                }
            }
        }
        "sync-ledger" => {
            let peer_ledger_arg = match args.get(3) {
                Some(p) => p.clone(),
                None => {
                    eprintln!(
                        "[FATAL] Usage: adr-verifier cnl sync-ledger <PEER_LEDGER> [--ledger <LOCAL_LEDGER>]"
                    );
                    process::exit(1);
                }
            };
            let peer_ledger_path = PathBuf::from(peer_ledger_arg);

            let mut local_ledger_path: Option<PathBuf> = None;
            let mut idx = 4;
            while idx < args.len() {
                match args[idx].as_str() {
                    "--ledger" => {
                        if let Some(val) = args.get(idx + 1) {
                            local_ledger_path = Some(PathBuf::from(val));
                            idx += 2;
                        } else {
                            eprintln!("[FATAL] Flag --ledger requires an argument");
                            process::exit(1);
                        }
                    }
                    other => {
                        eprintln!("[FATAL] Unknown flag '{}'", other);
                        process::exit(1);
                    }
                }
            }

            let peer_entries = match load_ledger(&peer_ledger_path) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!(
                        "[FATAL] Failed to load peer ledger at '{}': {}",
                        peer_ledger_path.display(),
                        e
                    );
                    process::exit(1);
                }
            };

            let local_path = resolve_ledger_path(local_ledger_path.as_deref());
            match sync_ledger_with_peer(&local_path, &peer_entries, false) {
                Ok(result) => {
                    match result {
                        GossipSyncResult::InSync { sequence, head_hash } => {
                            println!(
                                "[OK] Gossip sync: local and peer are IN SYNC (seq={}, head={})",
                                sequence, head_hash
                            );
                        }
                        GossipSyncResult::PeerBehind {
                            local_sequence,
                            peer_sequence,
                            head_hash: _,
                        } => {
                            println!(
                                "[OK] Gossip sync: peer is BEHIND local (local_seq={}, peer_seq={}). No update applied.",
                                local_sequence, peer_sequence
                            );
                        }
                        GossipSyncResult::Advanced {
                            prior_sequence,
                            new_sequence,
                            prior_head_hash: _,
                            new_head_hash,
                            entries_appended,
                        } => {
                            println!(
                                "[OK] Gossip sync: local ledger ADVANCED from seq {} to {} (+{} entries, new_head={})",
                                prior_sequence, new_sequence, entries_appended, new_head_hash
                            );
                        }
                    }
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("[FATAL] Gossip synchronization failed: {}", e);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!(
                "[FATAL] Unknown CNL subcommand '{}'. Available: check-extension, list, verify-hash, verify-ledger, ledger-head, gossip-head, sync-ledger",
                subcmd
            );
            process::exit(1);
        }
    }
}

fn run_structural_guard(contents: &str, path: &str) {
    let registry = match AdrRegistry::from_json_str(contents) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[FATAL] ADR registry JSON parse failure: {}", e);
            process::exit(1);
        }
    };
    match registry.verify_invariants() {
        Ok(()) => {
            println!(
                "[OK] ADR Structural Registry Verified: {} records satisfy all invariants. ({})",
                registry.adrs.len(),
                path,
            );
        }
        Err(e) => {
            eprintln!("[FATAL] ADR Invariant Violation: {}", e);
            process::exit(1);
        }
    }
}

fn run_boundary_guard(contents: &str, path: &str) {
    let registry = match PhaseMirrorRegistry::from_json_str(contents) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[FATAL] PML registry JSON parse failure: {}", e);
            process::exit(1);
        }
    };
    let config = KernelBoundaryConfig::default();
    match registry.verify_boundary(&config) {
        Ok(()) => {
            println!(
                "[OK] Phase Mirror Boundary Guard: {} plan ADRs, sorry={}, axioms={}, drift={}, open_tensions={}, score={}. ({})",
                registry.plan_adrs.len(),
                registry.lean.sorry_total,
                registry.lean.axioms_postulates,
                registry.manifest.drift,
                registry.tensions.open,
                registry.tensions.total_score,
                path,
            );
        }
        Err(e) => {
            eprintln!("[FATAL] Kernel Boundary Violation: {}", e);
            process::exit(1);
        }
    }
}

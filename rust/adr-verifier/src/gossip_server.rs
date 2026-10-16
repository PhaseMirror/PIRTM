// Minimal zero-dependency HTTP server and client for CNL Extension Ledger gossip synchronization.
// Conforms to ADR-0139 and CNL extension ledger invariants.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::extension_ledger::{
    get_ledger_head_announcement, load_ledger, sync_ledger_with_peer, ExtensionLedgerEntry,
    GossipSyncError, GossipSyncResult, LedgerHeadAnnouncement,
};

/// Writes a standard HTTP/1.1 response with the given status, content-type, and body.
pub fn write_http_response<W: Write>(
    writer: &mut W,
    status_code: u16,
    status_text: &str,
    content_type: &str,
    body: &str,
) -> std::io::Result<()> {
    write!(
        writer,
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code,
        status_text,
        content_type,
        body.len(),
        body
    )?;
    writer.flush()
}

/// Parses raw HTTP response into status code and body string.
pub fn parse_http_response(raw: &str) -> Result<(u16, String), String> {
    let (headers, body) = raw
        .split_once("\r\n\r\n")
        .ok_or_else(|| "Malformed HTTP response: missing header separator".to_string())?;
    let first_line = headers
        .lines()
        .next()
        .ok_or_else(|| "Empty HTTP response".to_string())?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Malformed status line in response".to_string());
    }
    let code: u16 = parts[1]
        .parse()
        .map_err(|_| format!("Invalid status code '{}'", parts[1]))?;
    Ok((code, body.to_string()))
}

/// Handles a single incoming HTTP request on a connection.
pub fn handle_http_request<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    ledger_path: &Path,
) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(reader);
    let mut request_line = String::new();
    if buf_reader.read_line(&mut request_line)? == 0 {
        return Ok(());
    }

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return write_http_response(
            writer,
            400,
            "Bad Request",
            "application/json",
            r#"{"error":"malformed_request_line"}"#,
        );
    }
    let method = parts[0];
    let path = parts[1];

    // Read headers to extract Content-Length
    let mut content_length: usize = 0;
    loop {
        let mut header_line = String::new();
        if buf_reader.read_line(&mut header_line)? == 0 {
            break;
        }
        let trimmed = header_line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some((k, v)) = trimmed.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                if let Ok(len) = v.trim().parse::<usize>() {
                    content_length = len;
                }
            }
        }
    }

    // Read body if content-length specified
    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        buf_reader.read_exact(&mut body)?;
    }
    let body_str = String::from_utf8_lossy(&body);

    match (method, path) {
        ("GET", "/head") | ("GET", "/gossip/head") | ("GET", "/") => {
            match load_ledger(ledger_path) {
                Ok(entries) => {
                    let announcement = get_ledger_head_announcement(&entries);
                    let json_body = serde_json::to_string_pretty(&announcement)
                        .unwrap_or_else(|_| "{}".to_string());
                    write_http_response(writer, 200, "OK", "application/json", &json_body)
                }
                Err(e) => {
                    let err_body = serde_json::json!({
                        "error": "failed_to_load_ledger",
                        "details": e.to_string()
                    })
                    .to_string();
                    write_http_response(
                        writer,
                        500,
                        "Internal Server Error",
                        "application/json",
                        &err_body,
                    )
                }
            }
        }
        ("GET", "/ledger") => {
            match load_ledger(ledger_path) {
                Ok(entries) => {
                    let json_body = serde_json::to_string_pretty(&entries)
                        .unwrap_or_else(|_| "[]".to_string());
                    write_http_response(writer, 200, "OK", "application/json", &json_body)
                }
                Err(e) => {
                    let err_body = serde_json::json!({
                        "error": "failed_to_load_ledger",
                        "details": e.to_string()
                    })
                    .to_string();
                    write_http_response(
                        writer,
                        500,
                        "Internal Server Error",
                        "application/json",
                        &err_body,
                    )
                }
            }
        }
        ("POST", "/sync") | ("POST", "/gossip/sync") => {
            // Support both direct array `[...]` and wrapped `{"entries": [...]}`
            let peer_entries: Result<Vec<ExtensionLedgerEntry>, _> = serde_json::from_str(&body_str)
                .or_else(|_| {
                    #[derive(serde::Deserialize)]
                    struct SyncEnvelope {
                        entries: Vec<ExtensionLedgerEntry>,
                    }
                    serde_json::from_str::<SyncEnvelope>(&body_str).map(|env| env.entries)
                });

            let peer_entries = match peer_entries {
                Ok(entries) => entries,
                Err(e) => {
                    let err_body = serde_json::json!({
                        "status": "rejected",
                        "error": "invalid_payload",
                        "details": e.to_string()
                    })
                    .to_string();
                    return write_http_response(
                        writer,
                        400,
                        "Bad Request",
                        "application/json",
                        &err_body,
                    );
                }
            };

            match sync_ledger_with_peer(ledger_path, &peer_entries, false) {
                Ok(result) => {
                    let ok_body = serde_json::json!({
                        "status": "ok",
                        "result": result
                    })
                    .to_string();
                    write_http_response(writer, 200, "OK", "application/json", &ok_body)
                }
                Err(GossipSyncError::DivergentForkRejected {
                    fork_sequence,
                    local_hash,
                    peer_hash,
                }) => {
                    let err_body = serde_json::json!({
                        "status": "rejected",
                        "error": "divergent_fork",
                        "fork_sequence": fork_sequence,
                        "local_hash": local_hash,
                        "peer_hash": peer_hash
                    })
                    .to_string();
                    write_http_response(writer, 409, "Conflict", "application/json", &err_body)
                }
                Err(GossipSyncError::CompetingForkAtSameHeight {
                    sequence,
                    local_hash,
                    peer_hash,
                }) => {
                    let err_body = serde_json::json!({
                        "status": "rejected",
                        "error": "competing_fork",
                        "sequence": sequence,
                        "local_hash": local_hash,
                        "peer_hash": peer_hash
                    })
                    .to_string();
                    write_http_response(writer, 409, "Conflict", "application/json", &err_body)
                }
                Err(GossipSyncError::InvalidPeerChain(e)) => {
                    let err_body = serde_json::json!({
                        "status": "rejected",
                        "error": "invalid_peer_chain",
                        "details": e.to_string()
                    })
                    .to_string();
                    write_http_response(
                        writer,
                        400,
                        "Bad Request",
                        "application/json",
                        &err_body,
                    )
                }
                Err(GossipSyncError::IoError(e)) => {
                    let err_body = serde_json::json!({
                        "status": "error",
                        "error": "io_error",
                        "details": e
                    })
                    .to_string();
                    write_http_response(
                        writer,
                        500,
                        "Internal Server Error",
                        "application/json",
                        &err_body,
                    )
                }
            }
        }
        _ => {
            let not_found_body = serde_json::json!({
                "error": "not_found",
                "method": method,
                "path": path
            })
            .to_string();
            write_http_response(
                writer,
                404,
                "Not Found",
                "application/json",
                &not_found_body,
            )
        }
    }
}

/// Minimal TCP HTTP server for serving gossip announcements and ledger synchronization.
pub struct GossipHttpServer {
    listener: TcpListener,
    ledger_path: PathBuf,
}

impl GossipHttpServer {
    pub fn bind(addr: &str, ledger_path: PathBuf) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self {
            listener,
            ledger_path,
        })
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    /// Serves a single incoming connection (useful for deterministic tests).
    pub fn serve_one(&self) -> std::io::Result<()> {
        let (stream, _) = self.listener.accept()?;
        let mut reader = stream.try_clone()?;
        let mut writer = stream;
        handle_http_request(&mut reader, &mut writer, &self.ledger_path)
    }

    /// Serves incoming requests until the `stop_flag` is set to true.
    pub fn serve_with_stop_flag(&self, stop_flag: Arc<AtomicBool>) -> std::io::Result<()> {
        self.listener.set_nonblocking(true)?;
        while !stop_flag.load(Ordering::Relaxed) {
            match self.listener.accept() {
                Ok((stream, _)) => {
                    let mut reader = match stream.try_clone() {
                        Ok(r) => r,
                        Err(_) => continue,
                    };
                    let mut writer = stream;
                    let _ = handle_http_request(&mut reader, &mut writer, &self.ledger_path);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// Helper client function to query `/head` over HTTP.
pub fn http_get_head(addr: &str) -> Result<LedgerHeadAnnouncement, String> {
    let clean_addr = addr
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let mut stream =
        TcpStream::connect(clean_addr).map_err(|e| format!("Connect error to '{}': {}", clean_addr, e))?;
    let req = format!(
        "GET /head HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        clean_addr
    );
    stream
        .write_all(req.as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("Read error: {}", e))?;

    let (status_code, body) = parse_http_response(&response)?;
    if status_code != 200 {
        return Err(format!("HTTP {}: {}", status_code, body));
    }
    serde_json::from_str(&body).map_err(|e| format!("JSON parse error: {}", e))
}

/// Helper client function to send a peer ledger to `/sync` over HTTP.
pub fn http_post_sync(
    addr: &str,
    entries: &[ExtensionLedgerEntry],
) -> Result<GossipSyncResult, String> {
    let clean_addr = addr
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let mut stream =
        TcpStream::connect(clean_addr).map_err(|e| format!("Connect error to '{}': {}", clean_addr, e))?;
    let body = serde_json::to_string(entries)
        .map_err(|e| format!("Serialization error: {}", e))?;
    let req = format!(
        "POST /sync HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        clean_addr,
        body.len(),
        body
    );
    stream
        .write_all(req.as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("Read error: {}", e))?;

    let (status_code, body) = parse_http_response(&response)?;
    if status_code == 200 {
        let parsed: serde_json::Value =
            serde_json::from_str(&body).map_err(|e| format!("JSON error: {}", e))?;
        if let Some(res) = parsed.get("result") {
            serde_json::from_value(res.clone())
                .map_err(|e| format!("Result parse error: {}", e))
        } else {
            serde_json::from_str(&body).map_err(|e| format!("Result parse error: {}", e))
        }
    } else {
        Err(format!("HTTP {}: {}", status_code, body))
    }
}

/// Helper client function to fetch the complete ledger from `/ledger` over HTTP.
pub fn http_get_ledger(addr: &str) -> Result<Vec<ExtensionLedgerEntry>, String> {
    let clean_addr = addr
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    let mut stream =
        TcpStream::connect(clean_addr).map_err(|e| format!("Connect error to '{}': {}", clean_addr, e))?;
    let req = format!(
        "GET /ledger HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        clean_addr
    );
    stream
        .write_all(req.as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("Read error: {}", e))?;

    let (status_code, body) = parse_http_response(&response)?;
    if status_code != 200 {
        return Err(format!("HTTP {}: {}", status_code, body));
    }
    serde_json::from_str(&body).map_err(|e| format!("JSON parse error: {}", e))
}

/// Report summarizing synchronization against a specific remote peer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PeerSyncReport {
    pub peer: String,
    pub status: String,
    pub peer_sequence: Option<u64>,
    pub local_sequence: u64,
    pub message: String,
}

/// Queries a remote peer's `/head` and syncs forward extensions if peer sequence > local sequence.
/// Rejects divergent forks fail-closed without modifying local ledger.
pub fn poll_and_sync_peer(ledger_path: &Path, peer_addr: &str) -> PeerSyncReport {
    let local_entries = match load_ledger(ledger_path) {
        Ok(e) => e,
        Err(e) => {
            return PeerSyncReport {
                peer: peer_addr.to_string(),
                status: "local_error".to_string(),
                peer_sequence: None,
                local_sequence: 0,
                message: format!("Failed to load local ledger: {}", e),
            };
        }
    };
    let local_ann = get_ledger_head_announcement(&local_entries);

    let peer_ann = match http_get_head(peer_addr) {
        Ok(ann) => ann,
        Err(e) => {
            return PeerSyncReport {
                peer: peer_addr.to_string(),
                status: "unreachable".to_string(),
                peer_sequence: None,
                local_sequence: local_ann.sequence,
                message: format!("Peer head unreachable: {}", e),
            };
        }
    };

    if peer_ann.sequence == local_ann.sequence {
        if peer_ann.head_hash == local_ann.head_hash {
            return PeerSyncReport {
                peer: peer_addr.to_string(),
                status: "in_sync".to_string(),
                peer_sequence: Some(peer_ann.sequence),
                local_sequence: local_ann.sequence,
                message: format!("Chains in sync at sequence {}", local_ann.sequence),
            };
        } else {
            return PeerSyncReport {
                peer: peer_addr.to_string(),
                status: "competing_fork_rejected".to_string(),
                peer_sequence: Some(peer_ann.sequence),
                local_sequence: local_ann.sequence,
                message: format!(
                    "Competing fork at same sequence {}: local '{}' != peer '{}'",
                    local_ann.sequence, local_ann.head_hash, peer_ann.head_hash
                ),
            };
        }
    }

    if peer_ann.sequence > local_ann.sequence {
        // Fetch peer ledger and sync forward
        let peer_entries = match http_get_ledger(peer_addr) {
            Ok(entries) => entries,
            Err(e) => {
                return PeerSyncReport {
                    peer: peer_addr.to_string(),
                    status: "fetch_error".to_string(),
                    peer_sequence: Some(peer_ann.sequence),
                    local_sequence: local_ann.sequence,
                    message: format!("Failed to fetch peer ledger: {}", e),
                };
            }
        };

        match sync_ledger_with_peer(ledger_path, &peer_entries, false) {
            Ok(GossipSyncResult::Advanced {
                prior_sequence,
                new_sequence,
                entries_appended,
                new_head_hash,
                ..
            }) => PeerSyncReport {
                peer: peer_addr.to_string(),
                status: "advanced".to_string(),
                peer_sequence: Some(peer_ann.sequence),
                local_sequence: new_sequence,
                message: format!(
                    "Local ledger advanced from seq {} to {} (+{} entries, head={})",
                    prior_sequence, new_sequence, entries_appended, new_head_hash
                ),
            },
            Ok(other) => PeerSyncReport {
                peer: peer_addr.to_string(),
                status: "synced".to_string(),
                peer_sequence: Some(peer_ann.sequence),
                local_sequence: local_ann.sequence,
                message: format!("{:?}", other),
            },
            Err(e) => PeerSyncReport {
                peer: peer_addr.to_string(),
                status: "fork_rejected".to_string(),
                peer_sequence: Some(peer_ann.sequence),
                local_sequence: local_ann.sequence,
                message: format!("Sync rejected: {}", e),
            },
        }
    } else {
        // Peer is behind local
        PeerSyncReport {
            peer: peer_addr.to_string(),
            status: "peer_behind".to_string(),
            peer_sequence: Some(peer_ann.sequence),
            local_sequence: local_ann.sequence,
            message: format!(
                "Local is ahead (local_seq={}, peer_seq={})",
                local_ann.sequence, peer_ann.sequence
            ),
        }
    }
}

/// Polls and syncs against a list of configured peers.
pub fn poll_and_sync_all_peers(ledger_path: &Path, peers: &[String]) -> Vec<PeerSyncReport> {
    peers
        .iter()
        .map(|peer| poll_and_sync_peer(ledger_path, peer))
        .collect()
}

/// Background gossip synchronization daemon that periodically queries peers and syncs forward extensions.
pub struct GossipSyncDaemon {
    pub ledger_path: PathBuf,
    pub peers: Vec<String>,
    pub poll_interval_ms: u64,
}

impl GossipSyncDaemon {
    pub fn new(ledger_path: PathBuf, peers: Vec<String>, poll_interval_ms: u64) -> Self {
        Self {
            ledger_path,
            peers,
            poll_interval_ms,
        }
    }

    /// Performs a single poll-and-sync pass across all configured peers.
    pub fn run_once(&self) -> Vec<PeerSyncReport> {
        poll_and_sync_all_peers(&self.ledger_path, &self.peers)
    }

    /// Runs periodic synchronization loop until `stop_flag` is set.
    pub fn run_loop_with_stop_flag(&self, stop_flag: Arc<AtomicBool>) {
        while !stop_flag.load(Ordering::Relaxed) {
            let _ = self.run_once();
            let steps = (self.poll_interval_ms / 50).max(1);
            for _ in 0..steps {
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }

    /// Spawns the daemon in a background OS thread.
    pub fn spawn_in_background(self, stop_flag: Arc<AtomicBool>) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            self.run_loop_with_stop_flag(stop_flag);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::extension_checker::{check_extension, ExtensionCertificate, ExtensionCheckRequest};
    use crate::extension_ledger::append_certificate_to_ledger;

    struct TempLedgerFile {
        path: PathBuf,
    }

    impl TempLedgerFile {
        fn new() -> Self {
            let mut p = std::env::temp_dir();
            p.push(format!(
                "test_gossip_http_{}_{}.jsonl",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            Self { path: p }
        }
    }

    impl Drop for TempLedgerFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    fn make_test_cert(token: &str, prime: u64, class: &str) -> ExtensionCertificate {
        let req = ExtensionCheckRequest {
            token: token.to_string(),
            proposed_prime: Some(prime),
            proposed_class: Some(class.to_string()),
            proof_path: None,
            override_canon_hash: None,
        };
        check_extension(&req).expect("Valid test certificate")
    }

    #[test]
    fn test_http_endpoint_serves_head() {
        let tmp = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2 = make_test_cert("scale", 3, "verb");
        append_certificate_to_ledger(&tmp.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        let entry2 = append_certificate_to_ledger(&tmp.path, &cert2, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        let server = GossipHttpServer::bind("127.0.0.1:0", tmp.path.clone()).unwrap();
        let addr = server.local_addr().unwrap();

        let stop_flag = Arc::new(AtomicBool::new(false));
        let flag_clone = stop_flag.clone();
        let handle = std::thread::spawn(move || {
            server.serve_with_stop_flag(flag_clone)
        });

        let ann = http_get_head(&format!("127.0.0.1:{}", addr.port())).expect("HTTP GET /head succeeds");
        assert_eq!(ann.sequence, 2);
        assert_eq!(ann.head_hash, entry2.entry_hash);
        assert_eq!(ann.entry_count, 2);

        stop_flag.store(true, Ordering::Relaxed);
        let _ = handle.join();
    }

    #[test]
    fn test_http_endpoint_sync_advances_and_rejects_divergence() {
        let tmp_local = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2 = make_test_cert("scale", 3, "verb");

        // Local has cert1
        append_certificate_to_ledger(&tmp_local.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();

        let server = GossipHttpServer::bind("127.0.0.1:0", tmp_local.path.clone()).unwrap();
        let addr = server.local_addr().unwrap();

        let stop_flag = Arc::new(AtomicBool::new(false));
        let flag_clone = stop_flag.clone();
        let handle = std::thread::spawn(move || {
            server.serve_with_stop_flag(flag_clone)
        });

        let peer_addr = format!("127.0.0.1:{}", addr.port());

        // 1. Peer sends valid forward extension: cert1 + cert2
        let tmp_peer = TempLedgerFile::new();
        append_certificate_to_ledger(&tmp_peer.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_peer.path, &cert2, Some("2026-10-16T00:01:00Z".to_string())).unwrap();
        let peer_entries = load_ledger(&tmp_peer.path).unwrap();

        let res = http_post_sync(&peer_addr, &peer_entries).expect("HTTP POST /sync succeeds");
        match res {
            GossipSyncResult::Advanced { prior_sequence, new_sequence, entries_appended, .. } => {
                assert_eq!(prior_sequence, 1);
                assert_eq!(new_sequence, 2);
                assert_eq!(entries_appended, 1);
            }
            _ => panic!("Expected Advanced result"),
        }

        // 2. Peer sends divergent fork at sequence 2 (e.g. menthol with prime 53)
        let tmp_divergent = TempLedgerFile::new();
        let cert2_divergent = make_test_cert("menthol", 53, "sector");
        append_certificate_to_ledger(&tmp_divergent.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_divergent.path, &cert2_divergent, Some("2026-10-16T00:01:00Z".to_string())).unwrap();
        let divergent_entries = load_ledger(&tmp_divergent.path).unwrap();

        let err_res = http_post_sync(&peer_addr, &divergent_entries);
        assert!(err_res.is_err(), "Divergent fork must be rejected by HTTP endpoint");
        let err_msg = err_res.unwrap_err();
        assert!(err_msg.contains("409"), "Must return 409: {}", err_msg);
        assert!(err_msg.contains("competing_fork") || err_msg.contains("divergent_fork"));

        // Verify local ledger was NOT modified by divergent sync
        let local_check = load_ledger(&tmp_local.path).unwrap();
        assert_eq!(local_check.len(), 2);
        assert_eq!(local_check[1].certificate.token, "scale");

        stop_flag.store(true, Ordering::Relaxed);
        let _ = handle.join();
    }

    #[test]
    fn test_http_endpoint_not_found() {
        let tmp = TempLedgerFile::new();
        let server = GossipHttpServer::bind("127.0.0.1:0", tmp.path.clone()).unwrap();
        let addr = server.local_addr().unwrap();

        let stop_flag = Arc::new(AtomicBool::new(false));
        let flag_clone = stop_flag.clone();
        let handle = std::thread::spawn(move || {
            server.serve_with_stop_flag(flag_clone)
        });

        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write_all(b"GET /unknown HTTP/1.1\r\nConnection: close\r\n\r\n").unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).unwrap();
        let (status, _) = parse_http_response(&resp).unwrap();
        assert_eq!(status, 404);

        stop_flag.store(true, Ordering::Relaxed);
        let _ = handle.join();
    }

    #[test]
    fn test_daemon_poll_and_sync_advances_on_higher_peer() {
        let tmp_a = TempLedgerFile::new();
        let tmp_b = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2 = make_test_cert("scale", 3, "verb");

        // Node A has only cert1 (seq 1)
        append_certificate_to_ledger(&tmp_a.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();

        // Node B has cert1 + cert2 (seq 2)
        append_certificate_to_ledger(&tmp_b.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_b.path, &cert2, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        // Server for Node B
        let server_b = GossipHttpServer::bind("127.0.0.1:0", tmp_b.path.clone()).unwrap();
        let addr_b = server_b.local_addr().unwrap();
        let stop_server = Arc::new(AtomicBool::new(false));
        let stop_clone = stop_server.clone();
        let handle_b = std::thread::spawn(move || {
            server_b.serve_with_stop_flag(stop_clone)
        });

        let peer_b_url = format!("127.0.0.1:{}", addr_b.port());

        // Node A polls Node B
        let report = poll_and_sync_peer(&tmp_a.path, &peer_b_url);
        assert_eq!(report.status, "advanced");
        assert_eq!(report.local_sequence, 2);

        // Verify Node A's ledger on disk now contains both entries
        let entries_a = load_ledger(&tmp_a.path).unwrap();
        assert_eq!(entries_a.len(), 2);
        assert_eq!(entries_a[1].certificate.token, "scale");

        stop_server.store(true, Ordering::Relaxed);
        let _ = handle_b.join();
    }

    #[test]
    fn test_daemon_poll_and_sync_rejects_divergent_peer() {
        let tmp_a = TempLedgerFile::new();
        let tmp_c = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2_a = make_test_cert("scale", 3, "verb");
        let cert2_c = make_test_cert("menthol", 53, "sector");

        // Node A has cert1 + cert2_a
        append_certificate_to_ledger(&tmp_a.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_a.path, &cert2_a, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        // Node C has cert1 + divergent cert2_c
        append_certificate_to_ledger(&tmp_c.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_c.path, &cert2_c, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        let server_c = GossipHttpServer::bind("127.0.0.1:0", tmp_c.path.clone()).unwrap();
        let addr_c = server_c.local_addr().unwrap();
        let stop_server = Arc::new(AtomicBool::new(false));
        let stop_clone = stop_server.clone();
        let handle_c = std::thread::spawn(move || {
            server_c.serve_with_stop_flag(stop_clone)
        });

        let peer_c_url = format!("127.0.0.1:{}", addr_c.port());

        // Node A polls Node C (competing fork at same height)
        let report = poll_and_sync_peer(&tmp_a.path, &peer_c_url);
        assert_eq!(report.status, "competing_fork_rejected");

        // Verify Node A's ledger was NOT modified
        let entries_a = load_ledger(&tmp_a.path).unwrap();
        assert_eq!(entries_a.len(), 2);
        assert_eq!(entries_a[1].certificate.token, "scale");

        stop_server.store(true, Ordering::Relaxed);
        let _ = handle_c.join();
    }

    #[test]
    fn test_daemon_background_thread_spawns_and_syncs() {
        let tmp_a = TempLedgerFile::new();
        let tmp_b = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2 = make_test_cert("scale", 3, "verb");

        // Node A has cert1
        append_certificate_to_ledger(&tmp_a.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();

        // Node B has cert1 + cert2
        append_certificate_to_ledger(&tmp_b.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_b.path, &cert2, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        let server_b = GossipHttpServer::bind("127.0.0.1:0", tmp_b.path.clone()).unwrap();
        let addr_b = server_b.local_addr().unwrap();
        let stop_server = Arc::new(AtomicBool::new(false));
        let stop_clone = stop_server.clone();
        let handle_b = std::thread::spawn(move || {
            server_b.serve_with_stop_flag(stop_clone)
        });

        let peer_b_url = format!("127.0.0.1:{}", addr_b.port());

        // Spawn background sync daemon for Node A with 30ms poll interval
        let daemon = GossipSyncDaemon::new(tmp_a.path.clone(), vec![peer_b_url], 30);
        let stop_daemon = Arc::new(AtomicBool::new(false));
        let daemon_handle = daemon.spawn_in_background(stop_daemon.clone());

        // Wait up to 500ms for background thread to sync
        let mut synced = false;
        for _ in 0..20 {
            std::thread::sleep(std::time::Duration::from_millis(25));
            if let Ok(entries) = load_ledger(&tmp_a.path) {
                if entries.len() == 2 {
                    synced = true;
                    break;
                }
            }
        }
        assert!(synced, "Background daemon should have synced entries to Node A");

        stop_daemon.store(true, Ordering::Relaxed);
        let _ = daemon_handle.join();

        stop_server.store(true, Ordering::Relaxed);
        let _ = handle_b.join();
    }
}

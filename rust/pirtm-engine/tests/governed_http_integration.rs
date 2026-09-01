use pirtm_engine::http_server::GovernedHttpServer;
use pirtm_engine::spectral::Ensemble;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_governed_http_server_end_to_end() {
    let ensemble = Ensemble {
        name: "governed_http_ensemble".to_string(),
        adjacency: vec![vec![0.0, 0.4], vec![0.4, 0.0]],
        lambdas: vec![0.9, 0.9],
    };

    let server = GovernedHttpServer::new(19999, ensemble);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let server_handle = thread::spawn(move || {
        server.listen(running_clone).expect("GovernedHttpServer listen failed");
    });

    thread::sleep(Duration::from_millis(100));

    let mut stream = TcpStream::connect("127.0.0.1:19999").expect("Client failed to connect to server");
    stream.write_all(b"GET /governed-data HTTP/1.1\r\nHost: localhost\r\n\r\n").expect("Client write failed");
    stream.shutdown(std::net::Shutdown::Write).expect("Shutdown write failed");

    let mut response_bytes = Vec::new();
    stream.read_to_end(&mut response_bytes).expect("Client read failed");
    let response_str = String::from_utf8_lossy(&response_bytes);

    assert!(response_str.contains("HTTP/1.1 200 OK"));
    assert!(response_str.contains("qmhes_tag"));
    assert!(response_str.contains("goldilocks_proof_receipt"));

    running.store(false, Ordering::SeqCst);
    let _ = server_handle.join();
}

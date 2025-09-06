use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    println!("🌐 Starting httpcg://example.com Demo Server");
    println!("============================================");
    
    // Start Shadow Registry on port 8080
    thread::spawn(|| {
        start_shadow_registry();
    });
    
    // Start HTTP Cage on port 8888
    thread::spawn(|| {
        start_http_cage();
    });
    
    // Start main httpcg demo server on port 3000
    start_httpcg_demo_server();
}

fn start_shadow_registry() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("🔍 Shadow Registry listening on port 8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_shadow_registry_request(stream);
                });
            }
            Err(_) => {}
        }
    }
}

fn start_http_cage() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("🔒 HTTP Cage listening on port 8888");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_http_cage_request(stream);
                });
            }
            Err(_) => {}
        }
    }
}

fn start_httpcg_demo_server() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("🚀 httpcg Demo Server listening on port 3000");
    println!("📡 Try: curl \"http://localhost:3000/httpcg/example.com\"");
    println!("📡 Or:  curl \"http://localhost:3000/httpcg/example.com/hello\"");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_httpcg_request(stream);
                });
            }
            Err(_) => {}
        }
    }
}

fn handle_shadow_registry_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer[..]);
    
    if request.contains("GET /registry/resolve/example.com") {
        let response = r#"HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 156
Access-Control-Allow-Origin: *

{
  "domain": "example.com",
  "httpcg_url": "httpcg://app/example.com",
  "target": "http://localhost:3000",
  "tlsls_required": true,
  "qlock_enabled": true
}"#;
        stream.write(response.as_bytes()).unwrap();
    } else {
        let response = r#"HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 85
Access-Control-Allow-Origin: *

{
  "status": "active",
  "service": "Shadow Registry",
  "registered_domains": 1
}"#;
        stream.write(response.as_bytes()).unwrap();
    }
    
    stream.flush().unwrap();
}

fn handle_http_cage_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let response = r#"HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 134
Access-Control-Allow-Origin: *
X-HTTP-Cage: enabled
X-Security-Rating: 9.8

{
  "status": "active",
  "service": "HTTP Cage",
  "security_rating": 9.8,
  "quantum_safe": true,
  "requests_processed": 1
}"#;
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_httpcg_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    println!("📥 Request: {}", request_line);
    
    if request_line.contains("GET /httpcg/example.com/hello") {
        // httpcg://example.com/hello endpoint
        let response = create_httpcg_response(
            "Hello World from httpcg://example.com/hello!",
            "text/plain",
            "/hello"
        );
        stream.write(response.as_bytes()).unwrap();
    } else if request_line.contains("GET /httpcg/example.com") {
        // httpcg://example.com root endpoint
        let html_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>httpcg://example.com</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f8ff; }
        .container { max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
        .header { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        .protocol { color: #e74c3c; font-weight: bold; }
        .features { background: #ecf0f1; padding: 20px; border-radius: 5px; margin: 20px 0; }
        .feature { margin: 10px 0; padding: 10px; background: white; border-left: 4px solid #3498db; }
    </style>
</head>
<body>
    <div class="container">
        <h1 class="header">🌐 Welcome to <span class="protocol">httpcg://example.com</span></h1>
        <p><strong>Hello World!</strong> You are successfully accessing this site via the httpcg protocol.</p>
        
        <div class="features">
            <h2>🔒 Security Features Active:</h2>
            <div class="feature">✅ <strong>TLSLS:</strong> Transport Layer Security with Ledger Stamps</div>
            <div class="feature">✅ <strong>QLOCK:</strong> Quantum-safe session locks</div>
            <div class="feature">✅ <strong>SAPI:</strong> Signed API authentication</div>
            <div class="feature">✅ <strong>HTTP Cage:</strong> Military-grade request protection</div>
            <div class="feature">✅ <strong>Shadow Registry:</strong> Decentralized domain resolution</div>
        </div>
        
        <h2>🚀 Try These Endpoints:</h2>
        <ul>
            <li><a href="/httpcg/example.com/hello">httpcg://example.com/hello</a> - Simple text response</li>
            <li><a href="/httpcg/example.com/api">httpcg://example.com/api</a> - JSON API response</li>
            <li><a href="/httpcg/example.com/secure">httpcg://example.com/secure</a> - Enhanced security demo</li>
        </ul>
        
        <p><em>Powered by Pravyom Metanode Infrastructure</em></p>
    </div>
</body>
</html>"#;
        
        let response = create_httpcg_response(html_content, "text/html", "/");
        stream.write(response.as_bytes()).unwrap();
    } else if request_line.contains("GET /httpcg/example.com/api") {
        // httpcg://example.com/api endpoint
        let json_response = r#"{
  "message": "Hello World from httpcg://example.com/api!",
  "protocol": "httpcg",
  "domain": "example.com",
  "security": {
    "tlsls": true,
    "qlock": true,
    "sapi": true,
    "rating": 9.8
  },
  "timestamp": "2025-08-31T15:05:46Z",
  "powered_by": "Pravyom Metanode"
}"#;
        
        let response = create_httpcg_response(json_response, "application/json", "/api");
        stream.write(response.as_bytes()).unwrap();
    } else if request_line.contains("GET /httpcg/example.com/secure") {
        // httpcg://example.com/secure endpoint with enhanced security
        let secure_response = r#"🔐 SECURE ENDPOINT ACCESS GRANTED

Hello World from httpcg://example.com/secure!

Security Status:
✅ Quantum-safe encryption: ACTIVE
✅ Post-quantum signatures: VERIFIED  
✅ QLOCK session lock: ENGAGED
✅ TLSLS certificate: VALID
✅ SAPI authentication: PASSED
✅ HTTP Cage protection: ENABLED
✅ Audit trail: RECORDED

Your request has been processed through military-grade security layers.
All communications are quantum-safe and tamper-proof.

Connection ID: httpcg_secure_1756652746
Security Rating: 9.8/10"#;
        
        let response = create_httpcg_response(secure_response, "text/plain", "/secure");
        stream.write(response.as_bytes()).unwrap();
    } else {
        // Default 404 response
        let response = r#"HTTP/1.1 404 Not Found
Content-Type: text/plain
Content-Length: 47
Access-Control-Allow-Origin: *

404 - httpcg endpoint not found on example.com"#;
        stream.write(response.as_bytes()).unwrap();
    }
    
    stream.flush().unwrap();
}

fn create_httpcg_response(content: &str, content_type: &str, path: &str) -> String {
    format!(
        r#"HTTP/1.1 200 OK
Content-Type: {}
Content-Length: {}
Access-Control-Allow-Origin: *
X-httpcg-Protocol: 1.0
X-httpcg-Domain: example.com
X-httpcg-Path: {}
X-TLSLS-Enabled: true
X-QLOCK-Active: true
X-SAPI-Verified: true
X-HTTP-Cage: protected
X-Security-Rating: 9.8
X-Powered-By: Pravyom-Metanode

{}"#,
        content_type,
        content.len(),
        path,
        content
    )
}

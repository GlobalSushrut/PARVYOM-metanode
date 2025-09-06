# Starter Repository Guide (Web 3.5)

A minimal, reproducible scaffold to go from init → up → request → verify.

---

## Repository Layout
```
bpi-starter/
├─ bpicompose.yml
├─ agreements/
│  ├─ basic.v1.yaml
│  └─ plugins/
│     └─ rate_limiter.rs
├─ services/
│  └─ hello-world/
│     ├─ Dockerfile
│     ├─ main.py
│     └─ requirements.txt
├─ scripts/
│  ├─ setup.sh
│  └─ deploy.sh
└─ docs/
   ├─ getting-started.md
   └─ architecture.md
```

---

## bpicompose.yml (example)
```yaml
version: "1.0"
project:
  id: "hello-bpi"
  region: "us-west-2"

mainnet:
  validators: "auto-3"
  da: { k: 8, n: 12, sizeKB: 256 }
  anchors: ["sepolia", "polygon-mumbai"]
  lanes: ["http"]
  receipts: "headersOnly"

agreement:
  id: "basic-v1"
  yaml: "agreements/basic.v1.yaml"
  defaults:
    max_requests_per_minute: 1000
    max_payload_size: "1MB"

services:
  hello-world:
    image: "bpi-starter/hello-world:latest"
    ports: ["8080:8080"]
    env:
      FLASK_ENV: "production"
      LOG_LEVEL: "info"
    agreement: "basic-v1"
    receipts: true
```

---

## Agreement (agreements/basic.v1.yaml)
```yaml
version: "1.0"
name: "basic-policy"
description: "Rate limiting + basic input/output constraints"

rules:
  pre_execution:
    - name: rate_limit
      type: rate_limiter
      config: { requests_per_minute: 1000, burst_size: 100 }
    - name: payload_size
      type: size_limit
      config: { max_bytes: 1048576 }
  post_execution:
    - name: response_filter
      type: content_filter
      config: { redact_patterns: ["password", "secret", "token"] }

validation:
  deterministic: true
  timeout_ms: 5000
  max_memory_mb: 128
```

---

## Rust Plugin Skeleton (agreements/plugins/rate_limiter.rs)
```rust
// Build to WASM; export pre_execution/post_execution symbols
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Config { requests_per_minute: u32, burst_size: u32 }
#[derive(Serialize)]
struct Decision { allow: bool, reason: Option<String> }

#[no_mangle]
pub extern "C" fn pre_execution(cfg_ptr: *const u8, cfg_len: usize) -> *const u8 {
    let _cfg_bytes = unsafe { std::slice::from_raw_parts(cfg_ptr, cfg_len) };
    // TODO: parse, check counters, return decision
    let out = serde_json::to_vec(&Decision { allow: true, reason: None }).unwrap();
    Box::into_raw(out.into_boxed_slice()) as *const u8
}

#[no_mangle]
pub extern "C" fn post_execution(_: *const u8, _: usize) -> *const u8 {
    let out = serde_json::to_vec(&Decision { allow: true, reason: None }).unwrap();
    Box::into_raw(out.into_boxed_slice()) as *const u8
}
```

---

## Service (services/hello-world/main.py)
```python
from flask import Flask, request, jsonify
import time, os
app = Flask(__name__)

@app.get('/')
def hello():
    return jsonify({"message": "Hello from BPI Web 3.5!", "ts": int(time.time())})

@app.post('/submit')
def submit():
    return jsonify({"status": "processed", "ts": int(time.time())})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=int(os.getenv('PORT', 8080)))
```

### Dockerfile
```Dockerfile
FROM python:3.11-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY . .
EXPOSE 8080
CMD ["python", "main.py"]
```

### requirements.txt
```
flask==3.0.2
```

---

## CLI Golden Path (local devnet)
- bpi init
- bpi up
- bpi deploy hello-world
- curl http://localhost:8080/
- bpi verify
- bpi receipts get <id>

Exit codes: 0 ok; 10 policy deny; 20 consensus fail; 30 DA fail; 40 anchor fail.

---

## Notes
- Prefer deterministic libs and configs; avoid wall-clock and unseeded RNG.
- Pin agreements after simulate; treat receipts as your audit trail.
- Keep images small and reproducible for faster CI/CD.

# TCP Servers

A Rust project implementing all major protocols that run over TCP — built from scratch using `std::net::TcpListener`. Each protocol lives in its own binary under `src/bin/`.

## Project Goal

Implement every major TCP-based protocol from the ground up, with no external protocol libraries — just raw TCP streams and Rust.

## Protocols

| Protocol   | File                  | Status        | Port  |
|------------|-----------------------|---------------|-------|
| HTTP/1.1   | `src/bin/http.rs`     | 🟡 In Progress | 8000  |
| FTP        | `src/bin/ftp.rs`      | ⬜ Planned     | 21    |
| SMTP       | `src/bin/smtp.rs`     | ⬜ Planned     | 25    |
| POP3       | `src/bin/pop3.rs`     | ⬜ Planned     | 110   |
| IMAP       | `src/bin/imap.rs`     | ⬜ Planned     | 143   |
| SSH        | `src/bin/ssh.rs`      | ⬜ Planned     | 22    |
| DNS/TCP    | `src/bin/dns.rs`      | ⬜ Planned     | 53    |
| WebSocket  | `src/bin/websocket.rs`| ⬜ Planned     | 8080  |
| Redis RESP | `src/bin/redis.rs`    | ⬜ Planned     | 6379  |
| MQTT       | `src/bin/mqtt.rs`     | ⬜ Planned     | 1883  |
| Telnet     | `src/bin/telnet.rs`   | ⬜ Planned     | 23    |

## Running a Protocol

Each protocol is a separate binary. Run any with:

```bash
cargo run --bin <protocol>
```

For example:

```bash
cargo run --bin http
cargo run --bin ftp
cargo run --bin smtp
```

## HTTP/1.1 — Current State & Roadmap

### Implemented
- TCP listener on `127.0.0.1:8000`
- Accept incoming connections
- Send a valid HTTP/1.1 response with JSON body and correct headers (`Content-Length`, `Content-Type`, `Connection: close`)

### TODO — HTTP
- [ ] Request parsing (method, path, HTTP version)
- [ ] Header parsing (key-value pairs)
- [ ] Body parsing (with `Content-Length` support)
- [ ] Routing (match method + path to handlers)
- [ ] Query string parsing
- [ ] Persistent connections (`Connection: keep-alive`)
- [ ] Chunked transfer encoding
- [ ] HTTP/1.0 support
- [ ] Error responses (400, 404, 405, 500)
- [ ] Middleware pipeline (logging, auth, etc.)

## General TODO

- [ ] Thread-per-connection or async (tokio) concurrency model
- [ ] TLS support (for HTTPS, SMTPS, IMAPS, etc.)
- [ ] Shared connection handling abstractions
- [ ] Protocol-level error handling and graceful shutdown
- [ ] Integration tests per protocol

## Structure

```
src/
└── bin/
    ├── http.rs
    ├── ftp.rs       (planned)
    ├── smtp.rs      (planned)
    ├── pop3.rs      (planned)
    ├── imap.rs      (planned)
    ├── ssh.rs       (planned)
    ├── dns.rs       (planned)
    ├── websocket.rs (planned)
    ├── redis.rs     (planned)
    ├── mqtt.rs      (planned)
    └── telnet.rs    (planned)
```

## License

[MIT](LICENSE)

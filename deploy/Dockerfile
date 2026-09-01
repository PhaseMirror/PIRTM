# Multi-stage Dockerfile for PIRTM MCP Daemon
FROM rust:1.81-bookworm AS builder

WORKDIR /usr/src/pilang
COPY rust/ .
RUN cargo build --release -p pirtm-compiler

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/pilang/target/release/pirtm /usr/local/bin/pirtm

EXPOSE 8090

ENTRYPOINT ["/usr/local/bin/pirtm"]
CMD ["mcp", "start", "--transport", "tcp", "--port", "8090"]

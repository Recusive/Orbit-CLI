# codex-rs/network-proxy/

Network proxy library for the Codex sandbox. Provides HTTP and SOCKS5 proxy servers that enforce domain-based network access policies.

## What this folder does

The `codex-network-proxy` crate implements a transparent network proxy that sits between sandboxed commands and the internet. It supports:

- **HTTP proxy** with domain allow/deny lists and method restrictions (Full vs Limited mode)
- **SOCKS5 proxy** with optional UDP support
- **MITM TLS interception** for inspecting HTTPS traffic when enabled
- **Upstream proxy chaining** for environments behind corporate proxies
- **Unix socket proxying** for container daemon access (e.g., `docker.sock`)
- **Dynamic policy reloading** at runtime
- **Blocked request observation** for audit/logging

The proxy enforces network policies defined in `config.toml` and can operate in Full mode (all methods allowed) or Limited mode (GET/HEAD/OPTIONS only).

## Where it plugs in

- **Consumed by**: `codex-core` (spawns the proxy for sandbox network enforcement)
- **Depends on**: `rama-*` crates (HTTP/TCP/TLS/SOCKS5 framework), `codex-utils-absolute-path`, `codex-utils-home-dir`, `codex-utils-rustls-provider`, `globset` (domain matching), `tokio`, `tracing`

## Key files

| File | Role |
|------|------|
| `Cargo.toml` | Crate manifest; rama-* dependencies at pinned alpha versions |
| `README.md` | User-facing documentation |
| `src/lib.rs` | Public API surface: re-exports config, proxy builder/handle, network policy types, state management |
| `src/config.rs` | `NetworkProxyConfig`/`NetworkProxySettings`: proxy URL, SOCKS URL, mode (Full/Limited), domain lists, unix socket allowlist, loopback clamping, address resolution |
| `src/proxy.rs` | `NetworkProxy`/`NetworkProxyBuilder`/`NetworkProxyHandle`: main proxy lifecycle, CLI `Args`, environment variable constants for proxy URLs |
| `src/http_proxy.rs` | HTTP proxy request handling with domain policy enforcement |
| `src/socks5.rs` | SOCKS5 proxy implementation |
| `src/mitm.rs` | MITM TLS interception for HTTPS method enforcement |
| `src/certs.rs` | TLS certificate generation for MITM |
| `src/policy.rs` | Domain normalization and matching logic |
| `src/network_policy.rs` | `NetworkPolicyDecider` trait and `NetworkPolicyDecision`/`NetworkDecision` types |
| `src/state.rs` | `NetworkProxyState`, `NetworkProxyConstraints`, config validation |
| `src/runtime.rs` | `ConfigReloader`, `BlockedRequestObserver`, runtime state management |
| `src/upstream.rs` | Upstream proxy chaining logic |
| `src/responses.rs` | HTTP error response generation |
| `src/reasons.rs` | Human-readable block reason formatting |

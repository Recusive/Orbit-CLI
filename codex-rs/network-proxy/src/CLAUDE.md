# codex-rs/network-proxy/src/

Source directory for the `codex-network-proxy` crate.

## What this folder does

Implements an HTTP + SOCKS5 network proxy that enforces domain-based access policies for sandboxed Codex commands. Built on the `rama` framework for async networking.

## Key files

| File | Purpose |
|------|---------|
| `lib.rs` | Module declarations and public re-exports. Exports: `NetworkProxyConfig`, `NetworkMode`, `NetworkProxy`, `NetworkProxyBuilder`, `NetworkProxyHandle`, `Args`, proxy env var constants, `NetworkPolicyDecider`, `NetworkPolicyDecision`, `BlockedRequestObserver`, `ConfigReloader`, state types |
| `config.rs` | Configuration types: `NetworkProxyConfig` (top-level), `NetworkProxySettings` (enabled, proxy_url, socks_url, mode, domain lists, unix socket allowlist, MITM flag, bind address settings). `NetworkMode::Full` allows all methods; `NetworkMode::Limited` restricts to GET/HEAD/OPTIONS. `resolve_runtime()` validates config and produces socket addresses. `clamp_bind_addrs()` enforces loopback binding when unix sockets are enabled |
| `proxy.rs` | `NetworkProxy` lifecycle management, `NetworkProxyBuilder` for configuration, `NetworkProxyHandle` for runtime control. Manages reserved TCP listeners, proxy task spawning. Defines `PROXY_URL_ENV_KEYS`, `NO_PROXY_ENV_KEYS`, `ALL_PROXY_ENV_KEYS` |
| `http_proxy.rs` | HTTP proxy request processing: CONNECT tunneling, plain HTTP forwarding, domain policy enforcement, method restriction in Limited mode |
| `socks5.rs` | SOCKS5 proxy implementation with optional UDP relay support |
| `mitm.rs` | Man-in-the-middle TLS interception: decrypts HTTPS to inspect HTTP methods and enforce Limited mode policy on inner requests |
| `mitm_tests.rs` | Unit tests for MITM functionality |
| `certs.rs` | Dynamic TLS certificate generation for MITM: creates per-domain certificates signed by a local CA |
| `policy.rs` | `normalize_host()` for consistent domain matching; globset-based domain allow/deny list evaluation |
| `network_policy.rs` | `NetworkPolicyDecider` trait, `NetworkPolicyRequest`/`NetworkPolicyRequestArgs`, `NetworkPolicyDecision`, `NetworkDecision` (Allow/Deny with source tracking), `NetworkProtocol` enum |
| `state.rs` | `NetworkProxyState` (runtime proxy state), `NetworkProxyConstraints` (policy validation), `PartialNetworkConfig`/`PartialNetworkProxyConfig` (partial config updates), `build_config_state()`, `validate_policy_against_constraints()` |
| `runtime.rs` | `ConfigReloader` (hot-reload proxy config), `BlockedRequestObserver`/`BlockedRequest` (audit trail for denied requests), `ConfigState`, `NetworkProxyState` |
| `upstream.rs` | Upstream proxy chaining: forwards requests through an existing corporate/system proxy |
| `responses.rs` | HTTP error response builders for blocked/denied requests |
| `reasons.rs` | Human-readable formatting of block reasons for logging and error responses |

## Imports / exports

- **Imports from workspace**: `codex-utils-absolute-path`, `codex-utils-home-dir`, `codex-utils-rustls-provider`
- **External deps**: `rama-core`, `rama-http`, `rama-http-backend`, `rama-net`, `rama-socks5`, `rama-tcp`, `rama-tls-rustls`, `rama-unix` (unix only), `globset`, `anyhow`, `clap`, `tokio`, `tracing`, `url`
- **Exports**: See `lib.rs` for the full public API

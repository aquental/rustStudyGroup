# Symposium

Concise introduction to **Symposium**, the Rust project hosted at https://github.com/symposium-dev/symposium.

## What is Symposium?

**Symposium** is an open-source framework for building **AI-native development tools** "the Rust way". It focuses on enabling **AI agents** to collaborate more effectively with human developers inside modern code editors.

It is **not** a database library, SQL toolkit, ORM, or general-purpose Rust crate like SQLx — instead, it targets the emerging space of **AI-assisted programming**, with strong emphasis on source-code awareness, editor integration, and extensible agent workflows.

The project describes itself as:

> "AI the Rust Way"

It provides infrastructure so AI agents can:

- Directly read and reason about real Rust crate source code (instead of relying only on docs or hallucinations)
- Interact with code editors via standardized protocols
- Support future human–AI collaborative patterns

## Key Features (as of early 2026)

- **Rust Crate Sources Tool** — AI agents can query and analyze the actual source code of any published Rust crate from crates.io.
- **ACP / SACP Protocol Support** — Built around the **Agent-Client Protocol** (ACP) and Symposium extensions (SACP), enabling structured communication between editors and AI agents.
- **Editor Integrations**:
  - Zed (native support)
  - VS Code (via dedicated extension)
  - Planned: IntelliJ, NeoVim
- **Proxy Components** — Modular pieces like `symposium-acp-proxy`, `symposium-crate-sources-proxy` for orchestrating agent chains and tools.
- **Upcoming / Planned**:
  - Rich human–AI collaboration patterns (Sparkle identities, taskspaces, walkthroughs)
  - Crate-author-provided AI metadata via `Cargo.toml` (domain-specific knowledge injection)

## Current Status

- **Pre-alpha / exploratory** — Frequent breaking changes expected.
- Actively developed (multiple releases in late 2025 / early 2026, 10+ contributors).
- ~67 stars on GitHub (small but growing niche project).
- Multiple related crates published on crates.io under the `symposium-*` namespace.

## Key Links & References (January 2026)

- **Main Repository** (source of truth)  
  https://github.com/symposium-dev/symposium

- **Project Website**  
  https://symposium.dev/

- **Documentation / Book**  
  https://symposium-dev.github.io/symposium/

- **Contributing Guide**  
  https://symposium-dev.github.io/symposium/contribute.html

- **Main Crates on crates.io / docs.rs** (examples):
  - `symposium-acp-proxy` → core proxy & orchestration  
    https://crates.io/crates/symposium-acp-proxy
  - `symposium-crate-sources-proxy` → crate source research tool  
    https://crates.io/crates/symposium-crate-sources-proxy
  - `sacp` / `sacp-tokio` → protocol SDKs  
    https://crates.io/crates/sacp

## Who is it for?

Symposium is interesting if you are:

- Building AI coding assistants or agents
- Working on editor extensions for AI (Zed / VS Code especially)
- Interested in protocol-driven human–AI collaboration in Rust
- Want AI agents to have deeper, verifiable access to Rust ecosystem source code

It is **not** a general-purpose library for most Rust applications — it's a specialized toolkit for the AI-in-the-editor niche.

Symposium is to experiment with AI-powered Rust development, Symposium is one of the more Rust-native efforts in this space (as opposed to relying solely on external LLM APIs or non-Rust tooling).

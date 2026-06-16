# AGENTS.md — tuidns

## Build & Run

```bash
cargo build           # debug build
cargo build --release # release build
cargo run             # run TUI
cargo check           # check compilation quickly
cargo clippy          # lint (14 warnings currently)
cargo fmt             # format code
cargo test            # no tests exist in repo
```

## External Runtime Dependencies

Must be in `PATH`: `dig`, `nslookup`, `whois`, `openssl`, `ping`.

## Architecture (current, NOT the README)

```
src/
  main.rs              -- Entry point: raw mode, alternate screen, event loop
  app.rs               -- App state, key handling, TUI layout
  domain/              -- Data types (DnsQuery, Host, etc.) + static panel catalog
  infrastructure/      -- Raw CLI calls (dig, ping, openssl, whois, nslookup) + TCP port check
  orchestrators/       -- Parallel execution coordination (spawns threads, aggregates results)
  services/            -- Business logic wrappers around infrastructure calls
  ui/                  -- Ratatui widgets (tables, cursor, info panels)
```

## Known Issues

- **Clipboard breaks on X11**: `clipboard_service.rs` creates and drops `ClipboardContext` per call. Without a clipboard manager, `x11-clipboard` releases selection ownership when `ctx` drops, so paste yields nothing. Fix: keep `ClipboardContext` alive (e.g., in `App` struct) or use `xclip` directly.
- **README architecture section is stale**: still documents `models/` and `controllers/`; actual layout uses `domain/`, `infrastructure/`, `orchestrators/`.
- **No tests**: entire repo has zero unit/integration tests.
- **`cargo clippy` has 14 warnings**: includes `collapsible_if`, `needless_borrow`, `let_and_return`, `single_char_add_str`, `useless_format`, etc.

## Crate dependencies

`copypasta 0.10.2`, `crossterm 0.29.0`, `ratatui 0.30.0`. Rust edition 2024.

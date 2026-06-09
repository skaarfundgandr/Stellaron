# Stellaron — Tauri v2 Desktop Ebook Reader

## Project layout

```
stellaron/            ← Frontend (Vite + React + TypeScript)
├── src/              React app (entry: src/main.tsx → App.tsx)
├── index.html        HTML shell, port 1420
├── vite.config.js    Tauri-aware Vite config (strict port, HMR)
└── package.json      npm scripts, frontend deps

stellaron/src-tauri/  ← Backend (Rust, Tauri v2)
├── src/              Layered: domain → application → infrastructure → api
├── tests/            Integration tests
├── Cargo.toml        crate name = stellaron_lib (NOT stellaron)
├── AGENTS.md         FULL backend docs — read this for Rust conventions
└── docs/backend/     Detailed layer docs
```

## Critical commands

```bash
# Dev (opens Tauri window with hot-reload on both sides)
npm run tauri dev

# Frontend-only build (tsc typecheck THEN vite build — type errors block it)
npm run build

# Lint & format (run before committing)
cargo clippy --all-targets --all-features
cargo fmt

# Run all Rust tests (MUST be serial — SQLite)
cargo test -- --test-threads=1

# Run diesel migrations
diesel migration run       # apply pending
diesel migration redo      # revert + re-apply last
diesel migration generate <name>  # scaffold new
```

## Rust backend — read src-tauri/AGENTS.md first

Key gotchas every agent hits:

- **Crate name is `stellaron_lib`**, not `stellaron`. All internal `use stellaron_lib::...` imports use this.
- **SQLite write locking**: every write must acquire `lock_db()` or you get "database is locked".
- **`#[serial_test::serial]`** on every DB test. Run with `--test-threads=1`.
- **All file I/O must use `tokio::task::spawn_blocking`** — blocking the Tokio runtime deadlocks.
- **Do not store DB connections in structs**. Always acquire from the pool.
- After any migration, commit `src/infrastructure/database/models/schema.rs`.

## Frontend conventions

- **React Router v7**: `BrowserRouter` at root. Layouts: `RootLayout` (pages with sidebar/header), `PlainLayout` (full-screen info pages), and standalone `BookPage` (immersive reader, no layout wrapper).
- **Tailwind v4** via `@tailwindcss/postcss` PostCSS plugin. Custom `stellar-*` colors/animations defined in `tailwind.config.js`.
- **`tailwind.config.js` is CommonJS** even though the project is `"type": "module"` — it uses `require()` and `module.exports`.
- **Tauri IPC**: frontend calls `invoke("command_name", { args })` from `@tauri-apps/api/core`. All commands registered in `src-tauri/src/main.rs`.
- **Dual lockfiles exist**: `package-lock.json` (npm) and `bun.lock` (bun). Prefer npm.
- **Fonts**: Inter (from `@fontsource/inter` package) + Playfair Display (Google Fonts CDN in `index.html`).

## Tauri configuration

- Dev server: `http://localhost:1420` (fixed port, `strictPort: true`)
- Capabilities: `src-tauri/capabilities/default.json` — fs scope is `$HOME/**` and `C:\**`
- Plugins: opener, dialog, process, fs
- `npm run tauri dev` auto-starts the Vite dev server via `beforeDevCommand`

- Prefix commands such as `cargo`/`git` with `rtk` (e.g. `rtk cargo check`) to filter output and save tokens.

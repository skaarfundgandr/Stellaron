# Stellaron — Agent Instructions

Tauri v2 desktop ebook reader. React 19 + Tailwind v4 frontend, Rust backend with Diesel/SQLite.

## Commands

```bash
# Full dev (starts Vite on :1420 + compiles Rust)
cd src-tauri && cargo tauri dev

# Frontend only (Vite dev server)
npm run dev

# Build frontend (tsc + vite build)
npm run build

# Rust checks
cd src-tauri && cargo check
cd src-tauri && cargo clippy

# Tests — MUST be single-threaded (SQLite limitation)
cd src-tauri && cargo test -- --test-threads=1
```

No linter, formatter, or CI is configured.

## Architecture

Layered Rust backend in `src-tauri/src/`:

- `domain/` — Pure models, DTOs, repository traits, `DomainError`. Zero framework deps.
- `application/` — Use-case functions + Diesel-backed repo implementations. State wired in `state.rs`.
- `infrastructure/` — DB pool, Diesel ORM models, `schema.rs`, migrations, file handlers (EPUB/PDF).
- `api/` — `#[tauri::command]` handlers bridging frontend to use cases.
- `utils/` — Checksum/serialization helpers.

Frontend in `src/`: pages, components, `services/tauriService.ts` (IPC calls), `types/`.

## Critical Conventions

**SQLite write locking:** Writes run inside a `conn.transaction(…)`. Contention is handled per-connection via `PRAGMA busy_timeout = 2000` (set in the deadpool `post_create` hook, as its own statement, first). No app-level `lock_db()` mutex. See `src-tauri/AGENTS.md` “SQLite Write Contention” for the full rationale (diesel/diesel-async do not auto-handle this).

```rust
let mut conn = connect_from_pool().await?;
conn.transaction(async |connection| { /* insert/update/delete */ Ok(()) }).await?;
```

**Lib crate name:** `stellaron_lib` (not `stellaron`). Use this in test imports:
```rust
use stellaron_lib::domain::models::book::Book;
```

**Test pattern:** Every DB test needs all three:
```rust
#[tokio::test]
#[serial_test::serial]
async fn test_name() { ... }
```

**Diesel schema:** `schema.rs` is auto-generated — never edit manually. Run `diesel migration run` after creating migrations. `DATABASE_URL` defaults to `./database.db` if unset.

**Migrations:** Embedded at compile time via `embed_migrations!()` and auto-applied on app startup. Config in `src-tauri/diesel.toml`.

**Repository pattern:** Traits in `domain/repository.rs`, implementations in `application/repository/*.rs`, wired via `AppState` in `main.rs`.

## Frontend

- Tailwind v4 via `@tailwindcss/postcss` (not the v3 plugin). Custom theme colors defined in `tailwind.config.js` (`stellar-*`).
- Vite ignores `src-tauri/` and `*.db*` files during watch.
- No eslint or prettier configured — follow existing code style.

## Gotchas

- `libsqlite3-sys` must use `bundled` feature (already set in Cargo.toml).
- Everything is Tokio async. Blocking code will deadlock.
- Old `database.db` files may be incompatible after schema changes — delete and restart.
- Test fixtures in `src-tauri/tests/fixtures/` (test.pdf, epub files).
- Send only raw HTML from backend to frontend; no CSS/JS processing in Rust handlers.

## Docs

Detailed architecture docs in `docs/backend/`. Existing backend-focused AGENTS.md at `src-tauri/AGENTS.md`.

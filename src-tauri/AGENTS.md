# Stellaron — Backend Architecture & Conventions

Tauri v2 desktop ebook reader. Single-user, no auth. See root `AGENTS.md` for commands.

## Layer Architecture

```
[Frontend (React)]
      ↓  Tauri IPC invoke
[api/commands/]      ← thin wrappers: extract AppState, map DomainError → String
      ↓
[api/handlers/]      ← orchestrate repos + use cases, return Result<T, DomainError>
      ↓
[application/]       ← use-case functions (book.rs, annotation.rs, etc.)
      ↓
[domain/]            ← models, DTOs, repository traits, DomainError
      ↑
[infrastructure/]    ← DB pool/Diesel, file parsers (EPUB/PDF)
```

Architecture follows [sheroz/axum-rest-api-sample](https://github.com/sheroz/axum-rest-api-sample).

## Project Structure

```
src/
├── main.rs                        # DB init, embedded migrations, AppState wiring, Tauri launch
├── lib.rs                         # Crate root — exports as stellaron_lib
│
├── domain/
│   ├── models/                    # Pure domain entities (book.rs, author.rs, etc.)
│   ├── dto/                       # DTOs with From<Model> impls (book_dto.rs)
│   ├── repository.rs              # All repository traits in one file + New*/Update* input structs
│   └── error.rs                   # DomainError enum (thiserror)
│
├── application/
│   ├── *.rs                       # Use-case functions (import_book, list_books, etc.)
│   ├── state.rs                   # AppState: Arc<dyn Trait> for each repo
│   ├── repository/                # Diesel-backed repo implementations
│   └── service/                   # Application services (book_service.rs)
│
├── infrastructure/
│   ├── database/
│   │   ├── database.rs            # Connection pool (deadpool + diesel-async) + busy_timeout pragma
│   │   ├── models/                # Diesel ORM models
│   │   ├── schema.rs              # AUTO-GENERATED — never edit manually
│   │   └── migrations/            # SQL migrations (embedded at compile time)
│   └── file_handlers/             # EPUB parser (rbook), PDF parser (pdf_oxide)
│
├── api/
│   ├── commands/                  # #[tauri::command] handlers — thin wrappers
│   ├── handlers/                  # Business logic orchestration layer
│   └── error.rs                   # API error mapping
│
└── utils/                         # Checksum/serialization helpers
```

## Critical Conventions

### SQLite Write Contention (busy_timeout)

SQLite allows only one writer at a time at the file level (even in WAL mode). Rather than serializing writes with an app-level mutex, every pooled connection sets `PRAGMA busy_timeout = 2000` in the deadpool `post_create` hook — SQLite then waits/retries up to 2 s instead of failing immediately with "database is locked". No `lock_db()` / global mutex exists.

Per diesel's official guidance, each `PRAGMA` must be its **own** `batch_execute` call and order matters (`busy_timeout` first so later pragmas can wait):

```rust
conn.batch_execute("PRAGMA busy_timeout = 2000;").await?;
conn.batch_execute("PRAGMA journal_mode = WAL;").await?;
conn.batch_execute("PRAGMA synchronous = NORMAL;").await?;
conn.batch_execute("PRAGMA foreign_keys = ON;").await?;
```

Writes therefore need only a transaction, no lock:

```rust
use crate::infrastructure::database::database::connect_from_pool;
let mut conn = connect_from_pool().await?;
conn.transaction(async |connection| {
    diesel::insert_into(…).execute(connection).await?;
    Ok::<(), diesel::result::Error>(())
}).await?;
```

> Verified against diesel 2.3.x / diesel-async 0.9 (current pins): neither auto-sets `busy_timeout` nor serializes SQLite writes; the "database is locked" error is a SQLite limitation, not a diesel bug. The pool wraps `SyncConnectionWrapper<SqliteConnection>`, which runs blocking SQLite ops via `spawn_blocking` — concurrent writers on separate pool connections still race without `busy_timeout`.

### Repository Pattern

- **Traits:** All in `domain/repository.rs` (one file, all entities).
- **Implementations:** `application/repository/*.rs` (one file per entity).
- **Input structs:** `NewBook`, `UpdateBook`, `NewBookmark`, etc. defined alongside traits in `domain/repository.rs`.
- **Return types:** `Result<Vec<T>, DomainError>` or `Result<Option<T>, DomainError>`.

### Use Case Pattern

Each use case is a plain async fn in `application/*.rs`. Takes `&Arc<dyn Trait>` refs + domain inputs, returns DTO.

### Dependency Injection

All repos wired in `main.rs` via `AppState`, passed to Tauri commands as `State<'_, AppState>`.

### Commands vs Handlers

- `api/commands/*.rs` — Tauri `#[tauri::command]` entry points. Extract `AppState`, call handler, map `DomainError` to `String`.
- `api/handlers/*.rs` — Orchestration layer. Extract repos from `AppState`, call application use cases. Return `Result<T, DomainError>`.

## Database & Migrations

- `DATABASE_URL` defaults to `./database.db` if unset (via `dotenvy`).
- Migrations are embedded at compile time (`embed_migrations!()`) and auto-applied on startup.
- Config: `diesel.toml` — schema output to `src/infrastructure/database/models/schema.rs`.
- **Always commit `schema.rs`** after running migrations.

### Creating a New Migration

Always use `diesel migration generate` — **never hand-create the migration folder or `up.sql`/`down.sql` by hand**. The command auto-scaffolds the folder under the configured `migrations/` directory with both files empty and ready to edit:

```bash
cd src-tauri && DATABASE_URL=./database.db diesel migration generate <name>   # 0002_ratings, 0003_collections, …
# edit <name>/up.sql and <name>/down.sql, then:
cd src-tauri && DATABASE_URL=./database.db diesel migration run   # applies SQL + regenerates schema.rs
```

Full workflow per new table:
1. **Generate** the scaffolding (creates `<version>_<name>/` with empty `up.sql` + `down.sql`).
2. **Fill** `up.sql` (CREATE TABLE / indexes) and `down.sql` (DROP TABLE IF EXISTS …).
3. **Run** `diesel migration run` — applies the SQL *and* regenerates `schema.rs` (adds the table block, `joinable!`s, and `allow_tables_to_appear_in_same_query!`).
4. **Commit** both the new migration folder and the regenerated `schema.rs`.

CLI reference: `diesel migration generate <name>`, `diesel migration run`, `diesel migration redo`, `diesel migration list`.
- `schema.rs` is auto-generated — never edit it manually. The build breaks until `diesel migration run` regenerates it, which is the signal to run it.

## Testing

**All tests live exclusively in `src-tauri/tests/` — never add `#[cfg(test)] mod tests` blocks inside `src/`.** The existing layout (`epub_handler_tests.rs`, `pdf_handler_tests.rs`, `image_processing_test.rs`) is the pattern. Integration-only tests keep the library binary-lean and compile tests only under the test runner. Verify with `grep -rn '#\[cfg(test)\]\|#\[tokio::test\]\|#\[test\]' src-tauri/src` (should return nothing).

SQLite does not support concurrent writes — all DB tests must be serial:

```bash
cargo test -- --test-threads=1
```

Every DB test needs all three decorators:

```rust
#[tokio::test]
#[serial_test::serial]
async fn test_name() { ... }
```

Import crate as `stellaron_lib` in integration tests (`tests/`).

Test fixtures in `tests/fixtures/` (test.pdf, epub files).

## Gotchas

- `libsqlite3-sys` must use `bundled` feature (already in Cargo.toml).
- Everything is Tokio async. Blocking code will deadlock.
- Old `database.db` files may be incompatible after schema changes — delete and restart.
- Send only raw HTML from handlers to frontend; no CSS/JS processing in backend.
- `schema.rs` is auto-generated — never edit manually.

## Docs

Detailed architecture docs in `docs/backend/`. Use Context7 for crate/library documentation lookup.

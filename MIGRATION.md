# libsql -> turso backend migration

This release replaces the `libsql` backend with `turso` (`0.4.4`) as a full backend swap.

## Cargo changes

- Removed `libsql` dependency, patch override, and wasm target-specific `libsql` section.
- Added optional dependency: `turso = { version = "0.4.4", features = ["sync"], optional = true }`.
- Feature flags now center on Turso:
  - `default = ["turso_default"]`
  - `turso_default = ["turso", "dep:serde", "dep:serde_json", "dep:chrono", "dep:uuid", "dep:libsql-orm-macros", "dep:anyhow"]`
  - `turso = ["dep:turso"]`
- Removed legacy `native = ["libsql"]` feature.
- `cloudflare` remains as a legacy/deprecated feature gate for worker-only dependencies and does not provide Turso database connectivity.

## Runtime and API behavior changes

- `Database` now wraps `turso::Connection` and keeps the underlying DB handle alive internally.
- `Database::new_local(path)` now creates local DB connections through `turso::Builder`.
- `Database::new_connect(url, token)` now uses `turso::sync::Builder` remote sync connections.
- Query/execute parameter handling now maps empty params to `()` and non-empty params to `turso::Params::Positional`.

## Compatibility layer updates

- `src/compat.rs` now aliases `Libsql*` compatibility names to `turso::{Error, Row, Rows, Value}` when `turso` is enabled.
- Non-`turso` fallback stubs remain available for wasm-only/no-db builds.

## Query/model row mapping changes

- `turso::Row` does not expose `column_name(i)`, so dynamic row mapping now uses prepared statement metadata (`Statement::columns()`) where needed.
- `Model::row_to_map` now maps row values using `Model::columns()` order.
- `row.get_value(i)` handling was updated for Turso's `Result<Value>` API (`.ok().unwrap_or(...)`).

## Error conversion changes

- `Error` now implements `From<turso::Error>` (replacing `From<libsql::Error>`).

## Cloudflare note

- Cloudflare Workers database support from the old `libsql` backend path is not available in this Turso backend migration.
- If you require Cloudflare database connectivity, stay on a legacy `libsql`-backed release until a Turso-compatible worker path is introduced.

# OrchidDB Rust client

Compile Cypher, Gremlin text, or SPARQL to SQL. Execute through your own
`SqlSession`, whose result implements Arrow 58 `RecordBatchReader`.
No database driver is a production dependency. You own connections, transactions,
extensions, UDFs, schema discovery, and cache invalidation. Cross-engine federation
is not implemented; route a plan to one compatible engine.

```rust,ignore
let plan = orchiddb_client::compile(request).await?;
let mut batches = orchiddb_client::execute(&mut your_session, &plan).await?;
for batch in &mut batches {
    let batch = batch?; // Native Arrow RecordBatch; no cell-by-cell conversion.
}
```

Add the client to an application's Cargo.toml (Cargo.lock records the commit;
production applications should also set `rev` to their reviewed commit):

```toml
[dependencies]
orchiddb-client = { git = "https://github.com/OrchidDB/OrchidDB-rust", branch = "main", default-features = false }
```

Run the complete caller-owned DuckDB example:

```sh
cargo run --example borrowed_duckdb
cargo test
```

DuckDB is dev-only, bundled by the test feature. To use an existing matching
DuckDB 1.5.2 library set `DUCKDB_LIB_DIR` and run with `--no-default-features`.
Retained Rust batches retain their buffers after advancing/dropping the reader.
Dropping readers releases the mutable session borrow. Arrow batch transport does
not guarantee streaming execution inside every database.

See [the example](examples/borrowed_duckdb.rs) for mappings, declared UDF signatures,
borrowed connections, native Arrow schema/batches, and caller-controlled rollback.
[Core protocol documentation](https://github.com/OrchidDB/OrchidDB/blob/main/docs/compiler.md).

## Releases

Use a pinned Git dependency from this repository (`package = "orchiddb-client"`).
The release workflow validates `vVERSION`, tests, and creates a draft GitHub release
with a checksummed source archive. The core commit is pinned in Cargo.toml/lock.
Crates.io publication is deliberately disabled: the compiler is not yet published
and depends on a modified path-vendored spargebra. Publishing a manifest rewritten
to upstream spargebra would silently change compiler behavior. Once core and its
parser dependency are publishable, replace Git dependencies with versioned crates,
remove `publish = false`, and configure a crates.io token/trusted publisher.
The existing OrchidDB license applies; see LICENSE.md.

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
cargo run --example borrowed_duckdb --features bundled-test-driver
cargo test --features bundled-test-driver
```

DuckDB is dev-only. The optional `bundled-test-driver` feature builds it for
examples and tests; default features are empty. To use an existing matching
DuckDB 1.5.2 library, set `DUCKDB_LIB_DIR` and omit that feature.
Retained Rust batches retain their buffers after advancing/dropping the reader.
Dropping readers releases the mutable session borrow. Arrow batch transport does
not guarantee streaming execution inside every database.

See [the example](examples/borrowed_duckdb.rs) for mappings, declared UDF signatures,
borrowed connections, native Arrow schema/batches, and caller-controlled rollback.
[Core protocol documentation](https://github.com/OrchidDB/OrchidDB/blob/main/docs/compiler.md).

## Releases

Version 0.1.0 is prepared for crates.io as `orchiddb-client`, depending on
`orchiddb = 0.1.0`. The engine embeds its modified SPARQL parser; no separate
parser crate is needed. The Git dependency stays pinned for development until
the first registry release is available.

Run the manual `release.yml` workflow with `ref=main`, `publish=false` to test
and verify both registry archives before either is published. Download the
checksummed `verified-crates` workflow artifact for inspection. Nothing is
uploaded to crates.io unless `publish=true` is explicitly selected with a
matching existing version tag. Publish the engine first, then the client.
Both use the `CARGO_REGISTRY_TOKEN` secret in the `crates-io` environment.
The CLI is distributed separately through GitHub Releases.

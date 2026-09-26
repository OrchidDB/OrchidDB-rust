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

Add the published client to your application's Cargo.toml:

```toml
[dependencies]
orchiddb-client = { version = "=0.1.0", default-features = false }
```

Run the complete caller-owned DuckDB example:

```sh
cargo run --manifest-path examples/Cargo.toml --features bundled
```

The standalone example depends on crates.io packages, including its own DuckDB driver. Its `bundled` feature builds DuckDB; OrchidDB itself contains no driver. To use an existing matching
DuckDB 1.5.2 library, set `DUCKDB_LIB_DIR` and omit that feature.
Retained Rust batches retain their buffers after advancing/dropping the reader.
Dropping readers releases the mutable session borrow. Arrow batch transport does
not guarantee streaming execution inside every database.

See [the example](examples/borrowed_duckdb.rs) for mappings, declared UDF signatures,
borrowed connections, native Arrow schema/batches, and caller-controlled rollback.
[Core protocol documentation](https://github.com/OrchidDB/OrchidDB/blob/main/docs/compiler.md).

## Releases

Version 0.1.0 is published on crates.io as `orchiddb-client`, depending on
`orchiddb = 0.1.0`. The engine embeds its modified SPARQL parser; no separate
parser crate is needed. The library checkout retains its pinned Git dependency for core development; the standalone example uses the published registry dependency.

Run the manual `release.yml` workflow with `ref=main`, `publish=false` to test
and verify both registry archives before either is published. Download the
checksummed `verified-crates` workflow artifact for inspection. Nothing is
uploaded to crates.io unless `publish=true` is explicitly selected with a
matching existing version tag. Publish the engine first, then the client.
Both use the `CARGO_REGISTRY_TOKEN` secret in the `crates-io` environment.
The CLI is distributed separately through GitHub Releases.

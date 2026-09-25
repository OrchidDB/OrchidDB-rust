//! SQL compilation and Arrow execution using application-owned sessions.
//! No database driver is included in this crate's normal dependency graph.
pub use orchiddb::compiler;
pub use orchiddb::compiler::{compile, compile_json, CompileRequest, CompiledSql};
pub use orchiddb::execution::{execute, ExecutionError, SqlDialect, SqlSession};
pub use arrow;
pub use arrow::record_batch::{RecordBatch, RecordBatchReader};

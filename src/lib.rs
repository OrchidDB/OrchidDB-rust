//! SQL compilation and Arrow execution using application-owned sessions.
//! No database driver is included in this crate's normal dependency graph.
pub use arrow;
pub use arrow::record_batch::{RecordBatch, RecordBatchReader};
pub use orchiddb::compiler;
pub use orchiddb::compiler::{CompileRequest, CompiledSql, compile, compile_json};
pub use orchiddb::execution::{ExecutionError, SqlDialect, SqlSession, execute};

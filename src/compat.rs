//! Compatibility layer for handling different database backends
//!
//! This module provides type aliases and compatibility functions
//! for working with both libsql and WASM-only environments.

#[cfg(all(feature = "libsql", target_arch = "wasm32"))]
pub use libsql::wasm::Rows as LibsqlRows;
#[cfg(all(feature = "libsql", target_arch = "wasm32"))]
pub use libsql::{Error as LibsqlError, Row as LibsqlRow, Value as LibsqlValue};

#[cfg(all(feature = "libsql", not(target_arch = "wasm32")))]
pub use libsql::{
    Error as LibsqlError, Row as LibsqlRow, Rows as LibsqlRows, Value as LibsqlValue,
};

#[cfg(not(feature = "libsql"))]
#[derive(Debug, Clone, PartialEq)]
pub enum LibsqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

#[cfg(not(feature = "libsql"))]
pub struct LibsqlRow {
    data: std::collections::HashMap<String, LibsqlValue>,
}

#[cfg(not(feature = "libsql"))]
impl LibsqlRow {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }

    pub fn get(&self, index: usize) -> Result<&LibsqlValue, crate::error::Error> {
        // For WASM implementation, we'll use a simple index-based access
        // This is a stub - in a real implementation you'd map indices to column names
        Err(crate::error::Error::Generic(
            "Column access by index not supported in WASM mode".to_string(),
        ))
    }

    pub fn get_value(&self, index: usize) -> Option<LibsqlValue> {
        // Stub implementation
        Some(LibsqlValue::Null)
    }

    pub fn column_count(&self) -> usize {
        // Stub implementation
        0
    }

    pub fn column_name(&self, _index: usize) -> Option<&str> {
        // Stub implementation
        None
    }
}

#[cfg(not(feature = "libsql"))]
pub struct LibsqlRows {
    rows: Vec<LibsqlRow>,
    index: std::cell::Cell<usize>,
}

#[cfg(not(feature = "libsql"))]
impl LibsqlRows {
    pub fn new(rows: Vec<LibsqlRow>) -> Self {
        Self {
            rows,
            index: std::cell::Cell::new(0),
        }
    }

    pub async fn next(&self) -> Result<Option<&LibsqlRow>, crate::error::Error> {
        let current_index = self.index.get();
        if current_index < self.rows.len() {
            self.index.set(current_index + 1);
            Ok(self.rows.get(current_index))
        } else {
            Ok(None)
        }
    }
}

#[cfg(not(feature = "libsql"))]
pub type LibsqlError = crate::error::Error;

/// Create a null value compatible with both backends
pub fn null_value() -> LibsqlValue {
    #[cfg(feature = "libsql")]
    return libsql::Value::Null;

    #[cfg(not(feature = "libsql"))]
    return LibsqlValue::Null;
}

/// Create a text value compatible with both backends
pub fn text_value(s: String) -> LibsqlValue {
    #[cfg(feature = "libsql")]
    return libsql::Value::Text(s);

    #[cfg(not(feature = "libsql"))]
    return LibsqlValue::Text(s);
}

/// Create an integer value compatible with both backends
pub fn integer_value(i: i64) -> LibsqlValue {
    #[cfg(feature = "libsql")]
    return libsql::Value::Integer(i);

    #[cfg(not(feature = "libsql"))]
    return LibsqlValue::Integer(i);
}

/// Create a real/float value compatible with both backends
pub fn real_value(f: f64) -> LibsqlValue {
    #[cfg(feature = "libsql")]
    return libsql::Value::Real(f);

    #[cfg(not(feature = "libsql"))]
    return LibsqlValue::Real(f);
}

/// Create a blob value compatible with both backends
pub fn blob_value(b: Vec<u8>) -> LibsqlValue {
    #[cfg(feature = "libsql")]
    return libsql::Value::Blob(b);

    #[cfg(not(feature = "libsql"))]
    return LibsqlValue::Blob(b);
}

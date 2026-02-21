#[cfg(feature = "turso")]
pub use turso::{Error as LibsqlError, Row as LibsqlRow, Rows as LibsqlRows, Value as LibsqlValue};

#[cfg(not(feature = "turso"))]
#[derive(Debug, Clone, PartialEq)]
pub enum LibsqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

#[cfg(not(feature = "turso"))]
#[derive(Clone)]
pub struct LibsqlRow {
    data: std::collections::HashMap<String, LibsqlValue>,
}

#[cfg(not(feature = "turso"))]
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

    pub fn get_value(&self, _index: usize) -> Result<LibsqlValue, crate::error::Error> {
        Ok(LibsqlValue::Null)
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

#[cfg(not(feature = "turso"))]
pub struct LibsqlRows {
    rows: Vec<LibsqlRow>,
    index: usize,
}

#[cfg(not(feature = "turso"))]
impl LibsqlRows {
    pub fn new(rows: Vec<LibsqlRow>) -> Self {
        Self {
            rows,
            index: 0,
        }
    }

    pub async fn next(&mut self) -> Result<Option<LibsqlRow>, crate::error::Error> {
        if self.index < self.rows.len() {
            let row = self.rows[self.index].clone();
            self.index += 1;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }
}

#[cfg(not(feature = "turso"))]
pub type LibsqlError = crate::error::Error;

/// Create a null value compatible with both backends
pub fn null_value() -> LibsqlValue {
    #[cfg(feature = "turso")]
    return turso::Value::Null;

    #[cfg(not(feature = "turso"))]
    return LibsqlValue::Null;
}

/// Create a text value compatible with both backends
pub fn text_value(s: String) -> LibsqlValue {
    #[cfg(feature = "turso")]
    return turso::Value::Text(s);

    #[cfg(not(feature = "turso"))]
    return LibsqlValue::Text(s);
}

/// Create an integer value compatible with both backends
pub fn integer_value(i: i64) -> LibsqlValue {
    #[cfg(feature = "turso")]
    return turso::Value::Integer(i);

    #[cfg(not(feature = "turso"))]
    return LibsqlValue::Integer(i);
}

/// Create a real/float value compatible with both backends
pub fn real_value(f: f64) -> LibsqlValue {
    #[cfg(feature = "turso")]
    return turso::Value::Real(f);

    #[cfg(not(feature = "turso"))]
    return LibsqlValue::Real(f);
}

/// Create a blob value compatible with both backends
pub fn blob_value(b: Vec<u8>) -> LibsqlValue {
    #[cfg(feature = "turso")]
    return turso::Value::Blob(b);

    #[cfg(not(feature = "turso"))]
    return LibsqlValue::Blob(b);
}

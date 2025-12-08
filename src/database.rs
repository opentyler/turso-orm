//! Database connection and query execution
//!
//! This module handles the connection to Turso databases and provides
//! query execution capabilities for Cloudflare Workers.

#[cfg(all(not(target_arch = "wasm32"), feature = "libsql"))]
use libsql::Connection;
#[cfg(all(target_arch = "wasm32", feature = "libsql"))]
use libsql::wasm::{CloudflareSender, Connection};

/// Database connection wrapper for Turso in Cloudflare Workers
///
/// Provides a high-level interface for connecting to and interacting with
/// Turso databases in WebAssembly environments, specifically optimized
/// for Cloudflare Workers.
///
/// # Examples
///
/// ```no_run
/// use libsql_orm::Database;
///
/// async fn connect_example() -> Result<(), Box<dyn std::error::Error>> {
///     let db = Database::new_connect(
///         "libsql://your-db.turso.io",
///         "your-auth-token"
///     ).await?;
///     Ok(())
/// }
/// ```
#[cfg(any(feature = "libsql", not(target_arch = "wasm32")))]
pub struct Database {
    #[cfg(all(target_arch = "wasm32", feature = "libsql"))]
    pub inner: Connection<CloudflareSender>,
    #[cfg(all(not(target_arch = "wasm32"), feature = "libsql"))]
    pub inner: Connection,
}

#[cfg(all(target_arch = "wasm32", not(feature = "libsql")))]
pub struct Database {
    // Placeholder for WASM-only builds without libsql
    _phantom: std::marker::PhantomData<()>,
}

#[cfg(all(target_arch = "wasm32", feature = "libsql"))]
impl From<Connection<CloudflareSender>> for Database {
    fn from(inner: Connection<CloudflareSender>) -> Self {
        Self { inner }
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "libsql"))]
impl From<Connection> for Database {
    fn from(inner: Connection) -> Self {
        Self { inner }
    }
}

#[cfg(feature = "libsql")]
impl Database {
    /// Creates a new database connection to a Turso database
    ///
    /// # Arguments
    ///
    /// * `url` - The database URL (e.g., "turso://your-db.turso.io")
    /// * `token` - The authentication token for the database
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Database` instance or a `libsql::Error`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsql_orm::Database;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let db = Database::new_connect(
    ///         "turso://your-db.turso.io",
    ///         "your-auth-token"
    ///     ).await?;
    ///     println!("Connected to database successfully!");
    ///     Ok(())
    /// }
    /// ```
    #[cfg(target_arch = "wasm32")]
    pub async fn new_connect(url: &str, token: &str) -> std::result::Result<Self, crate::compat::LibsqlError> {
        let conn = Connection::open_cloudflare_worker(url.to_string(), token.to_string());
        conn.execute("SELECT 1", ()).await.map(|_| Self::from(conn))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn new_connect(_url: &str, _token: &str) -> std::result::Result<Self, crate::compat::LibsqlError> {
        // For native builds, return an error directing users to use the full libsql crate
        panic!("Native database connections not supported in this build configuration. Use the 'libsql_default' feature for native support.")
    }

    /// Executes a SQL query with parameters
    ///
    /// # Arguments
    ///
    /// * `sql` - The SQL query string
    /// * `params` - Vector of parameters to bind to the query
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `Rows` iterator or a `libsql::Error`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsql_orm::Database;
    ///
    /// async fn query_example(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    ///     let rows = db.query(
    ///         "SELECT * FROM users WHERE age > ?",
    ///         vec![libsql::Value::Integer(18)]
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn query(
        &self,
        sql: &str,
        params: Vec<crate::compat::LibsqlValue>,
    ) -> Result<crate::compat::LibsqlRows, crate::compat::LibsqlError> {
        self.inner.query(sql, params).await
    }

    /// Execute a SQL statement with parameters
    pub async fn execute(
        &self,
        sql: &str,
        params: Vec<crate::compat::LibsqlValue>,
    ) -> Result<u64, crate::compat::LibsqlError> {
        self.inner.execute(sql, params).await
    }
}

#[cfg(all(target_arch = "wasm32", not(feature = "libsql")))]
impl Database {
    /// Creates a new database connection for WASM without libsql
    pub async fn new_connect(_url: &str, _token: &str) -> Result<Self, crate::error::Error> {
        Ok(Database {
            _phantom: std::marker::PhantomData,
        })
    }

    /// Query method stub for WASM-only builds
    pub async fn query(&self, _sql: &str, _params: Vec<crate::compat::LibsqlValue>) -> Result<crate::compat::LibsqlRows, crate::compat::LibsqlError> {
        // Return empty results for WASM-only builds
        Ok(crate::compat::LibsqlRows::new(vec![]))
    }

    /// Execute method stub for WASM-only builds
    pub async fn execute(&self, _sql: &str, _params: Vec<crate::compat::LibsqlValue>) -> Result<u64, crate::compat::LibsqlError> {
        // Return success for WASM-only builds (stub implementation)
        Ok(0)
    }
}

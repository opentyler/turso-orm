#[cfg(feature = "turso")]
enum DatabaseInner {
    Local(turso::Database),
    Sync(turso::sync::Database),
}

#[cfg(feature = "turso")]
pub struct Database {
    _db: DatabaseInner,
    pub inner: turso::Connection,
}

#[cfg(feature = "turso")]
impl Database {
    fn keep_alive(&self) {
        match &self._db {
            DatabaseInner::Local(db) => {
                let _ = db;
            }
            DatabaseInner::Sync(db) => {
                let _ = db;
            }
        }
    }

    pub async fn new_local(path: &str) -> std::result::Result<Self, turso::Error> {
        let db = turso::Builder::new_local(path).build().await?;
        let conn = db.connect()?;
        Ok(Self {
            _db: DatabaseInner::Local(db),
            inner: conn,
        })
    }

    pub async fn new_connect(
        url: &str,
        token: &str,
    ) -> std::result::Result<Self, crate::compat::LibsqlError> {
        let db = turso::sync::Builder::new_remote(":memory:")
            .with_remote_url(url)
            .with_auth_token(token)
            .bootstrap_if_empty(true)
            .build()
            .await?;
        let conn = db.connect().await?;
        Ok(Self {
            _db: DatabaseInner::Sync(db),
            inner: conn,
        })
    }

    pub async fn query(
        &self,
        sql: &str,
        params: Vec<crate::compat::LibsqlValue>,
    ) -> Result<crate::compat::LibsqlRows, crate::compat::LibsqlError> {
        self.keep_alive();
        if params.is_empty() {
            self.inner.query(sql, ()).await
        } else {
            self.inner.query(sql, params).await
        }
    }

    pub async fn execute(
        &self,
        sql: &str,
        params: Vec<crate::compat::LibsqlValue>,
    ) -> Result<u64, crate::compat::LibsqlError> {
        self.keep_alive();
        if params.is_empty() {
            self.inner.execute(sql, ()).await
        } else {
            self.inner.execute(sql, params).await
        }
    }
}

#[cfg(not(feature = "turso"))]
pub struct Database {
    _phantom: std::marker::PhantomData<()>,
}

#[cfg(not(feature = "turso"))]
impl Database {
    pub async fn new_connect(_url: &str, _token: &str) -> Result<Self, crate::error::Error> {
        Ok(Database {
            _phantom: std::marker::PhantomData,
        })
    }

    pub async fn query(
        &self,
        _sql: &str,
        _params: Vec<crate::compat::LibsqlValue>,
    ) -> Result<crate::compat::LibsqlRows, crate::compat::LibsqlError> {
        Ok(crate::compat::LibsqlRows::new(vec![]))
    }

    pub async fn execute(
        &self,
        _sql: &str,
        _params: Vec<crate::compat::LibsqlValue>,
    ) -> Result<u64, crate::compat::LibsqlError> {
        Ok(0)
    }
}

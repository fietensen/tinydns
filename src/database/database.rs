pub struct Database {
    sqlite_pool: sqlx::Pool<sqlx::Sqlite>,
}

impl Database {
    pub async fn init(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db_pool = sqlx::SqlitePool::connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)).await?;
        
        sqlx::migrate!("./migrations").run(&db_pool).await?;
    
        Ok(Database{
            sqlite_pool: db_pool,
        })
    }

    #[allow(unused)] // TODO: this might come in handy if we want to chain "virtual" nameservers (e.g. for blocklists)
    pub async fn init_mem() -> Result<Self, Box<dyn std::error::Error>> {
        let db_pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;

        sqlx::migrate!("./migrations").run(&db_pool).await?;

        Ok(Database{
            sqlite_pool: db_pool,
        })
    }

    pub fn config_dns_tbl(&self) -> String {
        "user_dns_records".to_string()
    }

    pub fn get_pool(&self) -> &sqlx::Pool<sqlx::Sqlite> {
        &self.sqlite_pool
    }
}

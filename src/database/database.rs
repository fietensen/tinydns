use crate::protocol::packet::ResourceRecord;

use super::record_query::RecordQuery;

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

    pub async fn query_record(&self, record_query: &RecordQuery) -> Option<ResourceRecord> {
        Some(record_query._fetch_one(&self.sqlite_pool).await?.serialize())
    }
}
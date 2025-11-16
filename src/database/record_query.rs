use crate::protocol::packet::{RecordType, ResourceRecord};

#[derive(Default)]
pub struct RecordQuery {
    valid: bool,
    domain_name: Option<String>,
    record_type: Option<RecordType>,
    // TODO: add more queryable fields
}

#[derive(sqlx::FromRow)]
pub struct RecordEntity {
    id: u64,
    domain_name: String,
    record_type: u16,
    record_value: Vec<u8>,
    ttl: u32,
    priority: Option<u32>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    is_active: bool
}

impl Default for RecordEntity {
    fn default() -> Self {
        Self {
            id: 0,
            domain_name: String::default(),
            record_type: 0,
            record_value: Vec::default(),
            ttl: 3600,
            priority: None,
            created_at: None,
            updated_at: None,
            is_active: false
        }
    }
}

impl RecordQuery {
    pub async fn _fetch_one(&self, db: &sqlx::Pool<sqlx::Sqlite>, tbl_name: String) -> Option<RecordEntity> {
        if !self.valid {
            return None;
        }

        let mut builder = sqlx::QueryBuilder::new(format!("SELECT * FROM {} WHERE 1=1", tbl_name));
        
        if let Some(domain_name) = self.domain_name() {
            builder.push(" AND domain_name = ").push_bind(domain_name);
        }

        if let Some(record_type) = self.record_type() {
            let record_type: u16 = record_type.into();
            builder.push(" AND record_type = ").push_bind(record_type);
        }

        builder.push(" LIMIT 1");

        log::trace!("querying domain name {} with record type {:?}", self.domain_name()?, self.record_type()?);
        Some(
            builder.build_query_as().fetch_one(db).await.ok()?
        )
    }

    pub fn with_domain_name(mut self, domain_name: String) -> Self {
        self.domain_name = Some(domain_name);
        self.valid = true;
        self
    }

    pub fn with_record_type(mut self, record_type: RecordType) -> Self {
        self.record_type = Some(record_type);
        self.valid = true;
        self
    }

    pub fn domain_name(&self) -> Option<String> {
        Some(self.domain_name.as_ref()?.to_owned())
    }

    pub fn record_type(&self) -> Option<RecordType> {
        Some(self.record_type.as_ref()?.clone())
    }


}

impl RecordEntity {
    pub async fn _insert(self, db: &sqlx::Pool<sqlx::Sqlite>, tbl_name: String) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(&format!("INSERT INTO {}(domain_name, record_type, record_value, ttl, priority) VALUES (?, ?, ?, ?, ?);", tbl_name))
            .bind(self.domain_name)
            .bind(self.record_type)
            .bind(self.record_value)
            .bind(self.ttl)
            .bind(self.priority)
            .execute(db).await?;

        Ok(())
    }

    pub fn serialize(self) -> ResourceRecord {
        // TODO: if the name is not set, the packet will
        //      still successfully serialize (but be malformed)
        ResourceRecord::default()
            .with_rtype(self.record_type.into())
            .with_rdata(self.record_value)
            .with_rclass(1) // TODO: this shouldn't be hardcoded
            .with_ttl(self.ttl)
    }

    pub fn with_domain_name(mut self, domain_name: String) -> Self {
        self.domain_name = domain_name;
        self
    }

    pub fn with_record_type(mut self, record_type: RecordType) -> Self {
        self.record_type = record_type.into();
        self
    }

    pub fn with_record_value(mut self, record_value: Vec<u8>) -> Self {
        self.record_value = record_value;
        self
    }

    pub fn with_ttl(mut self, ttl: u32) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = Some(priority);
        self
    }
}

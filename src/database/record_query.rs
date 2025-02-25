use crate::protocol::packet::{RecordType, ResourceRecord};

#[derive(Default)]
pub struct RecordQuery {
    valid: bool,
    domain_name: Option<String>,
    record_type: Option<RecordType>,
    // TODO: add more queryable fields
}

#[derive(sqlx::FromRow)]
pub struct RecordResult {
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

impl RecordQuery {
    pub async fn _fetch_one(&self, db: &sqlx::Pool<sqlx::Sqlite>) -> Option<RecordResult> {
        if !self.valid {
            return None;
        }

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM user_dns_records WHERE 1=1");
        
        if let Some(domain_name) = self.domain_name() {
            builder.push(" AND domain_name = ").push_bind(domain_name);
        }

        if let Some(record_type) = self.record_type() {
            let record_type: u16 = record_type.into();
            builder.push(" AND record_type = ").push_bind(record_type);
        }

        builder.push(" LIMIT 1");

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

impl RecordResult {
    pub fn serialize(self) -> ResourceRecord {
        // TODO: if the name is not set, the packet will
        //      still successfully serialize (but be malformed)
        ResourceRecord::default()
            .with_rtype(self.record_type.into())
            .with_rdata(self.record_value)
            .with_rclass(1) // TODO: this shouldn't be hardcoded
            .with_ttl(self.ttl)
    }
}
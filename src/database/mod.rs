mod database;
mod record_query;

pub use record_query::RecordQuery;
pub use record_query::RecordEntity;
pub use database::Database;

#[cfg(test)]
mod tests {
    use crate::{database::record_query::RecordEntity, nameserver::Nameserver, protocol::{packet::RecordType, util::{self}}};

    use super::*;

    #[tokio::test]
    async fn test_database() {
        // check if database can be created
        let db = Database::init_mem().await.unwrap();

        let query = RecordQuery::default()
            .with_domain_name("dns.is.tiny".to_string())
            .with_record_type(crate::protocol::packet::RecordType::CNAME);

        let nameserver = Nameserver::new(&db);

        // check for nonexistent resource
        let record = nameserver.query_record(&query).await;
        assert!(record.is_none());

        let encoded_domain_name = util::encode_domain("dns.is.tidy".to_string()).unwrap();
        let record = RecordEntity::default()
            .with_domain_name("dns.is.tiny".to_string())
            .with_record_type(RecordType::CNAME)
            .with_record_value(encoded_domain_name.clone());

        // check for database insert
        assert!(nameserver.insert_record(record).await.is_ok());

        let record = nameserver.query_record(&query).await;
        assert!(record.is_some());

        assert_eq!(record.unwrap().serialize().rdata(), encoded_domain_name);
    }
}

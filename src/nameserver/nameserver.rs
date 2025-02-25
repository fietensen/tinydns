use crate::{database::{Database, RecordEntity, RecordQuery}, protocol::{answer::AnswerEntry, packet::Question}};

pub struct Nameserver<'a> {
    db: &'a Database
}

impl<'a> Nameserver<'a> {
    pub fn new(db: &'a Database) -> Self {
        Nameserver {
            db
        }
    }

    /*
        Tries to answer DNS question locally
    */
    pub async fn try_answer(&self, question: Question) -> Option<AnswerEntry> {
        let query = RecordQuery::default()
            .with_domain_name(question.name())
            .with_record_type(question.qtype());

        // TODO?: Support qclasses other than IN and ANY 
        if question.qclass() != 1 && question.qclass() != 255 {
            return None;
        }

        Some(AnswerEntry{
            authoritive: true,
            resource: Some(
                self.query_record(&query)
                    .await?
                    .serialize()
                    .with_name(question.name())
                ),
            ..Default::default()
        })
    }

    /*
        Query a record
    */
    pub async fn query_record(&self, record_query: &RecordQuery) -> Option<RecordEntity> {
        Some(record_query._fetch_one(self.db.get_pool(), self.db.config_dns_tbl()).await?)
    }

    /*

    */
    pub async fn insert_record(&self, record: RecordEntity) -> Result<(), Box<dyn std::error::Error>> {
        record._insert(self.db.get_pool(), self.db.config_dns_tbl()).await
    }


}

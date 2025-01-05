use super::packet::ResourceRecord;

#[derive(Default)]
pub struct AnswerEntry {
    pub resource: Option<ResourceRecord>,
    pub authoritive: bool,
    pub authority: Option<String>,
    pub additional: Vec<ResourceRecord>,
}

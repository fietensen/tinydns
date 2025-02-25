use super::packet::ResourceRecord;

#[derive(Default)]
pub struct AnswerEntry {
    pub resource: Option<ResourceRecord>,
    pub authoritive: bool,
    pub authority: Option<ResourceRecord>,
    pub additional: Vec<ResourceRecord>,
}

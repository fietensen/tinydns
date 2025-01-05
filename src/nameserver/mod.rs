use crate::{
    protocol::{answer::AnswerEntry, packet::Question},
    server::ServerConfig,
};

/*
    Tries to answer DNS question locally
*/
pub fn try_answer(question: Question, config: &ServerConfig) -> Option<AnswerEntry> {
    None
}

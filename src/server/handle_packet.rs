use crate::protocol::{
    answer::AnswerEntry,
    packet::{
        flags::{Flags, HeaderFlags, ResponseCode},
        Packet, PacketBuilder, Question, ResourceRecord,
    },
    util,
};

use super::ServerConfig;

/*
    Answer single question or return authority for iterative querying
*/
pub async fn answer_question<'a>(question: Question, config: &ServerConfig<'a>) -> AnswerEntry {
    if let Some(nameserver) = config.nameserver() {
        if let Some(answer) = nameserver.try_answer(question.clone()).await {
            return answer;
        }
    }

    config
        .resolver()
        .get_zoneauthority(util::get_upzone(question.name()), config)
        .await
}

/*
    Batch-answer questions. Recursion should be desired.
*/
pub async fn answer_batch<'a>(
    questions: Vec<Question>,
    config: &ServerConfig<'a>,
) -> Vec<AnswerEntry> {
    let mut delegated_questions = Vec::new();
    let mut answers = Vec::new();

    if let Some(nameserver) = config.nameserver() {
        // Resolve all locally answerable questions using our nameserver, delegate the rest
        for question in questions.clone() {
            if let Some(answer) = nameserver.try_answer(question.clone()).await {
                answers.push(answer);
            } else {
                delegated_questions.push(question);
            }
        }
    } else {
        // No nameserver configured, delegate ALL questions
        log::trace!(
            "Resolving {} question:s recursively",
            delegated_questions.len()
        );
        answers.extend(config.resolver().resolve_recursive(questions).await);
        return answers;
    }

    if !delegated_questions.is_empty() {
        answers.extend(
            config
                .resolver()
                .resolve_recursive(delegated_questions)
                .await,
        );
    }

    log::info!("Resolved {} questions", questions.len());
    answers
}

pub async fn handle_packet<'a>(
    packet: Packet,
    config: &ServerConfig<'a>,
) -> Result<Packet, Box<dyn std::error::Error>> {
    let questions = packet.clone().questions;
    let mut authoritive = true;
    let mut authorities: Vec<ResourceRecord> = Vec::new();
    let mut answer_records: Vec<ResourceRecord> = Vec::new();
    let mut additional_records: Vec<ResourceRecord> = Vec::new();
    let recursion_desired =
        HeaderFlags::from(packet.header.flags).1 & (Flags::RD as u16) == (Flags::RD as u16);

    // TODO: RFC 2308, Section 2.2 Compliance: (Case: No Data / Record Entry doesn't exist) -> Return SOA
    log::trace!("Handling {} question:s", questions.len());
    if recursion_desired {
        let answers = answer_batch(questions.clone(), config).await;
        for answer in answers {
            authoritive = authoritive && answer.authoritive;

            // add authorities that can answer the question
            if let Some(aauth) = answer.authority {
                if !authorities.contains(&aauth) {
                    authorities.push(aauth);
                }
            }

            // add additional records useful to the client
            for additional in answer.additional {
                if !additional_records.contains(&additional) {
                    additional_records.push(additional);
                }
            }

            if let Some(record) = answer.resource {
                // TODO: should duplicate answer records also be removed?
                answer_records.push(record);
            }
        }
    } else {
        // answer on per-question basis
        for question in questions.clone() {
            let answer = answer_question(question.clone(), config).await;
            authoritive = authoritive && answer.authoritive;

            // add authorities that can answer the question
            if let Some(aauth) = answer.authority {
                if !authorities.contains(&aauth) {
                    authorities.push(aauth);
                }
            }

            // add additional records useful to the client
            for additional in answer.additional {
                if !additional_records.contains(&additional) {
                    additional_records.push(additional);
                }
            }

            if let Some(record) = answer.resource {
                // TODO: should duplicate answer records also be removed?
                answer_records.push(record);
            }
        }
    }

    if answer_records.len() > 0 || authorities.len() > 0 {
        let header_flags = HeaderFlags::new()
            .with_opcode(HeaderFlags::from(packet.header.flags).0)
            .with_rcode(ResponseCode::NoError)
            .with_flag(Flags::QR)
            .with_flag(Flags::RA);

        Ok(PacketBuilder::new()
            .with_flags(if authoritive {
                header_flags.with_flag(Flags::AA)
            } else {
                header_flags
            })
            .with_id(packet.header.id)
            .with_qentries(questions)
            .with_aentries(answer_records)
            .with_authentries(authorities)
            .with_addentries(additional_records)
            .build())
    } else {
        Err("No questions could be answered.".into())
    }
}

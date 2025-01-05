use crate::protocol::{
    self,
    answer::AnswerEntry,
    packet::{
        flags::{Flags, HeaderFlags, ResponseCode},
        Packet, PacketBuilder, Question, RecordType, ResourceRecord,
    },
};

use super::ServerConfig;

pub async fn answer_question(
    question: Question,
    recursion_desired: bool,
    config: &ServerConfig,
) -> AnswerEntry {
    /* Let NS or Resolver answer question */
    AnswerEntry::default()
}

pub async fn handle_packet(
    packet: Packet,
    config: &ServerConfig,
) -> Result<Packet, Box<dyn std::error::Error>> {
    let questions = packet.clone().questions;
    let mut authoritive = true;
    let mut authorities: Vec<(Question, String)> = Vec::new();
    let mut answer_records: Vec<ResourceRecord> = Vec::new();
    let mut additional_records: Vec<ResourceRecord> = Vec::new();

    // answer on per-question basis
    for question in questions.clone() {
        let answer = answer_question(
            question.clone(),
            HeaderFlags::from(packet.header.flags).1 & (Flags::RD as u16) == (Flags::RD as u16),
            config,
        )
        .await;
        authoritive = authoritive && answer.authoritive;

        // add authorities that can answer the question
        if let Some(aauth) = answer.authority {
            let authst = (question, aauth);
            if !authorities.contains(&authst) {
                authorities.push(authst);
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

    if answer_records.len() > 0 || authorities.len() > 0 {
        return Err("couldn't answer query questions".into());
    } else {
        // assemble answer from parts

        // turn authorities vector to vector containing respective PTR records
        let mut authority_vec: Vec<ResourceRecord> = Vec::new();
        for (auth_origin, auth_ref) in authorities {
            authority_vec.push(
                ResourceRecord::default()
                    .with_name(auth_origin.name())
                    .with_rclass(1)
                    .with_rdata(protocol::util::encode_domain(auth_ref)?)
                    // TODO: make this adaptable to corresponding NS (default TTL 3600s [1h])
                    .with_ttl(3600)
                    .with_rtype(RecordType::NS),
            )
        }

        let header_flags = HeaderFlags::new()
            .with_opcode(HeaderFlags::from(packet.header.flags).0)
            .with_rcode(ResponseCode::ServerFailure)
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
            .with_authentries(authority_vec)
            .with_addentries(additional_records)
            .build())
    }
}

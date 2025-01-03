use protocol::packet::flags::{Flags, HeaderFlags, OpCode};
use protocol::packet::{Packet, PacketBuilder, Question};

mod protocol;

fn main() {
    /*let packet_data = vec![
        0, 4, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0,
        1, 0, 1,
    ];

    let packet = Packet::deserialize(&packet_data).unwrap();

    println!("{:?}", packet);
    */

    let packet_flags = HeaderFlags::new()
        .with_opcode(OpCode::Query)
        .with_flag(Flags::RD);

    let qentry = Question::default()
        .with_name("google.com".into())
        .with_qtype(1)
        .with_qclass(1);

    let mut packet = PacketBuilder::new()
        .with_id(4)
        .with_flags(packet_flags)
        .with_qentry(qentry)
        .build();

    let serialized = packet.serialize().unwrap();

    println!("{:?}", serialized);

    let deserialized = Packet::deserialize(&serialized).unwrap();

    println!("{:?}", deserialized);
}

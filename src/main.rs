mod protocol;

fn main() {
    let pkt = protocol::PacketBuilder::new().build().serialize().unwrap();
    println!("{:?}", pkt);
}

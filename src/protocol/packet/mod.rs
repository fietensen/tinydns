mod builder;
pub mod flags;
mod header;
mod packet;
mod question;
mod record_type;
mod resource_record;

pub use builder::PacketBuilder;
pub use packet::Packet;
pub use question::Question;

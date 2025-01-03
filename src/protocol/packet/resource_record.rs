#[derive(Default)]
pub struct ResourceRecord {}

impl ResourceRecord {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }
}

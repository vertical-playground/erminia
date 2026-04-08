pub struct InitializeResponse {}

pub enum Response {
    Initialize(InitializeResponse),
}

impl Response {
    pub fn send(&self) -> std::io::Result<()> {
        Ok(())
    }
}

use crate::error::Result;

pub struct Source(pub String);

impl super::Source for Source {
    fn convert(&self) -> Result<String> {
        Ok(self.0.clone())
    }
}

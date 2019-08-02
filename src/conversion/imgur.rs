use crate::error::Result;
use std::path::Path;

pub struct Source(pub String);

impl super::Source for Source {
    fn convert(&self) -> Result<String> {
        let link = Path::new(&self.0).with_extension("mp4");

        let source_url = link.to_string_lossy().to_string();

        super::convert(source_url.to_string())
    }
}

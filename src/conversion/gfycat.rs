use reqwest;
use serde_json::Value;

use std::path::Path;

use crate::error::{Error, Result};

pub struct Source(pub String);

impl super::Source for Source {
    fn convert(&self) -> Result<String> {
        let file_handle = Path::new(&self.0)
            .file_stem()
            .ok_or(Error::GfycatIncorrectUrl)?
            .to_string_lossy();

        let url =
            format!("{}{}", dotenv!("GFYCAT_POST_ENDPOINT"), file_handle);

        println!("gfycat url is {}", url);

        let post: Value = reqwest::get(&url)
            .map_err(Error::GfycatPost)?
            .json()
            .map_err(Error::GfycatPost)?;

        let source_url = dig_video_url(&post)
            .ok_or_else(|| Error::GfycatPostParse(self.0.clone()))?;

        super::convert(source_url.to_owned())
    }
}

fn dig_video_url(value: &Value) -> Option<&str> {
    value["gfyItem"]["mp4Url"].as_str()
}

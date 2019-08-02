use reqwest;
use serde_json::Value;

use crate::error::{Error, Result};

pub struct Source(pub String);

impl super::Source for Source {
    fn convert(&self) -> Result<String> {
        let post: Value = reqwest::get(&format!("{}.json", self.0))
            .map_err(Error::RedditPost)?
            .json()
            .map_err(Error::RedditPost)?;

        let source_url = dig_video_url(&post)
            .ok_or_else(|| Error::RedditPostParse(self.0.clone()))?;

        super::convert(source_url.to_owned())
    }
}

fn dig_video_url(value: &Value) -> Option<&str> {
    value[0]
        ["data"]
        ["children"]
        [0]
        ["data"]
        ["secure_media"]
        ["reddit_video"]
        ["fallback_url"].as_str()
}

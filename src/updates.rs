use reqwest;
use serde::{Deserialize, Deserializer};

use crate::error::{Error, Result};

pub type Update = (Comment, Post);

#[derive(Deserialize)]
struct Response<T> {
    data: Vec<T>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Comment {
    pub link_id: String,
    pub id: String,
    pub body: String,
    #[serde(rename = "created_utc")]
    pub timestamp: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Post {
    pub url: String,
    pub domain: String,
    pub is_video: bool,
    pub full_link: String,
    #[serde(rename = "id", deserialize_with = "prepend_link_type")]
    pub link_id: String,
    pub author: String,
}

pub fn get_updates(time: i32) -> Result<Vec<Update>> {
    let trigger = dotenv!("TRIGGER_STRING");

    let comments = fetch_comments(time, trigger)
        .and_then(|updates| filter_comments(trigger, updates))?;

    let posts = fetch_posts(&comments)?;

    let result = comments
        .into_iter()
        .map(|comment| {
            let post = posts.iter()
                .find(|post| comment.link_id == post.link_id)
                .unwrap();

            (comment, post.clone())
        })
        .collect();

    Ok(result)
}

fn fetch_comments(time: i32, trigger: &str) -> Result<Vec<Comment>> {
    let client = reqwest::Client::new();

    let ref params = [
        ("q", trigger),
        ("after", &time.to_string()),
    ];

    let response: Response<Comment> = client.get(dotenv!("UPDATES_URL"))
        .query(params)
        .send()
        .map_err(Error::PushshiftUpdates)?
        .json()
        .map_err(Error::PushshiftParse)?;

    Ok(response.data)
}

fn filter_comments(trigger: &str, updates: Vec<Comment>)
                 -> Result<Vec<Comment>> {
    let result = updates.into_iter()
        .filter(|x| x.body == trigger)
        .collect();

    Ok(result)
}

fn fetch_posts(comments: &Vec<Comment>) -> Result<Vec<Post>> {
    let client = reqwest::Client::new();

    let post_ids: Vec<&str> = comments.iter()
        .map(|x| &x.link_id[3..])
        .collect();

    // Nay, I don't want reqwest URL-encode the comma.
    let url = format!("{}?ids={}", dotenv!("POSTS_URL"), post_ids.join(","));

    let response: Response<Post> = client.get(&url)
        .send()
        .map_err(Error::PushshiftUpdates)?
        .json()
        .map_err(Error::PushshiftParse)?;

    Ok(response.data)
}


fn prepend_link_type<'de, D>(deserializer: D)
                             -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut link_type = "t3_".to_owned();
    let string = String::deserialize(deserializer)?;

    link_type.push_str(&string);

    Ok(link_type)
}

use reqwest::{self, header};
use serde::Deserialize;
use serde_json::Value;

use crate::error::{Error, Result};

pub struct RedditApi {
    access_token: Option<String>,
    client: reqwest::Client,
}

#[derive(Deserialize)]
struct RedditAuthResponse {
    access_token: String,
}

impl RedditApi {
    pub fn new() -> Self {
        Self {
            access_token: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn comment(&mut self,
                   link: &String,
                   comment_id: &String,
                   author_id: &String) -> Result<()> {
        let mut count = 0;
        let mut pm_author = false;
        loop {
            count += 1;

            if count == 10 {
                return Err(Error::RedditRetry);
            }

            if let Some(access_token) = &self.access_token {
                let result = if pm_author {
                    self.do_message(
                        access_token,
                        &format!("{} {}", dotenv!("PM_TEXT"), link),
                        author_id
                    )
                } else {
                    self.do_comment(
                        access_token,
                        &format!("{} {}", dotenv!("COMMENT_TEXT"), link),
                        &format!("t1_{}", comment_id)
                    )
                };

                match result {
                    Err(Error::RedditUnauthorized) => {
                        self.access_token = None;

                        continue;
                    },
                    Err(Error::RedditPossibleBan) => {
                        pm_author = true;
                    },
                    res => break res
                }
            }

            self.renew_token()?;
        }
    }

    fn do_comment(&self,
                  access_token: &String,
                  text: &String,
                  thing_id: &String) -> Result<()> {
        let ref params = [
            ("api_type", "json"),
            ("text", text),
            ("thing_id", thing_id)
        ];

        self.do_request(
            access_token,
            dotenv!("REDDIT_COMMENT_URL"),
            params
        )
    }

    fn do_message(&self,
                  access_token: &String,
                  text: &String,
                  thing_id: &String) -> Result<()> {
        let ref params = [
            ("api_type", "json"),
            ("subject", dotenv!("PM_SUBJECT")),
            ("text", text),
            ("to", thing_id)
        ];

        self.do_request(
            access_token,
            dotenv!("REDDIT_PM_URL"),
            params
        )
    }

    fn do_request(&self,
                  access_token: &String,
                  url: &str,
                  params: &[(&str, &str)]) -> Result<()> {
        let auth_header = format!("Bearer {}", access_token);

        let response: Value =
            self.client.post(url)
            .header(header::AUTHORIZATION, auth_header)
            .header(header::USER_AGENT, dotenv!("BOT_USER_AGENT"))
            .form(params)
            .send()
            .map_err(comment_error)?
            .json()
            .map_err(comment_error)?;

        match response["json"]["errors"].as_array() {
            Some(vec) => {
                if vec.is_empty() {
                    Ok(())
                } else {
                    Err(Error::RedditPossibleBan)
                }
            },
            _ => Err(Error::RedditPossibleBan)
        }
    }

    fn renew_token(&mut self) -> Result<()> {
        let auth_header = format!("Basic {}", dotenv!("REDDIT_AUTH_TOKEN"));

        let ref params = [
            ("grant_type", "password"),
            ("username", dotenv!("REDDIT_USERNAME")),
            ("password", dotenv!("REDDIT_PASSWORD")),
        ];

        let res: RedditAuthResponse = self.client
            .post(dotenv!("REDDIT_AUTH_URL"))
            .header(header::AUTHORIZATION, auth_header)
            .form(params)
            .send()
            .map_err(Error::RedditToken)?
            .json()
            .map_err(Error::RedditToken)?;

        self.access_token = Some(res.access_token);

        Ok(())
    }
}

fn comment_error(err: reqwest::Error) -> Error {
    if let Some(code) = err.status() {
        if code == 401 {
            return Error::RedditUnauthorized;
        }
    }
   Error::RedditComment
}

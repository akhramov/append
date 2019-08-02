#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate diesel;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod conversion;
mod error;
mod ffmpeg;
mod file;
mod persistence;
mod publish;
mod reddit;
mod schema;
mod updates;

fn main() {
    let mut reddit = reddit::RedditApi::new();

    let posts = persistence::Posts::new().unwrap();

    let last_update = posts.last()
        .map(|x| x.timestamp)
        .unwrap_or(0);

    let updates = updates::get_updates(last_update).unwrap();

    updates.into_iter()
        .for_each(|update| {
            let (comment, post) = update.clone();
            let maybe_url =
                conversion::infer_source(&posts, update).convert();

            match maybe_url {
                Ok(url) => {
                    reddit
                        .comment(&url, &comment.id, &post.author)
                        .unwrap();

                    posts
                        .upsert(post.link_id, url, comment.timestamp)
                        .unwrap();
                },

                Err(err) => {
                    println!("Unsupported source {}", err);
                }
            }
        });
}

use crate::error::{Error, Result};
use crate::ffmpeg;
use crate::file::File;
use crate::persistence::Posts;
use crate::publish;
use crate::updates::Update;

mod cache;
mod gfycat;
mod imgur;
mod reddit;

pub trait Source {
    fn convert(&self) -> Result<String>;
}

pub struct Unsupported(String);

impl Source for Unsupported {
    fn convert(&self) -> Result<String> {
        Err(Error::UnsupportedSource(self.0.clone()))
    }
}

pub fn infer_source(posts: &Posts, update: Update) -> Box<dyn Source> {
    let (comment, post) = update;
    let url = post.url;

    if let Some(cached_post) = posts.find(comment.link_id) {
        return Box::new(cache::Source(cached_post.video_link));
    }

    if post.is_video {
        return Box::new(reddit::Source(post.full_link));
    }

    match &post.domain[..] {
        "gfycat.com" => return Box::new(gfycat::Source(url)),
        "i.imgur.com" => return Box::new(imgur::Source(url)),
        _ => return Box::new(Unsupported(url)),
    }
}

fn convert(path: String) -> Result<String> {
    let file = download_file(&path)?;
    let file_path = file.path();

    let prepender = ffmpeg::Concat::new(dotenv!("VIDEO_TO_APPEND"));
    let video = prepender.call(file_path)?;

    publish::publish(video)
}

fn download_file(path: &str) -> Result<File> {
    let mut file = File::new()?;

    reqwest::get(path)
        .map_err(|_| Error::FileDownload(path.to_owned()))?
        .copy_to(&mut file)
        .map_err(|_| Error::FileSave(path.to_owned()))?;

    Ok(file)
}

#[cfg(test)]
mod tests {
    use crate::persistence::Posts;
    use crate::updates::{Comment, Post};
    use super::{infer_source, Error};

    #[test]
    fn test_unsupported_source() {
        let posts = Posts::new().unwrap();

        let (comment, post) = generate_update();

        let post_url = post.url.clone();
        let ref source = infer_source(&posts, (comment, post));
        let result = source.convert().err();

        if let Some(Error::UnsupportedSource(desc)) = result {
            assert_eq!(desc, post_url);
        } else {
            panic!("Test failed");
        }
    }

    #[test]
    fn test_cached_source() {
        let posts = Posts::new().unwrap();

        let (comment, post) = generate_update();
        posts.upsert("test_link_id", "some_url", 42);

        let ref source = infer_source(&posts, (comment, post));
        let result = source.convert().ok();

        if let Some(url) = result {
            assert_eq!(url, "some_url");
        } else {
            panic!("Test failed");
        }
    }

    fn generate_update() -> (Comment, Post) {
        let comment = Comment {
            link_id: "test_link_id".to_owned(),
            id: "Foobar".to_owned(),
            body: "Foobar".to_owned(),
            timestamp: 42424242,
        };

        let post = Post {
            url: "url.com".to_owned(),
            domain: "domain.bar".to_owned(),
            is_video: false,
            full_link: "https://unsupported".to_owned(),
            link_id: "https://unsupported".to_owned(),
            author: "why_the_lucky_stiff".to_owned(),
        };

        (comment, post)
    }
}

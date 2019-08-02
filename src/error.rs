use std::io::Error as IoError;

use diesel::ConnectionError as DieselConnectionError;
use diesel::result::Error as DieselInsertError;
use failure::Fail;
use reqwest::Error as ReqwestError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Failed to download file {}", _0)]
    FileDownload(String),
    #[fail(display = "Failed to save file {}", _0)]
    FileSave(String),
    #[fail(display = "Unsupported source {}", _0)]
    UnsupportedSource(String),
    #[fail(display = "[Reddit] Unable to get a post {:?}", _0)]
    RedditPost(ReqwestError),
    #[fail(display = "[Reddit] Unable to parse server response ({})", _0)]
    RedditPostParse(String),
    #[fail(display = "[Reddit] Unable to get auth token ({:?})", _0)]
    RedditToken(ReqwestError),
    #[fail(display = "[Reddit] Failed to reply. Giving up...")]
    RedditRetry,
    #[fail(display = "[Reddit] Unauthorized! Token expired?")]
    RedditUnauthorized,
    #[fail(display = "[Reddit] Comment error.")]
    RedditComment,
    #[fail(display = "[Reddit] Possibly banned.")]
    RedditPossibleBan,
    #[fail(display = "[Imgur] Unable to upload video {:?}", _0)]
    ImgurUpload(ReqwestError),
    #[fail(display = "[Imgur] Unable to parse server response ({:?})", _0)]
    ImgurUploadParse(ReqwestError),
    #[fail(display = "[ffmpeg] Error during file conversion occured")]
    Ffmpeg,
    #[fail(display = "IO error occured during file conversion: {}", _0)]
    Io(#[fail(cause)] IoError),
    #[fail(display = "[DB] Failed to establish connection: {:?}", _0)]
    DbConnection(DieselConnectionError),
    #[fail(display = "[DB] Failed to insert a row: {:?}", _0)]
    DbInsert(DieselInsertError),
    #[fail(display = "[Pushshift] Unable to get updates ({:?})", _0)]
    PushshiftUpdates(ReqwestError),
    #[fail(display = "[Pushshift] Unable to parse server response ({:?})", _0)]
    PushshiftParse(ReqwestError),
    #[fail(display = "[Gfycat] Unable to get a post {:?}", _0)]
    GfycatPost(ReqwestError),
    #[fail(display = "[Gfycat] Unable to parse server response ({})", _0)]
    GfycatPostParse(String),
    #[fail(display = "[Gfycat] Unable to stem Gfycat URL")]
    GfycatIncorrectUrl,
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

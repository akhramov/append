use std::io::Read;

use crate::error::{Error, Result};

use reqwest::{self, header, multipart};
use serde::{Deserialize};

#[derive(Deserialize)]
struct ImgurUploadResponse {
    data: ImgurUploadResponseData,
}

#[derive(Deserialize)]
struct ImgurUploadResponseData {
    link: String,
}

pub fn publish<T: Read + Send + 'static>(video: T) -> Result<String> {
    let client = reqwest::Client::new();
    let auth_header = format!("Client-ID {}", dotenv!("IMGUR_CLIENT_ID"));

    let res: ImgurUploadResponse = client.post(dotenv!("IMGUR_UPLOAD_URL"))
        .header(header::AUTHORIZATION, auth_header)
        .multipart(formdata(video))
        .send()
        .map_err(Error::ImgurUpload)?
        .json()
        .map_err(Error::ImgurUploadParse)?;

    Ok(res.data.link)
}

fn formdata<T: Read + Send  + 'static>(data: T) -> multipart::Form {
    let part = multipart::Part::reader(data).file_name("image");

    multipart::Form::new().part("video", part)
}

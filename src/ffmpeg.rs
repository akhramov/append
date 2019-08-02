use std::process::{Command, Child, Stdio};
use std::io::{Read, Write};

use crate::error::{Error, Result};

pub struct Concat<'a> {
    second_video: &'a str,
}

impl<'a> Concat<'a> {
    pub fn new(second_video: &'a str) -> Self {
        Self { second_video }
    }

    pub fn call<T: AsRef<str>>(&self, first_video: T) -> Result<impl Read> {
        let input = format!("file '{}'\n\
                             file '{}'",
                            first_video.as_ref(),
                            self.second_video);

        let mut command = spawn_ffmpeg()?;

        let stdin = command.stdin.as_mut()
            .ok_or(Error::Ffmpeg)?;
        stdin.write_all(input.as_bytes())?;

        command.stdout.ok_or(Error::Ffmpeg)
    }
}

fn spawn_ffmpeg() -> Result<Child> {
    Command::new(dotenv!("FFMPEG_BINARY"))
        .args(&["-protocol_whitelist", "file,pipe",
                "-safe", "0",
                "-f", "concat",
                "-i", "-",
                "-c", "copy",
                "-movflags", "frag_keyframe",
                "-f", "mp4",
                "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(Error::from)
}

use std::fs;
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

pub struct File {
    pub path: PathBuf,
    file: fs::File,
}

impl File {
    pub fn new() -> Result<Self> {
        let downloads_dir = dotenv!("DOWNLOADS_DIRECTORY");

        fs::create_dir_all(downloads_dir)?;

        let path = Path::new(downloads_dir)
            .join("downloaded-file");

        let file = fs::File::create(&path)?;

        Ok(Self { path, file })
    }

    pub fn path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

// impl Drop for File {
//     fn drop(&mut self) {
//         fs::remove_file(&*self.path);
//     }
// }

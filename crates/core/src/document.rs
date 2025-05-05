use std::path::PathBuf;

pub struct Document {
    pub path: PathBuf,
    pub content: String,
    pub hash: Option<String>,
}

impl Document {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self {
            path,
            content,
            hash: None,
        }
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = Some(hash);
    }

    pub fn path_str(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

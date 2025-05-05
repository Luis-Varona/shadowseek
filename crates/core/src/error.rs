pub enum CoreError {
    CrawlFailed {
        path: String,
        source: std::io::Error,
    },
    ExtractionFailed {
        path: String,
        source: String,
    },
    HashingFailed(String),
    IndexFailed(String),
    Other(String),
}

pub type CoreResult<T> = Result<T, CoreError>;

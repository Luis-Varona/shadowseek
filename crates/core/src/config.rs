use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

const DEFAULT_CRAWL_ROOT: &str = ".";
const DEFAULT_RECURSIVE: bool = true;
const DEFAULT_SIMILARITY_THRESHOLD: f64 = 0.85;

pub enum FileType {
    FileTypeText(FileTypeText),
    FileTypeOffice(FileTypeOffice),
    FileTypeWeb(FileTypeWeb),
    Pdf,
}

// TODO: Add support for .md files, maybe? Seems hard to find a good crate, though...
pub enum FileTypeText {
    Txt,  // Using `textract`
    Rtf,  // Using `rtf-parser`
    Epub, // Using `epub`
}

pub enum FileTypeOffice {
    Docx, // Using `textract`
    Odt,  // Using `textract`
}

pub enum FileTypeWeb {
    Html, // Using `textract`
    Htm,  // Using `textract`
}

impl FileType {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "txt" => Some(FileType::FileTypeText(FileTypeText::Txt)),
            "rtf" => Some(FileType::FileTypeText(FileTypeText::Rtf)),
            "epub" => Some(FileType::FileTypeText(FileTypeText::Epub)),
            "docx" => Some(FileType::FileTypeOffice(FileTypeOffice::Docx)),
            "odt" => Some(FileType::FileTypeOffice(FileTypeOffice::Odt)),
            "html" => Some(FileType::FileTypeWeb(FileTypeWeb::Html)),
            "htm" => Some(FileType::FileTypeWeb(FileTypeWeb::Htm)),
            "pdf" => Some(FileType::Pdf),
            _ => None,
        }
    }
}

pub struct Config {
    pub crawl_root: PathBuf,
    pub recursive: bool,
    pub similarity_threshold: f64,
    pub file_extensions: Option<Vec<FileType>>,
    pub max_file_size: Option<u64>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            crawl_root: PathBuf::from(DEFAULT_CRAWL_ROOT),
            recursive: DEFAULT_RECURSIVE,
            similarity_threshold: DEFAULT_SIMILARITY_THRESHOLD,
            file_extensions: None,
            max_file_size: None,
        }
    }

    // TODO: Implement (part of) argument parsing logic?
}

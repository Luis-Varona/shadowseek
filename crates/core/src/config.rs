// Copyright 2025 Luis M. B. Varona
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

const DEFAULT_ROOT_DIR: &str = ".";
const DEFAULT_RECURSIVE: bool = true;
const DEFAULT_SIM_THRESHOLD: f64 = 0.85;
const DEFAULT_FILE_EXTENSIONS: &[&str] =
    &["txt", "rtf", "epub", "docx", "odt", "html", "htm", "pdf"];

// TODO: Add support for .md files, maybe? Seems hard to find a good crate, though...
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FileType {
    Txt,  // Using `textract`
    Rtf,  // Using `rtf-parser`
    Epub, // Using `epub`
    Docx, // Using `textract`
    Odt,  // Using `textract`
    Html, // Using `textract`
    Htm,  // Using `textract`
    Pdf,  // Using `textract`
}

impl FileType {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "txt" => Some(FileType::Txt),
            "rtf" => Some(FileType::Rtf),
            "epub" => Some(FileType::Epub),
            "docx" => Some(FileType::Docx),
            "odt" => Some(FileType::Odt),
            "html" => Some(FileType::Html),
            "htm" => Some(FileType::Htm),
            "pdf" => Some(FileType::Pdf),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "shadowseek",
    version = "0.1.0-dev",
    author = "Luis M. B. Varona",
    about = "Fast, lightweight near-duplicate detection tool for text files",
    long_about = r#"ShadowSeek is a CLI tool for near-duplicate detection in text files.
Written in native Rust, it offers fast execution and low memory overhead,
with no dependencies on external runtime environments."#
)]
pub struct Config {
    #[arg(short = 'r', long, default_value = DEFAULT_ROOT_DIR, help = "Root directory to crawl")]
    pub root_dir: PathBuf,
    #[arg(short = 'R', long, default_value_t = DEFAULT_RECURSIVE, help = "Crawl recursively")]
    pub recursive: bool,
    #[arg(
        short = 's',
        long,
        default_value_t = DEFAULT_SIM_THRESHOLD,
        help = "Jaccard similarity threshold (0.0 - 1.0)",
    )]
    pub sim_threshold: f64,
    #[arg(short = 'e',
        long,
        value_enum,
        default_values = DEFAULT_FILE_EXTENSIONS,
        value_delimiter = ',',
        help = "File extensions to include (comma-separated)"
    )]
    pub file_exts: Vec<FileType>,
    #[arg(short = 'm', long, help = "Maximum file size (in bytes)")]
    pub max_filesize: Option<u64>,
}

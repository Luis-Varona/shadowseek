// Copyright 2025 Luis M. B. Varona
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{CoreError, CoreResult};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn crawl_dir(root: &Path, recursive: bool) -> CoreResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = read_dir(root).map_err(|e| CoreError::CrawlFailed {
        path: root.to_string_lossy().to_string(),
        source: e,
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| CoreError::CrawlFailed {
            path: root.to_string_lossy().to_string(),
            source: e,
        })?;
        let path = entry.path();

        match entry.file_type() {
            Ok(file_type) if file_type.is_file() => {
                files.push(path);
            }
            Ok(file_type) if file_type.is_dir() && recursive => {
                let nested_files = crawl_dir(&path, recursive)?;
                files.extend(nested_files);
            }
            Ok(_) => {
                continue;
            }
            Err(e) => {
                return Err(CoreError::CrawlFailed {
                    path: path.to_string_lossy().to_string(),
                    source: e,
                });
            }
        }
    }

    Ok(files)
}

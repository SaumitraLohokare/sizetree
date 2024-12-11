use std::{fs, path::Path};

use crate::errors::Result;

const KB: u64 = 1000;
const MB: u64 = KB * 1000;
const GB: u64 = MB * 1000;

pub fn file_size(path: &Path) -> Result<u64> {
    Ok(fs::metadata(path)?.len())
}

pub fn human_readable(size: u64) -> String {
    if size < KB {
        format!("{size} B")
    } else if size >= KB && size < MB {
        let kb = (size / KB) as f64;
        let b = (size % KB) as f64 / KB as f64;
        format!("{:.02} KB", kb + b)
    } else if size >= MB && size < GB {
        let mb = (size / MB) as f64;
        let kb = (size % MB) as f64 / MB as f64;
        format!("{:.02} MB", mb + kb)
    } else {
        let gb = (size / GB) as f64;
        let mb = (size % GB) as f64 / GB as f64;
        format!("{:.02} GB", gb + mb)
    }
}
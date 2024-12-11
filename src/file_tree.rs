use std::{fs, path::PathBuf};

use crate::errors::Result;

pub struct FileTree {
    pub root: FileTreeNode,
}

impl FileTree {
    pub fn new(root: FileTreeNode) -> Self {
        Self { root }
    }
}


pub struct FileTreeNode {
    path: PathBuf,
    size: u64,
    children: Vec<FileTreeNode>,
}

impl FileTreeNode {
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self {
            path,
            size,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: FileTreeNode) {
        self.children.push(child);
    }

    pub fn update_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn absolute_path(&self) -> Result<String> {
        // Remove unnecessary Windows path prefix for better display

        // NOTE: Linux does allow you to name a file with `\\?\`...
        // But I doubt any sane person person will use that as a file name...
        // So, I dont really care
        let path = fs::canonicalize(&self.path)?.display().to_string().replace("\\\\?\\", "");

        Ok(path)
    }

    pub fn file_name(&self) -> Option<String> {
        Some(self.path.file_name()?.to_str()?.to_string())
    }
}

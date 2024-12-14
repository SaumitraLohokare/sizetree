use std::{
    fs,
    path::{Path, PathBuf},
};

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
    name: Option<String>,
    size: Option<u64>,
    children: Vec<FileTreeNode>,
    expanded: bool,
}

impl FileTreeNode {
    pub fn new(path: PathBuf, size: Option<u64>) -> Self {
        Self {
            name: Self::file_name(&path),
            size,
            children: vec![],
            expanded: false,
        }
    }

    pub fn expand(&mut self) {
        self.expanded = true;
    }

    pub fn collapse(&mut self) {
        self.expanded = false;
    }

    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    pub fn add_child(&mut self, child: FileTreeNode) {
        self.children.push(child);
    }

    pub fn get_children(&self) -> &Vec<FileTreeNode> {
        &self.children
    }

    pub fn update_size(&mut self, size: u64) {
        self.size = Some(size);
    }

    pub fn name(&self) -> &str {
        match &self.name {
            Some(n) => n,
            None => "Could not find file name",
        }
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn absolute_path(path: &Path) -> Result<String> {
        // Remove unnecessary Windows path prefix for better display

        // NOTE: Linux does allow you to name a file with `\\?\`...
        // But I doubt any sane person person will use that as a file name...
        // So, I dont really care
        let path = fs::canonicalize(path)?
            .display()
            .to_string()
            .replace("\\\\?\\", "");

        Ok(path)
    }

    fn file_name(path: &Path) -> Option<String> {
        let name = match path.file_name() {
            Some(f) => f.to_str()?.to_string(),
            None => Self::absolute_path(path)
                .map_or("Could not get file name or path.".to_string(), |s| s),
        };

        Some(name)
    }
}

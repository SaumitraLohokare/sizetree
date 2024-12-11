use std::{env::args, fs, path::Path};

use errors::{Result, SizeTreeError::*};
use file_tree::{FileTree, FileTreeNode};
use size_utils::{file_size, human_readable};

pub mod errors;
mod file_tree;
mod size_utils;

fn main() -> Result<()> {
    let arg_path = args().skip(1).next();

    let tree = if let Some(arg_path) = arg_path {
        println!("User entered: {arg_path}");

        collect_paths(Path::new(&arg_path))?
    } else {
        collect_paths(Path::new("."))?
    };

    println!("Path: {}", tree.root.absolute_path()?);
    println!(
        "Total size: {}",
        if let Some(size) = tree.root.size() {
            human_readable(size)
        } else {
            "?".to_string()
        }
    );

    Ok(())
}

fn collect_paths(base: &Path) -> Result<FileTree> {
    if !base.exists() {
        return Err(PathDoesNotExist);
    };

    let root = match base {
        x if x.is_file() => FileTreeNode::new(x.to_path_buf(), Some(file_size(x)?)),
        x if x.is_dir() => collect_dir(x)?,
        _ => return Err(UnsuportedFileType),
    };

    Ok(FileTree::new(root))
}

fn collect_dir(dir: &Path) -> Result<FileTreeNode> {
    let entries = fs::read_dir(dir)?;

    let mut node = FileTreeNode::new(dir.to_path_buf(), None);

    let mut size = 0;

    for entry in entries {
        match entry?.path() {
            x if x.is_file() => {
                // Ignore files for which we have no permission
                if let Ok(file_size) = file_size(&x) {
                    size += file_size;
                    node.add_child(FileTreeNode::new(x, Some(file_size)));
                } else {
                    node.add_child(FileTreeNode::new(x, None));
                }
            }
            x if x.is_dir() => {
                // Ignore folders for which we have no permissions
                if let Ok(dir_node) = collect_dir(&x) {
                    size += match dir_node.size() {
                        Some(dir_size) => dir_size,
                        None => 0,
                    };
                    node.add_child(dir_node);
                } else {
                    node.add_child(FileTreeNode::new(x, None));
                }
            }
            _ => (), // Skipping all other file types
        }
    }

    node.update_size(size);

    Ok(node)
}

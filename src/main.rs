use std::{env::args, fs, path::Path};

use crossterm::event::{read, Event::{Key, Resize}, KeyCode, KeyEvent};
use errors::{Result, SizeTreeError::*};
use file_tree::{FileTree, FileTreeNode};
use size_utils::{file_size, human_readable};
use tui::Tui;

pub mod errors;
mod file_tree;
mod size_utils;
mod tui;

// TODO: Cursor

fn main() -> Result<()> {
    let arg_path = args().skip(1).next();

    let mut tree = if let Some(arg_path) = arg_path {
        println!("User entered: {arg_path}");

        collect_paths(Path::new(&arg_path))?
    } else {
        collect_paths(Path::new("."))?
    };

    let mut tui = Tui::new()?;

    tree.root.expand();
    
    loop {
        tui.clear();

        tui.print("sizetree", 2, 0);

        print_tree(&mut tui, &tree);
        
        tui.flush()?;

        let event = read()?;
        
        match event {
            Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), _) => break,
                (KeyCode::Esc, _) => break,
                
                _ => (),
            },

            Resize(w, h) => tui.resize(w as usize, h as usize),
            
            _ => (),
        }
    }

    Ok(())
}

fn print_tree(tui: &mut Tui, tree: &FileTree) {
    for (y, line) in get_tree_node_lines(&tree.root, tui.width, 0).iter().enumerate() {
        tui.print(&line, PADDING, y + PADDING);
    }
}

const PADDING: usize = 2;

fn get_tree_node_main_line(node: &FileTreeNode, width: usize, padding: usize) -> String {
    let name = node.name();

    let size = match node.size() {
        Some(s) => &human_readable(s),
        None => "?",
    };

    let spaces = width - padding - PADDING * 2 - name.len() - size.len();
    let spaces = " ".repeat(spaces);

    let padding = " ".repeat(padding);

    format!("{padding}{name}{spaces}{size}")
}

fn get_tree_node_lines(node: &FileTreeNode, width: usize, padding: usize) -> Vec<String> {
    let mut lines = vec![get_tree_node_main_line(node, width, padding)];

    if node.is_expanded() {
        for child_node in node.get_children() {
            lines.extend(get_tree_node_lines(child_node, width, padding + PADDING));
        }
    }
    
    lines
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

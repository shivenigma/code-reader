use anyhow::Result;
use egui::{CollapsingHeader, Ui};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::editor::EditorState;

#[derive(Debug, Clone)]
pub enum FileNode {
    File {
        name: String,
        path: PathBuf,
    },
    Directory {
        name: String,
        path: PathBuf,
        children: Vec<FileNode>,
        is_expanded: bool,
    },
}

impl FileNode {
    fn from_path(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| {
                path.to_string_lossy().to_string()
            });
            
        if path.is_dir() {
            let mut children = Vec::new();
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let child_path = entry.path();
                
                // Skip hidden files
                if let Some(file_name) = child_path.file_name() {
                    if file_name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }
                
                children.push(FileNode::from_path(&child_path)?);
            }
            
            // Sort children: directories first, then files, both alphabetically
            children.sort_by(|a, b| {
                match (a, b) {
                    (FileNode::Directory { .. }, FileNode::File { .. }) => std::cmp::Ordering::Less,
                    (FileNode::File { .. }, FileNode::Directory { .. }) => std::cmp::Ordering::Greater,
                    (FileNode::Directory { name: name_a, .. }, FileNode::Directory { name: name_b, .. }) => {
                        name_a.cmp(name_b)
                    },
                    (FileNode::File { name: name_a, .. }, FileNode::File { name: name_b, .. }) => {
                        name_a.cmp(name_b)
                    },
                }
            });
            
            Ok(FileNode::Directory {
                name,
                path: path.to_path_buf(),
                children,
                is_expanded: false,
            })
        } else {
            Ok(FileNode::File {
                name,
                path: path.to_path_buf(),
            })
        }
    }
    
    fn render(&mut self, ui: &mut Ui, editor_state: &mut EditorState) {
        match self {
            FileNode::File { name, path } => {
                let response = ui.selectable_label(
                    editor_state.is_file_open(path),
                    format!("📄 {}", name),
                );
                
                if response.clicked() {
                    if let Err(e) = editor_state.open_file(path.clone()) {
                        log::error!("Failed to open file: {:?}", e);
                    }
                }
            },
            FileNode::Directory { name, path, children, is_expanded } => {
                let id = ui.make_persistent_id(path);
                CollapsingHeader::new(format!("📁 {}", name))
                    .id_source(id)
                    .default_open(*is_expanded)
                    .show(ui, |ui| {
                        for child in children.iter_mut() {
                            child.render(ui, editor_state);
                        }
                    })
                    .header_response
                    .clicked()
                    .then(|| {
                        *is_expanded = !*is_expanded;
                    });
            }
        }
    }
}

pub struct FileExplorer {
    root: Option<FileNode>,
}

impl FileExplorer {
    pub fn new() -> Self {
        Self { root: None }
    }
    
    pub fn set_root(&mut self, path: PathBuf) -> Result<()> {
        self.root = Some(FileNode::from_path(&path)?);
        Ok(())
    }
    
    pub fn render(&mut self, ui: &mut Ui, editor_state: &mut EditorState) {
        if let Some(root) = &mut self.root {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                root.render(ui, editor_state);
            });
        } else {
            ui.label("No workspace opened");
        }
    }
}

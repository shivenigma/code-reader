use anyhow::Result;
use egui::{Color32, ScrollArea, Sense, TextEdit, Ui, Widget};
use ropey::Rope;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

mod buffer;
pub use buffer::Buffer;

pub struct EditorState {
    // All open buffers
    buffers: HashMap<PathBuf, Buffer>,
    
    // Tab ordering
    tab_order: Vec<PathBuf>,
    
    // Active buffer
    active_buffer: Option<PathBuf>,
    
    // Syntax highlighting
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    current_theme: String,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            tab_order: Vec::new(),
            active_buffer: None,
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            current_theme: "base16-ocean.dark".to_string(),
        }
    }
    
    pub fn is_file_open(&self, path: &Path) -> bool {
        self.buffers.contains_key(path)
    }
    
    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        if !self.buffers.contains_key(&path) {
            // Load the file
            let content = fs::read_to_string(&path)?;
            
            // Determine syntax based on file extension
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            
            // Create a new buffer
            let buffer = Buffer::new(content, &path, extension, &self.syntax_set)?;
            
            // Add to open buffers
            self.buffers.insert(path.clone(), buffer);
            self.tab_order.push(path.clone());
        }
        
        // Set as active buffer
        self.active_buffer = Some(path);
        
        Ok(())
    }
    
    pub fn close_file(&mut self, path: &Path) {
        self.buffers.remove(path);
        self.tab_order.retain(|p| p != path);
        
        // Update active buffer if needed
        if self.active_buffer.as_ref().map_or(false, |p| p == path) {
            self.active_buffer = self.tab_order.last().cloned();
        }
    }
    
    pub fn render_tabs(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            for path in self.tab_order.clone() {
                if let Some(buffer) = self.buffers.get(&path) {
                    let file_name = path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Untitled".to_string());
                    
                    let is_active = self.active_buffer.as_ref().map_or(false, |p| *p == path);
                    
                    ui.selectable_value(
                        &mut self.active_buffer,
                        Some(path.clone()),
                        file_name,
                    );
                    
                    // Add a close button
                    if ui.small_button("✕").clicked() {
                        self.close_file(&path);
                    }
                }
            }
        });
        
        ui.separator();
    }
    
    pub fn render_editor(&mut self, ui: &mut Ui) {
        if let Some(active_path) = &self.active_buffer {
            if let Some(buffer) = self.buffers.get_mut(active_path) {
                // Render the editor
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        buffer.render(ui, &self.syntax_set, &self.theme_set, &self.current_theme);
                    });
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No file open. Select a file from the explorer to get started.");
            });
        }
    }
}

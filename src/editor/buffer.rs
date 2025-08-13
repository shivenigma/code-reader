use anyhow::Result;
use egui::{Color32, Response, TextEdit, Ui, ScrollArea};
use ropey::Rope;
use std::path::{Path, PathBuf};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

pub struct Buffer {
    content: Rope,
    path: PathBuf,
    syntax: Option<String>,
    modified: bool,
}

impl Buffer {
    pub fn new(
        content: String, 
        path: &Path, 
        extension: &str, 
        syntax_set: &SyntaxSet
    ) -> Result<Self> {
        // Determine syntax based on file extension
        let syntax = if !extension.is_empty() {
            syntax_set
                .find_syntax_by_extension(extension)
                .map(|s| s.name.clone())
        } else {
            // Try to determine syntax by first line
            let first_line = content.lines().next().unwrap_or("");
            syntax_set
                .find_syntax_by_first_line(first_line)
                .map(|s| s.name.clone())
        };
        
        Ok(Self {
            content: Rope::from_str(&content),
            path: path.to_path_buf(),
            syntax,
            modified: false,
        })
    }
    
    pub fn render(
        &mut self, 
        ui: &mut Ui, 
        syntax_set: &SyntaxSet, 
        theme_set: &ThemeSet,
        theme_name: &str,
    ) -> Response {
        // Convert the rope to a string for display
        let content_str = self.content.to_string();
        
        // Get the theme
        let theme = theme_set.themes.get(theme_name).unwrap_or_else(|| {
            &theme_set.themes["base16-ocean.dark"]
        });
        
        // Create a read-only text display
        ScrollArea::vertical().show(ui, |ui| {
            TextEdit::multiline(&mut content_str.clone())
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .interactive(false)  // Make it non-interactive (read-only)
                .show(ui)
                .response
        }).inner
        
        // Since we're read-only, no need to check for modifications
    }
    
    pub fn save(&mut self) -> Result<()> {
        std::fs::write(&self.path, self.content.to_string())?;
        self.modified = false;
        Ok(())
    }
}

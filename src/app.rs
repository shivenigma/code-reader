use eframe::{egui, CreationContext, Frame};
use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::editor::{Buffer, EditorState};
use crate::file_explorer::FileExplorer;
use crate::ui::theme::Theme;

pub struct CodeReaderApp {
    // UI state
    sidebar_width: f32,
    theme: Theme,
    
    // File explorer
    file_explorer: FileExplorer,
    
    // Editor state
    editor_state: EditorState,
    
    // Current workspace
    workspace_path: Option<PathBuf>,
}

impl CodeReaderApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // Set up custom fonts if needed
        // cc.egui_ctx.set_fonts(custom_fonts);
        
        // Initialize app state
        Self {
            sidebar_width: 250.0,
            theme: Theme::default(),
            file_explorer: FileExplorer::new(),
            editor_state: EditorState::new(),
            workspace_path: None,
        }
    }
    
    pub fn open_workspace(&mut self, path: PathBuf) -> Result<()> {
        if path.is_dir() {
            self.workspace_path = Some(path.clone());
            self.file_explorer.set_root(path)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Not a valid directory: {:?}", path))
        }
    }
    
    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open Folder...").clicked() {
                    // Show a folder picker dialog
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Select workspace folder")
                        .pick_folder() 
                    {
                        if let Err(e) = self.open_workspace(path) {
                            log::error!("Failed to open workspace: {:?}", e);
                        }
                    }
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            
            ui.menu_button("View", |ui| {
                if ui.button("Toggle File Explorer").clicked() {
                    self.sidebar_width = if self.sidebar_width > 0.0 { 0.0 } else { 250.0 };
                    ui.close_menu();
                }
            });
        });
    }
    
    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        if self.sidebar_width <= 0.0 {
            return;
        }
        
        egui::SidePanel::left("file_explorer_panel")
            .resizable(true)
            .default_width(self.sidebar_width)
            .width_range(100.0..=400.0)
            .show_inside(ui, |ui| {
                ui.heading("Files");
                ui.separator();
                
                if self.workspace_path.is_none() {
                    ui.label("No workspace opened");
                    if ui.button("Open Folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("Select workspace folder")
                            .pick_folder() 
                        {
                            if let Err(e) = self.open_workspace(path) {
                                log::error!("Failed to open workspace: {:?}", e);
                            }
                        }
                    }
                } else {
                    // Render file tree
                    self.file_explorer.render(ui, &mut self.editor_state);
                }
            });
    }
    
    fn render_editor_area(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Render tabs
            self.editor_state.render_tabs(ui);
            
            // Render editor for the active buffer
            self.editor_state.render_editor(ui);
        });
    }
}

impl eframe::App for CodeReaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render menu bar
            self.render_menu_bar(ui);
            
            // Main content with sidebar and editor area
            egui::Frame::none()
                .fill(self.theme.background_color)
                .show(ui, |ui| {
                    // Split layout for sidebar and editor
                    ui.horizontal(|ui| {
                        self.render_sidebar(ui);
                        self.render_editor_area(ui);
                    });
                });
        });
    }
}

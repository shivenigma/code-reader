use egui::Color32;

pub struct Theme {
    pub background_color: Color32,
    pub text_color: Color32,
    pub accent_color: Color32,
    pub sidebar_background: Color32,
    pub editor_background: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background_color: Color32::from_rgb(30, 30, 30),
            text_color: Color32::from_rgb(220, 220, 220),
            accent_color: Color32::from_rgb(75, 135, 220),
            sidebar_background: Color32::from_rgb(37, 37, 38),
            editor_background: Color32::from_rgb(30, 30, 30),
        }
    }
}

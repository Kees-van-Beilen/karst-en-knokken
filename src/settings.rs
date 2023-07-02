use bevy::prelude::*;


pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SettingsTextRendering::default());
    }
}

#[derive(Resource)]
pub struct SettingsTextRendering {
    pub font_path: String,
    pub font_path_bold: String,
    pub font_scale_multiplier: f32,
}

impl Default for SettingsTextRendering {
    fn default() -> Self {
        Self { font_path: "open_sans.ttf".to_string(),font_path_bold: "open_sans_bold.ttf".to_string(), font_scale_multiplier: 1.0 }
    }
}
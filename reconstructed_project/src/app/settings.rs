use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub artist_name: Option<String>,
    pub social_media_link: Option<String>,
    pub font_path: PathBuf,
    pub font_size: u16,
    pub text_opacity: f32,
    pub text_color: [u8; 4],
    pub text_move_speed: f32,
    pub fade_duration_secs: f32,
    pub social_media_display_duration_secs: f32,
    pub social_media_font_size: u16,
    pub social_media_opacity: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            artist_name: None,
            social_media_link: None,
            font_path: PathBuf::from("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
            font_size: 24,
            text_opacity: 1.0,
            text_color: [255, 255, 255, 255],
            text_move_speed: 100.0,
            fade_duration_secs: 5.0,
            social_media_display_duration_secs: 10.0,
            social_media_font_size: 16,
            social_media_opacity: 0.8,
        }
    }
}

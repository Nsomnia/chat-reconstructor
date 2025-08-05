use anyhow::Result;
use rand::Rng;
use std::time::Duration;
use vibestream::app::Settings;
use vibestream::audio::AudioPlayer;

#[derive(Debug, Clone)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct TextPosition {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub opacity: f32,
    pub fade_direction: i32, // -1 for fading out, 1 for fading in, 0 for stable
}

pub struct TextOverlay {
    artist_position: TextPosition,
    title_position: TextPosition,
    social_media_position: TextPosition,
    social_media_corner: Corner,
    social_media_timer: Duration,
    font: rusttype::Font<'static>,
}

impl TextOverlay {
    pub fn new(settings: Settings) -> Result<Self> {
        // Load font
        let font_data = std::fs::read(&settings.font_path)?;
        let font = rusttype::Font::try_from_vec(font_data)
            .ok_or_else(|| anyhow::anyhow!("Failed to load font"))?;

        // Initialize positions
        let artist_position = TextPosition {
            x: 100.0,
            y: 100.0,
            vx: 50.0,
            vy: 30.0,
            opacity: settings.text_opacity,
            fade_direction: 0,
        };

        let title_position = TextPosition {
            x: 100.0,
            y: 150.0,
            vx: -40.0,
            vy: 35.0,
            opacity: settings.text_opacity,
            fade_direction: 0,
        };

        let social_media_position = TextPosition {
            x: 10.0,
            y: 10.0,
            vx: 0.0,
            vy: 0.0,
            opacity: settings.social_media_opacity,
            fade_direction: 0,
        };

        let mut rng = rand::thread_rng();
        let social_media_corner = match rng.gen_range(0..4) {
            0 => Corner::TopLeft,
            1 => Corner::TopRight,
            2 => Corner::BottomLeft,
            _ => Corner::BottomRight,
        };

        Ok(Self {
            artist_position,
            title_position,
            social_media_position,
            social_media_corner,
            social_media_timer: Duration::from_secs(0),
            font,
        })
    }

    pub fn update(&mut self, delta_time: Duration, audio_player: &AudioPlayer, settings: &Settings) -> Result<()> {
        let delta_secs = delta_time.as_secs_f32();

        // Update social media timer
        self.social_media_timer += delta_time;

        // Move social media link to a new corner if it's time
        if self.social_media_timer.as_secs_f32() >= settings.social_media_display_duration_secs {
            self.social_media_timer = Duration::from_secs(0);

            let mut rng = rand::thread_rng();
            self.social_media_corner = match rng.gen_range(0..4) {
                0 => Corner::TopLeft,
                1 => Corner::TopRight,
                2 => Corner::BottomLeft,
                _ => Corner::BottomRight,
            };
        }

        // Get current track info
        let (artist, title) = if let Some(current_file) = audio_player.get_current_file() {
            let path = std::path::Path::new(current_file);

            // Try to read ID3 tags
            if let Ok(tag) = id3::Tag::read_from_path(path) {
                let artist = tag.artist()
                    .map(|s| s.to_string())
                    .or_else(|| settings.artist_name.clone())
                    .unwrap_or_else(|| "Unknown Artist".to_string());

                let title = tag.title()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| {
                        path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("Unknown Title")
                            .to_string()
                    });

                (artist, title)
            } else {
                // Fallback to filename and settings
                let artist = settings.artist_name.clone()
                    .unwrap_or_else(|| "Unknown Artist".to_string());

                let title = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown Title")
                    .to_string();

                (artist, title)
            }
        } else {
            ("No Track".to_string(), "No Track".to_string())
        };

        // Update artist position with bouncing effect
        self.update_text_position(
            &mut self.artist_position,
            &artist,
            settings.font_size,
            delta_secs,
            settings.text_move_speed,
            audio_player,
            settings.fade_duration_secs,
        )?;

        // Update title position with bouncing effect
        self.update_text_position(
            &mut self.title_position,
            &title,
            settings.font_size,
            delta_secs,
            settings.text_move_speed,
            audio_player,
            settings.fade_duration_secs,
        )?;

        Ok(())
    }

    fn update_text_position(
        &mut self,
        position: &mut TextPosition,
        text: &str,
        font_size: u16,
        delta_secs: f32,
        move_speed: f32,
        audio_player: &AudioPlayer,
        fade_duration_secs: f32,
    ) -> Result<()> {
        // Calculate text dimensions
        let scale = rusttype::Scale::uniform(font_size as f32);
        let v_metrics = self.font.v_metrics(scale);
        let text_width = self.font
            .layout(text, scale, rusttype::Point { x: 0.0, y: 0.0 })
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .last()
            .unwrap_or(0.0);
        let text_height = v_metrics.ascent - v_metrics.descent;

        // Update position
        position.x += position.vx * delta_secs;
        position.y += position.vy * delta_secs;

        // Bounce off edges
        if position.x <= 0.0 || position.x + text_width >= 800.0 {
            position.vx = -position.vx;
            // Add some randomness to prevent getting stuck
            let mut rng = rand::thread_rng();
            position.vy += rng.gen_range(-10.0..10.0);
        }

        if position.y <= 0.0 || position.y + text_height >= 600.0 {
            position.vy = -position.vy;
            // Add some randomness to prevent getting stuck
            let mut rng = rand::thread_rng();
            position.vx += rng.gen_range(-10.0..10.0);
        }

        // Handle fading
        if let Some(duration) = audio_player.get_duration() {
            let position = audio_player.get_position();
            let remaining = duration - position;

            // Start fading out after fade_duration_secs from start
            if position.as_secs_f32() >= fade_duration_secs && remaining.as_secs_f32() >= fade_duration_secs {
                if position.fade_direction != -1 {
                    position.fade_direction = -1;
                }
            }
            // Start fading in when fade_duration_secs from end
            else if remaining.as_secs_f32() < fade_duration_secs {
                if position.fade_direction != 1 {
                    position.fade_direction = 1;
                }
            }
            // Otherwise, stay at current opacity
            else {
                position.fade_direction = 0;
            }

            // Apply fading
            if position.fade_direction != 0 {
                let fade_amount = (delta_secs / fade_duration_secs) * position.fade_direction as f32;
                position.opacity = (position.opacity + fade_amount).clamp(0.2, 1.0);
            }
        }

        Ok(())
    }

    pub fn render(&self, width: u32, height: u32) -> Result<()> {
        // Update social media position based on corner
        let (social_x, social_y) = match self.social_media_corner {
            Corner::TopLeft => (10.0, 10.0),
            Corner::TopRight => (width as f32 - 200.0, 10.0),
            Corner::BottomLeft => (10.0, height as f32 - 30.0),
            Corner::BottomRight => (width as f32 - 200.0, height as f32 - 30.0),
        };

        // In a real implementation, we would render the text using OpenGL
        // For now, we'll just leave this as a placeholder

        Ok(())
    }
}

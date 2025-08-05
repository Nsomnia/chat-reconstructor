use egui::{Ui, RichText};
use std::time::Duration;
use vibestream::audio::{AudioPlayer, Playlist};

pub struct Controls {
    seek_bar_value: f32,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            seek_bar_value: 0.0,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, audio_player: &mut AudioPlayer, playlist: &mut Playlist) {
        ui.horizontal_centered(|ui| {
            // Previous track button
            if ui.button("⏮").clicked() {
                if let Some(_) = playlist.previous() {
                    // Load the new track
                    if let Some(item) = playlist.get_current_item() {
                        if let Err(e) = audio_player.load_file(&item.path) {
                            eprintln!("Error loading file: {}", e);
                        } else {
                            audio_player.play();
                        }
                    }
                }
            }

            // Play/Pause button
            let play_pause_text = if audio_player.is_paused() || !audio_player.is_playing() {
                "▶"
            } else {
                "⏸"
            };

            if ui.button(RichText::new(play_pause_text).size(24.0)).clicked() {
                audio_player.toggle_play_pause();
            }

            // Stop button
            if ui.button("⏹").clicked() {
                audio_player.stop();
            }

            // Next track button
            if ui.button("⏭").clicked() {
                if let Some(_) = playlist.next() {
                    // Load the new track
                    if let Some(item) = playlist.get_current_item() {
                        if let Err(e) = audio_player.load_file(&item.path) {
                            eprintln!("Error loading file: {}", e);
                        } else {
                            audio_player.play();
                        }
                    }
                }
            }
        });

        ui.add_space(10.0);

        // Seek bar
        ui.horizontal(|ui| {
            let duration = audio_player.get_duration().unwrap_or(Duration::from_secs(0));
            let position = audio_player.get_position();

            // Update seek bar value
            if duration.as_secs_f32() > 0.0 {
                self.seek_bar_value = position.as_secs_f32() / duration.as_secs_f32();
            } else {
                self.seek_bar_value = 0.0;
            }

            // Format time strings
            let pos_str = format!("{:02}:{:02}", position.as_secs() / 60, position.as_secs() % 60);
            let dur_str = format!("{:02}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60);

            ui.label(pos_str);

            if ui.add(egui::Slider::new(&mut self.seek_bar_value, 0.0..=1.0).show_value(false))
                .changed()
            {
                // Seek to new position
                let new_pos = Duration::from_secs_f32(self.seek_bar_value * duration.as_secs_f32());
                if let Err(e) = audio_player.seek(new_pos) {
                    eprintln!("Error seeking: {}", e);
                }
            }

            ui.label(dur_str);
        });

        ui.add_space(10.0);

        // Current track info
        if let Some(item) = playlist.get_current_item() {
            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new(item.artist.as_deref().unwrap_or("Unknown Artist")).size(16.0).strong());
                    ui.label(RichText::new(item.title.as_deref().unwrap_or("Unknown Title")).size(14.0));
                });
            });
        }
    }
}

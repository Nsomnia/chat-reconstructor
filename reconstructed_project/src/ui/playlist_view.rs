use egui::{Ui, RichText};
use vibestream::audio::{AudioPlayer, Playlist, PlaylistItem};
use rfd::FileDialog;

pub struct PlaylistView {
    selected_index: Option<usize>,
    drag_start_index: Option<usize>,
}

impl PlaylistView {
    pub fn new() -> Self {
        Self {
            selected_index: None,
            drag_start_index: None,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, playlist: &mut Playlist, audio_player: &mut AudioPlayer) {
        let items = playlist.get_items();
        let current_index = playlist.get_current_index();

        for (i, item) in items.iter().enumerate() {
            let is_current = current_index == Some(i);
            let is_selected = self.selected_index == Some(i);

            let mut frame = egui::Frame::default();
            if is_current {
                frame = frame.fill(egui::Color32::from_rgb(50, 50, 100));
            } else if is_selected {
                frame = frame.fill(egui::Color32::from_rgb(40, 40, 40));
            }

            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Track number
                    ui.label(RichText::new(format!("{}", i + 1)).size(12.0));

                    // Artist and title
                    ui.vertical(|ui| {
                        ui.label(RichText::new(item.artist.as_deref().unwrap_or("Unknown Artist")).size(12.0).strong());
                        ui.label(RichText::new(item.title.as_deref().unwrap_or("Unknown Title")).size(11.0));
                    });

                    // Remove button
                    if ui.button("❌").clicked() {
                        playlist.remove_item(i);
                        if self.selected_index == Some(i) {
                            self.selected_index = None;
                        }
                    }

                    // Handle drag and drop
                    if ui.input(|i| i.pointer.primary_pressed()) {
                        self.selected_index = Some(i);
                        self.drag_start_index = Some(i);
                    }

                    if ui.input(|i| i.pointer.primary_released()) {
                        if let (Some(start), Some(end)) = (self.drag_start_index, self.selected_index) {
                            if start != end {
                                playlist.move_item(start, end);
                            }
                        }
                        self.drag_start_index = None;
                    }

                    // Handle double-click to play
                    if ui.input(|i| i.pointer.double_clicked()) {
                        playlist.set_current_index(Some(i));
                        if let Some(item) = playlist.get_current_item() {
                            if let Err(e) = audio_player.load_file(&item.path) {
                                eprintln!("Error loading file: {}", e);
                            } else {
                                audio_player.play();
                            }
                        }
                    }
                });
            });
        }

        // Add file button
        if ui.button("Add File...").clicked() {
            if let Some(path) = FileDialog::new()
                .add_filter("Audio Files", &["mp3", "wav", "flac", "ogg"])
                .pick_file()
            {
                playlist.add_item(path);
            }
        }

        // Add folder button
        if ui.button("Add Folder...").clicked() {
            if let Some(path) = FileDialog::new().pick_folder() {
                // Add all audio files in the folder
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            let extension = path.extension()
                                .and_then(|s| s.to_str())
                                .unwrap_or("");

                            if ["mp3", "wav", "flac", "ogg"].contains(&extension) {
                                playlist.add_item(path);
                            }
                        }
                    }
                }
            }
        }
    }
}

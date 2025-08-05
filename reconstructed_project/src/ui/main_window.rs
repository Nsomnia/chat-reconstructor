use egui::{Context, ScrollArea, Ui};
use vibestream::audio::{AudioPlayer, Playlist};
use rfd::FileDialog;

pub struct MainWindow {
    controls: Controls,
    playlist_view: PlaylistView,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            controls: Controls::new(),
            playlist_view: PlaylistView::new(),
        }
    }

    pub fn show(
        &mut self,
        ctx: &Context,
        audio_player: &mut AudioPlayer,
        playlist: &mut Playlist,
        show_settings: &mut bool,
    ) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open File...").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Audio Files", &["mp3", "wav", "flac", "ogg"])
                            .pick_file()
                        {
                            playlist.clear();
                            playlist.add_item(path);
                            playlist.set_current_index(Some(0));

                            if let Some(item) = playlist.get_current_item() {
                                if let Err(e) = audio_player.load_file(&item.path) {
                                    eprintln!("Error loading file: {}", e);
                                } else {
                                    audio_player.play();
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Open Folder...").clicked() {
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

                                if !playlist.get_items().is_empty() {
                                    playlist.set_current_index(Some(0));

                                    if let Some(item) = playlist.get_current_item() {
                                        if let Err(e) = audio_player.load_file(&item.path) {
                                            eprintln!("Error loading file: {}", e);
                                        } else {
                                            audio_player.play();
                                        }
                                    }
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Config", |ui| {
                    if ui.button("Settings").clicked() {
                        *show_settings = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // Show about dialog
                        egui::Window::new("About VibeStream")
                            .collapsible(false)
                            .resizable(false)
                            .show(ctx, |ui| {
                                ui.heading("VibeStream");
                                ui.label("An audio player with projectM visualization");
                                ui.label("Version 0.1.0");
                                ui.add_space(10.0);
                                ui.label("Created for artists to showcase their music");
                                ui.label("with captivating visuals.");
                                ui.add_space(10.0);
                                if ui.button("OK").clicked() {
                                    // Close window
                                }
                            });
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            // This is where the projectM visualization would be rendered
            // We're leaving it empty as the visualization is handled by SDL/OpenGL
        });

        // Left panel for playlist
        egui::SidePanel::left("playlist_panel")
            .default_width(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Playlist");
                ScrollArea::vertical().show(ui, |ui| {
                    self.playlist_view.show(ui, playlist, audio_player);
                });
            });

        // Bottom panel for controls
        egui::TopBottomPanel::bottom("controls_panel")
            .default_height(100.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.controls.show(ui, audio_player, playlist);
            });

        // Right panel for additional controls
        egui::SidePanel::right("extra_controls_panel")
            .default_width(150.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Audio Settings");
                ui.add_space(10.0);

                // Volume control
                ui.label("Volume:");
                let mut volume = audio_player.get_volume();
                if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).text("Volume")).changed() {
                    audio_player.set_volume(volume);
                }

                ui.add_space(10.0);

                // Visualization settings could go here
                ui.heading("Visualization");
                ui.add_space(5.0);

                if ui.button("Random Preset").clicked() {
                    // Handle random preset
                }

                ui.add_space(5.0);

                if ui.button("Next Preset").clicked() {
                    // Handle next preset
                }

                ui.add_space(5.0);

                if ui.button("Previous Preset").clicked() {
                    // Handle previous preset
                }

                ui.add_space(10.0);

                ui.heading("Recording");
                ui.add_space(5.0);

                if ui.button("Start Recording").clicked() {
                    // Handle start recording
                }

                ui.add_space(5.0);

                if ui.button("Stop Recording").clicked() {
                    // Handle stop recording
                }
            });
    }
}

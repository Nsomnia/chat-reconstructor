use anyhow::Result;
use glutin::ContextWrapper;
use glutin::config::GlConfig;
use glutin::display::Display;
use glutin::surface::Surface;
use std::time::Duration;
use vibestream::audio::{AudioPlayer, Playlist};
use vibestream::config::Config;
use vibestream::ui::{MainWindow, SettingsWindow};
use vibestream::visualization::{ProjectMWrapper, TextOverlay};

pub struct VibeStreamApp {
    config: Config,
    window: winit::window::Window,
    gl_context: ContextWrapper<glutin::PossiblyCurrentContext, glutin::surface::WindowSurface>,
    audio_player: AudioPlayer,
    playlist: Playlist,
    pub visualization: ProjectMWrapper,
    text_overlay: TextOverlay,
    main_window: MainWindow,
    settings_window: SettingsWindow,
    show_settings: bool,
}

impl VibeStreamApp {
    pub fn new(
        config: Config,
        window: winit::window::Window,
        gl_context: ContextWrapper<glutin::PossiblyCurrentContext, glutin::surface::WindowSurface>,
    ) -> Result<Self> {
        // Initialize audio player
        let audio_player = AudioPlayer::new()?;

        // Initialize playlist
        let playlist = Playlist::new();

        // Initialize visualization
        let mut visualization = ProjectMWrapper::new(
            window.inner_size().width,
            window.inner_size().height,
        )?;

        // Set preset and texture paths if available
        if let Ok(preset_path) = std::env::var("PROJECTM_PRESET_PATH") {
            visualization.set_preset_path(&preset_path)?;
        }

        if let Ok(texture_path) = std::env::var("PROJECTM_TEXTURE_PATH") {
            visualization.set_texture_path(&texture_path)?;
        }

        // Initialize text overlay
        let text_overlay = TextOverlay::new(config.settings.clone())?;

        // Initialize UI components
        let main_window = MainWindow::new();
        let settings_window = SettingsWindow::new(config.settings.clone());

        Ok(Self {
            config,
            window,
            gl_context,
            audio_player,
            playlist,
            visualization,
            text_overlay,
            main_window,
            settings_window,
            show_settings: false,
        })
    }

    pub fn update(&mut self, delta_time: Duration) -> Result<()> {
        // Update audio player
        self.audio_player.update()?;

        // Update visualization
        self.visualization.update(&self.audio_player)?;

        // Update text overlay
        self.text_overlay.update(
            delta_time,
            &self.audio_player,
            &self.config.settings,
        )?;

        // Handle track completion
        if self.audio_player.is_playing() && self.audio_player.get_position() >= self.audio_player.get_duration().unwrap_or(Duration::from_secs(0)) {
            if let Some(_) = self.playlist.next() {
                if let Some(item) = self.playlist.get_current_item() {
                    self.audio_player.load_file(&item.path)?;
                    self.audio_player.play();
                }
            } else {
                self.audio_player.stop();
            }
        }

        Ok(())
    }

    pub fn render(&mut self, egui_ctx: &egui::Context) -> Result<()> {
        // Update visualization size if window was resized
        let size = self.window.inner_size();
        self.visualization.resize(size.width, size.height);

        // Clear the screen
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Render visualization
        self.visualization.render()?;

        // Render text overlay
        self.text_overlay.render(size.width, size.height)?;

        // Render UI
        if self.show_settings {
            if self.settings_window.show(egui_ctx, &mut self.config.settings) {
                self.show_settings = false;
            }
        } else {
            self.main_window.show(
                egui_ctx,
                &mut self.audio_player,
                &mut self.playlist,
                &mut self.visualization,
                &mut self.show_settings,
            );
        }

        Ok(())
    }

    pub fn save_config(&self) -> Result<()> {
        self.config.save()?;
        Ok(())
    }
}

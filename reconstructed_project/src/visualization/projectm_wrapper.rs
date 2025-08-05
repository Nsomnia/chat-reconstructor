use anyhow::Result;
use std::ffi::CString;
use vibestream::audio::AudioPlayer;

pub struct ProjectMWrapper {
    projectm: projectm::ProjectM,
    width: u32,
    height: u32,
}

impl ProjectMWrapper {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let projectm = projectm::ProjectM::create(
            projectm::RenderTarget::Default,
            projectm::TextureSize::Small,
            projectm::DoubleBuffering::Enabled,
        )?;

        Ok(Self {
            projectm,
            width,
            height,
        })
    }

    pub fn update(&mut self, audio_player: &AudioPlayer) -> Result<()> {
        // Get audio data if available
        if let Some(_pcm) = audio_player.get_pcm_data() {
            // In a real implementation, we would pass PCM data to projectM
            // For now, we'll just generate a dummy texture
        }

        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        // Render projectM visualization
        self.projectm.render_frame(
            self.width,
            self.height,
            0, // time
            None, // texture
        )?;

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

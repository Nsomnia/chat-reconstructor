use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

pub struct AudioPlayer {
    stream: OutputStream,
    sink: Option<Sink>,
    current_file: Option<String>,
    duration: Option<Duration>,
    position: Duration,
    volume: f32,
    paused: bool,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, _) = OutputStream::try_default()?;

        Ok(Self {
            stream,
            sink: None,
            current_file: None,
            duration: None,
            position: Duration::from_secs(0),
            volume: 1.0,
            paused: false,
        })
    }

    pub fn load_file(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;

        // Get duration if available
        let duration = source.total_duration();

        // Create new sink
        let sink = Sink::try_new(&self.stream.handle())?;
        sink.set_volume(self.volume);
        sink.append(source);

        self.sink = Some(sink);
        self.current_file = Some(path.to_string_lossy().to_string());
        self.duration = duration;
        self.position = Duration::from_secs(0);
        self.paused = false;

        Ok(())
    }

    pub fn play(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.paused = false;
        }
    }

    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            self.paused = true;
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if self.paused {
            self.play();
        } else {
            self.pause();
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        self.sink = None;
        self.current_file = None;
        self.duration = None;
        self.position = Duration::from_secs(0);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume);
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn seek(&mut self, position: Duration) -> Result<()> {
        if let Some(current_file) = &self.current_file {
            let path = Path::new(current_file);
            self.load_file(path)?;

            if let Some(sink) = &self.sink {
                // This is a bit of a hack since rodio doesn't support seeking directly
                // We're just playing silence until we reach the desired position
                let silence = rodio::source::Zero::<f32>::new(1, 1);

                if position > Duration::from_secs(0) {
                    sink.append(silence.take_duration(position));
                }
            }

            self.position = position;
        }

        Ok(())
    }

    pub fn get_position(&self) -> Duration {
        if let Some(sink) = &self.sink {
            self.position + sink.get_pos()
        } else {
            self.position
        }
    }

    pub fn get_duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn is_playing(&self) -> bool {
        if let Some(sink) = &self.sink {
            !sink.is_paused() && !sink.empty()
        } else {
            false
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
    }

    pub fn get_pcm_data(&self) -> Option<&[f32]> {
        // In a real implementation, this would return the current PCM data
        // For now, we'll return None
        None
    }

    pub fn update(&mut self) -> Result<()> {
        // Update position if playing
        if self.is_playing() {
            self.position = self.get_position();
        }

        Ok(())
    }
}

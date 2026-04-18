use crate::algorithms::SortStep;
use crate::config::ParsedAudio;

#[cfg(feature = "audio")]
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
#[cfg(feature = "audio")]
use std::time::Duration;

enum Backend {
    #[cfg(feature = "audio")]
    Rodio { _stream: OutputStream, handle: OutputStreamHandle, volume: f32 },
    Bel,
    Silent,
}

pub struct Player {
    backend:   Backend,
    muted:     bool,
    last_done: bool,
}

impl Player {
    pub fn new(cfg: &ParsedAudio) -> Self {
        if !cfg.enabled {
            return Self { backend: Backend::Silent, muted: false, last_done: false };
        }

        #[cfg(feature = "audio")]
        {
            use crate::config::AudioBackend;
            if matches!(cfg.backend, AudioBackend::Auto | AudioBackend::Rodio) {
                if let Ok((stream, handle)) = OutputStream::try_default() {
                    return Self {
                        backend: Backend::Rodio { _stream: stream, handle, volume: cfg.volume },
                        muted: false,
                        last_done: false,
                    };
                }
            }
            if matches!(cfg.backend, AudioBackend::Auto | AudioBackend::Bel) {
                return Self { backend: Backend::Bel, muted: false, last_done: false };
            }
        }

        Self { backend: Backend::Silent, muted: false, last_done: false }
    }

    pub fn toggle_mute(&mut self) { self.muted = !self.muted; }
    pub fn is_muted(&self) -> bool { self.muted }

    pub fn play_step(&mut self, step: &SortStep, array_size: usize, speed_ms: u64, is_done: bool) {
        if self.muted { return; }

        let newly_done = is_done && !self.last_done;
        if is_done  { self.last_done = true;  }
        else        { self.last_done = false; }

        if newly_done {
            match &self.backend {
                Backend::Bel => { print!("\x07"); }
                #[cfg(feature = "audio")]
                Backend::Rodio { handle, volume, .. } => {
                    play_sine(handle, 880.0, *volume, 120);
                }
                Backend::Silent => {}
            }
            return;
        }

        #[cfg(feature = "audio")]
        if let Backend::Rodio { handle, volume, .. } = &self.backend {
            let idx = step.swapping.first()
                .or_else(|| step.comparing.first())
                .copied();
            if let Some(i) = idx {
                if i < step.data.len() && array_size > 0 {
                    let ratio = step.data[i] as f64 / array_size as f64;
                    let freq  = (220.0 + ratio * 880.0) as f32;
                    let dur   = speed_ms.min(60);
                    play_sine(handle, freq, *volume, dur);
                }
            }
        }

        let _ = (step, array_size, speed_ms);
    }
}

#[cfg(feature = "audio")]
fn play_sine(handle: &OutputStreamHandle, freq: f32, volume: f32, dur_ms: u64) {
    if let Ok(sink) = Sink::try_new(handle) {
        let source = rodio::source::SineWave::new(freq)
            .take_duration(Duration::from_millis(dur_ms))
            .amplify(volume);
        sink.append(source);
        sink.detach();
    }
}

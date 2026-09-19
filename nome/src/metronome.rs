use hilen::system::ScreenAwake;
use mnomer::{
    BeatPattern, BeatPatternType, BeatPlayer, ToneConfiguration,
    frequency_relative_semitone_equal_temperament,
};

pub const BEATS: usize = 4;

const MIN_BPM: u16 = 20;
const MAX_BPM: u16 = 300;
const START_BPM: u16 = 100;

const TONE_FREQ: f64 = 440.0;
const ACCENT_SEMITONES: f64 = 5.0;

/// One working metronome, owning its own player so nothing is shared.
pub struct Metronome {
    player:       BeatPlayer,
    bpm:          u16,
    screen_awake: Option<ScreenAwake>,
}

impl Default for Metronome {
    fn default() -> Self {
        Self {
            player:       make_player(),
            bpm:          START_BPM,
            screen_awake: None,
        }
    }
}

impl Metronome {
    /// Runs on the audio thread every time a new beat starts sounding, so the
    /// picture comes from the same clock as the sound. It must not block, hop
    /// to the main thread before touching any view.
    pub fn on_beat(&mut self, listener: impl Fn(usize) + Send + Sync + 'static) {
        self.player.set_on_beat(listener);
    }

    pub fn bpm(&self) -> u16 {
        self.bpm
    }

    pub fn set_bpm(&mut self, bpm: i32) {
        let bpm = bpm.clamp(MIN_BPM.into(), MAX_BPM.into());
        self.bpm = u16::try_from(bpm).expect("bpm was clamped into u16 range");
        self.player.set_bpm(self.bpm);
    }

    pub fn change_bpm(&mut self, delta: i16) {
        self.set_bpm(i32::from(self.bpm) + i32::from(delta));
    }

    pub fn is_playing(&self) -> bool {
        self.player.is_playing()
    }

    pub fn toggle(&mut self) {
        if self.is_playing() {
            self.stop();
        } else {
            self.player.play_beat().expect("failed to start the beat");
            self.screen_awake = Some(ScreenAwake::acquire());
        }
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.screen_awake = None;
    }
}

impl Drop for Metronome {
    fn drop(&mut self) {
        self.stop();
    }
}

fn make_player() -> BeatPlayer {
    let normal = ToneConfiguration {
        frequency:   TONE_FREQ,
        sample_rate: 48000.0,
        length:      0.05,
        overtones:   1,
        channels:    1,
    };

    let accent = ToneConfiguration {
        frequency: frequency_relative_semitone_equal_temperament(TONE_FREQ, ACCENT_SEMITONES),
        ..normal
    };

    BeatPlayer::new(
        START_BPM,
        4,
        normal,
        accent,
        BeatPattern(vec![
            BeatPatternType::Accent,
            BeatPatternType::Beat,
            BeatPatternType::Beat,
            BeatPatternType::Beat,
        ]),
    )
}

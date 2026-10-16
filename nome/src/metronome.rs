use std::sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, Ordering},
};

use hilen::{
    audio::{Sound, SoundClock, Tone},
    refs::{Weak, manage::DataManager},
    store::OnDisk,
    system::ScreenAwake,
    time::frame_seconds,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};

pub const BEATS: u8 = 4;

const MIN_BPM: u16 = 20;
const MAX_BPM: u16 = 300;
const START_BPM: u16 = 100;

const ACCENT_SEMITONES: f64 = 5.0;
const HI_HAT_VOLUME: f32 = 0.6;
const HI_HAT_ACCENT_VOLUME: f32 = 1.0;

static CLICK: LazyLock<OnDisk<Click>> = LazyLock::new(|| OnDisk::new("click.json"));

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Click {
    #[default]
    Beep,
    HiHat,
}

/// One working metronome, owning its own clock so nothing is shared.
pub struct Metronome {
    clock:        SoundClock,
    bpm:          u16,
    click:        Click,
    /// Read by the feed of the clock, so a new choice is heard from the next
    /// queued beat on, while it plays.
    hi_hat:       Arc<AtomicBool>,
    beep:         Weak<Sound>,
    hi_hat_sound: Weak<Sound>,
    screen_awake: Option<ScreenAwake>,
}

impl Default for Metronome {
    fn default() -> Self {
        // The engine starts every sound at 0.1, the click is the only sound here.
        Sound::set_volume(1.0);
        let click = CLICK.get().unwrap_or_default();
        info!("click {click:?}, {START_BPM} bpm");
        Self {
            clock: SoundClock::new(f64::from(START_BPM)),
            bpm: START_BPM,
            click,
            hi_hat: Arc::new(AtomicBool::new(click == Click::HiHat)),
            beep: Sound::get("beep.wav"),
            hi_hat_sound: Sound::get("hihat.wav"),
            screen_awake: None,
        }
    }
}

impl Metronome {
    pub fn bpm(&self) -> u16 {
        self.bpm
    }

    pub fn set_bpm(&mut self, bpm: i32) {
        let bpm = bpm.clamp(MIN_BPM.into(), MAX_BPM.into());
        self.bpm = u16::try_from(bpm).expect("bpm was clamped into u16 range");
        self.clock.set_bpm(f64::from(self.bpm));
        info!("{} bpm", self.bpm);
    }

    pub fn change_bpm(&mut self, delta: i16) {
        self.set_bpm(i32::from(self.bpm) + i32::from(delta));
    }

    pub fn click(&self) -> Click {
        self.click
    }

    pub fn set_click(&mut self, click: Click) {
        self.click = click;
        self.hi_hat.store(click == Click::HiHat, Ordering::Relaxed);
        CLICK.set(click);
        info!("click {click:?}");
    }

    pub fn is_playing(&self) -> bool {
        self.clock.is_running()
    }

    /// Seconds of 1 bar of `BEATS` beats.
    pub fn bar_seconds(&self) -> f64 {
        60.0 * f64::from(BEATS) / f64::from(self.bpm)
    }

    /// Where the bar stands in the frame that is drawn, 0 to 1, from the time
    /// of the clock, so the picture follows the sound.
    pub fn bar_phase(&self) -> f64 {
        (self.clock.beats_at(frame_seconds()) / f64::from(BEATS)).rem_euclid(1.0)
    }

    pub fn toggle(&mut self) {
        if self.is_playing() {
            self.stop();
        } else {
            self.start();
        }
    }

    fn start(&mut self) {
        let beep = self.beep;
        let hi_hat_sound = self.hi_hat_sound;
        let hi_hat = self.hi_hat.clone();
        let queue = move |clock: &mut SoundClock, beat: f64| {
            let accent = beat_index(beat) == 0;
            if hi_hat.load(Ordering::Relaxed) {
                let volume = if accent {
                    HI_HAT_ACCENT_VOLUME
                } else {
                    HI_HAT_VOLUME
                };
                clock.play_at_beat(&hi_hat_sound, beat, volume);
            } else if accent {
                let tone = Tone {
                    volume:   1.0,
                    rate:     Tone::rate_of(ACCENT_SEMITONES),
                    end_beat: None,
                };
                clock.play_tone_at_beat(&beep, beat, tone);
            } else {
                clock.play_at_beat(&beep, beat, 1.0);
            }
        };

        // The first feed call comes a few ms after the start, so beat 0 goes in
        // before it.
        queue(&mut self.clock, 0.0);
        let mut next = 1.0;
        self.clock.set_feed(move |clock| {
            let now = clock.beats();
            if next < now.ceil() {
                debug!("the feed skipped from beat {next} to {}", now.ceil());
                next = now.ceil();
            }
            let until = now + clock.bpm() / 120.0;
            while next < until {
                queue(clock, next);
                next += 1.0;
            }
        });
        self.clock.start();
        self.screen_awake = Some(ScreenAwake::acquire());
        info!("start, {} bpm, click {:?}", self.bpm, self.click);
    }

    pub fn stop(&mut self) {
        if self.is_playing() {
            info!("stop");
        }
        self.clock.stop();
        self.screen_awake = None;
    }
}

impl Drop for Metronome {
    fn drop(&mut self) {
        self.stop();
    }
}

fn beat_index(beat: f64) -> usize {
    // A queued beat is a whole number, the cast only drops the `.0`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let beat = beat.round() as usize;
    beat % usize::from(BEATS)
}

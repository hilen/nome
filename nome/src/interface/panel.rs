use hilen::{
    Event,
    refs::Weak,
    system::AppActivity,
    ui::{Button, Label, Setup, UIColor, ViewData, view},
};

use crate::{
    interface::{click_choice::ClickChoice, tempo_controls::TempoControls},
    metronome::Metronome,
};

/// The number, the sound choice, the six step buttons and Start, plus the
/// metronome they drive.
///
/// The screen above it only draws the beat, so the controls and the readout
/// live in one place and cannot drift apart from each other.
#[view]
pub struct ControlPanel {
    /// Fires after playback or the tempo changes, including background stops,
    /// so the screen can start, move or clear its beat.
    pub changed: Event<()>,

    #[educe(Default = Metronome::default())]
    metronome: Metronome,

    #[init]
    tempo:    Label,
    click:    ClickChoice,
    controls: TempoControls,
    start:    Button,
}

impl ControlPanel {
    /// Room a variant has to leave free at the bottom of its own frame.
    pub const HEIGHT: f32 = 220.0;

    pub fn set_palette(
        self: Weak<Self>,
        surface: impl Into<UIColor> + Copy,
        text: impl Into<UIColor> + Copy,
        accent: impl Into<UIColor> + Copy,
        on_accent: impl Into<UIColor> + Copy,
        radius: f32,
    ) -> Weak<Self> {
        self.tempo.set_text_color(text);
        self.click.set_style(surface, text, accent, on_accent, radius);
        self.click.set_selected(self.metronome.click());
        self.controls.set_style(surface, text, radius);
        self.start.set_color(accent).set_text_color(on_accent).set_corner_radius(radius);
        self
    }

    pub fn is_playing(&self) -> bool {
        self.metronome.is_playing()
    }

    pub fn bar_seconds(&self) -> f64 {
        self.metronome.bar_seconds()
    }

    pub fn bar_phase(&self) -> f64 {
        self.metronome.bar_phase()
    }

    fn refresh(self: Weak<Self>) {
        self.tempo.set_text(self.metronome.bpm());
        self.start.set_text(if self.metronome.is_playing() {
            "Stop"
        } else {
            "Start"
        });
    }
}

impl Setup for ControlPanel {
    fn setup(mut self: Weak<Self>) {
        AppActivity::changed().val(self, move |active| {
            if !active && self.metronome.is_playing() {
                self.metronome.stop();
                self.refresh();
                self.changed.trigger(());
            }
        });

        self.tempo.set_text_size(46).place().fit_text().center_x().t(0);

        self.click.place().lr(0).t(58).h(44);
        self.click.changed.val(move |click| {
            self.metronome.set_click(click);
            self.click.set_selected(click);
        });

        self.controls.place().lr(0).t(110).h(44);
        self.controls.changed.val(move |delta| {
            self.metronome.change_bpm(delta);
            self.refresh();
            self.changed.trigger(());
        });

        self.start.set_text_size(18).place().lrb(0).h(46);
        self.start.on_tap(move || {
            self.metronome.toggle();
            self.refresh();
            self.changed.trigger(());
        });

        self.refresh();
    }
}

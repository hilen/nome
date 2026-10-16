use hilen::{
    Event,
    refs::Weak,
    ui::{Button, Setup, UIColor, ViewData, view},
};

use crate::metronome::Click;

/// Beep or hi-hat, 2 buttons side by side, the chosen one in the accent color.
#[view]
pub struct ClickChoice {
    pub changed: Event<Click>,

    surface:   Option<UIColor>,
    text:      Option<UIColor>,
    accent:    Option<UIColor>,
    on_accent: Option<UIColor>,

    #[init]
    beep:   Button,
    hi_hat: Button,
}

impl ClickChoice {
    pub fn set_style(
        mut self: Weak<Self>,
        surface: impl Into<UIColor> + Copy,
        text: impl Into<UIColor> + Copy,
        accent: impl Into<UIColor> + Copy,
        on_accent: impl Into<UIColor> + Copy,
        radius: f32,
    ) -> Weak<Self> {
        self.surface = Some(surface.into());
        self.text = Some(text.into());
        self.accent = Some(accent.into());
        self.on_accent = Some(on_accent.into());
        self.beep.set_corner_radius(radius);
        self.hi_hat.set_corner_radius(radius);
        self
    }

    pub fn set_selected(self: Weak<Self>, click: Click) {
        let (Some(surface), Some(text), Some(accent), Some(on_accent)) =
            (self.surface, self.text, self.accent, self.on_accent)
        else {
            return;
        };
        for (button, own) in [(self.beep, Click::Beep), (self.hi_hat, Click::HiHat)] {
            if own == click {
                button.set_color(accent).set_text_color(on_accent);
            } else {
                button.set_color(surface).set_text_color(text);
            }
        }
    }
}

impl Setup for ClickChoice {
    fn setup(self: Weak<Self>) {
        self.place().all_hor().all(6);

        self.beep.set_text("Beep").set_text_size(18);
        self.beep.on_tap(move || self.changed.trigger(Click::Beep));

        self.hi_hat.set_text("Hi-hat").set_text_size(18);
        self.hi_hat.on_tap(move || self.changed.trigger(Click::HiHat));
    }
}

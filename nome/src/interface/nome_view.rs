use hilen::{
    refs::Weak,
    ui::{Container, LoopAnimation, Setup, UIManager, ViewData, ViewSubviews, view},
};

use crate::{interface::panel::ControlPanel, metronome::BEATS};

const BACKGROUND: &str = "#12100F";
const DIM: &str = "#241F1C";
const BEAT: &str = "#FF8A5B";
const ACCENT: &str = "#FFD166";
const TEXT: &str = "#F4EFEA";

/// The metronome screen. Four tall bars side by side, and the beat sweeps left
/// to right across them the way a bar of music reads.
#[view]
pub struct NomeView {
    cells: Vec<Weak<Container>>,

    #[init]
    field: Container,
    panel: ControlPanel,
}

impl Setup for NomeView {
    fn setup(mut self: Weak<Self>) {
        UIManager::set_clear_color(BACKGROUND);

        self.field.place().lrt(20).b(ControlPanel::HEIGHT + 30.0).all_hor().all(10);

        for _ in 0..BEATS {
            let cell = self.field.add_view::<Container>();
            cell.set_color(DIM).set_corner_radius(10);
            self.cells.push(cell);
        }

        self.panel.changed.val(move |()| self.follow());

        self.panel.set_palette(DIM, TEXT, BEAT, BACKGROUND, 10.0);
        self.panel.place().lrb(16).h(ControlPanel::HEIGHT);
    }
}

impl NomeView {
    /// The light is a declared loop of 1 bar, so on Apple platforms Core
    /// Animation shows it and the app sleeps between beats. It starts at the
    /// phase of the sound clock, again after every change of the tempo.
    fn follow(self: Weak<Self>) {
        if !self.panel.is_playing() {
            self.field.stop_loop();
            self.clear();
            return;
        }

        let quarters = (1..BEATS).map(|beat| f32::from(beat) / f32::from(BEATS));
        let pose = move |phase: f32| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let beat = (phase * f32::from(BEATS)) as usize;
            self.light(beat.min(usize::from(BEATS) - 1));
        };
        self.field.run_loop(
            LoopAnimation::new(self.panel.bar_seconds(), pose)
                .changes_at(quarters)
                .starting_at(self.panel.bar_phase()),
        );
    }

    fn light(self: Weak<Self>, beat: usize) {
        for (index, cell) in self.cells.iter().enumerate() {
            let color = if index != beat {
                DIM
            } else if index == 0 {
                ACCENT
            } else {
                BEAT
            };

            cell.set_color(color);
        }
    }

    fn clear(self: Weak<Self>) {
        for cell in &self.cells {
            cell.set_color(DIM);
        }
    }
}

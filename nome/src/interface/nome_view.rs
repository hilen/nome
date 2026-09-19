use hilen::{
    dispatch::on_main,
    refs::Weak,
    ui::{Container, Setup, UIManager, ViewData, ViewSubviews, view},
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

        // The player reports each beat from the audio thread, so the light
        // comes off the same clock as the click and cannot drift from it.
        // Views are main thread only, hence the hop.
        self.panel.on_beat(move |beat| {
            on_main(move || {
                if self.is_ok() && self.panel.is_playing() {
                    self.light(beat);
                }
            });
        });

        self.panel.toggled.val(move |()| {
            if !self.panel.is_playing() {
                self.clear();
            }
        });

        self.panel.set_palette(DIM, TEXT, BEAT, BACKGROUND, 10.0);
        self.panel.place().lrb(16).h(ControlPanel::HEIGHT);
    }
}

impl NomeView {
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

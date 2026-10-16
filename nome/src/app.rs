use hilen::{
    App,
    refs::Own,
    ui::{Setup, Size, View},
};
use log::info;

use crate::interface::nome_view::NomeView;

#[derive(Default)]
pub struct NomeApp;

impl App for NomeApp {
    fn make_root_view(&self) -> Own<dyn View> {
        info!("nome {} starts", env!("CARGO_PKG_VERSION"));
        NomeView::new()
    }

    fn log_targets(&self) -> &'static [&'static str] {
        &["nome"]
    }

    /// Physical pixels, so this is the portrait shape the screen was designed
    /// at once a retina display halves it.
    fn initial_size(&self) -> Size {
        (960, 1560).into()
    }
}

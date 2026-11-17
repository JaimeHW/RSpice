//! Where widgets put things, published in test builds.
//!
//! A widget publishes the rect it laid an element out at, under a key that
//! names the element, and a test reads the pass's report back. A measurement
//! read back from the pixels would be a second account of where the code put
//! things. Every widget publishes into one report per context, so a page's
//! test sees the page's own elements and the kit's alike.

use egui::Rect;

/// Rects published this pass, by what they are, in the order published.
#[derive(Debug, Clone, Default)]
struct Probes(Vec<(String, Rect)>);

fn probes_id() -> egui::Id {
    egui::Id::new("rspice_ui_kit.probes")
}

/// Publish where one element landed, for the geometry report.
pub fn probe(ui: &egui::Ui, key: &str, rect: Rect) {
    ui.ctx().data_mut(|data| {
        data.get_temp_mut_or_default::<Probes>(probes_id())
            .0
            .push((key.to_owned(), rect));
    });
}

/// Everything published since the last take, which empties the report.
pub fn take(ctx: &egui::Context) -> Vec<(String, Rect)> {
    ctx.data_mut(|data| {
        let probes = data.get_temp::<Probes>(probes_id()).unwrap_or_default();
        data.remove::<Probes>(probes_id());
        probes.0
    })
}

/// Where one pass's elements landed.
#[derive(Debug, Clone, Default)]
pub struct Geometry {
    probes: Vec<(String, Rect)>,
}

impl Geometry {
    pub fn all(&self, key: &str) -> Vec<Rect> {
        self.probes
            .iter()
            .filter(|(name, _)| name == key)
            .map(|(_, rect)| *rect)
            .collect()
    }

    pub fn one(&self, key: &str) -> Option<Rect> {
        self.all(key).into_iter().next()
    }
}

/// One pass of `page` over `input` in an unframed central panel, and where
/// its widgets put things on it.
pub fn pass(
    ctx: &egui::Context,
    input: egui::RawInput,
    mut page: impl FnMut(&mut egui::Ui),
) -> (egui::FullOutput, Geometry) {
    let _ = take(ctx);
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| page(ui));
    });
    let probes = take(ctx);
    (output, Geometry { probes })
}

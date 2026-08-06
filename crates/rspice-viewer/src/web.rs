//! The wasm-bindgen boundary: mount one figure runtime on one page canvas.

use eframe::wasm_bindgen::{self, prelude::*};
use rspice_publication_contract::ManifestEntry;

use crate::payload::{self, LoadedFigure};
use crate::scene::SceneCamera;

struct FigureApp {
    figure: LoadedFigure,
    camera: SceneCamera,
}

impl eframe::App for FigureApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match &self.figure.plot {
            Some(plot) => crate::plot::plot_pane(ui, plot),
            None => crate::scene::scene_pane(ui, &self.figure.scene, &mut self.camera),
        }
    }
}

/// Hydrate one figure: verify the fetched payload against its manifest
/// entry and start the runtime on the entry's canvas.
///
/// Any error is returned before the runtime starts, so the page's static
/// SVG stays untouched when hydration is rejected.
#[wasm_bindgen]
pub async fn hydrate_figure(
    canvas_id: String,
    manifest_entry_json: String,
    payload_bytes: Vec<u8>,
) -> Result<(), JsValue> {
    let entry: ManifestEntry = serde_json::from_str(&manifest_entry_json)
        .map_err(|error| JsValue::from_str(&format!("manifest entry rejected: {error}")))?;
    let figure = payload::load_figure(&payload_bytes, &entry.payload, entry.figure_id)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| JsValue::from_str("no document"))?;
    let canvas = document
        .get_element_by_id(&canvas_id)
        .ok_or_else(|| JsValue::from_str("figure canvas not found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(move |_context| {
                Ok(Box::new(FigureApp {
                    figure,
                    camera: SceneCamera::default(),
                }))
            }),
        )
        .await
}

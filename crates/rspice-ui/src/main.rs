//! RSpice Application Entry Point
//!
//! Cross-platform entry point for RSpice UI.
//! Uses egui for commercial-grade GPU-accelerated rendering.

// =============================================================================
// Desktop Entry Point - Commercial-grade GPU-native UI
// =============================================================================

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Initialize logging with wgpu_core set to warn to reduce noise
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,wgpu_core=warn"),
    )
    .init();

    log::info!("Starting RSpice UI with egui (commercial-grade GPU rendering)...");

    // Configure eframe options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("RSpice - Circuit Simulator")
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0]),
        vsync: true,
        // egui anti-aliases analytically (feathered tessellation); hardware
        // MSAA would quadruple fill bandwidth for no visible gain.
        multisampling: 1,
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            // One queued frame instead of wgpu's default two: a full frame
            // less input-to-photon latency while panning the canvas, with
            // vsync still pacing presentation.
            desired_maximum_frame_latency: Some(1),
            ..Default::default()
        },
        ..Default::default()
    };

    // Launch the egui application
    if let Err(e) = eframe::run_native(
        "RSpice",
        options,
        Box::new(|cc| Ok(Box::new(rspice_ui::RSpiceApp::new(cc)))),
    ) {
        log::error!("Failed to launch RSpice: {}", e);
        std::process::exit(1);
    }
}

// =============================================================================
// Web Entry Point — eframe WebRunner into the #rspice_canvas element
// =============================================================================

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Info).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let canvas = document
            .get_element_by_id("rspice_canvas")
            .expect("web shell must provide #rspice_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("#rspice_canvas is not a canvas element");

        let result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(rspice_ui::RSpiceApp::new(cc)))),
            )
            .await;

        // Hand off from the static loading overlay once egui owns the canvas.
        if let Some(loading) = document.get_element_by_id("rspice_loading") {
            match result {
                Ok(_) => loading.remove(),
                Err(e) => {
                    loading.set_inner_html(&format!(
                        "<p class=\"err\">RSpice failed to start: {e:?}</p>\
                         <p class=\"err\">A WebGPU-capable browser (current Chrome/Edge) is required.</p>"
                    ));
                }
            }
        }
    });
}

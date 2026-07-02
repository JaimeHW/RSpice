//! RSpice Application Entry Point
//!
//! Cross-platform entry point for RSpice UI.
//! Uses egui for commercial-grade GPU-accelerated rendering.

#![allow(deprecated)]

// =============================================================================
// Desktop Entry Point - Commercial-grade GPU-native UI
// =============================================================================

/// Decode the embedded brand PNG into an egui window icon (taskbar / alt-tab).
/// Embedded at compile time so the running app needs no icon file on disk;
/// returns `None` on any decode surprise so a bad asset never blocks startup.
#[cfg(not(target_arch = "wasm32"))]
fn load_window_icon() -> Option<egui::IconData> {
    const ICON_PNG: &[u8] = include_bytes!("../../../assets/brand/icon-256.png");

    let mut decoder = png::Decoder::new(std::io::Cursor::new(ICON_PNG));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().ok()?;
    let mut buf = vec![0u8; reader.output_buffer_size()?];
    let info = reader.next_frame(&mut buf).ok()?;

    let rgba = match info.color_type {
        png::ColorType::Rgba => {
            buf.truncate(info.buffer_size());
            buf
        }
        png::ColorType::Rgb => buf[..info.buffer_size()]
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        _ => return None,
    };

    Some(egui::IconData {
        rgba,
        width: info.width,
        height: info.height,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Keep RSpice diagnostics visible while suppressing routine GPU backend
    // probe warnings from normal desktop stderr.
    env_logger::Builder::from_env(rspice_ui::common::logging::native_log_env()).init();

    log::info!("Starting RSpice UI with egui (commercial-grade GPU rendering)...");

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("RSpice - Circuit Simulator")
        .with_inner_size([1400.0, 900.0])
        .with_min_inner_size([800.0, 600.0]);
    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    // Configure eframe options
    let options = eframe::NativeOptions {
        viewport,
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
#[wasm_bindgen::prelude::wasm_bindgen(js_name = runRspiceUiWorkerRequest)]
pub fn run_rspice_ui_worker_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::run_rspice_ui_worker_request(value)
}

#[cfg(target_arch = "wasm32")]
const WEB_STARTUP_REQUIREMENT: &str = "A WebGPU-capable browser (current Chrome/Edge) is required.";

#[cfg(target_arch = "wasm32")]
fn report_web_startup_error(document: Option<&web_sys::Document>, message: &str) {
    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(message));

    let Some(document) = document else {
        return;
    };
    let Some(loading) = document.get_element_by_id("rspice_loading") else {
        return;
    };

    loading.set_inner_html("");
    for text in [message, WEB_STARTUP_REQUIREMENT] {
        let Ok(paragraph) = document.create_element("p") else {
            return;
        };
        paragraph.set_class_name("err");
        paragraph.set_text_content(Some(text));
        let _ = loading.append_child(&paragraph);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        report_web_startup_error(
            None,
            "RSpice failed to start: no browser document is available.",
        );
        return;
    };

    eframe::WebLogger::init(log::LevelFilter::Info).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {
        let Some(canvas_element) = document.get_element_by_id("rspice_canvas") else {
            report_web_startup_error(
                Some(&document),
                "RSpice failed to start: the web shell did not provide #rspice_canvas.",
            );
            return;
        };
        let Ok(canvas) = canvas_element.dyn_into::<web_sys::HtmlCanvasElement>() else {
            report_web_startup_error(
                Some(&document),
                "RSpice failed to start: #rspice_canvas is not a canvas element.",
            );
            return;
        };

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
                Err(e) => report_web_startup_error(
                    Some(&document),
                    &format!("RSpice failed to start: {e:?}"),
                ),
            }
        }
    });
}

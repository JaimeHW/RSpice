//! The drawn colour of a retained trace.

use crate::presentation::trace_color;
use rspice_results::waveform::RetainedTrace;
use rspice_ui_kit::tokens::Tokens;

/// Resolve a waveform's display color from its stored hex + palette fallback.
pub fn waveform_color(waveform: &RetainedTrace, index: usize, t: &Tokens) -> egui::Color32 {
    trace_color(
        &waveform.color,
        t.color.traces[index % t.color.traces.len()],
    )
}

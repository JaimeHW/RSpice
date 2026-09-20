//! Authored transistor pin roles from the implementation selected by the builder.
use super::*;

/// Optional external MOS terminals beyond the standard drain/gate/source pins.
///
/// Indices refer to the flattened element's authored node list, never internal
/// model states. A native SOI device's fourth pin is its back gate (E); an
/// optional fifth pin is its body contact (P). Floating internal bodies and
/// implicit three-terminal VDMOS body ties have no external body pin.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MosTerminalLayout {
    /// External bulk or body-contact pin, if explicitly connected.
    pub body: Option<usize>,
    /// External back-gate/substrate electrode, if explicitly connected.
    pub back_gate: Option<usize>,
}

/// Optional external BJT electrical terminal beyond collector/base/emitter.
/// LEVEL=11 VBIC's optional fourth thermal pin is not a substrate terminal.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BjtTerminalLayout {
    /// Explicit electrical substrate pin in the authored flattened node list.
    pub substrate: Option<usize>,
}

impl CircuitData {
    /// External substrate role from the actual selected BJT implementation.
    pub fn bjt_terminal_layout(&self, device: &str) -> Option<BjtTerminalLayout> {
        self.bjt_terminal_layouts
            .get(&device.to_ascii_uppercase())
            .copied()
    }

    pub(crate) fn record_bjt_terminal_layout(&mut self, device: &str, layout: BjtTerminalLayout) {
        self.bjt_terminal_layouts
            .insert(device.to_ascii_uppercase(), layout);
    }

    /// Pin roles emitted by the actual selected native/generated implementation.
    pub fn mos_terminal_layout(&self, device: &str) -> Option<MosTerminalLayout> {
        self.mos_terminal_layouts
            .get(&device.to_ascii_uppercase())
            .copied()
    }

    pub(crate) fn record_mos_terminal_layout(&mut self, device: &str, layout: MosTerminalLayout) {
        self.mos_terminal_layouts
            .insert(device.to_ascii_uppercase(), layout);
    }
}

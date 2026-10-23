//! The HDL output a run keeps for its result and its checks.

use std::sync::Arc;

use super::EngineBridge;
use crate::error::SimulationError;

/// The run's HDL output (design P2 section 4.3): kept, bounded, for the
/// result, and still handed to whatever sink the caller installed.
#[derive(Debug, Default)]
pub(super) struct HdlOutputCapture {
    pub(super) collector: rspice_core::config::HdlOutputCollector,
    forward: Option<Arc<dyn rspice_core::config::HdlOutputSink>>,
}

impl rspice_core::config::HdlOutputSink for HdlOutputCapture {
    fn emit(&self, record: &rspice_core::config::HdlOutputRecord) {
        self.collector.emit(record);
        if let Some(forward) = &self.forward {
            forward.emit(record);
        }
    }
}

impl EngineBridge {
    /// `engine`, also keeping what the deck's HDL designs print, on an engine
    /// that shares its metrics and diagnostics.
    pub(super) fn capturing_hdl_output(
        &self,
        engine: &rspice_core::Engine,
    ) -> Result<(rspice_core::Engine, Arc<HdlOutputCapture>), SimulationError> {
        let capture = Arc::new(HdlOutputCapture {
            forward: engine.config().hdl.output.clone(),
            ..HdlOutputCapture::default()
        });
        let mut capturing = engine.config().clone();
        capturing.hdl.output = Some(capture.clone());
        let engine = engine
            .try_resolved_with_config(capturing)
            .map_err(|error| self.translate_error(error.into()))?;
        Ok((engine, capture))
    }
}

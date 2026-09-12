//! Resolving a caller's spelling to a retained transient series.
//!
//! `TransientResult` is addressed four ways -- by node index, by node name, by
//! `NodeIdentifier`, and by the full SPICE output grammar -- and every one of
//! them has to refuse the same things for the same reasons: an index past the
//! namespace, a name the deck never wrote, and an event-only net that carries
//! no voltage to return. Keeping the four together is what makes
//! `signal("v(clk)")` and `voltage_waveform("clk")` raise the same exception
//! for the same net, which is the contract the suites assert. `mod.rs` keeps
//! the constructors and the Python-facing signatures; this is the resolution
//! they all delegate to.

use super::*;

impl PyTransientResult {
    pub(super) fn device_op_values(&self, device: &str, parameter: &str) -> PyResult<&[f64]> {
        self.inner
            .device_op_traces
            .iter()
            .find(|trace| {
                trace.device_name.eq_ignore_ascii_case(device)
                    && trace.parameter.eq_ignore_ascii_case(parameter)
            })
            .map(|trace| trace.values.as_slice())
            .ok_or_else(|| {
                crate::errors::key_error(format!(
                    "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
                ))
            })
    }

    pub(super) fn checked_time_index(&self, time_index: usize) -> AccessResult<()> {
        if time_index < self.inner.time.len() {
            Ok(())
        } else {
            Err(invalid_time_index_error(time_index, self.inner.time.len()))
        }
    }

    pub(super) fn checked_waveform(&self, node: usize) -> AccessResult<Vec<f64>> {
        if node == 0 {
            return Ok(vec![0.0; self.inner.num_points()]);
        }

        let waveform = self
            .inner
            .try_voltage_waveform(node)
            .ok_or_else(|| invalid_node_index_error(node, self.inner.num_nodes))?;
        // An empty column is an unretained channel, and an empty array is the
        // honest answer for one. An event-only net is not that: it has no
        // voltage to retain in the first place, so it is refused by name.
        if let Some((name, kind)) = self.event_only_node(node) {
            return Err(event_only_node_error(
                &name,
                kind,
                EventTraceSurface::Result,
            ));
        }
        Ok(waveform.to_vec())
    }

    /// The node's name and event domain when the result carries it as events
    /// and not as a voltage.
    pub(super) fn event_only_node(
        &self,
        node: usize,
    ) -> Option<(String, rspice_core::analysis::transient::EventOnlyNetKind)> {
        let name = self.inner.node_names.get(node.checked_sub(1)?)?;
        self.inner
            .event_only_node_kind(name)
            .map(|kind| (name.clone(), kind))
    }

    /// The refusal `signal(spec)` owes an event-only net, decided before the
    /// core resolver is asked.
    ///
    /// `signal("v(clk)")` and `voltage_waveform("clk")` refuse the same net for
    /// the same reason, so they have to raise the same exception type. The core
    /// resolver reports the miss as a netlist error, which the binding renders
    /// as a `ValueError`; recognising the spelling here — with the same parser
    /// the output cards use — makes the contract "an event-only net is a
    /// `KeyError`" whichever accessor was called. Only the single-node voltage
    /// spelling qualifies, the same restriction the core probe applies.
    pub(super) fn event_only_probe_refusal(&self, spec: &str) -> Option<ResultAccessError> {
        let rspice_core::netlist::SaveSignal::Voltage(node) =
            rspice_core::netlist::parse_save_probe(spec.trim())?
        else {
            return None;
        };
        let kind = self.inner.event_only_node_kind(&node)?;
        Some(event_only_node_error(
            &node,
            kind,
            EventTraceSurface::Result,
        ))
    }

    pub(super) fn checked_waveform_named(&self, name: &str) -> AccessResult<Vec<f64>> {
        if is_ground_name(name) {
            return self.checked_waveform(0);
        }

        let node = self
            .inner
            .node_index_named(name)
            .ok_or_else(|| unknown_node_name_error(name))?;
        self.checked_waveform(node)
    }

    pub(super) fn waveform_for(&self, node: &NodeIdentifier) -> PyResult<Vec<f64>> {
        match node {
            NodeIdentifier::Index(idx) => self.checked_waveform(*idx),
            NodeIdentifier::Name(name) => self.checked_waveform_named(name),
        }
        .map_err(PyErr::from)
    }

    /// Resolve any SPICE output specification to a time series.
    ///
    /// The core output resolver owns this grammar, so a differential pair, a
    /// branch current, a device-lead current, an `@device[param]` observable,
    /// and a hierarchy spelling mean exactly what they mean on a `.PRINT` or
    /// `.FOUR` card. The binding layer only maps the typed failure onto an
    /// exception.
    pub(crate) fn probe_waveform(&self, spec: &str) -> PyResult<Vec<f64>> {
        if let Some(refusal) = self.event_only_probe_refusal(spec) {
            return Err(PyErr::from(refusal));
        }
        rspice_core::analysis::evaluate_transient_probe_with_abort(
            None,
            &self.inner,
            spec,
            &rspice_core::abort_signal::NoAbort,
        )
        .map_err(|error| match error {
            rspice_core::SimulationError::RequestedSignalUnavailable(_) => {
                crate::errors::key_error(error.to_string())
            }
            other => crate::errors::value_error(other.to_string()),
        })
    }
}

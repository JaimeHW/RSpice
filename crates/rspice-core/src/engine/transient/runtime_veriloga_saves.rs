//! `.SAVE I(x)` on an instance of a runtime-compiled Verilog-A module.
//!
//! A transient reports a device's current from a solved branch unknown or
//! from a derived lead current the device computes (`DerivedTransientBranch
//! Current`). A runtime `.va` instance, analog-only or mixed, has neither: its
//! compiled body stamps its terminal rows and keeps no per-terminal current.
//! The save would otherwise match nothing, and the result would silently lack
//! the column the deck asked for. So it is refused before any solver work,
//! with the supported way to measure the same current.

use super::*;
#[cfg(feature = "veriloga")]
use crate::engine::UnsupportedCapabilityError;
#[cfg(feature = "veriloga")]
use crate::identity::AnalysisKind;

impl Engine {
    /// Refuse a `.SAVE I(name)` that names a runtime Verilog-A instance.
    #[cfg(feature = "veriloga")]
    pub(in crate::engine) fn ensure_runtime_veriloga_current_saves(
        netlist: &Netlist,
        circuit: &crate::circuit::CircuitData,
    ) -> Result<(), SimulationError> {
        for signal in &netlist.saves.signals {
            let SaveSignal::Current(name) = signal else {
                continue;
            };
            let analog = circuit
                .veriloga_devices
                .iter()
                .find(|device| device.name.eq_ignore_ascii_case(name))
                .map(|device| {
                    (
                        device.name.to_string(),
                        Some(device.model_name().to_string()),
                    )
                });
            let mixed = || {
                circuit
                    .mixed_signal_hosts
                    .iter()
                    .find(|host| host.instance_name().eq_ignore_ascii_case(name))
                    .map(|host| (host.instance_name().to_string(), None))
            };
            let Some((instance, module)) = analog.or_else(mixed) else {
                continue;
            };
            let mut refusal = UnsupportedCapabilityError::new(
                "output.veriloga.terminal_current",
                format!(
                    "`.SAVE I({name})` names the runtime-compiled Verilog-A instance \
                     '{instance}', which does not report a terminal current; put a zero-volt \
                     source in series with the terminal and save its current instead"
                ),
            )
            .for_analysis_kind(AnalysisKind::Tran)
            .for_instance(instance);
            if let Some(module) = module {
                refusal = refusal.for_module(module);
            }
            return Err(refusal.into());
        }
        Ok(())
    }

    /// Without the `veriloga` feature no runtime instance exists.
    #[cfg(not(feature = "veriloga"))]
    pub(in crate::engine) fn ensure_runtime_veriloga_current_saves(
        _netlist: &Netlist,
        _circuit: &crate::circuit::CircuitData,
    ) -> Result<(), SimulationError> {
        Ok(())
    }
}

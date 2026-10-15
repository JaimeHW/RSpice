//! Exact `G + sC` descriptors of runtime Verilog-A devices and the analog
//! halves of mixed Verilog-AMS modules.
//!
//! A complex stamp answers one frequency; a filter's or an integrator's state
//! sampled at two of them is a fit, not the device. Each instance instead
//! hands over its descriptor: the entries on circuit unknowns join the base
//! `G` and `C` exactly where the complex stamp put them, and each `laplace_*`
//! filter, `idt` and `ddt`-of-`s` cut is a further state with its own row.
//! An `idt` without an initial condition is lowered to an internal node whose
//! equation is `ddt(y) - x`, so its state is already a circuit unknown and
//! arrives as ordinary circuit entries.
use super::*;
use crate::Complex64;
use crate::device::veriloga::VerilogADevice;
use crate::engine::UnsupportedCapabilityError;
use crate::solver::ComplexMatrix;
use rspice_veriloga::vm::{DescriptorIndex, SmallSignalDescriptor};

/// The analysis type the descriptor is evaluated under: pole-zero is an AC
/// linearization, so `analysis("ac")` reads true as it does for `.ac`.
const AC_ANALYSIS: u8 = 1;

pub(super) struct VerilogADescriptor {
    instance: String,
    descriptor: SmallSignalDescriptor,
}

fn invalid(instance: &str, detail: impl std::fmt::Display) -> SimulationError {
    SimulationError::Circuit(format!(
        "Verilog-A instance '{instance}' pole-zero descriptor failed: {detail}"
    ))
}

impl VerilogADescriptor {
    /// Every runtime device and host analog half, linearized at `op` the way
    /// the complex stamp linearizes it: a clone switched to AC, at the
    /// circuit's point time for a host.
    pub(super) fn collect(
        circuit: &CircuitData,
        op: &[Value],
    ) -> Result<Vec<Self>, SimulationError> {
        let devices = circuit
            .veriloga_devices()
            .iter()
            .map(|device| (device.name.to_string(), device, None));
        let hosts = circuit.mixed_signal_hosts.iter().map(|host| {
            (
                host.instance_name().to_owned(),
                host.analog_device(),
                Some(circuit.veriloga_point_time),
            )
        });
        devices
            .chain(hosts)
            .map(|(instance, device, time)| Self::linearize(instance, device, time, op))
            .collect()
    }

    fn linearize(
        instance: String,
        device: &VerilogADevice,
        time: Option<Value>,
        op: &[Value],
    ) -> Result<Self, SimulationError> {
        let mut cloned = device.clone();
        cloned
            .try_set_analysis_type(AC_ANALYSIS)
            .map_err(|error| invalid(&instance, error))?;
        if let Some(time) = time {
            cloned
                .try_set_time(time)
                .map_err(|error| invalid(&instance, error))?;
        }
        let descriptor = cloned.try_small_signal_descriptor(op).map_err(|error| {
            if let rspice_veriloga::vm::VmError::NonRationalOperator(operator) = error {
                SimulationError::from(
                    UnsupportedCapabilityError::new(
                        "analysis.pz.non_rational_operator",
                        format!(
                            "pole-zero analysis does not support Verilog-A instance \
                             '{instance}' (module '{}'): it uses {operator}, which has no \
                             finite rational G + sC descriptor",
                            device.model_name()
                        ),
                    )
                    .for_analysis_kind(crate::identity::AnalysisKind::PoleZero)
                    .for_instance(instance.clone())
                    .for_module(device.model_name().to_owned()),
                )
            } else {
                invalid(&instance, error)
            }
        })?;
        let this = Self {
            instance,
            descriptor,
        };
        for entry in &this.descriptor.entries {
            if matches!(entry.row, DescriptorIndex::NoiseInput)
                || matches!(entry.col, DescriptorIndex::NoiseInput)
            {
                return Err(invalid(
                    &this.instance,
                    "a small-signal descriptor has no noise input",
                ));
            }
            if !entry.g.is_finite() || !entry.c.is_finite() {
                return Err(invalid(&this.instance, "a coefficient is non-finite"));
            }
        }
        Ok(this)
    }

    pub(super) fn state_count(&self) -> usize {
        self.descriptor.states.len()
    }

    /// Add the entries on circuit unknowns to the base pencil. `g` and `c`
    /// are the fills at `omega = 0` and `omega = 1`, so an entry `g + s·c`
    /// lands as the complex stamp would have put it: `g` on the first and
    /// `g + j·c` on the second.
    pub(super) fn stamp_circuit_entries(
        descriptors: &[Self],
        g: &mut ComplexMatrix,
        c: &mut ComplexMatrix,
    ) {
        for descriptor in descriptors {
            for entry in &descriptor.descriptor.entries {
                if let (DescriptorIndex::Circuit(row), DescriptorIndex::Circuit(col)) =
                    (entry.row, entry.col)
                {
                    g.add(row, col, Complex64::new(entry.g, 0.0));
                    c.add(row, col, Complex64::new(entry.g, entry.c));
                }
            }
        }
    }

    /// Append every explicit state, with its rows and columns, to the dense
    /// pencil after the states already there.
    pub(super) fn stamp_states(descriptors: &[Self], g: &mut Matrix, c: &mut Matrix) {
        let count = descriptors
            .iter()
            .map(Self::state_count)
            .fold(0usize, usize::saturating_add);
        if count == 0 {
            return;
        }
        let mut first = Engine::descriptor_expand_square(g, c, count);
        for descriptor in descriptors {
            let index = |at: DescriptorIndex| match at {
                DescriptorIndex::Circuit(row) => row,
                DescriptorIndex::State(state) => first + state,
                DescriptorIndex::NoiseInput => unreachable!("rejected at collection"),
            };
            for entry in &descriptor.descriptor.entries {
                if matches!(
                    (entry.row, entry.col),
                    (DescriptorIndex::Circuit(_), DescriptorIndex::Circuit(_))
                ) {
                    continue;
                }
                let (row, col) = (index(entry.row), index(entry.col));
                g.add(row, col, entry.g);
                c.add(row, col, entry.c);
            }
            first += descriptor.state_count();
        }
    }
}

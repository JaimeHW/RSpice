//! Native BSIM4 elementary noise, including externalized resistor mechanisms.

use super::*;
use crate::analysis::noise::{NoiseSource, NoiseSourceIdentity};
use crate::device::Bsim4v8Device;

impl Engine {
    pub(in crate::engine::hb) fn bsim4_owns_periodic_resistor_noise(
        circuit: &CircuitData,
        name: &str,
    ) -> bool {
        let Some((owner, suffix)) = name.rsplit_once(".__") else {
            return false;
        };
        ["rd", "rs", "rg"]
            .iter()
            .any(|tag| suffix.eq_ignore_ascii_case(tag))
            && circuit
                .bsim4v8
                .devices
                .iter()
                .any(|device| device.name.eq_ignore_ascii_case(owner))
    }

    pub(super) fn collect_bsim4_periodic_noise_sources(
        device: &Bsim4v8Device,
        solution: &[Value],
    ) -> Result<Vec<NoiseSource>, SimulationError> {
        if device.core.model.tnoi_mod == 2 {
            return Err(SimulationError::Circuit(format!(
                "BSIM4 '{}' TNOIMOD=2 requires periodic correlated gate/drain noise",
                device.name
            )));
        }
        let (mut sources, correlated) = Self::collect_bsim4v8_noise_sources(device)?;
        debug_assert!(correlated.is_empty());
        let (op, bias) = device.noise_operating_point();
        let model = &device.core.model;
        let size = &device.core.size;
        let inst = &device.core.inst;
        let (mut drain, mut source) = device
            .external_rds_conductances(solution)
            .unwrap_or((inst.drain_conductance, inst.source_conductance));
        if model.tnoi_mod == 1 && op.idovvds > 0.0 && op.esat_l != 0.0 {
            let shape = (op.vgsteff / op.esat_l).powi(2);
            let beta = model.rnoia * (1.0 + shape * model.tnoia * size.leff);
            let theta = (model.rnoib * (1.0 + shape * model.tnoib * size.leff))
                .min(0.9)
                .min(0.9 * beta);
            let adjusted = if bias.vds >= 0.0 {
                &mut source
            } else {
                &mut drain
            };
            if *adjusted > 0.0 && adjusted.is_finite() {
                *adjusted *= 1.0 + theta * theta * *adjusted / op.idovvds;
            }
        }
        let mut resistor = |mechanism: &str,
                            pos: usize,
                            neg: usize,
                            conductance: Value|
         -> Result<(), SimulationError> {
            if pos == neg {
                return Ok(());
            }
            let label = format!("{}:{mechanism}", device.name);
            let conductance = Self::checked_noise_product(&label, conductance, device.multiplier)?;
            if let Some(resistance) = Self::noise_resistance_from_conductance(&label, conductance)?
            {
                sources.push(
                    NoiseSource::thermal(device.name.clone(), pos, neg, resistance)
                        .with_identity(NoiseSourceIdentity::mechanism(&device.name, mechanism)),
                );
            }
            Ok(())
        };
        resistor("RD", device.node_drain, device.node_drain_external, drain)?;
        resistor(
            "RS",
            device.node_source,
            device.node_source_external,
            source,
        )?;
        match model.rgate_mod {
            1 => resistor(
                "RG",
                device.node_gate,
                device.node_gate_external,
                inst.gate_conductance,
            )?,
            3 => resistor(
                "RG",
                device.node_gate_mid,
                device.node_gate_external,
                inst.gate_conductance,
            )?,
            _ => {}
        }
        Ok(sources)
    }
}

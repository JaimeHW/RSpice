//! Authoritative identity and implementation metadata for schematic devices.
//!
//! `ComponentType` remains the persisted legacy discriminator.  The stable id
//! in this descriptor is the migration target for a catalog that can also hold
//! generated Verilog-A and registry-provided XSPICE devices without growing a
//! second hard-coded enum.

use super::ComponentType;

/// The simulation path responsible for a built-in schematic device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceImplementation {
    /// A native SPICE element emitted directly by the schematic netlister.
    Native,
    /// A higher-level schematic object lowered into one or more native cards.
    Synthesized,
    /// A registered XSPICE code model, identified by its canonical model type.
    Xspice { model_type: &'static str },
    /// A connectivity or hierarchy object that is not itself a primitive card.
    Structural,
    /// A model/library-bound hierarchical instance.
    LibraryCell,
}

/// Immutable metadata shared by persistence migration, placement, symbols,
/// properties, netlisting, and release qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceDescriptor {
    /// Product-wide, serialization-safe identity.  Never reuse or rename an id
    /// after release; aliases belong in the document migration layer.
    pub stable_id: &'static str,
    /// Default embedded SVG filename, relative to `assets/component_symbols`.
    pub default_symbol_asset: Option<&'static str>,
    /// Simulation implementation selected for the device.
    pub implementation: DeviceImplementation,
}

impl ComponentType {
    /// Return the single authoritative descriptor for this legacy device kind.
    pub const fn descriptor(self) -> DeviceDescriptor {
        use ComponentType::*;
        use DeviceImplementation::{LibraryCell, Native, Structural, Synthesized};

        let (stable_id, default_symbol_asset, implementation) = match self {
            Resistor => ("rspice.native.resistor", Some("resistor.svg"), Native),
            Capacitor => (
                "rspice.native.capacitor",
                Some("cap_unpolarized.svg"),
                Native,
            ),
            Inductor => ("rspice.native.inductor", Some("inductor.svg"), Native),
            Transformer => (
                "rspice.synthesized.transformer",
                Some("transformer_symmetrical.svg"),
                Synthesized,
            ),
            CoupledInductor => (
                "rspice.native.coupled_inductor",
                Some("k_coupling.svg"),
                Native,
            ),
            Diode => ("rspice.native.diode", Some("diode.svg"), Native),
            NpnBjt => ("rspice.native.npn_bjt", Some("bjt_npn.svg"), Native),
            PnpBjt => ("rspice.native.pnp_bjt", Some("bjt_pnp.svg"), Native),
            Nmos => (
                "rspice.native.nmos",
                Some("mos_n_chan_enh_no_substrate.svg"),
                Native,
            ),
            Pmos => (
                "rspice.native.pmos",
                Some("mos_p_chan_enh_no_substrate.svg"),
                Native,
            ),
            Njfet => ("rspice.native.njfet", Some("jfet_n_chan.svg"), Native),
            Pjfet => ("rspice.native.pjfet", Some("jfet_p_chan.svg"), Native),
            Nmesfet => ("rspice.native.nmesfet", Some("mesfet_n_chan.svg"), Native),
            Pmesfet => ("rspice.native.pmesfet", Some("mesfet_p_chan.svg"), Native),
            NVdmos => (
                "rspice.native.n_vdmos",
                Some("mos_n_chan_enh_body_diode_discrete.svg"),
                Native,
            ),
            PVdmos => (
                "rspice.native.p_vdmos",
                Some("mos_p_chan_enh_body_diode_discrete.svg"),
                Native,
            ),
            NmosSoi => ("rspice.native.nmos_soi", Some("mos_n_soi.svg"), Native),
            PmosSoi => ("rspice.native.pmos_soi", Some("mos_p_soi.svg"), Native),
            NpnBjt4 => (
                "rspice.native.npn_bjt_4t",
                Some("bjt_npn_substrate.svg"),
                Native,
            ),
            PnpBjt4 => (
                "rspice.native.pnp_bjt_4t",
                Some("bjt_pnp_substrate.svg"),
                Native,
            ),
            NpnBjt5 => (
                "rspice.native.npn_bjt_5t",
                Some("bjt_npn_thermal.svg"),
                Native,
            ),
            PnpBjt5 => (
                "rspice.native.pnp_bjt_5t",
                Some("bjt_pnp_thermal.svg"),
                Native,
            ),
            SaturableInductor => (
                "rspice.native.saturable_inductor",
                Some("inductor_saturable.svg"),
                Native,
            ),
            Memristor => ("rspice.native.memristor", Some("memristor.svg"), Native),
            VoltageSource => (
                "rspice.native.voltage_source.dc",
                Some("v_src_dc.svg"),
                Native,
            ),
            CurrentSource => ("rspice.native.current_source.dc", Some("i_src.svg"), Native),
            VoltageSourceAc => (
                "rspice.native.voltage_source.ac",
                Some("v_src_ac_vertical.svg"),
                Native,
            ),
            VoltageSourcePulse => (
                "rspice.native.voltage_source.pulse",
                Some("v_src_pulse.svg"),
                Native,
            ),
            VoltageSourceSin => (
                "rspice.native.voltage_source.sin",
                Some("v_src_ac_vertical.svg"),
                Native,
            ),
            VoltageSourcePwl => (
                "rspice.native.voltage_source.pwl",
                Some("v_src_pwl.svg"),
                Native,
            ),
            VoltageSourcePwlFile => (
                "rspice.native.voltage_source.pwl_file",
                Some("v_src_pwl_file.svg"),
                Native,
            ),
            VoltageSourceExp => (
                "rspice.native.voltage_source.exp",
                Some("v_src_exp.svg"),
                Native,
            ),
            VoltageSourceSffm => (
                "rspice.native.voltage_source.sffm",
                Some("v_src_sffm.svg"),
                Native,
            ),
            VoltageSourceAm => (
                "rspice.native.voltage_source.am",
                Some("v_src_am.svg"),
                Native,
            ),
            VoltageSourcePat => (
                "rspice.native.voltage_source.pat",
                Some("v_src_pat.svg"),
                Native,
            ),
            VoltageSourceNoise => (
                "rspice.native.voltage_source.noise",
                Some("v_src_noise.svg"),
                Native,
            ),
            CurrentSourceAc => (
                "rspice.native.current_source.ac",
                Some("i_src_ac.svg"),
                Native,
            ),
            CurrentSourcePulse => (
                "rspice.native.current_source.pulse",
                Some("i_src_pulse.svg"),
                Native,
            ),
            CurrentSourceSin => (
                "rspice.native.current_source.sin",
                Some("i_src_ac.svg"),
                Native,
            ),
            CurrentSourcePwl => (
                "rspice.native.current_source.pwl",
                Some("i_src_pwl.svg"),
                Native,
            ),
            CurrentSourcePwlFile => (
                "rspice.native.current_source.pwl_file",
                Some("i_src_pwl_file.svg"),
                Native,
            ),
            CurrentSourceExp => (
                "rspice.native.current_source.exp",
                Some("i_src_exp.svg"),
                Native,
            ),
            CurrentSourceSffm => (
                "rspice.native.current_source.sffm",
                Some("i_src_sffm.svg"),
                Native,
            ),
            CurrentSourceAm => (
                "rspice.native.current_source.am",
                Some("i_src_am.svg"),
                Native,
            ),
            CurrentSourcePat => (
                "rspice.native.current_source.pat",
                Some("i_src_pat.svg"),
                Native,
            ),
            CurrentSourceNoise => (
                "rspice.native.current_source.noise",
                Some("i_src_noise.svg"),
                Native,
            ),
            Vcvs => ("rspice.native.vcvs", Some("vcvs.svg"), Native),
            Vccs => ("rspice.native.vccs", Some("vccs.svg"), Native),
            Ccvs => ("rspice.native.ccvs", Some("ccvs.svg"), Native),
            Cccs => ("rspice.native.cccs", Some("cccs.svg"), Native),
            OpAmp => ("rspice.synthesized.op_amp", Some("opamp.svg"), Synthesized),
            BehavioralSource => ("rspice.native.behavioral_source", Some("b_src.svg"), Native),
            VSwitch => (
                "rspice.native.voltage_switch",
                Some("switch_voltage.svg"),
                Native,
            ),
            ISwitch => (
                "rspice.native.current_switch",
                Some("switch_current.svg"),
                Native,
            ),
            GenericSwitch => (
                "rspice.native.expression_switch",
                Some("switch_expression.svg"),
                Native,
            ),
            TransmissionLine => (
                "rspice.native.transmission_line.lossless",
                Some("transmission_line.svg"),
                Native,
            ),
            LossyTransmissionLine => (
                "rspice.native.transmission_line.lossy",
                Some("tline_lossy.svg"),
                Native,
            ),
            CoupledTransmissionLine => (
                "rspice.native.transmission_line.coupled",
                Some("tline_coupled.svg"),
                Native,
            ),
            RfPort => (
                "rspice.synthesized.rf_port",
                Some("rf_port.svg"),
                Synthesized,
            ),
            LoopProbe => (
                "rspice.synthesized.loop_probe",
                Some("loop_probe.svg"),
                Synthesized,
            ),
            Ground => (
                "rspice.structural.ground",
                Some("ground_signal.svg"),
                Structural,
            ),
            Port => ("rspice.structural.port", Some("port.svg"), Structural),
            CellInstance => ("rspice.library.cell_instance", None, LibraryCell),
            XspiceGain => xspice("rspice.xspice.gain", "gain", "xspice_gain.svg"),
            XspiceSummer => xspice("rspice.xspice.summer", "summer", "xspice_summer.svg"),
            XspiceMultiplier => xspice("rspice.xspice.mult", "mult", "xspice_multiplier.svg"),
            XspiceDivider => xspice("rspice.xspice.divide", "divide", "xspice_divider.svg"),
            XspiceLimiter => xspice("rspice.xspice.limit", "limit", "xspice_limiter.svg"),
            XspiceIntegrator => xspice("rspice.xspice.int", "int", "xspice_integrator.svg"),
            XspiceDifferentiator => {
                xspice("rspice.xspice.d_dt", "d_dt", "xspice_differentiator.svg")
            }
            XspiceInverter => xspice(
                "rspice.xspice.d_inverter",
                "d_inverter",
                "xspice_inverter.svg",
            ),
            XspiceBuffer => xspice("rspice.xspice.d_buffer", "d_buffer", "xspice_buffer.svg"),
            XspiceAndGate => xspice("rspice.xspice.d_and", "d_and", "xspice_and.svg"),
            XspiceOrGate => xspice("rspice.xspice.d_or", "d_or", "xspice_or.svg"),
            XspiceNandGate => xspice("rspice.xspice.d_nand", "d_nand", "xspice_nand.svg"),
            XspiceNorGate => xspice("rspice.xspice.d_nor", "d_nor", "xspice_nor.svg"),
            XspiceXorGate => xspice("rspice.xspice.d_xor", "d_xor", "xspice_xor.svg"),
            XspiceTristate => xspice(
                "rspice.xspice.d_tristate",
                "d_tristate",
                "xspice_tristate.svg",
            ),
            XspiceDFlipFlop => xspice("rspice.xspice.d_dff", "d_dff", "xspice_dff.svg"),
            XspiceJkFlipFlop => xspice("rspice.xspice.d_jkff", "d_jkff", "xspice_jkff.svg"),
            XspiceSrLatch => xspice(
                "rspice.xspice.d_srlatch",
                "d_srlatch",
                "xspice_sr_latch.svg",
            ),
            XspiceAdcBridge => xspice(
                "rspice.xspice.adc_bridge",
                "adc_bridge",
                "xspice_adc_bridge.svg",
            ),
            XspiceDacBridge => xspice(
                "rspice.xspice.dac_bridge",
                "dac_bridge",
                "xspice_dac_bridge.svg",
            ),
        };

        DeviceDescriptor {
            stable_id,
            default_symbol_asset,
            implementation,
        }
    }
}

const fn xspice(
    stable_id: &'static str,
    model_type: &'static str,
    symbol: &'static str,
) -> (&'static str, Option<&'static str>, DeviceImplementation) {
    (
        stable_id,
        Some(symbol),
        DeviceImplementation::Xspice { model_type },
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn built_in_stable_ids_are_unique_and_portable() {
        let mut ids = HashSet::new();
        for kind in ComponentType::ALL {
            let id = kind.descriptor().stable_id;
            assert!(ids.insert(id), "duplicate stable device id: {id}");
            assert!(
                id.bytes().all(|byte| byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_')),
                "non-portable stable device id: {id}"
            );
        }
    }

    #[test]
    fn every_directly_placeable_legacy_kind_has_a_default_symbol() {
        for kind in ComponentType::ALL {
            let descriptor = kind.descriptor();
            if descriptor.implementation != DeviceImplementation::LibraryCell {
                assert!(
                    descriptor.default_symbol_asset.is_some(),
                    "{} has no default symbol asset",
                    descriptor.stable_id
                );
            }
        }
    }

    #[test]
    fn xspice_descriptors_have_unique_canonical_model_types() {
        let mut model_types = HashSet::new();
        for kind in ComponentType::ALL {
            if let DeviceImplementation::Xspice { model_type } = kind.descriptor().implementation {
                assert!(
                    model_types.insert(model_type),
                    "duplicate XSPICE model type: {model_type}"
                );
            }
        }
        assert_eq!(model_types.len(), 20);
    }
}

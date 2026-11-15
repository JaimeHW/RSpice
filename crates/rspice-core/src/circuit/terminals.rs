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

    /// Visit the node on each terminal of every native bipolar transistor:
    /// collector, base, emitter and substrate.
    pub(crate) fn visit_bipolar_transistor_terminals(&self, visit: &mut impl FnMut(NodeId)) {
        for bjt in &self.bjts.devices {
            for node in [
                bjt.node_collector,
                bjt.node_base,
                bjt.node_emitter,
                bjt.node_substrate,
            ] {
                visit(node);
            }
        }
    }

    /// Visit the node on each terminal of every transistor in the circuit:
    /// every device whose conduction between two terminals a third one
    /// controls, which is where a circuit switches.
    ///
    /// This is the device set the transient activity bound watches
    /// ([`crate::constants::DEVICE_ACTIVITY_STEP_BOUND`]), so it is complete
    /// by construction rather than by family: every native transistor family,
    /// and every generated compact model with three or more electrical
    /// terminals. A two-terminal device (a diode, varactor or resistor model)
    /// has no control terminal, and its own charge truncation already follows
    /// the one voltage it has.
    ///
    /// The terminals are the device's own electrical connections: a family
    /// that folds its series resistors into the device reports the nodes its
    /// intrinsic model sits on, a thermal port is not a voltage, and a
    /// generated model's terminal the card left unconnected is a private node
    /// no deck net shares. A node shared by several devices is visited once
    /// for each; ground is visited too, and callers skip it.
    pub(crate) fn visit_transistor_terminals(&self, visit: &mut impl FnMut(NodeId)) {
        self.visit_bipolar_transistor_terminals(visit);
        for mos in &self.mosfets.devices {
            for node in [
                mos.node_drain,
                mos.node_gate,
                mos.node_source,
                mos.node_bulk,
            ] {
                visit(node);
            }
        }
        for jfet in &self.jfets {
            for node in [jfet.drain, jfet.gate, jfet.source] {
                visit(node);
            }
        }
        for mos in &self.bsim3v3.devices {
            for node in [
                mos.node_drain,
                mos.node_gate,
                mos.node_source,
                mos.node_bulk,
            ] {
                visit(node);
            }
        }
        for mos in &self.bsim4v8.devices {
            for node in [
                mos.node_drain_external,
                mos.node_gate_external,
                mos.node_source_external,
                mos.node_bulk_external,
            ] {
                visit(node);
            }
        }
        // A floating SOI body has no contact, so its `P` node is ground.
        for mos in &self.b3soi.devices {
            for node in [
                mos.node_drain,
                mos.node_gate_external,
                mos.node_source,
                mos.node_e,
                mos.node_p,
            ] {
                visit(node);
            }
        }
        for mos in &self.b3soi_pd.devices {
            for node in [
                mos.node_drain,
                mos.node_gate,
                mos.node_source,
                mos.node_e,
                mos.node_p,
            ] {
                visit(node);
            }
        }
        // A fully depleted body is pinned, never stamped: its contact is not
        // a terminal of the device.
        for mos in &self.b3soi_fd.devices {
            for node in [mos.node_drain, mos.node_gate, mos.node_source, mos.node_e] {
                visit(node);
            }
        }
        for mos in &self.ekv26s.devices {
            for node in [
                mos.node_drain,
                mos.node_gate,
                mos.node_source,
                mos.node_bulk,
            ] {
                visit(node);
            }
        }
        for mos in &self.ekv3s.devices {
            for node in [
                mos.node_drain,
                mos.node_gate,
                mos.node_source,
                mos.node_bulk,
            ] {
                visit(node);
            }
        }
        for mos in &self.vdmoses.devices {
            for node in [mos.drain, mos.gate, mos.source, mos.bulk] {
                visit(node);
            }
        }
        #[cfg(feature = "veriloga-builtins-base")]
        for device in self.generated_veriloga_devices.iter() {
            let terminals = device.external_terminals();
            let electrical = |discipline: &str| discipline.eq_ignore_ascii_case("electrical");
            if terminals
                .iter()
                .filter(|terminal| electrical(terminal.discipline))
                .count()
                < 3
            {
                continue;
            }
            for (terminal, &node) in terminals.iter().zip(&device.nodes) {
                if electrical(terminal.discipline)
                    && !self.compact_model_private_nodes.contains(&node)
                {
                    visit(node);
                }
            }
        }
    }
}

//! Signed physical voltages at the model's intrinsic electrical nodes.
//!
//! Read accepted solution coordinates directly: legacy operating-point fields
//! can contain polarity-folded or limited biases, and external pins can sit
//! across lead resistance. Neither is the physical intrinsic voltage needed
//! for a junction rating. Thermal and charge-deficit coordinates are excluded.
use super::*;

impl CircuitData {
    /// Intrinsic voltage observations this concrete implementation can supply.
    /// No external-pin fallback is implied for an absent observation.
    pub fn intrinsic_voltage_catalog(&self) -> HashMap<String, Vec<OpLabel>> {
        let mut catalog = HashMap::<String, Vec<OpLabel>>::new();
        self.visit_intrinsic_voltage_nodes(|name, label, _, _| {
            catalog.entry(name.to_owned()).or_default().push(label);
        });
        catalog
    }

    fn visit_intrinsic_voltage_nodes(&self, mut visit: impl FnMut(&str, OpLabel, NodeId, NodeId)) {
        macro_rules! fet {
            ($name:expr, $d:expr, $g:expr, $s:expr, $b:expr) => {{
                for (label, positive, negative) in [
                    (OpLabel::INTRINSIC_VGS, $g, $s),
                    (OpLabel::INTRINSIC_VDS, $d, $s),
                    (OpLabel::INTRINSIC_VGD, $g, $d),
                    (OpLabel::INTRINSIC_VBS, $b, $s),
                    (OpLabel::INTRINSIC_VBD, $b, $d),
                    (OpLabel::INTRINSIC_VGB, $g, $b),
                ] {
                    visit($name, label, positive, negative);
                }
            }};
        }
        macro_rules! bulk {
            ($devices:expr) => {
                for dev in $devices {
                    fet!(
                        &dev.name,
                        dev.node_drain,
                        dev.node_gate,
                        dev.node_source,
                        dev.node_bulk
                    );
                }
            };
        }
        bulk!(&self.mosfets.devices);
        bulk!(&self.bsim3v3.devices);
        bulk!(&self.bsim4v8.devices);
        bulk!(&self.ekv26s.devices);
        bulk!(&self.ekv3s.devices);
        macro_rules! soi {
            ($devices:expr) => {
                for dev in $devices {
                    fet!(
                        &dev.name,
                        dev.node_drain,
                        dev.node_gate,
                        dev.node_source,
                        dev.node_body
                    );
                    for (label, positive, negative) in [
                        (OpLabel::INTRINSIC_VES, dev.node_e, dev.node_source),
                        (OpLabel::INTRINSIC_VED, dev.node_e, dev.node_drain),
                        (OpLabel::INTRINSIC_VGE, dev.node_gate, dev.node_e),
                        (OpLabel::INTRINSIC_VBODY_BACKGATE, dev.node_body, dev.node_e),
                    ] {
                        visit(&dev.name, label, positive, negative);
                    }
                }
            };
        }
        soi!(&self.b3soi_fd.devices);
        soi!(&self.b3soi.devices);
        soi!(&self.b3soi_pd.devices);
        for dev in &self.vdmoses.devices {
            fet!(
                &dev.name,
                dev.drain_int.unwrap_or(dev.drain),
                dev.gate,
                dev.source_int.unwrap_or(dev.source),
                dev.bulk
            );
        }
        for dev in &self.jfets {
            for (label, positive, negative) in [
                (OpLabel::INTRINSIC_VGS, dev.gate, dev.source),
                (OpLabel::INTRINSIC_VDS, dev.drain, dev.source),
                (OpLabel::INTRINSIC_VGD, dev.gate, dev.drain),
            ] {
                visit(&dev.name, label, positive, negative);
            }
        }
        for dev in &self.bjts.devices {
            // A non-promoted GP has no remaining local series unknowns; its
            // stored terminals already point behind builder-lowered leads.
            let (c, b, e, s) = if dev.mna_promoted() {
                (dev.node_ci, dev.node_bi, dev.node_ei, dev.node_si)
            } else {
                (
                    dev.node_collector,
                    dev.node_base,
                    dev.node_emitter,
                    dev.node_substrate,
                )
            };
            for (label, positive, negative) in [
                (OpLabel::INTRINSIC_VBE, b, e),
                (OpLabel::INTRINSIC_VCE, c, e),
                (OpLabel::INTRINSIC_VBC, b, c),
            ] {
                visit(&dev.name, label, positive, negative);
            }
            if !dev.uses_three_terminal_vbic() {
                for (label, positive) in [
                    (OpLabel::INTRINSIC_VCSUB, c),
                    (OpLabel::INTRINSIC_VBSUB, b),
                    (OpLabel::INTRINSIC_VESUB, e),
                ] {
                    visit(&dev.name, label, positive, s);
                }
            }
        }
        for dev in &self.diodes.devices {
            visit(
                &dev.name,
                OpLabel::INTRINSIC_VAK,
                dev.node_anode,
                dev.node_cathode,
            );
        }
    }

    pub(super) fn add_intrinsic_voltage_observations(
        &self,
        report: &mut DeviceOpReport,
        solution: &[Value],
    ) -> Result<(), String> {
        let mut entries = report
            .entries
            .iter_mut()
            .map(|entry| (entry.name.clone(), entry))
            .collect::<HashMap<_, _>>();
        let voltage = |node: NodeId| {
            if node == 0 {
                Some(0.0)
            } else {
                solution.get(node - 1).copied()
            }
        };
        let mut error = None;
        self.visit_intrinsic_voltage_nodes(|name, label, positive, negative| {
            let value = voltage(positive).zip(voltage(negative)).map(|(p, n)| p - n);
            match (
                entries.get_mut(name),
                value.filter(|value| value.is_finite()),
            ) {
                (Some(entry), Some(value)) => entry.params.push((label.as_str(), value)),
                _ => {
                    error = Some(format!(
                        "device '{name}' has no finite {} observation at its intrinsic nodes",
                        label.as_str()
                    ))
                }
            }
        });
        error.map_or(Ok(()), Err)
    }
}

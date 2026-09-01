//! Read-only queries over an assembled [`CircuitData`].
//!
//! Device and node counts, sorted node and branch name listings, matrix and
//! RHS allocation sized from the topology, and the per-device operating-point
//! summary ([`DeviceOpEntry`] / [`DeviceOpReport`]) that frontends render as
//! an `.OP` table. Nothing here mutates circuit state.

use super::*;

/// One device's operating-point summary line.
#[derive(Debug, Clone)]
pub struct DeviceOpEntry {
    /// Instance name as written in the netlist (hierarchical after flatten).
    pub name: String,
    /// Device family label ("MOSFET", "BJT", "DIODE", ...).
    pub device_kind: &'static str,
    /// Operating region, when the device family defines one.
    pub region: Option<&'static str>,
    /// Named operating-point quantities in display order.
    pub params: Vec<(&'static str, Value)>,
}

impl DeviceOpEntry {
    /// Build an entry from the canonical label vocabulary.
    ///
    /// Labels arrive typed rather than as free strings because frontends
    /// persist this report and restore it by interning: text outside
    /// [`OP_LABELS`] is text the reader cannot turn back into a label, so an
    /// emitter is given no way to write it. Reporting a new quantity therefore
    /// starts by naming it in the vocabulary, which is what keeps the reader in
    /// step.
    pub fn new(
        name: String,
        device_kind: OpLabel,
        region: Option<OpLabel>,
        params: Vec<(OpLabel, Value)>,
    ) -> Self {
        Self {
            name,
            device_kind: device_kind.as_str(),
            region: region.map(OpLabel::as_str),
            params: params
                .into_iter()
                .map(|(label, value)| (label.as_str(), value))
                .collect(),
        }
    }

    /// True when every label this entry carries can be read back.
    ///
    /// The typed constructor already guarantees it; this re-checks the finished
    /// entry so a struct literal written inside the crate, or a generated
    /// catalog label with no catalog behind it, cannot slip past unnoticed.
    pub fn labels_resolve(&self) -> bool {
        resolve_op_label(self.device_kind).is_some()
            && self
                .region
                .is_none_or(|region| resolve_op_label(region).is_some())
            && self
                .params
                .iter()
                .all(|(name, _)| resolve_op_label(name).is_some())
    }
}

/// Per-device operating-point report for an entire solved circuit —
/// the data behind a Spectre-style OP info table.
#[derive(Debug, Clone, Default)]
pub struct DeviceOpReport {
    pub entries: Vec<DeviceOpEntry>,
}

impl DeviceOpReport {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// True when every label in the report can be read back by a frontend that
    /// persisted it. See [`DeviceOpEntry::labels_resolve`].
    pub fn labels_resolve(&self) -> bool {
        self.entries.iter().all(DeviceOpEntry::labels_resolve)
    }
}

impl CircuitData {
    fn accumulate_floating_component_current(
        component_by_node: &[Option<usize>],
        net_current: &mut [Value],
        current_scale: &mut [Value],
        node_pos: NodeId,
        node_neg: NodeId,
        current: Value,
    ) {
        let component_of = |node: NodeId| component_by_node.get(node).copied().flatten();
        let component_pos = component_of(node_pos);
        let component_neg = component_of(node_neg);
        if component_pos.is_some() && component_pos == component_neg {
            // A source wholly inside one conductive component establishes
            // differential behavior but contributes neither net current nor
            // tolerance scale to that component's common-mode KCL.
            return;
        }
        let mut add = |node: NodeId, signed_current: Value| {
            let Some(Some(component)) = component_by_node.get(node) else {
                return;
            };
            net_current[*component] += signed_current;
            current_scale[*component] += current.abs();
        };
        add(node_pos, current);
        add(node_neg, -current);
    }

    fn driven_floating_component_nodes(
        &self,
        net_current: &[Value],
        current_scale: &[Value],
        abstol: Value,
        reltol: Value,
    ) -> Vec<String> {
        self.dc_floating_component_nodes
            .iter()
            .enumerate()
            .filter(|(component, _)| {
                if !self
                    .dc_floating_component_is_certain
                    .get(*component)
                    .copied()
                    .unwrap_or(false)
                {
                    return false;
                }
                let net = net_current[*component];
                let scale = current_scale[*component];
                !net.is_finite() || net.abs() > abstol.max(0.0) + reltol.max(0.0) * scale.abs()
            })
            .flat_map(|(_, nodes)| nodes.iter().cloned())
            .collect()
    }

    /// Floating components with a nonzero net installed independent-source
    /// current. This is safe before solving and follows the circuit-owned DC
    /// values, including loaded PWL/PAT snapshots and live `.DC` sweep values.
    pub(crate) fn independent_dc_drive_nodes(&self, abstol: Value, reltol: Value) -> Vec<String> {
        let component_count = self.dc_floating_component_nodes.len();
        let mut net_current = vec![0.0; component_count];
        let mut current_scale = vec![0.0; component_count];
        for index in 0..self.current_sources.len() {
            Self::accumulate_floating_component_current(
                &self.dc_floating_component_by_node,
                &mut net_current,
                &mut current_scale,
                self.current_sources.node_pos[index],
                self.current_sources.node_neg[index],
                self.current_sources.dc_values[index],
            );
        }
        self.driven_floating_component_nodes(&net_current, &current_scale, abstol, reltol)
    }

    /// Nodes that no chain of DC-conducting elements ties to ground.
    ///
    /// Their DC voltage is set by the solver's conditioning shunt rather than
    /// by the circuit, so an operating point reported for them would be an
    /// artifact of the shunt's size. Empty when the circuit is sound or when
    /// its topology could not be analyzed.
    pub fn no_dc_path_nodes(&self) -> &[String] {
        &self.no_dc_path_nodes
    }

    /// Floating-component identifier for one nodal matrix row.
    pub(crate) fn dc_floating_component_for_matrix_row(&self, row: usize) -> Option<usize> {
        let component = self
            .dc_floating_component_by_node
            .get(row.checked_add(1)?)
            .copied()
            .flatten()?;
        self.dc_floating_component_is_certain
            .get(component)
            .copied()
            .unwrap_or(false)
            .then_some(component)
    }

    /// Nodal matrix rows belonging to one statically identified floating
    /// component after final node remapping.
    pub(crate) fn dc_floating_component_matrix_rows(&self, component: usize) -> Vec<usize> {
        self.dc_floating_component_by_node
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(node, candidate)| (*candidate == Some(component)).then_some(node - 1))
            .collect()
    }

    /// No-DC-path nodes in components driven by a current-source equation.
    ///
    /// Unlike an unforced capacitive island, these nodes cannot be assigned a
    /// finite operating point without making the result depend on the solver's
    /// conditioning shunt. The DC driver rejects them unless the deck supplies
    /// an explicit global shunt through `.OPTIONS RSHUNT`.
    pub fn fatal_no_dc_path_nodes(&self) -> &[String] {
        &self.fatal_no_dc_path_nodes
    }

    /// Conductance installed from `.OPTIONS RSHUNT`, in siemens.
    ///
    /// This is part of the assembled physical circuit and is zero when the
    /// option is absent. It is kept separate from numerical GMIN so callers
    /// can explain effective topology and checkpoint identity precisely.
    pub fn global_shunt_conductance(&self) -> Value {
        self.global_shunt_conductance
    }

    /// Whether `.OPTIONS RSHUNT` installed a physical shunt on electrical nodes.
    pub fn has_global_shunt(&self) -> bool {
        self.global_shunt_conductance > 0.0
    }

    /// Whether a matrix row inside the nodal prefix represents a private
    /// non-electrical DAE state rather than an electrical node voltage.
    ///
    /// Solver-wide electrical aids such as nodal GMIN and voltage clamps must
    /// not alter these rows. The state still lives in the nodal prefix so it
    /// can reuse the sparse topology and capacitor companion infrastructure.
    pub(crate) fn is_non_electrical_state_matrix_index(&self, index: usize) -> bool {
        self.non_electrical_state_nodes.contains(&(index + 1))
    }

    pub(crate) fn non_electrical_state_mask(&self) -> Vec<bool> {
        let mut mask = vec![false; self.num_nodes()];
        for node in &self.non_electrical_state_nodes {
            if *node > 0
                && let Some(slot) = mask.get_mut(*node - 1)
            {
                *slot = true;
            }
        }
        mask
    }

    /// Linearize every behavioral source at the DC operating point so the
    /// small-signal (AC/noise/sensitivity) assembly can read the cached
    /// partials immutably. One-shot per analysis for frequency-invariant
    /// expressions; frequency-dependent sources are selectively refreshed
    /// at each analysis point.
    pub(crate) fn prepare_behavioral_small_signal(
        &mut self,
        dc_solution: &[Value],
    ) -> Result<(), String> {
        for source in &mut self.behavioral_sources.voltage_sources {
            source
                .linearize_at(dc_solution)
                .map_err(|error| error.to_string())?;
        }
        for source in &mut self.behavioral_sources.current_sources {
            source
                .linearize_at(dc_solution)
                .map_err(|error| error.to_string())?;
        }
        self.capacitors
            .prepare_solution_dependent_small_signal(dc_solution, 0.0)
    }

    /// Refresh behavioral-source Jacobians for one AC frequency point.
    ///
    /// Most behavioral expressions are frequency-independent. Re-evaluating
    /// here is nevertheless required for Xyce `FREQ`/`HERTZ` parameter
    /// graphs and behavioralized passive values, whose small-signal
    /// conductance can change at every point.
    pub(crate) fn prepare_behavioral_small_signal_at_frequency(
        &mut self,
        dc_solution: &[Value],
        frequency: Value,
    ) -> Result<(), String> {
        for source in &mut self.behavioral_sources.voltage_sources {
            if source.is_frequency_dependent() {
                source
                    .linearize_at_frequency(dc_solution, frequency)
                    .map_err(|error| error.to_string())?;
            }
        }
        for source in &mut self.behavioral_sources.current_sources {
            if source.is_frequency_dependent() {
                source
                    .linearize_at_frequency(dc_solution, frequency)
                    .map_err(|error| error.to_string())?;
            }
        }
        self.capacitors
            .prepare_solution_dependent_small_signal(dc_solution, frequency)
    }

    /// Linearize every behavioral source at an arbitrary state and active
    /// frequency. Distortion uses this for nearby bias states, where even a
    /// frequency-invariant expression must be refreshed because its inputs
    /// have changed.
    pub(crate) fn prepare_behavioral_small_signal_state_at_frequency(
        &mut self,
        state: &[Value],
        frequency: Value,
    ) -> Result<(), String> {
        for source in &mut self.behavioral_sources.voltage_sources {
            source
                .linearize_at_state_and_frequency(state, frequency)
                .map_err(|error| error.to_string())?;
        }
        for source in &mut self.behavioral_sources.current_sources {
            source
                .linearize_at_state_and_frequency(state, frequency)
                .map_err(|error| error.to_string())?;
        }
        self.capacitors
            .prepare_solution_dependent_small_signal(state, frequency)
    }

    /// Build the per-device operating-point report from the device state
    /// cached by the last accepted Newton solution. Call after a DC
    /// operating point (or any analysis that leaves devices at a solution).
    pub fn device_op_report(&self) -> DeviceOpReport {
        let mut entries = Vec::new();

        for mosfet in &self.mosfets.devices {
            let op = mosfet.op_values();
            entries.push(DeviceOpEntry::new(
                mosfet.name.clone(),
                mosfet.device_kind(),
                Some(op.region),
                vec![
                    (OpLabel::ID, op.id),
                    (OpLabel::VGS, op.vgs),
                    (OpLabel::VDS, op.vds),
                    (OpLabel::VBS, op.vbs),
                    (OpLabel::VTH, op.vth),
                    (OpLabel::VDSAT, op.vdsat),
                    (OpLabel::GM, op.gm),
                    (OpLabel::GDS, op.gds),
                    (OpLabel::GMB, op.gmb),
                ],
            ));
        }

        for dev in &self.bsim3v3.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry::new(
                dev.name.clone(),
                OpLabel::BSIM3,
                Some(region),
                vec![
                    (OpLabel::ID, id),
                    (OpLabel::VGS, vgs),
                    (OpLabel::VDS, vds),
                    (OpLabel::VBS, vbs),
                    (OpLabel::VTH, vth),
                    (OpLabel::VDSAT, vdsat),
                    (OpLabel::GM, gm),
                    (OpLabel::GDS, gds),
                    (OpLabel::GMB, gmbs),
                ],
            ));
        }

        for dev in &self.bsim4v8.devices {
            let (id, vgs, vds, vbs, vth, vdsat, output_vdsat, gm, gds, gmbs, region) =
                dev.op_values();
            entries.push(DeviceOpEntry::new(
                dev.name.clone(),
                OpLabel::BSIM4,
                Some(region),
                vec![
                    (OpLabel::ID, id),
                    (OpLabel::VGS, vgs),
                    (OpLabel::VDS, vds),
                    (OpLabel::VBS, vbs),
                    (OpLabel::VTH, vth),
                    (OpLabel::VDSAT, vdsat),
                    (OpLabel::OUTPUT_VDSAT, output_vdsat),
                    (OpLabel::GM, gm),
                    (OpLabel::GDS, gds),
                    (OpLabel::GMB, gmbs),
                ],
            ));
        }

        for dev in &self.b3soi_fd.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry::new(
                dev.name.clone(),
                OpLabel::B3SOIFD,
                Some(region),
                vec![
                    (OpLabel::ID, id),
                    (OpLabel::IS, -id),
                    (OpLabel::VGS, vgs),
                    (OpLabel::VDS, vds),
                    (OpLabel::VBS, vbs),
                    (OpLabel::VTH, vth),
                    (OpLabel::VDSAT, vdsat),
                    (OpLabel::GM, gm),
                    (OpLabel::GDS, gds),
                    (OpLabel::GMB, gmbs),
                ],
            ));
        }

        for dev in &self.b3soi.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry::new(
                dev.name.clone(),
                OpLabel::B3SOIDD,
                Some(region),
                vec![
                    (OpLabel::ID, id),
                    (OpLabel::IS, -id),
                    (OpLabel::VGS, vgs),
                    (OpLabel::VDS, vds),
                    (OpLabel::VBS, vbs),
                    (OpLabel::VTH, vth),
                    (OpLabel::VDSAT, vdsat),
                    (OpLabel::GM, gm),
                    (OpLabel::GDS, gds),
                    (OpLabel::GMB, gmbs),
                ],
            ));
        }

        for dev in &self.b3soi_pd.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry::new(
                dev.name.clone(),
                OpLabel::B3SOIPD,
                Some(region),
                vec![
                    (OpLabel::ID, id),
                    (OpLabel::IS, -id),
                    (OpLabel::VGS, vgs),
                    (OpLabel::VDS, vds),
                    (OpLabel::VBS, vbs),
                    (OpLabel::VTH, vth),
                    (OpLabel::VDSAT, vdsat),
                    (OpLabel::GM, gm),
                    (OpLabel::GDS, gds),
                    (OpLabel::GMB, gmbs),
                ],
            ));
        }

        for dev in &self.ekv26s.devices {
            let op = dev.op_values();
            entries.push(DeviceOpEntry::new(
                dev.name.clone(),
                OpLabel::EKV26,
                None,
                vec![
                    (OpLabel::ID, op.id),
                    (OpLabel::VGS, op.vgs),
                    (OpLabel::VDS, op.vds),
                    (OpLabel::VBS, op.vbs),
                ],
            ));
        }

        for dev in &self.ekv3s.devices {
            let op = dev.op_values();
            entries.push(DeviceOpEntry::new(
                dev.name.clone(),
                OpLabel::EKV3,
                None,
                vec![
                    (OpLabel::ID, op.id),
                    (OpLabel::VGS, op.vgs),
                    (OpLabel::VDS, op.vds),
                    (OpLabel::VBS, op.vbs),
                    (OpLabel::GM, op.gm),
                ],
            ));
        }

        for vdmos in &self.vdmoses.devices {
            let (id, vgs, vds, diode_id, power, region) = vdmos.op_values();
            entries.push(DeviceOpEntry::new(
                vdmos.name.clone(),
                OpLabel::VDMOS,
                Some(region),
                vec![
                    (OpLabel::ID, id),
                    (OpLabel::VGS, vgs),
                    (OpLabel::VDS, vds),
                    (OpLabel::IDIODE, diode_id),
                    (OpLabel::POWER, power),
                ],
            ));
        }

        for bjt in &self.bjts.devices {
            let (vbe, vbc, ic, ib, gm) = bjt.op_values();
            let [_, _, ie, is] = bjt.operating_point_terminal_currents();
            let beta = if ib.abs() > 1e-30 { ic / ib } else { 0.0 };
            entries.push(DeviceOpEntry::new(
                bjt.name.clone(),
                OpLabel::BJT,
                None,
                vec![
                    (OpLabel::IC, ic),
                    (OpLabel::IB, ib),
                    (OpLabel::IE, ie),
                    (OpLabel::IS, is),
                    (OpLabel::VBE, vbe),
                    (OpLabel::VCE, vbe - vbc),
                    (OpLabel::BETA, beta),
                    (OpLabel::GM, gm),
                ],
            ));
        }

        for diode in &self.diodes.devices {
            let (vd, id, gd, cd) = diode.op_values();
            entries.push(DeviceOpEntry::new(
                diode.name.clone(),
                OpLabel::DIODE,
                None,
                vec![
                    (OpLabel::VD, vd),
                    (OpLabel::ID, id),
                    (OpLabel::GD, gd),
                    (OpLabel::CD, cd),
                ],
            ));
        }

        // Xyce LEVEL=2 self-consistent thermal resistors expose their
        // material resistance and temperature through the ordinary device-op
        // channel.  Ordinary scalar resistors remain derived branch values;
        // only devices with an actual thermal state are reported here.
        for (index, name) in self.resistors.names.iter().enumerate() {
            let Some(state) = self.resistors.thermal.get(index).and_then(Option::as_ref) else {
                continue;
            };
            entries.push(DeviceOpEntry::new(
                name.clone(),
                OpLabel::RESISTOR,
                None,
                vec![
                    (OpLabel::R, state.output_resistance),
                    (OpLabel::TEMP, state.temperature_celsius),
                ],
            ));
        }

        for jfet in &self.jfets {
            let (vgs, vds, ids, gm, gds, igs, igd) = jfet.op_values();
            let device_kind = match jfet.params.channel_model {
                crate::device::JfetChannelModel::ShichmanHodges
                | crate::device::JfetChannelModel::XyceSydney => OpLabel::JFET,
                crate::device::JfetChannelModel::ParkerSkellern => OpLabel::JFET2,
                crate::device::JfetChannelModel::XyceModifiedShockley => OpLabel::JFET2_XYCE,
                crate::device::JfetChannelModel::LegacyMesfet => OpLabel::MESFET,
                crate::device::JfetChannelModel::Hfet1 if jfet.params.hfet_level == 6 => {
                    OpLabel::HFET2
                }
                crate::device::JfetChannelModel::Hfet1 => OpLabel::HFET1,
            };
            entries.push(DeviceOpEntry::new(
                jfet.name.clone(),
                device_kind,
                None,
                vec![
                    (OpLabel::ID, ids),
                    (OpLabel::VGS, vgs),
                    (OpLabel::VDS, vds),
                    (OpLabel::GM, gm),
                    (OpLabel::GDS, gds),
                    (OpLabel::IGS, igs),
                    (OpLabel::IGD, igd),
                ],
            ));
        }

        // A solution-dependent capacitor exposes its effective instantaneous
        // C value through the same operating-point trace channel used by
        // compact-model parameters. The transient engine refreshes this value
        // on every accepted point before collecting the report.
        for (index, name) in self.capacitors.names.iter().enumerate() {
            if self.capacitors.is_internal(index)
                || self
                    .capacitors
                    .value_expressions
                    .get(index)
                    .and_then(Option::as_ref)
                    .is_none()
            {
                continue;
            }
            entries.push(DeviceOpEntry::new(
                name.clone(),
                OpLabel::CAPACITOR,
                None,
                vec![(
                    OpLabel::C,
                    self.capacitors
                        .effective_capacitances
                        .get(index)
                        .copied()
                        .unwrap_or(Value::NAN),
                )],
            ));
        }

        // Xyce nonlinear-core internal probes are exposed through the
        // canonical `YMIN!<K-name>` namespace.  Xyce LEVEL=1 reports M in
        // kA/m while LEVEL=2 reports M in A/m; both use Oersted/Gauss for H/B
        // unless BHSIUNITS=1 switches those fields back to SI units.  Keep
        // the native model constants (and their evaluation order) here:
        // MutIndNonLin2 initializes HCgsFactor to this decimal literal and
        // forms B as BCgsFactor * ((4e-7*pi) * (H + M)).
        const XYCE_H_CGS_FACTOR: Value = 0.012566370614359;
        const XYCE_B_CGS_FACTOR: Value = 10000.0;
        const XYCE_MU_0: Value = 4.0e-7 * std::f64::consts::PI;
        for binding in &self.jiles_atherton_inductors {
            let Some(name) = binding.core_output_name.as_deref() else {
                continue;
            };
            let h_si = binding.device.magnetic_field();
            let b_output = if binding.core_bh_si_units {
                binding.device.flux_density()
            } else {
                // MutIndNonLin stores H in Oersted before forming its
                // default Gauss-valued B output.  Preserve that canonical
                // Xyce conversion instead of converting the physical SI B
                // directly; the distinction matters for high-field cores.
                XYCE_B_CGS_FACTOR
                    * (XYCE_MU_0
                        * (XYCE_H_CGS_FACTOR * h_si
                            + binding.device.xyce_core_reported_magnetization()))
            };
            entries.push(DeviceOpEntry::new(
                name.to_string(),
                OpLabel::NONLINEAR_CORE,
                None,
                vec![
                    (
                        OpLabel::M,
                        if binding.device.is_xyce_core_level2() {
                            binding.device.xyce_core_reported_magnetization()
                        } else {
                            binding.device.xyce_core_reported_magnetization() / 1.0e3
                        },
                    ),
                    (
                        OpLabel::H,
                        if binding.core_bh_si_units {
                            h_si
                        } else {
                            XYCE_H_CGS_FACTOR * h_si
                        },
                    ),
                    (OpLabel::B, b_output),
                ],
            ));
        }

        // Shared multi-winding Xyce Core devices publish the same canonical
        // YMIN!KNAME namespace as single-winding bindings.  The constitutive
        // state lives on the group device, so report it once per K-card
        // rather than once per winding.
        for binding in &self.xyce_core_groups {
            let name = binding.core_output_name.as_str();
            let h_si = binding.device.magnetic_field();
            let b_output = if binding.core_bh_si_units {
                binding.device.flux_density()
            } else {
                XYCE_B_CGS_FACTOR
                    * (XYCE_MU_0
                        * (XYCE_H_CGS_FACTOR * h_si
                            + binding.device.xyce_core_reported_magnetization()))
            };
            entries.push(DeviceOpEntry::new(
                name.to_string(),
                OpLabel::NONLINEAR_CORE,
                None,
                vec![
                    (
                        OpLabel::M,
                        if binding.device.is_xyce_core_level2() {
                            binding.device.xyce_core_reported_magnetization()
                        } else {
                            binding.device.xyce_core_reported_magnetization() / 1.0e3
                        },
                    ),
                    (
                        OpLabel::H,
                        if binding.core_bh_si_units {
                            h_si
                        } else {
                            XYCE_H_CGS_FACTOR * h_si
                        },
                    ),
                    (OpLabel::B, b_output),
                ],
            ));
        }

        // Generated compact models expose every external lead current from
        // the exact Verilog-A flow contributions captured during stamping.
        // Canonical parameter names come from module-port metadata, never the
        // instance designator or a presumed D/G/S/B position. A compatible
        // SPICE card route may add a small set of explicit conventional
        // aliases (for example diode ID -> current entering terminal `a`).
        //
        // These labels are the one family the fixed vocabulary cannot name,
        // because the catalog compiled into the build decides them. They are
        // resolved back out of that same catalog on read.
        #[cfg(feature = "veriloga-builtins-base")]
        for device in self.generated_veriloga_devices.iter() {
            let currents = device.terminal_currents();
            let terminals = device.external_terminals();
            debug_assert_eq!(terminals.len(), currents.len());
            let mut params = terminals
                .iter()
                .zip(currents.iter().copied())
                .map(|(terminal, current)| {
                    (OpLabel::generated(terminal.current_parameter), current)
                })
                .collect::<Vec<_>>();
            for alias in device.terminal_current_aliases() {
                if params
                    .iter()
                    .any(|(parameter, _)| parameter.as_str().eq_ignore_ascii_case(alias.parameter))
                {
                    continue;
                }
                if let Some((index, _)) = terminals
                    .iter()
                    .enumerate()
                    .find(|(_, terminal)| terminal.name.eq_ignore_ascii_case(alias.terminal))
                    && let Some(current) = currents.get(index).copied()
                {
                    params.push((OpLabel::generated(alias.parameter), current));
                }
            }
            entries.push(DeviceOpEntry::new(
                device.instance_name.clone(),
                OpLabel::generated(device.model_name),
                None,
                params,
            ));
        }

        let report = DeviceOpReport { entries };
        debug_assert!(
            report.labels_resolve(),
            "an operating-point report carries a label no reader can restore; \
             name it in the label vocabulary"
        );
        report
    }

    /// Build an accepted transient report, replacing native diode and legacy
    /// BJT lead currents with the exact totals from their committed
    /// companions. Ordinary `.OP` and every other parameter retain the static
    /// report; these overrides exist only for this transient sample and cannot
    /// leak into a later analysis.
    pub(crate) fn transient_device_op_report(
        &self,
        solution: &[Value],
        accepted_diode_displacement_currents: &[Value],
        accepted_bjt_terminal_currents: &[Option<[Value; 4]>],
    ) -> Result<DeviceOpReport, String> {
        if accepted_diode_displacement_currents.len() != self.diodes.devices.len() {
            return Err(format!(
                "accepted diode current count mismatch: history has {}, circuit has {}",
                accepted_diode_displacement_currents.len(),
                self.diodes.devices.len()
            ));
        }
        let mut report = self.device_op_report();
        for (diode, displacement_current) in self
            .diodes
            .devices
            .iter()
            .zip(accepted_diode_displacement_currents.iter().copied())
        {
            let node_voltage = |node: NodeId| {
                if node == 0 {
                    0.0
                } else {
                    solution.get(node - 1).copied().unwrap_or(0.0)
                }
            };
            let voltage = node_voltage(diode.node_anode) - node_voltage(diode.node_cathode);
            let total_current = diode.stamped_conduction_current(voltage) + displacement_current;
            let entry = report
                .entries
                .iter_mut()
                .find(|entry| {
                    entry
                        .device_kind
                        .eq_ignore_ascii_case(OpLabel::DIODE.as_str())
                        && entry.name.eq_ignore_ascii_case(&diode.name)
                })
                .ok_or_else(|| {
                    format!(
                        "native diode '{}' is missing from its transient operating-point report",
                        diode.name
                    )
                })?;
            let id = entry
                .params
                .iter_mut()
                .find(|(parameter, _)| parameter.eq_ignore_ascii_case(OpLabel::ID.as_str()))
                .ok_or_else(|| {
                    format!(
                        "native diode '{}' transient report is missing ID",
                        diode.name
                    )
                })?;
            id.1 = total_current;
        }
        for (bjt, intrinsic) in self
            .bjts
            .devices
            .iter()
            .zip(accepted_bjt_terminal_currents.iter())
        {
            let Some(intrinsic) = intrinsic else {
                continue;
            };
            let authored = bjt.authored_transient_lead_currents(solution, *intrinsic)?;
            let Some(entry) = report.entries.iter_mut().find(|entry| {
                entry
                    .device_kind
                    .eq_ignore_ascii_case(OpLabel::BJT.as_str())
                    && entry.name.eq_ignore_ascii_case(&bjt.name)
            }) else {
                continue;
            };
            for (parameter, value) in &mut entry.params {
                if parameter.eq_ignore_ascii_case(OpLabel::IC.as_str()) {
                    *value = authored[0];
                } else if parameter.eq_ignore_ascii_case(OpLabel::IB.as_str()) {
                    *value = authored[1];
                } else if parameter.eq_ignore_ascii_case(OpLabel::IE.as_str()) {
                    *value = authored[2];
                } else if parameter.eq_ignore_ascii_case(OpLabel::IS.as_str()) {
                    *value = authored[3];
                }
            }
        }
        Ok(report)
    }

    /// Initial transient samples have zero companion current. Still route the
    /// cached static C/B/E/S values through builder-externalized RC/RB/RE so
    /// t=0 and checkpoint re-anchor samples observe the authored pins.
    pub(crate) fn initial_transient_device_op_report(
        &self,
        solution: &[Value],
    ) -> Result<DeviceOpReport, String> {
        let currents = self
            .bjts
            .devices
            .iter()
            .map(|bjt| {
                bjt.uses_legacy_gummel_poon()
                    .then(|| bjt.operating_point_terminal_currents())
            })
            .collect::<Vec<_>>();
        let diode_displacement_currents = vec![0.0; self.diodes.devices.len()];
        self.transient_device_op_report(solution, &diode_displacement_currents, &currents)
    }

    /// Read-only access to linear resistor storage (names, nodes, conductances).
    pub fn resistor_storage(&self) -> &Resistors {
        &self.resistors
    }

    /// Native diode terminal nodes by instance name, if present.
    pub fn diode_terminal_nodes(&self, name: &str) -> Option<(NodeId, NodeId)> {
        self.diodes
            .devices
            .iter()
            .find(|diode| diode.name.eq_ignore_ascii_case(name))
            .map(|diode| (diode.node_anode, diode.node_cathode))
    }

    /// Read-only access to capacitor storage (names, nodes, capacitances, ICs,
    /// and authored/internal provenance).
    ///
    /// Internal capacitors are canonical integration companions and remain
    /// visible here for diagnostics. Use [`Capacitors::is_internal`] or
    /// [`Capacitors::authored_len`] when presenting authored-device views.
    pub fn capacitor_storage(&self) -> &Capacitors {
        &self.capacitors
    }

    /// Read-only access to inductor storage (names, nodes, inductances, ICs).
    pub fn inductor_storage(&self) -> &Inductors {
        &self.inductors
    }

    /// Read-only access to native diode storage.
    ///
    /// [`Self::diode_terminal_nodes`] answers the common topology question;
    /// this exposes the resolved instances themselves, for callers that need
    /// to see which model parameters a deck actually built into.
    pub fn diode_storage(&self) -> &Diodes {
        &self.diodes
    }

    /// Read-only access to resolved generic-switch instances.
    pub fn generic_switch_storage(&self) -> &[crate::device::GenericSwitch] {
        &self.generic_switches
    }

    /// Get node names sorted by their node index (1, 2, 3, ...)
    /// Returns a Vec where index i contains the name of node (i+1)
    /// This is useful for waveform output labels like V(N001), V(N002)
    pub fn node_names_sorted(&self) -> Vec<String> {
        // Create a vec with one entry per non-ground node
        let mut names: Vec<(NodeId, String)> = self
            .node_map
            .iter()
            .filter(|(_, id)| **id > 0) // Exclude ground (id 0)
            .map(|(name, id)| (*id, name.clone()))
            .collect();

        // Sort by node ID
        names.sort_by_key(|(id, _)| *id);

        // Remove duplicates (keep first occurrence for each ID - in case of aliases like GND/gnd/0)
        names.dedup_by_key(|(id, _)| *id);

        // Extract just the names in order
        names.into_iter().map(|(_, name)| name).collect()
    }

    /// Every elaborated independent voltage and current source, in canonical
    /// case-insensitive name order.
    ///
    /// This is the exact vocabulary the analyses that refer a result to an
    /// excitation accept -- noise input referral resolves its named source
    /// against these two collections and nothing else. Unlike
    /// [`crate::Engine::transient_source_names`] it does not require an
    /// authored transient waveform, so a DC- or AC-only excitation is listed
    /// too.
    pub fn independent_source_names(&self) -> Vec<String> {
        let mut names = self
            .voltage_sources
            .names
            .iter()
            .chain(self.current_sources.names.iter())
            .cloned()
            .collect::<Vec<_>>();
        names.sort_by(|left, right| {
            left.to_ascii_lowercase()
                .cmp(&right.to_ascii_lowercase())
                .then_with(|| left.cmp(right))
        });
        names.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
        names
    }

    /// Get branch names sorted by their branch ordinal (1, 2, 3, ...).
    /// Returns a Vec where index i contains the canonical name of branch (i+1).
    pub fn branch_names_sorted(&self) -> Vec<String> {
        self.branch_name_by_ordinal
            .iter()
            .enumerate()
            .map(|(idx, name)| name.clone().unwrap_or_else(|| format!("BRANCH{}", idx + 1)))
            .collect()
    }

    /// How many device instances this circuit contains.
    ///
    /// One count per element the netlist author wrote, plus every element the
    /// builder synthesizes as a stamping device in its own right rather than
    /// as private state of another one -- a distributed line's lumped
    /// sections, an auto-generated XSPICE bridge.
    ///
    /// A device collection is left out of the sum only when adding it would
    /// count something twice or count something that is not a device. Storage
    /// that re-describes an element another collection already owns is the
    /// first case: an expansion of one authored card into its realization
    /// terms, or a nonlinear overlay bound to a row of ordinary device
    /// storage. Work lists awaiting resolution, frozen load schedules,
    /// per-step residual scratch, and node/topology metadata are the second.
    ///
    /// `tests/device_count_inventory.rs` fails if a `CircuitData` collection
    /// is neither summed here nor named in that test's exclusion list, so this
    /// stays a whole inventory as new device families arrive.
    pub fn device_count(&self) -> usize {
        // Simulator-generated capacitors integrate another device's private
        // state variable, so the capacitor pipeline counts at authored length.
        let passive = self.resistors.len()
            + self.resistor_branches.len()
            + self.capacitors.authored_len()
            + self.inductors.len();
        let sources = self.voltage_sources.len()
            + self.current_sources.len()
            + self.vcvs.len()
            + self.vccs.len()
            + self.cccs.len()
            + self.ccvs.len()
            + self.behavioral_sources.len();
        let junction = self.diodes.len()
            + self.bjts.len()
            + self.mosfets.len()
            + self.bsim3v3.len()
            + self.bsim4v8.len()
            + self.b3soi.len()
            + self.b3soi_fd.len()
            + self.b3soi_pd.len()
            + self.ekv26s.len()
            + self.ekv3s.len()
            + self.vdmoses.len()
            + self.jfets.len()
            + self.xyce_memristors.len();
        let switches = self.vswitches.len() + self.iswitches.len() + self.generic_switches.len();
        let distributed = self.tlines.len() + self.coupled_tlines.len();
        // A K card is one device however the builder realizes it: `couplings`
        // retains the linear cards whose mutual terms live in
        // `coupled_inductor_pairs`, `xyce_core_groups` holds the multi-winding
        // Core cards, and a single-winding Core reaches the circuit only as
        // the overlay owning a `YMIN!` namespace on its winding's inductor.
        let magnetic = self.couplings.len()
            + self.xyce_core_groups.len()
            + self
                .jiles_atherton_inductors
                .iter()
                .filter(|binding| binding.core_output_name.is_some())
                .count();
        let compiled = self.xspice_instances.len();
        let count = passive + sources + junction + switches + distributed + magnetic + compiled;
        #[cfg(feature = "veriloga")]
        let count = count + self.veriloga_devices.len();
        // A mixed Verilog-AMS module is one authored X-card, exactly as an
        // analog Verilog-A instance is, and it is in no other collection: the
        // route that builds one is the branch the device route does not take.
        // Its A/D and D/A bridges are part of the host rather than instances of
        // their own, so counting the hosts counts the cards.
        #[cfg(feature = "veriloga")]
        let count = count + self.mixed_signal_hosts.len();
        #[cfg(feature = "veriloga-builtins-base")]
        let count = count + self.generated_veriloga_devices.len();
        count
    }

    /// Create a triplet matrix for this circuit
    pub fn create_matrix(&self) -> TripletMatrix {
        let size = self.matrix_size();
        TripletMatrix::new(size)
    }

    /// Create RHS vector for this circuit
    pub fn create_rhs(&self) -> Vec<Value> {
        vec![0.0; self.matrix_size()]
    }

    /// Link all device stamps to a StaticMatrix for O(1) stamping
    /// Call this after build_matrix() to bake CSC indices into devices
    pub fn link_indices(&mut self, matrix: &StaticMatrix) {
        // Linear devices
        self.resistors.link_indices(matrix);
        let num_nodes = self.num_nodes;
        self.resistor_branches
            .link_indices(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.capacitors
            .link_indices(matrix, |br_ordinal| num_nodes + br_ordinal);
        self.voltage_sources
            .link_indices(matrix, |br_ordinal| num_nodes + br_ordinal);

        // Nonlinear devices
        self.diodes.link_all(matrix);
        self.bjts.link_all(matrix);
        self.mosfets.link_all(matrix);
        for dev in &mut self.b3soi.devices {
            dev.resolve_instance_ic_branches(num_nodes);
        }
        for dev in &mut self.b3soi_fd.devices {
            dev.resolve_instance_ic_branches(num_nodes);
        }
        for dev in &mut self.b3soi_pd.devices {
            dev.resolve_instance_ic_branches(num_nodes);
        }
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let num_nodes = self.num_nodes;
            self.generated_veriloga_devices_mut()
                .link_static_stamps(matrix, num_nodes);
        }
        for jfet in &mut self.jfets {
            jfet.link(matrix);
        }
        for binding in &mut self.coupled_inductor_pairs {
            let branch1_matrix_index = self.num_nodes + binding.branch1_ordinal;
            let branch2_matrix_index = self.num_nodes + binding.branch2_ordinal;
            binding
                .device
                .set_branches(branch1_matrix_index, branch2_matrix_index);
        }
        for binding in &mut self.multi_winding_transformers {
            let branches: Vec<NodeId> = binding
                .branch_ordinals
                .iter()
                .map(|branch_ordinal| self.num_nodes + *branch_ordinal)
                .collect();
            binding.device.set_branches(branches);
        }
        for binding in &mut self.jiles_atherton_inductors {
            let branch_matrix_index = self.num_nodes + binding.branch_ordinal;
            binding.device.set_branch_index(branch_matrix_index);
        }
        for tline in &mut self.tlines {
            if let Some((branch1, branch2)) = tline
                .txl_branch_ordinals()
                .or_else(|| tline.ltra_branch_ordinals())
                .or_else(|| tline.zero_length_branch_ordinals())
            {
                tline.set_branches(self.num_nodes + branch1, self.num_nodes + branch2);
            }
        }
        for tline in &mut self.coupled_tlines {
            if let Some(branches) = tline.native_branch_ordinals() {
                let matrix_indices = branches.matrix_indices_from_ordinals(self.num_nodes);
                tline
                    .set_native_branch_matrix_indices(matrix_indices.b1, matrix_indices.b2)
                    .expect("validated CPL native branch ordinals resolve to matrix indices");
            }
        }
    }
}

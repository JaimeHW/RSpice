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
}

impl CircuitData {
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
    pub(crate) fn prepare_behavioral_small_signal(&mut self, dc_solution: &[Value]) {
        for source in &mut self.behavioral_sources.voltage_sources {
            source.linearize_at(dc_solution);
        }
        for source in &mut self.behavioral_sources.current_sources {
            source.linearize_at(dc_solution);
        }
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
    ) {
        for source in &mut self.behavioral_sources.voltage_sources {
            if source.is_frequency_dependent() {
                source.linearize_at_frequency(dc_solution, frequency);
            }
        }
        for source in &mut self.behavioral_sources.current_sources {
            if source.is_frequency_dependent() {
                source.linearize_at_frequency(dc_solution, frequency);
            }
        }
    }

    /// Linearize every behavioral source at an arbitrary state and active
    /// frequency. Distortion uses this for nearby bias states, where even a
    /// frequency-invariant expression must be refreshed because its inputs
    /// have changed.
    pub(crate) fn prepare_behavioral_small_signal_state_at_frequency(
        &mut self,
        state: &[Value],
        frequency: Value,
    ) {
        for source in &mut self.behavioral_sources.voltage_sources {
            source.linearize_at_state_and_frequency(state, frequency);
        }
        for source in &mut self.behavioral_sources.current_sources {
            source.linearize_at_state_and_frequency(state, frequency);
        }
    }

    /// Build the per-device operating-point report from the device state
    /// cached by the last accepted Newton solution. Call after a DC
    /// operating point (or any analysis that leaves devices at a solution).
    pub fn device_op_report(&self) -> DeviceOpReport {
        let mut entries = Vec::new();

        for mosfet in &self.mosfets.devices {
            let op = mosfet.op_values();
            entries.push(DeviceOpEntry {
                name: mosfet.name.clone(),
                device_kind: mosfet.device_kind(),
                region: Some(op.region),
                params: vec![
                    ("id", op.id),
                    ("vgs", op.vgs),
                    ("vds", op.vds),
                    ("vbs", op.vbs),
                    ("vth", op.vth),
                    ("vdsat", op.vdsat),
                    ("gm", op.gm),
                    ("gds", op.gds),
                    ("gmb", op.gmb),
                ],
            });
        }

        for dev in &self.bsim3v3.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry {
                name: dev.name.clone(),
                device_kind: "BSIM3",
                region: Some(region),
                params: vec![
                    ("id", id),
                    ("vgs", vgs),
                    ("vds", vds),
                    ("vbs", vbs),
                    ("vth", vth),
                    ("vdsat", vdsat),
                    ("gm", gm),
                    ("gds", gds),
                    ("gmb", gmbs),
                ],
            });
        }

        for dev in &self.bsim4v8.devices {
            let (id, vgs, vds, vbs, vth, vdsat, output_vdsat, gm, gds, gmbs, region) =
                dev.op_values();
            entries.push(DeviceOpEntry {
                name: dev.name.clone(),
                device_kind: "BSIM4",
                region: Some(region),
                params: vec![
                    ("id", id),
                    ("vgs", vgs),
                    ("vds", vds),
                    ("vbs", vbs),
                    ("vth", vth),
                    ("vdsat", vdsat),
                    ("output_vdsat", output_vdsat),
                    ("gm", gm),
                    ("gds", gds),
                    ("gmb", gmbs),
                ],
            });
        }

        for dev in &self.b3soi_fd.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry {
                name: dev.name.clone(),
                device_kind: "B3SOIFD",
                region: Some(region),
                params: vec![
                    ("id", id),
                    ("is", -id),
                    ("vgs", vgs),
                    ("vds", vds),
                    ("vbs", vbs),
                    ("vth", vth),
                    ("vdsat", vdsat),
                    ("gm", gm),
                    ("gds", gds),
                    ("gmb", gmbs),
                ],
            });
        }

        for dev in &self.b3soi.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry {
                name: dev.name.clone(),
                device_kind: "B3SOIDD",
                region: Some(region),
                params: vec![
                    ("id", id),
                    ("is", -id),
                    ("vgs", vgs),
                    ("vds", vds),
                    ("vbs", vbs),
                    ("vth", vth),
                    ("vdsat", vdsat),
                    ("gm", gm),
                    ("gds", gds),
                    ("gmb", gmbs),
                ],
            });
        }

        for dev in &self.b3soi_pd.devices {
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
            entries.push(DeviceOpEntry {
                name: dev.name.clone(),
                device_kind: "B3SOIPD",
                region: Some(region),
                params: vec![
                    ("id", id),
                    ("is", -id),
                    ("vgs", vgs),
                    ("vds", vds),
                    ("vbs", vbs),
                    ("vth", vth),
                    ("vdsat", vdsat),
                    ("gm", gm),
                    ("gds", gds),
                    ("gmb", gmbs),
                ],
            });
        }

        for dev in &self.ekv26s.devices {
            let op = dev.op_values();
            entries.push(DeviceOpEntry {
                name: dev.name.clone(),
                device_kind: "EKV26",
                region: None,
                params: vec![
                    ("id", op.id),
                    ("vgs", op.vgs),
                    ("vds", op.vds),
                    ("vbs", op.vbs),
                ],
            });
        }

        for dev in &self.ekv3s.devices {
            let op = dev.op_values();
            entries.push(DeviceOpEntry {
                name: dev.name.clone(),
                device_kind: "EKV3",
                region: None,
                params: vec![
                    ("id", op.id),
                    ("vgs", op.vgs),
                    ("vds", op.vds),
                    ("vbs", op.vbs),
                    ("gm", op.gm),
                ],
            });
        }

        for vdmos in &self.vdmoses.devices {
            let (id, vgs, vds, diode_id, power, region) = vdmos.op_values();
            entries.push(DeviceOpEntry {
                name: vdmos.name.clone(),
                device_kind: "VDMOS",
                region: Some(region),
                params: vec![
                    ("id", id),
                    ("vgs", vgs),
                    ("vds", vds),
                    ("idiode", diode_id),
                    ("power", power),
                ],
            });
        }

        for bjt in &self.bjts.devices {
            let (vbe, vbc, ic, ib, gm) = bjt.op_values();
            let (_, _, ie) = bjt.operating_point_currents();
            let beta = if ib.abs() > 1e-30 { ic / ib } else { 0.0 };
            entries.push(DeviceOpEntry {
                name: bjt.name.clone(),
                device_kind: "BJT",
                region: None,
                params: vec![
                    ("ic", ic),
                    ("ib", ib),
                    ("ie", ie),
                    ("vbe", vbe),
                    ("vce", vbe - vbc),
                    ("beta", beta),
                    ("gm", gm),
                ],
            });
        }

        for diode in &self.diodes.devices {
            let (vd, id, gd, cd) = diode.op_values();
            entries.push(DeviceOpEntry {
                name: diode.name.clone(),
                device_kind: "DIODE",
                region: None,
                params: vec![("vd", vd), ("id", id), ("gd", gd), ("cd", cd)],
            });
        }

        // Xyce LEVEL=2 self-consistent thermal resistors expose their
        // material resistance and temperature through the ordinary device-op
        // channel.  Ordinary scalar resistors remain derived branch values;
        // only devices with an actual thermal state are reported here.
        for (index, name) in self.resistors.names.iter().enumerate() {
            let Some(state) = self.resistors.thermal.get(index).and_then(Option::as_ref) else {
                continue;
            };
            entries.push(DeviceOpEntry {
                name: name.clone(),
                device_kind: "RESISTOR",
                region: None,
                params: vec![
                    ("r", state.output_resistance),
                    ("temp", state.temperature_celsius),
                ],
            });
        }

        for jfet in &self.jfets {
            let (vgs, vds, ids, gm, gds, igs, igd) = jfet.op_values();
            let device_kind = match jfet.params.channel_model {
                crate::device::JfetChannelModel::ShichmanHodges
                | crate::device::JfetChannelModel::XyceSydney => "JFET",
                crate::device::JfetChannelModel::ParkerSkellern => "JFET2",
                crate::device::JfetChannelModel::XyceModifiedShockley => "JFET2_XYCE",
                crate::device::JfetChannelModel::LegacyMesfet => "MESFET",
                crate::device::JfetChannelModel::Hfet1 if jfet.params.hfet_level == 6 => "HFET2",
                crate::device::JfetChannelModel::Hfet1 => "HFET1",
            };
            entries.push(DeviceOpEntry {
                name: jfet.name.clone(),
                device_kind,
                region: None,
                params: vec![
                    ("id", ids),
                    ("vgs", vgs),
                    ("vds", vds),
                    ("gm", gm),
                    ("gds", gds),
                    ("igs", igs),
                    ("igd", igd),
                ],
            });
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
            entries.push(DeviceOpEntry {
                name: name.clone(),
                device_kind: "CAPACITOR",
                region: None,
                params: vec![(
                    "c",
                    self.capacitors
                        .effective_capacitances
                        .get(index)
                        .copied()
                        .unwrap_or(Value::NAN),
                )],
            });
        }

        // Xyce nonlinear-core internal probes are exposed through the
        // canonical `YMIN!<K-name>` namespace.  Xyce LEVEL=1 reports M in
        // kA/m while LEVEL=2 reports M in A/m; both use Oersted/Gauss for H/B
        // unless BHSIUNITS=1 switches those fields back to SI units.
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
                let mu_0 = 4.0 * std::f64::consts::PI * 1.0e-7;
                1.0e4
                    * mu_0
                    * (h_si * (4.0 * std::f64::consts::PI / 1.0e3)
                        + binding.device.xyce_core_reported_magnetization())
            };
            entries.push(DeviceOpEntry {
                name: name.to_string(),
                device_kind: "NONLINEAR_CORE",
                region: None,
                params: vec![
                    (
                        "m",
                        if binding.device.is_xyce_core_level2() {
                            binding.device.xyce_core_reported_magnetization()
                        } else {
                            binding.device.xyce_core_reported_magnetization() / 1.0e3
                        },
                    ),
                    (
                        "h",
                        if binding.core_bh_si_units {
                            h_si
                        } else {
                            h_si * (4.0 * std::f64::consts::PI / 1.0e3)
                        },
                    ),
                    ("b", b_output),
                ],
            });
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
                let mu_0 = 4.0 * std::f64::consts::PI * 1.0e-7;
                1.0e4
                    * mu_0
                    * (h_si * (4.0 * std::f64::consts::PI / 1.0e3)
                        + binding.device.xyce_core_reported_magnetization())
            };
            entries.push(DeviceOpEntry {
                name: name.to_string(),
                device_kind: "NONLINEAR_CORE",
                region: None,
                params: vec![
                    (
                        "m",
                        if binding.device.is_xyce_core_level2() {
                            binding.device.xyce_core_reported_magnetization()
                        } else {
                            binding.device.xyce_core_reported_magnetization() / 1.0e3
                        },
                    ),
                    (
                        "h",
                        if binding.core_bh_si_units {
                            h_si
                        } else {
                            h_si * (4.0 * std::f64::consts::PI / 1.0e3)
                        },
                    ),
                    ("b", b_output),
                ],
            });
        }

        DeviceOpReport { entries }
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

    /// Get branch names sorted by their branch ordinal (1, 2, 3, ...).
    /// Returns a Vec where index i contains the canonical name of branch (i+1).
    pub fn branch_names_sorted(&self) -> Vec<String> {
        self.branch_name_by_ordinal
            .iter()
            .enumerate()
            .map(|(idx, name)| name.clone().unwrap_or_else(|| format!("BRANCH{}", idx + 1)))
            .collect()
    }

    /// Total device count (for parallel stamping threshold)
    pub fn device_count(&self) -> usize {
        let count = self.resistors.len()
            + self.resistor_branches.len()
            + self.capacitors.authored_len()
            + self.inductors.len()
            + self.voltage_sources.len()
            + self.current_sources.len()
            + self.diodes.len()
            + self.bjts.len()
            + self.mosfets.len()
            + self.ekv26s.len()
            + self.ekv3s.len()
            + self.vdmoses.len()
            + self.jfets.len()
            + self.xyce_memristors.len()
            + self.vcvs.len()
            + self.vccs.len()
            + self.cccs.len()
            + self.ccvs.len()
            + self.coupled_inductor_pairs.len()
            + self.multi_winding_transformers.len()
            + self.jiles_atherton_inductors.len();
        #[cfg(feature = "veriloga")]
        let count = count + self.veriloga_device_count();
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

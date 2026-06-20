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
    /// Linearize every behavioral source at the DC operating point so the
    /// small-signal (AC/noise/sensitivity) assembly can read the cached
    /// partials immutably. One-shot per analysis: the partials are
    /// frequency-independent, and per-worker circuit clones carry them.
    pub(crate) fn prepare_behavioral_small_signal(&mut self, dc_solution: &[Value]) {
        for source in &mut self.behavioral_sources.voltage_sources {
            source.linearize_at(dc_solution);
        }
        for source in &mut self.behavioral_sources.current_sources {
            source.linearize_at(dc_solution);
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
                device_kind: "MOSFET",
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
            let (id, vgs, vds, vbs, vth, vdsat, gm, gds, gmbs, region) = dev.op_values();
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
            let beta = if ib.abs() > 1e-30 { ic / ib } else { 0.0 };
            entries.push(DeviceOpEntry {
                name: bjt.name.clone(),
                device_kind: "BJT",
                region: None,
                params: vec![
                    ("ic", ic),
                    ("ib", ib),
                    ("vbe", vbe),
                    ("vce", vbe - vbc),
                    ("beta", beta),
                    ("gm", gm),
                ],
            });
        }

        for diode in &self.diodes.devices {
            let (vd, id, gd) = diode.op_values();
            entries.push(DeviceOpEntry {
                name: diode.name.clone(),
                device_kind: "DIODE",
                region: None,
                params: vec![("vd", vd), ("id", id), ("gd", gd)],
            });
        }

        for jfet in &self.jfets {
            let (vgs, vds, ids, gm, gds, igs, igd) = jfet.op_values();
            let device_kind = match jfet.params.channel_model {
                crate::device::JfetChannelModel::ShichmanHodges => "JFET",
                crate::device::JfetChannelModel::ParkerSkellern => "JFET2",
                crate::device::JfetChannelModel::XyceModifiedShockley => "JFET2_XYCE",
                _ => "MESFET",
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

        DeviceOpReport { entries }
    }

    /// Read-only access to linear resistor storage (names, nodes, conductances).
    pub fn resistor_storage(&self) -> &Resistors {
        &self.resistors
    }

    /// Read-only access to capacitor storage (names, nodes, capacitances, ICs).
    pub fn capacitor_storage(&self) -> &Capacitors {
        &self.capacitors
    }

    /// Read-only access to inductor storage (names, nodes, inductances, ICs).
    pub fn inductor_storage(&self) -> &Inductors {
        &self.inductors
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
            + self.capacitors.len()
            + self.inductors.len()
            + self.voltage_sources.len()
            + self.current_sources.len()
            + self.diodes.len()
            + self.bjts.len()
            + self.mosfets.len()
            + self.vdmoses.len()
            + self.jfets.len()
            + self.vcvs.len()
            + self.vccs.len()
            + self.cccs.len()
            + self.ccvs.len()
            + self.coupled_inductor_pairs.len()
            + self.multi_winding_transformers.len()
            + self.jiles_atherton_inductors.len();
        #[cfg(feature = "veriloga")]
        {
            count + self.veriloga_device_count()
        }
        #[cfg(not(feature = "veriloga"))]
        {
            count
        }
    }

    /// Create a triplet matrix for this circuit
    pub fn create_matrix(&self) -> TripletMatrix {
        let size = self.matrix_size();
        let mut m = TripletMatrix::new(size);
        m.nrows = size;
        m.ncols = size;
        m
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
        self.capacitors.link_indices(matrix);
        let num_nodes = self.num_nodes;
        self.voltage_sources
            .link_indices(matrix, |br_ordinal| num_nodes + br_ordinal);

        // Nonlinear devices
        self.diodes.link_all(matrix);
        self.bjts.link_all(matrix);
        self.mosfets.link_all(matrix);
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

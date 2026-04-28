use super::*;

impl CircuitData {
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
    }
}

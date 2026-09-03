//! Linear harmonic-balance assembly, source setup, and direct solve helpers.

use super::*;
use std::cell::RefCell;
use std::f64::consts::PI;

impl ExactPeriodicNetwork {
    pub(crate) fn try_visit_direct_entries(
        &self,
        omega: Value,
        unknowns: usize,
        mut visitor: impl FnMut(usize, usize, Complex64),
    ) -> Result<(), HbError> {
        if !omega.is_finite() {
            return Err(HbError::InvalidCircuit(
                "periodic distributed-network frequency is non-finite".to_string(),
            ));
        }
        let finite = |value: Complex64| value.re.is_finite() && value.im.is_finite();
        let visitor = RefCell::new(&mut visitor);
        let add = |row: usize, column: usize, value: Complex64| -> Result<(), HbError> {
            if row >= unknowns || column >= unknowns || !finite(value) {
                return Err(HbError::InvalidCircuit(format!(
                    "periodic distributed-network entry ({row}, {column}, {value}) is malformed for {unknowns} unknowns"
                )));
            }
            (visitor.borrow_mut())(row, column, value);
            Ok(())
        };
        let add_diff =
            |row: usize, pos: usize, neg: usize, value: Complex64| -> Result<(), HbError> {
                if pos > 0 {
                    add(row, pos - 1, value)?;
                }
                if neg > 0 {
                    add(row, neg - 1, -value)?;
                }
                Ok(())
            };

        match self {
            Self::ScalarWave {
                name,
                node1_pos,
                node1_neg,
                node2_pos,
                node2_neg,
                branch1,
                branch2,
                impedance,
                delay,
                attenuation,
            } => {
                if name.trim().is_empty()
                    || !impedance.is_finite()
                    || *impedance <= 0.0
                    || !delay.is_finite()
                    || *delay < 0.0
                    || !attenuation.is_finite()
                    || *attenuation != 1.0
                    || branch1 == branch2
                {
                    return Err(HbError::InvalidCircuit(format!(
                        "periodic scalar transmission line '{name}' is malformed or is not exactly lossless"
                    )));
                }
                let q = Complex64::from_polar(*attenuation, -omega * *delay);
                let one = Complex64::new(1.0, 0.0);
                let z = Complex64::new(*impedance, 0.0);
                add_diff(*branch1, *node1_pos, *node1_neg, one)?;
                add_diff(*branch1, *node2_pos, *node2_neg, -q)?;
                add(*branch1, *branch1, -z)?;
                add(*branch1, *branch2, -q * z)?;
                add_diff(*branch2, *node2_pos, *node2_neg, one)?;
                add_diff(*branch2, *node1_pos, *node1_neg, -q)?;
                add(*branch2, *branch2, -z)?;
                add(*branch2, *branch1, -q * z)?;
            }
            Self::ScalarLtra {
                name,
                node1_pos,
                node1_neg,
                node2_pos,
                node2_neg,
                branch1,
                branch2,
                total_inductance,
                total_capacitance,
                total_resistance,
            } => {
                if name.trim().is_empty()
                    || !total_inductance.is_finite()
                    || *total_inductance < 0.0
                    || !total_capacitance.is_finite()
                    || *total_capacitance <= 0.0
                    || !total_resistance.is_finite()
                    || *total_resistance < 0.0
                    || (*total_inductance == 0.0 && *total_resistance == 0.0)
                    || branch1 == branch2
                {
                    return Err(HbError::InvalidCircuit(format!(
                        "periodic scalar LTRA line '{name}' is malformed"
                    )));
                }
                let one = Complex64::new(1.0, 0.0);
                if omega == 0.0 {
                    add(*branch1, *branch1, one)?;
                    add(*branch1, *branch2, one)?;
                    add_diff(*branch2, *node1_pos, *node1_neg, one)?;
                    add_diff(*branch2, *node2_pos, *node2_neg, -one)?;
                    add(*branch2, *branch1, Complex64::new(-*total_resistance, 0.0))?;
                } else {
                    let s_c = Complex64::new(0.0, omega * *total_capacitance);
                    let z_series = Complex64::new(*total_resistance, omega * *total_inductance);
                    let y0 = (s_c / z_series).sqrt();
                    let mut propagation = (s_c * z_series).sqrt();
                    // On a perfectly lossless line the radicand lies exactly
                    // on the negative real axis, where the principal square
                    // root loses the sign of frequency. Restore the physical
                    // conjugate branch so negative PAC/PXF sidebands remain
                    // the conjugates of their positive-frequency operators.
                    if *total_resistance == 0.0 && omega < 0.0 {
                        propagation = propagation.conj();
                    }
                    let q = (-propagation).exp();
                    if !finite(y0) || !finite(q) {
                        return Err(HbError::InvalidCircuit(format!(
                            "periodic scalar LTRA line '{name}' is non-representable at omega={omega}"
                        )));
                    }
                    for &(row, sp, sn, fp, far_neg, own_branch, far_branch) in &[
                        (
                            *branch1, *node1_pos, *node1_neg, *node2_pos, *node2_neg, *branch1,
                            *branch2,
                        ),
                        (
                            *branch2, *node2_pos, *node2_neg, *node1_pos, *node1_neg, *branch2,
                            *branch1,
                        ),
                    ] {
                        add_diff(row, sp, sn, y0)?;
                        add_diff(row, fp, far_neg, -y0 * q)?;
                        add(row, own_branch, -one)?;
                        add(row, far_branch, -q)?;
                    }
                }
            }
            Self::LosslessCpl {
                name,
                near_nodes,
                far_nodes,
                near_ref,
                far_ref,
                near_branches,
                far_branches,
                voltage_transform,
                current_transform,
                modal_impedances,
                modal_delays,
            } => {
                let n = near_nodes.len();
                if name.trim().is_empty()
                    || n < 2
                    || far_nodes.len() != n
                    || near_branches.len() != n
                    || far_branches.len() != n
                    || voltage_transform.len() != n
                    || current_transform.len() != n
                    || voltage_transform.iter().any(|row| row.len() != n)
                    || current_transform.iter().any(|row| row.len() != n)
                    || modal_impedances.len() != n
                    || modal_delays.len() != n
                    || near_nodes.iter().enumerate().any(|(index, node)| {
                        *node == *near_ref || near_nodes[..index].contains(node)
                    })
                    || far_nodes
                        .iter()
                        .enumerate()
                        .any(|(index, node)| *node == *far_ref || far_nodes[..index].contains(node))
                    || near_branches
                        .iter()
                        .enumerate()
                        .any(|(index, branch)| near_branches[..index].contains(branch))
                    || far_branches.iter().enumerate().any(|(index, branch)| {
                        far_branches[..index].contains(branch) || near_branches.contains(branch)
                    })
                {
                    return Err(HbError::InvalidCircuit(format!(
                        "periodic lossless CPL '{name}' has malformed modal topology"
                    )));
                }
                for mode in 0..n {
                    let z = modal_impedances[mode];
                    let delay = modal_delays[mode];
                    if !z.is_finite() || z <= 0.0 || !delay.is_finite() || delay <= 0.0 {
                        return Err(HbError::InvalidCircuit(format!(
                            "periodic lossless CPL '{name}' mode {} is malformed",
                            mode + 1
                        )));
                    }
                    let q = Complex64::from_polar(1.0, -omega * delay);
                    for &(
                        row,
                        self_nodes,
                        self_ref,
                        far_nodes,
                        far_ref,
                        self_branches,
                        far_branches,
                    ) in &[
                        (
                            near_branches[mode],
                            near_nodes.as_slice(),
                            *near_ref,
                            far_nodes.as_slice(),
                            *far_ref,
                            near_branches.as_slice(),
                            far_branches.as_slice(),
                        ),
                        (
                            far_branches[mode],
                            far_nodes.as_slice(),
                            *far_ref,
                            near_nodes.as_slice(),
                            *near_ref,
                            far_branches.as_slice(),
                            near_branches.as_slice(),
                        ),
                    ] {
                        for conductor in 0..n {
                            let av = voltage_transform[mode][conductor];
                            let bi = current_transform[mode][conductor];
                            if !av.is_finite() || !bi.is_finite() {
                                return Err(HbError::InvalidCircuit(format!(
                                    "periodic lossless CPL '{name}' modal transform is non-finite"
                                )));
                            }
                            add_diff(
                                row,
                                self_nodes[conductor],
                                self_ref,
                                Complex64::new(av, 0.0),
                            )?;
                            add_diff(row, far_nodes[conductor], far_ref, -q * av)?;
                            add(row, self_branches[conductor], Complex64::new(-z * bi, 0.0))?;
                            add(row, far_branches[conductor], -q * z * bi)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl HbSolver {
    /// Create a new HB solver from a configuration already authenticated by
    /// the caller.
    ///
    /// New code should use [`Self::try_new`]. This compatibility constructor
    /// never panics for a malformed configuration; it retains the validation
    /// error and every numerical solve returns that typed error before using
    /// the placeholder storage.
    pub fn new(config: HbConfig, num_nodes: usize) -> Self {
        match Self::try_new(config.clone(), num_nodes) {
            Ok(solver) => solver,
            Err(HbError::InvalidConfig(error)) => Self::invalid(config, error),
            Err(error) => Self::invalid(config, HbConfigError::new("config", error.to_string())),
        }
    }

    /// Validate the complete HB numerical contract and construct a solver.
    pub fn try_new(config: HbConfig, num_nodes: usize) -> Result<Self, HbError> {
        config.validate()?;
        let num_harmonics = config.num_harmonics;
        let fft_size = config.checked_fft_size()?;
        let fft = HbFft::try_with_size(num_harmonics, fft_size)?;

        Ok(Self {
            config,
            configuration_error: None,
            fft,
            num_nodes,
            num_harmonics,
            num_branches: 0,
            g_matrix: Vec::new(),
            periodic_g_matrix: Vec::new(),
            c_matrix: Vec::new(),
            l_matrix: Vec::new(),
            voltage_source_branches: Vec::new(),
            voltage_source_branch_names: Vec::new(),
            periodic_mna_branches: Vec::new(),
            periodic_mna_branch_names: Vec::new(),
            exact_mna_static_entries: Vec::new(),
            exact_mna_inductance_entries: Vec::new(),
            exact_periodic_networks: Vec::new(),
            node_names: (0..num_nodes).map(|i| format!("n{}", i)).collect(),
            source_spectra: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            nonlinear_devices: Vec::new(),
            nonlinear_device_names: Vec::new(),
            nonlinear_noise_temperatures: Vec::new(),
            #[cfg(feature = "veriloga")]
            veriloga_nonlinear_devices: Vec::new(),
        })
    }

    fn invalid(config: HbConfig, error: HbConfigError) -> Self {
        let fft = HbFft::minimal();
        Self {
            config,
            configuration_error: Some(error),
            fft,
            num_nodes: 0,
            num_harmonics: 1,
            num_branches: 0,
            g_matrix: Vec::new(),
            periodic_g_matrix: Vec::new(),
            c_matrix: Vec::new(),
            l_matrix: Vec::new(),
            voltage_source_branches: Vec::new(),
            voltage_source_branch_names: Vec::new(),
            periodic_mna_branches: Vec::new(),
            periodic_mna_branch_names: Vec::new(),
            exact_mna_static_entries: Vec::new(),
            exact_mna_inductance_entries: Vec::new(),
            exact_periodic_networks: Vec::new(),
            node_names: Vec::new(),
            source_spectra: Vec::new(),
            nonlinear_devices: Vec::new(),
            nonlinear_device_names: Vec::new(),
            nonlinear_noise_temperatures: Vec::new(),
            #[cfg(feature = "veriloga")]
            veriloga_nonlinear_devices: Vec::new(),
        }
    }

    pub(super) fn validate_configuration(&self) -> Result<(), HbError> {
        if let Some(error) = &self.configuration_error {
            return Err(HbError::InvalidConfig(error.clone()));
        }
        self.config.validate().map_err(HbError::from)?;
        self.validate_nonlinear_device_parameters()
    }

    /// Get number of harmonics
    pub fn num_harmonics(&self) -> usize {
        self.num_harmonics
    }

    /// Set node names
    pub fn set_node_names(&mut self, names: Vec<String>) {
        self.node_names = names;
    }

    /// Add a single conductance stamp at position (node_i, node_j)
    ///
    /// This is a low-level method that adds a single Y-matrix entry.
    /// For a resistor between two nodes, use `add_resistor` instead.
    pub fn add_conductance(&mut self, node_i: usize, node_j: usize, g: Value) {
        self.add_conductance_with_small_signal(node_i, node_j, g, g);
    }

    /// Add one static conductance entry with distinct large-signal and
    /// periodic small-signal values.
    pub(crate) fn add_conductance_with_small_signal(
        &mut self,
        node_i: usize,
        node_j: usize,
        conductance: Value,
        small_signal_conductance: Value,
    ) {
        self.g_matrix.push((node_i, node_j, conductance));
        self.periodic_g_matrix
            .push((node_i, node_j, small_signal_conductance));
    }

    /// Add a resistor between two nodes with full MNA stamping
    ///
    /// For a resistor R between nodes i and j with G = 1/R:
    /// - Y(i,i) += G
    /// - Y(j,j) += G
    /// - Y(i,j) -= G
    /// - Y(j,i) -= G
    ///
    /// Ground is represented by a node index >= num_nodes (effectively ignored).
    pub fn add_resistor(&mut self, node_i: usize, node_j: usize, r: Value) {
        if r.abs() < 1e-30 {
            return; // Avoid division by zero
        }
        let g = 1.0 / r;

        // Full MNA stamp
        self.add_conductance(node_i, node_i, g);
        if node_j < self.num_nodes {
            self.add_conductance(node_j, node_j, g);
            self.add_conductance(node_i, node_j, -g);
            self.add_conductance(node_j, node_i, -g);
        }
    }

    /// Add a single capacitance stamp at position (node_i, node_j)
    ///
    /// This is a low-level method. For a capacitor between two nodes,
    /// the caller should add all 4 MNA stamps manually or use a higher-level API.
    pub fn add_capacitance(&mut self, node_i: usize, node_j: usize, c: Value) {
        self.c_matrix.push((node_i, node_j, c));
    }

    /// Add inductance stamp
    ///
    /// In frequency domain, inductor admittance is Y_L = 1/(jωL).
    /// At DC (ω=0), inductor is short circuit (infinite admittance) - handled specially.
    /// At harmonic k: Y_L(k) = 1/(j * k * ω₀ * L) = -j/(k * ω₀ * L)
    pub fn add_inductance(&mut self, node_i: usize, node_j: usize, l: Value) {
        self.l_matrix.push((node_i, node_j, l));
    }

    /// Add voltage source with MNA branch current
    ///
    /// Proper MNA treatment: voltage sources require branch current variables
    /// to enforce voltage constraint without Norton approximation.
    pub fn add_voltage_source_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
    ) -> usize {
        let branch_idx = self.num_branches;
        self.voltage_source_branches.push(VoltageSourceBranch::new(
            node_pos, node_neg, branch_idx, dc_voltage,
        ));
        self.voltage_source_branch_names
            .push(format!("V{}", branch_idx + 1));
        self.num_branches += 1;
        branch_idx
    }

    /// Add voltage source with arbitrary AC harmonic entries.
    pub fn add_voltage_source_branch_harmonics(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
        harmonics: &[(usize, Value, Value)],
    ) -> usize {
        let branch_idx = self.num_branches;
        let mut branch = VoltageSourceBranch::new(node_pos, node_neg, branch_idx, dc_voltage);
        for (harmonic, magnitude, phase) in harmonics {
            branch = branch.with_harmonic(*harmonic, *magnitude, *phase);
        }
        self.voltage_source_branches.push(branch);
        self.voltage_source_branch_names
            .push(format!("V{}", branch_idx + 1));
        self.num_branches += 1;
        branch_idx
    }

    /// Fallibly register one authored ideal voltage source for an exact MNA
    /// solve. Unlike the compatibility helpers above, this production
    /// boundary rejects malformed topology, names, spectra, and allocation
    /// failures before mutating the solver.
    pub(crate) fn try_add_named_voltage_source_branch_harmonics(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
        harmonics: &[(usize, Value, Value)],
        name: &str,
    ) -> Result<usize, HbError> {
        if node_pos > self.num_nodes || node_neg > self.num_nodes || node_pos == node_neg {
            return Err(HbError::InvalidCircuit(format!(
                "ideal voltage source '{name}' has invalid terminal pair ({node_pos}, {node_neg}) for {} non-ground nodes",
                self.num_nodes
            )));
        }
        if name.is_empty() || name.trim() != name {
            return Err(HbError::InvalidCircuit(
                "ideal voltage-source names must be nonempty and have no surrounding whitespace"
                    .to_string(),
            ));
        }
        if self
            .voltage_source_branch_names
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(name))
        {
            return Err(HbError::InvalidCircuit(format!(
                "ideal voltage-source name '{name}' is duplicated"
            )));
        }
        if !dc_voltage.is_finite() {
            return Err(HbError::InvalidCircuit(format!(
                "ideal voltage source '{name}' has non-finite DC value {dc_voltage:e}"
            )));
        }

        let mut seen_harmonics = std::collections::HashSet::new();
        seen_harmonics
            .try_reserve(harmonics.len())
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "ideal voltage source '{name}' harmonic-map allocation failed: {error}"
                ))
            })?;
        let mut branch =
            VoltageSourceBranch::new(node_pos, node_neg, self.num_branches, dc_voltage);
        branch
            .ac_harmonics
            .try_reserve_exact(harmonics.len())
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "ideal voltage source '{name}' spectrum allocation failed: {error}"
                ))
            })?;
        for &(harmonic, magnitude, phase) in harmonics {
            if harmonic == 0 || harmonic > self.num_harmonics {
                return Err(HbError::InvalidCircuit(format!(
                    "ideal voltage source '{name}' references harmonic {harmonic}; expected 1..={}",
                    self.num_harmonics
                )));
            }
            if !magnitude.is_finite() || !phase.is_finite() {
                return Err(HbError::InvalidCircuit(format!(
                    "ideal voltage source '{name}' harmonic {harmonic} has a non-finite magnitude or phase"
                )));
            }
            if !seen_harmonics.insert(harmonic) {
                return Err(HbError::InvalidCircuit(format!(
                    "ideal voltage source '{name}' defines harmonic {harmonic} more than once"
                )));
            }
            branch.set_harmonic_component(harmonic, Complex64::from_polar(magnitude, phase));
        }
        branch
            .ac_harmonics
            .sort_unstable_by_key(|(harmonic, _)| *harmonic);

        let branch_idx = self.num_branches;
        let next_branch_count = branch_idx.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit("ideal voltage-source count exceeds this platform".to_string())
        })?;
        let mut owned_name = String::new();
        owned_name.try_reserve_exact(name.len()).map_err(|error| {
            HbError::InvalidCircuit(format!(
                "ideal voltage-source name allocation failed for '{name}': {error}"
            ))
        })?;
        owned_name.push_str(name);
        self.voltage_source_branches
            .try_reserve(1)
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "ideal voltage-source allocation failed for '{name}': {error}"
                ))
            })?;
        self.voltage_source_branch_names
            .try_reserve(1)
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "ideal voltage-source name-table allocation failed for '{name}': {error}"
                ))
            })?;
        self.voltage_source_branches.push(branch);
        self.voltage_source_branch_names.push(owned_name);
        self.num_branches = next_branch_count;
        Ok(branch_idx)
    }

    /// Preserve the authored name for an already-added MNA branch.
    pub fn set_voltage_source_branch_name(&mut self, branch_idx: usize, name: impl Into<String>) {
        if let Some(slot) = self.voltage_source_branch_names.get_mut(branch_idx) {
            *slot = name.into();
        }
    }

    /// Get number of MNA branch currents
    pub fn num_branches(&self) -> usize {
        self.num_branches
    }

    fn validate_periodic_mna_branch_identity(
        &self,
        node_pos: usize,
        node_neg: usize,
        branch_ordinal: usize,
        name: &str,
    ) -> Result<(), HbError> {
        let expected_ordinal =
            self.periodic_mna_branches
                .len()
                .checked_add(1)
                .ok_or_else(|| {
                    HbError::InvalidCircuit(
                        "periodic MNA branch ordinal exceeds this platform".to_string(),
                    )
                })?;
        if branch_ordinal != expected_ordinal {
            return Err(HbError::InvalidCircuit(format!(
                "periodic MNA branch '{name}' has canonical ordinal {branch_ordinal}; expected {expected_ordinal}"
            )));
        }
        if name.is_empty() || name.trim() != name {
            return Err(HbError::InvalidCircuit(format!(
                "periodic MNA branch ordinal {branch_ordinal} must have a nonempty name without surrounding whitespace"
            )));
        }
        if self
            .periodic_mna_branch_names
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(name))
        {
            return Err(HbError::InvalidCircuit(format!(
                "periodic MNA branch name '{name}' is duplicated"
            )));
        }
        if node_pos > self.num_nodes || node_neg > self.num_nodes {
            return Err(HbError::InvalidCircuit(format!(
                "periodic MNA branch '{name}' references node pair ({node_pos}, {node_neg}) outside 0..={}",
                self.num_nodes
            )));
        }
        if node_pos == node_neg {
            return Err(HbError::InvalidCircuit(format!(
                "periodic MNA branch '{name}' has identical terminals"
            )));
        }
        Ok(())
    }

    fn try_push_periodic_mna_branch(
        &mut self,
        branch: ExactMnaBranch,
        name: &str,
    ) -> Result<(), HbError> {
        let mut owned_name = String::new();
        owned_name.try_reserve_exact(name.len()).map_err(|error| {
            HbError::InvalidCircuit(format!(
                "periodic MNA branch-name allocation failed for '{name}': {error}"
            ))
        })?;
        owned_name.push_str(name);
        self.periodic_mna_branches.try_reserve(1).map_err(|error| {
            HbError::InvalidCircuit(format!(
                "periodic MNA branch allocation failed for '{name}': {error}"
            ))
        })?;
        self.periodic_mna_branch_names
            .try_reserve(1)
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "periodic MNA branch-name table allocation failed for '{name}': {error}"
                ))
            })?;
        self.periodic_mna_branches.push(branch);
        self.periodic_mna_branch_names.push(owned_name);
        Ok(())
    }

    /// Add an exact inductor branch for periodic small-signal MNA.
    pub(crate) fn try_add_periodic_inductor_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        inductance: Value,
        branch_ordinal: usize,
        name: &str,
    ) -> Result<(), HbError> {
        self.validate_periodic_mna_branch_identity(node_pos, node_neg, branch_ordinal, name)?;
        if !inductance.is_finite() || inductance == 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "periodic inductor branch '{name}' must have finite nonzero inductance"
            )));
        }
        self.try_push_periodic_mna_branch(
            ExactMnaBranch::Inductor {
                branch_ordinal,
                node_pos,
                node_neg,
                inductance,
            },
            name,
        )
    }

    /// Add an exact finite branch-form resistor for large-signal HB and its
    /// periodic small-signal systems.
    pub(crate) fn try_add_periodic_resistor_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        resistance: Value,
        small_signal_resistance: Value,
        branch_ordinal: usize,
        name: &str,
    ) -> Result<(), HbError> {
        self.validate_periodic_mna_branch_identity(node_pos, node_neg, branch_ordinal, name)?;
        if !resistance.is_finite() {
            return Err(HbError::InvalidCircuit(format!(
                "periodic resistor branch '{name}' must have finite resistance"
            )));
        }
        if !small_signal_resistance.is_finite() {
            return Err(HbError::InvalidCircuit(format!(
                "periodic resistor branch '{name}' must have finite small-signal resistance"
            )));
        }
        self.try_push_periodic_mna_branch(
            ExactMnaBranch::Resistor {
                branch_ordinal,
                node_pos,
                node_neg,
                resistance,
                small_signal_resistance,
            },
            name,
        )
    }

    /// Register a voltage constraint only in the periodic small-signal MNA
    /// system, without adding a zero-valued source to the large-signal
    /// operating-point source spectrum.
    pub(crate) fn try_add_periodic_voltage_source_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        source_index: usize,
        branch_ordinal: usize,
        name: &str,
    ) -> Result<(), HbError> {
        self.validate_periodic_mna_branch_identity(node_pos, node_neg, branch_ordinal, name)?;
        if self.periodic_mna_branches.iter().any(
            |branch| matches!(branch, ExactMnaBranch::VoltageSource { source_index: index, .. } if *index == source_index),
        ) {
            return Err(HbError::InvalidCircuit(format!(
                "periodic voltage-source branch '{name}' duplicates source index {source_index}"
            )));
        }
        let source = self.voltage_source_branches.get(source_index).cloned();
        self.try_push_periodic_mna_branch(
            ExactMnaBranch::VoltageSource {
                branch_ordinal,
                node_pos,
                node_neg,
                source_index,
                source,
            },
            name,
        )
    }

    /// Register an exact dependent ideal-voltage branch. The branch's control
    /// relation is stamped separately into the augmented static operator.
    pub(crate) fn try_add_periodic_controlled_voltage_source_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        branch_ordinal: usize,
        name: &str,
    ) -> Result<(), HbError> {
        self.validate_periodic_mna_branch_identity(node_pos, node_neg, branch_ordinal, name)?;
        self.try_push_periodic_mna_branch(
            ExactMnaBranch::ControlledVoltageSource {
                branch_ordinal,
                node_pos,
                node_neg,
            },
            name,
        )
    }

    pub(crate) fn try_add_periodic_network_port_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        branch_ordinal: usize,
        name: &str,
    ) -> Result<(), HbError> {
        self.validate_periodic_mna_branch_identity(node_pos, node_neg, branch_ordinal, name)?;
        self.try_push_periodic_mna_branch(
            ExactMnaBranch::NetworkPort {
                branch_ordinal,
                node_pos,
                node_neg,
            },
            name,
        )
    }

    pub(crate) fn try_add_exact_periodic_network(
        &mut self,
        network: ExactPeriodicNetwork,
    ) -> Result<(), HbError> {
        let unknowns = self
            .num_nodes
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "periodic distributed-network dimension exceeds this platform".to_string(),
                )
            })?;
        network.try_visit_direct_entries(0.0, unknowns, |_, _, _| {})?;
        self.exact_periodic_networks
            .try_reserve(1)
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "periodic distributed-network allocation failed: {error}"
                ))
            })?;
        self.exact_periodic_networks.push(network);
        Ok(())
    }

    /// Add one frequency-independent controlled-source entry to the complete
    /// node/branch MNA operator. `row` and `column` use zero-based augmented
    /// indices (all nodes, then canonical branches).
    pub(crate) fn try_add_exact_mna_static_entry(
        &mut self,
        row: usize,
        column: usize,
        value: Value,
        context: &str,
    ) -> Result<(), HbError> {
        let unknowns = self
            .num_nodes
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "periodic controlled-source MNA dimension exceeds this platform".to_string(),
                )
            })?;
        if row >= unknowns || column >= unknowns || !value.is_finite() {
            return Err(HbError::InvalidCircuit(format!(
                "periodic controlled-source {context} stamp ({row}, {column}, {value}) is malformed for {unknowns} unknowns"
            )));
        }
        self.exact_mna_static_entries
            .try_reserve(1)
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "periodic controlled-source {context} stamp allocation failed: {error}"
                ))
            })?;
        self.exact_mna_static_entries.push((row, column, value));
        Ok(())
    }

    /// Add one mutual-inductance coefficient to the exact augmented MNA
    /// operator. Both coordinates must be canonical branch-current slots.
    pub(crate) fn try_add_exact_mna_inductance_entry(
        &mut self,
        row: usize,
        column: usize,
        inductance: Value,
        context: &str,
    ) -> Result<(), HbError> {
        let unknowns = self
            .num_nodes
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "periodic mutual-inductance MNA dimension exceeds this platform".to_string(),
                )
            })?;
        if row < self.num_nodes
            || column < self.num_nodes
            || row >= unknowns
            || column >= unknowns
            || row == column
            || !inductance.is_finite()
        {
            return Err(HbError::InvalidCircuit(format!(
                "periodic mutual-inductance {context} stamp ({row}, {column}, {inductance}) is malformed for {unknowns} unknowns"
            )));
        }
        self.exact_mna_inductance_entries
            .try_reserve(1)
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "periodic mutual-inductance {context} stamp allocation failed: {error}"
                ))
            })?;
        self.exact_mna_inductance_entries
            .push((row, column, inductance));
        Ok(())
    }

    /// Fallibly copy the canonical periodic MNA names for retained results.
    pub(crate) fn try_periodic_mna_branch_names(&self) -> Result<Vec<String>, HbError> {
        if self.periodic_mna_branches.len() != self.periodic_mna_branch_names.len() {
            return Err(HbError::InvalidCircuit(format!(
                "periodic MNA descriptor/name cardinality differs ({} branches, {} names)",
                self.periodic_mna_branches.len(),
                self.periodic_mna_branch_names.len()
            )));
        }
        let mut names = Vec::new();
        names
            .try_reserve_exact(self.periodic_mna_branch_names.len())
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "periodic MNA result-name allocation failed: {error}"
                ))
            })?;
        for name in &self.periodic_mna_branch_names {
            let mut copy = String::new();
            copy.try_reserve_exact(name.len()).map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "periodic MNA result-name allocation failed for '{name}': {error}"
                ))
            })?;
            copy.push_str(name);
            names.push(copy);
        }
        Ok(names)
    }

    /// Resolve a circuit voltage-source ordinal to its periodic MNA branch.
    pub(crate) fn periodic_voltage_source_branch(&self, source_index: usize) -> Option<usize> {
        self.periodic_mna_branches
            .iter()
            .position(|branch| matches!(branch, ExactMnaBranch::VoltageSource { source_index: index, .. } if *index == source_index))
    }

    /// Canonical exact MNA descriptors shared by the large-signal and lifted
    /// periodic systems. The slice is in one-based circuit branch order.
    pub(crate) fn exact_mna_branches(&self) -> &[ExactMnaBranch] {
        &self.periodic_mna_branches
    }

    /// Names aligned exactly with [`Self::exact_mna_branches`].
    pub(crate) fn exact_mna_branch_names(&self) -> &[String] {
        &self.periodic_mna_branch_names
    }

    fn validate_linear_storage(&self, state: &HbSolverState) -> Result<(), HbError> {
        if !self.config.fundamental_freq.is_finite() || self.config.fundamental_freq <= 0.0 {
            return Err(HbError::InvalidCircuit(
                "linear HB fundamental frequency must be finite and positive".to_string(),
            ));
        }
        if !self.config.tolerance.is_finite() || self.config.tolerance < 0.0 {
            return Err(HbError::InvalidCircuit(
                "linear HB tolerance must be finite and nonnegative".to_string(),
            ));
        }
        let harmonic_count = self.num_harmonics.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit("linear HB harmonic count exceeds this platform".to_string())
        })?;
        if self.node_names.len() != self.num_nodes {
            return Err(HbError::InvalidCircuit(format!(
                "linear HB has {} node names for {} nodal unknowns",
                self.node_names.len(),
                self.num_nodes
            )));
        }
        if state.x.len() != self.num_nodes
            || state.residual.len() != self.num_nodes
            || state.residual_scale.len() != self.num_nodes
        {
            return Err(HbError::InvalidCircuit(format!(
                "linear HB state has {} voltage, {} residual, and {} scale rows; expected {} of each",
                state.x.len(),
                state.residual.len(),
                state.residual_scale.len(),
                self.num_nodes
            )));
        }
        for (row, ((spectrum, residual), scale)) in state
            .x
            .iter()
            .zip(&state.residual)
            .zip(&state.residual_scale)
            .enumerate()
        {
            if spectrum.len() != harmonic_count
                || residual.len() != harmonic_count
                || scale.len() != harmonic_count
            {
                return Err(HbError::InvalidCircuit(format!(
                    "linear HB state row {row} has voltage/residual/scale lengths {}/{}/{}; expected {harmonic_count}",
                    spectrum.len(),
                    residual.len(),
                    scale.len()
                )));
            }
        }
        if self.source_spectra.len() != self.num_nodes {
            return Err(HbError::InvalidCircuit(format!(
                "linear HB has {} source-spectrum rows for {} nodes",
                self.source_spectra.len(),
                self.num_nodes
            )));
        }
        for (node, spectrum) in self.source_spectra.iter().enumerate() {
            if spectrum.len() != harmonic_count {
                return Err(HbError::InvalidCircuit(format!(
                    "linear HB source-spectrum row {node} has {} coefficients; expected {harmonic_count}",
                    spectrum.len()
                )));
            }
            if spectrum
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "linear HB source-spectrum row {node} contains a non-finite coefficient"
                )));
            }
        }
        if self.periodic_g_matrix.len() != self.g_matrix.len()
            || self
                .periodic_g_matrix
                .iter()
                .zip(&self.g_matrix)
                .any(|(periodic, large_signal)| {
                    periodic.0 != large_signal.0 || periodic.1 != large_signal.1
                })
        {
            return Err(HbError::InvalidCircuit(
                "linear HB large-signal and periodic conductance topology is misaligned"
                    .to_string(),
            ));
        }
        for (kind, entries) in [
            ("conductance", &self.g_matrix),
            ("periodic small-signal conductance", &self.periodic_g_matrix),
            ("capacitance", &self.c_matrix),
            ("legacy inductance", &self.l_matrix),
        ] {
            for (entry, &(row, column, value)) in entries.iter().enumerate() {
                if row >= self.num_nodes || column >= self.num_nodes {
                    return Err(HbError::InvalidCircuit(format!(
                        "linear HB {kind} entry #{entry} ({row}, {column}) is outside its {}-node operator",
                        self.num_nodes
                    )));
                }
                if !value.is_finite() {
                    return Err(HbError::InvalidCircuit(format!(
                        "linear HB {kind} entry #{entry} ({row}, {column}) is non-finite"
                    )));
                }
            }
        }
        if self.voltage_source_branches.len() != self.num_branches
            || self.voltage_source_branch_names.len() != self.num_branches
        {
            return Err(HbError::InvalidCircuit(format!(
                "linear HB voltage-source storage has {} descriptors and {} names for {} source branches",
                self.voltage_source_branches.len(),
                self.voltage_source_branch_names.len(),
                self.num_branches
            )));
        }
        for (index, (branch, name)) in self
            .voltage_source_branches
            .iter()
            .zip(&self.voltage_source_branch_names)
            .enumerate()
        {
            Self::validate_voltage_source_descriptor(
                branch,
                name,
                index,
                self.num_nodes,
                self.num_harmonics,
            )?;
        }
        Ok(())
    }

    fn validate_voltage_source_descriptor(
        branch: &VoltageSourceBranch,
        name: &str,
        expected_index: usize,
        num_nodes: usize,
        num_harmonics: usize,
    ) -> Result<(), HbError> {
        if branch.branch_idx != expected_index {
            return Err(HbError::InvalidCircuit(format!(
                "ideal voltage source '{name}' has source index {}; expected {expected_index}",
                branch.branch_idx
            )));
        }
        if name.is_empty() || name.trim() != name {
            return Err(HbError::InvalidCircuit(format!(
                "ideal voltage source index {expected_index} has a non-canonical name"
            )));
        }
        if branch.node_pos > num_nodes
            || branch.node_neg > num_nodes
            || branch.node_pos == branch.node_neg
        {
            return Err(HbError::InvalidCircuit(format!(
                "ideal voltage source '{name}' has invalid terminal pair ({}, {}) for {num_nodes} non-ground nodes",
                branch.node_pos, branch.node_neg
            )));
        }
        if !branch.dc_voltage.is_finite() {
            return Err(HbError::InvalidCircuit(format!(
                "ideal voltage source '{name}' has a non-finite DC coefficient"
            )));
        }
        let mut previous_harmonic = 0;
        for &(harmonic, coefficient) in &branch.ac_harmonics {
            if harmonic == 0 || harmonic > num_harmonics || harmonic <= previous_harmonic {
                return Err(HbError::InvalidCircuit(format!(
                    "ideal voltage source '{name}' has non-canonical harmonic index {harmonic}; entries must be unique, ascending, and within 1..={num_harmonics}"
                )));
            }
            if !coefficient.re.is_finite() || !coefficient.im.is_finite() {
                return Err(HbError::InvalidCircuit(format!(
                    "ideal voltage source '{name}' harmonic {harmonic} is non-finite"
                )));
            }
            previous_harmonic = harmonic;
        }
        Ok(())
    }

    pub(crate) fn validate_exact_large_signal_mna(&self) -> Result<(), HbError> {
        if self.periodic_mna_branches.len() != self.periodic_mna_branch_names.len() {
            return Err(HbError::InvalidCircuit(format!(
                "exact linear MNA has {} descriptors for {} names",
                self.periodic_mna_branches.len(),
                self.periodic_mna_branch_names.len()
            )));
        }
        if !self.periodic_mna_branches.is_empty() && !self.l_matrix.is_empty() {
            return Err(HbError::InvalidCircuit(
                "exact linear MNA cannot combine inductor branch equations with legacy nodal inductor admittances"
                    .to_string(),
            ));
        }
        let graph_nodes = self.num_nodes.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit(
                "exact HB ideal-constraint graph exceeds this platform".to_string(),
            )
        })?;
        let mut graph_parents: Vec<usize> = (0..graph_nodes).collect();
        let mut graph_ranks = vec![0_u8; graph_nodes];
        fn graph_root(parents: &mut [usize], node: usize) -> usize {
            let mut root = node;
            while parents[root] != root {
                root = parents[root];
            }
            let mut cursor = node;
            while parents[cursor] != cursor {
                let next = parents[cursor];
                parents[cursor] = root;
                cursor = next;
            }
            root
        }
        let mut seen_sources = vec![false; self.voltage_source_branches.len()];
        for (index, (branch, name)) in self
            .periodic_mna_branches
            .iter()
            .zip(&self.periodic_mna_branch_names)
            .enumerate()
        {
            let expected_ordinal = index.checked_add(1).ok_or_else(|| {
                HbError::InvalidCircuit(
                    "exact linear MNA branch ordinal exceeds this platform".to_string(),
                )
            })?;
            if name.is_empty() || name.trim() != name {
                return Err(HbError::InvalidCircuit(format!(
                    "exact linear MNA branch ordinal {expected_ordinal} has a non-canonical name"
                )));
            }
            let (branch_ordinal, node_pos, node_neg, is_ideal_constraint) = match branch {
                ExactMnaBranch::VoltageSource {
                    branch_ordinal,
                    node_pos,
                    node_neg,
                    source_index,
                    source,
                } => {
                    let source = source.as_ref().ok_or_else(|| {
                        HbError::InvalidCircuit(format!(
                            "exact linear MNA voltage-source branch '{name}' has no authored large-signal spectrum"
                        ))
                    })?;
                    let source_name = self
                        .voltage_source_branch_names
                        .get(*source_index)
                        .ok_or_else(|| {
                            HbError::InvalidCircuit(format!(
                                "exact linear MNA voltage-source branch '{name}' references missing source index {source_index}"
                            ))
                        })?;
                    Self::validate_voltage_source_descriptor(
                        source,
                        source_name,
                        *source_index,
                        self.num_nodes,
                        self.num_harmonics,
                    )?;
                    if source_name != name
                        || source.node_pos != *node_pos
                        || source.node_neg != *node_neg
                        || self.voltage_source_branches.get(*source_index) != Some(source)
                    {
                        return Err(HbError::InvalidCircuit(format!(
                            "exact linear MNA branch '{name}' does not match its authored voltage-source descriptor"
                        )));
                    }
                    let seen = seen_sources.get_mut(*source_index).ok_or_else(|| {
                        HbError::InvalidCircuit(format!(
                            "exact linear MNA voltage-source branch '{name}' has out-of-range source index {source_index}"
                        ))
                    })?;
                    if std::mem::replace(seen, true) {
                        return Err(HbError::InvalidCircuit(format!(
                            "exact linear MNA voltage source '{name}' is registered more than once"
                        )));
                    }
                    (*branch_ordinal, *node_pos, *node_neg, true)
                }
                ExactMnaBranch::Inductor {
                    branch_ordinal,
                    node_pos,
                    node_neg,
                    inductance,
                } => {
                    if !inductance.is_finite() || *inductance == 0.0 {
                        return Err(HbError::InvalidCircuit(format!(
                            "exact linear MNA inductor '{name}' must have finite nonzero inductance"
                        )));
                    }
                    let omega0 = 2.0 * PI * self.config.fundamental_freq;
                    for harmonic in 1..=self.num_harmonics {
                        let impedance = omega0 * harmonic as Value * *inductance;
                        if !impedance.is_finite() || impedance == 0.0 {
                            return Err(HbError::InvalidCircuit(format!(
                                "exact linear MNA inductor '{name}' has non-representable impedance at harmonic {harmonic}"
                            )));
                        }
                    }
                    (*branch_ordinal, *node_pos, *node_neg, true)
                }
                ExactMnaBranch::Resistor {
                    branch_ordinal,
                    node_pos,
                    node_neg,
                    resistance,
                    small_signal_resistance,
                } => {
                    if !resistance.is_finite() || !small_signal_resistance.is_finite() {
                        return Err(HbError::InvalidCircuit(format!(
                            "exact linear MNA resistor '{name}' must have finite DC and small-signal resistances"
                        )));
                    }
                    (*branch_ordinal, *node_pos, *node_neg, *resistance == 0.0)
                }
                ExactMnaBranch::ControlledVoltageSource {
                    branch_ordinal,
                    node_pos,
                    node_neg,
                } => (*branch_ordinal, *node_pos, *node_neg, false),
                ExactMnaBranch::NetworkPort {
                    branch_ordinal,
                    node_pos,
                    node_neg,
                } => (*branch_ordinal, *node_pos, *node_neg, false),
            };
            if branch_ordinal != expected_ordinal {
                return Err(HbError::InvalidCircuit(format!(
                    "exact linear MNA branch '{name}' has ordinal {branch_ordinal}; expected {expected_ordinal}"
                )));
            }
            if node_pos > self.num_nodes || node_neg > self.num_nodes || node_pos == node_neg {
                return Err(HbError::InvalidCircuit(format!(
                    "exact linear MNA branch '{name}' has invalid terminal pair ({node_pos}, {node_neg}) for {} non-ground nodes",
                    self.num_nodes
                )));
            }
            if is_ideal_constraint {
                let root_pos = graph_root(&mut graph_parents, node_pos);
                let root_neg = graph_root(&mut graph_parents, node_neg);
                if root_pos == root_neg {
                    return Err(HbError::InvalidCircuit(format!(
                        "exact HB MNA has a singular or inconsistent conflicting ideal branch loop at '{name}'"
                    )));
                }
                if graph_ranks[root_pos] < graph_ranks[root_neg] {
                    graph_parents[root_pos] = root_neg;
                } else {
                    graph_parents[root_neg] = root_pos;
                    if graph_ranks[root_pos] == graph_ranks[root_neg] {
                        graph_ranks[root_pos] = graph_ranks[root_pos].saturating_add(1);
                    }
                }
            }
        }
        if seen_sources.iter().any(|seen| !seen) {
            return Err(HbError::InvalidCircuit(
                "exact linear MNA does not register every authored ideal voltage source"
                    .to_string(),
            ));
        }
        let unknowns = self
            .num_nodes
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "exact linear MNA dimension exceeds this platform".to_string(),
                )
            })?;
        for (entry, &(row, column, value)) in self.exact_mna_static_entries.iter().enumerate() {
            if row >= unknowns || column >= unknowns || !value.is_finite() {
                return Err(HbError::InvalidCircuit(format!(
                    "exact linear MNA controlled-source entry #{entry} ({row}, {column}, {value}) is malformed for {unknowns} unknowns"
                )));
            }
        }
        for (entry, &(row, column, inductance)) in
            self.exact_mna_inductance_entries.iter().enumerate()
        {
            if row < self.num_nodes
                || column < self.num_nodes
                || row >= unknowns
                || column >= unknowns
                || row == column
                || !inductance.is_finite()
            {
                return Err(HbError::InvalidCircuit(format!(
                    "exact linear MNA mutual-inductance entry #{entry} ({row}, {column}, {inductance}) is malformed for {unknowns} unknowns"
                )));
            }
            for harmonic in 1..=self.num_harmonics {
                let impedance =
                    2.0 * PI * self.config.fundamental_freq * harmonic as Value * inductance;
                if !impedance.is_finite() || (inductance != 0.0 && impedance == 0.0) {
                    return Err(HbError::InvalidCircuit(format!(
                        "exact linear MNA mutual-inductance entry #{entry} has non-representable impedance at harmonic {harmonic}"
                    )));
                }
            }
        }
        for network in &self.exact_periodic_networks {
            for harmonic in 0..=self.num_harmonics {
                network.try_visit_direct_entries(
                    2.0 * PI * self.config.fundamental_freq * harmonic as Value,
                    unknowns,
                    |_, _, _| {},
                )?;
            }
        }
        Ok(())
    }

    /// Add DC source current contribution at a node
    pub fn add_dc_source(&mut self, node: usize, current: Value) {
        if node < self.source_spectra.len() {
            self.source_spectra[node][0] += Complex64::new(current, 0.0);
        }
    }

    /// Set AC source contribution at an arbitrary harmonic for a node.
    ///
    /// `magnitude` is the physical source amplitude. The solver stores
    /// Fourier coefficients internally (harmonic k contributes
    /// `2*Re(c_k e^{jkwt})` to the synthesized waveform), so amplitudes
    /// convert with a factor 1/2 at this boundary; DC passes through.
    /// Stamping the full amplitude as the coefficient made every nonlinear
    /// device see twice the true voltage swing.
    pub fn set_harmonic_source(
        &mut self,
        node: usize,
        harmonic: usize,
        magnitude: Value,
        phase: Value,
    ) {
        if node < self.source_spectra.len() && harmonic < self.source_spectra[node].len() {
            let scale = if harmonic == 0 { 1.0 } else { 0.5 };
            self.source_spectra[node][harmonic] = Complex64::from_polar(magnitude * scale, phase);
        }
    }

    /// Add AC source contribution at an arbitrary harmonic for a node.
    ///
    /// Same amplitude-to-coefficient conversion as `set_harmonic_source`.
    pub fn add_harmonic_source(
        &mut self,
        node: usize,
        harmonic: usize,
        magnitude: Value,
        phase: Value,
    ) {
        if node < self.source_spectra.len() && harmonic < self.source_spectra[node].len() {
            let scale = if harmonic == 0 { 1.0 } else { 0.5 };
            self.source_spectra[node][harmonic] += Complex64::from_polar(magnitude * scale, phase);
        }
    }

    /// Compute residual for linear circuit (KCL: sum of currents INTO node = 0)
    ///
    /// Residual = I_source - (G*X + jω*C*X + (1/jωL)*X)
    ///          = I_source - Y*X
    ///
    /// For inductors, admittance Y_L = 1/(jωL) = -j/(ωL)
    /// At DC (ω=0): inductor is short circuit, requires special handling
    pub fn compute_linear_residual(&self, state: &mut HbSolverState) {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Start with source currents (positive = current INTO node)
        for node_res in &mut state.residual {
            for c in node_res.iter_mut() {
                *c = Complex64::new(0.0, 0.0);
            }
        }
        // The per-row current scale accumulates |contribution| alongside
        // every residual term (the SPICE KCL convergence reference).
        for node_scale in &mut state.residual_scale {
            for s in node_scale.iter_mut() {
                *s = 0.0;
            }
        }

        // Add source contributions first
        for (node, source) in self.source_spectra.iter().enumerate() {
            if node < state.residual.len() {
                for (k, &s) in source.iter().enumerate() {
                    if k < state.residual[node].len() {
                        state.residual[node][k] += s; // Source current INTO node
                        state.residual_scale[node][k] += s.norm();
                    }
                }
            }
        }

        // Subtract G*X contribution (current through conductance leaves node)
        for &(i, j, g) in &self.g_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        state.residual[i][k] -= g * state.x[j][k];
                        state.residual_scale[i][k] += g.abs() * state.x[j][k].norm();
                    }
                }
            }
        }

        // Subtract jω*C*X contribution (capacitor admittance current)
        for &(i, j, c) in &self.c_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        let j_omega = Complex64::new(0.0, omega_k);
                        state.residual[i][k] -= j_omega * c * state.x[j][k];
                        state.residual_scale[i][k] += omega_k * c.abs() * state.x[j][k].norm();
                    }
                }
            }
        }

        // Subtract 1/(jωL)*X contribution (inductor admittance current)
        // Y_L = 1/(jωL) = -j/(ωL)
        // At DC (k=0): inductor is short circuit - enforce V=0 (large admittance)
        for &(i, j, l) in &self.l_matrix {
            if i < state.x.len() && j < state.x.len() && l.abs() > 1e-30 {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        if k == 0 {
                            // DC: inductor is short circuit
                            // Preserve the signed matrix topology so the
                            // four entries enforce V_i = V_j.
                            let y_l = inductor_dc_short_admittance(l);
                            state.residual[i][k] -= y_l * state.x[j][k];
                            state.residual_scale[i][k] += y_l.abs() * state.x[j][k].norm();
                        } else {
                            // AC: Y_L = 1/(jωL) = -j/(ωL)
                            let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                            state.residual[i][k] -= y_l * state.x[j][k];
                            state.residual_scale[i][k] +=
                                state.x[j][k].norm() / (omega_k * l.abs());
                        }
                    }
                }
            }
        }

        state.compute_residual_norm();
    }

    /// Source value entering the branch KVL constraint at one harmonic, in
    /// the solver's internal Fourier-coefficient convention: stored AC
    /// entries are physical amplitudes, so harmonics k >= 1 convert with a
    /// factor 1/2 (see `set_harmonic_source`).
    pub(crate) fn voltage_source_value_at_harmonic(
        branch: &VoltageSourceBranch,
        harmonic: usize,
    ) -> Complex64 {
        if harmonic == 0 {
            Complex64::new(branch.dc_voltage, 0.0)
        } else {
            branch
                .ac_harmonics
                .iter()
                .find_map(|(index, value)| (*index == harmonic).then_some(*value * 0.5))
                .unwrap_or_else(|| Complex64::new(0.0, 0.0))
        }
    }

    fn compute_linear_residual_with_branches(
        &self,
        state: &mut HbSolverState,
        branch_currents: &[Vec<Complex64>],
        exact_mna: bool,
    ) -> Result<(), HbError> {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let h = self.num_harmonics + 1;

        for node_res in &mut state.residual {
            node_res.fill(Complex64::new(0.0, 0.0));
        }
        for node_scale in &mut state.residual_scale {
            node_scale.fill(0.0);
        }
        for branch_residual in &mut state.mna_branch_residual {
            branch_residual.fill(Complex64::new(0.0, 0.0));
        }
        for branch_scale in &mut state.mna_branch_residual_scale {
            branch_scale.fill(0.0);
        }

        // Start with nodal current source spectra.
        for (node, source) in self.source_spectra.iter().enumerate() {
            for (k, &value) in source.iter().enumerate() {
                state.residual[node][k] += value;
                state.residual_scale[node][k] += value.norm();
            }
        }

        // Subtract linear passive contributions.
        for &(i, j, g) in &self.g_matrix {
            for k in 0..h {
                let contribution = g * state.x[j][k];
                state.residual[i][k] -= contribution;
                state.residual_scale[i][k] += contribution.norm();
            }
        }
        for &(i, j, c) in &self.c_matrix {
            for k in 0..h {
                let omega_k = (k as f64) * omega0;
                let contribution = Complex64::new(0.0, omega_k) * c * state.x[j][k];
                state.residual[i][k] -= contribution;
                state.residual_scale[i][k] += contribution.norm();
            }
        }
        for &(i, j, l) in &self.l_matrix {
            if l.abs() > 1e-30 {
                for k in 0..h {
                    let contribution = if k == 0 {
                        inductor_dc_short_admittance(l) * state.x[j][k]
                    } else {
                        let omega_k = (k as f64) * omega0;
                        Complex64::new(0.0, -1.0 / (omega_k * l)) * state.x[j][k]
                    };
                    state.residual[i][k] -= contribution;
                    state.residual_scale[i][k] += contribution.norm();
                }
            }
        }

        // Subtract MNA branch current coupling in nodal equations.
        if exact_mna {
            for (branch_index, branch) in self.periodic_mna_branches.iter().enumerate() {
                let currents = &branch_currents[branch_index];
                let (_, node_pos, node_neg) = branch.ordinal_and_terminals();
                for (k, &current) in currents.iter().enumerate() {
                    if node_pos > 0 {
                        state.residual[node_pos - 1][k] -= current;
                        state.residual_scale[node_pos - 1][k] += current.norm();
                    }
                    if node_neg > 0 {
                        state.residual[node_neg - 1][k] += current;
                        state.residual_scale[node_neg - 1][k] += current.norm();
                    }
                }
            }
        } else {
            for branch in &self.voltage_source_branches {
                let currents = &branch_currents[branch.branch_idx];
                for (k, &current) in currents.iter().enumerate() {
                    if branch.node_pos > 0 {
                        state.residual[branch.node_pos - 1][k] -= current;
                        state.residual_scale[branch.node_pos - 1][k] += current.norm();
                    }
                    if branch.node_neg > 0 {
                        state.residual[branch.node_neg - 1][k] += current;
                        state.residual_scale[branch.node_neg - 1][k] += current.norm();
                    }
                }
            }
        }

        // Retain every branch KVL residual and its voltage-domain reference
        // scale so linear publication uses the same typed KCL/KVL
        // certificate as nonlinear Newton.
        if exact_mna {
            for (branch_index, branch) in self.periodic_mna_branches.iter().enumerate() {
                let (_, node_pos, node_neg) = branch.ordinal_and_terminals();
                for (k, &branch_current) in branch_currents[branch_index].iter().enumerate().take(h)
                {
                    let mut voltage_drop = Complex64::new(0.0, 0.0);
                    let mut voltage_scale = 0.0;
                    if node_pos > 0 {
                        let voltage = state.x[node_pos - 1][k];
                        voltage_drop += voltage;
                        voltage_scale += voltage.norm();
                    }
                    if node_neg > 0 {
                        let voltage = state.x[node_neg - 1][k];
                        voltage_drop -= voltage;
                        voltage_scale += voltage.norm();
                    }
                    let (residual, constitutive_scale) = match branch {
                        ExactMnaBranch::VoltageSource { source, .. } => {
                            let source = source.as_ref().ok_or_else(|| {
                                HbError::InvalidCircuit(
                                    "exact linear MNA voltage source lost its authored spectrum"
                                        .to_string(),
                                )
                            })?;
                            let source_value = Self::voltage_source_value_at_harmonic(source, k);
                            (source_value - voltage_drop, source_value.norm())
                        }
                        ExactMnaBranch::Inductor { inductance, .. } => {
                            let constitutive_voltage =
                                Complex64::new(0.0, (k as Value) * omega0 * *inductance)
                                    * branch_current;
                            (
                                constitutive_voltage - voltage_drop,
                                constitutive_voltage.norm(),
                            )
                        }
                        ExactMnaBranch::Resistor { resistance, .. } => {
                            let constitutive_voltage = *resistance * branch_current;
                            (
                                constitutive_voltage - voltage_drop,
                                constitutive_voltage.norm(),
                            )
                        }
                        ExactMnaBranch::ControlledVoltageSource { .. } => (-voltage_drop, 0.0),
                        ExactMnaBranch::NetworkPort { .. } => (Complex64::new(0.0, 0.0), 0.0),
                    };
                    state.mna_branch_residual[branch_index][k] = residual;
                    state.mna_branch_residual_scale[branch_index][k] =
                        voltage_scale + constitutive_scale;
                }
            }
        } else {
            for branch in &self.voltage_source_branches {
                for k in 0..h {
                    let mut voltage_drop = Complex64::new(0.0, 0.0);
                    let mut voltage_scale = 0.0;
                    if branch.node_pos > 0 {
                        let voltage = state.x[branch.node_pos - 1][k];
                        voltage_drop += voltage;
                        voltage_scale += voltage.norm();
                    }
                    if branch.node_neg > 0 {
                        let voltage = state.x[branch.node_neg - 1][k];
                        voltage_drop -= voltage;
                        voltage_scale += voltage.norm();
                    }
                    let source_value = Self::voltage_source_value_at_harmonic(branch, k);
                    state.mna_branch_residual[branch.branch_idx][k] = source_value - voltage_drop;
                    state.mna_branch_residual_scale[branch.branch_idx][k] =
                        voltage_scale + source_value.norm();
                }
            }
        }
        if exact_mna {
            for &(row, column, coefficient) in &self.exact_mna_static_entries {
                // The input column is a node voltage or a branch current
                // depending on `column`, so there is no one slice to walk.
                #[allow(clippy::needless_range_loop)]
                for k in 0..h {
                    let input = if column < self.num_nodes {
                        state.x[column][k]
                    } else {
                        branch_currents[column - self.num_nodes][k]
                    };
                    let contribution = coefficient * input;
                    if row < self.num_nodes {
                        state.residual[row][k] -= contribution;
                        state.residual_scale[row][k] += contribution.norm();
                    } else {
                        let branch = row - self.num_nodes;
                        state.mna_branch_residual[branch][k] -= contribution;
                        state.mna_branch_residual_scale[branch][k] += contribution.norm();
                    }
                }
            }
            for &(row, column, inductance) in &self.exact_mna_inductance_entries {
                let branch = row - self.num_nodes;
                let control_branch = column - self.num_nodes;
                for (k, &control_current) in
                    branch_currents[control_branch].iter().enumerate().take(h)
                {
                    let contribution =
                        Complex64::new(0.0, (k as Value) * omega0 * inductance) * control_current;
                    state.mna_branch_residual[branch][k] += contribution;
                    state.mna_branch_residual_scale[branch][k] += contribution.norm();
                }
            }
            let unknowns = self.num_nodes + self.periodic_mna_branches.len();
            // As above: the visitor reads whichever of the two unknown vectors
            // the entry's column names.
            #[allow(clippy::needless_range_loop)]
            for k in 0..h {
                let omega = k as Value * omega0;
                for network in &self.exact_periodic_networks {
                    network.try_visit_direct_entries(omega, unknowns, |row, column, value| {
                        let input = if column < self.num_nodes {
                            state.x[column][k]
                        } else {
                            branch_currents[column - self.num_nodes][k]
                        };
                        let contribution = value * input;
                        if row < self.num_nodes {
                            state.residual[row][k] -= contribution;
                            state.residual_scale[row][k] += contribution.norm();
                        } else {
                            let branch = row - self.num_nodes;
                            state.mna_branch_residual[branch][k] -= contribution;
                            state.mna_branch_residual_scale[branch][k] += contribution.norm();
                        }
                    })?;
                }
            }
        }
        state.compute_residual_norm();
        if !state.residual_norm.is_finite()
            || state
                .residual_scale
                .iter()
                .chain(&state.mna_branch_residual_scale)
                .flatten()
                .any(|scale| !scale.is_finite() || *scale < 0.0)
        {
            return Err(HbError::InvalidCircuit(
                "linear HB residual certificate contains a non-finite value".to_string(),
            ));
        }
        Ok(())
    }

    /// Solve for linear circuit (direct solve for diagonal harmonic blocks).
    ///
    /// Builds Y = G + jωC + 1/(jωL) and augments with MNA branch equations for
    /// ideal voltage sources when present.
    pub fn solve_linear(&self, state: &mut HbSolverState) -> Result<(), HbError> {
        self.validate_configuration()?;
        self.validate_linear_storage(state)?;
        let exact_mna = !self.periodic_mna_branches.is_empty();
        if exact_mna {
            self.validate_exact_large_signal_mna()?;
        }
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let m = if exact_mna {
            self.periodic_mna_branches.len()
        } else {
            self.num_branches
        };
        state.try_prepare_mna_branches(m, self.num_harmonics)?;
        let total_unknowns = n.checked_add(m).ok_or_else(|| {
            HbError::InvalidCircuit("linear HB MNA dimension exceeds this platform".to_string())
        })?;

        let mut branch_currents = vec![vec![Complex64::new(0.0, 0.0); h]; m];

        // For each harmonic, solve an independent linear system.
        for k in 0..h {
            let omega_k = (k as f64) * omega0;
            let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); total_unknowns]; total_unknowns];
            let mut rhs = vec![Complex64::new(0.0, 0.0); total_unknowns];

            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += g;
                }
            }

            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += Complex64::new(0.0, omega_k) * c;
                }
            }

            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    if k == 0 {
                        y_matrix[i][j] += inductor_dc_short_admittance(l);
                    } else {
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        y_matrix[i][j] += y_l;
                    }
                }
            }

            for (node, entry) in rhs.iter_mut().enumerate().take(n) {
                *entry = self
                    .source_spectra
                    .get(node)
                    .and_then(|s| s.get(k))
                    .copied()
                    .unwrap_or_default();
            }

            if exact_mna {
                for (branch_index, branch) in self.periodic_mna_branches.iter().enumerate() {
                    let row = n + branch_index;
                    let (_, node_pos, node_neg) = branch.ordinal_and_terminals();
                    if node_pos > 0 {
                        let node = node_pos - 1;
                        y_matrix[node][row] += Complex64::new(1.0, 0.0);
                        if !matches!(branch, ExactMnaBranch::NetworkPort { .. }) {
                            y_matrix[row][node] += Complex64::new(1.0, 0.0);
                        }
                    }
                    if node_neg > 0 {
                        let node = node_neg - 1;
                        y_matrix[node][row] -= Complex64::new(1.0, 0.0);
                        if !matches!(branch, ExactMnaBranch::NetworkPort { .. }) {
                            y_matrix[row][node] -= Complex64::new(1.0, 0.0);
                        }
                    }
                    match branch {
                        ExactMnaBranch::VoltageSource { source, .. } => {
                            let source = source.as_ref().ok_or_else(|| {
                                HbError::InvalidCircuit(
                                    "exact linear MNA voltage source lost its authored spectrum"
                                        .to_string(),
                                )
                            })?;
                            rhs[row] = Self::voltage_source_value_at_harmonic(source, k);
                        }
                        ExactMnaBranch::Inductor { inductance, .. } => {
                            y_matrix[row][row] -= Complex64::new(0.0, omega_k * *inductance);
                        }
                        ExactMnaBranch::Resistor { resistance, .. } => {
                            y_matrix[row][row] -= *resistance;
                        }
                        ExactMnaBranch::ControlledVoltageSource { .. } => {}
                        ExactMnaBranch::NetworkPort { .. } => {}
                    }
                }
                for &(row, column, value) in &self.exact_mna_static_entries {
                    y_matrix[row][column] += value;
                }
                for &(row, column, inductance) in &self.exact_mna_inductance_entries {
                    y_matrix[row][column] -= Complex64::new(0.0, omega_k * inductance);
                }
                for network in &self.exact_periodic_networks {
                    network.try_visit_direct_entries(
                        omega_k,
                        total_unknowns,
                        |row, column, value| {
                            y_matrix[row][column] += value;
                        },
                    )?;
                }
            } else {
                for branch in &self.voltage_source_branches {
                    let row = n + branch.branch_idx;
                    if branch.node_pos > 0 {
                        let node = branch.node_pos - 1;
                        y_matrix[node][row] += Complex64::new(1.0, 0.0);
                        y_matrix[row][node] += Complex64::new(1.0, 0.0);
                    }
                    if branch.node_neg > 0 {
                        let node = branch.node_neg - 1;
                        y_matrix[node][row] -= Complex64::new(1.0, 0.0);
                        y_matrix[row][node] -= Complex64::new(1.0, 0.0);
                    }
                    rhs[row] = Self::voltage_source_value_at_harmonic(branch, k);
                }
            }

            let solution = self.solve_complex_linear_system(&y_matrix, &rhs)?;
            if solution.len() != total_unknowns
                || solution
                    .iter()
                    .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "linear HB harmonic {k} produced a malformed or non-finite MNA solution"
                )));
            }

            for (harmonics, &node_value) in state.x.iter_mut().zip(&solution).take(n) {
                harmonics[k] = if k == 0 {
                    Complex64::new(node_value.re, 0.0)
                } else {
                    node_value
                };
            }
            for (harmonics, &branch_value) in branch_currents.iter_mut().zip(&solution[n..]).take(m)
            {
                harmonics[k] = if k == 0 {
                    Complex64::new(branch_value.re, 0.0)
                } else {
                    branch_value
                };
            }
        }

        state.mna_branch_currents = branch_currents;
        if m == 0 {
            self.compute_linear_residual(state);
        } else {
            let retained_branch_currents = state.mna_branch_currents.clone();
            self.compute_linear_residual_with_branches(
                state,
                &retained_branch_currents,
                exact_mna,
            )?;
        }
        state.converged = state.rows_converged_with_branch_tolerances(
            self.config.tolerance,
            self.config.abstol,
            crate::constants::VNTOL,
        );

        if state.converged {
            Ok(())
        } else {
            Err(HbError::ConvergenceFailed {
                iterations: 0,
                residual: state.residual_norm,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stamp_two_terminal_inductor(
        solver: &mut HbSolver,
        node_pos: usize,
        node_neg: usize,
        inductance: Value,
    ) {
        solver.add_inductance(node_pos, node_pos, inductance);
        solver.add_inductance(node_pos, node_neg, -inductance);
        solver.add_inductance(node_neg, node_pos, -inductance);
        solver.add_inductance(node_neg, node_neg, inductance);
    }

    #[test]
    fn dc_inductor_residual_preserves_off_diagonal_topology_sign() {
        let mut solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 2);
        stamp_two_terminal_inductor(&mut solver, 0, 1, 1.2e-6);

        let mut state = HbSolverState::new(2, 1);
        state.x[0][0] = Complex64::new(2.0, 0.0);
        state.x[1][0] = Complex64::new(3.0, 0.0);
        solver.compute_linear_residual(&mut state);

        let expected = DC_SHORT_CONDUCTANCE;
        assert_eq!(state.residual[0][0], Complex64::new(expected, 0.0));
        assert_eq!(state.residual[1][0], Complex64::new(-expected, 0.0));
        assert_eq!(
            state.residual[0][0] + state.residual[1][0],
            Complex64::new(0.0, 0.0),
            "a two-terminal inductor stamp must conserve current"
        );
        assert_eq!(state.residual_scale[0][0], 5.0 * expected);
        assert_eq!(state.residual_scale[1][0], 5.0 * expected);
    }

    #[test]
    fn dangling_series_rl_dc_solution_has_equal_nodes_and_zero_kcl() {
        const SOURCE_R: Value = 50.0;
        const CABLE_R: Value = 0.12;

        // V1--50R--line--0.12R--internal--1.2uH--dangling. The capacitor
        // loads `line` only at nonzero harmonics. At DC every node must equal
        // the 1 V source and the dangling R-L branch must carry zero current.
        let mut solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 4);
        solver.add_voltage_source_branch(1, 0, 1.0);
        solver.add_resistor(0, 1, SOURCE_R);
        solver.add_capacitance(1, 1, 420.0e-12);
        solver.add_resistor(1, 2, CABLE_R);
        stamp_two_terminal_inductor(&mut solver, 2, 3, 1.2e-6);

        let mut state = HbSolverState::new(4, 1);
        solver
            .solve_linear(&mut state)
            .expect("floating series R-L HB system must solve");

        let dc = state.x.iter().map(|node| node[0].re).collect::<Vec<_>>();
        for (index, voltage) in dc.iter().copied().enumerate() {
            assert!(
                (voltage - 1.0).abs() <= 1.0e-8,
                "DC node {index} was {voltage:.17e}, expected the 1 V source"
            );
        }

        let source_resistor_current = (dc[0] - dc[1]) / SOURCE_R;
        let cable_resistor_current = (dc[1] - dc[2]) / CABLE_R;
        let inductor_short_current = DC_SHORT_CONDUCTANCE * (dc[2] - dc[3]);
        let source_branch_current = state.mna_branch_currents[0][0].re;
        for (label, current) in [
            ("source resistor", source_resistor_current),
            ("cable resistor", cable_resistor_current),
            ("inductor short surrogate", inductor_short_current),
            ("source MNA branch", source_branch_current),
        ] {
            assert!(
                current.abs() <= 1.0e-9,
                "{label} carried nonzero DC current {current:.17e}"
            );
        }
        assert!(
            (cable_resistor_current - inductor_short_current).abs() <= 1.0e-9,
            "internal R-L node violates KCL"
        );
        assert!(
            state.residual.iter().all(|row| row[0].norm() <= 1.0e-9),
            "solved DC KCL residuals were {:?}",
            state.residual.iter().map(|row| row[0]).collect::<Vec<_>>()
        );
        assert!(state.converged, "linear HB solve must report convergence");
    }
}

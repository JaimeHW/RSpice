use super::*;

const DISTRIBUTED_LINE_SECTION_MIN: usize = 16;
const DISTRIBUTED_LINE_SECTION_MAX: usize = 64;
const DISTRIBUTED_LINE_TARGET_SECTION_DELAY: f64 = 25e-12;
const CPL_REALIZATION_TOL: f64 = 1e-18;
const CPL_REFERENCE_SHORT_RESISTANCE: f64 = 1e-6;

pub(in crate::engine::builder) fn distributed_line_section_count(
    conductors: usize,
    estimated_delay: f64,
) -> usize {
    let conductor_floor = conductors
        .saturating_mul(8)
        .max(DISTRIBUTED_LINE_SECTION_MIN);
    let delay_driven = if estimated_delay.is_finite() && estimated_delay > 0.0 {
        (estimated_delay / DISTRIBUTED_LINE_TARGET_SECTION_DELAY)
            .ceil()
            .max(1.0) as usize
    } else {
        conductor_floor
    };

    delay_driven.max(conductor_floor).clamp(
        conductor_floor,
        DISTRIBUTED_LINE_SECTION_MAX.max(conductor_floor),
    )
}

pub(in crate::engine::builder) fn build_scalar_rlgc_line(
    circuit: &mut CircuitData,
    instance_name: &str,
    node_near: usize,
    node_near_ref: usize,
    node_far: usize,
    node_far_ref: usize,
    params: TransmissionLineModelParams,
) -> Result<(), SimulationError> {
    let r = params.r.unwrap_or(0.0).max(0.0);
    let l = params.l.ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Transmission line '{}' requires finite positive L/L0 for distributed RLGC synthesis",
            instance_name
        ))
    })?;
    let g = params.g.unwrap_or(0.0).max(0.0);
    let c = params.c.ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Transmission line '{}' requires finite positive C/C0 for distributed RLGC synthesis",
            instance_name
        ))
    })?;
    let len = params.len.ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Transmission line '{}' requires finite positive LEN/LENGTH for distributed RLGC synthesis",
            instance_name
        ))
    })?;

    if !l.is_finite() || l <= 0.0 || !c.is_finite() || c <= 0.0 || !len.is_finite() || len <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Transmission line '{}' resolved invalid RLGC parameters (L={}, C={}, LEN={})",
            instance_name, l, c, len
        )));
    }

    let estimated_delay = len * (l * c).sqrt();
    let sections = distributed_line_section_count(1, estimated_delay);
    let section_length = len / sections as f64;
    let section_delay = estimated_delay / sections as f64;
    if section_delay.is_finite() && section_delay > 0.0 {
        // The synthesized ladder is only faithful when the transient solver
        // resolves each section's propagation interval with multiple steps.
        circuit.tighten_transient_max_step_hint(0.5 * section_delay);
    }
    let mut boundary_nodes = vec![node_near; sections + 1];
    boundary_nodes[sections] = node_far;
    for section in 1..sections {
        let node_name = format!("{}.__rlgc.b{}", instance_name, section);
        boundary_nodes[section] = circuit.get_or_create_node(&node_name);
    }

    let mut boundary_refs = vec![node_near_ref; sections + 1];
    boundary_refs[0] = node_near_ref;
    boundary_refs[sections] = node_far_ref;
    if node_near_ref != node_far_ref {
        for section in 1..sections {
            let node_name = format!("{}.__rlgc.ref{}", instance_name, section);
            boundary_refs[section] = circuit.get_or_create_node(&node_name);
        }
        for section in 0..sections {
            circuit.resistors.add(
                format!("{}.__rlgc.refwire{}", instance_name, section + 1),
                boundary_refs[section],
                boundary_refs[section + 1],
                CPL_REFERENCE_SHORT_RESISTANCE,
            );
        }
    }

    for boundary in 0..=sections {
        let weight = if boundary == 0 || boundary == sections {
            0.5
        } else {
            1.0
        };
        let lump_length = section_length * weight;
        let node = boundary_nodes[boundary];
        let reference = boundary_refs[boundary];

        let c_lump = c * lump_length;
        if c_lump > CPL_REALIZATION_TOL && node != reference {
            circuit.capacitors.add(
                format!("{}.__rlgc.cb{}", instance_name, boundary),
                node,
                reference,
                c_lump,
            );
        }

        let g_lump = g * lump_length;
        if g_lump > CPL_REALIZATION_TOL && node != reference {
            circuit.resistors.add(
                format!("{}.__rlgc.gb{}", instance_name, boundary),
                node,
                reference,
                1.0 / g_lump.max(CPL_REALIZATION_TOL),
            );
        }
    }

    for section in 0..sections {
        let start = boundary_nodes[section];
        let end = boundary_nodes[section + 1];
        let series_r = r * section_length;
        let series_l = l * section_length;
        let half_series_r = 0.5 * series_r;
        let winding_start = if half_series_r > CPL_REALIZATION_TOL {
            let next_node = circuit.get_or_create_node(&format!(
                "{}.__rlgc.s{}.r.pre",
                instance_name,
                section + 1
            ));
            circuit.resistors.add(
                format!("{}.__rlgc.s{}.r.pre", instance_name, section + 1),
                start,
                next_node,
                half_series_r,
            );
            next_node
        } else {
            start
        };

        let winding_end = if half_series_r > CPL_REALIZATION_TOL {
            let next_node = circuit.get_or_create_node(&format!(
                "{}.__rlgc.s{}.r.post",
                instance_name,
                section + 1
            ));
            circuit.resistors.add(
                format!("{}.__rlgc.s{}.r.post", instance_name, section + 1),
                next_node,
                end,
                half_series_r,
            );
            next_node
        } else {
            end
        };

        let branch =
            circuit.allocate_branch_named(&format!("{}.__rlgc.s{}.l", instance_name, section + 1));
        circuit.inductors.add(
            format!("{}.__rlgc.s{}.l", instance_name, section + 1),
            winding_start,
            winding_end,
            branch,
            series_l,
        );
    }

    Ok(())
}

pub(in crate::engine::builder) fn validate_cpl_model_params(
    model_name: &str,
    params: &CplModelParams,
) -> Result<(), SimulationError> {
    let conductors = params.l.len();
    for (label, matrix) in [
        ("R", &params.r),
        ("L", &params.l),
        ("C", &params.c),
        ("G", &params.g),
    ] {
        if matrix.len() != conductors || matrix.iter().any(|row| row.len() != conductors) {
            return Err(SimulationError::Circuit(format!(
                "CPL model '{}' has malformed {} matrix dimensions",
                model_name, label
            )));
        }
        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if !value.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "CPL model '{}' has non-finite {}[{},{}]",
                        model_name,
                        label,
                        row_idx + 1,
                        col_idx + 1
                    )));
                }
            }
        }
    }

    for i in 0..conductors {
        if params.r[i][i] < -CPL_REALIZATION_TOL {
            return Err(SimulationError::Circuit(format!(
                "CPL model '{}' has negative series resistance on conductor {}",
                model_name,
                i + 1
            )));
        }
        if params.l[i][i] <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "CPL model '{}' has non-positive self inductance on conductor {}",
                model_name,
                i + 1
            )));
        }
        if params.c[i][i] <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "CPL model '{}' has non-positive self capacitance on conductor {}",
                model_name,
                i + 1
            )));
        }
        if params.g[i][i] < -CPL_REALIZATION_TOL {
            return Err(SimulationError::Circuit(format!(
                "CPL model '{}' has negative shunt conductance on conductor {}",
                model_name,
                i + 1
            )));
        }

        for j in 0..conductors {
            if i == j {
                continue;
            }

            if params.l[i][j] < -CPL_REALIZATION_TOL {
                return Err(SimulationError::Circuit(format!(
                    "CPL model '{}' uses negative mutual inductance L[{},{}], which is not yet realizable",
                    model_name,
                    i + 1,
                    j + 1
                )));
            }
            if params.c[i][j] > CPL_REALIZATION_TOL {
                return Err(SimulationError::Circuit(format!(
                    "CPL model '{}' has positive off-diagonal capacitance C[{},{}], expected Maxwell form",
                    model_name,
                    i + 1,
                    j + 1
                )));
            }
            if params.g[i][j] > CPL_REALIZATION_TOL {
                return Err(SimulationError::Circuit(format!(
                    "CPL model '{}' has positive off-diagonal conductance G[{},{}], expected Maxwell form",
                    model_name,
                    i + 1,
                    j + 1
                )));
            }
        }

        let c_to_ref: f64 = params.c[i].iter().sum();
        if c_to_ref < -CPL_REALIZATION_TOL {
            return Err(SimulationError::Circuit(format!(
                "CPL model '{}' has non-passive capacitance row sum on conductor {}",
                model_name,
                i + 1
            )));
        }

        let g_to_ref: f64 = params.g[i].iter().sum();
        if g_to_ref < -CPL_REALIZATION_TOL {
            return Err(SimulationError::Circuit(format!(
                "CPL model '{}' has non-passive conductance row sum on conductor {}",
                model_name,
                i + 1
            )));
        }
    }

    for i in 0..conductors {
        for j in (i + 1)..conductors {
            let coupling_limit = (params.l[i][i] * params.l[j][j]).sqrt();
            if params.l[i][j].abs() > coupling_limit + CPL_REALIZATION_TOL {
                return Err(SimulationError::Circuit(format!(
                    "CPL model '{}' has mutual inductance L[{},{}]={} exceeding sqrt(Lii*Ljj)={}",
                    model_name,
                    i + 1,
                    j + 1,
                    params.l[i][j],
                    coupling_limit
                )));
            }
        }
    }

    Ok(())
}

pub(in crate::engine::builder) fn allocate_grounded_cpl_native_branch_currents(
    circuit: &mut CircuitData,
    coupled_tline_index: usize,
) -> Result<(), SimulationError> {
    let (name, conductors) = {
        let tline = circuit
            .coupled_tlines
            .get(coupled_tline_index)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "CPL native branch-current allocation references missing coupled line index {}",
                    coupled_tline_index
                ))
            })?;
        if !tline.has_grounded_references() {
            return Err(SimulationError::Circuit(format!(
                "CPL native branch-current topology for '{}' is only supported when both references are ground",
                tline.name
            )));
        }
        if tline.native_branch_ordinals().is_some() {
            return Ok(());
        }
        (tline.name.clone(), tline.conductors())
    };

    let mut b1 = Vec::with_capacity(conductors);
    let mut b2 = Vec::with_capacity(conductors);
    for conductor in 0..conductors {
        b1.push(circuit.allocate_branch_named(&format!("{}#b1[{}]", name, conductor + 1)));
        b2.push(circuit.allocate_branch_named(&format!("{}#b2[{}]", name, conductor + 1)));
    }

    let tline = circuit
        .coupled_tlines
        .get_mut(coupled_tline_index)
        .expect("validated coupled line index remains present");
    tline
        .set_native_branch_ordinals(b1, b2)
        .map_err(SimulationError::Circuit)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grounded_cpl_netlist() -> Netlist {
        Netlist::parse(
            r#"grounded cpl
P1 n1 n2 0 f1 f2 0 m
.model m cpl
+ r = 2 0.5 3
+ l = 1n 0.1n 1.2n
+ c = 1p -0.1p 1.1p
+ g = 0 0 0
+ length = 4
.end
"#,
        )
        .expect("test netlist parses")
    }

    fn triplet_value(matrix: &crate::solver::TripletMatrix, row: usize, col: usize) -> f64 {
        matrix
            .row_indices
            .iter()
            .zip(matrix.col_indices.iter())
            .zip(matrix.values.iter())
            .filter_map(|((&r, &c), &value)| (r == row && c == col).then_some(value))
            .sum()
    }

    fn valid_cpl_params_with_r(r: Vec<Vec<f64>>) -> CplModelParams {
        CplModelParams {
            r,
            l: vec![vec![1.0e-9, 1.0e-10], vec![1.0e-10, 1.0e-9]],
            c: vec![vec![1.0e-12, -1.0e-13], vec![-1.0e-13, 1.0e-12]],
            g: vec![vec![0.0, 0.0], vec![0.0, 0.0]],
            length: 1.0,
        }
    }

    #[test]
    fn cpl_validation_allows_finite_off_diagonal_series_resistance() {
        let params = valid_cpl_params_with_r(vec![vec![0.5, 1.0e-4], vec![1.0e-4, 0.5]]);

        validate_cpl_model_params("m", &params).expect("finite off-diagonal R is accepted");
    }

    #[test]
    fn cpl_validation_preserves_finite_negative_off_diagonal_series_resistance() {
        let params = valid_cpl_params_with_r(vec![vec![0.5, -1.0e-3], vec![-1.0e-3, 0.5]]);

        validate_cpl_model_params("m", &params)
            .expect("finite negative off-diagonal R is accepted");
    }

    #[test]
    fn transmission_cpl_grounded_native_branch_currents_are_active_by_default() {
        // The native ngspice-faithful convolution runtime is activated for
        // grounded coupled lines: building the circuit allocates one branch
        // current per conductor at each end so the transient solver can stamp
        // the cplload.c branch equations.
        let netlist = grounded_cpl_netlist();
        let circuit = crate::engine::Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        let tline = &circuit.coupled_tlines[0];
        assert!(tline.native_runtime_available());
        let branches = tline
            .native_branch_ordinals()
            .expect("native branch ordinals are allocated by default");
        assert_eq!(branches.b1.len(), 2);
        assert_eq!(branches.b2.len(), 2);
        // Two conductors * two ends = four branch-current unknowns.
        assert_eq!(circuit.num_branches(), 4);
    }

    #[test]
    fn transmission_cpl_grounded_native_branch_allocation_topology_and_linking() {
        let netlist = grounded_cpl_netlist();
        let engine = crate::engine::Engine::default();
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");

        allocate_grounded_cpl_native_branch_currents(&mut circuit, 0)
            .expect("grounded CPL native branches allocate");

        assert_eq!(circuit.num_nodes(), 4);
        assert_eq!(circuit.num_branches(), 4);
        assert_eq!(
            circuit.branch_names_sorted(),
            vec!["P1#b1[1]", "P1#b2[1]", "P1#b1[2]", "P1#b2[2]"]
        );
        let branches = circuit.coupled_tlines[0]
            .native_branch_ordinals()
            .expect("native branch ordinals are present");
        assert_eq!(branches.b1, vec![1, 3]);
        assert_eq!(branches.b2, vec![2, 4]);

        let matrix = engine.build_matrix(&circuit).expect("matrix builds");
        let n1 = circuit.get_node_by_name("n1").expect("n1 exists");
        let n2 = circuit.get_node_by_name("n2").expect("n2 exists");
        let f1 = circuit.get_node_by_name("f1").expect("f1 exists");
        let f2 = circuit.get_node_by_name("f2").expect("f2 exists");
        let b1_1 = circuit.get_branch_matrix_index(1);
        let b2_1 = circuit.get_branch_matrix_index(2);
        let b1_2 = circuit.get_branch_matrix_index(3);
        let b2_2 = circuit.get_branch_matrix_index(4);

        assert_eq!(matrix.nrows, 8);
        assert!(matrix.get_index(n1 - 1, b1_1 - 1).is_some());
        assert!(matrix.get_index(f1 - 1, b2_1 - 1).is_some());
        assert!(matrix.get_index(b1_1 - 1, b1_1 - 1).is_some());
        assert!(matrix.get_index(b1_1 - 1, b2_1 - 1).is_some());
        assert!(matrix.get_index(b2_1 - 1, n1 - 1).is_some());
        assert!(matrix.get_index(b2_1 - 1, f1 - 1).is_some());
        assert!(matrix.get_index(b2_1 - 1, b1_1 - 1).is_some());
        assert!(matrix.get_index(n2 - 1, b1_2 - 1).is_some());
        assert!(matrix.get_index(f2 - 1, b2_2 - 1).is_some());
        assert!(matrix.get_index(n1 - 1, f1 - 1).is_none());
        assert!(matrix.get_index(n1 - 1, b1_2 - 1).is_none());

        for &row in &[b1_1, b1_2] {
            for &col in &[n1, n2, f1, f2] {
                assert!(
                    matrix.get_index(row - 1, col - 1).is_some(),
                    "CPL b1 row {row} should reserve node column {col}"
                );
            }
            for &col in &[b2_1, b2_2] {
                assert!(
                    matrix.get_index(row - 1, col - 1).is_some(),
                    "CPL b1 row {row} should reserve b2 column {col}"
                );
            }
        }
        for &row in &[b2_1, b2_2] {
            for &col in &[n1, n2, f1, f2] {
                assert!(
                    matrix.get_index(row - 1, col - 1).is_some(),
                    "CPL b2 row {row} should reserve node column {col}"
                );
            }
            for &col in &[b1_1, b1_2] {
                assert!(
                    matrix.get_index(row - 1, col - 1).is_some(),
                    "CPL b2 row {row} should reserve b1 column {col}"
                );
            }
        }

        circuit.link_indices(&matrix);
        let matrix_branches = circuit.coupled_tlines[0]
            .native_branch_matrix_indices()
            .expect("native branch matrix indices are present");
        assert_eq!(matrix_branches.b1, vec![5, 7]);
        assert_eq!(matrix_branches.b2, vec![6, 8]);
    }

    #[test]
    fn transmission_cpl_grounded_native_dc_stamp_matches_ngspice_shape() {
        let netlist = grounded_cpl_netlist();
        let engine = crate::engine::Engine::default();
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");

        allocate_grounded_cpl_native_branch_currents(&mut circuit, 0)
            .expect("grounded CPL native branches allocate");

        let mut matrix = circuit.create_matrix();
        let mut rhs = circuit.create_rhs();
        circuit.stamp_dc(&mut matrix, &mut rhs);

        let n1 = circuit.get_node_by_name("n1").expect("n1 exists");
        let n2 = circuit.get_node_by_name("n2").expect("n2 exists");
        let f1 = circuit.get_node_by_name("f1").expect("f1 exists");
        let f2 = circuit.get_node_by_name("f2").expect("f2 exists");
        let b1_1 = circuit.get_branch_matrix_index(1);
        let b2_1 = circuit.get_branch_matrix_index(2);
        let b1_2 = circuit.get_branch_matrix_index(3);
        let b2_2 = circuit.get_branch_matrix_index(4);

        assert_eq!(triplet_value(&matrix, n1 - 1, b1_1 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, f1 - 1, b2_1 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b1_1 - 1, b1_1 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b1_1 - 1, b2_1 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b2_1 - 1, n1 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b2_1 - 1, f1 - 1), -1.0);
        assert_eq!(triplet_value(&matrix, b2_1 - 1, b1_1 - 1), -8.0);
        assert_eq!(triplet_value(&matrix, b2_1 - 1, b1_2 - 1), 0.0);

        assert_eq!(triplet_value(&matrix, n2 - 1, b1_2 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, f2 - 1, b2_2 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b1_2 - 1, b1_2 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b1_2 - 1, b2_2 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b2_2 - 1, n2 - 1), 1.0);
        assert_eq!(triplet_value(&matrix, b2_2 - 1, f2 - 1), -1.0);
        assert_eq!(triplet_value(&matrix, b2_2 - 1, b1_2 - 1), -12.0);

        assert!(rhs.iter().all(|value| *value == 0.0));
    }
}

pub(in crate::engine::builder) fn build_cpl_multiconductor_line(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &crate::netlist::Element,
    model_name: &str,
) -> Result<(), SimulationError> {
    if element.nodes.len() < 6 || !element.nodes.len().is_multiple_of(2) {
        return Err(SimulationError::Circuit(format!(
            "CPL transmission line '{}' requires 2*N+2 nodes (N conductors plus shared reference)",
            element.name
        )));
    }

    let conductors = element.nodes.len() / 2 - 1;
    if conductors < 2 {
        return Err(SimulationError::Circuit(format!(
            "CPL transmission line '{}' requires at least two signal conductors",
            element.name
        )));
    }

    let params = resolve_cpl_model_params(netlist, model_name, conductors)?.ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Transmission line '{}' references unknown model '{}'",
            element.name, model_name
        ))
    })?;
    validate_cpl_model_params(model_name, &params)?;

    let near_ref = circuit.get_or_create_node(&element.nodes[conductors]);
    let far_ref = circuit.get_or_create_node(&element.nodes[element.nodes.len() - 1]);
    let near_nodes: Vec<usize> = (0..conductors)
        .map(|idx| circuit.get_or_create_node(&element.nodes[idx]))
        .collect();
    let far_nodes: Vec<usize> = (0..conductors)
        .map(|idx| circuit.get_or_create_node(&element.nodes[conductors + 1 + idx]))
        .collect();

    let coupled_tline = crate::device::CoupledTransmissionLine::new(
        element.name.clone(),
        near_nodes,
        near_ref,
        far_nodes,
        far_ref,
        &params.r,
        &params.l,
        &params.c,
        &params.g,
        params.length,
    )
    .map_err(SimulationError::Circuit)?;
    let native_available = coupled_tline.native_runtime_available();
    if let Some(min_taul) = coupled_tline.native_min_taul_seconds() {
        // ngspice caps CKTmaxStep to 0.9*min(taul) for CPL so every mode's
        // propagation delay is resolved by the convolution recurrence. Match
        // that cap so the trapezoidal convolution tracks the reference instead
        // of drifting when the solver would otherwise take larger steps.
        if min_taul.is_finite() && min_taul > 0.0 {
            circuit.tighten_transient_max_step_hint(0.9 * min_taul);
        }
    } else {
        let min_mode_delay = coupled_tline.min_mode_delay();
        if min_mode_delay.is_finite() && min_mode_delay > 0.0 {
            circuit.tighten_transient_max_step_hint(0.25 * min_mode_delay);
        }
    }
    let coupled_index = circuit.coupled_tlines.len();
    circuit.coupled_tlines.push(coupled_tline);

    // Activate the ngspice-faithful native convolution runtime: allocate the
    // per-conductor branch-current unknowns at both ends so the transient
    // solver stamps the cplload.c branch equations. Lines without grounded
    // references (or where the native setup math could not be realized) keep the
    // modal Norton transient fallback and allocate no branch rows.
    if native_available {
        allocate_grounded_cpl_native_branch_currents(circuit, coupled_index)?;
    }

    Ok(())
}

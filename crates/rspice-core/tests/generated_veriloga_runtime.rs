#[cfg(any(
    feature = "veriloga-model-pspnqs104va",
    feature = "veriloga-model-l-utsoi-485e0ac9"
))]
use rspice_core::device::veriloga_builtins::GeneratedReactiveStamper;
#[cfg(feature = "veriloga-model-hicuml2va")]
use rspice_core::device::veriloga_builtins::builtins::hicuml2_v320__hicuml2va__25e676cf as hicuml2;
use rspice_core::device::veriloga_builtins::{
    GeneratedAnalysisKind, GeneratedEvalContext, GeneratedStamper, GeneratedStaticStampCache,
};
#[cfg(feature = "veriloga-builtins")]
use rspice_core::device::veriloga_builtins::{builtins, instantiate_builtin};
#[cfg(any(
    feature = "veriloga-model-pspnqs104va",
    feature = "veriloga-model-l-utsoi-485e0ac9"
))]
use rspice_core::solver::ComplexMatrix;
use rspice_core::solver::StaticMatrix;
#[cfg(feature = "veriloga-builtins")]
use rspice_core::{CircuitData, netlist::ParamContext};

#[cfg(feature = "veriloga-model-pspnqs104va")]
#[test]
fn generated_nqs_integrators_stamp_inverse_frequency_on_potential_rows() {
    use rspice_core::device::veriloga_builtins::builtins::psp104_nqs__pspnqs104va__1d25ee14::Instance;

    let nodes = (1..=Instance::NODE_COUNT).collect::<Vec<_>>();
    let branches = (1..=Instance::BRANCH_COUNT).collect::<Vec<_>>();
    let width = nodes.len() + branches.len();
    let entries = (0..width)
        .flat_map(|row| (0..width).map(move |column| (row, column, 0.0)))
        .collect::<Vec<_>>();
    let structure = StaticMatrix::from_triplets(width, width, &entries).unwrap();
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&structure, &nodes, &branches, nodes.len());
    let mut matrix = ComplexMatrix::from_real_structure(&structure);
    let mut voltages = vec![0.0; width];
    voltages[0] = 0.1;
    voltages[1] = 1.0;
    for (index, name) in Instance::INTERNAL_NODE_NAMES.iter().enumerate() {
        voltages[index + 4] = match *name {
            "gp" => 1.0,
            "di" => 0.1,
            name if name.starts_with("int") => 0.01,
            _ => 0.0,
        };
    }
    let ctx = GeneratedEvalContext::with_analysis(
        &voltages,
        300.15,
        nodes.len(),
        GeneratedAnalysisKind::Ac,
    );
    let mut instance = Instance::new(&nodes);
    instance.set_branch_indices(&branches);
    instance.set_parameter("swnqs", 1.0).unwrap();
    instance.finalize_parameters().unwrap();
    instance.stamp(
        &ctx,
        &mut GeneratedStamper::new_ac_real_with_static_cache(
            &mut matrix,
            &voltages,
            nodes.len(),
            &cache,
        ),
    );
    assert!(
        !ctx.evaluation_failed(),
        "{:?}",
        ctx.take_evaluation_error()
    );
    let history = instance.capture_rollback_state();
    // Each collocation node has a resistor branch and an independent spline
    // voltage source. Merging their currents loses the resistor's loading.
    let real = matrix.to_dense_real();
    for name in [
        "int1", "int2", "int3", "int4", "int5", "int6", "int7", "int8", "int9",
    ] {
        let internal = Instance::INTERNAL_NODE_NAMES
            .iter()
            .position(|node| *node == name)
            .unwrap();
        let couplings = real[4 + internal][nodes.len()..]
            .iter()
            .filter(|value| value.abs() > 0.5)
            .count();
        assert_eq!(
            couplings, 2,
            "{name} must couple to separate RES and SPLINE currents"
        );
    }
    let mut samples = Vec::new();
    for omega in [1e6, 2e6] {
        matrix.clear_values();
        instance.stamp_reactive(
            &ctx,
            &mut GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
                &mut matrix,
                &nodes,
                &branches,
                nodes.len(),
                omega,
                &cache,
            ),
        );
        assert!(
            !ctx.evaluation_failed(),
            "{:?}",
            ctx.take_evaluation_error()
        );
        samples.push(matrix.to_dense_imag());
        assert_eq!(instance.capture_rollback_state(), history);
    }
    // PSP's SPLINE potential equations contain idt(-Tnorm*fk, Qp_0).
    // Their Jacobians must scale as 1/omega. This block was entirely zero
    // when generated AC extracted only ddt charges.
    let mut nonzero = 0;
    for (first_row, second_row) in samples[0].iter().zip(&samples[1]).skip(nodes.len()) {
        for (&first, &second) in first_row.iter().zip(second_row) {
            let doubled = 2.0 * second;
            assert!(first.is_finite() && doubled.is_finite());
            assert!((first - doubled).abs() <= 1e-12 * first.abs());
            nonzero += usize::from(first != 0.0);
        }
    }
    assert!(
        nonzero > 0,
        "NQS integrator Jacobians must reach the AC matrix"
    );
}

#[cfg(feature = "veriloga-model-l-utsoi-485e0ac9")]
#[test]
fn generated_utsoi_nqs_capacitors_reference_global_ground() {
    use rspice_core::device::veriloga_builtins::builtins::l_utsoi_102_nqs__l_utsoi__485e0ac9::Instance;

    // The source declares `electrical gndnqs; ground gndnqs;`. Allocating it
    // as an internal node leaves the NQS network floating instead of grounded.
    assert!(!Instance::INTERNAL_NODE_NAMES.contains(&"gndnqs"));
    let nodes = (1..=Instance::NODE_COUNT).collect::<Vec<_>>();
    // Counted from one rather than written `1..=BRANCH_COUNT`: this module
    // declares no branch unknowns, and an inclusive range ending at zero is an
    // empty range spelled backwards.
    let branches = (1..).take(Instance::BRANCH_COUNT).collect::<Vec<_>>();
    let width = nodes.len() + branches.len();
    let entries = (0..width)
        .flat_map(|row| (0..width).map(move |column| (row, column, 0.0)))
        .collect::<Vec<_>>();
    let structure = StaticMatrix::from_triplets(width, width, &entries).unwrap();
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&structure, &nodes, &branches, nodes.len());
    let mut matrix = ComplexMatrix::from_real_structure(&structure);
    let mut voltages = vec![0.0; width];
    voltages[0] = 0.1;
    voltages[1] = 1.0;
    for (index, name) in Instance::INTERNAL_NODE_NAMES.iter().enumerate() {
        voltages[5 + index] = match *name {
            "gp" => 1.0,
            "di" => 0.1,
            _ => 0.0,
        };
    }
    let ctx = GeneratedEvalContext::with_analysis(
        &voltages,
        300.15,
        nodes.len(),
        GeneratedAnalysisKind::Ac,
    );
    let mut instance = Instance::new(&nodes);
    instance.set_branch_indices(&branches);
    instance.finalize_parameters().unwrap();
    instance.stamp(
        &ctx,
        &mut GeneratedStamper::new_ac_real_with_static_cache(
            &mut matrix,
            &voltages,
            nodes.len(),
            &cache,
        ),
    );
    instance.stamp_reactive(
        &ctx,
        &mut GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
            &mut matrix,
            &nodes,
            &branches,
            nodes.len(),
            1e6,
            &cache,
        ),
    );
    assert!(
        !ctx.evaluation_failed(),
        "{:?}",
        ctx.take_evaluation_error()
    );
    let reactive = matrix.to_dense_imag();
    for name in ["Gnqs", "Dnqs"] {
        let row = 5 + Instance::INTERNAL_NODE_NAMES
            .iter()
            .position(|node| *node == name)
            .unwrap();
        // Both state equations explicitly contribute ddt(1e-9*V(node,gndnqs)).
        assert!(
            (reactive[row][row] - 1e-3).abs() < 1e-12,
            "{name}: {:?}",
            reactive[row]
        );
    }
}

#[cfg(feature = "veriloga-model-angelov")]
#[test]
fn generated_angelov_series_resistance_keeps_its_physical_current_column() {
    use rspice_core::device::veriloga_builtins::builtins::angelov__angelov__98b92059::Instance;

    let nodes = (1..=Instance::NODE_COUNT).collect::<Vec<_>>();
    let branches = (1..=Instance::BRANCH_COUNT).collect::<Vec<_>>();
    let width = nodes.len() + branches.len();
    let entries = (0..width)
        .flat_map(|row| (0..width).map(move |column| (row, column, 0.0)))
        .collect::<Vec<_>>();
    let mut matrix = StaticMatrix::from_triplets(width, width, &entries).unwrap();
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, nodes.len());
    let voltages = vec![0.0; width];
    let ctx = GeneratedEvalContext::new(&voltages, 300.15, nodes.len());
    let mut instance = Instance::new(&nodes);
    instance.set_branch_indices(&branches);
    instance.set_parameter("Rd2", 0.0).unwrap();
    instance.set_parameter("Tcrs", 0.0).unwrap();
    let local = |name| {
        3 + Instance::INTERNAL_NODE_NAMES
            .iter()
            .position(|node| *node == name)
            .unwrap()
    };
    let (si, sii) = (local("si"), local("sii"));
    for resistance in [0.05, 0.125] {
        instance.set_parameter("Rs", resistance).unwrap();
        instance.finalize_parameters().unwrap();
        matrix.clear_values();
        let mut rhs = vec![0.0; width];
        instance.stamp(
            &ctx,
            &mut GeneratedStamper::new_with_static_cache(
                &mut matrix,
                &mut rhs,
                &voltages,
                nodes.len(),
                &cache,
            ),
        );
        assert!(
            !ctx.evaluation_failed(),
            "{:?}",
            ctx.take_evaluation_error()
        );
        let positions = matrix.stored_positions().collect::<Vec<_>>();
        let mut dense = vec![vec![0.0; width]; width];
        for ((row, column), &value) in positions.into_iter().zip(matrix.values_mut().iter()) {
            dense[row][column] = value;
        }
        let rows = (nodes.len()..width)
            .filter(|&row| dense[row][si].abs() == 1.0 && dense[row][sii] == -dense[row][si])
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 1, "Rs must own one physical branch row");
        let row = rows[0];
        // Several earlier V(g,gi) contributions must not shift the column
        // differentiating V(si,sii) = Rs * I(si,sii).
        assert!((dense[row][row].abs() - resistance).abs() < 1e-12);
        assert_eq!(dense[si][row].abs(), 1.0);
        assert_eq!(dense[sii][row], -dense[si][row]);
    }
}

#[cfg(feature = "veriloga-model-hicuml2va")]
fn stamp_hicuml2(instance: &mut hicuml2::Instance, temperature: f64) -> (Vec<u64>, Vec<u64>) {
    const NODE_COUNT: usize = hicuml2::Instance::NODE_COUNT;
    const BRANCH_COUNT: usize = hicuml2::Instance::BRANCH_COUNT;
    let axis_count = NODE_COUNT + BRANCH_COUNT;
    let triplets = (0..axis_count)
        .flat_map(|row| (0..axis_count).map(move |column| (row, column, 0.0)))
        .collect::<Vec<_>>();
    let mut matrix =
        StaticMatrix::from_triplets(axis_count, axis_count, &triplets).expect("dense test matrix");
    let mut rhs = vec![0.0; axis_count];
    let voltages = (0..axis_count)
        .map(|index| (index as f64 - 7.0) * 1.0e-3)
        .collect::<Vec<_>>();
    let nodes = (1..=NODE_COUNT).collect::<Vec<_>>();
    let branches = (1..=BRANCH_COUNT).collect::<Vec<_>>();
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, NODE_COUNT);
    let context = GeneratedEvalContext::new(&voltages, temperature, NODE_COUNT);
    let mut stamper = GeneratedStamper::new_with_static_cache(
        &mut matrix,
        &mut rhs,
        &voltages,
        NODE_COUNT,
        &cache,
    );
    instance.stamp(&context, &mut stamper);

    (
        matrix
            .values_mut()
            .iter()
            .map(|value| value.to_bits())
            .collect(),
        rhs.iter().map(|value| value.to_bits()).collect(),
    )
}

#[cfg(feature = "veriloga-model-hicuml2va")]
fn new_hicuml2() -> hicuml2::Instance {
    let nodes = (1..=hicuml2::Instance::NODE_COUNT).collect::<Vec<_>>();
    let branches = (1..=hicuml2::Instance::BRANCH_COUNT).collect::<Vec<_>>();
    let mut instance = hicuml2::Instance::new(&nodes);
    instance.set_branch_indices(&branches);
    instance
}

#[cfg(feature = "veriloga-model-hicuml2va")]
#[test]
fn generated_structured_cache_preserves_temperature_and_parameter_semantics() {
    let mut warmed = new_hicuml2();
    let _ = stamp_hicuml2(&mut warmed, 300.15);
    let warmed_same_temperature = stamp_hicuml2(&mut warmed, 300.15);

    let mut cold = new_hicuml2();
    let cold_same_temperature = stamp_hicuml2(&mut cold, 300.15);
    assert_eq!(
        warmed_same_temperature, cold_same_temperature,
        "reusing structured static segments must preserve stamp results"
    );

    let warmed_new_temperature = stamp_hicuml2(&mut warmed, 350.15);
    let mut cold_new_temperature = new_hicuml2();
    assert_eq!(
        warmed_new_temperature,
        stamp_hicuml2(&mut cold_new_temperature, 350.15),
        "temperature-static segments must be invalidated when temperature changes"
    );

    let mut original = new_hicuml2();
    let _ = stamp_hicuml2(&mut original, 300.15);
    let mut changed = original.clone();
    changed
        .set_parameter("flcomp", 299.0)
        .expect("valid HICUM parameter override");

    let changed_stamp = stamp_hicuml2(&mut changed, 300.15);
    let mut cold_changed = new_hicuml2();
    cold_changed
        .set_parameter("flcomp", 299.0)
        .expect("valid HICUM parameter override");
    assert_eq!(
        changed_stamp,
        stamp_hicuml2(&mut cold_changed, 300.15),
        "parameter changes must invalidate instance- and temperature-static segments"
    );

    let original_stamp = stamp_hicuml2(&mut original, 300.15);
    let mut cold_original = new_hicuml2();
    assert_eq!(
        original_stamp,
        stamp_hicuml2(&mut cold_original, 300.15),
        "copy-on-write cache invalidation must not alter the cloned source instance"
    );
}

#[test]
fn generated_and_core_thermal_voltage_preserve_the_si_ratio_and_extreme_ranges() {
    // Independently rounded from the exact SI ratio 1380649 / 16021766340,
    // multiplied by each temperature's exact binary64 value.
    for (temperature, expected) in [
        (300.15, 0x3f9a_7c55_c976_86ec),
        (1e-310, 0x0000_0000_67f5_e2f6),
        (1e308, 0x7f09_21cb_7e69_c6d6),
        (-0.0, 0x8000_0000_0000_0000),
    ] {
        let context = GeneratedEvalContext::new(&[], temperature, 0);
        assert_eq!(context.thermal_voltage().to_bits(), expected);
        assert_eq!(
            rspice_core::constants::thermal_voltage(temperature).to_bits(),
            expected
        );
    }
    assert_eq!(
        rspice_core::constants::VT_REFERENCE,
        rspice_core::constants::thermal_voltage(rspice_core::constants::TEMP_REFERENCE)
    );
}

#[test]
fn generated_eval_context_reports_analysis_mode() {
    let voltages = [1.0, 0.5];
    let dc = GeneratedEvalContext::new(&voltages, 300.15, 2);
    assert_eq!(dc.node_voltage(0), 0.0);
    assert_eq!(dc.node_voltage(1), 1.0);
    assert_eq!(dc.node_voltage(2), 0.5);
    assert_eq!(dc.temperature(), 300.15);
    assert!(dc.analysis("dc"));
    assert!(dc.analysis("op"));
    assert!(dc.analysis("static"));
    assert!(!dc.analysis("ac"));
    assert!(!dc.analysis("smallsig"));

    let ac = GeneratedEvalContext::with_analysis(&voltages, 300.15, 2, GeneratedAnalysisKind::Ac);
    assert!(ac.analysis("ac"));
    assert!(ac.analysis("smallsig"));
    assert!(!ac.analysis("dc"));
    assert!(!ac.analysis("noise"));
}

#[test]
fn generated_static_stamp_cache_retains_only_existing_csc_slots() {
    let nodes = (1..=80).collect::<Vec<_>>();
    let branches: [usize; 0] = [];
    let matrix = StaticMatrix::from_triplets(80, 80, &[(0, 0, 0.0), (7, 19, 0.0), (79, 79, 0.0)])
        .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();

    cache.link(&matrix, &nodes, &branches, 80);

    assert_eq!(cache.linked_slot_count(), 3);
}

#[test]
fn generated_cached_sparse_current_stamper_uses_static_axes() {
    let nodes = [1, 2, 3];
    let branches = [1];
    let voltages = [1.0, 2.0, 3.0, 4.0];
    let mut rhs = vec![0.0; 4];
    let mut matrix = StaticMatrix::from_triplets(
        4,
        4,
        &[
            (0, 0, 0.0),
            (0, 2, 0.0),
            (0, 3, 0.0),
            (1, 0, 0.0),
            (1, 2, 0.0),
            (1, 3, 0.0),
        ],
    )
    .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, 3);

    GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 3, &cache)
        .stamp_current_sparse_local::<2, 1>(
            Some(0),
            Some(1),
            5.0,
            [0, 2],
            [1.0, 3.0],
            [0],
            [4.0],
            2.0,
        );

    let row0_node0 = matrix.get_index(0, 0).expect("row0 node0").0;
    let row0_node2 = matrix.get_index(0, 2).expect("row0 node2").0;
    let row0_branch0 = matrix.get_index(0, 3).expect("row0 branch0").0;
    let row1_node0 = matrix.get_index(1, 0).expect("row1 node0").0;
    let row1_node2 = matrix.get_index(1, 2).expect("row1 node2").0;
    let row1_branch0 = matrix.get_index(1, 3).expect("row1 branch0").0;
    let values = matrix.values_mut();
    assert_eq!(values[row0_node0], 2.0);
    assert_eq!(values[row0_node2], 6.0);
    assert_eq!(values[row0_branch0], 8.0);
    assert_eq!(values[row1_node0], -2.0);
    assert_eq!(values[row1_node2], -6.0);
    assert_eq!(values[row1_branch0], -8.0);

    let equivalent = 5.0 - 2.0 * voltages[0] - 6.0 * voltages[2] - 8.0 * voltages[3];
    assert_eq!(rhs[0], -equivalent);
    assert_eq!(rhs[1], equivalent);
    assert_eq!(rhs[2], 0.0);
    assert_eq!(rhs[3], 0.0);
}

#[test]
fn generated_cached_sparse_potential_stamper_uses_static_axes() {
    let nodes = [1, 2];
    let branches = [1];
    let voltages = [1.0, 2.0, 3.0];
    let mut rhs = vec![0.0; 3];
    let mut matrix = StaticMatrix::from_triplets(3, 3, &[(2, 0, 0.0), (2, 1, 0.0), (2, 2, 0.0)])
        .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, 2);

    GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 2, &cache)
        .stamp_potential_sparse_local::<2, 1>(0, 7.0, [0, 1], [1.5, -0.25], [0], [2.0]);

    let n0 = matrix.get_index(2, 0).expect("n0 entry").0;
    let n1 = matrix.get_index(2, 1).expect("n1 entry").0;
    let b0 = matrix.get_index(2, 2).expect("b0 entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[n0], -1.5);
    assert_eq!(values[n1], 0.25);
    assert_eq!(values[b0], -2.0);

    let equivalent = 7.0 - 1.5 * voltages[0] - (-0.25) * voltages[1] - 2.0 * voltages[2];
    assert_eq!(rhs[2], equivalent);
    assert_eq!(rhs[0], 0.0);
    assert_eq!(rhs[1], 0.0);
}

#[test]
fn generated_cached_potential_branch_stamper_uses_static_axes() {
    let nodes = [1, 2];
    let branches = [1];
    let voltages = [1.0, 2.0, 3.0];
    let mut rhs = vec![0.0; 3];
    let mut matrix =
        StaticMatrix::from_triplets(3, 3, &[(0, 2, 0.0), (1, 2, 0.0), (2, 0, 0.0), (2, 1, 0.0)])
            .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, 2);

    GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 2, &cache)
        .stamp_potential_branch_local(Some(0), Some(1), 0, 3.0);

    let p_branch = matrix.get_index(0, 2).expect("p branch entry").0;
    let n_branch = matrix.get_index(1, 2).expect("n branch entry").0;
    let branch_p = matrix.get_index(2, 0).expect("branch p entry").0;
    let branch_n = matrix.get_index(2, 1).expect("branch n entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[p_branch], 3.0);
    assert_eq!(values[n_branch], -3.0);
    assert_eq!(values[branch_p], 1.0);
    assert_eq!(values[branch_n], -1.0);
    assert_eq!(rhs, vec![0.0, 0.0, 0.0]);
}

#[cfg(feature = "veriloga-builtins")]
#[test]
fn generated_builtin_allocates_internal_nodes_from_metadata() {
    if !builtins::builtin_names()
        .iter()
        .any(|name| name.eq_ignore_ascii_case("internal_res"))
    {
        eprintln!("internal_res generated builtin not present; skipping fixture-specific test");
        return;
    }

    assert_eq!(builtins::node_count("internal_res"), Some(2));
    assert_eq!(builtins::total_node_count("internal_res"), Some(3));
    assert_eq!(
        builtins::internal_node_names("internal_res"),
        Some(["x"].as_slice())
    );

    let mut circuit = CircuitData::new();
    let device = instantiate_builtin(
        "internal_res",
        "X1",
        &["p".to_string(), "n".to_string()],
        &[],
        &ParamContext::new(),
        &mut circuit,
    )
    .expect("instantiate generated internal-node builtin")
    .expect("internal_res builtin must instantiate");

    assert_eq!(device.nodes.len(), 3);
    assert_eq!(device.nodes[0], circuit.get_node_by_name("p").unwrap());
    assert_eq!(device.nodes[1], circuit.get_node_by_name("n").unwrap());
    assert_eq!(
        device.nodes[2],
        circuit.get_node_by_name("X1.__x.internal").unwrap()
    );
    assert_eq!(circuit.num_nodes(), 3);
}

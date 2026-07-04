use rspice_core::device::veriloga_generated::{
    GeneratedAnalysisKind, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper,
    GeneratedStamper, GeneratedStaticStampCache,
};
#[cfg(feature = "veriloga-builtins")]
use rspice_core::device::veriloga_generated::{builtins, instantiate_builtin};
use rspice_core::solver::{ComplexMatrix, StaticMatrix};
#[cfg(feature = "veriloga-builtins")]
use rspice_core::{CircuitData, netlist::ParamContext};

#[cfg(feature = "veriloga-builtins")]
#[test]
fn generated_builtins_are_materialized_in_source_tree() {
    let generated_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/device/veriloga_generated");
    let registry_path = generated_root.join("registry.rs");
    let registry = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", registry_path.display()));

    assert!(
        !registry.contains("devices/"),
        "generated registry should point at direct per-device folders"
    );

    let module_paths: Vec<_> = registry
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("#[path = \"")
                .and_then(|rest| rest.strip_suffix("\"]"))
        })
        .collect();
    assert_eq!(
        module_paths.len(),
        builtins::builtin_names().len(),
        "registry should include one source-tree module path per generated builtin"
    );

    for relative in module_paths {
        assert!(
            relative.ends_with("/mod.rs") && !relative.starts_with("devices/"),
            "generated module path should be a direct device folder: {relative}"
        );
        let module_path = generated_root.join(relative);
        assert!(
            module_path.exists(),
            "generated module should exist at {}",
            module_path.display()
        );
    }
}

#[test]
fn generated_dense_stamper_abi_is_slice_based() {
    let generated_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/device/veriloga_generated");
    let runtime_path = generated_root.join("mod.rs");
    let runtime = std::fs::read_to_string(&runtime_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", runtime_path.display()));

    assert!(
        !runtime.contains("stamp_current_dense<const"),
        "dense current stamps should not monomorphize on node or branch array lengths"
    );
    assert!(
        !runtime.contains("stamp_potential_dense<\n        const"),
        "dense potential stamps should not monomorphize on node or branch array lengths"
    );
    assert!(
        !runtime.contains("[Value; DN]") && !runtime.contains("[usize; N]"),
        "dense stamp ABI should accept slices, not const-generic array references"
    );
    assert!(runtime.contains("node_derivatives: &[Value],"));
    assert!(runtime.contains("branch_derivatives: &[Value],"));
    assert!(
        !runtime.contains("nodes.get(index)") && !runtime.contains("branches.get(index)"),
        "dense stamp loops should zip aligned generated slices instead of probing by index:\n{runtime}"
    );
    assert!(
        !runtime.contains("stamp_current_dense_row(")
            && !runtime.contains("stamp_current_reactive_dense_row("),
        "dense current stampers should fuse equivalent/current row work instead of walking derivative slices once per row:\n{runtime}"
    );
    assert!(
        !runtime.contains("stamp_current_const_row(")
            && !runtime.contains("stamp_current_node1_row(")
            && !runtime.contains("stamp_current_node2_row(")
            && !runtime.contains("stamp_current_node3_row(")
            && !runtime.contains("stamp_current_branch1_row(")
            && !runtime.contains("stamp_current_branch2_row(")
            && !runtime.contains("stamp_current_reactive_node1_row(")
            && !runtime.contains("stamp_current_reactive_node2_row(")
            && !runtime.contains("stamp_current_reactive_node3_row(")
            && !runtime.contains("stamp_current_reactive_branch1_row(")
            && !runtime.contains("stamp_current_reactive_branch2_row("),
        "fixed-arity current stampers should stamp both terminal rows from one derivative path:\n{runtime}"
    );
}

#[test]
fn generated_stamper_uses_linked_static_stamp_slots() {
    let manifest_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let generated_root = manifest_root.join("src/device/veriloga_generated");
    let runtime_path = generated_root.join("mod.rs");
    let introspection_path = manifest_root.join("src/circuit/introspection.rs");

    let runtime = std::fs::read_to_string(&runtime_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", runtime_path.display()));
    let introspection = std::fs::read_to_string(&introspection_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", introspection_path.display()));

    assert!(
        runtime.contains("pub struct GeneratedStaticStampCache"),
        "generated runtime should expose a per-instance static stamp cache:\n{runtime}"
    );
    assert!(
        runtime.contains("pub fn stamp_current_dense_local(")
            && runtime.contains("pub fn stamp_potential_dense_local("),
        "generated runtime should provide local-axis stamp entry points:\n{runtime}"
    );
    assert!(
        runtime.contains("matrix.stamp_direct"),
        "generated static stamps should use precomputed CSC slots instead of StaticMatrix::add:\n{runtime}"
    );
    assert!(
        introspection.contains(".link_static_stamps(matrix, num_nodes);"),
        "CircuitData::link_indices should link generated Verilog-A static slots:\n{introspection}"
    );
}

#[test]
fn generated_runtime_restores_snapshots_without_discarding_scratch_storage() {
    let manifest_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let generated_root = manifest_root.join("src/device/veriloga_generated");
    let runtime_path = generated_root.join("mod.rs");
    let registry_path = generated_root.join("registry.rs");
    let nonlinear_path = manifest_root.join("src/circuit/nonlinear.rs");

    let runtime = std::fs::read_to_string(&runtime_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", runtime_path.display()));
    let registry = std::fs::read_to_string(&registry_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", registry_path.display()));
    let nonlinear = std::fs::read_to_string(&nonlinear_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", nonlinear_path.display()));
    let nonlinear_lf = nonlinear.replace("\r\n", "\n");

    assert!(
        runtime.contains("pub(crate) fn restore_from_snapshot(&mut self, snapshot: Self)"),
        "generated runtime should restore snapshots in place:\n{runtime}"
    );
    assert!(
        runtime.contains("active.restore_from_snapshot(snapshot)"),
        "generated device vectors should preserve active instance storage when counts match:\n{runtime}"
    );
    assert!(
        registry.contains("pub fn restore_from_snapshot(&mut self, snapshot: Self)"),
        "generated registry should dispatch variant-matched restores in place:\n{registry}"
    );
    assert!(
        registry.contains("active.restore_from_snapshot(*snapshot)"),
        "boxed generated registry variants should restore from the boxed snapshot value:\n{registry}"
    );
    assert!(
        nonlinear_lf.contains(
            "self.generated_veriloga_devices\n                .restore_from_snapshot(snapshot.generated_veriloga_devices);"
        ),
        "nonlinear restore should not replace generated devices wholesale:\n{nonlinear}"
    );
}

#[test]
fn generated_stamper_linearizes_current_contribution() {
    let voltages = [1.0, 0.5];
    let mut rhs = vec![0.0; 2];
    let mut matrix =
        StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)])
            .expect("static matrix");

    let ctx = GeneratedEvalContext::new(&voltages, 300.15, 2);
    assert_eq!(ctx.node_voltage(0), 0.0);
    assert_eq!(ctx.node_voltage(1), 1.0);
    assert_eq!(ctx.node_voltage(2), 0.5);
    assert_eq!(ctx.temperature(), 300.15);
    assert!(
        (ctx.thermal_voltage() - rspice_core::constants::thermal_voltage(300.15)).abs() < 1e-18
    );

    GeneratedStamper::new(&mut matrix, &mut rhs, &voltages, 2).stamp_current(
        Some(1),
        Some(2),
        0.2,
        &[
            GeneratedDerivative::node(1, 0.01),
            GeneratedDerivative::node(2, -0.01),
        ],
    );

    let pp = matrix.get_index(0, 0).expect("pp entry").0;
    let pn = matrix.get_index(0, 1).expect("pn entry").0;
    let np = matrix.get_index(1, 0).expect("np entry").0;
    let nn = matrix.get_index(1, 1).expect("nn entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[pp], 0.01);
    assert_eq!(values[pn], -0.01);
    assert_eq!(values[np], -0.01);
    assert_eq!(values[nn], 0.01);

    let equivalent = 0.2 - (0.01 * 1.0 + -0.01 * 0.5);
    assert_eq!(rhs[0], -equivalent);
    assert_eq!(rhs[1], equivalent);
}

#[test]
fn generated_eval_context_reports_analysis_mode() {
    let voltages = [1.0, 0.5];
    let dc = GeneratedEvalContext::new(&voltages, 300.15, 2);
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
fn generated_dense_current_stamper_linearizes_in_one_dense_path() {
    let voltages = [1.0, 2.0, 3.0, 4.0];
    let mut rhs = vec![0.0; 4];
    let mut matrix = StaticMatrix::from_triplets(
        4,
        4,
        &[
            (0, 0, 0.0),
            (0, 1, 0.0),
            (0, 2, 0.0),
            (0, 3, 0.0),
            (1, 0, 0.0),
            (1, 1, 0.0),
            (1, 2, 0.0),
            (1, 3, 0.0),
            (2, 0, 0.0),
            (2, 1, 0.0),
            (2, 2, 0.0),
            (2, 3, 0.0),
            (3, 0, 0.0),
            (3, 1, 0.0),
            (3, 2, 0.0),
            (3, 3, 0.0),
        ],
    )
    .expect("static matrix");

    GeneratedStamper::new(&mut matrix, &mut rhs, &voltages, 3).stamp_current_dense(
        Some(1),
        Some(2),
        5.0,
        &[1, 0, 3],
        &[0.5, 99.0, -1.0],
        &[1],
        &[2.0],
        3.0,
    );

    let row0_node1 = matrix.get_index(0, 0).expect("row0 node1").0;
    let row0_node3 = matrix.get_index(0, 2).expect("row0 node3").0;
    let row0_branch = matrix.get_index(0, 3).expect("row0 branch").0;
    let row1_node1 = matrix.get_index(1, 0).expect("row1 node1").0;
    let row1_node3 = matrix.get_index(1, 2).expect("row1 node3").0;
    let row1_branch = matrix.get_index(1, 3).expect("row1 branch").0;
    let values = matrix.values_mut();

    assert_eq!(values[row0_node1], 1.5);
    assert_eq!(values[row0_node3], -3.0);
    assert_eq!(values[row0_branch], 6.0);
    assert_eq!(values[row1_node1], -1.5);
    assert_eq!(values[row1_node3], 3.0);
    assert_eq!(values[row1_branch], -6.0);

    let equivalent = 5.0 - 1.5 * 1.0 - (-3.0) * 3.0 - 6.0 * 4.0;
    assert_eq!(rhs[0], -equivalent);
    assert_eq!(rhs[1], equivalent);
    assert_eq!(rhs[2], 0.0);
    assert_eq!(rhs[3], 0.0);
}

#[test]
fn generated_indexed_dense_current_stamper_skips_static_zero_lanes() {
    let nodes = [1, 2, 3, 4];
    let branches: [usize; 0] = [];
    let voltages = [1.0, 2.0, 3.0, 4.0];
    let mut rhs = vec![0.0; 4];
    let mut matrix =
        StaticMatrix::from_triplets(4, 4, &[(0, 0, 0.0), (0, 2, 0.0), (1, 0, 0.0), (1, 2, 0.0)])
            .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, 4);

    GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 4, &cache)
        .stamp_current_indexed_dense_local(
            Some(0),
            Some(1),
            5.0,
            &[0, 2],
            &[1.0, 3.0],
            &[],
            &[],
            2.0,
        );

    let p0 = matrix.get_index(0, 0).expect("p0 entry").0;
    let p2 = matrix.get_index(0, 2).expect("p2 entry").0;
    let n0 = matrix.get_index(1, 0).expect("n0 entry").0;
    let n2 = matrix.get_index(1, 2).expect("n2 entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[p0], 2.0);
    assert_eq!(values[p2], 6.0);
    assert_eq!(values[n0], -2.0);
    assert_eq!(values[n2], -6.0);

    let equivalent = 5.0 - 2.0 * voltages[0] - 6.0 * voltages[2];
    assert_eq!(rhs[0], -equivalent);
    assert_eq!(rhs[1], equivalent);
    assert_eq!(rhs[2], 0.0);
    assert_eq!(rhs[3], 0.0);
}

#[test]
fn generated_cached_dense_current_stamper_uses_static_axes() {
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
        .stamp_current_dense_local(Some(0), Some(1), 5.0, &[0.5, 0.0, -1.0], &[2.0], 3.0);

    let row0_node0 = matrix.get_index(0, 0).expect("row0 node0").0;
    let row0_node2 = matrix.get_index(0, 2).expect("row0 node2").0;
    let row0_branch0 = matrix.get_index(0, 3).expect("row0 branch0").0;
    let row1_node0 = matrix.get_index(1, 0).expect("row1 node0").0;
    let row1_node2 = matrix.get_index(1, 2).expect("row1 node2").0;
    let row1_branch0 = matrix.get_index(1, 3).expect("row1 branch0").0;
    let values = matrix.values_mut();
    assert_eq!(values[row0_node0], 1.5);
    assert_eq!(values[row0_node2], -3.0);
    assert_eq!(values[row0_branch0], 6.0);
    assert_eq!(values[row1_node0], -1.5);
    assert_eq!(values[row1_node2], 3.0);
    assert_eq!(values[row1_branch0], -6.0);

    let equivalent = 5.0 - 1.5 * voltages[0] - (-3.0) * voltages[2] - 6.0 * voltages[3];
    assert_eq!(rhs[0], -equivalent);
    assert_eq!(rhs[1], equivalent);
    assert_eq!(rhs[2], 0.0);
    assert_eq!(rhs[3], 0.0);
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
fn generated_cached_generic_current_stamper_uses_static_axes() {
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
        .stamp_current_local(
            Some(0),
            Some(1),
            5.0,
            &[
                GeneratedDerivative::node(0, 2.0),
                GeneratedDerivative::node(2, 6.0),
                GeneratedDerivative::branch(0, 8.0),
            ],
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
fn generated_ac_real_stamper_uses_linked_static_slots() {
    let nodes = [1, 2, 3, 4];
    let branches: [usize; 0] = [];
    let voltages = [1.0, 2.0, 3.0, 4.0];
    let real_matrix =
        StaticMatrix::from_triplets(4, 4, &[(0, 0, 0.0), (0, 2, 0.0), (1, 0, 0.0), (1, 2, 0.0)])
            .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&real_matrix, &nodes, &branches, 4);
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    GeneratedStamper::new_ac_real_with_static_cache(&mut matrix, &voltages, 4, &cache)
        .stamp_current_indexed_dense_local(
            Some(0),
            Some(1),
            0.0,
            &[0, 2],
            &[1.0, 3.0],
            &[],
            &[],
            2.0,
        );

    let real = matrix.to_dense_real();
    assert_eq!(real[0][0], 2.0);
    assert_eq!(real[0][2], 6.0);
    assert_eq!(real[1][0], -2.0);
    assert_eq!(real[1][2], -6.0);
    assert_eq!(real[0][1], 0.0);
    assert_eq!(real[2][0], 0.0);
}

#[test]
fn generated_dense_reactive_current_stamper_stamps_both_rows() {
    let real_matrix = StaticMatrix::from_triplets(
        4,
        4,
        &[
            (0, 0, 0.0),
            (0, 1, 0.0),
            (0, 2, 0.0),
            (0, 3, 0.0),
            (1, 0, 0.0),
            (1, 1, 0.0),
            (1, 2, 0.0),
            (1, 3, 0.0),
            (2, 0, 0.0),
            (2, 1, 0.0),
            (2, 2, 0.0),
            (2, 3, 0.0),
            (3, 0, 0.0),
            (3, 1, 0.0),
            (3, 2, 0.0),
            (3, 3, 0.0),
        ],
    )
    .expect("static matrix");
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    GeneratedReactiveStamper::new(&mut matrix, 3, 10.0).stamp_current_reactive_dense(
        Some(1),
        Some(2),
        &[1, 0, 3],
        &[0.5, 99.0, -1.0],
        &[1],
        &[2.0],
        3.0,
    );

    let imag = matrix.to_dense_imag();
    assert_eq!(imag[0][0], 15.0);
    assert_eq!(imag[0][2], -30.0);
    assert_eq!(imag[0][3], 60.0);
    assert_eq!(imag[1][0], -15.0);
    assert_eq!(imag[1][2], 30.0);
    assert_eq!(imag[1][3], -60.0);
    assert_eq!(imag[0][1], 0.0);
    assert_eq!(imag[2][0], 0.0);
}

#[test]
fn generated_reactive_stamper_uses_linked_static_slots() {
    let nodes = [1, 2, 3, 4];
    let branches: [usize; 0] = [];
    let real_matrix =
        StaticMatrix::from_triplets(4, 4, &[(0, 0, 0.0), (0, 2, 0.0), (1, 0, 0.0), (1, 2, 0.0)])
            .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&real_matrix, &nodes, &branches, 4);
    let mut matrix = ComplexMatrix::from_real_structure(&real_matrix);

    GeneratedReactiveStamper::new_with_static_cache(&mut matrix, 4, 10.0, &cache)
        .stamp_current_reactive_dense(Some(1), Some(2), &[1, 3], &[0.5, -1.0], &[], &[], 2.0);

    let imag = matrix.to_dense_imag();
    assert_eq!(imag[0][0], 10.0);
    assert_eq!(imag[0][2], -20.0);
    assert_eq!(imag[1][0], -10.0);
    assert_eq!(imag[1][2], 20.0);
    assert_eq!(imag[0][1], 0.0);
    assert_eq!(imag[2][0], 0.0);
}

#[test]
fn generated_stamper_linearizes_potential_contribution_on_branch_row() {
    let voltages = [1.0, 0.5, 0.0];
    let mut rhs = vec![0.0; 3];
    let mut matrix = StaticMatrix::from_triplets(
        3,
        3,
        &[
            (0, 0, 0.0),
            (0, 1, 0.0),
            (0, 2, 0.0),
            (1, 0, 0.0),
            (1, 1, 0.0),
            (1, 2, 0.0),
            (2, 0, 0.0),
            (2, 1, 0.0),
            (2, 2, 0.0),
        ],
    )
    .expect("static matrix");

    let mut stamper = GeneratedStamper::new(&mut matrix, &mut rhs, &voltages, 2);
    stamper.stamp_potential_branch(Some(1), Some(2), 1, 3.0);
    stamper.stamp_potential(
        1,
        0.7,
        &[
            GeneratedDerivative::node(1, 0.2),
            GeneratedDerivative::node(2, -0.1),
        ],
    );

    let p_branch = matrix.get_index(0, 2).expect("p branch entry").0;
    let n_branch = matrix.get_index(1, 2).expect("n branch entry").0;
    let branch_p = matrix.get_index(2, 0).expect("branch p entry").0;
    let branch_n = matrix.get_index(2, 1).expect("branch n entry").0;
    let values = matrix.values_mut();

    assert_eq!(values[p_branch], 3.0);
    assert_eq!(values[n_branch], -3.0);
    assert_eq!(values[branch_p], 1.0 - 0.2);
    assert_eq!(values[branch_n], -1.0 - -0.1);

    let equivalent = 0.7 - (0.2 * 1.0 + -0.1 * 0.5);
    assert_eq!(rhs[2], equivalent);
}

#[test]
fn generated_indexed_dense_potential_stamper_skips_static_zero_lanes() {
    let nodes = [1, 2];
    let branches = [1];
    let voltages = [1.0, 2.0, 3.0];
    let mut rhs = vec![0.0; 3];
    let mut matrix =
        StaticMatrix::from_triplets(3, 3, &[(2, 0, 0.0), (2, 2, 0.0)]).expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, 2);

    GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 2, &cache)
        .stamp_potential_indexed_dense_local(0, 7.0, &[0], &[1.5], &[0], &[2.0]);

    let n0 = matrix.get_index(2, 0).expect("n0 entry").0;
    let b0 = matrix.get_index(2, 2).expect("b0 entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[n0], -1.5);
    assert_eq!(values[b0], -2.0);

    let equivalent = 7.0 - 1.5 * voltages[0] - 2.0 * voltages[2];
    assert_eq!(rhs[2], equivalent);
    assert_eq!(rhs[0], 0.0);
    assert_eq!(rhs[1], 0.0);
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
fn generated_cached_generic_potential_stamper_uses_static_axes() {
    let nodes = [1, 2];
    let branches = [1];
    let voltages = [1.0, 2.0, 3.0];
    let mut rhs = vec![0.0; 3];
    let mut matrix = StaticMatrix::from_triplets(3, 3, &[(2, 0, 0.0), (2, 1, 0.0), (2, 2, 0.0)])
        .expect("static matrix");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&matrix, &nodes, &branches, 2);

    GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 2, &cache)
        .stamp_potential_local(
            0,
            7.0,
            &[
                GeneratedDerivative::node(0, 1.5),
                GeneratedDerivative::node(1, -0.25),
                GeneratedDerivative::branch(0, 2.0),
            ],
        );

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

#[test]
fn generated_stamper_linearizes_current_contribution_against_branch_axis() {
    let voltages = [1.0, 0.5, 2.0];
    let mut rhs = vec![0.0; 3];
    let mut matrix = StaticMatrix::from_triplets(
        3,
        3,
        &[
            (0, 0, 0.0),
            (0, 2, 0.0),
            (1, 1, 0.0),
            (1, 2, 0.0),
            (2, 2, 0.0),
        ],
    )
    .expect("static matrix");

    GeneratedStamper::new(&mut matrix, &mut rhs, &voltages, 2).stamp_current(
        Some(1),
        Some(2),
        1.4,
        &[GeneratedDerivative::branch(1, 0.7)],
    );

    let p_branch = matrix.get_index(0, 2).expect("p branch entry").0;
    let n_branch = matrix.get_index(1, 2).expect("n branch entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[p_branch], 0.7);
    assert_eq!(values[n_branch], -0.7);

    let equivalent = 1.4 - 0.7 * 2.0;
    assert_eq!(rhs[0], -equivalent);
    assert_eq!(rhs[1], equivalent);
}

#[test]
fn generated_stamper_linearizes_potential_contribution_against_branch_axis() {
    let voltages = [1.0, 0.5, 2.0];
    let mut rhs = vec![0.0; 3];
    let mut matrix = StaticMatrix::from_triplets(
        3,
        3,
        &[
            (0, 0, 0.0),
            (0, 2, 0.0),
            (1, 1, 0.0),
            (1, 2, 0.0),
            (2, 0, 0.0),
            (2, 1, 0.0),
            (2, 2, 0.0),
        ],
    )
    .expect("static matrix");

    let mut stamper = GeneratedStamper::new(&mut matrix, &mut rhs, &voltages, 2);
    stamper.stamp_potential_branch(Some(1), Some(2), 1, 1.0);
    stamper.stamp_potential(1, 6.0, &[GeneratedDerivative::branch(1, 3.0)]);

    let branch_branch = matrix.get_index(2, 2).expect("branch branch entry").0;
    let values = matrix.values_mut();
    assert_eq!(values[branch_branch], -3.0);

    let equivalent = 6.0 - 3.0 * 2.0;
    assert_eq!(rhs[2], equivalent);
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

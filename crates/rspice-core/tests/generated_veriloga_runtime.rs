#![cfg(feature = "veriloga-builtins")]

use rspice_core::device::veriloga_generated::{
    GeneratedDerivative, GeneratedEvalContext, GeneratedStamper, builtins, instantiate_builtin,
};
use rspice_core::solver::StaticMatrix;
use rspice_core::{CircuitData, netlist::ParamContext};

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

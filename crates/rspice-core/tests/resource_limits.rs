use std::path::Path;
use std::sync::Arc;

use rspice_core::analysis::HbConfig;
use rspice_core::netlist::{IncludeProcessor, NetlistParseOptions, ParseError};
use rspice_core::{
    Engine, Netlist, ResourceKind, ResourceLimitError, ResourceLimits, SimulationConfig,
    SimulationConfigError, SimulationError,
};

fn limits_with(update: impl FnOnce(&mut ResourceLimits)) -> ResourceLimits {
    let mut limits = ResourceLimits::default();
    update(&mut limits);
    limits
}

#[test]
fn parser_rejects_root_bytes_before_materializing_lines() {
    let source = "limited source\nR1 1 0 1k\n.end\n";
    let options = NetlistParseOptions {
        resource_limits: limits_with(|limits| limits.max_netlist_bytes = source.len() - 1),
        ..NetlistParseOptions::default()
    };

    assert!(matches!(
        Netlist::parse_with_options(source, options),
        Err(ParseError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::NetlistBytes,
            requested,
            limit,
        })) if requested == source.len() && limit == source.len() - 1
    ));
}

#[test]
fn parser_rejects_excess_root_lines() {
    let options = NetlistParseOptions {
        resource_limits: limits_with(|limits| limits.max_netlist_lines = 2),
        ..NetlistParseOptions::default()
    };

    assert!(matches!(
        Netlist::parse_with_options("limited lines\nR1 1 0 1k\n.end\n", options),
        Err(ParseError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::NetlistLines,
            requested: 3,
            limit: 2,
        }))
    ));
}

#[test]
fn include_expansion_rejects_oversized_materialized_source() {
    let source = "limited expansion\nR1 1 0 1k\n.end\n";
    let limits = limits_with(|limits| limits.max_expanded_source_bytes = source.len() - 1);
    let mut processor =
        IncludeProcessor::new(Path::new("limited.cir")).with_resource_limits(limits);

    assert!(matches!(
        processor.expand_content(source, Path::new("limited.cir")),
        Err(ParseError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::ExpandedSourceBytes,
            requested,
            limit,
        })) if requested == source.len() && limit == source.len() - 1
    ));
}

#[test]
fn hierarchy_expansion_enforces_element_budget() {
    let netlist =
        Netlist::parse("limited hierarchy\nR1 1 0 1k\nR2 2 0 2k\n.end").expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_flattened_elements = 1),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::new(config).build_circuit(&netlist),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::FlattenedElements,
            requested: 2,
            limit: 1,
        }))
    ));
}

#[test]
fn hierarchy_expansion_enforces_depth_budget() {
    let netlist = Netlist::parse(
        "limited depth\n\
         X1 1 0 CELL\n\
         .subckt CELL a b\n\
         R1 a b 1k\n\
         .ends\n\
         .end",
    )
    .expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_hierarchy_depth = 0),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::new(config).build_circuit(&netlist),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::HierarchyDepth,
            requested: 1,
            limit: 0,
        }))
    ));
}

#[test]
fn circuit_construction_enforces_node_and_unknown_budgets() {
    let node_netlist =
        Netlist::parse("limited nodes\nR1 one two 1k\n.end").expect("node fixture parses");
    let node_config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_circuit_nodes = 1),
        ..SimulationConfig::default()
    };
    assert!(matches!(
        Engine::new(node_config).build_circuit(&node_netlist),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::CircuitNodes,
            requested: 2,
            limit: 1,
        }))
    ));

    let matrix_netlist =
        Netlist::parse("limited unknowns\nV1 one 0 1\n.end").expect("matrix fixture parses");
    let matrix_config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_matrix_unknowns = 1),
        ..SimulationConfig::default()
    };
    assert!(matches!(
        Engine::new(matrix_config).build_circuit(&matrix_netlist),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::MatrixUnknowns,
            requested: 2,
            limit: 1,
        }))
    ));
}

#[test]
fn locked_time_grid_is_rejected_during_configuration_validation() {
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_analysis_points = 2),
        locked_time_grid: Some(Arc::new(vec![0.0, 1.0, 2.0])),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        config.validate(),
        Err(SimulationConfigError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::AnalysisPoints,
            requested: 3,
            limit: 2,
        }))
    ));
}

#[test]
fn frequency_and_dc_sweeps_enforce_point_budgets_before_solving() {
    let netlist =
        Netlist::parse("limited analyses\nV1 1 0 1 AC 1\nR1 1 0 1k\n.end").expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_analysis_points = 2),
        ..SimulationConfig::default()
    };
    let engine = Engine::new(config);

    assert!(matches!(
        engine.run_ac(&netlist, &[1.0, 10.0, 100.0]),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::AnalysisPoints,
            requested: 3,
            limit: 2,
        }))
    ));
    assert!(matches!(
        engine.run_dc_sweep(&netlist, "V1", 0.0, 2.0, 1.0),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::AnalysisPoints,
            requested: 3,
            limit: 2,
        }))
    ));
}

#[test]
fn transient_preflight_rejects_a_minimum_point_count_over_budget() {
    let netlist =
        Netlist::parse("limited transient\nV1 1 0 1\nR1 1 0 1k\n.end").expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_analysis_points = 4),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::new(config).run_tran(&netlist, 1.0, 0.25),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::AnalysisPoints,
            requested: 5,
            limit: 4,
        }))
    ));
}

#[test]
fn independent_batch_analyses_enforce_run_budgets() {
    let netlist =
        Netlist::parse("limited batch\n.param P=1\nR1 1 0 {P}\n.end").expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_batch_runs = 2),
        ..SimulationConfig::default()
    };
    let engine = Engine::new(config);

    assert!(matches!(
        engine.run_monte_carlo(&netlist, 3, 7),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::BatchRuns,
            requested: 3,
            limit: 2,
        }))
    ));
    assert!(matches!(
        engine.run_step(&netlist, "P", &[1.0, 2.0, 3.0]),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::BatchRuns,
            requested: 3,
            limit: 2,
        }))
    ));
}

#[test]
fn result_shapes_are_bounded_independently_of_point_counts() {
    let netlist =
        Netlist::parse("limited result\n.param RVAL=1k\nV1 1 0 1 AC 1\nR1 1 0 {RVAL}\n.end")
            .expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_result_values = 2),
        ..SimulationConfig::default()
    };
    let engine = Engine::new(config);

    assert!(matches!(
        engine.run_ac(&netlist, &[1.0]),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::ResultValues,
            requested,
            limit: 2,
        })) if requested > 2
    ));
    assert!(matches!(
        engine.run_dc_op(&netlist),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::ResultValues,
            requested,
            limit: 2,
        })) if requested > 2
    ));
    assert!(matches!(
        engine.run_monte_carlo(&netlist, 1, 7),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::ResultValues,
            requested,
            limit: 2,
        })) if requested > 2
    ));
}

#[test]
fn step_results_enforce_the_aggregate_result_budget() {
    let netlist = Netlist::parse(
        "limited aggregate\n\
         .param RVAL=1k\n\
         V1 in 0 1\n\
         R1 in out {RVAL}\n\
         R2 out 0 1k\n\
         .end",
    )
    .expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_result_values = 15),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::new(config).run_step(&netlist, "RVAL", &[1_000.0, 2_000.0]),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::ResultValues,
            requested,
            limit: 15,
        })) if requested > 15
    ));
}

#[test]
fn harmonic_balance_rejects_oversized_collocation_before_circuit_build() {
    let netlist = Netlist::parse("limited harmonic balance\n.end").expect("fixture parses");
    let config = SimulationConfig {
        resource_limits: limits_with(|limits| limits.max_analysis_points = 3),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::new(config).run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(3)),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::AnalysisPoints,
            requested,
            limit: 3,
        })) if requested > 3
    ));
}

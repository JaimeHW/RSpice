use super::*;
use crate::state::{LibraryCellInstance, Point};

// -------------------------------------------------------------------------
// Net Tests
// -------------------------------------------------------------------------

#[test]
fn test_net_new() {
    let net = Net::new(1);
    assert_eq!(net.id, 1);
    assert!(net.points.is_empty());
    assert!(net.label.is_none());
}

#[test]
fn test_net_add_point() {
    let mut net = Net::new(1);
    net.add_point(Point::new(0, 0));
    net.add_point(Point::new(10, 0));
    assert_eq!(net.points.len(), 2);
    assert!(net.contains(Point::new(0, 0)));
    assert!(net.contains(Point::new(10, 0)));
    assert!(!net.contains(Point::new(5, 5)));
}

#[test]
fn test_net_merge() {
    let mut net1 = Net::new(1);
    net1.add_point(Point::new(0, 0));
    net1.add_point(Point::new(10, 0));

    let mut net2 = Net::new(2);
    net2.add_point(Point::new(10, 0));
    net2.add_point(Point::new(10, 10));
    net2.label = Some("VDD".to_string());

    net1.merge(&net2);

    assert_eq!(net1.points.len(), 3); // 0,0 + 10,0 + 10,10 (10,0 deduped)
    assert_eq!(net1.label, Some("VDD".to_string()));
}

#[test]
fn test_net_spice_name_default() {
    let net = Net::new(5);
    assert_eq!(net.spice_name(), "net5");
}

#[test]
fn test_net_spice_name_with_label() {
    let mut net = Net::new(1);
    net.label = Some("VCC".to_string());
    assert_eq!(net.spice_name(), "VCC");
}

#[test]
fn test_net_spice_name_ground() {
    let mut net = Net::new(1);
    net.label = Some("GND".to_string());
    assert_eq!(net.spice_name(), "0");

    net.label = Some("ground".to_string());
    assert_eq!(net.spice_name(), "0");

    net.label = Some("0".to_string());
    assert_eq!(net.spice_name(), "0");
}

// -------------------------------------------------------------------------
// Generator Construction Tests
// -------------------------------------------------------------------------

#[test]
fn test_generator_new() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert!(gen.nets.is_empty());
    assert!(gen.point_to_net.is_empty());
    assert!(gen.ground_net.is_none());
}

#[test]
fn test_generator_empty_schematic() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);
    let netlist = gen.generate();

    assert!(netlist.contains("* RSpice Netlist"));
    assert!(netlist.contains("* Components: 0"));
    assert!(netlist.contains(".end"));
}

// -------------------------------------------------------------------------
// Value Formatting Tests
// -------------------------------------------------------------------------

#[test]
fn test_format_value_empty() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_value(""), "1");
}

#[test]
fn test_format_value_with_si_prefix() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_value("1k"), "1k");
    assert_eq!(gen.format_value("10u"), "10u");
    assert_eq!(gen.format_value("100n"), "100n");
    assert_eq!(gen.format_value("1.5meg"), "1.5meg");
}

#[test]
fn test_format_value_numeric() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_value("1000"), "1000");
    assert_eq!(gen.format_value("1e-9"), "1e-9");
}

// -------------------------------------------------------------------------
// Node Formatting Tests
// -------------------------------------------------------------------------

#[test]
fn test_format_nodes_exact() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    let nodes = vec!["1".to_string(), "2".to_string()];
    assert_eq!(gen.format_nodes(&nodes, 2), "1 2");
}

#[test]
fn test_format_nodes_more_than_expected() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    let nodes = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    assert_eq!(gen.format_nodes(&nodes, 2), "1 2");
}

#[test]
fn test_format_nodes_less_than_expected() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    let nodes = vec!["1".to_string()];
    assert_eq!(gen.format_nodes(&nodes, 2), "1 0");
}

#[test]
fn test_format_nodes_empty() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    let nodes: Vec<String> = vec![];
    assert_eq!(gen.format_nodes(&nodes, 2), "0 0");
}

// -------------------------------------------------------------------------
// Source Value Formatting Tests
// -------------------------------------------------------------------------

#[test]
fn test_format_source_value_dc() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);

    let mut comp = Component::new(1, ComponentType::VoltageSource, Point::new(0, 0));
    comp.value = "5".to_string();
    assert!(gen.format_source_value(&comp).contains("DC 5"));
}

#[test]
fn test_format_source_value_ac() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);

    let mut comp = Component::new(1, ComponentType::VoltageSourceAc, Point::new(0, 0));
    comp.value = "1".to_string();
    assert!(gen.format_source_value(&comp).contains("AC 1"));
}

#[test]
fn test_format_source_value_pulse() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);

    let comp = Component::new(1, ComponentType::VoltageSourcePulse, Point::new(0, 0));
    assert!(gen.format_source_value(&comp).contains("PULSE("));
}

#[test]
fn test_format_source_value_sin() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);

    let comp = Component::new(1, ComponentType::VoltageSourceSin, Point::new(0, 0));
    assert!(gen.format_source_value(&comp).contains("SIN("));
}

// -------------------------------------------------------------------------
// Model Generation Tests
// -------------------------------------------------------------------------

#[test]
fn test_get_bjt_model_npn() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let comp = Component::new(1, ComponentType::NpnBjt, Point::new(0, 0)).with_name_value("Q1", "");
    let model = gen.get_bjt_model(&comp, None);

    assert!(model.contains("npn"));
    assert!(gen.models.values().any(|m| m.contains("NPN")));
}

#[test]
fn test_get_bjt_model_pnp() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let comp = Component::new(1, ComponentType::PnpBjt, Point::new(0, 0)).with_name_value("Q2", "");
    let model = gen.get_bjt_model(&comp, None);

    assert!(model.contains("pnp"));
    assert!(gen.models.values().any(|m| m.contains("PNP")));
}

#[test]
fn test_get_mosfet_model_nmos() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let comp = Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
    let model = gen.get_mosfet_model(&comp);

    assert!(model.contains("nmos"));
    assert!(gen.models.values().any(|m| m.contains("NMOS")));
    assert!(gen.models.values().any(|m| m.contains("VTO=0.7")));
}

#[test]
fn test_get_mosfet_model_pmos() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let comp = Component::new(1, ComponentType::Pmos, Point::new(0, 0)).with_name_value("M2", "");
    let model = gen.get_mosfet_model(&comp);

    assert!(model.contains("pmos"));
    assert!(gen.models.values().any(|m| m.contains("PMOS")));
    assert!(gen.models.values().any(|m| m.contains("VTO=-0.7")));
}

#[test]
fn test_get_jfet_model() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let comp = Component::new(1, ComponentType::Njfet, Point::new(0, 0)).with_name_value("J1", "");
    let model = gen.get_jfet_model(&comp);

    assert!(model.contains("njf"));
    assert!(gen.models.values().any(|m| m.contains("NJF")));
}

// -------------------------------------------------------------------------
// Accessor Tests
// -------------------------------------------------------------------------

#[test]
fn test_generator_nets_accessor() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert!(gen.nets().is_empty());
}

#[test]
fn test_generator_has_ground_initially_false() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert!(!gen.has_ground());
}

#[test]
fn test_generator_ground_net_id_initially_none() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert!(gen.ground_net_id().is_none());
}

// -------------------------------------------------------------------------
// Integration Tests
// -------------------------------------------------------------------------

#[test]
fn test_generate_with_analysis_commands() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let analysis = vec![".tran 1n 100n".to_string(), ".ac dec 10 1 1meg".to_string()];
    let netlist = gen.generate_with_analysis(&analysis);

    assert!(netlist.contains(".tran 1n 100n"));
    assert!(netlist.contains(".ac dec 10 1 1meg"));
    assert!(netlist.contains(".end"));
}

#[test]
fn test_generate_with_empty_analysis() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let netlist = gen.generate_with_analysis(&[]);

    assert!(netlist.contains("* RSpice Netlist"));
    assert!(netlist.contains(".end"));
    // Should not have analysis section when empty
    assert!(!netlist.contains("* Analysis commands"));
}

#[test]
fn test_generate_without_analysis_has_no_placeholder_op() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let netlist = gen.generate();

    assert!(netlist.contains("* RSpice Netlist"));
    assert!(netlist.contains(".end"));
    assert!(!netlist.contains("* Analysis commands"));
    assert!(!netlist.contains("\n.op\n"));
}

#[test]
fn test_generate_resets_internal_state_between_calls() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let first = gen.generate_with_analysis(&[".ac dec 10 1 1meg".to_string()]);
    assert!(first.contains(".ac dec 10 1 1meg"));

    let second = gen.generate_with_analysis(&[]);
    assert!(!second.contains(".ac dec 10 1 1meg"));
    assert!(!second.contains("* Analysis commands"));
}

#[test]
fn test_generate_netlist_with_analysis_convenience_api() {
    let schematic = SchematicState::default();
    let analysis = vec![".tran 1n 100n".to_string()];

    let result = generate_netlist_with_analysis(&schematic, &analysis);

    assert!(result.netlist.contains(".tran 1n 100n"));
    assert!(result.netlist.contains(".end"));
    assert!(result.errors.is_empty());
}

#[test]
fn test_generate_veriloga_include_and_cell_instance() {
    let mut schematic = SchematicState::default();
    let mut binding = LibraryCellInstance::new("veriloga", "my_resistor", "veriloga");
    binding.source_path = Some(std::path::PathBuf::from("models/my resistor.va"));
    binding.module_name = Some("my_resistor".to_string());
    binding.terminal_order = vec!["p".to_string(), "n".to_string()];

    let mut comp =
        Component::new(1, ComponentType::CellInstance, Point::new(0, 0)).with_name_value("X1", "");
    comp.library_cell = Some(binding);
    comp.params = "g=2m".to_string();
    schematic.components.push(comp);

    let mut gen = NetlistGenerator::new(&schematic);
    let netlist = gen.generate();

    assert!(netlist.contains(".VERILOGA \"models/my resistor.va\" my_resistor"));
    assert!(netlist.contains("X1"));
    assert!(netlist.contains("my_resistor g=2m"));
}

#[test]
fn test_generate_generic_include_for_spice_bound_cell_instance() {
    let mut schematic = SchematicState::default();
    let mut binding = LibraryCellInstance::new("user_lib", "lp_filter", "spice");
    binding.source_path = Some(std::path::PathBuf::from("models/lp_filter.sp"));
    binding.module_name = Some("lp_filter_subckt".to_string());
    binding.terminal_order = vec!["in".to_string(), "out".to_string(), "vss".to_string()];

    let mut comp =
        Component::new(1, ComponentType::CellInstance, Point::new(0, 0)).with_name_value("lp1", "");
    comp.library_cell = Some(binding);
    schematic.components.push(comp);

    let mut gen = NetlistGenerator::new(&schematic);
    let netlist = gen.generate();

    assert!(netlist.contains(".include \"models/lp_filter.sp\""));
    assert!(netlist.contains("Xlp1 "));
    assert!(netlist.contains(" lp_filter_subckt"));
}

#[test]
fn test_generate_deduplicates_generic_library_includes() {
    let mut schematic = SchematicState::default();

    for (id, name) in [(1_u64, "x1"), (2_u64, "x2")] {
        let mut binding = LibraryCellInstance::new("user_lib", "amp", "spice");
        binding.source_path = Some(std::path::PathBuf::from("models/amp.sp"));
        binding.module_name = Some("amp".to_string());
        binding.terminal_order = vec!["in".to_string(), "out".to_string()];

        let mut comp = Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
            .with_name_value(name, "");
        comp.library_cell = Some(binding);
        schematic.components.push(comp);
    }

    let mut gen = NetlistGenerator::new(&schematic);
    let netlist = gen.generate();
    let include_count = netlist.matches(".include \"models/amp.sp\"").count();
    assert_eq!(include_count, 1, "generic include should be emitted once");
}

#[test]
fn test_cell_instance_line_uses_module_or_cell_fallback() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut binding = LibraryCellInstance::new("veriloga", "fallback_cell", "veriloga");
    binding.source_path = Some(std::path::PathBuf::from("fallback.va"));
    binding.terminal_order = vec!["p".to_string(), "n".to_string()];

    let mut comp = Component::new(1, ComponentType::CellInstance, Point::new(10, 20))
        .with_name_value("inst1", "");
    comp.library_cell = Some(binding);

    let line = gen
        .generate_instance_line(&comp)
        .expect("cell instance should netlist");

    assert!(line.starts_with("Xinst1 "));
    assert!(line.contains(" fallback_cell"));
}

#[test]
fn test_generate_netlist_reports_error_for_missing_terminal_order() {
    let mut schematic = SchematicState::default();
    let mut binding = LibraryCellInstance::new("user_lib", "amp", "spice");
    binding.source_path = Some(std::path::PathBuf::from("models/amp.sp"));
    binding.module_name = Some("amp".to_string());
    // Intentionally leave terminal_order empty for strict validation.

    let mut comp = Component::new(1, ComponentType::CellInstance, Point::new(0, 0))
        .with_name_value("x_missing_ports", "");
    comp.library_cell = Some(binding);
    schematic.components.push(comp);

    let result = generate_netlist(&schematic);
    assert!(!result.errors.is_empty());
    assert!(result
        .errors
        .iter()
        .any(|err| err.contains("terminal order metadata")));
}

#[test]
fn test_generate_netlist_warns_on_conflicting_veriloga_module_binding() {
    let mut schematic = SchematicState::default();

    for (id, module) in [(1_u64, "mod_a"), (2_u64, "mod_b")] {
        let mut binding = LibraryCellInstance::new("veriloga", "same_cell", "veriloga");
        binding.source_path = Some(std::path::PathBuf::from("models/shared.va"));
        binding.module_name = Some(module.to_string());
        binding.terminal_order = vec!["p".to_string(), "n".to_string()];

        let mut comp = Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
            .with_name_value(format!("x{}", id), "");
        comp.library_cell = Some(binding);
        schematic.components.push(comp);
    }

    let result = generate_netlist(&schematic);
    assert!(!result.warnings.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|warn| warn.contains("Conflicting Verilog-A module bindings")));
}

// -------------------------------------------------------------------------
// Timestamp Tests
// -------------------------------------------------------------------------

#[test]
fn test_chrono_lite_timestamp() {
    let ts = chrono_lite_timestamp();
    // Should be numeric (unix timestamp)
    assert!(!ts.is_empty());
    // Should be parseable as a number
    assert!(ts.parse::<u64>().is_ok() || ts == "unknown");
}

// -------------------------------------------------------------------------
// Node Name Tests
// -------------------------------------------------------------------------

#[test]
fn test_get_node_name_floating() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);

    // Point not in any net should return float_XXX
    let name = gen.get_node_name(Point::new(5, 3));
    assert!(name.starts_with("float_"));
}

// =========================================================================
// Parameter Serialization Tests (Spectre Parity)
// =========================================================================
//
// These tests verify that component.params are correctly appended to
// netlist lines, following Cadence Spectre conventions.

// -------------------------------------------------------------------------
// Helper Function Tests
// -------------------------------------------------------------------------

#[test]
fn test_format_params_empty() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_params(""), "");
    assert_eq!(gen.format_params("   "), "");
}

#[test]
fn test_format_params_single() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_params("m=2"), " m=2");
}

#[test]
fn test_format_params_multiple() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_params("m=2 tc1=0.01"), " m=2 tc1=0.01");
}

#[test]
fn test_format_params_with_whitespace() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    // Leading/trailing whitespace should be trimmed
    assert_eq!(gen.format_params("  m=2  "), " m=2");
}

#[test]
fn test_format_value_with_params_value_only() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_value_with_params("1k", ""), "1k");
}

#[test]
fn test_format_value_with_params_both() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(gen.format_value_with_params("1k", "m=2"), "1k m=2");
}

#[test]
fn test_format_value_with_params_complex() {
    let schematic = SchematicState::default();
    let gen = NetlistGenerator::new(&schematic);
    assert_eq!(
        gen.format_value_with_params("4.7k", "m=2 tc1=0.01 tc2=0.001"),
        "4.7k m=2 tc1=0.01 tc2=0.001"
    );
}

// -------------------------------------------------------------------------
// Passive Component Parameter Tests
// -------------------------------------------------------------------------

#[test]
fn test_resistor_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k");
    comp.params = "m=2 tc1=0.01".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("R1"));
    assert!(line.contains("1k"));
    assert!(line.contains("m=2"));
    assert!(line.contains("tc1=0.01"));
}

#[test]
fn test_resistor_without_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let comp =
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k");

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("R1"));
    assert!(line.contains("1k"));
    // Should not have spurious whitespace at end
    assert!(!line.ends_with(' '));
}

#[test]
fn test_capacitor_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Capacitor, Point::new(0, 0)).with_name_value("C1", "100p");
    comp.params = "ic=0 m=4".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("C1"));
    assert!(line.contains("100p"));
    assert!(line.contains("ic=0"));
    assert!(line.contains("m=4"));
}

#[test]
fn test_inductor_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Inductor, Point::new(0, 0)).with_name_value("L1", "10u");
    comp.params = "ic=0 m=1".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("L1"));
    assert!(line.contains("10u"));
    assert!(line.contains("ic=0"));
}

#[test]
fn test_diode_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Diode, Point::new(0, 0)).with_name_value("D1", "1n");
    comp.params = "area=2 m=1".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("D1"));
    assert!(line.contains("area=2"));
}

// -------------------------------------------------------------------------
// Source Parameter Tests
// -------------------------------------------------------------------------

#[test]
fn test_voltage_source_dc_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp = Component::new(1, ComponentType::VoltageSource, Point::new(0, 0))
        .with_name_value("V1", "5");
    comp.params = "acmag=1 acphase=0".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("V1"));
    assert!(line.contains("DC 5"));
    assert!(line.contains("acmag=1"));
    assert!(line.contains("acphase=0"));
}

#[test]
fn test_voltage_source_ac_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp = Component::new(1, ComponentType::VoltageSourceAc, Point::new(0, 0))
        .with_name_value("V2", "1");
    comp.params = "phase=45".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("V2"));
    assert!(line.contains("AC 1"));
    assert!(line.contains("phase=45"));
}

#[test]
fn test_voltage_source_pulse_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    // Pulse sources use structured parameters for V1, V2, TD, TR, TF, PW, PER
    // The component.params contains key=value pairs for these parameters
    let mut comp = Component::new(1, ComponentType::VoltageSourcePulse, Point::new(0, 0))
        .with_name_value("V3", "");
    comp.params = "v1=0 v2=5 td=0 tr=1n tf=1n pw=10n period=20n".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("V3"));
    assert!(line.contains("PULSE("));
    // Check that structured parameters are parsed correctly
    assert!(line.contains("PULSE(0 5 0 1n 1n 10n 20n)"));
}

#[test]
fn test_current_source_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp = Component::new(1, ComponentType::CurrentSource, Point::new(0, 0))
        .with_name_value("I1", "1m");
    comp.params = "acmag=100u".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("I1"));
    assert!(line.contains("DC 1m"));
    assert!(line.contains("acmag=100u"));
}

// -------------------------------------------------------------------------
// Transistor Parameter Tests
// -------------------------------------------------------------------------

#[test]
fn test_nmos_with_dimension_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
    comp.params = "w=1u l=180n as=1p ad=1p ps=2u pd=2u".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.starts_with("M"));
    assert!(line.contains("M1"));
    assert!(line.contains("nmos"));
    assert!(line.contains("w=1u"));
    assert!(line.contains("l=180n"));
    assert!(line.contains("as=1p"));
    assert!(line.contains("ad=1p"));
}

#[test]
fn test_pmos_with_dimension_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Pmos, Point::new(0, 0)).with_name_value("M2", "");
    comp.params = "w=2u l=180n m=4".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("M2"));
    assert!(line.contains("pmos"));
    assert!(line.contains("w=2u"));
    assert!(line.contains("l=180n"));
    assert!(line.contains("m=4"));
}

#[test]
fn test_npn_bjt_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::NpnBjt, Point::new(0, 0)).with_name_value("Q1", "");
    comp.params = "area=2 m=1".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.starts_with("Q"));
    assert!(line.contains("Q1"));
    assert!(line.contains("npn"));
    assert!(line.contains("area=2"));
}

#[test]
fn test_bjt_uses_value_as_explicit_model_name() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::NpnBjt, Point::new(0, 0)).with_name_value("Q1", "2N2222");
    comp.params = "area=2".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains(" 2N2222 "));
    assert!(line.contains("area=2"));
    assert!(!line.contains("model="));
    assert!(!gen.models.contains_key("2N2222"));
}

#[test]
fn test_bjt_uses_model_param_and_removes_duplicate_model_key() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::NpnBjt, Point::new(0, 0)).with_name_value("Q1", "");
    comp.params = "model=2N2222 area=2 m=1".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains(" 2N2222 "));
    assert!(line.contains("area=2"));
    assert!(line.contains("m=1"));
    assert!(!line.contains("model=2N2222"));
    assert!(!gen.models.contains_key("2N2222"));
}

#[test]
fn test_pnp_bjt_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::PnpBjt, Point::new(0, 0)).with_name_value("Q2", "");
    comp.params = "area=1.5".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("Q2"));
    assert!(line.contains("pnp"));
    assert!(line.contains("area=1.5"));
}

#[test]
fn test_njfet_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Njfet, Point::new(0, 0)).with_name_value("J1", "");
    comp.params = "area=1 m=2".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.starts_with("J"));
    assert!(line.contains("J1"));
    assert!(line.contains("njf"));
    assert!(line.contains("area=1"));
    assert!(line.contains("m=2"));
}

// -------------------------------------------------------------------------
// Controlled Source Parameter Tests
// -------------------------------------------------------------------------

#[test]
fn test_vcvs_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Vcvs, Point::new(0, 0)).with_name_value("E1", "10");
    comp.params = "max=5 min=-5".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.starts_with("E"));
    assert!(line.contains("E1"));
    assert!(line.contains("10"));
    assert!(line.contains("max=5"));
    assert!(line.contains("min=-5"));
}

#[test]
fn test_vccs_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Vccs, Point::new(0, 0)).with_name_value("G1", "1m");
    comp.params = "ic=0".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.starts_with("G"));
    assert!(line.contains("G1"));
    assert!(line.contains("1m"));
    assert!(line.contains("ic=0"));
}

#[test]
fn test_ccvs_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Ccvs, Point::new(0, 0)).with_name_value("H1", "1k");
    comp.params = "max=10".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.starts_with("H"));
    assert!(line.contains("H1"));
    assert!(line.contains("1k"));
    assert!(line.contains("max=10"));
}

#[test]
fn test_cccs_with_params() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Cccs, Point::new(0, 0)).with_name_value("F1", "100");
    comp.params = "m=2".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.starts_with("F"));
    assert!(line.contains("F1"));
    assert!(line.contains("100"));
    assert!(line.contains("m=2"));
}

// -------------------------------------------------------------------------
// Edge Case Parameter Tests
// -------------------------------------------------------------------------

#[test]
fn test_params_with_negative_values() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k");
    comp.params = "tc1=-0.01 tc2=-0.001".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("tc1=-0.01"));
    assert!(line.contains("tc2=-0.001"));
}

#[test]
fn test_params_with_scientific_notation() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
    comp.params = "w=1e-6 l=1.8e-7".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("w=1e-6"));
    assert!(line.contains("l=1.8e-7"));
}

#[test]
fn test_params_with_expressions() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k");
    // Spectre supports expressions in parameters
    comp.params = "m='2*scale'".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("m='2*scale'"));
}

#[test]
fn test_params_preserves_order() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
    comp.params = "w=1u l=180n as=1p ad=1p".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    // Parameters should maintain their order
    let w_pos = line.find("w=1u").unwrap();
    let l_pos = line.find("l=180n").unwrap();
    let as_pos = line.find("as=1p").unwrap();
    let ad_pos = line.find("ad=1p").unwrap();

    assert!(w_pos < l_pos);
    assert!(l_pos < as_pos);
    assert!(as_pos < ad_pos);
}

#[test]
fn test_empty_params_no_trailing_space() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let comp =
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k");

    let line = gen.generate_instance_line(&comp).unwrap();

    // Line should not end with trailing space when params are empty
    assert!(!line.ends_with(' '));
    assert!(line.ends_with("1k") || line.ends_with("0 0 1k") || line.contains("1k"));
}

#[test]
fn test_params_with_quoted_values() {
    let schematic = SchematicState::default();
    let mut gen = NetlistGenerator::new(&schematic);

    let mut comp =
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k");
    comp.params = "model=\"res_hi\"".to_string();

    let line = gen.generate_instance_line(&comp).unwrap();

    assert!(line.contains("model=\"res_hi\""));
}

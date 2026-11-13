use rspice_core::netlist::{Flattener, FlattenerConfig, HierarchyPath, Netlist};

#[test]
fn metadata_and_parameter_lookup_describe_concrete_instances() {
    let netlist = Netlist::parse("* metadata\n.PARAM r=10\n.SUBCKT cell a PARAMS: r=20\nR1 a 0 {r}\n.ENDS\nX1 out cell r=30\nX2 aux cell r=40\n.END\n").unwrap();
    let mut flattener = Flattener::with_config(&netlist.subcircuits, FlattenerConfig::debug());
    flattener.flatten(&netlist).unwrap();
    assert_eq!(flattener.instance_metadata().len(), 2);
    let first = &flattener.instance_metadata()[0];
    assert_eq!(first.path.to_string(), "X1");
    assert_eq!(first.subcircuit_name, "cell");
    assert!(
        first
            .instance_params
            .iter()
            .any(|(name, value)| name.eq_ignore_ascii_case("r") && *value == 30.0)
    );
    assert_eq!(
        flattener
            .param_resolver()
            .resolve("r", &HierarchyPath::parse("X1", '.')),
        Some(30.0)
    );
}

#[test]
fn reusing_a_flattener_does_not_retain_old_globals() {
    let first = Netlist::parse("* first\n.PARAM old=10\nR1 out 0 1k\n.END\n").unwrap();
    let second = Netlist::parse("* second\nR1 out 0 1k\n.END\n").unwrap();
    let mut flattener = Flattener::new(&[]);
    flattener.flatten(&first).unwrap();
    assert_eq!(
        flattener
            .param_resolver()
            .resolve("old", &HierarchyPath::root()),
        Some(10.0)
    );
    flattener.flatten(&second).unwrap();
    assert_eq!(
        flattener
            .param_resolver()
            .resolve("old", &HierarchyPath::root()),
        None
    );
}

#[test]
fn nested_scopes_include_defaults_and_locals_with_custom_paths() {
    let netlist = Netlist::parse("* nested metadata\n.PARAM r=10 inherited=7\n.SUBCKT leaf a PARAMS: r=20\n.PARAM local={r+1}\nR1 a 0 {local}\n.ENDS\n.SUBCKT parent a PARAMS: r=30\nXinner a leaf\n.ENDS\nXtop out parent r=40\nXpeer aux leaf r=50\n.END\n").unwrap();
    let mut flattener = Flattener::with_config(
        &netlist.subcircuits,
        FlattenerConfig {
            hierarchy_separator: '/',
            ..FlattenerConfig::debug()
        },
    );
    flattener.flatten(&netlist).unwrap();
    let metadata = flattener.instance_metadata();
    assert_eq!(
        metadata
            .iter()
            .map(|m| m.path.to_string())
            .collect::<Vec<_>>(),
        ["Xtop", "Xtop/Xinner", "Xpeer"]
    );
    assert_eq!(metadata[0].children, ["Xtop/Xinner"]);
    assert!(metadata[1].children.is_empty());
    assert_eq!(metadata[1].path.depth(), 2);
    assert_eq!(metadata[1].path.parent().as_ref(), Some(&metadata[0].path));
    assert!(metadata[1].instance_params.is_empty());
    let resolver = flattener.param_resolver();
    for (path, r, local) in [("XTOP/XINNER", 20.0, 21.0), ("XPEER", 50.0, 51.0)] {
        let path = HierarchyPath::parse(path, '/');
        assert_eq!(resolver.resolve("r", &path), Some(r));
        assert_eq!(resolver.resolve("local", &path.child("R1")), Some(local));
        assert_eq!(resolver.resolve("inherited", &path), Some(7.0));
    }
    assert_eq!(resolver.resolve("r", &metadata[0].path), Some(40.0));
}

#[test]
fn failed_expansions_clear_metadata_and_a_retry_replaces_it() {
    let valid = Netlist::parse("* valid\n.PARAM value=1\n.SUBCKT cell a PARAMS: r=20\nR1 a 0 {r}\n.ENDS\nX1 out cell r=30\n.END\n").unwrap();
    let invalid =
        Netlist::parse("* invalid\n.PARAM value=2\nX2 out cell r=40\nXbad out missing\n.END\n")
            .unwrap();
    let mut flattener = Flattener::with_config(&valid.subcircuits, FlattenerConfig::debug());
    flattener.flatten(&valid).unwrap();
    assert_eq!(flattener.instance_metadata().len(), 1);
    assert!(flattener.flatten(&invalid).is_err());
    assert!(flattener.instance_metadata().is_empty());
    assert_eq!(
        flattener
            .param_resolver()
            .resolve("value", &HierarchyPath::root()),
        None
    );
    assert_eq!(
        flattener
            .param_resolver()
            .resolve("r", &HierarchyPath::parse("X2", '.')),
        None
    );
    flattener.flatten(&valid).unwrap();
    assert_eq!(flattener.instance_metadata().len(), 1);
    assert_eq!(flattener.instance_metadata()[0].path.to_string(), "X1");
    assert_eq!(
        flattener
            .param_resolver()
            .resolve("value", &HierarchyPath::root()),
        Some(1.0)
    );
}

#[test]
fn metadata_does_not_change_samples_and_is_opt_in() {
    let source = "* sampled metadata\n.options seed=37\n.SUBCKT cell a PARAMS: r=20\nR1 a 0 {r}\n.ENDS\nX1 out cell r={100+aunif(0,1)}\nX2 aux cell r={100+aunif(0,1)}\n.END\n";
    let run = |collect_metadata| {
        let netlist = Netlist::parse(source).unwrap();
        let mut flattener = Flattener::with_config(
            &netlist.subcircuits,
            FlattenerConfig {
                collect_metadata,
                ..Default::default()
            },
        );
        let elements = flattener.flatten(&netlist).unwrap();
        let values = elements
            .iter()
            .map(|e| match e.kind {
                rspice_core::netlist::ElementKind::Resistor { value, .. } => value,
                _ => panic!("expected a resistor"),
            })
            .collect::<Vec<_>>();
        if collect_metadata {
            for (metadata, value) in flattener.instance_metadata().iter().zip(&values) {
                assert_eq!(metadata.instance_params[0].1, *value);
                assert_eq!(
                    flattener.param_resolver().resolve("r", &metadata.path),
                    Some(*value)
                );
            }
        } else {
            assert!(flattener.instance_metadata().is_empty());
            assert_eq!(
                flattener
                    .param_resolver()
                    .resolve("r", &HierarchyPath::parse("X1", '.')),
                None
            );
        }
        (
            values,
            rspice_core::netlist::expr::eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        )
    };
    assert_eq!(run(false), run(true));
}

#[test]
fn non_real_bindings_shadow_numeric_ancestor_values_in_metadata() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::netlist::NetlistParseOptions;
    let netlist = Netlist::parse_with_options("* symbolic metadata\n.PARAM dynamic=10 z=20\n.SUBCKT cell a\n.PARAM dynamic={TIME} z={3+4j}\nR1 a 0 1k\n.ENDS\nX1 out cell\n.END\n", NetlistParseOptions { expression_dialect: ExpressionDialect::Xyce, ..Default::default() }).unwrap();
    let mut flattener = Flattener::with_config(&netlist.subcircuits, FlattenerConfig::debug());
    flattener.flatten(&netlist).unwrap();
    let path = &flattener.instance_metadata()[0].path;
    assert_eq!(flattener.param_resolver().resolve("dynamic", path), None);
    assert_eq!(flattener.param_resolver().resolve("z", path), None);
    assert_eq!(
        flattener
            .param_resolver()
            .resolve("dynamic", &HierarchyPath::root()),
        Some(10.0)
    );
}

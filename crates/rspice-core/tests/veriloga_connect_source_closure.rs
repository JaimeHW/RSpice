#![cfg(feature = "veriloga")]

use rspice_core::{
    Engine, Netlist, ProjectVerilogARuntimeRegistration,
    register_project_veriloga_runtimes_for_session,
};
use rspice_veriloga::{
    CompilerOptions, VerilogACompiler, VirtualCompileLimits, VirtualSourceBundle, VirtualSourceFile,
};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

struct SourceTree(PathBuf);
impl SourceTree {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-source-group-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn write(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, source).unwrap();
        path
    }
}
impl Drop for SourceTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct EditDuringCompilation {
    header: PathBuf,
    edited: AtomicBool,
}
impl rspice_core::abort_signal::AbortSignal for EditDuringCompilation {
    fn is_aborted(&self) -> bool {
        false
    }
    fn observe_progress(&self, fraction: f64) {
        // Preparation has completed; mutate the include while the first
        // selected module is being emitted, before the second is compiled.
        if fraction >= 0.8 && !self.edited.swap(true, Ordering::SeqCst) {
            std::fs::write(&self.header, "`define GAIN 2e-3\n").unwrap();
        }
    }
}

#[test]
fn connection_library_and_selected_modules_use_one_snapshot_then_refresh_changed_dependencies() {
    let tree = SourceTree::new();
    let header = tree.write("gain.vh", "`define GAIN 1e-3\n");
    let mut source = String::from("`include \"gain.vh\"\n");
    for name in ["first", "second"] {
        source.push_str(&format!(
            "module {name}(p,n,q); inout p,n; electrical p,n; output q; reg q; parameter real gain=`GAIN; initial q=1; analog I(p,n)<+gain*V(p,n); endmodule\n"
        ));
    }
    let root = tree.write("models.va", &source);
    let mut library = String::new();
    for (_, body) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        library.push_str(body);
    }
    library.push_str("\nconnectrules volts; connect d2a #(.vsup(1.0)); endconnectrules\n");
    let connections = tree.write("connections.vams", &library);
    let deck = Netlist::parse(&format!(
        "* one source snapshot for two selected modules\nV1 in 0 1\nR1 in p 1k\nX1 p 0 q1 FIRST\nX2 p 0 q2 SECOND\n.va \"{}\" FIRST module=first\n.va \"{}\" SECOND module=second\n.va \"{}\"\n.end\n",
        root.to_string_lossy().replace('\\', "/"),
        root.to_string_lossy().replace('\\', "/"),
        connections.to_string_lossy().replace('\\', "/")
    )).unwrap();
    let edit = EditDuringCompilation {
        header,
        edited: AtomicBool::new(false),
    };
    let first = Engine::default()
        .run_tran_with_abort(&deck, 2e-9, 0.2e-9, &edit)
        .unwrap();
    assert!(edit.edited.load(Ordering::SeqCst));
    let refreshed = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    for (result, expected) in [(&first, 1.0 / 3.0), (&refreshed, 1.0 / 5.0)] {
        for (node, voltage) in [("p", expected), ("q1", 1.0), ("q2", 1.0)] {
            let index = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(node))
                .unwrap();
            assert!(
                result.voltages[index]
                    .iter()
                    .all(|v| (v - voltage).abs() < 1e-9),
                "{node}: expected {voltage}, got {:?}",
                result.voltages[index]
            );
        }
    }
}

#[test]
fn virtual_models_share_active_included_connection_rules_after_transport_and_cache_hits() {
    let mut root = String::from("`define LOW_SUPPLY\n`include \"rules.vams\"\n");
    for name in ["first", "second"] {
        root.push_str(&format!(
            "module {name}(p,clk,q); input p; electrical p; input clk; wire clk; output q; wire q; assign q=clk; analog I(p)<+0; endmodule\n"
        ));
    }
    let mut rules = String::new();
    for (_, body) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        rules.push_str(body);
    }
    rules.push_str(
        "\n`ifdef LOW_SUPPLY\n`define SUPPLY 1.0\n`else\n`define SUPPLY 5.0\n`endif\nconnectrules selected; connect a2d #(.vsup(`SUPPLY)); connect d2a #(.vsup(`SUPPLY)); endconnectrules\n",
    );
    let bundle = VirtualSourceBundle::new(
        "models/root.vams",
        [
            VirtualSourceFile::new("models/root.vams", root),
            VirtualSourceFile::new("models/rules.vams", rules),
            VirtualSourceFile::new("inactive.vams", "invalid unused source"),
        ],
    )
    .unwrap();
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let mut registrations = Vec::new();
    let mut keys = Vec::new();
    for name in ["first", "second"] {
        let compiled = compiler
            .compile_virtual_runtime(&bundle, name, VirtualCompileLimits::default())
            .unwrap();
        compiled.validate_integrity().unwrap();
        let source_key = PathBuf::from(format!(
            "__rspice_project__/connect-closure-{}/{name}/root.vams",
            std::process::id()
        ));
        assert!(!source_key.exists());
        let canonical_ir =
            serde_json::from_slice(&serde_json::to_vec(&compiled.runtime.canonical_ir).unwrap())
                .unwrap();
        registrations.push(ProjectVerilogARuntimeRegistration {
            source_key: source_key.clone(),
            aliases: vec![name.to_owned()],
            model: compiled.runtime.model,
            canonical_ir,
        });
        keys.push(source_key);
    }
    register_project_veriloga_runtimes_for_session(registrations).unwrap();
    let deck = Netlist::parse(&format!(
        "* sealed active connection closure\nV1 p 0 1\nVclk clk 0 1\nX1 p clk q1 first\nX2 p clk q2 second\n.va \"{}\" first\n.va \"{}\" second\n.end\n",
        keys[0].display(), keys[1].display()
    ))
    .unwrap();
    // A second build uses the registered cache entries, with no ambient source.
    for _ in 0..2 {
        let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
        for node in ["q1", "q2"] {
            let index = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(node))
                .unwrap();
            assert!(
                result.voltages[index]
                    .iter()
                    .all(|v| (v - 1.0).abs() < 1e-9),
                "{node} must use the active 1 V conversion, not default 3.3 V or inactive 5 V: {:?}",
                result.voltages[index]
            );
        }
    }
}

#[test]
fn different_roots_cannot_mix_versions_of_a_shared_source_snapshot() {
    let tree = SourceTree::new();
    let header = tree.write("gain.vh", "`define GAIN 1e-3\n");
    let root_source = |name: &str| {
        format!(
            "`include \"gain.vh\"\nmodule {name}(p,n,q); inout p,n; electrical p,n; output q; reg q; parameter real gain=`GAIN; initial q=1; analog I(p,n)<+gain*V(p,n); endmodule\n"
        )
    };
    let first = tree.write("first.va", &root_source("first"));
    let second = tree.write("second.va", &root_source("second"));
    let deck = Netlist::parse(&format!(
        "* root source consistency\nV1 p 0 1\nX1 p 0 q1 first\nX2 p 0 q2 second\n.va \"{}\" first\n.va \"{}\" second\n.end\n",
        first.to_string_lossy().replace('\\', "/"),
        second.to_string_lossy().replace('\\', "/"),
    )).unwrap();
    let edit = EditDuringCompilation {
        header,
        edited: AtomicBool::new(false),
    };
    let error = Engine::default()
        .build_circuit_with_abort(&deck, &edit)
        .unwrap_err()
        .to_string();
    assert!(edit.edited.load(Ordering::SeqCst));
    for expected in ["gain.vh", "first.va", "second.va", "stable source snapshot"] {
        assert!(error.contains(expected), "{error}");
    }
    // The interrupted build may populate the optimization cache, but all
    // entries retain their captured identities and the next build can recover.
    Engine::default().build_circuit(&deck).unwrap();
}

fn connection_alternatives() -> String {
    let mut source = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
        .iter()
        .map(|(_, body)| *body)
        .collect::<String>();
    source.push_str("\nconnectrules Low; connect d2a #(.vsup(1.0)); endconnectrules\nconnectrules High; connect d2a #(.vsup(5.0)); endconnectrules\nconnectrules Empty; endconnectrules\n");
    source
}

#[test]
fn named_configuration_changes_file_cached_and_transported_virtual_boundaries() {
    let tree = SourceTree::new();
    let module = "`include \"rules.vams\"\nmodule driver(p,q); inout p; electrical p; output q; reg q; parameter real gain=1e-3; initial q=1; analog I(p)<+gain*V(p); endmodule\n";
    let rules = connection_alternatives();
    tree.write("rules.vams", &rules);
    let file = tree.write("driver.vams", module);
    let bundle = VirtualSourceBundle::new(
        "driver.vams",
        [
            VirtualSourceFile::new("driver.vams", module),
            VirtualSourceFile::new("rules.vams", rules),
        ],
    )
    .unwrap();
    let compiled = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_virtual_runtime(&bundle, "driver", VirtualCompileLimits::default())
    .unwrap();
    let virtual_key = PathBuf::from(format!(
        "__rspice_project__/named-configuration-{}/driver.vams",
        std::process::id()
    ));
    let canonical_ir =
        serde_json::from_slice(&serde_json::to_vec(&compiled.runtime.canonical_ir).unwrap())
            .unwrap();
    register_project_veriloga_runtimes_for_session(vec![ProjectVerilogARuntimeRegistration {
        source_key: virtual_key.clone(),
        aliases: vec!["DRIVER".to_owned()],
        model: compiled.runtime.model,
        canonical_ir,
    }])
    .unwrap();
    for path in [&file, &virtual_key] {
        let deck_text = format!(
            "* named connections\nV1 p 0 1\nX1 p q DRIVER gain=2e-3\n.va \"{}\" DRIVER\n",
            path.to_string_lossy().replace('\\', "/")
        );
        // Reconfiguration must reuse model code without reusing the previous
        // boundary selection, including after virtual transport/specialization.
        for (configuration, expected) in [("Low", 1.0), ("High", 5.0)] {
            let deck = Netlist::parse(&format!(
                "{deck_text}.options connectrules=\"{configuration}\"\n.end\n"
            ))
            .unwrap();
            assert_eq!(deck.options.connect_rules.as_deref(), Some(configuration));
            let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("q"))
                .unwrap();
            assert!(
                result.voltages[output]
                    .iter()
                    .all(|v| (v - expected).abs() < 1e-9),
                "{configuration}: {:?}",
                result.voltages[output]
            );
        }
        for (selection, expected) in [
            ("", "select one with .options connectrules=NAME"),
            (".options connectrules=low\n", "Unknown connectrules 'low'"),
            (
                ".options connectrules=Empty\n",
                "no connect statement applies",
            ),
        ] {
            let deck = Netlist::parse(&format!("{deck_text}{selection}.end\n")).unwrap();
            let error = Engine::default()
                .build_circuit(&deck)
                .unwrap_err()
                .to_string();
            assert!(error.contains(expected), "{error}");
        }
    }
}

#[test]
fn named_configuration_selects_across_roots_independently_of_include_order() {
    let tree = SourceTree::new();
    let device = tree.write("driver.va", "module driver(p,q); inout p; electrical p; output q; reg q; initial q=1; analog I(p)<+0; endmodule\n");
    let low = tree.write(
        "low.vams",
        &connection_alternatives().replace("connectrules High;", "connectrules Unused;"),
    );
    let high = tree.write(
        "high.vams",
        &connection_alternatives().replace("connectrules Low;", "connectrules Other;"),
    );
    // Only the High block from high.vams is selected; all other blocks in
    // either source stay inactive, including their duplicate Empty names.
    for sources in [[&low, &high], [&high, &low]] {
        let imports = sources
            .iter()
            .map(|path| format!(".va \"{}\"\n", path.to_string_lossy().replace('\\', "/")))
            .collect::<String>();
        let deck = Netlist::parse(&format!("* source selection\nV1 p 0 1\nX1 p q DRIVER\n.va \"{}\" DRIVER\n{imports}.options connectrules=High\n.end\n", device.to_string_lossy().replace('\\', "/"))).unwrap();
        let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("q"))
            .unwrap();
        assert!(
            result.voltages[output]
                .iter()
                .all(|v| (v - 5.0).abs() < 1e-9)
        );
        let mut ambiguous = deck.clone();
        ambiguous.options.connect_rules = Some("Empty".to_owned());
        let error = Engine::default()
            .build_circuit(&ambiguous)
            .unwrap_err()
            .to_string();
        for expected in ["ambiguous", "low.vams", "high.vams"] {
            assert!(error.contains(expected), "{error}");
        }
    }
}

#[test]
fn source_qualified_configuration_resolves_duplicate_names_after_virtual_remapping() {
    let tree = SourceTree::new();
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let mut file_keys = Vec::new();
    let mut virtual_keys = Vec::new();
    let mut registrations = Vec::new();
    for (name, supply) in [("LOWLIB", 1.0), ("HIGHLIB", 5.0)] {
        let bodies = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
            .iter()
            .map(|(_, body)| *body)
            .collect::<String>();
        let source = format!(
            "{bodies}\nconnectrules Shared; connect d2a #(.vsup({supply})); endconnectrules\nmodule {name}(p,q); inout p; electrical p; output q; reg q; initial q=1; analog I(p)<+0; endmodule\n"
        );
        file_keys.push(tree.write(&format!("{name}.vams"), &source));
        let bundle =
            VirtualSourceBundle::new("root.vams", [VirtualSourceFile::new("root.vams", source)])
                .unwrap();
        let compiled = compiler
            .compile_virtual_runtime(&bundle, name, VirtualCompileLimits::default())
            .unwrap();
        let source_key = PathBuf::from(format!(
            "__rspice_project__/qualified-configuration-{}/{name}.vams",
            std::process::id()
        ));
        let canonical_ir =
            serde_json::from_slice(&serde_json::to_vec(&compiled.runtime.canonical_ir).unwrap())
                .unwrap();
        registrations.push(ProjectVerilogARuntimeRegistration {
            source_key: source_key.clone(),
            aliases: vec![name.to_owned()],
            model: compiled.runtime.model,
            canonical_ir,
        });
        virtual_keys.push(source_key);
    }
    register_project_veriloga_runtimes_for_session(registrations).unwrap();
    for sources in [&file_keys, &virtual_keys] {
        let deck_text = format!(
            "* qualified connections\nV1 p 0 1\nX1 p q LOWLIB\n.va \"{}\" LOWLIB\n.va \"{}\" HIGHLIB\n.options connectrules=Shared\n",
            sources[0].to_string_lossy().replace('\\', "/"),
            sources[1].to_string_lossy().replace('\\', "/")
        );
        let mut deck = Netlist::parse(&format!("{deck_text}.end\n")).unwrap();
        let error = Engine::default()
            .build_circuit(&deck)
            .unwrap_err()
            .to_string();
        assert!(error.contains("ambiguous"), "{error}");
        // SPICE aliases ignore case; the Shared identifier retains Verilog case.
        // HIGH follows LOW in traversal, so selecting it must exclude LOW.
        for (alias, expected) in [("highlib", 5.0), ("lowlib", 1.0)] {
            deck = Netlist::parse(&format!(
                "{deck_text}.options connectrules_source=\"{alias}\"\n.end\n"
            ))
            .unwrap();
            let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("q"))
                .unwrap();
            assert!(
                result.voltages[output]
                    .iter()
                    .all(|v| (v - expected).abs() < 1e-9),
                "{alias}: {:?}",
                result.voltages[output]
            );
        }
        deck.options.connect_rules_source = Some("missing".to_owned());
        let error = Engine::default()
            .build_circuit(&deck)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("found 0 sources")
                && error.contains("LOWLIB")
                && error.contains("HIGHLIB"),
            "{error}"
        );
        deck.options.connect_rules_source = Some("lowlib".to_owned());
        deck.veriloga_includes[1].model_name = Some("LOWLIB".to_owned());
        let error = Engine::default()
            .build_circuit(&deck)
            .unwrap_err()
            .to_string();
        assert!(error.contains("found 2 sources"), "{error}");
    }
}

#[test]
fn standalone_virtual_connections_register_atomically_with_devices() {
    use rspice_core::{
        ProjectVerilogAConnectionLibraryRegistration, ProjectVerilogASourceRegistration as Source,
        ResourceLimits, register_project_veriloga_sources_for_session,
        register_project_veriloga_sources_for_session_with_limits,
    };
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let library_bundle = VirtualSourceBundle::new(
        "rules.vams",
        [VirtualSourceFile::new(
            "rules.vams",
            connection_alternatives(),
        )],
    )
    .unwrap();
    let prepared_library = compiler
        .prepare_virtual_runtime_source(&library_bundle, VirtualCompileLimits::default())
        .unwrap();
    assert_eq!(prepared_library.module_names().count(), 0);
    let artifact = serde_json::from_slice(
        &serde_json::to_vec(&prepared_library.connection_artifact().unwrap()).unwrap(),
    )
    .unwrap();
    let source_prefix = format!(
        "__rspice_project__/standalone-connections-{}",
        std::process::id()
    );
    let library_key = PathBuf::from(format!("{source_prefix}/rules.vams"));
    let device_key = PathBuf::from(format!("{source_prefix}/driver.vams"));
    assert!(!library_key.exists() && !device_key.exists());
    let runtime = compiler.compile_runtime("module driver(p,q); inout p; electrical p; output q; reg q; initial q=1; analog I(p)<+0; endmodule", None).unwrap();
    let device = ProjectVerilogARuntimeRegistration {
        source_key: device_key.clone(),
        aliases: vec!["DRIVER".to_owned()],
        model: runtime.model,
        canonical_ir: runtime.canonical_ir,
    };
    let library = ProjectVerilogAConnectionLibraryRegistration {
        source_key: library_key.clone(),
        aliases: vec!["CONNECTIONS".to_owned()],
        artifact,
    };
    let sources = vec![
        Source::Runtime(device.clone()),
        Source::Connections(library.clone()),
    ];
    register_project_veriloga_sources_for_session(sources.clone()).unwrap();
    let deck = |name: &str| {
        Netlist::parse(&format!("* standalone virtual library\nV1 p 0 1\nX1 p q DRIVER\n.va \"{}\" DRIVER\n.va \"{}\" CONNECTIONS\n.options connectrules={name} connectrules_source=connections\n.end\n", device_key.display(), library_key.display())).unwrap()
    };
    let assert_output = |name: &str, expected: f64| {
        let result = Engine::default()
            .run_tran(&deck(name), 2e-9, 0.2e-9)
            .unwrap();
        let q = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("q"))
            .unwrap();
        assert!(
            result.voltages[q]
                .iter()
                .all(|v| (v - expected).abs() < 1e-9),
            "{name}: {:?}",
            result.voltages[q]
        );
    };
    assert_output("Low", 1.0);

    let mut limits = ResourceLimits::default();
    limits.max_shared_cache_bytes = 1;
    let error = register_project_veriloga_sources_for_session_with_limits(sources.clone(), limits)
        .unwrap_err();
    assert!(
        error.contains("shared_cache_bytes limit exceeded"),
        "{error}"
    );
    let mut limits = ResourceLimits::default();
    limits.max_expanded_source_bytes = 1;
    let error =
        register_project_veriloga_sources_for_session_with_limits(sources, limits).unwrap_err();
    assert!(
        error.contains("expanded_source_bytes limit exceeded"),
        "{error}"
    );
    let mut collision = device.clone();
    collision.source_key = library_key.clone();
    let mut pending = device;
    pending.source_key = PathBuf::from(format!("{source_prefix}/pending.vams"));
    let error = register_project_veriloga_sources_for_session(vec![
        Source::Runtime(pending.clone()),
        Source::Runtime(collision),
    ])
    .unwrap_err();
    assert!(error.contains("differing installed artifact"), "{error}");
    let pending_deck = Netlist::parse(&format!(
        "* rejected candidate\nV1 p 0 1\nX1 p q DRIVER\n.va \"{}\" DRIVER\n.end\n",
        pending.source_key.display()
    ))
    .unwrap();
    let error = Engine::default()
        .build_circuit(&pending_deck)
        .unwrap_err()
        .to_string();
    assert!(error.contains("not installed"), "{error}");

    let mut wrong_kind = deck("Low");
    wrong_kind.veriloga_includes[1].selected_module = Some("driver".to_owned());
    let error = Engine::default()
        .build_circuit(&wrong_kind)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("registered connection library") && error.contains("cannot be selected"),
        "{error}"
    );
    // Failed budgets and mixed-kind collisions must leave both old entries usable.
    assert_output("High", 5.0);
}

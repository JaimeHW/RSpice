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

#![cfg(feature = "veriloga")]

use rspice_core::{
    Engine, Netlist, ProjectVerilogARuntimeRegistration,
    register_project_veriloga_runtimes_for_session,
};
use rspice_veriloga::{
    CompilerOptions, VerilogACompiler, VirtualCompileLimits, VirtualSourceBundle, VirtualSourceFile,
};
use std::path::PathBuf;

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

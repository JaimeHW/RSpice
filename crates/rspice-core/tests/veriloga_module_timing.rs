#![cfg(feature = "veriloga")]

use rspice_core::{
    Engine, Netlist, ProjectVerilogARuntimeRegistration,
    register_project_veriloga_runtimes_for_session,
};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::path::PathBuf;

const SOURCE: &str = r#"
`timescale 10ns/100ps
module coarse(q);
    output q; reg q;
    initial begin q=0; #0.125 q=1; end
endmodule
`timescale 1ns/1ps
module timed(p,n);
    inout p,n; electrical p,n;
    reg selected; wire late;
    coarse child(late);
    parameter real D=0.4505;
    initial begin selected=1; #D selected=0; end
    analog I(p,n) <+ (1 + selected + late) * V(p,n) / 1000;
endmodule
"#;

#[test]
fn module_time_queries_feed_digital_defaults_delays_and_analog_equations() {
    let source = r#"
`timescale 10ns/100ps
module coarse_query(q);
    output q; reg q;
    initial q=($simparam("timeUnit")==1e-8 && $simparam("timePrecision")==1e-10);
endmodule
`timescale 1ns/1ps
module queries(p,n);
    inout p,n; electrical p,n;
    reg selected; wire child_ok;
    coarse_query child(child_ok);
    parameter real D=$simparam("timeUnit")/2;
    initial begin
        selected=($simparam("timeUnit",99.0)==1e-9 && $simparam("timePrecision")==1e-12);
        #(D/1e-9) selected=0;
    end
    analog I(p,n)<+(1+selected+child_ok)*V(p,n)/1000;
endmodule
"#;
    let runtime = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(source, Some("queries"))
    .unwrap();
    let key = PathBuf::from(format!(
        "__rspice_project__/module-time-queries-{}/queries.va",
        std::process::id()
    ));
    register_project_veriloga_runtimes_for_session([ProjectVerilogARuntimeRegistration {
        source_key: key.clone(),
        aliases: vec!["timing_queries".into()],
        model: runtime.model,
        canonical_ir: runtime.canonical_ir,
    }])
    .unwrap();
    let deck = Netlist::parse(&format!("* declaration-owned digital queries\nV1 in 0 1\nR1 in p 1000\nX1 p 0 timing_queries\n.va \"{}\" timing_queries\n.end\n", key.display())).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 0.2e-9).unwrap();
    let p = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("p"))
        .unwrap();
    let mut phases = [0; 2];
    for (&time, &voltage) in result.time.iter().zip(&result.voltages[p]) {
        let phase = usize::from(time >= 0.5e-9 - 1e-20);
        phases[phase] += 1;
        let expected = if phase == 0 { 0.25 } else { 1.0 / 3.0 };
        assert!(
            (voltage - expected).abs() < 1e-9,
            "t={time:e}: {voltage}, expected {expected}"
        );
    }
    assert!(phases.into_iter().all(|count| count > 0));
    assert!(result.time.iter().any(|time| (time - 0.5e-9).abs() < 1e-20));
}

#[test]
fn module_timescales_drive_exact_mixed_breakpoints_from_file_and_sealed_sources() {
    let path = std::env::temp_dir().join(format!("rspice-module-timing-{}.va", std::process::id()));
    std::fs::write(&path, SOURCE).unwrap();
    struct SourceFile(PathBuf);
    impl Drop for SourceFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let _file = SourceFile(path.clone());
    let runtime = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(SOURCE, Some("timed"))
    .unwrap();
    let key = PathBuf::from(format!(
        "__rspice_project__/module-timing-{}/timed.va",
        std::process::id()
    ));
    register_project_veriloga_runtimes_for_session([ProjectVerilogARuntimeRegistration {
        source_key: key.clone(),
        aliases: vec!["timed".into()],
        model: runtime.model,
        canonical_ir: serde_json::from_slice(&serde_json::to_vec(&runtime.canonical_ir).unwrap())
            .unwrap(),
    }])
    .unwrap();

    for (source, selection) in [(path, " module=timed"), (key, "")] {
        let deck = Netlist::parse(&format!("* two module time scales\nV1 in 0 1\nR1 in p 1000\nX1 p 0 timed\n.va \"{}\" timed{selection}\n.end\n", source.display().to_string().replace('\\', "/"))).unwrap();
        let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
        let p = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case("p"))
            .unwrap_or_else(|| panic!("missing p in {:?}", result.node_names));
        let mut phases = [0; 3];
        for (&time, &voltage) in result.time.iter().zip(&result.voltages[p]) {
            let phase = if time < 0.451e-9 - 1e-20 {
                0
            } else if time < 1.3e-9 - 1e-20 {
                1
            } else {
                2
            };
            phases[phase] += 1;
            let expected = if phase == 1 { 0.5 } else { 1.0 / 3.0 };
            assert!(
                (voltage - expected).abs() < 1e-9,
                "{} at {time:e}: {voltage}, expected {expected}",
                source.display()
            );
        }
        assert!(phases.into_iter().all(|count| count > 0));
        for event in [0.451e-9, 1.3e-9] {
            assert!(
                result.time.iter().any(|time| (time - event).abs() < 1e-20),
                "{}: missing breakpoint {event:e}",
                source.display()
            );
        }
    }
}

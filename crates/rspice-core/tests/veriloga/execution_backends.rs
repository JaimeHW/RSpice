//! Every result names the code that ran it: the Verilog-A runtime this build
//! runs runtime-compiled modules on, the generated compact models, the
//! Verilog host and XSPICE's event-driven models.

use rspice_core::backends::{AnalogBackend, BackendRecorder, DigitalBackend, ExecutionBackends};
use rspice_core::engine::{SimulationConfig, SpiceDialect};
use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// The runtime a native build runs Verilog-A on, and every other host build.
const RUNTIME: AnalogBackend = if cfg!(feature = "veriloga-native") {
    AnalogBackend::NativeJit
} else {
    AnalogBackend::Interpreter
};

struct SourceFile(std::path::PathBuf);

impl SourceFile {
    fn new(extension: &str, source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_execution_backends_{}_{}.{extension}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).expect("write source file");
        Self(path)
    }

    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for SourceFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// What the operating point of `deck` ran.
fn ran(deck: &str) -> ExecutionBackends {
    ran_as(SpiceDialect::Ngspice, deck)
}

/// What the operating point of `deck` ran, read in `dialect`.
fn ran_as(dialect: SpiceDialect, deck: &str) -> ExecutionBackends {
    let netlist =
        Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"));
    let recorder = BackendRecorder::new();
    {
        let _recording = recorder.install();
        Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("the operating point solves: {error}\n{deck}"));
    }
    recorder.snapshot().expect("the run built a circuit")
}

const RESISTOR: &str = "`include \"disciplines.vams\"\n\
                        module vres(p, n);\n\
                        \x20 inout p, n;\n\
                        \x20 electrical p, n;\n\
                        \x20 parameter real r = 1000.0 from (0:inf);\n\
                        \x20 analog I(p, n) <+ V(p, n) / r;\n\
                        endmodule\n";

#[test]
fn built_in_devices_name_no_backend() {
    let ran = ran("* divider\nv1 in 0 1\nr1 in out 1k\nr2 out 0 1k\n.op\n.end\n");
    assert_eq!(ran, ExecutionBackends::default());
    assert_eq!(ran.to_string(), "analog: built-in devices; digital: none");
}

#[test]
fn a_runtime_compiled_module_names_the_runtime_of_this_build() {
    let model = SourceFile::new("va", RESISTOR);
    let ran = ran(&format!(
        "* divider\nv1 in 0 1\nx1 in out vres r=1k\nr2 out 0 1k\n.va \"{}\" vres\n.op\n.end\n",
        model.path()
    ));
    assert_eq!(ran.analog, [RUNTIME]);
    assert!(ran.digital.is_empty(), "{ran}");
    assert_eq!(
        ran.to_string(),
        format!("analog: {}; digital: none", RUNTIME.label())
    );
}

/// A design whose discrete half drives the deck through the boundary inside
/// it: the Verilog host runs it, and with no analog block of its own no
/// Verilog-A runtime is named.
#[test]
fn a_mixed_design_names_the_verilog_host() {
    let design = SourceFile::new(
        "vams",
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module drv(o);\n\
         \x20 output o;\n\
         \x20 reg o;\n\
         \x20 initial o = 1'b1;\n\
         endmodule\n\
         module t(n);\n\
         \x20 inout n;\n\
         \x20 electrical n;\n\
         \x20 drv u(n);\n\
         endmodule\n",
    );
    let ran = ran(&format!(
        "* held one\n.hdl \"{}\" module=t\n.param vcc=3.3\nxt out t\nrload out 0 1k\n.op\n.end\n",
        design.path()
    ));
    assert_eq!(ran.digital, [DigitalBackend::VerilogHost], "{ran}");
    assert!(ran.analog.is_empty(), "{ran}");
}

/// An authored event-driven code model is XSPICE's digital engine; the
/// bridges into and out of it are part of that engine too.
#[test]
fn an_xspice_digital_model_names_xspice() {
    let ran = ran("* inverter\n\
         v1 in 0 1\n\
         a1 [in] [din] adc1\n\
         .model adc1 adc_bridge(in_low=0.3 in_high=0.7)\n\
         a2 din dout inv1\n\
         .model inv1 d_inverter\n\
         a3 [dout] [out] dac1\n\
         .model dac1 dac_bridge(out_low=0 out_high=1)\n\
         r1 out 0 1k\n\
         .op\n\
         .end\n");
    assert_eq!(ran.digital, [DigitalBackend::Xspice], "{ran}");
    assert!(ran.analog.is_empty(), "{ran}");
}

#[cfg(feature = "veriloga-model-diode-cmc")]
#[test]
fn a_generated_compact_model_is_named_by_its_module() {
    let ran = ran("* diode\nv1 in 0 0.6\nr1 in a 1k\nxd a 0 diode_cmc\n.op\n.end\n");
    assert_eq!(ran.analog, [AnalogBackend::GeneratedModel], "{ran}");
    assert!(
        ran.generated_models
            .iter()
            .any(|model| model.eq_ignore_ascii_case("diode_cmc")),
        "{ran}"
    );
}

/// EKV 2.6 by LEVEL runs the generated `ekv_va` where it is compiled in.
/// Without it the native EKV 2.6 port runs the card, and the result and the
/// log both say so.
#[cfg(not(feature = "veriloga-model-ekv-va"))]
#[test]
fn ekv_by_level_without_its_generated_module_runs_the_native_port_and_says_so() {
    let (ran, log) = crate::log_capture::lines_logged(log::LevelFilter::Warn, || {
        ran("* EKV 2.6\n\
             vdd d 0 dc 1.8\n\
             vg g 0 dc 1.2\n\
             m1 d g 0 0 ekvm w=10u l=1u\n\
             .model ekvm nmos level=260\n\
             .op\n.end\n")
    });
    let [substitution] = ran.substitutions.as_slice() else {
        panic!("one substitution: {ran}")
    };
    assert!(substitution.instance.eq_ignore_ascii_case("m1"), "{ran}");
    assert!(substitution.model.eq_ignore_ascii_case("ekvm"), "{ran}");
    assert_eq!(substitution.requested, "ekv_va");
    assert_eq!(substitution.feature, "veriloga-model-ekv-va");
    assert_eq!(substitution.ran, "EKV 2.6");
    assert!(
        ran.to_string()
            .ends_with("ran the native EKV 2.6 in place of ekv_va (feature veriloga-model-ekv-va)"),
        "{ran}"
    );
    assert!(
        log.iter()
            .any(|line| line.contains("ekv_va") && line.contains("native EKV 2.6")),
        "{log:?}"
    );
}

/// A Xyce `Q` card selects VBIC 1.3 by LEVEL=11 and 12; without the generated
/// module the native VBIC runs it and says so. Read as ngspice, the same card
/// asks for no generated module, so nothing was substituted.
#[cfg(not(feature = "veriloga-model-vbic13"))]
#[test]
fn vbic_by_xyce_level_without_its_generated_module_runs_the_native_port_and_says_so() {
    let deck = "* VBIC\nvc c 0 5\nvb b 0 0.7\nrc c cc 1k\nq1 cc b 0 qm\n\
                .model qm NPN LEVEL=11\n.op\n.end\n";
    let ran = ran_as(SpiceDialect::Xyce, deck);
    let [substitution] = ran.substitutions.as_slice() else {
        panic!("one substitution: {ran}")
    };
    assert!(substitution.instance.eq_ignore_ascii_case("q1"), "{ran}");
    assert_eq!(substitution.requested, "vbic13");
    assert_eq!(substitution.feature, "veriloga-model-vbic13");
    assert_eq!(substitution.ran, "VBIC");
    assert!(ran_as(SpiceDialect::Ngspice, deck).substitutions.is_empty());
}

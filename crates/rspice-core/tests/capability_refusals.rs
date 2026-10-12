//! Capability refusals reach the engine's public API as typed refusals.
//!
//! "RSpice understood this deck and declines to run it" is categorically
//! different from "this deck is wrong", and every frontend reports the two
//! differently: an unsupported analysis/device pair is a capability gap to
//! record against the roadmap, while a malformed deck is the author's to fix.
//!
//! These tests drive real decks through the public entry points rather than
//! constructing errors, because the value of the taxonomy is that the refusal
//! survives the whole path from the parser or the elaborator out to the caller.

use rspice_core::analysis::HbConfig;
use rspice_core::netlist::ParseError;
use rspice_core::{Engine, Netlist, SimulationError, SimulationErrorCategory, SimulationErrorCode};

/// The capability token a refusal published, or a panic naming what came back.
fn capability_token(error: &SimulationError) -> &'static str {
    let descriptor = error.descriptor();
    assert_eq!(
        descriptor.code,
        SimulationErrorCode::UnsupportedCapability,
        "expected a capability refusal, got {error}"
    );
    assert_eq!(descriptor.category, SimulationErrorCategory::Capability);
    match error {
        SimulationError::UnsupportedCapability(refusal) => refusal.capability,
        other => panic!("descriptor and variant disagreed: {other}"),
    }
}

fn parse(source: &str) -> Netlist {
    Netlist::parse(source).expect("deck parses")
}

/// Resolve a deck node to the solver index the analysis entry points take.
fn node(netlist: &Netlist, name: &str) -> usize {
    Engine::default()
        .build_circuit(netlist)
        .expect("circuit builds")
        .get_node_by_name(name)
        .unwrap_or_else(|| panic!("node {name} exists"))
}

#[test]
fn harmonic_balance_refuses_a_device_it_cannot_stamp() {
    // Exact HB stamps the Gummel-Poon electrical equations -- a plain NPN
    // card solves -- so the device it declines is narrower than "a BJT": the
    // legacy excess-phase extension delays transport current in time, and the
    // harmonic residual carries no such state. The card is a device HB
    // understands and declines rather than a bad card. `LEVEL=1` is
    // load-bearing: `TD` on its own selects VBIC, whose excess phase IS
    // represented, and the deck would then be accepted.
    let netlist = parse(
        "hb capability\n\
         V1 in 0 SIN(0 0.5 1e6)\n\
         Vcc vcc 0 5\n\
         R1 in b 1k\n\
         Rc vcc c 1k\n\
         Q1 c b 0 npnmod\n\
         .model npnmod NPN LEVEL=1 IS=1e-16 BF=100 TD=1n\n\
         .end\n",
    );
    let error = Engine::default()
        .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
        .expect_err("HB must refuse a device it has no stamp for");
    // The exact token, not a prefix: a refusal from some other HB gate would
    // satisfy the namespace and hide the day this stamp arrives.
    assert_eq!(capability_token(&error), "analysis.hb.device");
    assert!(
        error.to_string().contains("excess-phase"),
        "the refusal must name the equations HB cannot stamp: {error}"
    );
}

#[test]
fn driven_pss_refuses_a_source_waveform_it_cannot_authenticate() {
    let netlist = parse(
        "pss capability\n\
         V1 in 0 PWL(0 0 1u 1 2u 0)\n\
         R1 in out 1k\n\
         C1 out 0 1n\n\
         .end\n",
    );
    let error = Engine::default()
        .validate_periodic_source_contract(&netlist, &["V1".to_owned()], 1.0e6)
        .expect_err("a PWL drive has no authenticated period");
    assert_eq!(
        capability_token(&error),
        "analysis.pss.driven_source_waveform"
    );
}

#[test]
fn pole_zero_refuses_transmission_lines() {
    let netlist = parse(
        "pz capability\n\
         V1 in 0 1\n\
         T1 in 0 out 0 Z0=50 TD=1n\n\
         R1 out 0 50\n\
         .end\n",
    );
    let output = node(&netlist, "out");
    let input = node(&netlist, "in");
    let error = Engine::default()
        .run_pz(&netlist, input, output)
        .expect_err("pole-zero has no distributed-line descriptor");
    assert_eq!(capability_token(&error), "analysis.pz.device");
}

#[test]
fn noise_refuses_a_charge_model_it_does_not_implement() {
    // A fractional BSIM4 CVCHARGEMOD is a valid authored selection whose
    // charge equations this build does not implement. DC still solves; every
    // charge-based analysis, noise included, declines it.
    let netlist = parse(
        "noise capability\n\
         .model NCH NMOS LEVEL=14 VERSION=4.8 CVCHARGEMOD=1.5\n\
         M1 d g 0 0 NCH W=1u L=1u\n\
         Vg g 0 DC 0.8 AC 1\n\
         Vdd dd 0 DC 1.2\n\
         Rd dd d 1k\n\
         .end\n",
    );
    // Node 1 is the first non-ground node; the deck is refused during
    // elaboration, before any index could be resolved from a built circuit.
    let error = Engine::default()
        .run_noise(&netlist, 1, &[1.0e6], 300.15)
        .expect_err("an unimplemented charge model cannot produce noise");
    assert_eq!(capability_token(&error), "device.bsim4.cvchargemod");
}

#[test]
fn rlgc_ltra_with_shunt_conductance_is_refused_before_any_solve() {
    let netlist = parse(
        "ltra capability\n\
         V1 in 0 1\n\
         O1 in 0 out 0 rgline\n\
         .model rgline LTRA R=1 G=1e-3 L=1n C=1p LEN=1\n\
         Rl out 0 50\n\
         .end\n",
    );
    let error = Engine::default()
        .run_dc_op(&netlist)
        .expect_err("neither reference simulator defines an RLGC line with shunt conductance");
    assert_eq!(capability_token(&error), "device.ltra.rlgc_conductance");
}

#[test]
fn xyce_y_device_families_are_refused_by_the_grammar_with_their_span() {
    let error = Netlist::parse(
        "y capability\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         YDELAY delay1 2 0 1 0 TD=10N\n\
         .op\n\
         .end\n",
    )
    .expect_err("an unlowerable Y-device family must not parse");

    let ParseError::UnsupportedCapability {
        origin,
        capability,
        detail,
    } = &error
    else {
        panic!("Y-device refusals must be capability refusals, got {error}");
    };
    assert_eq!(*capability, "netlist.xyce.ydevice.no_model_program");
    assert_eq!(origin.line, 4, "the refusal must name the authoring line");
    assert!(
        detail.contains("YDELAY"),
        "the refusal must name the keyword: {detail}"
    );
}

#[test]
fn a_checkpoint_request_this_deck_cannot_satisfy_is_refused_before_the_run() {
    // A voltage-controlled switch accumulates accepted hysteresis state that
    // the checkpoint format does not carry, so asking for a checkpoint is a
    // capability gap — not a bad deck, which is why the same deck must still
    // run. The refusal comes before any solver work: the only consumer of a
    // checkpoint is a resume, so there is nothing to be gained by solving to
    // tstop first and refusing then.
    let netlist = parse(
        "checkpoint capability\n\
         V1 in 0 1\n\
         VC c 0 1\n\
         S1 in out c 0 SW1\n\
         R1 out 0 1k\n\
         .model SW1 SW(VT=0.5 VH=0.1 RON=1 ROFF=1meg)\n\
         .end\n",
    );
    let error = Engine::default()
        .run_tran_checkpointed(&netlist, 10.0e-9, 1.0e-9)
        .expect_err("a deck whose accepted state cannot be captured has no checkpoint to return");
    assert_eq!(
        capability_token(&error),
        "analysis.tran.checkpoint_capability"
    );
    assert!(
        error
            .to_string()
            .contains("voltage-controlled switch accepted hysteresis state is not checkpointed"),
        "the refusal must name the state owner that blocks the checkpoint: {error}"
    );

    Engine::default()
        .run_tran(&netlist, 10.0e-9, 1.0e-9)
        .expect("the same deck runs when nothing asks for a checkpoint");
}

#[test]
fn a_capability_refusal_keeps_its_category_across_the_parse_boundary() {
    // The parser and the elaborator raise refusals through different error
    // types. Both must arrive at a caller as the same category, otherwise a
    // frontend has to know which stage refused in order to report it.
    let netlist = parse(
        "y capability in elaboration\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .end\n",
    );
    let elaborated = Engine::default().run_dc_op(&netlist);
    assert!(elaborated.is_ok(), "control deck must run");

    let refused = Netlist::parse(
        "y capability\n\
         V1 in 0 1\n\
         YNEURON n1 a b neuronmod\n\
         .op\n\
         .end\n",
    )
    .expect_err("neuron families are owned by a separate effort");
    assert!(
        matches!(refused, ParseError::UnsupportedCapability { .. }),
        "got {refused}"
    );
}

/// Pole-zero over runtime Verilog-A devices, whose descriptor admission is
/// decided per instance by the operators the compiled analog body uses.
///
/// Placed beside [`pole_zero_refuses_transmission_lines`] because it is the
/// same refusal: a `G + sC` descriptor pair has no room for a transport delay
/// or for state the runtime carries privately, and either one linearized at a
/// single frequency is a two-point fit rather than the device's response.
#[cfg(feature = "veriloga")]
mod veriloga_pole_zero {
    use super::{Engine, capability_token, node, parse};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn write_model(name: &str, source: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("rspice_pz_{}_{}.va", name, std::process::id()));
        let mut file = std::fs::File::create(&path).expect("create model file");
        file.write_all(source.as_bytes()).expect("write model");
        path
    }

    /// `V(out)/I(in)` across a 1k series resistor into a 1k load the Verilog-A
    /// instance shunts, so the transfer is `1000/(1 + 1000*Yd(s))` and carries
    /// the module's response and nothing else. The drive is a current source
    /// because a `.pz` voltage input parallel to an ideal source has no
    /// transfer to extract.
    fn pz_deck(model: &Path, module: &str) -> String {
        format!(
            "* pole-zero over a runtime Verilog-A device\n\
             I1 in 0 DC 0 AC 1\n\
             R1 in out 1k\n\
             RL out 0 1k\n\
             X1 out 0 {module}\n\
             .va \"{}\" {module}\n\
             .end\n",
            model.display().to_string().replace('\\', "/"),
        )
    }

    /// Run the deck, require a `analysis.pz.device` refusal, and hand back its
    /// text folded to lower case: a deck names its instance in whichever case
    /// the elaborator kept, and that is not what these tests are pinning.
    fn refused(model: &Path, module: &str, why: &str) -> String {
        let netlist = parse(&pz_deck(model, module));
        let input = node(&netlist, "in");
        let output = node(&netlist, "out");
        let error = match Engine::default().run_pz(&netlist, input, output) {
            Ok(result) => panic!(
                "{why}, but pole-zero answered with poles {:?}",
                result.poles
            ),
            Err(error) => error,
        };
        assert_eq!(capability_token(&error), "analysis.pz.device");
        error.to_string().to_ascii_lowercase()
    }

    #[test]
    fn pole_zero_refuses_a_verilog_a_transport_delay() {
        // Yd(s) = 1e-3*exp(-s*1n), so the transfer is 1000/(1 + exp(-s*1n)):
        // infinitely many poles on the imaginary axis, at s = j*pi*(2k+1)*1e9.
        // Sampling that at w = 0 and w = 1 rad/s fits Im(Yd)/w = -1e-12, a
        // NEGATIVE capacitance, and before this refusal `.pz` answered with a
        // single pole at s = +2e9 -- one root, wrong magnitude, and a sign
        // that reports a passive delay-loaded network as unstable.
        let model = write_model(
            "absdelay",
            "module pz_absdelay(p, n);\n\
             inout p, n;\n\
             electrical p, n;\n\
             analog I(p, n) <+ 1.0e-3 * absdelay(V(p, n), 1.0e-9);\n\
             endmodule\n",
        );
        let message = refused(
            &model,
            "pz_absdelay",
            "exp(-s*td) has no rational G + sC descriptor",
        );
        for expected in ["'x1'", "'pz_absdelay'", "uses absdelay"] {
            assert!(
                message.contains(expected),
                "the refusal must name the instance, the module and the operator: {message}"
            );
        }
        let _ = std::fs::remove_file(model);
    }

    #[test]
    fn pole_zero_refuses_a_verilog_a_laplace_filter() {
        // A rational response is refused too, and for a reason the analytic
        // answer makes concrete. Yd(s) = 1e-3/(1 + s*1n) gives a transfer of
        // 1000*(1 + s*1n)/(2 + s*1n): a pole at -2e9 and a zero at -1e9. The
        // filter's state is the runtime's own, never a descriptor column, so
        // the two-point fit returned a pole at +2e9 and NO zero at all.
        let model = write_model(
            "laplace",
            "module pz_laplace(p, n);\n\
             inout p, n;\n\
             electrical p, n;\n\
             analog I(p, n) <+ 1.0e-3 * laplace_nd(V(p, n), '{1.0}, '{1.0, 1.0e-9});\n\
             endmodule\n",
        );
        let message = refused(
            &model,
            "pz_laplace",
            "a laplace filter's state is not a descriptor column",
        );
        for expected in ["'x1'", "'pz_laplace'", "uses laplace"] {
            assert!(
                message.contains(expected),
                "the refusal must name the instance, the module and the operator: {message}"
            );
        }
        let _ = std::fs::remove_file(model);
    }

    #[test]
    fn pole_zero_admits_a_ddt_only_verilog_a_device() {
        // The other half of the declaration: `ddt` charge *is* the descriptor's
        // C contribution, so a module that integrates nothing else is admitted
        // exactly like a native capacitor. Yd(s) = 1e-3 + s*1e-12 gives
        // 1000/(2 + s*1e-9), whose single pole at -2e9 is what PZ must return.
        let model = write_model(
            "ddt",
            "module pz_ddt(p, n);\n\
             inout p, n;\n\
             electrical p, n;\n\
             analog I(p, n) <+ 1.0e-3 * V(p, n) + ddt(1.0e-12 * V(p, n));\n\
             endmodule\n",
        );
        let netlist = parse(&pz_deck(&model, "pz_ddt"));
        let input = node(&netlist, "in");
        let output = node(&netlist, "out");
        let result = Engine::default()
            .run_pz(&netlist, input, output)
            .expect("a ddt-only module exports its charge as a finite descriptor state");
        assert_eq!(result.poles.len(), 1, "{:#?}", result.poles);
        let pole = result.poles[0];
        assert!(
            (pole.re + 2.0e9).abs() <= 1.0e-6 * 2.0e9 && pole.im.abs() <= 1.0e-6 * 2.0e9,
            "the admitted descriptor must return the analytic pole -2e9: {pole}"
        );
        let dc_gain = result.dc_gain.expect("the transfer has a finite DC value");
        assert!(
            (dc_gain - 500.0).abs() <= 1.0e-6 * 500.0,
            "H(0) = 1000/(1 + 1000*1e-3): {dc_gain}"
        );
        let _ = std::fs::remove_file(model);
    }
}

/// The analyses a mixed Verilog-AMS deck takes part in around its held
/// equilibrium, and the ones that still refuse it by name.
///
/// The operating point is committed as the held equilibrium (P1 design S1),
/// so every small-signal analysis and a DC sweep run on a mixed deck. A
/// periodic analysis asks for a steady state with a *running* discrete half,
/// which has no formulation yet (P7), and pole-zero waits for the descriptor
/// wiring (P1.6); both refuse through the capability matrix with a typed
/// token, analysis kind, instance and module.
#[cfg(feature = "veriloga")]
mod mixed_signal_discrete_state {
    use super::{Engine, SimulationError, capability_token, node, parse};
    use std::io::Write;
    use std::path::PathBuf;

    /// A module with both halves: a process that never settles, and a
    /// conductance the analog solver would have to stamp.
    const CLOCK_DIVIDER: &str = "`include \"disciplines.vams\"\n\
         module clock_divider(p, n, qdiv);\n\
         \x20 inout p, n; electrical p, n;\n\
         \x20 output qdiv; reg clk, qdiv;\n\
         \x20 initial clk = 1'b0;\n\
         \x20 initial qdiv = 1'b0;\n\
         \x20 always #5 clk = ~clk;\n\
         \x20 always @(posedge clk) qdiv <= ~qdiv;\n\
         \x20 analog I(p, n) <+ V(p, n) / 1000.0;\n\
         endmodule\n";

    /// One file per test: the tests run in parallel and each removes its own.
    fn write_model(test: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_mixed_capability_{test}_{}.va",
            std::process::id()
        ));
        let mut file = std::fs::File::create(&path).expect("create model file");
        file.write_all(CLOCK_DIVIDER.as_bytes())
            .expect("write model");
        path
    }

    fn clock_divider_deck(model: &std::path::Path) -> String {
        format!(
            "* a mixed module around its held equilibrium\n\
             v1 in 0 dc 1 ac 1 sin(1 0.1 1meg)\n\
             r1 in p 1k\n\
             x1 p 0 qdiv clock_divider\n\
             rq qdiv 0 10k\n\
             .va \"{}\" clock_divider\n\
             .end\n",
            model.display().to_string().replace('\\', "/"),
        )
    }

    /// The structured identity every matrix refusal carries.
    fn assert_refusal(
        error: &SimulationError,
        token: &str,
        kind: rspice_core::identity::AnalysisKind,
        label: &str,
    ) {
        assert_eq!(capability_token(error), token, "{error}");
        let SimulationError::UnsupportedCapability(refusal) = error else {
            unreachable!("capability_token checked the variant");
        };
        assert_eq!(refusal.analysis_kind, Some(kind), "{error}");
        assert_eq!(refusal.instance.as_deref(), Some("x1"), "{error}");
        assert_eq!(refusal.module.as_deref(), Some("clock_divider"), "{error}");
        assert_eq!(error.descriptor().analysis_kind, Some(kind));
        assert_eq!(error.instance_name(), Some("x1"));
        let lowered = error.to_string().to_ascii_lowercase();
        assert!(
            lowered.contains(&label.to_ascii_lowercase()) && lowered.contains("'x1'"),
            "the prose names the analysis and the instance: {error}"
        );
        // The token tracks which roadmap item closes the cell; the prose a
        // person reads names no plan item or lane.
        let text = error.to_string();
        let names_a_plan_item = text.as_bytes().windows(4).any(|window| {
            window[0] == b'P'
                && window[1].is_ascii_digit()
                && window[2] == b'.'
                && window[3].is_ascii_digit()
        });
        assert!(
            !names_a_plan_item && !lowered.contains("planned"),
            "no plan identifier reaches the refusal text: {error}"
        );
    }

    #[test]
    fn every_analysis_around_the_held_state_runs_on_a_mixed_deck() {
        let model = write_model("held");
        let netlist = parse(&clock_divider_deck(&model));
        let probe = node(&netlist, "p");
        let engine = Engine::default();

        let sweep = engine
            .run_dc_sweep(&netlist, "v1", 0.0, 1.0, 0.5)
            .expect(".dc sweeps a mixed deck, carrying its held state");
        assert_eq!(sweep.len(), 3);
        let ac = engine
            .run_ac(&netlist, &[1.0e3])
            .expect(".ac linearizes the held equilibrium");
        // The module's continuous half is a 1k conductance on p: the divider
        // with r1 halves the drive, which is the module taking part.
        let p = ac[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("p"))
            .expect("p is published");
        assert!(
            (ac[0].voltages[p].re - 0.5).abs() < 1.0e-9,
            "{:?}",
            ac[0].voltages[p]
        );
        engine
            .run_noise(&netlist, probe, &[1.0e3], 300.15)
            .expect(".noise runs on the held equilibrium");
        engine
            .run_sensitivity_linearized(&netlist, probe, None)
            .expect(".sens runs on the held equilibrium");
        engine
            .run_transfer_function(&netlist, "p", None, false, "v1")
            .expect(".tf runs on the held equilibrium");
        engine
            .run_dc_op(&netlist)
            .expect("`.op` solves a mixed deck at its coupled time-zero fixed point");

        let _ = std::fs::remove_file(model);
    }

    /// A periodic analysis asks for a steady state with the discrete half
    /// running, which the matrix refuses (P7), and pole-zero waits for the
    /// descriptor wiring (P1.6). Both name the analysis kind, the instance
    /// and its module as typed fields, not only in prose.
    #[test]
    fn a_periodic_analysis_refuses_a_running_discrete_state_by_name() {
        use rspice_core::identity::AnalysisKind;
        let model = write_model("periodic");
        let netlist = parse(&clock_divider_deck(&model));
        let engine = Engine::default();

        let pss = engine
            .run_pss(
                &netlist,
                rspice_core::analysis::PssConfig::new(1.0e6).with_harmonics(2),
            )
            .expect_err("a mixed deck has no periodic steady state yet");
        assert_refusal(
            &pss,
            "analysis.mixed_signal.discrete_state",
            AnalysisKind::Pss,
            "PSS analysis",
        );

        let hb = engine
            .run_hb(&netlist, super::HbConfig::new(1.0e6).with_harmonics(2))
            .expect_err("a mixed deck has no harmonic-balance formulation yet");
        assert_refusal(
            &hb,
            "analysis.mixed_signal.discrete_state",
            AnalysisKind::HarmonicBalance,
            "harmonic-balance analysis",
        );

        let pz = engine
            .run_pz(&netlist, node(&netlist, "in"), node(&netlist, "p"))
            .expect_err("pole-zero of a held mixed module lands with the descriptor wiring");
        assert_refusal(
            &pz,
            "analysis.mixed_signal.discrete_state",
            AnalysisKind::PoleZero,
            "pole-zero analysis",
        );

        let _ = std::fs::remove_file(model);
    }
}

/// F4: a refusal raised while the shared AC bias is prepared names the
/// caller's analysis, not "AC analysis" — the transfer function here.
#[test]
fn a_transfer_function_refusal_names_the_transfer_function() {
    let netlist = parse(
        "tf over a coupled line\n\
         V1 in 0 DC 1 AC 1\n\
         R1 in a 50\n\
         P1 a b 0 c d 0 cpl_model\n\
         .model cpl_model CPL\n+ R = 0.1 0 0.1\n+ L = 1e-6 0 1e-6\n+ G = 0 0 0\n\
         + C = 1e-10 0 1e-10\n+ length = 0.1\n\
         R2 c 0 50\n\
         R3 b 0 50\n\
         R4 d 0 50\n\
         .end\n",
    );
    let error = Engine::default()
        .run_transfer_function(&netlist, "c", None, false, "V1")
        .expect_err("a coupled line has no small-signal load");
    assert_eq!(
        capability_token(&error),
        "analysis.ac.device.coupled_transmission_line"
    );
    assert_eq!(
        error.descriptor().analysis_kind,
        Some(rspice_core::identity::AnalysisKind::TransferFunction)
    );
    assert!(
        error.to_string().contains("transfer function"),
        "the refusal names the transfer function: {error}"
    );
}

/// Every supported cell of the capability matrix, run on a fixture of its
/// family: the evidence the matrix cites for "supported", in one place.
///
/// Each fixture is one shared analog harness — an inverting amplifier with a
/// loop probe, an S-parameter port and a swept load — plus the family's
/// instance. For every analysis the matrix marks supported, or restricted by
/// a gate the fixture passes, the analysis must run; for every refused cell
/// whose analysis the fixture can ask for, it must refuse with the cell's
/// token.
#[cfg(feature = "veriloga")]
mod capability_matrix_cells {
    use super::{Engine, SimulationError, capability_token, node, parse};
    use rspice_core::analysis::{FourierAnalysis, FourierConfig, PssConfig, StbConfig};
    use rspice_core::engine::{
        CapabilityAnalysis, CapabilityFamily, CellSupport, InstanceGate, capability_cell,
    };
    use rspice_core::netlist::AnalysisCommand;
    use rspice_core::{Netlist, NoAbort};

    struct SourceFile(std::path::PathBuf);

    impl SourceFile {
        fn new(name: &str, source: &str) -> Self {
            // The name is the whole file name, in a directory of this
            // process's own: an unselected `.hdl` source declares its module
            // by its file stem.
            let directory = std::env::temp_dir()
                .join(format!("rspice_capability_fixture_{}", std::process::id()));
            std::fs::create_dir_all(&directory).expect("create fixture directory");
            let path = directory.join(name);
            std::fs::write(&path, source).expect("write fixture source");
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

    const HARNESS: &str = "vin in 0 dc 0.5 sin(0.5 0.1 1meg) ac 1 distof1 0.01\n\
         r1 in sum 1k\n\
         rf sum fb 1k\n\
         vprobe out fb 0\n\
         eamp out 0 0 sum 100\n\
         rl out 0 {rlv}\n\
         .param rlv=1k\n\
         rs1 out p2 100\n\
         rs2 p2 0 100\n\
         .sp lin 1 1k 1k port1=(p2)\n\
         .fft v(out) np=16\n";

    const NOISY_LOAD: &str = "`include \"disciplines.vams\"\n\
         module noisy_load(p, n);\n\
           inout p, n; electrical p, n;\n\
           analog I(p, n) <+ V(p, n) / 5k + ddt(10p * V(p, n)) + white_noise(1.0e-22, \"load\");\n\
         endmodule\n";

    const TRIM: &str = "`timescale 1ns/1ps\n\
         module trim_reg(csn, tr1, tr0);\n\
           input csn;\n\
           output tr1, tr0;\n\
           reg tr1, tr0;\n\
           initial begin tr1 = 1'b1; tr0 = 1'b0; end\n\
           always @(negedge csn) tr0 = 1'b1;\n\
         endmodule\n";

    /// The family's instance, attached to the harness, and the files it reads.
    fn fixture(family: CapabilityFamily) -> Option<(String, Vec<SourceFile>)> {
        Some(match family {
            CapabilityFamily::SpiceNative => (
                "d1 out dn dmod\nrdn dn 0 10k\nc1 out 0 1n\n.model dmod d is=1e-14\n".into(),
                Vec::new(),
            ),
            CapabilityFamily::XspiceAnalog => (
                "again out xo gainblk\n.model gainblk gain (gain=0.5)\nrxo xo 0 1k\n".into(),
                Vec::new(),
            ),
            CapabilityFamily::XspiceEvent => (
                "aadc [in] [od] adcm\n.model adcm adc_bridge (in_low=0.25 in_high=0.25)\n\
                 ainv od odn invm\n.model invm d_inverter\n\
                 adac [odn] [xq] dacm\n.model dacm dac_bridge (out_low=0 out_high=1)\n\
                 rxq xq sum 100k\n"
                    .into(),
                Vec::new(),
            ),
            CapabilityFamily::VerilogARuntime => {
                let load = SourceFile::new("noisy_load.va", NOISY_LOAD);
                (
                    format!("xva out 0 noisy_load\n.va \"{}\" noisy_load\n", load.path()),
                    vec![load],
                )
            }
            CapabilityFamily::VerilogAGenerated => {
                if !cfg!(feature = "veriloga-model-diode-cmc") {
                    return None;
                }
                ("xd2 out dg diode_cmc\nrdg dg 0 1k\n".into(), Vec::new())
            }
            CapabilityFamily::VerilogAmsMixed => {
                let trim = SourceFile::new("trim_reg.v", TRIM);
                (
                    format!(
                        ".param vcc=1.8\nvcsn csn 0 1.8\nxtrim csn tr1 tr0 trim_reg\n\
                         rt1 tr1 sum 100k\nrt0 tr0 sum 200k\n.hdl \"{}\"\n",
                        trim.path()
                    ),
                    vec![trim],
                )
            }
            CapabilityFamily::RealNumber => (
                "aobs out rv obs\n.model obs v_to_real (gain=1)\n\
                 arv rv xr drv\n.model drv real_to_v\nrxr xr 0 1k\n"
                    .into(),
                Vec::new(),
            ),
        })
    }

    fn deck(family: CapabilityFamily, extra: &str, cards: &str) -> Netlist {
        parse(&format!(
            "* {} fixture\n{HARNESS}{extra}{cards}.end\n",
            family.tag()
        ))
    }

    /// Run one analysis on the fixture; `None` when the fixture cannot ask
    /// for it (a periodic analysis with no runner here).
    fn run(
        family: CapabilityFamily,
        extra: &str,
        analysis: CapabilityAnalysis,
    ) -> Option<Result<(), SimulationError>> {
        let engine = Engine::default();
        let netlist = deck(family, extra, "");
        let out = || node(&netlist, "out");
        let ok = |result: Result<(), SimulationError>| Some(result);
        match analysis {
            CapabilityAnalysis::Op => ok(engine.run_dc_op(&netlist).map(drop)),
            CapabilityAnalysis::Dc => ok(engine
                .run_dc_sweep(&netlist, "vin", 0.4, 0.6, 0.1)
                .map(drop)),
            CapabilityAnalysis::Ac => ok(engine.run_ac(&netlist, &[1.0e3, 1.0e6]).map(drop)),
            CapabilityAnalysis::Noise => ok(engine
                .run_noise(&netlist, out(), &[1.0e3], 300.15)
                .map(drop)),
            CapabilityAnalysis::Tf => ok(engine
                .run_transfer_function(&netlist, "out", None, false, "vin")
                .map(drop)),
            CapabilityAnalysis::Sp => {
                let card = netlist
                    .analyses
                    .iter()
                    .find(|command| matches!(command, AnalysisCommand::Sp { .. }))
                    .cloned()
                    .expect("the harness carries a .sp card");
                ok(engine
                    .run_sp_with_abort(&netlist, &card, &NoAbort)
                    .map(drop))
            }
            CapabilityAnalysis::Stb => ok(engine
                .run_stb(
                    &netlist,
                    StbConfig::new()
                        .with_sweep(1.0e3, 1.0e6, 4)
                        .with_probe("vprobe"),
                )
                .map(drop)),
            CapabilityAnalysis::Pz => {
                // A current into the summing node: `in` is an ideal source.
                ok(engine
                    .run_pz(&netlist, node(&netlist, "sum"), out())
                    .map(drop))
            }
            CapabilityAnalysis::Sens => ok(engine
                .run_sensitivity_linearized(&netlist, out(), None)
                .map(drop)),
            CapabilityAnalysis::Disto => {
                ok(engine.run_distortion(&netlist, &[1.0e3], None).map(drop))
            }
            CapabilityAnalysis::Tran => ok(engine.run_tran(&netlist, 2.0e-6, 2.0e-8).map(drop)),
            CapabilityAnalysis::TranNoise => {
                let noisy = deck(family, extra, ".tran 20n 2u noisefmax=100meg noiseseed=3\n");
                ok(engine.run_tran(&noisy, 2.0e-6, 2.0e-8).map(drop))
            }
            // Nine harmonics of 1 MHz need samples at least every 13.9 ns.
            CapabilityAnalysis::Fourier => {
                ok(engine.run_tran(&netlist, 4.0e-6, 5.0e-9).and_then(|tran| {
                    let out = tran
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .expect("out is retained");
                    FourierAnalysis::new(FourierConfig::new(1.0e6))
                        .analyze(&tran.time, &tran.voltages[out])
                        .map(drop)
                        .map_err(|error| SimulationError::Circuit(error.to_string()))
                }))
            }
            CapabilityAnalysis::Fft => {
                ok(engine.run_tran(&netlist, 2.0e-6, 2.0e-8).and_then(|tran| {
                    engine
                        .evaluate_transient_fft_results(&netlist, &tran, &NoAbort)
                        .map(drop)
                }))
            }
            CapabilityAnalysis::Sweep => {
                ok(engine.run_step(&netlist, "rlv", &[900.0, 1100.0]).map(drop))
            }
            CapabilityAnalysis::Pss => Some(
                engine
                    .run_pss(&netlist, PssConfig::new(1.0e6).with_harmonics(2))
                    .map(drop),
            ),
            CapabilityAnalysis::Hb => Some(
                engine
                    .run_hb(&netlist, super::HbConfig::new(1.0e6).with_harmonics(2))
                    .map(drop),
            ),
            CapabilityAnalysis::Pac
            | CapabilityAnalysis::Pnoise
            | CapabilityAnalysis::Pxf
            | CapabilityAnalysis::Pstb
            | CapabilityAnalysis::Envelope
            | CapabilityAnalysis::QuasiPeriodic => None,
        }
    }

    #[test]
    fn every_supported_cell_runs_on_its_family_fixture() {
        let mut failures = Vec::new();
        for family in CapabilityFamily::ALL {
            let Some((extra, _files)) = fixture(family) else {
                continue;
            };
            for analysis in CapabilityAnalysis::ALL {
                let cell = capability_cell(family, analysis);
                let Some(outcome) = run(family, &extra, analysis) else {
                    continue;
                };
                match (cell, outcome) {
                    (CellSupport::Supported { .. }, Err(error)) => failures.push(format!(
                        "{} × {} is supported but refused: {error}",
                        family.tag(),
                        analysis.tag()
                    )),
                    (
                        CellSupport::Restricted {
                            gate:
                                InstanceGate::NativeDeviceChecks
                                | InstanceGate::TransientNoiseRenderable,
                            ..
                        },
                        Err(error),
                    ) => failures.push(format!(
                        "{} × {} restricts to devices the fixture holds, but refused: {error}",
                        family.tag(),
                        analysis.tag()
                    )),
                    (CellSupport::Refused { token, .. }, outcome) => match outcome {
                        Err(error)
                            if matches!(error, SimulationError::UnsupportedCapability(_))
                                && capability_token(&error) == token => {}
                        Err(error) => failures.push(format!(
                            "{} × {} must refuse with {token}, got {error}",
                            family.tag(),
                            analysis.tag()
                        )),
                        Ok(()) => failures.push(format!(
                            "{} × {} is refused but ran",
                            family.tag(),
                            analysis.tag()
                        )),
                    },
                    _ => {}
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}

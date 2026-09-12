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
    // Exact HB does not represent the complete Gummel-Poon equations, so a
    // BJT is a device HB understands and declines rather than a bad card.
    let netlist = parse(
        "hb capability\n\
         V1 in 0 SIN(0 0.5 1e6)\n\
         Vcc vcc 0 5\n\
         R1 in b 1k\n\
         Rc vcc c 1k\n\
         Q1 c b 0 npnmod\n\
         .model npnmod NPN IS=1e-16 BF=100\n\
         .end\n",
    );
    let error = Engine::default()
        .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
        .expect_err("HB must refuse a device it has no stamp for");
    let token = capability_token(&error);
    assert!(
        token.starts_with("analysis.hb."),
        "HB refusals must be namespaced under their analysis: {token}"
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
    // A behavioral source with an SDT integrator accumulates accepted state
    // the checkpoint format does not carry, so asking for a checkpoint is a
    // capability gap — not a bad deck, which is why the same deck must still
    // run. The refusal comes before any solver work: the only consumer of a
    // checkpoint is a resume, so there is nothing to be gained by solving to
    // tstop first and refusing then.
    let netlist = parse(
        "checkpoint capability\n\
         V1 in 0 1\n\
         B1 out 0 V={SDT(V(in))}\n\
         R1 out 0 1k\n\
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
            .contains("behavioral-source accepted SDT state is not checkpointed"),
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

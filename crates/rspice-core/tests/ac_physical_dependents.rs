//! Dependent small-signal analyses must use the physical device-stamped operator.
//!
//! These public-API regressions deliberately use impedances above the former
//! blanket AC diagonal's equivalent resistance. A solver may diagnose a truly
//! singular operator, but it must not make these nonsingular circuits easier by
//! changing their transfer, noise, sensitivity, or pole locations.

use rspice_core::constants::K_BOLTZMANN;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const TEMPERATURE_K: f64 = 300.15;
const HIGH_RESISTANCE_OHM: f64 = 1.0e18;
// This resistor value is high enough that a 1e-15 S blanket shunt would
// introduce a readily detectable ~2e-4 relative error in the output PSD.
const HIGH_NOISE_RESISTANCE_OHM: f64 = 1.0e11;

fn physical_engine() -> Engine {
    let mut config = SimulationConfig::default();
    // These tests qualify the authored circuit, not an explicitly requested
    // simulator shunt. Junction gmin is irrelevant to the linear fixtures but
    // is also cleared so the test contract is unambiguous.
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    Engine::try_new(config).expect("physical test configuration is valid")
}

fn node_id(engine: &Engine, netlist: &Netlist, name: &str) -> usize {
    engine
        .build_circuit(netlist)
        .expect("circuit builds")
        .get_node_by_name(name)
        .unwrap_or_else(|| panic!("node {name} exists"))
}

fn assert_relative(actual: f64, expected: f64, relative_tolerance: f64, quantity: &str) {
    assert!(actual.is_finite(), "{quantity} is non-finite: {actual}");
    assert_ne!(expected, 0.0, "relative oracle must be nonzero");
    let relative_error = ((actual - expected) / expected).abs();
    assert!(
        relative_error <= relative_tolerance,
        "{quantity}: actual={actual:.17e}, expected={expected:.17e}, relative error={relative_error:.3e}"
    );
}

#[test]
fn parameter_ac_magnitude_sensitivity_reports_the_null_cusp() {
    let netlist =
        Netlist::parse("AC output null\n.param gain=1\nV1 in 0 AC 1\nE1 out 0 in 0 {gain}\n.end\n")
            .unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    // The API's nominal override is zero even though the authored value is one.
    // Vout=gain, so |Vout| has opposing one-sided derivatives at this point.
    let error = engine
        .run_sensitivity_ac(&netlist, output, "gain", 0.0, &[1.0], None)
        .expect_err("a cusp cannot be reported as zero sensitivity")
        .to_string();
    assert!(error.contains("gain"), "{error}");
    assert!(error.contains("nondifferentiable-magnitude"), "{error}");
}

#[test]
fn parameter_ac_magnitude_sensitivity_respects_the_run_budget() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::AcSensitivityOutput;
    let netlist = Netlist::parse(
        "AC study budget\n.param gain=1\nV1 in 0 AC 1\nE1 out 0 in 0 {gain}\n.end\n",
    )
    .unwrap();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_batch_runs = 2;
    let engine = Engine::try_new(config).unwrap();
    let output = node_id(&engine, &netlist, "out");
    // Since 6e36744f3 ("Propagate root parameter derivatives through linear DC
    // and AC equations") this linear circuit is differentiated by one adjoint
    // solve instead of a replayed finite-difference stencil, so a two-run
    // budget no longer bounds it. That solve is still charged to the budget: a
    // study that has already spent both runs cannot buy the sensitivity run.
    assert!(matches!(
        engine.run_output_sensitivity_ac_with_abort(
            &netlist,
            AcSensitivityOutput::Voltage {
                positive: output,
                negative: None,
            },
            "gain",
            1.0,
            &[1.0],
            None,
            &mut 2,
            &NoAbort,
        ),
        Err(rspice_core::SimulationError::ResourceLimit(_))
    ));
    // Vout=gain*Vin with |Vin|=1, so d|Vout|/dgain is exactly one run's worth.
    assert_relative(
        engine
            .run_sensitivity_ac(&netlist, output, "gain", 1.0, &[1.0], None)
            .unwrap()[0],
        1.0,
        1e-12,
        "unit-gain AC magnitude sensitivity",
    );
}

#[test]
fn sensitivity_refinement_rejects_a_hidden_parameter_kink() {
    let netlist = Netlist::parse(
        "Parameter kink\n.param gain=0\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {abs(gain)}\n.end\n",
    )
    .unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    for result in [
        engine.run_sensitivity(&netlist, output, "gain", 0.0, None),
        engine
            .run_sensitivity_ac(&netlist, output, "gain", 0.0, &[1.0], None)
            .map(|values| values[0]),
    ] {
        let error = result.expect_err("opposing one-sided slopes must not masquerade as zero");
        // Since 97e921d4f ("Retain complex parameter directions through
        // definition-time bindings") the cusp is refused at the binding site
        // instead of by a disagreeing finite-difference stencil, and the
        // replayed override names the parameter in its canonical upper case.
        let message = error.to_string();
        assert!(message.to_ascii_lowercase().contains("gain"), "{message}");
        assert!(message.contains("no two-sided derivative"), "{message}");
    }
}

#[test]
fn sensitivity_expansion_preserves_the_nearby_expression_branch() {
    let deck = |gain: &str| {
        format!(
            "Nearby branch\n.param gain={gain}\nV1 in 0 DC 1 AC 1\n\
             E1 out 0 in 0 {{1+if(abs(gain)<=1e-12,gain,2*gain)}}\n.end\n"
        )
    };
    let netlist = Netlist::parse(&deck("0")).unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    // Within |gain| <= 1e-12 the authored gain is exactly 1+gain, so the local
    // slope is one; the distant branch's two must never replace it. The kink
    // in the discarded condition belongs to a selection that does not switch
    // here, so it leaves that slope defined. A central difference taken inside
    // the nearby branch is the oracle.
    let probe = |gain: &str| {
        engine
            .run_dc_op(&Netlist::parse(&deck(gain)).unwrap())
            .expect("the nearby branch solves")
            .try_voltage_named("out")
            .expect("out is solved")
    };
    assert_relative(
        (probe("1e-13") - probe("-1e-13")) / 2e-13,
        1.0,
        1e-2,
        "nearby branch finite-difference oracle",
    );
    for (derivative, quantity) in [
        (
            engine
                .run_sensitivity(&netlist, output, "gain", 0.0, None)
                .expect("the nearby branch has a local slope"),
            "nearby branch DC sensitivity",
        ),
        (
            engine
                .run_sensitivity_ac(&netlist, output, "gain", 0.0, &[1.0], None)
                .expect("the nearby branch has a local slope")[0],
            "nearby branch AC magnitude sensitivity",
        ),
    ] {
        assert_relative(derivative, 1.0, 1e-12, quantity);
    }
}

#[test]
fn sensitivity_replays_same_card_and_included_parameter_dependencies() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::netlist::{NetlistParseOptions, SealedSourceBundle, SealedSourceEdge};
    let parameters = ".param base=2 derived={3*base}\n";
    let circuit = "V1 in 0 DC 1 AC 1\nE1 out 0 in 0 {base+derived}\n.end\n";
    use std::io::Write;
    struct ParameterFile(std::path::PathBuf);
    impl Drop for ParameterFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let directory = std::env::temp_dir();
    let filename = format!(
        "rspice-parameter-replay-{}-{}.inc",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let path = directory.join(&filename);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    let _cleanup = ParameterFile(path);
    file.write_all(parameters.as_bytes()).unwrap();
    drop(file);
    let included = Netlist::parse_with_path(
        &format!("Included dependencies\n.include {filename}\n{circuit}"),
        &directory.join("main.cir"),
    )
    .unwrap();
    let inline = Netlist::parse(&format!("Same-card dependencies\n{parameters}{circuit}")).unwrap();
    let root = directory.join("sealed.cir");
    let child = directory.join("sealed.inc");
    let source = "Sealed dependencies\n.include sealed.inc\n.end\n".to_owned();
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.clone()),
            (
                child.clone(),
                format!("{parameters}{}", circuit.trim_end_matches(".end\n")),
            ),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "sealed.inc".into(),
            target: child,
        }],
    )
    .unwrap();
    let sealed = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        &source,
        &root,
        bundle,
        NetlistParseOptions::default(),
        &NoAbort,
    )
    .unwrap();
    let engine = physical_engine();
    for netlist in [&inline, &included, &sealed] {
        let output = node_id(&engine, netlist, "out");
        let dc = engine
            .run_sensitivity(netlist, output, "base", 2.0, None)
            .unwrap();
        let ac = engine
            .run_sensitivity_ac(netlist, output, "base", 2.0, &[1.0], None)
            .unwrap();
        assert_relative(dc, 4.0, 1e-8, "dependent parameter DC sensitivity");
        assert_relative(ac[0], 4.0, 1e-8, "dependent parameter AC sensitivity");
    }
}

#[test]
fn sensitivity_retains_the_zero_resistor_flicker_domain_boundary() {
    use rspice_core::analysis::AcSensitivityOutput;
    let netlist = Netlist::parse(
        "Zero flicker parameter\n.param noise=0\nI1 0 out DC 1 AC 1\n\
         R1 out 0 RM 1\n.model RM R(KF={noise})\n.end\n",
    )
    .unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    let probe = AcSensitivityOutput::Voltage {
        positive: output,
        negative: None,
    };
    let filters = ["RM:KF".to_owned()];
    // KF changes resistor noise, so both deterministic transfer derivatives
    // are zero, including at the nonnegative coefficient's domain boundary.
    let dc = engine
        .run_sensitivity_dc_complete(&netlist, probe.clone(), &filters)
        .unwrap();
    let ac = engine
        .run_sensitivity_ac_complete(&netlist, probe, &[1.0], &filters)
        .unwrap();
    assert_eq!(dc.get("RM:KF").unwrap().absolute, 0.0);
    assert_eq!(
        ac.get("RM:KF").unwrap().absolute,
        [rspice_core::Complex64::new(0.0, 0.0)]
    );
    assert_eq!(
        engine
            .run_sensitivity(&netlist, output, "noise", 0.0, None)
            .unwrap(),
        0.0
    );
    assert_eq!(
        engine
            .run_sensitivity_ac(&netlist, output, "noise", 0.0, &[1.0], None)
            .unwrap(),
        [0.0]
    );
}

#[test]
fn sensitivity_refinement_retains_a_level1_mos_domain_boundary() {
    let netlist = Netlist::parse(
        "MOS boundary\nVG gate 0 2\nVD drain 0 2\nVB body 0 -1\n\
         M1 drain gate 0 body NM W=1u L=1u\n.model NM NMOS(LEVEL=1 VTO=1 KP=0 GAMMA=0 PHI=0.6)\n.end\n",
    ).unwrap();
    let result = physical_engine()
        .run_sensitivity_dc_complete(
            &netlist,
            rspice_core::analysis::AcSensitivityOutput::BranchCurrent("VD".into()),
            &["NM:KP".into()],
        )
        .unwrap();
    // The source supplies Id = KP/2 * (Vgs - Vto)^2 at gamma=0.
    let expected = -0.5;
    assert_relative(
        result.get("NM:KP").unwrap().absolute,
        expected,
        1e-9,
        "MOS boundary derivative",
    );
}

#[test]
fn sensitivity_refinement_resolves_zero_mos_body_effect() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::AcSensitivityOutput;
    let source = "MOS body effect\n.param effect=0\nVG gate 0 DC 2 AC 1\nVD drain 0 2\nVB body 0 -1\n\
         M1 drain gate 0 body NM W=1u L=1u\n.model NM NMOS(LEVEL=1 VTO=1 KP=1m GAMMA=0 PHI=0.6)\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    let engine = physical_engine();
    let output = AcSensitivityOutput::BranchCurrent("VD".into());
    let filters = ["NM:GAMMA".to_owned()];
    let dc = engine
        .run_sensitivity_dc_complete(&netlist, output.clone(), &filters)
        .unwrap();
    let ac = engine
        .run_sensitivity_ac_complete(&netlist, output.clone(), &[1.0, 1e9], &filters)
        .unwrap();
    // In saturation Id = KP/2*(Vgs-Vto-gamma*body_shift)^2 and gm = KP*(Vgs-Vth).
    // Vgs-Vto=1, so these two source-current derivatives have the same value.
    let expected = 1e-3 * (1.6_f64.sqrt() - 0.6_f64.sqrt());
    assert_relative(
        dc.get("NM:GAMMA").unwrap().absolute,
        expected,
        1e-5,
        "DC body effect sensitivity",
    );
    for derivative in &ac.get("NM:GAMMA").unwrap().absolute {
        assert_relative(derivative.re, expected, 1e-5, "AC body effect sensitivity");
        assert_eq!(derivative.im, 0.0);
    }
    let parameterized = Netlist::parse(&source.replace("GAMMA=0", "GAMMA={effect}")).unwrap();
    for delta in [None, Some(1e-3)] {
        let mut runs = 0;
        let dc = engine
            .run_output_sensitivity_with_abort(
                &parameterized,
                output.clone(),
                "effect",
                0.0,
                delta,
                &mut runs,
                &NoAbort,
            )
            .unwrap();
        let ac = engine
            .run_output_sensitivity_ac_with_abort(
                &parameterized,
                output.clone(),
                "effect",
                0.0,
                &[1.0, 1e9],
                delta,
                &mut runs,
                &NoAbort,
            )
            .unwrap();
        assert_relative(dc, expected, 1e-5, "authored DC body effect sensitivity");
        for derivative in ac {
            assert_relative(
                derivative.value().unwrap(),
                -expected,
                1e-5,
                "authored AC magnitude body effect sensitivity",
            );
        }
    }
}

#[test]
fn sensitivity_refinement_rejects_unclassified_circuit_failures() {
    let netlist = Netlist::parse(
        "Invalid trial\n.param gain=0\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {1+gain}\n\
         VFAIL conflict 0 1\nRFAIL conflict 0 {if(gain<0,0,1)}\n.end\n",
    )
    .unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    for result in [
        engine.run_sensitivity(&netlist, output, "gain", 0.0, None),
        engine
            .run_sensitivity_ac(&netlist, output, "gain", 0.0, &[1.0], None)
            .map(|values| values[0]),
    ] {
        let error = result.expect_err("an inconsistent circuit does not establish a domain bound");
        let message = error.to_string();
        // RFAIL steps from 0 to 1 exactly at gain=0, so its value has no
        // two-sided derivative there. Since 97e921d4f ("Retain complex
        // parameter directions through definition-time bindings") that is
        // refused at the owning card instead of being inferred from a replayed
        // trial that fails to build; the refinement driver's own reading of an
        // unclassified trial failure is covered by
        // `refinement_does_not_treat_failed_trials_as_parameter_boundaries`.
        assert!(message.contains("RFAIL"), "{message}");
        assert!(
            message.contains("no two-sided parameter derivative"),
            "{message}"
        );
    }
}

#[test]
fn sensitivity_refinement_resolves_curvature_and_smooth_stationary_points() {
    let engine = physical_engine();
    for (expression, nominal, expected) in [
        ("exp(100*gain)", 1.0, 100.0 * 100.0_f64.exp()),
        ("gain*gain", 0.0, 0.0),
    ] {
        let netlist = Netlist::parse(&format!(
            "Parameter curvature\n.param gain={nominal}\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {{{expression}}}\n.end\n"
        )).unwrap();
        let output = node_id(&engine, &netlist, "out");
        for derivative in [
            engine
                .run_sensitivity(&netlist, output, "gain", nominal, None)
                .unwrap(),
            engine
                .run_sensitivity_ac(&netlist, output, "gain", nominal, &[1.0], None)
                .unwrap()[0],
        ] {
            if expected == 0.0 {
                assert_eq!(derivative, 0.0);
            } else {
                assert_relative(derivative, expected, 1e-5, "refined sensitivity");
            }
        }
    }
}

#[test]
fn sensitivity_refinement_uses_finite_stencils_at_the_binary64_boundary() {
    let engine = physical_engine();
    let netlist = Netlist::parse(&format!(
        "Finite parameter boundary\n.param drive={}\nV1 out 0 {{drive}}\n.end\n",
        f64::MAX
    ))
    .unwrap();
    let output = node_id(&engine, &netlist, "out");
    assert_relative(
        engine
            .run_sensitivity(&netlist, output, "drive", f64::MAX, None)
            .unwrap(),
        1.0,
        2e-12,
        "boundary sensitivity",
    );
    let complete = engine
        .run_sensitivity_dc_complete(
            &netlist,
            rspice_core::analysis::AcSensitivityOutput::Voltage {
                positive: output,
                negative: None,
            },
            &["V1".to_owned()],
        )
        .unwrap();
    assert_relative(
        complete.get("V1").unwrap().absolute,
        1.0,
        2e-12,
        "complete boundary sensitivity",
    );
}

#[test]
fn parameter_ac_magnitude_sensitivity_projects_at_the_nominal_point() {
    let netlist = Netlist::parse(
        "AC near an output null\n.param gain=1\nV1 in 0 AC 1 60\nE1 out 0 in 0 {gain}\n.end\n",
    )
    .unwrap();
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");
    for nominal in [-1e-4_f64, 1e-4] {
        // The complex transfer is exactly linear even when the stencil crosses
        // its null. Differencing magnitudes gives +/-0.1 instead of +/-1.
        let derivative = engine
            .run_sensitivity_ac(&netlist, output, "gain", nominal, &[1.0], Some(1e-3))
            .unwrap()[0];
        assert_relative(derivative, nominal.signum(), 2e-12, "magnitude derivative");
    }

    let stationary = Netlist::parse(
        "Smooth zero magnitude\n.param gain=1\nV1 in 0 AC 1\nE1 out 0 in 0 {gain*gain}\n.end\n",
    )
    .unwrap();
    let derivative = engine
        .run_sensitivity_ac(&stationary, output, "gain", 0.0, &[1.0], None)
        .unwrap();
    assert_eq!(derivative, [0.0], "|gain squared| has a zero derivative");
}

#[test]
fn ideal_voltage_short_has_exactly_zero_output_noise() {
    let netlist = Netlist::parse(
        "* an ideal voltage source shorts every parallel noise source\n\
         VCLAMP out 0 DC 0\n\
         RNOISE out 0 1k\n\
         .end\n",
    )
    .expect("noise deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_noise_ports(&netlist, output, None, &[1.0e3], TEMPERATURE_K)
        .expect("noise analysis solves");

    assert_eq!(result.len(), 1);
    // V(out)=0 is an exact ideal-source branch equation. Therefore every
    // parallel current-noise transfer is exactly zero and S_v,out = 0 V²/Hz.
    assert_eq!(result[0].output_noise_density, 0.0);
}

#[test]
fn high_impedance_resistor_noise_is_four_k_t_r() {
    let netlist = Netlist::parse(
        "* open-circuit resistor thermal-noise voltage\n\
         RNOISE out 0 1e11\n\
         .end\n",
    )
    .expect("noise deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_noise_ports(&netlist, output, None, &[1.0e3], TEMPERATURE_K)
        .expect("noise analysis solves");
    let expected_psd = 4.0 * K_BOLTZMANN * TEMPERATURE_K * HIGH_NOISE_RESISTANCE_OHM;

    assert_eq!(result.len(), 1);
    // A resistor has Norton PSD 4kT/R A²/Hz and transfer R V/A, hence
    // S_v,out = (4kT/R) R² = 4kTR V²/Hz.
    assert_relative(
        result[0].output_noise_density,
        expected_psd,
        1.0e-12,
        "high-Z resistor output-noise PSD",
    );
}

#[test]
fn adjoint_sensitivity_preserves_extreme_resistors_and_small_outputs() {
    let engine = physical_engine();
    for (resistance, current) in [
        (1e-200, 1e200),
        (1e-20, 1e20),
        (1.0, 1e-200),
        (1e200, 1e-200),
    ] {
        let netlist = Netlist::parse(&format!(
            "Scaled resistor sensitivity\nI1 0 out {current:e}\nR1 out 0 {resistance:e}\n.end\n"
        ))
        .unwrap();
        let result = engine
            .run_sensitivity_linearized(&netlist, 1, None)
            .unwrap();
        assert_relative(result.output_value, current * resistance, 2e-12, "V=IR");
        let resistor = result
            .get("R1")
            .expect("finite nonzero conductance must remain eligible");
        assert_relative(resistor.absolute, current, 2e-12, "dV/dR=I");
        assert_relative(
            resistor.normalized.value().unwrap(),
            1.0,
            2e-12,
            "normalized resistance derivative",
        );
        let source = result.get("I1").unwrap();
        assert_relative(source.absolute, resistance, 2e-12, "dV/dI=R");
        assert_relative(
            source.normalized.value().unwrap(),
            1.0,
            2e-12,
            "normalized source derivative",
        );
    }
}

#[test]
fn high_impedance_adjoint_sensitivity_matches_closed_form() {
    let netlist = Netlist::parse(
        "* one-node high-Z adjoint oracle\n\
         IBIAS 0 out DC 1e-18\n\
         RLOAD out 0 1e18\n\
         .end\n",
    )
    .expect("sensitivity deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_sensitivity_linearized(&netlist, output, None)
        .expect("linearized adjoint sensitivity solves");

    // V = I R = 1 V, so dV/dR = I and dV/dI = R. The corresponding
    // dimensionless normalized sensitivities are both exactly one.
    assert_relative(result.output_value, 1.0, 1.0e-12, "DC output voltage");
    let resistor = result.get("RLOAD").expect("resistor sensitivity");
    assert_relative(resistor.absolute, 1.0e-18, 1.0e-10, "dV(out)/dRLOAD");
    assert_relative(
        resistor.normalized.value().unwrap(),
        1.0,
        1.0e-10,
        "normalized RLOAD sensitivity",
    );
    let source = result.get("IBIAS").expect("source sensitivity");
    assert_relative(
        source.absolute,
        HIGH_RESISTANCE_OHM,
        1.0e-10,
        "dV(out)/dIBIAS",
    );
    assert_relative(
        source.normalized.value().unwrap(),
        1.0,
        1.0e-10,
        "normalized IBIAS sensitivity",
    );
}

#[test]
fn high_impedance_transfer_function_is_unperturbed() {
    let netlist = Netlist::parse(
        "* one-node high-Z transimpedance\n\
         IIN 0 out DC 0\n\
         RLOAD out 0 1e18\n\
         .end\n",
    )
    .expect("transfer-function deck parses");
    let engine = physical_engine();

    let result = engine
        .run_transfer_function(&netlist, "out", None, false, "iin")
        .expect("transfer-function analysis solves");

    // A unit input current develops Vout = Iin R. The same lone resistor is
    // both the input impedance seen by IIN and the output impedance seen by
    // the independent output test current.
    assert_relative(
        result.gain,
        HIGH_RESISTANCE_OHM,
        1.0e-12,
        "transimpedance gain",
    );
    assert_relative(
        result.input_impedance,
        HIGH_RESISTANCE_OHM,
        1.0e-12,
        "input impedance",
    );
    assert_relative(
        result.output_impedance,
        HIGH_RESISTANCE_OHM,
        1.0e-12,
        "output impedance",
    );
}

#[test]
fn high_impedance_rc_pole_remains_at_minus_one_over_rc() {
    let netlist = Netlist::parse(
        "* one-state high-Z RC pole\n\
         IIN 0 out DC 0 AC 1\n\
         RLOAD out 0 1e18\n\
         CLOAD out 0 1\n\
         .end\n",
    )
    .expect("pole-zero deck parses");
    let engine = physical_engine();
    let output = node_id(&engine, &netlist, "out");

    let result = engine
        .run_pz(&netlist, output, output)
        .expect("pole-zero analysis solves");

    assert_eq!(result.poles.len(), 1, "poles={:?}", result.poles);
    let pole = result.poles[0];
    // For Y(s)=1/R+sC, the natural root is s=-1/(RC)=-1e-18 rad/s.
    assert_relative(pole.re, -1.0e-18, 1.0e-10, "RC pole real part");
    assert_eq!(pole.im, 0.0, "the first-order passive pole must be real");
}

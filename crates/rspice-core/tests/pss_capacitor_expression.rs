//! Expression-capacitor history through shooting and transient continuation.
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::PssConfig;
use rspice_core::config::ExpressionDialect;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{Netlist, NetlistParseOptions};
use rspice_core::numerics::integration::IntegrationMethod;

fn parse(deck: &str) -> Netlist {
    Netlist::parse_with_options(
        deck,
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap()
}

fn config() -> PssConfig {
    let mut config = PssConfig::new(1e3)
        .with_points_per_period(256)
        .with_tstab_periods(0)
        .with_tolerance(1e-8);
    config.abstol = 1e-12;
    config
}

#[test]
fn expression_capacitor_pss_matches_charge_law_and_continues() {
    // Manufactured solution: out=0.5+0.2*sin(wt); C(out,ctrl)*d(out)/dt
    // plus the resistor current is the independently authored forcing.
    for (method, ic) in [
        (IntegrationMethod::Trapezoidal, ""),
        (IntegrationMethod::Gear2, " IC=0.5"),
    ] {
        let follower = if ic.is_empty() {
            ""
        } else {
            "Ecopy mirror 0 out 0 2\nRcopy mirror 0 1k\n"
        };
        let deck = parse(&format!(
            "Nonlinear capacitor\nVctrl ctrl 0 SIN(0.2 0.1 1k 0 0 90)\n\
            Bdrive 0 out I={{(0.5+0.2*sin(2*pi*1k*time))/1k + 100n*(1+0.1*(0.5+0.2*sin(2*pi*1k*time))+0.2*V(ctrl))*2*pi*1k*0.2*cos(2*pi*1k*time)}}\n\
            R1 out 0 1k\nC1 out 0 C={{100n*(1+0.1*V(out)+0.2*V(ctrl))}}{ic}\n{follower}.end\n"
        ));
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce)
        });
        let (analysis, state) = engine
            .run_pss_with_continuation_state(&deck, config())
            .unwrap();
        let index = analysis
            .result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        for (&time, &value) in analysis
            .result
            .time
            .iter()
            .zip(&analysis.result.waveforms[index].values)
        {
            let truth = 0.5 + 0.2 * (std::f64::consts::TAU * 1e3 * time).sin();
            assert!(
                (value - truth).abs() < 3e-4,
                "{method:?} {time:e}: {value} vs {truth}"
            );
        }
        let (continued, _) = engine
            .run_tran_from_pss_state(&deck, &state, 2e-4, 2e-6)
            .unwrap();
        for (&time, &value) in continued
            .time
            .iter()
            .zip(continued.try_voltage_waveform_named("out").unwrap())
        {
            let truth = 0.5 + 0.2 * (std::f64::consts::TAU * 1e3 * time).sin();
            assert!(
                (value - truth).abs() < 4e-4,
                "continued {method:?} {time:e}: {value} vs {truth}"
            );
        }
    }
}

#[test]
fn expression_capacitor_pss_resolves_authored_time_corners() {
    let deck = parse(
        "Capacitor clock\nVdrive out 0 SIN(0 .2 1k)\n\
        C1 out 0 C={100n*(1+.1*table(mod(time,1m),0,0,123u,1,137u,0,1m,0))+0*V(out)}\n.end\n",
    );
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let analysis = engine
        .run_pss(&deck, config().with_tstab_periods(1))
        .unwrap();
    for corner in [123e-6_f64, 137e-6] {
        assert!(
            analysis
                .result
                .time
                .iter()
                .any(|time| (*time - corner).abs() < 8. * f64::EPSILON * corner),
            "missing capacitor event {corner:e}"
        );
    }
}

#[test]
fn expression_capacitor_pss_owns_integral_coordinates_and_rebases_history() {
    let deck = parse(
        "Capacitor integral\nVdrive out 0 SIN(0 0.2 1k)\n\
        C1 out 0 C={100n*(1+0.1*1k*SDT(V(out)))}\n.end\n",
    );
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let point = engine
        .run_pss_operating_point_with_abort(&deck, config(), &NoAbort)
        .unwrap();
    assert_eq!(point.shooting_state_basis(), &["C:C1:sdt:0".to_owned()]);
    assert_eq!(point.analysis().monodromy.len(), 1);
    assert!((point.analysis().monodromy[0][0] - 1.).abs() < 1e-5);
    let (_, state) = engine
        .run_pss_with_continuation_state(&deck, config())
        .unwrap();
    let (continued, _) = engine
        .run_tran_from_pss_state(&deck, &state, 2e-4, 1e-6)
        .unwrap();
    let current = continued
        .try_branch_current_waveform_named("Vdrive")
        .unwrap();
    for (&time, &current) in continued.time.iter().zip(current).skip(2) {
        let angle = std::f64::consts::TAU * 1e3 * time;
        let integral = 0.2 * (1. - angle.cos()) / (std::f64::consts::TAU * 1e3);
        let truth =
            -100e-9 * (1. + 100. * integral) * 0.2 * std::f64::consts::TAU * 1e3 * angle.cos();
        assert!(
            (current - truth).abs() < 1e-6,
            "{time:e}: {current:e} vs {truth:e}"
        );
    }
    for expression in ["100n*(1+time)", "100n*(1+SDT(1e-12))"] {
        let invalid = parse(&format!(
            "Invalid capacitor clock\nV1 out 0 SIN(0 0.2 1k)\nC1 out 0 C={{{expression}}}\n.end\n"
        ));
        assert!(engine.run_pss(&invalid, config()).is_err(), "{expression}");
    }
}

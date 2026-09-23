//! Expression charge has a restart origin; physical voltage/current and SDT
//! memory must continue from the carrier even when mean current is nonzero.
use rspice_core::analysis::{HbConfig, HbContinuationLimitation};
use rspice_core::config::{ExpressionDialect, SpiceDialect};
use rspice_core::engine::{
    Engine, HbEnvelopeStateGuarantee, SimulationConfig, TransientCheckpoint,
};
use rspice_core::netlist::NetlistParseOptions;
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn envelope_expression_capacitors_keep_coupled_state_integrals_and_charge_drift() {
    for (method, ic) in [
        (IntegrationMethod::Trapezoidal, ""),
        (IntegrationMethod::Gear2, " IC=.9"),
    ] {
        let deck = Netlist::parse_with_options(&format!(
            "Expression capacitor origin\n\
            Vctrl ctrl 0 SIN(0 .2 1k 0 0 90)\n\
            Vmod mod 0 PWL(0 .1 .1m .1 .2m .2 1m .2)\n\
            Bdrive 0 out I={{(.3+.2*sin(2*pi*1k*time))/1k+100n*(1+.3*(.3+.2*sin(2*pi*1k*time))^2+.2*(.1+.15*cos(2*pi*1k*time))^2+.02*(1-cos(2*pi*1k*time))+V(mod))*.2*2*pi*1k*cos(2*pi*1k*time)}}\n\
            Bdrive2 0 out2 I={{(.1+.15*cos(2*pi*1k*time))/1k-200n*(1+.2*(.1+.15*cos(2*pi*1k*time))^2+.3*(.3+.2*sin(2*pi*1k*time))^2+.1*cos(2*pi*1k*time))*.15*2*pi*1k*sin(2*pi*1k*time)}}\n\
            R1 out 0 1k\nR2 out2 0 1k\n\
            C1 out 0 C={{100n*(1+.3*V(out)^2+.2*V(out2)^2+.1*(2*pi*1k)^2*SDT(SDT(V(ctrl)))+V(mod))}}{ic}\n\
            C2 out2 0 C={{200n*(1+.2*V(out2)^2+.3*V(out)^2+.1*cos(2*pi*1k*time))}}{ic}\n\
            Bmemory memory 0 V={{(2*pi*1k)^2*SDT(SDT(V(ctrl)))}}\nRm memory 0 1k\n\
            .options hbint tahb=0\n.end\n"
        ), NetlistParseOptions { expression_dialect: ExpressionDialect::Xyce, ..Default::default() }).unwrap();
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            // Resolve currents through the very short post-breakpoint steps;
            // default voltage convergence permits larger differentiated error.
            transient_nonlinear_reltol: Some(1e-8),
            transient_nonlinear_rhstol: Some(1e-11),
            convergence_config: rspice_core::config::ConvergenceConfig::default()
                .with_voltage_tolerances(1e-8, 1e-10)
                .with_residual_reltol(1e-8),
            ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce)
        });
        let mut config = HbConfig::new(1e3)
            .with_harmonics(5)
            .with_collocation_points(33);
        config.tolerance = 1e-9;
        config.abstol = 1e-13;
        let result = engine
            .run_envelope_with_abort(&deck, config, &["Vmod".into()], 0.25e-3, 0.5e-6, &NoAbort)
            .unwrap();
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExpressionChargeRestartV1
        );
        // Display-only phase state continues to advertise its missing history;
        // only the authenticated initializer may establish the restart epoch.
        assert!(
            result
                .carrier()
                .continuation_limitations
                .contains(&HbContinuationLimitation::CapacitorChargeHistoryNotRetained)
        );
        let spectrum = result
            .carrier()
            .reactive_spectra
            .iter()
            .find(|row| row.device_name.eq_ignore_ascii_case("C1"))
            .unwrap();
        assert!(
            spectrum.current_coefficients[0].re.abs() > 1e-8,
            "test must exercise non-periodic accumulated charge"
        );
        let origin = result.continued_transient();
        assert!((origin.try_voltage_waveform_named("out").unwrap()[0] - 0.3).abs() < 1e-8);
        let initial_current =
            100e-9 * (1.0 + 0.3 * 0.3 * 0.3 + 0.2 * 0.25 * 0.25 + 0.1) * 0.2 * TAU * 1e3;
        assert!(
            (origin.try_branch_current_waveform_named("C1").unwrap()[0] - initial_current).abs()
                < 1e-10
        );
        let checkpoint =
            TransientCheckpoint::from_text(&result.final_checkpoint().to_text()).unwrap();
        assert!(checkpoint.capability().is_resumable());
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 0.5e-3, 0.5e-6)
            .unwrap();
        for transient in [origin, &resumed] {
            let out = transient.try_voltage_waveform_named("out").unwrap();
            let out2 = transient.try_voltage_waveform_named("out2").unwrap();
            let memory = transient.try_voltage_waveform_named("memory").unwrap();
            let current1 = transient.try_branch_current_waveform_named("C1").unwrap();
            let current2 = transient.try_branch_current_waveform_named("C2").unwrap();
            for (index, &time) in transient.time.iter().enumerate() {
                let angle = TAU * 1e3 * time;
                let v1 = 0.3 + 0.2 * angle.sin();
                let v2 = 0.1 + 0.15 * angle.cos();
                let z = 0.2 * (1.0 - angle.cos());
                let modulation = 0.1 + ((time - 0.1e-3) * 1e3).clamp(0.0, 0.1);
                let c1 = 100e-9 * (1.0 + 0.3 * v1 * v1 + 0.2 * v2 * v2 + 0.1 * z + modulation);
                let c2 = 200e-9 * (1.0 + 0.2 * v2 * v2 + 0.3 * v1 * v1 + 0.1 * angle.cos());
                for (name, actual, expected) in [
                    ("out", out[index], v1),
                    ("out2", out2[index], v2),
                    ("memory", memory[index], z),
                ] {
                    assert!(
                        (actual - expected).abs() < 2e-5,
                        "{method:?} {name} at {time}: {actual} vs {expected}"
                    );
                }
                for (name, actual, expected) in [
                    ("C1", current1[index], c1 * 0.2 * TAU * 1e3 * angle.cos()),
                    ("C2", current2[index], -c2 * 0.15 * TAU * 1e3 * angle.sin()),
                ] {
                    assert!(
                        (actual - expected).abs() < 3e-8,
                        "{method:?} {name} at {time}: {actual} vs {expected}"
                    );
                }
            }
        }
        let tag =
            rspice_core::execution::result_document::EnvelopeGuaranteeTag::from(result.guarantee());
        assert_eq!(
            serde_json::to_string(&tag).unwrap(),
            "\"expression-charge-restart-v1\""
        );
    }
}

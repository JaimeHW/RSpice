use super::*;

/// A constant high-injection bias has a tiny reverse diffusion capacitance
/// alongside a much larger forward diffusion charge. None of the intrinsic
/// terminals is voltage-clamped: the authored drives include actual private
/// RB/RC/RE drops, calculated from the independent DC transport equations.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_exact_phase_retains_a_small_reverse_charge_mode_at_private_dc_bias() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let state = REGIMES[2].state(0.0, dialect);
        let ic = state.forward_transport - state.reverse / state.charge_factor - state.reverse / BR;
        let ib = state.forward / BF + state.reverse / BR;
        let expected = [ic, ib, -ic - ib];
        let resistance = [5.0, 25.0, 2.0];
        let intrinsic = [state.collector, state.base, 0.0];
        for polarity in [1.0, -1.0] {
            let kind = if polarity > 0.0 { "NPN" } else { "PNP" };
            let drive: [f64; 3] =
                std::array::from_fn(|i| polarity * (intrinsic[i] + resistance[i] * expected[i]));
            let source = Netlist::parse(&format!(
                "Private GP DC charge modes\nVC c 0 {:.17e}\nVB b 0 {:.17e}\nVE e 0 {:.17e}\nQ1 c b e qm\n.model qm {kind} IS={IS} BF={BF} BR={BR} IKF={IKF} IKR={IKR} VAF={VAF} VAR={VAR} TF={TF} PTF=57.29577951308232 TNOM=27 RB=25 RBM=25 RC=5 RE=2 {}\n.options TEMP=27 GMIN=0 RELTOL=1e-7 ABSTOL=1e-15 VNTOL=1e-10 METHOD=TRAP\n.end\n",
                drive[0], drive[1], drive[2], ChargeLaw::BIASED.parameters()
            )).unwrap();
            let mut config = SimulationConfig {
                gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                integration_method: IntegrationMethod::Trapezoidal,
                ..SimulationConfig::default().with_spice_dialect(dialect)
            };
            config.convergence_config.gmin_target = 0.0;
            let result = Engine::new(config)
                .run_tran(&source, 2.5e-9, 4e-12)
                .unwrap_or_else(|error| panic!("{dialect:?}/{kind}: {error}"));
            assert_eq!(result.time.last(), Some(&2.5e-9));
            for (i, name) in ["vc", "vb", "ve"].into_iter().enumerate() {
                for (index, current) in result
                    .try_branch_current_waveform_named(name)
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    assert!(
                        (-polarity * current - expected[i]).abs()
                            < 1e-12 + 1e-5 * expected[i].abs(),
                        "{dialect:?}/{kind}/{name}/t={}: {current:e} != {:e}",
                        result.time[index],
                        -polarity * expected[i]
                    );
                }
            }
        }
    }
}

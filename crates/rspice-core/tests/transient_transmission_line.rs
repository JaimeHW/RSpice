use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientCheckpoint};
use rspice_core::netlist::Netlist;

#[test]
fn loaded_lossless_line_preserves_dc_bias_through_startup_and_resume() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for (near_reference, far_reference, polarity) in
            [(0.0, 0.0, 1.0), (3.0, -2.0, 1.0), (3.0, -2.0, -1.0)]
        {
            let (near_port, far_port) = if polarity > 0.0 {
                ("near nr", "far fr")
            } else {
                ("nr near", "fr far")
            };
            let deck = Netlist::parse(&format!(
                "biased delay line\nVNR nr 0 {near_reference}\nVFR fr 0 {far_reference}\n\
                 VDRIVE drive nr {}\nRS drive near 25\nT1 {near_port} {far_port} Z0=50 TD=1u\n\
                 RL far fr 75\n.save all\n.end\n",
                polarity * 2.0,
            ))
            .unwrap();
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let dc = engine.run_dc_op(&deck).unwrap();
            // The existing DC fallback has a 1 mOhm series resistance. Its
            // small bias error is allowed; a Z0-sized startup droop is not.
            for (node, reference) in [("near", near_reference), ("far", far_reference)] {
                let voltage = polarity * (dc.try_voltage_named(node).unwrap() - reference);
                assert!(
                    (voltage - 1.5).abs() < 2e-5,
                    "{dialect:?}: {node} DC={voltage}"
                );
            }
            let (first, checkpoint) = engine
                .run_tran_checkpointed(&deck, 0.75e-6, 0.125e-6)
                .unwrap();
            let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&deck, &checkpoint, 3.25e-6, 0.125e-6)
                .unwrap();
            for result in [&first, &resumed] {
                for (node, reference) in [("near", near_reference), ("far", far_reference)] {
                    for (&time, &voltage) in result
                        .time
                        .iter()
                        .zip(result.try_voltage_waveform_named(node).unwrap())
                    {
                        let voltage = polarity * (voltage - reference);
                        assert!(
                            (voltage - 1.5).abs() < 2e-5,
                            "{dialect:?}, refs={near_reference}/{far_reference}, polarity={polarity}: {node} at {time:e} = {voltage}"
                        );
                    }
                }
            }
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

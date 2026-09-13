//! Public delay lifecycle trace retained across the shared-runtime extraction.

use rspice_veriloga::vm::DelayBuffer;

#[test]
fn transport_delay_public_state_and_value_trace() {
    for bounded in [false, true] {
        let maximum = bounded.then_some(2.5);
        let mut buffer = DelayBuffer::new(2);
        let mut time = 0.0;
        let mut trace = Vec::new();
        for step in 0..48 {
            let value = (time * 0.0625_f64).exp() - 0.75;
            let delay = if bounded {
                0.125 * (1 + step % 25) as f64
            } else {
                1.25
            };
            let accepted = buffer.clone();
            let first = buffer.eval(time, value, delay, maximum).unwrap();
            // A discarded Newton probe may replace a candidate but cannot
            // change any of the accepted samples used by the next probe.
            buffer.eval(time, value + 17.0, delay, maximum).unwrap();
            let repeated = buffer.eval(time, value, delay, maximum).unwrap();
            let mut restored = accepted;
            let after_rollback = restored.eval(time, value, delay, maximum).unwrap();
            assert_eq!(first.to_bits(), repeated.to_bits());
            assert_eq!(first.to_bits(), after_rollback.to_bits());
            buffer.commit().unwrap();
            restored.commit().unwrap();
            assert_eq!(format!("{buffer:?}"), format!("{restored:?}"));
            trace.push((
                time.to_bits(),
                value.to_bits(),
                first.to_bits(),
                format!("{buffer:?}"),
            ));
            time += 0.125 * (1 + step % 7) as f64;
        }
        println!(
            "DELAY_TRACE={}",
            serde_json::json!({
                "bounded": bounded,
                "size": std::mem::size_of::<DelayBuffer>(),
                "alignment": std::mem::align_of::<DelayBuffer>(),
                "samples": trace,
            })
        );
    }
}

#[test]
fn transport_delay_preserves_an_analytic_ramp_across_short_and_long_steps() {
    for delay in [0.125, 1.25, 8.0] {
        let mut buffer = DelayBuffer::new(1);
        let mut time = 0.0;
        for step in 0..32 {
            let output = buffer.eval(time, 2.0 * time + 0.5, delay, None).unwrap();
            let expected = 2.0 * (time - delay).max(0.0) + 0.5;
            // The analytic dyadic ramp is exact; evaluating the interpolant
            // also rounds its fractional weight and multiply/add operations.
            assert!(
                output.to_bits().abs_diff(expected.to_bits()) <= 2,
                "delay={delay} time={time}: ramp {output} versus {expected}"
            );
            buffer.commit().unwrap();
            time += 0.125 * (1 + step % 7) as f64;
        }
    }
}

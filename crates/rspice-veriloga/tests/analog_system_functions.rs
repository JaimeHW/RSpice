//! Value-returning system functions evaluate the same on every route, once per
//! accepted point, and a rejected point leaves no draw behind.

use rspice_veriloga::device::VerilogAEvaluationSnapshot;
use rspice_veriloga_runtime::random;
mod support;
use support::DeviceFixture;

fn fixture(declarations: &str, body: &str) -> DeviceFixture {
    DeviceFixture::compile(&format!(
        r#"`include "disciplines.vams"
module draws(p,n);
inout p,n; electrical p,n;
{declarations}
analog begin {body} I(p,n) <+ V(p,n); end
endmodule"#
    ))
}

#[test]
fn a_seed_variable_draws_the_ieee_stream_once_per_accepted_point() {
    let fixture = fixture(
        "integer seed, r; real g; analog initial seed = 42;",
        "r = $random(seed); g = $rdist_normal(seed, 1.0, 0.5);",
    );
    let mut device = fixture.device("DRAWS", &[1, 0]);
    let mut seed = 42;
    for _ in 0..3 {
        let r = random::random(&mut seed);
        let g = random::rdist_normal(&mut seed, 1.0, 0.5);
        // Newton evaluates the point again and again: every evaluation
        // starts from the accepted seed and draws the same values.
        for voltage in [0.5, 0.25, 0.125] {
            device.update_voltages(&[voltage]);
            device.try_evaluate().unwrap();
            fixture.observe(&mut device);
            device.try_compute_jacobian().unwrap();
            assert_eq!(device.variable("r"), Some(f64::from(r)));
            assert_eq!(device.variable("g"), Some(g));
            assert_eq!(device.variable("seed"), Some(f64::from(seed)));
        }
        device.try_advance_state().unwrap();
    }
}

#[test]
fn a_rejected_point_leaves_no_draw_behind() {
    let fixture = fixture(
        "integer seed, r; real u; analog initial seed = 7;",
        "r = $random; u = $arandom(11) + $rdist_uniform(seed, 0, 1);",
    );
    let mut device = fixture.device("DRAWS", &[1, 0]);
    let mut stream = 0;
    let mut internal = 11;
    let mut seed = 7;
    let mut expected = || {
        let r = random::random(&mut stream);
        let u =
            f64::from(random::random(&mut internal)) + random::rdist_uniform(&mut seed, 0.0, 1.0);
        (Some(f64::from(r)), Some(u))
    };
    let first = expected();
    let second = expected();
    let mut snapshot = VerilogAEvaluationSnapshot::default();
    device.try_evaluate().unwrap();
    assert_eq!((device.variable("r"), device.variable("u")), first);
    device.try_advance_state().unwrap();
    device.capture_evaluation_state(&mut snapshot);
    device.try_evaluate().unwrap();
    assert_eq!((device.variable("r"), device.variable("u")), second);
    // The trial is rejected: the retry draws the same values again.
    device.restore_evaluation_state(&snapshot).unwrap();
    device.try_evaluate().unwrap();
    assert_eq!((device.variable("r"), device.variable("u")), second);
    device.try_advance_state().unwrap();
    device.try_evaluate().unwrap();
    assert_eq!((device.variable("r"), device.variable("u")), expected());
}

/// A checkpoint carries the accepted seeds, the stream and the internal
/// seeds alike, through its portable words: a resumed instance draws exactly
/// what the uninterrupted one draws next.
#[test]
fn a_checkpoint_resumes_every_seed_bit_for_bit() {
    use rspice_veriloga::device::VerilogADeviceCheckpoint;
    let fixture = fixture(
        "integer seed, r; real u; analog initial seed = 7;",
        "r = $random; u = $arandom(11) + $rdist_normal(seed, 0, 1);",
    );
    let mut device = fixture.device("DRAWS", &[1, 0]);
    for _ in 0..2 {
        device.try_evaluate().unwrap();
        device.try_advance_state().unwrap();
    }
    let checkpoint = device.checkpoint_state().unwrap();
    let checkpoint = VerilogADeviceCheckpoint::from_words(
        checkpoint.instance_name.clone(),
        checkpoint.model_name.clone(),
        checkpoint.source_digest.clone(),
        checkpoint.shape_identity.clone(),
        &checkpoint.to_words(),
    )
    .unwrap();
    let mut resumed = fixture.device("DRAWS", &[1, 0]);
    resumed.validate_checkpoint_state(&checkpoint).unwrap();
    resumed.apply_validated_checkpoint_state(&checkpoint);
    resumed.restore_checkpoint_files(&checkpoint).unwrap();
    for _ in 0..3 {
        device.try_evaluate().unwrap();
        resumed.try_evaluate().unwrap();
        for name in ["r", "u", "seed"] {
            assert_eq!(
                resumed.variable(name).map(f64::to_bits),
                device.variable(name).map(f64::to_bits),
                "{name}"
            );
        }
        device.try_advance_state().unwrap();
        resumed.try_advance_state().unwrap();
    }
}

/// A call inside an event's body runs only when the event fires: an
/// `@(initial_step)` draw happens at the first point and never again, so it
/// takes one value from the stream and every later point's draw follows.
#[test]
fn a_call_in_an_event_body_runs_only_when_the_event_fires() {
    let fixture = fixture(
        "integer seed, r, a, b, count; analog initial begin seed = 42; count = 0; end",
        "@(initial_step) begin r = $random(seed); a = $random; count = count + 1; end \
         b = $random;",
    );
    let mut device = fixture.device("DRAWS", &[1, 0]);
    device.set_analysis_type(2);
    let mut seed = 42;
    let first = random::random(&mut seed);
    let mut stream = 0;
    let a = random::random(&mut stream);
    for point in 0..3 {
        let b = random::random(&mut stream);
        device.set_analysis_step(point == 0, false);
        device.try_evaluate().unwrap();
        assert_eq!(device.variable("r"), Some(f64::from(first)));
        assert_eq!(device.variable("seed"), Some(f64::from(seed)));
        assert_eq!(device.variable("count"), Some(1.0));
        assert_eq!(device.variable("a"), Some(f64::from(a)));
        assert_eq!(device.variable("b"), Some(f64::from(b)), "point {point}");
        device.try_advance_state().unwrap();
    }
}

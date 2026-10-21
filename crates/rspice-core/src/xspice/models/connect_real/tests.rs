use super::*;

const LAW: RampLaw = RampLaw {
    rise: 2.0e-9,
    fall: 4.0e-9,
};

#[test]
fn a_ramp_takes_its_full_edge_time_and_holds_the_left_limit() {
    let up = Ramp {
        start: 1.0e-9,
        from: 0.0,
        target: 1.0,
    };
    assert_eq!(up.at(1.0e-9, LAW), 0.0);
    assert!((up.at(2.0e-9, LAW) - 0.5).abs() < 1e-12);
    // At the end the stepper is asked to land on, exactly the target.
    let end = up.end(LAW).expect("a ramp ends");
    assert!((end - 3.0e-9).abs() < 1e-24);
    assert_eq!(up.at(end, LAW), 1.0);
    let down = up.toward(2.0e-9, 0.0, LAW);
    assert!((down.from - 0.5).abs() < 1e-12);
    assert!((down.at(4.0e-9, LAW) - 0.25).abs() < 1e-12);
    assert!((down.end(LAW).expect("a ramp ends") - 6.0e-9).abs() < 1e-24);
    assert_eq!(Ramp::settled(0.0, 2.0).end(LAW), None);
}

#[test]
fn the_sample_reference_advances_in_whole_steps() {
    let law = SampleLaw {
        vdelta: 0.01,
        ttol: 0.0,
    };
    assert_eq!(law.crossed(0.0, 0.009), None);
    assert!((law.crossed(0.0, 0.0105).unwrap() - 0.01).abs() < 1e-15);
    assert!((law.crossed(0.0, 0.0345).unwrap() - 0.03).abs() < 1e-15);
    assert!((law.crossed(0.03, 0.0195).unwrap() - 0.02).abs() < 1e-15);
    let every = SampleLaw {
        vdelta: 0.0,
        ttol: 0.0,
    };
    assert_eq!(every.crossed(1.0, 1.0), None);
    assert_eq!(every.crossed(1.0, 1.25), Some(1.25));
}

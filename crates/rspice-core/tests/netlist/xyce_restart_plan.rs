use rspice_core::engine::{TransientCheckpointEncoding, XyceRestartJobPlan, XyceRestartPlanError};
use rspice_core::netlist::XyceRestartInterval;
use rspice_core::{ResourceKind, ResourceLimitError};

#[test]
fn interval_transitions_apply_at_their_authored_times() {
    let intervals = [
        XyceRestartInterval {
            time: 10.0,
            interval: 4.0,
        },
        XyceRestartInterval {
            time: 17.0,
            interval: 1.0,
        },
    ];
    let plan = XyceRestartJobPlan::new("state", 3.0, &intervals, 19.0, None, 32)
        .expect("valid restart plan");

    assert_eq!(
        plan.nominal_times(),
        &[0.0, 3.0, 6.0, 9.0, 10.0, 14.0, 17.0, 18.0, 19.0]
    );
    assert_eq!(plan.encoding(), TransientCheckpointEncoding::Packed);
    assert_eq!(plan.logical_name(0.0).as_deref(), Some("state0"));
    assert_eq!(plan.logical_name(10.0).as_deref(), Some("state10"));
    assert_eq!(plan.logical_name(19.0).as_deref(), Some("state19"));
    assert_eq!(plan.logical_name(12.0), None);
}

#[test]
fn pack_false_selects_the_canonical_unpacked_encoding() {
    let plan = XyceRestartJobPlan::new("state", 1.0, &[], 1.0, Some(false), 2)
        .expect("valid unpacked restart plan");
    assert_eq!(plan.encoding(), TransientCheckpointEncoding::Unpacked);
}

#[test]
fn schedule_growth_returns_a_typed_analysis_point_limit() {
    let error = XyceRestartJobPlan::new("state", 1.0, &[], 4.0, None, 4)
        .expect_err("five requested checkpoints exceed a four-point limit");
    assert_eq!(
        error.to_string(),
        ".OPTIONS RESTART schedule exceeds the configured analysis-point limit of 4"
    );
    assert_eq!(
        error,
        XyceRestartPlanError::AnalysisPointLimit {
            source: ResourceLimitError {
                resource: ResourceKind::AnalysisPoints,
                requested: 5,
                limit: 4,
            },
        }
    );
}

#[test]
fn six_digit_filename_collisions_fail_before_simulation() {
    let intervals = [XyceRestartInterval {
        time: 1.0,
        interval: 1.0e-7,
    }];
    let error = XyceRestartJobPlan::new("state", 1.0, &intervals, 1.000_000_1, None, 8)
        .expect_err("adjacent nominal times have one Xyce filename");
    assert_eq!(
        error.to_string(),
        ".OPTIONS RESTART filename precision maps more than one checkpoint to 'state1'; choose a wider checkpoint interval or shorter stop time"
    );
    assert_eq!(
        error,
        XyceRestartPlanError::LogicalNameCollision {
            logical_name: "state1".to_string(),
        }
    );
}

#[test]
fn public_plan_names_match_xyce_defaultfloat_boundaries() {
    let oracle = [
        (0x3e35_798e_e230_8c3a, "5e-09"),
        (0x3f1a_36e2_0f35_445d, "9.99999e-05"),
        (0x3f1a_36e2_0f35_445e, "0.0001"),
        (0x3f1a_36e2_eb1c_432d, "0.0001"),
        (0x3f50_624d_4981_4abb, "0.001"),
        (0x40f8_69ff_5c28_f5c3, "100000"),
        (0x412e_847e_ffff_ffff, "999999"),
        (0x412e_847f_0000_0000, "1e+06"),
        (0x412e_8480_0000_0000, "1e+06"),
        (0x54b2_49ad_2594_c37d, "1e+100"),
        (0x2b2b_ff2e_e48e_0530, "1e-100"),
        (0x0000_0000_0000_0001, "4.94066e-324"),
        (0x7fef_ffff_ffff_ffff, "1.79769e+308"),
    ];

    for (bits, expected_suffix) in oracle {
        let time = f64::from_bits(bits);
        let plan = XyceRestartJobPlan::new("state", time, &[], time, None, 2)
            .expect("one-checkpoint boundary plan");
        let expected = format!("state{expected_suffix}");
        assert_eq!(
            plan.logical_name(time).as_deref(),
            Some(expected.as_str()),
            "Xyce defaultfloat mismatch for {time:.17e} (0x{bits:016X})"
        );
    }
}

#[test]
fn invalid_numeric_contracts_fail_closed() {
    assert_eq!(
        XyceRestartJobPlan::new("state", 0.0, &[], 1.0, None, 2),
        Err(XyceRestartPlanError::InvalidInitialInterval)
    );
    assert_eq!(
        XyceRestartJobPlan::new("state", 1.0, &[], f64::INFINITY, None, 2),
        Err(XyceRestartPlanError::InvalidStopTime)
    );

    let bad_time = [XyceRestartInterval {
        time: f64::NAN,
        interval: 1.0,
    }];
    assert_eq!(
        XyceRestartJobPlan::new("state", 1.0, &bad_time, 2.0, None, 4),
        Err(XyceRestartPlanError::InvalidTransitionTime { index: 0 })
    );

    let bad_interval = [XyceRestartInterval {
        time: 1.0,
        interval: 0.0,
    }];
    assert_eq!(
        XyceRestartJobPlan::new("state", 1.0, &bad_interval, 2.0, None, 4),
        Err(XyceRestartPlanError::InvalidTransitionInterval { index: 0 })
    );
}

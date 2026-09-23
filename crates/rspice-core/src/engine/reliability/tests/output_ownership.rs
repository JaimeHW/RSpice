use super::*;

#[test]
fn reliability_output_ownership_preserves_stress_mesh_and_lifetime_results() {
    let mut request = request();
    request.study.mission.truncate(1);
    request.study.mission[0].duration_s = 1.5e-6;
    request.study.transient_stress = Some(ReliabilityTransientWindow {
        step_s: 1e-7,
        start_s: 0.25e-6,
        stop_s: 1.75e-6,
        max_step_s: Some(5e-8),
        use_initial_conditions: false,
    });
    request.target_years = vec![3e-6 / SECONDS_PER_AGING_YEAR];
    let deck = DECK.replace("VG gate 0 0", "VG gate 0 PWL(0 0 1u -1 2u 0)");
    let run = |options: &str| {
        let netlist =
            Netlist::parse(&deck.replace(".end", &format!(".save V(drain)\n{options}\n.end")))
                .unwrap();
        let original_options = netlist.options.clone();
        let result = Engine::new(Default::default())
            .run_reliability_with_abort(&netlist, &request, &NoAbort)
            .unwrap();
        result
            .validate_retained_payload_with_abort(&Default::default(), &NoAbort)
            .unwrap();
        assert_eq!(
            netlist.options.output_interval_schedule,
            original_options.output_interval_schedule
        );
        assert_eq!(
            netlist.options.output_time_points,
            original_options.output_time_points
        );
        assert_eq!(
            netlist.options.output_snapshots,
            original_options.output_snapshots
        );
        result
    };
    let baseline = run("");
    assert!(baseline.stress.phases[0].time_s.len() > 10);
    for options in [
        ".options OUTPUT INITIAL_INTERVAL=500n",
        // Shared deck output times can belong to another transient analysis.
        // They need not lie in this representative stress window.
        ".options OUTPUT OUTPUTTIMEPOINTS=173n,600n,3u",
        ".options OUTPUT SNAPSHOTS=1",
    ] {
        assert_eq!(run(options), baseline, "{options}");
    }
}

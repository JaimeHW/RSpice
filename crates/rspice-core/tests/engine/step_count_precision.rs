use rspice_core::Netlist;
#[cfg(target_pointer_width = "64")]
use rspice_core::netlist::AnalysisCommand;
#[cfg(target_pointer_width = "64")]
use rspice_core::netlist::StepSweep;

#[cfg(target_pointer_width = "64")]
#[test]
fn logarithmic_step_counts_keep_exact_integer_literals() {
    for (kind, count) in [("DEC", 9_007_199_254_740_993usize), ("OCT", usize::MAX)] {
        let netlist = Netlist::parse(&format!(
            "* exact count\n.step {kind} param r 1 10 {count}\n.end\n"
        ))
        .unwrap();
        let AnalysisCommand::Step(step) = &netlist.analyses[0] else {
            panic!("missing step");
        };
        let actual = match step.sweep {
            StepSweep::Decade {
                points_per_decade, ..
            } => points_per_decade,
            StepSweep::Octave {
                points_per_octave, ..
            } => points_per_octave,
            _ => panic!("unexpected sweep"),
        };
        assert_eq!(actual, count);
    }
}

#[test]
fn invalid_logarithmic_counts_identify_the_card_and_field() {
    for count in ["1e309", "0", "-1", "1.5", "18446744073709551616"] {
        let error = Netlist::parse(&format!(
            "* invalid count\n.step DEC param r 1 10 {count}\n.end\n"
        ))
        .unwrap_err();
        assert!(
            error.to_string().contains(".STEP DEC points per interval"),
            "{count}: {error}"
        );
    }
}

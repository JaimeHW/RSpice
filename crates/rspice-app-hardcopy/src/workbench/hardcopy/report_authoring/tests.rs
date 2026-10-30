//! Report authoring gates every mutation on exact authority.
//!
//! Opening the surface mutates nothing; a plan creates the outline once;
//! page settings and order commit revision-checked transactions and fail
//! closed at the document boundaries. The layout cases hold each internal seam
//! to exactly one pane, which is what keeps a resize from splitting a block
//! across two.

use super::*;
use rspice_results_ui::report::page_marker;

#[test]
fn report_page_markers_remain_semantically_stable_after_reordering() {
    assert_eq!(page_marker(0, "Executive summary"), "1");
    assert_eq!(page_marker(5, "Executive summary"), "1");
    assert_eq!(page_marker(0, "Run manifests"), "A");
    assert_eq!(page_marker(7, "Custom appendix"), "+");
}

#[test]
fn report_joint_yield_requires_aligned_all_spec_sample_trails() {
    fn result(measurement: &str, trail: Vec<bool>) -> rspice_results::yield_analysis::YieldResult {
        let total_runs = trail.len();
        let pass_count = trail.iter().filter(|passed| **passed).count();
        rspice_results::yield_analysis::YieldResult {
            spec: rspice_results::yield_analysis::YieldSpec::lower(measurement, 0.0, "V"),
            total_runs,
            pass_count,
            fail_count: total_runs - pass_count,
            yield_percent: pass_count as f64 / total_runs as f64 * 100.0,
            stats: rspice_results::yield_analysis::DistributionStats::default(),
            trail,
            samples: vec![0.0; total_runs],
        }
    }

    let aligned = [
        result("V(out)", vec![true, true, false]),
        result("I(V1)", vec![true, false, true]),
    ];
    assert_eq!(
        ReportJointYield::from_results(&aligned),
        Some(ReportJointYield {
            passing: 1,
            total: 3,
        })
    );

    let misaligned = [
        result("V(out)", vec![true, false]),
        result("I(V1)", vec![true]),
    ];
    assert_eq!(ReportJointYield::from_results(&misaligned), None);
}

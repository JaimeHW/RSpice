//! Suite execution, summary printing, and aggregate statistics.

use super::*;

impl TestRunner {
    /// Run all tests in a subdirectory
    pub fn run_suite(&self, subdir: &str) -> Vec<TestResult> {
        let tests = self.discover_tests(subdir);
        tests.iter().map(|p| self.run_test(p)).collect()
    }

    /// Print a summary of test results
    pub fn print_summary(results: &[TestResult]) {
        let total = results.len();
        let passed = results
            .iter()
            .filter(|r| r.passed && r.error.is_none())
            .count();
        let skipped = results
            .iter()
            .filter(|r| r.error.as_ref().is_some_and(|e| e.starts_with("SKIPPED")))
            .count();
        let expected_unsupported = results
            .iter()
            .filter(|r| is_expected_unsupported_result(r))
            .count();
        let failed = total - passed - skipped - expected_unsupported;

        println!("\n════════════════════════════════════════════════════════════");
        println!(
            "  Test Summary: {} total | {} passed | {} failed | {} skipped | {} expected unsupported",
            total, passed, failed, skipped, expected_unsupported
        );
        println!("════════════════════════════════════════════════════════════\n");

        // Print failures first
        for result in results
            .iter()
            .filter(|r| !r.passed && r.error.as_ref().is_none_or(|e| !e.starts_with("SKIPPED")))
        {
            let analysis = result.analysis_type.as_deref().unwrap_or("?");
            println!("  ✗ {} [{}] - {:?}", result.name, analysis, result.error);
            for mismatch in result.mismatches.iter().take(3) {
                println!(
                    "      mismatch: {} @ x={} expected={} actual={} rel_err={:.3e}",
                    mismatch.node,
                    mismatch.x_value,
                    mismatch.expected,
                    mismatch.actual,
                    mismatch.relative_error
                );
            }
        }

        // Then skipped
        if skipped > 0 {
            println!();
            for result in results
                .iter()
                .filter(|r| r.error.as_ref().is_some_and(|e| e.starts_with("SKIPPED")))
            {
                if let Some(ref err) = result.error {
                    println!("  ⊘ {} - {}", result.name, err);
                }
            }
        }

        if expected_unsupported > 0 {
            println!();
            for result in results.iter().filter(|r| is_expected_unsupported_result(r)) {
                if let Some(ref err) = result.error {
                    println!("  ! {} - {}", result.name, err);
                }
            }
        }
    }

    /// Generate aggregate statistics
    pub fn statistics(results: &[TestResult]) -> TestStatistics {
        let total = results.len();
        let passed = results
            .iter()
            .filter(|r| r.passed && r.error.is_none())
            .count();
        let skipped = results
            .iter()
            .filter(|r| r.error.as_ref().is_some_and(|e| e.starts_with("SKIPPED")))
            .count();
        let expected_unsupported = results
            .iter()
            .filter(|r| is_expected_unsupported_result(r))
            .count();
        let failed = total - passed - skipped - expected_unsupported;
        let total_time_ms: u128 = results.iter().map(|r| r.duration_ms).sum();

        TestStatistics {
            total,
            passed,
            failed,
            skipped,
            expected_unsupported,
            total_time_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statistics_counts_expected_unsupported_separately() {
        let results = vec![
            TestResult {
                name: "digital_source".to_string(),
                passed: true,
                error: Some(expected_unsupported_message(
                    "d_source requires event kernel",
                )),
                mismatches: Vec::new(),
                duration_ms: 1,
                analysis_type: Some("EXPECTED_UNSUPPORTED".to_string()),
            },
            TestResult {
                name: "rc".to_string(),
                passed: true,
                error: None,
                mismatches: Vec::new(),
                duration_ms: 1,
                analysis_type: Some("TRAN".to_string()),
            },
        ];

        let stats = TestRunner::statistics(&results);

        assert_eq!(stats.total, 2);
        assert_eq!(stats.passed, 1);
        assert_eq!(stats.failed, 0);
        assert_eq!(stats.skipped, 0);
        assert_eq!(stats.expected_unsupported, 1);
        assert!((stats.pass_rate() - 50.0).abs() < 0.01);
    }
}

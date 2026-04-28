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
        let failed = total - passed - skipped;

        println!(
            "\nâ•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•"
        );
        println!(
            "  Test Summary: {} total | {} passed | {} failed | {} skipped",
            total, passed, failed, skipped
        );
        println!(
            "â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•\n"
        );

        // Print failures first
        for result in results
            .iter()
            .filter(|r| !r.passed && r.error.as_ref().is_none_or(|e| !e.starts_with("SKIPPED")))
        {
            let analysis = result.analysis_type.as_deref().unwrap_or("?");
            println!("  âœ— {} [{}] - {:?}", result.name, analysis, result.error);
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
                    println!("  âŠ˜ {} - {}", result.name, err);
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
        let failed = total - passed - skipped;
        let total_time_ms: u128 = results.iter().map(|r| r.duration_ms).sum();

        TestStatistics {
            total,
            passed,
            failed,
            skipped,
            total_time_ms,
        }
    }
}

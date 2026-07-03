use rspice_core::testing::{TestResult, TestRunner, TestRunnerConfig};
use rspice_core::xspice::CodeModelRegistry;
use rspice_core::xspice::conformance::{
    ConformanceIssue, ConformanceSeverity, IfSpecConformancePolicy, XspiceParitySkippedDeck,
    audit_ngspice_ifspec_event_port_types, audit_ngspice_ifspec_tree,
    audit_ngspice_xspice_examples, materialize_ngspice_xspice_parity_suite,
    materialize_ngspice_xspice_smoke_suite,
};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_MAX_ISSUES: usize = 200;

fn main() {
    if let Err(err) = run() {
        eprintln!("xspice conformance audit error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse(std::env::args().skip(1))?;
    let registry = CodeModelRegistry::with_builtins();
    let icm_root = args
        .ngspice_source_root
        .join("src")
        .join("xspice")
        .join("icm");
    let policy = IfSpecConformancePolicy::ngspice46();
    let report =
        audit_ngspice_ifspec_tree(&icm_root, &registry, &policy).map_err(|err| err.to_string())?;
    let event_port_report =
        audit_ngspice_ifspec_event_port_types(&icm_root, &policy).map_err(|err| err.to_string())?;

    print_ifspec_report(&args, &report.issues, report.checked_models);
    print_event_port_report(&args, &event_port_report);
    let mut failed = report.has_errors() || event_port_report.has_unsupported_event_ports();

    if args.audit_examples || args.run_examples {
        let corpus = audit_ngspice_xspice_examples(&args.ngspice_source_root)
            .map_err(|err| err.to_string())?;
        let needs_adjudication = corpus.needs_adjudication();
        println!();
        println!("XSPICE example corpus audit");
        println!("example decks: {}", corpus.decks.len());
        println!("runnable: {}", corpus.runnable_count());
        println!("scripted control: {}", corpus.scripted_control_count());
        println!("reusable fragments: {}", corpus.reusable_fragment_count());
        println!("expected invalid: {}", corpus.expected_invalid_count());
        println!("third-party excluded: {}", corpus.excluded_count());
        println!("needs adjudication: {}", needs_adjudication.len());

        for deck in needs_adjudication.iter().take(args.max_issues) {
            println!("- {}: {:?}", deck.relative_path.display(), deck.disposition);
        }
        if needs_adjudication.len() > args.max_issues {
            println!(
                "... {} additional deck(s) omitted; rerun with --max-issues {} or higher",
                needs_adjudication.len() - args.max_issues,
                needs_adjudication.len()
            );
        }
        failed |= !needs_adjudication.is_empty();
    }

    if args.run_examples {
        let suite_root = args
            .example_suite_root
            .clone()
            .unwrap_or_else(unique_example_suite_root);
        let smoke_suite =
            materialize_ngspice_xspice_smoke_suite(&args.ngspice_source_root, &suite_root)
                .map_err(|err| err.to_string())?;
        let run_count = args
            .example_limit
            .unwrap_or(smoke_suite.runnable_decks.len())
            .min(smoke_suite.runnable_decks.len());
        let runner = TestRunner::new(&smoke_suite.root, example_runner_config(&args));
        let mut results = Vec::new();
        for (relative, deck) in smoke_suite
            .runnable_relative_decks
            .iter()
            .zip(smoke_suite.runnable_decks.iter())
            .take(run_count)
        {
            results.push((relative.clone(), runner.run_test(deck)));
        }
        print_example_run_report(&results, smoke_suite.runnable_decks.len());
        failed |= results.iter().any(|(_, result)| !result.passed);

        if !args.keep_example_suite && args.example_suite_root.is_none() {
            let _ = std::fs::remove_dir_all(&smoke_suite.root);
        } else {
            println!("materialized example suite: {}", smoke_suite.root.display());
        }
    }

    if args.run_example_parity {
        let suite_root = args
            .parity_suite_root
            .clone()
            .unwrap_or_else(unique_example_parity_suite_root);
        let parity_suite =
            materialize_ngspice_xspice_parity_suite(&args.ngspice_source_root, &suite_root)
                .map_err(|err| err.to_string())?;
        let run_count = args
            .example_limit
            .unwrap_or(parity_suite.parity_decks.len())
            .min(parity_suite.parity_decks.len());
        let runner = TestRunner::new(&parity_suite.root, example_parity_runner_config(&args)?);
        let mut results = Vec::new();
        for (relative, deck) in parity_suite
            .parity_relative_decks
            .iter()
            .zip(parity_suite.parity_decks.iter())
            .take(run_count)
        {
            results.push((relative.clone(), runner.run_test(deck)));
        }
        print_example_parity_report(
            &results,
            parity_suite.parity_decks.len(),
            &parity_suite.skipped_decks,
            args.max_issues,
        );
        failed |= results.iter().any(|(_, result)| !result.passed);

        if !args.keep_example_suite && args.parity_suite_root.is_none() {
            let _ = std::fs::remove_dir_all(&parity_suite.root);
        } else {
            println!(
                "materialized parity example suite: {}",
                parity_suite.root.display()
            );
        }
    }

    if failed {
        Err("XSPICE conformance audit failed".to_string())
    } else {
        Ok(())
    }
}

#[derive(Debug)]
struct Args {
    ngspice_source_root: PathBuf,
    max_issues: usize,
    audit_examples: bool,
    run_examples: bool,
    example_limit: Option<usize>,
    example_max_time_ms: u128,
    example_suite_root: Option<PathBuf>,
    parity_suite_root: Option<PathBuf>,
    run_example_parity: bool,
    ngspice_exe: Option<PathBuf>,
    keep_example_suite: bool,
}

impl Args {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut ngspice_source_root = None;
        let mut max_issues = DEFAULT_MAX_ISSUES;
        let mut audit_examples = false;
        let mut run_examples = false;
        let mut example_limit = None;
        let mut example_max_time_ms = TestRunnerConfig::default().max_time_per_test_ms;
        let mut example_suite_root = None;
        let mut parity_suite_root = None;
        let mut run_example_parity = false;
        let mut ngspice_exe = None;
        let mut keep_example_suite = false;

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "--ngspice-source-root" => {
                    ngspice_source_root = Some(next_path(&mut args, &flag)?);
                }
                "--max-issues" => {
                    max_issues = next_parse(&mut args, &flag)?;
                }
                "--examples" => {
                    audit_examples = true;
                }
                "--run-examples" => {
                    run_examples = true;
                }
                "--run-example-parity" => {
                    audit_examples = true;
                    run_example_parity = true;
                }
                "--ngspice-exe" => {
                    ngspice_exe = Some(next_path(&mut args, &flag)?);
                }
                "--example-limit" => {
                    example_limit = Some(next_parse(&mut args, &flag)?);
                }
                "--example-max-time-ms" => {
                    example_max_time_ms = next_parse(&mut args, &flag)?;
                }
                "--example-suite-root" => {
                    example_suite_root = Some(next_path(&mut args, &flag)?);
                }
                "--parity-suite-root" => {
                    parity_suite_root = Some(next_path(&mut args, &flag)?);
                }
                "--keep-example-suite" => {
                    keep_example_suite = true;
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => return Err(format!("unknown argument '{flag}'")),
            }
        }

        let ngspice_source_root = ngspice_source_root
            .or_else(|| std::env::var_os("NGSPICE_SOURCE_ROOT").map(PathBuf::from))
            .ok_or_else(|| "missing --ngspice-source-root or NGSPICE_SOURCE_ROOT".to_string())?;

        Ok(Self {
            ngspice_source_root,
            max_issues,
            audit_examples,
            run_examples,
            example_limit,
            example_max_time_ms,
            example_suite_root,
            parity_suite_root,
            run_example_parity,
            ngspice_exe,
            keep_example_suite,
        })
    }
}

fn print_help() {
    println!(
        "Usage: xspice_ifspec_audit --ngspice-source-root <path> [--examples] [--run-examples] [--run-example-parity] [--max-issues <n>]\n\
         Compares RSpice builtin XSPICE metadata against ngspice src/xspice/icm/**/ifspec.ifs.\n\
         With --examples, also adjudicates ngspice examples/xspice decks without running third-party cosimulation.\n\
         With --run-examples, materializes runnable examples into a smoke suite and executes them through RSpice.\n\
         With --run-example-parity, instruments runnable examples for .print all numeric comparison and uses --ngspice-exe or NGSPICE_EXE as the oracle."
    );
}

fn next_path<I>(args: &mut I, flag: &str) -> Result<PathBuf, String>
where
    I: Iterator<Item = String>,
{
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("missing value for {flag}"))
}

fn next_parse<I, T>(args: &mut I, flag: &str) -> Result<T, String>
where
    I: Iterator<Item = String>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    args.next()
        .ok_or_else(|| format!("missing value for {flag}"))?
        .parse::<T>()
        .map_err(|err| format!("invalid value for {flag}: {err}"))
}

fn print_ifspec_report(args: &Args, issues: &[ConformanceIssue], checked_models: usize) {
    let error_count = issues
        .iter()
        .filter(|issue| issue.severity == ConformanceSeverity::Error)
        .count();
    let warning_count = issues.len().saturating_sub(error_count);

    println!("XSPICE ifspec audit");
    println!(
        "ngspice source root: {}",
        args.ngspice_source_root.display()
    );
    println!("official models checked: {checked_models}");
    println!("metadata errors: {error_count}");
    println!("metadata warnings: {warning_count}");

    for issue in issues.iter().take(args.max_issues) {
        println!("- {}", format_issue(issue));
    }
    if issues.len() > args.max_issues {
        println!(
            "... {} additional issue(s) omitted; rerun with --max-issues {} or higher",
            issues.len() - args.max_issues,
            issues.len()
        );
    }
}

fn print_event_port_report(
    args: &Args,
    report: &rspice_core::xspice::conformance::XspiceEventPortCatalogReport,
) {
    println!();
    println!("XSPICE event port type audit");
    println!("official models checked: {}", report.checked_models);
    println!(
        "unsupported event port type references: {}",
        report.unsupported_event_ports.len()
    );
    for port in report.unsupported_event_ports.iter().take(args.max_issues) {
        println!(
            "- {}.{} {} {:?} ({})",
            port.model,
            port.port,
            port.source,
            port.port_type,
            port.path.display()
        );
    }
    if report.unsupported_event_ports.len() > args.max_issues {
        println!(
            "... {} additional unsupported event port(s) omitted; rerun with --max-issues {} or higher",
            report.unsupported_event_ports.len() - args.max_issues,
            report.unsupported_event_ports.len()
        );
    }
}

fn format_issue(issue: &ConformanceIssue) -> String {
    match &issue.path {
        Some(path) => format!(
            "{:?} {}: {} ({})",
            issue.severity,
            issue.model,
            issue.message,
            path.display()
        ),
        None => format!("{:?} {}: {}", issue.severity, issue.model, issue.message),
    }
}

fn unique_example_suite_root() -> PathBuf {
    unique_suite_root("rspice-xspice-examples")
}

fn unique_example_parity_suite_root() -> PathBuf {
    unique_suite_root("rspice-xspice-parity")
}

fn unique_suite_root(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
}

fn example_runner_config(args: &Args) -> TestRunnerConfig {
    TestRunnerConfig {
        max_time_per_test_ms: args.example_max_time_ms.max(1),
        ..TestRunnerConfig::default()
    }
}

fn example_parity_runner_config(args: &Args) -> Result<TestRunnerConfig, String> {
    let ngspice_exe = args
        .ngspice_exe
        .clone()
        .or_else(|| std::env::var_os("NGSPICE_EXE").map(PathBuf::from))
        .ok_or_else(|| "--run-example-parity requires --ngspice-exe or NGSPICE_EXE".to_string())?;

    Ok(TestRunnerConfig {
        max_time_per_test_ms: args.example_max_time_ms.max(1),
        live_ngspice_source_root: Some(args.ngspice_source_root.clone()),
        live_ngspice_exe: Some(ngspice_exe),
        live_ngspice_timeout_ms: Some(args.example_max_time_ms.max(1)),
        live_ngspice_direct_decks: true,
        ..TestRunnerConfig::default()
    })
}

fn print_example_run_report(results: &[(PathBuf, TestResult)], total_runnable: usize) {
    let passed = results.iter().filter(|(_, result)| result.passed).count();
    let failed = results.len().saturating_sub(passed);
    println!();
    println!("XSPICE runnable example smoke run");
    println!("runnable decks: {total_runnable}");
    println!("executed: {}", results.len());
    println!("passed: {passed}");
    println!("failed: {failed}");

    for (relative, result) in results.iter().filter(|(_, result)| !result.passed) {
        println!(
            "- {}: {}",
            relative.display(),
            result
                .error
                .as_deref()
                .unwrap_or("failed without diagnostic")
        );
    }
}

fn print_example_parity_report(
    results: &[(PathBuf, TestResult)],
    total_parity: usize,
    skipped: &[XspiceParitySkippedDeck],
    max_issues: usize,
) {
    let passed = results.iter().filter(|(_, result)| result.passed).count();
    let failed = results.len().saturating_sub(passed);
    println!();
    println!("XSPICE runnable example numeric parity run");
    println!("parity-capable decks: {total_parity}");
    println!("skipped non-comparable runnable decks: {}", skipped.len());
    println!("executed: {}", results.len());
    println!("passed: {passed}");
    println!("failed: {failed}");

    for skipped in skipped.iter().take(max_issues) {
        println!(
            "- skipped {}: {}",
            skipped.relative_path.display(),
            skipped.reason
        );
    }
    if skipped.len() > max_issues {
        println!(
            "... {} additional skipped deck(s) omitted; rerun with --max-issues {} or higher",
            skipped.len() - max_issues,
            skipped.len()
        );
    }

    for (relative, result) in results.iter().filter(|(_, result)| !result.passed) {
        println!(
            "- {}: {}",
            relative.display(),
            result
                .error
                .as_deref()
                .unwrap_or("failed without diagnostic")
        );
        for mismatch in result.mismatches.iter().take(3) {
            println!(
                "  {} at {}: expected {}, actual {}, relerr {}",
                mismatch.node,
                mismatch.x_value,
                mismatch.expected,
                mismatch.actual,
                mismatch.relative_error
            );
        }
    }
}

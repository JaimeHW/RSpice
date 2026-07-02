use rspice_core::testing::{TestResult, TestRunner, TestRunnerConfig};
use rspice_core::xspice::CodeModelRegistry;
use rspice_core::xspice::conformance::{
    ConformanceIssue, ConformanceSeverity, IfSpecConformancePolicy, audit_ngspice_ifspec_tree,
    audit_ngspice_xspice_examples, materialize_ngspice_xspice_smoke_suite,
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
    let report = audit_ngspice_ifspec_tree(
        &args
            .ngspice_source_root
            .join("src")
            .join("xspice")
            .join("icm"),
        &registry,
        &IfSpecConformancePolicy::ngspice46(),
    )
    .map_err(|err| err.to_string())?;

    print_ifspec_report(&args, &report.issues, report.checked_models);
    let mut failed = report.has_errors();

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
                "--example-limit" => {
                    example_limit = Some(next_parse(&mut args, &flag)?);
                }
                "--example-max-time-ms" => {
                    example_max_time_ms = next_parse(&mut args, &flag)?;
                }
                "--example-suite-root" => {
                    example_suite_root = Some(next_path(&mut args, &flag)?);
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
            keep_example_suite,
        })
    }
}

fn print_help() {
    println!(
        "Usage: xspice_ifspec_audit --ngspice-source-root <path> [--examples] [--run-examples] [--max-issues <n>]\n\
         Compares RSpice builtin XSPICE metadata against ngspice src/xspice/icm/**/ifspec.ifs.\n\
         With --examples, also adjudicates ngspice examples/xspice decks without running third-party cosimulation.\n\
         With --run-examples, materializes runnable examples into a smoke suite and executes them through RSpice."
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
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "rspice-xspice-examples-{}-{nanos}",
        std::process::id()
    ))
}

fn example_runner_config(args: &Args) -> TestRunnerConfig {
    TestRunnerConfig {
        max_time_per_test_ms: args.example_max_time_ms.max(1),
        ..TestRunnerConfig::default()
    }
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

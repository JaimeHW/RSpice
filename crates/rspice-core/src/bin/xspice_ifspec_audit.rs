use rspice_core::xspice::CodeModelRegistry;
use rspice_core::xspice::conformance::{
    ConformanceIssue, ConformanceSeverity, IfSpecConformancePolicy, audit_ngspice_ifspec_tree,
    audit_ngspice_xspice_examples,
};
use std::path::PathBuf;

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

    if args.audit_examples {
        let corpus = audit_ngspice_xspice_examples(&args.ngspice_source_root)
            .map_err(|err| err.to_string())?;
        let needs_adjudication = corpus.needs_adjudication();
        println!();
        println!("XSPICE example corpus audit");
        println!("example decks: {}", corpus.decks.len());
        println!("runnable: {}", corpus.runnable_count());
        println!("scripted control: {}", corpus.scripted_control_count());
        println!("reusable fragments: {}", corpus.reusable_fragment_count());
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
}

impl Args {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut ngspice_source_root = None;
        let mut max_issues = DEFAULT_MAX_ISSUES;
        let mut audit_examples = false;

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
        })
    }
}

fn print_help() {
    println!(
        "Usage: xspice_ifspec_audit --ngspice-source-root <path> [--examples] [--max-issues <n>]\n\
         Compares RSpice builtin XSPICE metadata against ngspice src/xspice/icm/**/ifspec.ifs.\n\
         With --examples, also adjudicates ngspice examples/xspice decks without running third-party cosimulation."
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

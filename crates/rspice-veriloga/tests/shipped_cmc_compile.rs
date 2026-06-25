//! Full compilation gate for every shipped Verilog-A model source.
//!
//! Run explicitly with:
//! `cargo test -p rspice-veriloga --test shipped_cmc_compile shipped_veriloga_models_compile_end_to_end -- --ignored --nocapture`
//!
//! To focus on one package, file, or module:
//! `RSPICE_CMC_COMPILE_FILTER=mvsg cargo test -p rspice-veriloga --test shipped_cmc_compile shipped_veriloga_models_compile_end_to_end -- --ignored --nocapture`
//!
//! To tune qualification runtime:
//! `RSPICE_CMC_COMPILE_JOBS=4 RSPICE_CMC_COMPILE_TIMEOUT_SECS=25 cargo test -p rspice-veriloga --test shipped_cmc_compile shipped_veriloga_models_compile_end_to_end -- --ignored --nocapture`

use rspice_veriloga::preprocessor::MacroDef;
use rspice_veriloga::{
    CodeGenerator, CompiledModel, CompilerOptions, Lexer, Parser, Preprocessor, SemanticAnalyzer,
    SourceMap, VerilogACompiler,
};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const CHILD_ENV: &str = "RSPICE_CMC_COMPILE_CHILD";
const ROOT_ENV: &str = "RSPICE_CMC_COMPILE_ROOT";
const SOURCE_ENV: &str = "RSPICE_CMC_COMPILE_SOURCE";
const MODULE_ENV: &str = "RSPICE_CMC_COMPILE_MODULE";
const INCLUDES_ENV: &str = "RSPICE_CMC_COMPILE_INCLUDES";
const TIMEOUT_ENV: &str = "RSPICE_CMC_COMPILE_TIMEOUT_SECS";
const FILTER_ENV: &str = "RSPICE_CMC_COMPILE_FILTER";
const TIMINGS_ENV: &str = "RSPICE_CMC_COMPILE_TIMINGS";
const JOBS_ENV: &str = "RSPICE_CMC_COMPILE_JOBS";
const DEFAULT_MODULE_TIMEOUT_SECS: u64 = 20;
const DEFAULT_MAX_JOBS: usize = 4;

#[derive(Debug)]
struct ModelSource {
    package: String,
    path: PathBuf,
    modules: Vec<String>,
    include_dirs: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
struct CompileJob {
    source_index: usize,
    module: String,
}

struct CompletedCompileJob {
    source_index: usize,
    module: String,
    result: CompileChildResult,
}

#[test]
#[ignore = "full shipped CMC compile qualification; run explicitly while the compiler frontier is still moving"]
fn shipped_veriloga_models_compile_end_to_end() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");

    let mut sources =
        discover_model_sources(&root).expect("discover shipped Verilog-A model sources");
    if let Ok(filter) = env::var(FILTER_ENV)
        && !filter.trim().is_empty()
    {
        sources = filter_sources(sources, &root, &filter);
        assert!(
            !sources.is_empty(),
            "{FILTER_ENV}={filter:?} did not match any shipped Verilog-A model source or module"
        );
    } else {
        assert!(
            sources.len() >= 20,
            "expected the shipped CMC corpus to be discovered, got {} module-bearing sources",
            sources.len()
        );
    }

    let timeout = module_timeout();
    let jobs = compile_jobs(&sources);
    assert!(
        !jobs.is_empty(),
        "expected at least one shipped Verilog-A module to compile"
    );
    let worker_count = compile_worker_count(jobs.len());
    eprintln!(
        "compiling {} shipped Verilog-A module(s) with {} worker(s), {:.2}s timeout per module",
        jobs.len(),
        worker_count,
        timeout.as_secs_f64()
    );

    let root = Arc::new(root);
    let sources = Arc::new(sources);
    let jobs = Arc::new(jobs);
    let next_job = Arc::new(AtomicUsize::new(0));
    let (sender, receiver) = mpsc::channel();
    let mut workers = Vec::new();

    for _ in 0..worker_count {
        let sender = sender.clone();
        let root = Arc::clone(&root);
        let sources = Arc::clone(&sources);
        let jobs = Arc::clone(&jobs);
        let next_job = Arc::clone(&next_job);
        workers.push(thread::spawn(move || {
            loop {
                let job_index = next_job.fetch_add(1, Ordering::Relaxed);
                if job_index >= jobs.len() {
                    break;
                }

                let job = &jobs[job_index];
                let source = &sources[job.source_index];
                let result = run_compile_child(&root, source, &job.module, timeout);
                if sender
                    .send(CompletedCompileJob {
                        source_index: job.source_index,
                        module: job.module.clone(),
                        result,
                    })
                    .is_err()
                {
                    break;
                }
            }
        }));
    }
    drop(sender);

    let mut failures = Vec::new();
    let mut compiled = 0usize;
    for completed in receiver {
        let source = &sources[completed.source_index];
        match completed.result {
            CompileChildResult::Compiled { elapsed, output } => {
                compiled += 1;
                if env::var_os(TIMINGS_ENV).is_some() && !output.trim().is_empty() {
                    eprint!("{output}");
                }
                eprintln!(
                    "compiled {} :: {} in {:.2}s",
                    source
                        .path
                        .strip_prefix(root.as_ref())
                        .unwrap_or(&source.path)
                        .display(),
                    completed.module,
                    elapsed.as_secs_f64()
                );
            }
            CompileChildResult::Failed {
                status,
                elapsed,
                output,
            } => failures.push(format!(
                "{} :: {} :: {} failed with {status} after {:.2}s\n{}",
                source.package,
                source
                    .path
                    .strip_prefix(root.as_ref())
                    .unwrap_or(&source.path)
                    .display(),
                completed.module,
                elapsed.as_secs_f64(),
                trim_output(&output)
            )),
            CompileChildResult::TimedOut { elapsed, output } => {
                failures.push(format!(
                    "{} :: {} :: {} timed out after {:.2}s (limit: {:.2}s)\n{}",
                    source.package,
                    source
                        .path
                        .strip_prefix(root.as_ref())
                        .unwrap_or(&source.path)
                        .display(),
                    completed.module,
                    elapsed.as_secs_f64(),
                    timeout.as_secs_f64(),
                    trim_output(&output)
                ));
            }
        }
    }

    for worker in workers {
        worker
            .join()
            .expect("shipped Verilog-A compile worker must not panic");
    }

    assert!(
        failures.is_empty(),
        "failed to compile {} shipped Verilog-A module(s); compiled {} successfully:\n{}",
        failures.len(),
        compiled,
        failures.join("\n")
    );
    assert!(
        compiled >= sources.len(),
        "at least one module per source compiles"
    );
}

fn compile_jobs(sources: &[ModelSource]) -> Vec<CompileJob> {
    sources
        .iter()
        .enumerate()
        .flat_map(|(source_index, source)| {
            source.modules.iter().map(move |module| CompileJob {
                source_index,
                module: module.clone(),
            })
        })
        .collect()
}

#[test]
#[ignore = "spawned by shipped_veriloga_models_compile_end_to_end for per-module isolation"]
fn compile_one_shipped_veriloga_module_child() {
    if env::var_os(CHILD_ENV).is_none() {
        return;
    }

    let root = PathBuf::from(env::var_os(ROOT_ENV).expect("missing CMC compile root"));
    let source = PathBuf::from(env::var_os(SOURCE_ENV).expect("missing CMC compile source"));
    let module = env::var(MODULE_ENV).expect("missing CMC compile module");
    let include_dirs: Vec<PathBuf> = env::var_os(INCLUDES_ENV)
        .map(|paths| env::split_paths(&paths).collect())
        .unwrap_or_default();

    let options = CompilerOptions {
        include_paths: include_dirs,
        ..CompilerOptions::default()
    };
    let result = if env::var_os(TIMINGS_ENV).is_some() {
        compile_file_module_with_timings(&source, &module, &options)
    } else {
        VerilogACompiler::new(options).compile_file_module(&source, Some(&module))
    };

    let model = match result {
        Ok(model) => model,
        Err(error) => {
            eprintln!(
                "failed to compile {} :: {}: {error}",
                source.strip_prefix(&root).unwrap_or(&source).display(),
                module
            );
            std::process::exit(1);
        }
    };

    if model.name.as_str() != module {
        eprintln!(
            "compiler returned module '{}' while compiling {} :: {}",
            model.name,
            source.strip_prefix(&root).unwrap_or(&source).display(),
            module
        );
        std::process::exit(1);
    }
}

fn compile_file_module_with_timings(
    source: &Path,
    module: &str,
    options: &CompilerOptions,
) -> rspice_veriloga::CompileResult<CompiledModel> {
    let total_start = Instant::now();

    let mut pp = Preprocessor::new();
    for include_path in &options.include_paths {
        pp.add_include_path(include_path);
    }
    for (name, value) in &options.defines {
        pp.define(name, MacroDef::simple(value.as_deref().unwrap_or("")));
    }

    let phase_start = Instant::now();
    let preprocessed = pp
        .preprocess_file(source)
        .map_err(|e| rspice_veriloga::CompileError::io_error(format!("Preprocessor error: {e}")))?;
    let preprocess_elapsed = phase_start.elapsed();
    let dependency_count = pp.take_dependencies().len();

    let phase_start = Instant::now();
    let source_map = SourceMap::new();
    let source_id = source_map.add_source(source.display().to_string(), &preprocessed);
    let tokens = Lexer::new(&preprocessed, source_id).collect_tokens()?;
    let lex_elapsed = phase_start.elapsed();

    let phase_start = Instant::now();
    let source_file = Parser::new(&tokens).parse()?;
    let parse_elapsed = phase_start.elapsed();

    let phase_start = Instant::now();
    let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;
    let semantic_elapsed = phase_start.elapsed();

    let phase_start = Instant::now();
    let model = CodeGenerator::new().generate_module(&analyzed, Some(module))?;
    let codegen_elapsed = phase_start.elapsed();

    eprintln!(
        "timings {} :: {} preprocess={:.3}s lex={:.3}s parse={:.3}s semantic={:.3}s codegen={:.3}s total={:.3}s bytes={} tokens={} deps={} vars={} assignments={} assignment_instrs={} stamps={} jacobians={} stamp_instrs={}",
        source.display(),
        module,
        preprocess_elapsed.as_secs_f64(),
        lex_elapsed.as_secs_f64(),
        parse_elapsed.as_secs_f64(),
        semantic_elapsed.as_secs_f64(),
        codegen_elapsed.as_secs_f64(),
        total_start.elapsed().as_secs_f64(),
        preprocessed.len(),
        tokens.len(),
        dependency_count,
        model.num_variables,
        count_assignment_steps(&model),
        count_assignment_instructions(&model),
        model.stamp_programs.len(),
        model
            .stamp_programs
            .iter()
            .map(|program| program.jacobian_programs.len() + program.reactive_jacobians.len())
            .sum::<usize>(),
        count_stamp_instructions(&model)
    );

    Ok(model)
}

fn count_assignment_steps(model: &CompiledModel) -> usize {
    fn count_steps(steps: &[rspice_veriloga::codegen::AssignmentStep]) -> usize {
        steps
            .iter()
            .map(|step| match step {
                rspice_veriloga::codegen::AssignmentStep::Assign(_) => 1,
                rspice_veriloga::codegen::AssignmentStep::AssignIndexed { .. } => 1,
                rspice_veriloga::codegen::AssignmentStep::Loop { body, .. } => {
                    1 + count_steps(body)
                }
            })
            .sum()
    }
    count_steps(&model.assignment_steps)
}

fn count_assignment_instructions(model: &CompiledModel) -> usize {
    fn count_steps(steps: &[rspice_veriloga::codegen::AssignmentStep]) -> usize {
        steps
            .iter()
            .map(|step| match step {
                rspice_veriloga::codegen::AssignmentStep::Assign(assign) => {
                    assign.program.instructions.len()
                }
                rspice_veriloga::codegen::AssignmentStep::AssignIndexed {
                    index, value, ..
                } => index.instructions.len() + value.instructions.len(),
                rspice_veriloga::codegen::AssignmentStep::Loop { condition, body } => {
                    condition.instructions.len() + count_steps(body)
                }
            })
            .sum()
    }
    count_steps(&model.assignment_steps)
}

fn count_stamp_instructions(model: &CompiledModel) -> usize {
    model
        .stamp_programs
        .iter()
        .map(|program| {
            program.value_program.instructions.len()
                + program
                    .jacobian_programs
                    .iter()
                    .map(|entry| entry.program.instructions.len())
                    .sum::<usize>()
                + program
                    .reactive_jacobians
                    .iter()
                    .map(|entry| entry.program.instructions.len())
                    .sum::<usize>()
                + program
                    .static_condition
                    .as_ref()
                    .map(|condition| condition.instructions.len())
                    .unwrap_or(0)
        })
        .sum()
}

enum CompileChildResult {
    Compiled {
        elapsed: Duration,
        output: String,
    },
    Failed {
        status: String,
        elapsed: Duration,
        output: String,
    },
    TimedOut {
        elapsed: Duration,
        output: String,
    },
}

fn run_compile_child(
    root: &Path,
    source: &ModelSource,
    module: &str,
    timeout: Duration,
) -> CompileChildResult {
    let include_paths =
        env::join_paths(&source.include_dirs).expect("package include paths should join");
    let start = Instant::now();
    let mut command = Command::new(env::current_exe().expect("current test executable"));
    command
        .arg("compile_one_shipped_veriloga_module_child")
        .arg("--exact")
        .arg("--ignored")
        .arg("--nocapture")
        .env(CHILD_ENV, "1")
        .env(ROOT_ENV, root)
        .env(SOURCE_ENV, &source.path)
        .env(MODULE_ENV, module)
        .env(INCLUDES_ENV, include_paths)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if env::var_os(TIMINGS_ENV).is_some() {
        command.env("RSPICE_VERILOGA_COMPILE_TIMINGS", "1");
    }
    let mut child = command
        .spawn()
        .expect("spawn per-module Verilog-A compile child");

    loop {
        if child
            .try_wait()
            .expect("poll per-module Verilog-A compile child")
            .is_some()
        {
            let output = child
                .wait_with_output()
                .expect("collect per-module Verilog-A compile child output");
            let elapsed = start.elapsed();
            let combined = command_output(&output);
            return if output.status.success() {
                CompileChildResult::Compiled {
                    elapsed,
                    output: combined,
                }
            } else {
                CompileChildResult::Failed {
                    status: output.status.to_string(),
                    elapsed,
                    output: combined,
                }
            };
        }

        if start.elapsed() >= timeout {
            let _ = child.kill();
            let output = child
                .wait_with_output()
                .expect("collect timed-out Verilog-A compile child output");
            return CompileChildResult::TimedOut {
                elapsed: start.elapsed(),
                output: command_output(&output),
            };
        }

        thread::sleep(Duration::from_millis(25));
    }
}

fn module_timeout() -> Duration {
    let seconds = env::var(TIMEOUT_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|seconds| *seconds > 0)
        .unwrap_or(DEFAULT_MODULE_TIMEOUT_SECS);
    Duration::from_secs(seconds)
}

fn compile_worker_count(job_count: usize) -> usize {
    if job_count == 0 {
        return 0;
    }

    let default_jobs = thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .clamp(1, DEFAULT_MAX_JOBS);
    let requested_jobs = env::var(JOBS_ENV)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|jobs| *jobs > 0)
        .unwrap_or(default_jobs);

    requested_jobs.min(job_count).max(1)
}

fn command_output(output: &std::process::Output) -> String {
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

fn trim_output(output: &str) -> String {
    const MAX_CHARS: usize = 4000;
    let trimmed = output.trim();
    if trimmed.chars().count() <= MAX_CHARS {
        return trimmed.to_string();
    }

    let tail: String = trimmed
        .chars()
        .rev()
        .take(MAX_CHARS)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    format!("... output truncated ...\n{tail}")
}

fn discover_model_sources(root: &Path) -> io::Result<Vec<ModelSource>> {
    let mut files = Vec::new();
    collect_va_files(root, &mut files)?;

    let mut sources = Vec::new();
    for path in files {
        let text = fs::read_to_string(&path)?;
        let modules = extract_modules(&text);
        if modules.is_empty() {
            continue;
        }

        let relative = path.strip_prefix(root).unwrap_or(&path);
        let package = package_name(relative).unwrap_or_else(|| ".".to_string());
        let package_root = package_root(root, relative);
        let include_dirs = discover_include_dirs(&package_root)?;

        sources.push(ModelSource {
            package,
            path,
            modules,
            include_dirs,
        });
    }

    sources.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(sources)
}

fn filter_sources(sources: Vec<ModelSource>, root: &Path, filter: &str) -> Vec<ModelSource> {
    let needle = filter.to_ascii_lowercase();
    sources
        .into_iter()
        .filter_map(|mut source| {
            let relative = source.path.strip_prefix(root).unwrap_or(&source.path);
            let package_matches = source.package.to_ascii_lowercase().contains(&needle);
            let path_matches = relative
                .to_string_lossy()
                .to_ascii_lowercase()
                .contains(&needle);
            if package_matches || path_matches {
                return Some(source);
            }

            source
                .modules
                .retain(|module| module.to_ascii_lowercase().contains(&needle));
            (!source.modules.is_empty()).then_some(source)
        })
        .collect()
}

fn collect_va_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if should_skip(dir) {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if should_skip(&path) {
            continue;
        }
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            collect_va_files(&path, files)?;
        } else if metadata.is_file() && has_extension(&path, "va") {
            files.push(path);
        }
    }
    files.sort();
    Ok(())
}

fn package_name(relative: &Path) -> Option<String> {
    let parts: Vec<_> = relative
        .components()
        .filter_map(component_to_string)
        .take(2)
        .collect();
    (parts.len() >= 2).then(|| format!("{}/{}", parts[0], parts[1]))
}

fn package_root(root: &Path, relative: &Path) -> PathBuf {
    let mut components = relative.components().filter_map(component_to_string);
    if let (Some(collection), Some(package)) = (components.next(), components.next()) {
        root.join(collection).join(package)
    } else {
        root.to_path_buf()
    }
}

fn discover_include_dirs(package_root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs = BTreeSet::new();
    collect_include_dirs(package_root, &mut dirs)?;
    Ok(dirs.into_iter().collect())
}

fn collect_include_dirs(dir: &Path, dirs: &mut BTreeSet<PathBuf>) -> io::Result<bool> {
    if should_skip(dir) {
        return Ok(false);
    }

    let mut contains_include_file = false;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if should_skip(&path) {
            continue;
        }
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if collect_include_dirs(&path, dirs)? {
                contains_include_file = true;
            }
        } else if metadata.is_file() && is_include_file(&path) {
            contains_include_file = true;
        }
    }

    if contains_include_file {
        dirs.insert(dir.to_path_buf());
    }
    Ok(contains_include_file)
}

fn is_include_file(path: &Path) -> bool {
    ["va", "vams", "inc", "include", "h"]
        .iter()
        .any(|extension| has_extension(path, extension))
}

fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}

fn should_skip(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.') || name == "__MACOSX")
}

fn component_to_string(component: Component<'_>) -> Option<String> {
    match component {
        Component::Normal(part) => part.to_str().map(str::to_string),
        _ => None,
    }
}

fn extract_modules(source: &str) -> Vec<String> {
    let tokens = tokenize_without_comments(source);
    let mut modules = Vec::new();
    let mut iter = tokens.iter();
    while let Some(token) = iter.next() {
        if token.eq_ignore_ascii_case("module")
            && let Some(name) = iter.next()
            && is_identifier(name)
        {
            modules.push(name.clone());
        }
    }
    modules.sort();
    modules.dedup();
    modules
}

fn tokenize_without_comments(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = source.chars().peekable();
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut in_string = false;
    let mut string_escape = false;

    while let Some(ch) = chars.next() {
        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                let _ = chars.next();
                in_block_comment = false;
            }
            continue;
        }
        if in_string {
            if string_escape {
                string_escape = false;
            } else if ch == '\\' {
                string_escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '/' && chars.peek() == Some(&'/') {
            flush_token(&mut current, &mut tokens);
            let _ = chars.next();
            in_line_comment = true;
            continue;
        }
        if ch == '/' && chars.peek() == Some(&'*') {
            flush_token(&mut current, &mut tokens);
            let _ = chars.next();
            in_block_comment = true;
            continue;
        }
        if ch == '"' {
            flush_token(&mut current, &mut tokens);
            in_string = true;
            continue;
        }

        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
            current.push(ch);
        } else {
            flush_token(&mut current, &mut tokens);
        }
    }
    flush_token(&mut current, &mut tokens);
    tokens
}

fn flush_token(current: &mut String, tokens: &mut Vec<String>) {
    if !current.is_empty() {
        tokens.push(std::mem::take(current));
    }
}

fn is_identifier(token: &str) -> bool {
    token
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '$')
}

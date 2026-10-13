//! One case: a deck run on the route this binary was built as.
//!
//! # The headless path (default)
//!
//! The deck splits into its analysis card and its circuit. The card is lowered
//! by `rspice_simulation::manual_deck::build_manual_deck_queue` (the reader the
//! application uses for an authored deck) and re-spelled as a study task's
//! card (`analysis_spec_to_config(..).to_spice()`, the lowering `rspice study`
//! applies), so the task graph is exactly what a study document of the same
//! request would produce. The circuit goes through
//! [`rspice_simulation::execution::prepare_headless_run`] with the native
//! include resolver; the snapshot is authorized and each task runs on
//! [`rspice_simulation::runner::SimulationRunner`] with dependency artifacts
//! threaded as `rspice study run` threads them.
//!
//! HDL sources (`.hdl`/`.va`/`.vams`/`.verilog` directives) are compiled with
//! the unified runtime compiler options and sealed as
//! [`PreparedVerilogARuntime`]s under project-scoped virtual keys, and the
//! directive is rewritten to the `.veriloga "<key>" <alias>` form the
//! application emits for project Verilog-A. The headless preparation refuses
//! file-backed HDL directives as unsealed external dependencies, so this is the
//! only way a mixed design reaches that path, and it is the way the app's
//! model-library and project sources reach it.
//!
//! # The engine fallback (amendment A1)
//!
//! Three analysis kinds the headless contract cannot express run through the
//! `rspice_core::Engine` entries directly, and the case records
//! `dispatch = "engine"` with the reason ([`ENGINE_FALLBACKS`]).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use rspice_app_types::product::AnalysisInstanceId;
use rspice_core::abort_signal::AbortSignal;
use rspice_simulation::execution::{
    HeadlessPreparationError, HeadlessRunInput, HeadlessSourceResolver, HeadlessTaskRequest,
    PreparedRunAuthorization, prepare_headless_run,
};
use rspice_simulation::manual_deck::ManualDeckError;
use rspice_simulation::preparation::QueuedAnalysis;
use rspice_simulation::results::SimulationResult;
use rspice_simulation::veriloga::{PreparedVerilogARuntime, PreparedVerilogARuntimeSet};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

use super::Route;
use super::case::{CaseResult, ErrorRecord, Observation, canonical_signal};
use super::design::{Design, analysis_kind, deck_cards};
use super::measure::parse_expression;
use crate::suites::deadline::DeadlineAbort;

/// Analysis kinds that run through the engine entry, and why.
pub const ENGINE_FALLBACKS: [(&str, &str); 3] = [
    (
        "mc-tran",
        "a transient per Monte Carlo trial: the headless MonteCarlo task runs a DC operating point per trial (or a study base reduced to named .meas scalars), so per-trial waveforms and events are not expressible",
    ),
    (
        "step-tran",
        "a transient per .step point: the headless Parametric task reduces each point to one terminal value per signal, so per-point waveforms and events are not expressible",
    ),
    (
        "ckpt",
        "an uninterrupted transient and a resume from a checkpoint captured inside it, in one case: the headless graph has no task that resumes from a checkpoint another task of the same run produced",
    ),
];

/// One case request.
pub struct CaseRequest<'a> {
    pub design: &'a Design,
    pub variant: &'a str,
    pub analysis: &'a str,
    /// Run the held-digital O5 circuit with this deck's card instead.
    pub o5: bool,
    pub budget: Duration,
}

/// Run one case to a result. Never panics on simulator failure; a panic is a
/// crash the parent records from the missing file.
pub fn run_case(request: &CaseRequest<'_>) -> CaseResult {
    let started = Instant::now();
    let route = Route::compiled();
    let kind = analysis_kind(request.analysis).unwrap_or("unknown");
    let mut result = match compose(request) {
        Err(error) => CaseResult::new(route.token(), "none").failed(harness_error(error)),
        Ok((origin, source)) => {
            if let Some((_, reason)) = ENGINE_FALLBACKS
                .iter()
                .find(|(fallback, _)| *fallback == kind)
            {
                let mut result = engine_case(request, kind, &origin, &source);
                result.dispatch_reason = Some((*reason).to_owned());
                result
            } else {
                headless_case(request, &origin, &source)
            }
        }
    };
    let (signals, nets) = retained_outputs(request.design, request.variant, request.analysis);
    for observation in &mut result.results {
        observation.retain(&signals, &nets);
        observation.quarantine_nonfinite();
    }
    result.elapsed_s = started.elapsed().as_secs_f64();
    result
}

/// The deck (or the O5 circuit spliced with the deck's cards) and its origin.
fn compose(request: &CaseRequest<'_>) -> Result<(PathBuf, String), String> {
    let deck_path = request.design.deck_path(request.variant, request.analysis);
    let deck = std::fs::read_to_string(&deck_path)
        .map_err(|error| format!("{}: {error}", deck_path.display()))?;
    let deck_path = std::path::absolute(&deck_path).map_err(|error| error.to_string())?;
    if !request.o5 {
        return Ok((deck_path, deck));
    }
    let o5_path = request.design.o5_path(request.variant);
    let o5 = std::fs::read_to_string(&o5_path)
        .map_err(|error| format!("{}: {error}", o5_path.display()))?;
    let mut lines: Vec<String> = o5.lines().map(str::to_owned).collect();
    while lines
        .last()
        .is_some_and(|line| line.trim().is_empty() || line.trim().eq_ignore_ascii_case(".end"))
    {
        lines.pop();
    }
    for (_, card) in deck_cards(&deck) {
        lines.push(card);
    }
    lines.push(".end".to_owned());
    let o5_path = std::path::absolute(&o5_path).map_err(|error| error.to_string())?;
    Ok((o5_path, lines.join("\n") + "\n"))
}

/// Signals and nets the design's quantities read for this cell.
pub fn retained_outputs(
    design: &Design,
    variant: &str,
    analysis: &str,
) -> (Vec<String>, Vec<String>) {
    let mut signals = BTreeSet::new();
    let mut nets = BTreeSet::new();
    for quantity in design.quantities_for(variant, analysis) {
        for measure in [Some(&quantity.measure), quantity.of.as_ref()]
            .into_iter()
            .flatten()
        {
            let mut expressions: Vec<String> = measure.signals.clone().unwrap_or_default();
            if let Some(signal) = measure
                .signal
                .as_ref()
                .and_then(|signal| signal.for_variant(variant))
            {
                expressions.push(signal);
            }
            if matches!(measure.kind.as_str(), "noise" | "noise_int") && measure.signal.is_none() {
                signals.insert("onoise".to_owned());
            }
            if measure.kind == "stb" {
                signals.insert("loopgain".to_owned());
            }
            for expression in expressions {
                let lower = expression.trim().to_ascii_lowercase();
                if matches!(lower.as_str(), "onoise" | "inoise") {
                    signals.insert(lower);
                    continue;
                }
                for (_, name) in parse_expression(&expression) {
                    signals.insert(name);
                }
            }
            if let Some(list) = measure
                .nets
                .as_ref()
                .and_then(|nets| nets.for_variant(variant))
            {
                nets.extend(list.into_iter().map(|net| net.to_ascii_lowercase()));
            }
            if let Some(strobe) = &measure.strobe {
                nets.insert(strobe.to_ascii_lowercase());
            }
        }
        if quantity.measure.kind == "identical" {
            signals.insert("*".to_owned());
            nets.insert("*".to_owned());
        }
    }
    (signals.into_iter().collect(), nets.into_iter().collect())
}

/// Remove the analysis cards (and their continuation lines) from a deck.
pub fn strip_cards(source: &str) -> String {
    let mut out = Vec::new();
    let mut skipping = false;
    for (index, line) in source.lines().enumerate() {
        if index == 0 {
            out.push(line);
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with('+') && skipping {
            continue;
        }
        let word = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        skipping = super::design::ANALYSIS_CARDS.contains(&word.as_str());
        if !skipping {
            out.push(line);
        }
    }
    out.join("\n") + "\n"
}

// ---------------------------------------------------------------------------
// HDL sealing
// ---------------------------------------------------------------------------

const HDL_DIRECTIVES: [&str; 5] = [".hdl", ".va", ".veriloga", ".vams", ".verilog"];

/// One HDL directive of a deck.
#[derive(Debug, Clone, PartialEq)]
pub struct HdlDirective {
    pub line: usize,
    pub path: String,
    pub alias: Option<String>,
    pub module: Option<String>,
}

pub fn hdl_directives(source: &str) -> Vec<HdlDirective> {
    let mut directives = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        let Some(word) = trimmed.split_whitespace().next() else {
            continue;
        };
        if !HDL_DIRECTIVES.contains(&word.to_ascii_lowercase().as_str()) {
            continue;
        }
        let rest = trimmed[word.len()..].trim();
        let (path, rest) = if let Some(quoted) = rest.strip_prefix('"') {
            let end = quoted.find('"').unwrap_or(quoted.len());
            (
                quoted[..end].to_owned(),
                quoted.get(end + 1..).unwrap_or_default(),
            )
        } else {
            let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
            (rest[..end].to_owned(), &rest[end..])
        };
        let mut alias = None;
        let mut module = None;
        for token in rest.split_whitespace() {
            if let Some(name) = token
                .strip_prefix("module=")
                .or_else(|| token.strip_prefix("MODULE="))
            {
                module = Some(name.to_owned());
            } else if alias.is_none() {
                alias = Some(token.to_owned());
            }
        }
        directives.push(HdlDirective {
            line: index,
            path,
            alias,
            module,
        });
    }
    directives
}

/// Compile every HDL source the circuit names and rewrite its directive to the
/// sealed project form.
fn seal_hdl(
    circuit: &str,
    origin: &Path,
    design_slug: &str,
) -> Result<(String, PreparedVerilogARuntimeSet), CaseError> {
    let directives = hdl_directives(circuit);
    if directives.is_empty() {
        return Ok((circuit.to_owned(), PreparedVerilogARuntimeSet::default()));
    }
    let directory = origin.parent().unwrap_or(Path::new("."));
    let compiler = rspice_veriloga::VerilogACompiler::new(
        rspice_simulation::compilation::unified_runtime_compiler_options(),
    );
    let mut lines: Vec<String> = circuit.lines().map(str::to_owned).collect();
    let mut runtimes = Vec::new();
    for directive in directives {
        let path = directory.join(&directive.path);
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("source.va")
            .to_owned();
        let source = std::fs::read_to_string(&path).map_err(|error| ErrorRecord {
            category: "elaboration".into(),
            token: Some("missing_source".into()),
            module: Some(file_name.clone()),
            location: Some(path.display().to_string()),
            stage: Some("compile".into()),
            detail: error.to_string(),
            display: format!("{}: {error}", path.display()),
            ..Default::default()
        })?;
        let bundle = rspice_veriloga::VirtualSourceBundle::new(
            file_name.clone(),
            [rspice_veriloga::VirtualSourceFile::new(
                file_name.clone(),
                source.clone(),
            )],
        )
        .map_err(|error| compile_refusal(&file_name, &path, error.to_string()))?;
        let limits = rspice_simulation::project_veriloga::project_virtual_compile_limits();
        let discovered = compiler
            .discover_virtual_modules(&bundle, limits)
            .map_err(|failure| compile_refusal(&file_name, &path, failure.to_string()))?;
        let module = match (&directive.module, &directive.alias) {
            (Some(module), _) => module.clone(),
            (None, Some(alias))
                if discovered
                    .module_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(alias)) =>
            {
                discovered
                    .module_names
                    .iter()
                    .find(|name| name.eq_ignore_ascii_case(alias))
                    .cloned()
                    .unwrap_or_default()
            }
            _ => match discovered.module_names.as_slice() {
                [only] => only.clone(),
                many => {
                    return Err(compile_refusal(
                        &file_name,
                        &path,
                        format!(
                            "{} declares {} modules; name one with module=",
                            file_name,
                            many.len()
                        ),
                    )
                    .into());
                }
            },
        };
        let compilation = compiler
            .compile_virtual_runtime(&bundle, &module, limits)
            .map_err(|error| compile_refusal(&module, &path, error.to_string()))?;
        let alias = directive.alias.clone().unwrap_or_else(|| module.clone());
        let source_key = sealed_source_key(design_slug, &module, &file_name);
        let digest = rspice_app_types::product::ContentDigest::from_bytes(
            <sha2::Sha256 as sha2::Digest>::digest(source.as_bytes()).into(),
        );
        let runtime = PreparedVerilogARuntime::try_from_virtual_compilation(
            source_key.clone(),
            digest,
            alias.clone(),
            &compilation,
        )
        .map_err(|error| compile_refusal(&module, &path, error.to_string()))?;
        runtimes.push(runtime);
        lines[directive.line] = format!(".veriloga \"{source_key}\" {alias}");
    }
    let set = PreparedVerilogARuntimeSet::try_new(runtimes).map_err(harness_error)?;
    Ok((lines.join("\n") + "\n", set))
}

/// The virtual key a sealed runtime is installed under. The engine keys an
/// HDL source by its file *and* the module selected from it
/// (`VerilogASourceKey { source_path, selected_module }`), and the
/// application's project keys carry the selected module too, so one file
/// named by two directives (`module=tb_sar`, `module=sar8`) is two runtimes
/// with two keys; keyed by the file alone the second would collide with the
/// first.
pub fn sealed_source_key(design_slug: &str, module: &str, file_name: &str) -> String {
    format!("__rspice_project__/mixed-corpus/{design_slug}/{module}/{file_name}")
}

fn compile_refusal(module: &str, path: &Path, message: String) -> ErrorRecord {
    let location = first_location(&message).unwrap_or_else(|| path.display().to_string());
    ErrorRecord {
        category: "elaboration".into(),
        token: Some("compile_refusal".into()),
        token_form: Some("structured".into()),
        module: Some(module.to_owned()),
        location: Some(location),
        stage: Some("compile".into()),
        detail: message.clone(),
        display: format!(
            "{}: the Verilog-A compiler refused this source: {message}",
            path.display()
        ),
        ..Default::default()
    }
}

/// `file:line[:col]` at the start of a compiler message, if it leads with one.
fn first_location(message: &str) -> Option<String> {
    let head = message.split_whitespace().next()?;
    let head = head.trim_end_matches(':');
    let mut parts = head.rsplitn(3, ':');
    let last = parts.next()?;
    last.parse::<usize>().ok()?;
    Some(head.to_owned())
}

// ---------------------------------------------------------------------------
// Headless dispatch
// ---------------------------------------------------------------------------

fn headless_case(request: &CaseRequest<'_>, origin: &Path, source: &str) -> CaseResult {
    let route = Route::compiled();
    let mut result = CaseResult::new(route.token(), "simulation");
    let cards: Vec<String> = deck_cards(source)
        .into_iter()
        .map(|(_, card)| card)
        .collect();
    let circuit = strip_cards(source);
    let (circuit, runtimes) = match seal_hdl(&circuit, origin, &request.design.slug) {
        Ok(sealed) => sealed,
        Err(error) => return result.failed(*error),
    };
    let tasks = match headless_tasks(&circuit, origin, &cards, &runtimes) {
        Ok(tasks) => tasks,
        Err(error) => return result.failed(*error),
    };
    let labels: HashMap<AnalysisInstanceId, String> = tasks
        .iter()
        .map(|task| (task.instance_id, task.label.clone()))
        .collect();
    let abort = DeadlineAbort::new(Instant::now(), request.budget.as_millis());
    let mut input = HeadlessRunInput::new(&circuit, origin, tasks);
    input.resolver = HeadlessSourceResolver::Native {
        include_search_paths: Vec::new(),
    };
    input.veriloga_runtimes = runtimes;
    let snapshot = match prepare_headless_run(input, &abort) {
        Ok(snapshot) => snapshot,
        Err(error) => return result.failed(preparation_error(error)),
    };
    for advisory in snapshot.metadata().advisories {
        result.warnings.push(advisory);
    }
    let dispatch = match PreparedRunAuthorization::default().authorize_campaign_member(snapshot) {
        Ok(dispatch) => dispatch,
        Err(error) => return result.failed(harness_error(error)),
    };
    let mut pending = dispatch.into_tasks();
    let mut artifacts = HashMap::new();
    let mut runner = rspice_simulation::runner::SimulationRunner::new();
    while let Some(task) = pending.pop_front() {
        let instance = task.instance_id();
        let label = labels.get(&instance).cloned().unwrap_or_default();
        let resolved = match task.resolve_dependency_artifacts(&artifacts) {
            Ok(resolved) => resolved,
            Err(error) => return result.failed(harness_error(error.to_string())),
        };
        let producer = match resolved.artifact_producer() {
            Ok(producer) => producer,
            Err(error) => return result.failed(harness_error(error.to_string())),
        };
        if let Err(error) = runner.start_prepared(resolved, false) {
            return result.failed(simulation_error(&error));
        }
        let outcome = loop {
            if abort.is_aborted() {
                runner.abort();
            }
            if let Some(outcome) = runner.poll_result() {
                break outcome;
            }
            for line in runner.drain_engine_log() {
                if result.warnings.len() < 64 {
                    result.warnings.push(line.message);
                }
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        let simulated = match outcome {
            Ok(simulated) => simulated,
            Err(rspice_simulation::error::SimulationError::Aborted) if abort.is_aborted() => {
                return result.failed(ErrorRecord {
                    category: "timeout".into(),
                    detail: format!("budget of {} s spent", request.budget.as_secs()),
                    display: "case budget spent".into(),
                    stage: Some("execution".into()),
                    ..Default::default()
                });
            }
            Err(error) => return result.failed(simulation_error(&error)),
        };
        match producer.capture(&simulated, &pending) {
            Ok(Some(artifact)) => {
                artifacts.insert(instance, artifact);
            }
            Ok(None) => {}
            Err(error) => return result.failed(harness_error(error.to_string())),
        }
        result.results.push(observation(&simulated, &label));
    }
    result
}

/// Lower the deck's cards into the explicit task graph a study would carry.
///
/// The cards are lowered against the whole sealed circuit, includes expanded,
/// as the application's manual-deck preparation lowers its expanded source: a
/// driven `.PSS` takes its tone list from the circuit's time-varying sources,
/// so a card read without its circuit binds none, and the circuit elaborates
/// only with the sealed HDL runtimes its `.veriloga` lines name.
fn headless_tasks(
    circuit: &str,
    origin: &Path,
    cards: &[String],
    runtimes: &PreparedVerilogARuntimeSet,
) -> Result<Vec<HeadlessTaskRequest>, CaseError> {
    let directory = origin.parent().unwrap_or(Path::new("."));
    let expanded = rspice_core::netlist::IncludeProcessor::new(directory)
        .expand_content(circuit, origin)
        .map_err(|error| ErrorRecord {
            category: "parse".into(),
            stage: Some("preparation".into()),
            detail: error.to_string(),
            display: error.to_string(),
            ..Default::default()
        })?;
    let body: Vec<&str> = expanded
        .lines()
        .filter(|line| !line.trim().eq_ignore_ascii_case(".end"))
        .collect();
    let deck = format!("{}\n{}\n.end\n", body.join("\n"), cards.join("\n"));
    let queue = rspice_simulation::manual_deck::build_manual_deck_queue_with_runtimes(
        27.0, &deck, runtimes,
    )
    .map_err(|error| match error {
        // The engine's own refusal of the circuit, met while lowering a card,
        // keeps its structure.
        ManualDeckError::Refused(refusal) => ErrorRecord {
            stage: Some("preparation".into()),
            ..simulation_error(&refusal)
        },
        ManualDeckError::Cards(errors) => ErrorRecord {
            category: "parse".into(),
            stage: Some("preparation".into()),
            detail: errors.join("; "),
            display: errors.join("; "),
            ..Default::default()
        },
    })?;
    let mut tasks: Vec<HeadlessTaskRequest> = Vec::with_capacity(queue.len());
    for queued in queue {
        let line = study_card(&queued.spec).map_err(|message| ErrorRecord {
            category: "harness".into(),
            stage: Some("preparation".into()),
            detail: message.clone(),
            display: message,
            ..Default::default()
        })?;
        let label = queued.spec.run_type().display_name().to_owned();
        tasks.push(HeadlessTaskRequest {
            instance_id: AnalysisInstanceId::new(),
            label,
            dependencies: Vec::new(),
            analysis: QueuedAnalysis {
                spec: queued.spec,
                config: None,
                spec_options: queued.spec_options,
                analysis_line: line,
                numeric_override: None,
            },
        });
    }
    // Bind each consumer to the one producer of the artifact it needs, as the
    // manual-deck preparation does; the headless graph is explicit about it.
    use rspice_simulation::prepared_dependency::{ExecutionArtifactKind, required_artifact_kinds};
    let producers = |predicate: &dyn Fn(&AnalysisSpec) -> bool| -> Vec<AnalysisInstanceId> {
        tasks
            .iter()
            .filter(|task| predicate(&task.analysis.spec))
            .map(|task| task.instance_id)
            .collect()
    };
    let transient = producers(&|spec| matches!(spec, AnalysisSpec::Transient { .. }));
    let periodic = producers(&|spec| matches!(spec, AnalysisSpec::Pss { .. }));
    let harmonic = producers(&|spec| matches!(spec, AnalysisSpec::HarmonicBalance { .. }));
    let qpss = producers(&|spec| matches!(spec, AnalysisSpec::Qpss { .. }));
    let operating =
        producers(&|spec| matches!(spec, AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }));
    for task in &mut tasks {
        let kinds = required_artifact_kinds(&task.analysis.spec, &task.analysis.spec_options);
        let chosen = kinds.iter().find_map(|kind| {
            let candidates = match kind {
                ExecutionArtifactKind::TransientTrajectory => &transient,
                ExecutionArtifactKind::PeriodicState => &periodic,
                ExecutionArtifactKind::HbState => &harmonic,
                ExecutionArtifactKind::QpssState => &qpss,
                ExecutionArtifactKind::DcOperatingPointSeed => &operating,
            };
            (candidates.len() == 1).then(|| candidates[0])
        });
        if let Some(producer) = chosen
            && producer != task.instance_id
        {
            task.dependencies = vec![producer];
        }
    }
    Ok(tasks)
}

/// The card a study document carries for `spec` (`rspice study`'s lowering).
fn study_card(spec: &AnalysisSpec) -> Result<String, String> {
    use rspice_simulation::analysis_preparation::{
        analysis_spec_to_config, build_ac_data_command, build_transient_noise_command,
    };
    match spec {
        AnalysisSpec::Fft { request } => Ok(request.to_card()),
        AnalysisSpec::TransientNoise { .. } => build_transient_noise_command(spec),
        AnalysisSpec::AcData { .. } => build_ac_data_command(spec),
        AnalysisSpec::LegacyDcOp
        | AnalysisSpec::DcOp { .. }
        | AnalysisSpec::DcSweep { .. }
        | AnalysisSpec::Ac { .. }
        | AnalysisSpec::Transient { .. }
        | AnalysisSpec::Noise { .. }
        | AnalysisSpec::PoleZero { .. }
        | AnalysisSpec::Sensitivity { .. } => {
            analysis_spec_to_config(spec).map(|config| config.to_spice())
        }
        AnalysisSpec::MonteCarlo { .. } | AnalysisSpec::Parametric | AnalysisSpec::Corner => {
            Err("structured sweeps are not expressible as a headless task".into())
        }
        _ => Ok(String::new()),
    }
}

// ---------------------------------------------------------------------------
// Results
// ---------------------------------------------------------------------------

fn observation(result: &SimulationResult, label: &str) -> Observation {
    let mut observation = Observation {
        label: label.to_owned(),
        ..Default::default()
    };
    let waveforms_into =
        |observation: &mut Observation,
         waveforms: &HashMap<String, rspice_simulation::results::WaveformData>| {
            for (name, waveform) in waveforms {
                let key = canonical_signal(name);
                if waveform.is_complex {
                    let imag = waveform.y_imag.clone().unwrap_or_default();
                    observation.complex.insert(
                        key,
                        waveform
                            .y_values
                            .iter()
                            .zip(imag.iter().chain(std::iter::repeat(&0.0)))
                            .map(|(re, im)| [*re, *im])
                            .collect(),
                    );
                } else {
                    observation.real.insert(key, waveform.y_values.clone());
                }
            }
        };
    match result {
        SimulationResult::DcOp(op) => {
            observation.kind = "op".into();
            for (name, value) in &op.node_voltages {
                observation
                    .real
                    .insert(canonical_signal(name), vec![*value]);
            }
            for (name, value) in &op.branch_currents {
                observation.real.insert(branch_name(name), vec![*value]);
            }
        }
        SimulationResult::DcSweep {
            sweep_values,
            waveforms,
            ..
        } => {
            observation.kind = "dc".into();
            observation.axis = sweep_values.clone();
            waveforms_into(&mut observation, waveforms);
        }
        SimulationResult::Transient {
            time,
            waveforms,
            events,
            ..
        } => {
            observation.kind = "tran".into();
            observation.axis = time.clone();
            waveforms_into(&mut observation, waveforms);
            for history in &events.digital {
                let trace = history
                    .points
                    .iter()
                    .map(|point| {
                        let letter =
                            super::events::xspice_code_letter(point.value_code).unwrap_or("x");
                        (point.time_s, letter.to_owned())
                    })
                    .collect::<Vec<_>>();
                observation.events.insert(
                    history.node_name.to_ascii_lowercase(),
                    super::events::collapse(&trace),
                );
            }
            for history in &events.real {
                let trace = history
                    .points
                    .iter()
                    .map(|point| (point.time_s, format!("{:e}", point.value)))
                    .collect::<Vec<_>>();
                observation
                    .events
                    .insert(history.node_name.to_ascii_lowercase(), trace);
            }
        }
        SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } => {
            observation.kind = "ac".into();
            observation.axis = frequencies.clone();
            waveforms_into(&mut observation, waveforms);
        }
        SimulationResult::Noise {
            frequencies,
            output_noise,
            input_noise,
            ..
        } => {
            observation.kind = "noise".into();
            observation.axis = frequencies.clone();
            observation
                .real
                .insert("onoise".into(), output_noise.clone());
            if let Some(input) = input_noise {
                observation.real.insert("inoise".into(), input.clone());
            }
        }
        SimulationResult::TransferFunction {
            gain,
            input_resistance,
            output_resistance,
            ..
        } => {
            observation.kind = "tf".into();
            for (name, value) in [
                ("gain", gain),
                ("zin", input_resistance),
                ("zout", output_resistance),
            ] {
                if let Some(value) = value {
                    observation.scalars.insert(name.into(), scalar(*value));
                }
            }
        }
        SimulationResult::Stb { response, .. } => {
            observation.kind = "stb".into();
            let margins = &response.margins;
            if let Some(margin) = &margins.phase_margin {
                observation.scalars.insert("pm".into(), margin.value);
            }
            if let Some(margin) = &margins.gain_margin {
                observation.scalars.insert("gm".into(), margin.value);
            }
            if let Some(ugf) = margins.unity_gain_bandwidth() {
                observation.scalars.insert("ugf".into(), ugf);
            }
            observation.axis = response
                .bode_points
                .iter()
                .map(|point| point.frequency)
                .collect();
            observation.complex.insert(
                "loopgain".into(),
                response
                    .bode_points
                    .iter()
                    .map(|point| [point.loop_gain.re, point.loop_gain.im])
                    .collect(),
            );
        }
        SimulationResult::PoleZero { poles, zeros, .. } => {
            observation.kind = "pz".into();
            observation.poles = poles.iter().map(|(re, im)| [*re, *im]).collect();
            observation.zeros = zeros.iter().map(|(re, im)| [*re, *im]).collect();
        }
        SimulationResult::SensitivityStudy { evidence } => {
            observation.kind = "sens".into();
            for row in &evidence.rows {
                if let Some(rspice_results::sensitivity::SensitivityValue::Available(value)) =
                    row.raw.first()
                {
                    observation
                        .scalars
                        .insert(row.parameter.to_ascii_lowercase(), *value);
                }
            }
        }
        other => {
            observation.kind = "other".into();
            observation.label = format!("{label} ({})", result_name(other));
        }
    }
    observation
}

fn result_name(result: &SimulationResult) -> &'static str {
    match result {
        SimulationResult::MonteCarlo { .. } => "monte carlo",
        SimulationResult::Parametric { .. } => "parametric",
        SimulationResult::HarmonicBalance { .. } => "harmonic balance",
        SimulationResult::Pstb { .. } => "pstb",
        _ => "unclassified",
    }
}

fn branch_name(name: &str) -> String {
    let lower = name.trim().to_ascii_lowercase();
    if lower.starts_with("i(") {
        canonical_signal(&lower)
    } else {
        format!("i({})", lower.trim_end_matches("#branch"))
    }
}

fn scalar(value: rspice_simulation::results::TransferFunctionScalar) -> f64 {
    use rspice_simulation::results::TransferFunctionScalar;
    match value {
        TransferFunctionScalar::Finite(value) => value,
        TransferFunctionScalar::PositiveInfinity => f64::INFINITY,
        TransferFunctionScalar::NegativeInfinity => f64::NEG_INFINITY,
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// A case's error on its way to `case-result.json`, boxed: an [`ErrorRecord`]
/// is a dozen optional strings and only the failure path builds one.
type CaseError = Box<ErrorRecord>;

fn harness_error(message: impl Into<String>) -> ErrorRecord {
    let message = message.into();
    ErrorRecord {
        category: "harness".into(),
        detail: message.clone(),
        display: message,
        ..Default::default()
    }
}

/// A capability token flattened into a message: `unsupported capability
/// [token][ in <analysis>][ at run <coordinate>]: detail`.
pub fn prose_capability(text: &str) -> Option<(String, Option<String>, String)> {
    let lower = text.to_ascii_lowercase();
    let start = lower.find("unsupported capability [")? + "unsupported capability [".len();
    let end = start + text[start..].find(']')?;
    let token = text[start..end].to_owned();
    let rest = &text[end + 1..];
    let (head, detail) = rest.split_once(": ").unwrap_or((rest, ""));
    let analysis = head.trim().strip_prefix("in ").map(|value| {
        value
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_owned()
    });
    Some((token, analysis, detail.trim().to_owned()))
}

fn from_prose(category_if_plain: &str, display: String, stage: &str) -> ErrorRecord {
    if let Some((token, analysis, detail)) = prose_capability(&display) {
        return ErrorRecord {
            category: "capability".into(),
            token: Some(token),
            token_form: Some("prose".into()),
            analysis,
            stage: Some(stage.into()),
            detail,
            display,
            ..Default::default()
        };
    }
    ErrorRecord {
        category: category_if_plain.into(),
        stage: Some(stage.into()),
        detail: display.clone(),
        display,
        ..Default::default()
    }
}

fn simulation_error(error: &rspice_simulation::error::SimulationError) -> ErrorRecord {
    use rspice_simulation::error::SimulationError as E;
    let display = error.to_string();
    match error {
        E::Elaboration {
            instance,
            module,
            kind,
            location,
            message,
        } => ErrorRecord {
            category: "elaboration".into(),
            token: Some(kind.clone()),
            token_form: Some("structured".into()),
            instance: instance.clone(),
            module: module.clone(),
            location: location.clone(),
            stage: Some("execution".into()),
            detail: message.clone(),
            display,
            ..Default::default()
        },
        E::UnsupportedCapability {
            capability,
            analysis_kind,
            instance,
            module,
            message,
        } => ErrorRecord {
            category: "capability".into(),
            token: Some(capability.clone()),
            token_form: Some("structured".into()),
            instance: instance.clone(),
            module: module.clone(),
            analysis_kind: analysis_kind.clone(),
            stage: Some("execution".into()),
            // The reason without the token: the message is the core
            // rendering, `unsupported capability [token] ...: reason`.
            detail: prose_capability(message)
                .map(|(_, _, detail)| detail)
                .filter(|detail| !detail.is_empty())
                .unwrap_or_else(|| message.clone()),
            display,
            ..Default::default()
        },
        E::ParseError(_) => from_prose("parse", display, "execution"),
        E::CircuitError(_) => from_prose("simulation", display, "execution"),
        E::SolverError(_) => from_prose("solver", display, "execution"),
        E::ConvergenceFailed { .. } | E::Attributed { .. } => {
            from_prose("convergence", display, "execution")
        }
        E::ResourceLimit { .. } | E::ResourceFailure(_) => {
            from_prose("resource", display, "execution")
        }
        E::InvalidConfig(_) => from_prose("simulation", display, "execution"),
        E::ThreadPanic => ErrorRecord {
            category: "internal".into(),
            code: Some("thread_panic".into()),
            detail: display.clone(),
            display,
            stage: Some("execution".into()),
            ..Default::default()
        },
        _ => from_prose("internal", display, "execution"),
    }
}

fn preparation_error(error: HeadlessPreparationError) -> ErrorRecord {
    match error {
        HeadlessPreparationError::Parse(parse) => match &parse {
            rspice_core::netlist::ParseError::UnsupportedCapability {
                origin,
                capability,
                detail,
            } => ErrorRecord {
                category: "parse".into(),
                token: Some((*capability).to_owned()),
                token_form: Some("structured".into()),
                location: Some(origin.to_string()),
                stage: Some("preparation".into()),
                detail: detail.clone(),
                display: parse.to_string(),
                ..Default::default()
            },
            _ => ErrorRecord {
                category: "parse".into(),
                location: parse.source_location().map(|location| location.to_string()),
                stage: Some("preparation".into()),
                detail: parse.to_string(),
                display: parse.to_string(),
                ..Default::default()
            },
        },
        HeadlessPreparationError::Aborted => ErrorRecord {
            category: "timeout".into(),
            stage: Some("preparation".into()),
            detail: "budget spent during preparation".into(),
            display: "case budget spent".into(),
            ..Default::default()
        },
        HeadlessPreparationError::ResourceLimit(limit) => {
            let display = limit.to_string();
            ErrorRecord {
                category: "resource".into(),
                stage: Some("preparation".into()),
                detail: display.clone(),
                display,
                ..Default::default()
            }
        }
        other => from_prose("preparation", other.to_string(), "preparation"),
    }
}

fn core_error(error: &rspice_core::SimulationError) -> ErrorRecord {
    let display = error.to_string();
    match error {
        rspice_core::SimulationError::UnsupportedCapability(refusal) => ErrorRecord {
            category: "capability".into(),
            token: Some(refusal.capability.to_owned()),
            token_form: Some("structured".into()),
            analysis: refusal.analysis.map(|analysis| analysis.to_string()),
            analysis_kind: error
                .descriptor()
                .analysis_kind
                .map(|kind| kind.tag().to_owned()),
            instance: refusal.instance.clone(),
            module: refusal.module.clone(),
            location: refusal.location.as_ref().map(ToString::to_string),
            stage: Some("execution".into()),
            detail: refusal.detail.clone(),
            display,
            ..Default::default()
        },
        rspice_core::SimulationError::Elaboration(elaboration) => ErrorRecord {
            category: "elaboration".into(),
            token: Some(elaboration.kind.as_str().to_owned()),
            token_form: Some("structured".into()),
            instance: elaboration.instance.clone(),
            module: elaboration.module.clone(),
            location: elaboration.span.as_ref().map(ToString::to_string),
            stage: Some("execution".into()),
            detail: display.clone(),
            display,
            ..Default::default()
        },
        rspice_core::SimulationError::Aborted | rspice_core::SimulationError::TimeLimitExceeded => {
            ErrorRecord {
                category: "timeout".into(),
                stage: Some("execution".into()),
                detail: display.clone(),
                display,
                ..Default::default()
            }
        }
        other => {
            let category = match other.descriptor().category {
                rspice_core::SimulationErrorCategory::Convergence => "convergence",
                rspice_core::SimulationErrorCategory::Solver => "solver",
                rspice_core::SimulationErrorCategory::Netlist => "parse",
                rspice_core::SimulationErrorCategory::ResourceLimit => "resource",
                _ => "simulation",
            };
            ErrorRecord {
                category: category.into(),
                code: Some(format!("{:?}", other.descriptor().code)),
                stage: Some("execution".into()),
                detail: display.clone(),
                display,
                ..Default::default()
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Engine fallback
// ---------------------------------------------------------------------------

fn engine_case(request: &CaseRequest<'_>, kind: &str, origin: &Path, source: &str) -> CaseResult {
    let route = Route::compiled();
    let result = CaseResult::new(route.token(), "engine");
    let netlist = match rspice_core::Netlist::parse_with_path(source, origin) {
        Ok(netlist) => netlist,
        Err(error) => {
            let display = error.to_string();
            return result.failed(match &error {
                rspice_core::netlist::ParseError::UnsupportedCapability {
                    capability,
                    origin,
                    detail,
                } => ErrorRecord {
                    category: "parse".into(),
                    token: Some((*capability).to_owned()),
                    token_form: Some("structured".into()),
                    location: Some(origin.to_string()),
                    stage: Some("preparation".into()),
                    detail: detail.clone(),
                    display,
                    ..Default::default()
                },
                _ => ErrorRecord {
                    category: "parse".into(),
                    stage: Some("preparation".into()),
                    detail: display.clone(),
                    display,
                    ..Default::default()
                },
            });
        }
    };
    let mut config = rspice_core::engine::SimulationConfig::default();
    if let Some(dialect) = dialect(&request.design.file.dialect) {
        config.spice_dialect = dialect;
    }
    let engine = rspice_core::Engine::new(config);
    let abort = DeadlineAbort::new(Instant::now(), request.budget.as_millis());
    let Some((step, stop, start, max_step, uic)) =
        netlist.analyses.iter().find_map(|analysis| match analysis {
            rspice_core::netlist::AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            } => Some((*step, *stop, *start, *max_step, *uic)),
            _ => None,
        })
    else {
        return result.failed(harness_error("deck has no .tran card"));
    };
    let internal_max =
        match rspice_core::execution::resolve_transient_maximum_step(step, stop, start, max_step) {
            Ok(value) => value,
            Err(error) => return result.failed(harness_error(error.to_string())),
        };
    let startup = rspice_core::engine::TransientStartupMode::from_uic(uic);
    let outcome = match kind {
        "mc-tran" => monte_carlo(
            &engine,
            &netlist,
            source,
            origin,
            stop,
            internal_max,
            startup,
            &abort,
        ),
        "step-tran" => step_points(&engine, &netlist, stop, internal_max, startup, &abort),
        "ckpt" => checkpoint(
            &engine,
            &netlist,
            request,
            stop,
            internal_max,
            startup,
            &abort,
        ),
        other => Err(harness_error(format!("no engine dispatch for {other}")).into()),
    };
    match outcome {
        Ok(observation) => {
            let mut result = result;
            result.results.push(observation);
            result
        }
        Err(error) => result.failed(*error),
    }
}

fn dialect(name: &str) -> Option<rspice_core::config::SpiceDialect> {
    use rspice_core::config::SpiceDialect;
    match name.to_ascii_lowercase().as_str() {
        "best-available" => Some(SpiceDialect::BestAvailable),
        "ngspice" => Some(SpiceDialect::Ngspice),
        "xyce" => Some(SpiceDialect::Xyce),
        _ => None,
    }
}

fn run_transient(
    engine: &rspice_core::Engine,
    netlist: &rspice_core::Netlist,
    stop: f64,
    max_step: f64,
    startup: rspice_core::engine::TransientStartupMode,
    abort: &dyn AbortSignal,
) -> Result<Observation, rspice_core::SimulationError> {
    let result =
        engine.run_tran_with_startup_mode_and_abort(netlist, stop, max_step, startup, abort)?;
    Ok(transient_observation(&result))
}

/// A core transient result as an observation (the fallback's result shape).
pub fn transient_observation(result: &rspice_core::engine::TransientResult) -> Observation {
    let mut observation = Observation {
        kind: "tran".into(),
        axis: result.time.clone(),
        ..Default::default()
    };
    for (name, series) in result.node_names.iter().zip(&result.voltages) {
        if !series.is_empty() {
            observation
                .real
                .insert(canonical_signal(name), series.clone());
        }
    }
    for (name, series) in result.branch_names.iter().zip(&result.branch_currents) {
        if !series.is_empty() {
            observation.real.insert(branch_name(name), series.clone());
        }
    }
    for trace in &result.digital_traces {
        let points: Vec<(f64, String)> = trace
            .points
            .iter()
            .map(|point| {
                (
                    point.time,
                    super::events::digital_value_letter(point.value).to_owned(),
                )
            })
            .collect();
        observation.events.insert(
            trace.node_name.to_ascii_lowercase(),
            super::events::collapse(&points),
        );
    }
    for trace in &result.real_traces {
        observation.events.insert(
            trace.node_name.to_ascii_lowercase(),
            trace
                .points
                .iter()
                .map(|point| (point.time, format!("{:e}", point.value)))
                .collect(),
        );
    }
    observation
}

/// Record every top-level `.param` value the trial netlist resolved.
fn record_parameters(netlist: &rspice_core::Netlist, observation: &mut Observation) {
    for (name, value) in netlist.params.all_params() {
        if value.is_finite() {
            observation
                .scalars
                .insert(format!("param:{}", name.to_ascii_lowercase()), value);
        }
    }
}

/// Statistical functions a deck may state its Monte Carlo variation with.
const STATISTICAL_FUNCTIONS: [&str; 5] = ["agauss(", "aunif(", "gauss(", "unif(", "limit("];

/// Whether the deck, with every file it includes, states a statistical
/// function outside comments. An include that cannot be resolved is an
/// error, not "no statistics".
pub fn deck_states_statistics(source: &str, origin: &Path) -> Result<bool, String> {
    let directory = origin.parent().unwrap_or(Path::new("."));
    let expanded = rspice_core::netlist::IncludeProcessor::new(directory)
        .expand_content(source, origin)
        .map_err(|error| error.to_string())?;
    Ok(expanded.lines().any(|line| {
        let code = line.split(['$', ';']).next().unwrap_or_default().trim();
        !code.starts_with('*') && {
            let lower = code.to_ascii_lowercase();
            STATISTICAL_FUNCTIONS
                .iter()
                .any(|function| lower.contains(function))
        }
    }))
}

#[allow(clippy::too_many_arguments)]
fn monte_carlo(
    engine: &rspice_core::Engine,
    netlist: &rspice_core::Netlist,
    source: &str,
    origin: &Path,
    stop: f64,
    max_step: f64,
    startup: rspice_core::engine::TransientStartupMode,
    abort: &dyn AbortSignal,
) -> Result<Observation, CaseError> {
    use rspice_core::engine::{MonteCarloStudyConfig, MonteCarloVariationSource};
    let Some(card) = netlist.analyses.iter().find_map(|analysis| match analysis {
        rspice_core::netlist::AnalysisCommand::MonteCarlo(card) => Some(card.clone()),
        _ => None,
    }) else {
        return Err(harness_error("deck has no .mc card").into());
    };
    let mut study =
        MonteCarloStudyConfig::new(card.runs, card.seed.unwrap_or(1), vec!["trial".into()]);
    study.first_trial = card.first_trial;
    // The corpus decks state their variation as agauss/aunif expressions,
    // in the deck or in a file it includes. A deck that states none and whose
    // card names no parameter would have every `.param` perturbed by the
    // card's default spread (a supply `vcc` included); that is refused, never
    // run as if it were the design's variation.
    let statistical = deck_states_statistics(source, origin).map_err(|error| {
        harness_error(format!(
            "the deck's includes could not be resolved to find its statistics: {error}"
        ))
    })?;
    if statistical {
        study.variation_source = MonteCarloVariationSource::DeckStatistics;
    } else if card.params.is_empty() {
        return Err(harness_error(
            "the .mc card names no parameter and neither the deck nor any file it includes states a statistical function (agauss, aunif, gauss, unif, limit); refusing to perturb every .param",
        )
        .into());
    } else {
        study.distribution = match card.distribution {
            rspice_core::netlist::MonteCarloDistribution::Gaussian => {
                rspice_core::analysis::Distribution::Gaussian {
                    sigma: card.relative_spread,
                }
            }
            rspice_core::netlist::MonteCarloDistribution::Uniform => {
                rspice_core::analysis::Distribution::Uniform {
                    tolerance: card.relative_spread,
                }
            }
            rspice_core::netlist::MonteCarloDistribution::WorstCase => {
                rspice_core::analysis::Distribution::WorstCase {
                    tolerance: card.relative_spread,
                }
            }
        };
        study.parameter_filter = card.params.clone();
    }
    let trials: Mutex<BTreeMap<usize, Result<Observation, ErrorRecord>>> =
        Mutex::new(BTreeMap::new());
    let outcome = engine.run_monte_carlo_measurements_with_abort(
        netlist,
        &study,
        abort,
        |engine, trial, index, abort| {
            let outcome = run_transient(engine, trial, stop, max_step, startup, abort);
            let recorded = match &outcome {
                Ok(observation) => {
                    let mut observation = observation.clone();
                    observation.label = format!("trial {index}");
                    record_parameters(trial, &mut observation);
                    Ok(observation)
                }
                Err(error) => Err(core_error(error)),
            };
            trials.lock().expect("trial map").insert(index, recorded);
            outcome.map(|_| vec![0.0])
        },
    );
    if let Err(error) = outcome {
        return Err(core_error(&error).into());
    }
    let trials = trials.into_inner().expect("trial map");
    let mut observation = Observation {
        kind: "mc".into(),
        label: format!("{} trials", trials.len()),
        ..Default::default()
    };
    for (_, trial) in trials {
        match trial {
            Ok(trial) => observation.trials.push(trial),
            // A trial that failed is a failed case: the corpus asserts every trial.
            Err(error) => return Err(error.into()),
        }
    }
    Ok(observation)
}

fn step_points(
    engine: &rspice_core::Engine,
    netlist: &rspice_core::Netlist,
    stop: f64,
    max_step: f64,
    startup: rspice_core::engine::TransientStartupMode,
    abort: &dyn AbortSignal,
) -> Result<Observation, CaseError> {
    let limits = engine.config().resource_limits;
    let plan = rspice_core::execution::DeckPlan::from_netlist(netlist, &limits)
        .map_err(|error| harness_error(format!("deck plan: {error}")))?;
    let materializer = engine
        .prepare_deck_plan_materializer_with_abort(netlist, &plan, abort)
        .map_err(|error| harness_error(format!("materializer: {error}")))?;
    let mut observation = Observation {
        kind: "step".into(),
        label: format!("{} points", materializer.len()),
        ..Default::default()
    };
    for index in 0..materializer.len() {
        let run = materializer
            .materialize_run_with_abort(index, abort)
            .map_err(|error| harness_error(format!("materialize point {index}: {error}")))?;
        let (_, point_netlist, _, _) = run.into_parts();
        let mut point = run_transient(engine, &point_netlist, stop, max_step, startup, abort)
            .map_err(|error| core_error(&error))?;
        point.label = format!("point {index}");
        record_parameters(&point_netlist, &mut point);
        observation.trials.push(point);
    }
    Ok(observation)
}

fn checkpoint(
    engine: &rspice_core::Engine,
    netlist: &rspice_core::Netlist,
    request: &CaseRequest<'_>,
    stop: f64,
    max_step: f64,
    startup: rspice_core::engine::TransientStartupMode,
    abort: &dyn AbortSignal,
) -> Result<Observation, CaseError> {
    let times = request
        .design
        .analysis(request.analysis)
        .map(|analysis| analysis.checkpoint_at.clone())
        .unwrap_or_default();
    let (uninterrupted, checkpoints) = engine
        .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
            netlist, stop, max_step, startup, &times, abort,
        )
        .map_err(|error| core_error(&error))?;
    let mut observation = transient_observation(&uninterrupted);
    observation.kind = "ckpt".into();
    observation.label = "uninterrupted".into();
    for scheduled in checkpoints {
        let (resumed, _) = engine
            .run_tran_resume_with_abort(netlist, &scheduled.checkpoint, stop, max_step, abort)
            .map_err(|error| core_error(&error))?;
        let mut resumed = transient_observation(&resumed);
        resumed.label = format!("resumed@{:e}", scheduled.nominal_time);
        observation.trials.push(resumed);
    }
    Ok(observation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cards_strip_with_their_continuations_and_keep_the_title() {
        let deck =
            ".tran is the title\nR1 a 0 1k\n.tran 1n\n+ 10n\n.ac dec 1 1 10\nC1 a 0 1p\n.end\n";
        assert_eq!(
            strip_cards(deck),
            ".tran is the title\nR1 a 0 1k\nC1 a 0 1p\n.end\n"
        );
    }

    #[test]
    fn hdl_directives_parse_paths_aliases_and_modules() {
        let deck = "t\n.hdl \"../net/sdq.vams\"\n.va ../net/x.va xalias module=Top\nX1 a b sdq\n";
        let directives = hdl_directives(deck);
        assert_eq!(directives.len(), 2);
        assert_eq!(directives[0].path, "../net/sdq.vams");
        assert_eq!(directives[0].alias, None);
        assert_eq!(directives[1].alias.as_deref(), Some("xalias"));
        assert_eq!(directives[1].module.as_deref(), Some("Top"));
    }

    #[test]
    fn monte_carlo_statistics_are_found_in_included_files() {
        let dir = tempfile::tempdir().unwrap();
        let origin = dir.path().join("deck.cir");
        std::fs::write(
            dir.path().join("caps_mc.cir"),
            "* draws\n.param c0v={20f*(1+agauss(0,5e-3,1))}\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("caps_nominal.cir"),
            "* agauss( in a comment is not a statistic\n.param c0v=20f vcc=1.8\n",
        )
        .unwrap();
        let deck =
            |include: &str| format!("t\n.include {include}\n.mc 4 seed 3\n.tran 1n 1u\n.end\n");
        assert_eq!(
            deck_states_statistics(&deck("caps_mc.cir"), &origin),
            Ok(true)
        );
        assert_eq!(
            deck_states_statistics(&deck("caps_nominal.cir"), &origin),
            Ok(false)
        );
        assert!(deck_states_statistics(&deck("missing.cir"), &origin).is_err());
    }

    #[test]
    fn one_file_named_by_two_directives_seals_two_runtimes() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("pair.v"),
            "module tb(c);\n  output c;\n  reg c;\n  initial c = 1'b0;\nendmodule\n\
             module dut(c, q);\n  input c;\n  output q;\n  reg q;\n  initial q = 1'b0;\n  always @(posedge c) q <= ~q;\nendmodule\n",
        )
        .unwrap();
        let circuit = "t\n.hdl \"pair.v\" module=tb\n.hdl \"pair.v\" module=dut\nxtb c tb\nxdut c q dut\n.end\n";
        let (rewritten, runtimes) =
            seal_hdl(circuit, &dir.path().join("deck.cir"), "cxx").expect("two runtimes seal");
        let keys: Vec<&str> = runtimes
            .device_runtimes()
            .map(|runtime| runtime.source_key())
            .collect();
        assert_eq!(
            keys,
            [
                "__rspice_project__/mixed-corpus/cxx/dut/pair.v",
                "__rspice_project__/mixed-corpus/cxx/tb/pair.v"
            ]
        );
        assert!(
            rewritten.contains(".veriloga \"__rspice_project__/mixed-corpus/cxx/tb/pair.v\" tb\n")
                && rewritten
                    .contains(".veriloga \"__rspice_project__/mixed-corpus/cxx/dut/pair.v\" dut\n"),
            "{rewritten}"
        );
    }

    #[test]
    fn flattened_capability_refusals_are_recovered_from_their_display() {
        let text = "Circuit error: unsupported capability [analysis.mixed_signal.discrete_state] in 0d6c-uuid: AC analysis cannot run with 'xq'";
        let (token, analysis, detail) = prose_capability(text).unwrap();
        assert_eq!(token, "analysis.mixed_signal.discrete_state");
        assert_eq!(analysis.as_deref(), Some("0d6c-uuid"));
        assert_eq!(detail, "AC analysis cannot run with 'xq'");
        assert!(prose_capability("Circuit error: singular matrix").is_none());
        assert_eq!(
            first_location("sdq.vams:12:5: unsupported").as_deref(),
            Some("sdq.vams:12:5")
        );
        assert_eq!(first_location("no location here"), None);
    }

    #[test]
    fn study_cards_match_the_study_document_lowering() {
        let queue = rspice_simulation::manual_deck::build_manual_deck_queue(
            27.0,
            "* t\n.tran 0.1u 2m\n.ac dec 10 1 10meg\n.tf v(out) vin\n.end\n",
        )
        .unwrap();
        let cards: Vec<String> = queue
            .iter()
            .map(|task| study_card(&task.spec).unwrap())
            .collect();
        assert!(
            cards[0].to_ascii_lowercase().starts_with(".tran"),
            "{cards:?}"
        );
        assert!(
            cards[1].to_ascii_lowercase().starts_with(".ac"),
            "{cards:?}"
        );
        assert_eq!(cards[2], "", "{cards:?}");
    }
}

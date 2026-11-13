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
//! The deck's HDL sources (its `.hdl`/`.va`/`.vams`/`.verilog`/`.ahdl_include`
//! lines) are sealed by
//! [`rspice_simulation::netlist_preparation::seal_deck_hdl_sources`], the step
//! the application's prepared manual deck runs: each file is compiled with the
//! unified runtime compiler options, sealed as a prepared runtime under a
//! content key, and its line rewritten to the `.veriloga "<key>" <alias>` form
//! a sealed project runtime takes. The headless preparation refuses
//! file-backed HDL directives as unsealed external dependencies, so the sealed
//! runtimes travel beside the deck in the run input.
//!
//! # Families of transients
//!
//! A Monte Carlo transient, a `.STEP` transient and a checkpoint resume run on
//! the same route as one task per member: a transient per trial or point,
//! each naming its member of the deck's own `.MC` or `.STEP` card, and an
//! uninterrupted transient with a checkpoint schedule plus one resume per
//! checkpoint, each bound to the checkpoint that transient captured. The
//! members' results are gathered into the case's one family observation.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use rspice_core::abort_signal::AbortSignal;
use rspice_simulation::execution::{
    HeadlessPreparationError, HeadlessRunInput, HeadlessSourceResolver, HeadlessTaskRequest,
    PreparedRunAuthorization, prepare_headless_run,
};
use rspice_simulation::manual_deck::ManualDeckError;
use rspice_simulation::netlist_preparation::{
    DeckHdlRefusal, IncludeSearchChain, seal_deck_hdl_sources,
};
use rspice_simulation::preparation::QueuedAnalysis;
use rspice_simulation::results::SimulationResult;
use rspice_simulation::veriloga::PreparedVerilogARuntimeSet;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_vocabulary::product::AnalysisInstanceId;

use super::Route;
use super::case::{
    AssertCheck, CaseResult, ErrorRecord, HdlCheck, MeasureRecord, Observation, canonical_signal,
};
use super::design::{Design, analysis_kind, at_transient_state, deck_cards};
use super::measure::parse_expression;
use crate::suites::deadline::DeadlineAbort;

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
        Ok((origin, source)) => headless_case(request, kind, &origin, &source),
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
    let o5_path = request
        .design
        .o5_path_for(request.variant, request.analysis);
    let o5 = std::fs::read_to_string(&o5_path)
        .map_err(|error| format!("{}: {error}", o5_path.display()))?;
    let mut lines: Vec<String> = o5.lines().map(str::to_owned).collect();
    while lines
        .last()
        .is_some_and(|line| line.trim().is_empty() || line.trim().eq_ignore_ascii_case(".end"))
    {
        lines.pop();
    }
    // A card around a transient state has a held twin at the operating
    // point: the twin runs the card alone, without its state.
    let at_state = at_transient_state(request.analysis);
    for (word, card) in deck_cards(&deck) {
        if !at_state {
            lines.push(card);
        } else if word != ".tran" {
            lines.push(split_state(&card).map_or(card, |(card, _)| card));
        }
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
            if matches!(measure.kind.as_str(), "meas" | "checks") {
                // A `.MEASURE` result and a check are named by the deck, not
                // retained.
                continue;
            }
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
// Headless dispatch
// ---------------------------------------------------------------------------

/// A Verilog-A source the deck names that could not be sealed, recorded as
/// the engine's elaboration refusal of the same kind.
fn hdl_refusal(refusal: &DeckHdlRefusal) -> ErrorRecord {
    ErrorRecord {
        category: "elaboration".into(),
        token: Some(refusal.kind.as_str().into()),
        token_form: Some("structured".into()),
        module: Some(
            refusal
                .module
                .clone()
                .unwrap_or_else(|| refusal.file_name()),
        ),
        location: Some(refusal.location()),
        stage: Some("compile".into()),
        detail: refusal.detail.clone(),
        display: refusal.to_string(),
        ..Default::default()
    }
}

fn headless_case(request: &CaseRequest<'_>, kind: &str, origin: &Path, source: &str) -> CaseResult {
    let route = Route::compiled();
    let mut result = CaseResult::new(route.token(), "simulation");
    let cards: Vec<String> = deck_cards(source)
        .into_iter()
        .map(|(_, card)| card)
        .collect();
    let circuit = strip_cards(source);
    let (circuit, runtimes) =
        match seal_deck_hdl_sources(&circuit, Some(origin), &IncludeSearchChain::default()) {
            Ok(sealed) => (sealed.source, sealed.runtimes),
            Err(refusal) => return result.failed(hdl_refusal(&refusal)),
        };
    let lowered = match kind {
        "mc-tran" | "step-tran" | "ckpt" => {
            family_tasks(request, kind, &circuit, origin, source, &cards, &runtimes)
        }
        _ => headless_tasks(&circuit, origin, &cards, &runtimes)
            .map(|tasks| (tasks, Family::default())),
    };
    let (tasks, family) = match lowered {
        Ok(lowered) => lowered,
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
    // A family's members are independent tasks and run side by side, as
    // the engine's own Monte Carlo runs its trials; a dependent task starts
    // once every producer it binds has finished. Results keep task order.
    let concurrency = if family.kind.is_empty() {
        1
    } else {
        std::thread::available_parallelism().map_or(1, usize::from)
    };
    let mut pending = dispatch.into_tasks();
    let order: HashMap<AnalysisInstanceId, usize> = pending
        .iter()
        .enumerate()
        .map(|(index, task)| (task.instance_id(), index))
        .collect();
    let mut observations: Vec<Option<Observation>> = vec![None; order.len()];
    let mut finished: std::collections::HashSet<AnalysisInstanceId> =
        std::collections::HashSet::new();
    let mut artifacts = HashMap::new();
    let mut active: Vec<ActiveTask> = Vec::new();
    let fail = |active: &mut Vec<ActiveTask>, result: CaseResult, error: ErrorRecord| {
        for task in active.iter_mut() {
            task.runner.abort();
        }
        result.failed(error)
    };
    loop {
        while active.len() < concurrency {
            let Some(position) = pending.iter().position(|task| {
                task.dependency_bindings()
                    .iter()
                    .all(|binding| finished.contains(&binding.producer_instance_id()))
            }) else {
                break;
            };
            let task = pending.remove(position).expect("a ready task");
            let instance = task.instance_id();
            let label = labels.get(&instance).cloned().unwrap_or_default();
            let resolved = match task.resolve_dependency_artifacts(&artifacts) {
                Ok(resolved) => resolved,
                Err(error) => {
                    return fail(&mut active, result, harness_error(error.to_string()));
                }
            };
            let producer = match resolved.artifact_producer() {
                Ok(producer) => producer,
                Err(error) => {
                    return fail(&mut active, result, harness_error(error.to_string()));
                }
            };
            let mut runner = rspice_simulation::runner::SimulationRunner::new();
            if let Err(error) = runner.start_prepared(resolved, false) {
                return fail(&mut active, result, simulation_error(&error));
            }
            active.push(ActiveTask {
                runner,
                instance,
                label,
                producer,
            });
        }
        if active.is_empty() {
            break;
        }
        if abort.is_aborted() {
            for task in &mut active {
                task.runner.abort();
            }
        }
        let mut index = 0;
        while index < active.len() {
            let Some(outcome) = active[index].runner.poll_result() else {
                for line in active[index].runner.drain_engine_log() {
                    if result.warnings.len() < 64 {
                        result.warnings.push(line.message);
                    }
                }
                index += 1;
                continue;
            };
            let done = active.swap_remove(index);
            let simulated = match outcome {
                Ok(simulated) => simulated,
                Err(rspice_simulation::error::SimulationError::Aborted) if abort.is_aborted() => {
                    let error = ErrorRecord {
                        category: "timeout".into(),
                        detail: format!("budget of {} s spent", request.budget.as_secs()),
                        display: "case budget spent".into(),
                        stage: Some("execution".into()),
                        ..Default::default()
                    };
                    return fail(&mut active, result, error);
                }
                Err(error) => return fail(&mut active, result, simulation_error(&error)),
            };
            let checkpoints = done.runner.take_task_transient_checkpoints();
            match done.producer.capture_with_transient_checkpoints(
                &simulated,
                &checkpoints,
                &pending,
            ) {
                Ok(Some(artifact)) => {
                    artifacts.insert(done.instance, artifact);
                }
                Ok(None) => {}
                Err(error) => {
                    return fail(&mut active, result, harness_error(error.to_string()));
                }
            }
            observations[order[&done.instance]] = Some(observation(&simulated, &done.label));
            finished.insert(done.instance);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    result.results = observations.into_iter().flatten().collect();
    family.gather(&mut result.results);
    result
}

/// A dispatched task and what it needs when it finishes.
struct ActiveTask {
    runner: rspice_simulation::runner::SimulationRunner,
    instance: AnalysisInstanceId,
    label: String,
    producer: rspice_simulation::execution_artifact::PreparedArtifactProducer,
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
    // A card around a transient state lowers without its state (a typed
    // request carries none) and runs as authored, bound to the deck's
    // transient, as a study states it.
    let mut state_card = None;
    let lowered: Vec<String> = cards
        .iter()
        .map(|card| match split_state(card) {
            Some((plain, _)) => {
                state_card = Some(card.clone());
                plain
            }
            None => card.clone(),
        })
        .collect();
    let deck = format!("{}\n{}\n.end\n", body.join("\n"), lowered.join("\n"));
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
                ExecutionArtifactKind::TransientTrajectory
                | ExecutionArtifactKind::TransientCheckpoint => &transient,
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
    if let Some(card) = state_card {
        let mut states = tasks.iter_mut().filter(|task| {
            matches!(
                task.analysis.spec,
                AnalysisSpec::Ac { .. } | AnalysisSpec::Noise { .. }
            )
        });
        let (Some(task), None, [parent]) = (states.next(), states.next(), transient.as_slice())
        else {
            return Err(Box::new(harness_error(
                "a deck around a transient state carries one .tran card and one .ac or .noise card",
            )));
        };
        task.analysis.analysis_line = card;
        task.dependencies = vec![*parent];
    }
    Ok(tasks)
}

/// A card without its transient-state keyword, and the keyword: corpus decks
/// spell `ACTIMES=` and `CHECKPOINT=` as one token, `actimes=550u`.
pub fn split_state(card: &str) -> Option<(String, String)> {
    let words: Vec<&str> = card.split_whitespace().collect();
    let position = words.iter().position(|word| {
        let lower = word.to_ascii_lowercase();
        lower.starts_with("actimes=") || lower.starts_with("checkpoint=")
    })?;
    let keyword = words[position].to_owned();
    let plain = words
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != position)
        .map(|(_, word)| *word)
        .collect::<Vec<_>>()
        .join(" ");
    Some((plain, keyword))
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
         waveforms: &HashMap<String, rspice_simulation::results::SampledWaveform>| {
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
            measurements,
            ..
        } => {
            observation.kind = "tran".into();
            observation.axis = time.clone();
            waveforms_into(&mut observation, waveforms);
            record_measures(measurements, &mut observation);
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
    if let Some(checks) = result.checks() {
        record_hdl_checks(&checks.hdl, &mut observation);
        record_assertions(&checks.assertions, &mut observation);
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

// ---------------------------------------------------------------------------
// Families of transients
// ---------------------------------------------------------------------------

fn dialect(name: &str) -> Option<rspice_core::config::SpiceDialect> {
    use rspice_core::config::SpiceDialect;
    match name.to_ascii_lowercase().as_str() {
        "best-available" => Some(SpiceDialect::BestAvailable),
        "ngspice" => Some(SpiceDialect::Ngspice),
        "xyce" => Some(SpiceDialect::Xyce),
        _ => None,
    }
}

/// Record the HDL's severity-task records on `observation`, each site by its
/// file name and line.
fn record_hdl_checks(
    records: &[rspice_core::config::HdlOutputRecord],
    observation: &mut Observation,
) {
    use rspice_core::config::{HdlSeverity, HdlStream};
    for record in records {
        let HdlStream::Diagnostic { severity } = &record.stream else {
            continue;
        };
        let severity = match severity {
            HdlSeverity::Info => "info",
            HdlSeverity::Warning => "warning",
            HdlSeverity::Error => "error",
            HdlSeverity::Fatal => "fatal",
        };
        let file = Path::new(&record.origin.file)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&record.origin.file);
        observation.hdl_checks.push(HdlCheck {
            severity: severity.to_owned(),
            site: format!("{file}:{}", record.origin.line),
            at: record.seconds,
        });
    }
}

/// Record the deck's `.ASSERT` reports on `observation`. Reports under one
/// name merge in order: a vector file's checks report per output bit and per
/// mismatch, all under the file's name.
fn record_assertions(
    reports: &[rspice_core::analysis::AssertionReport],
    observation: &mut Observation,
) {
    for report in reports {
        let name = match &report.instance {
            Some(instance) => format!("{}[{instance}]", report.name),
            None => report.name.clone(),
        };
        let check = observation
            .assertions
            .entry(name.to_ascii_lowercase())
            .or_insert_with(|| AssertCheck {
                starts: Vec::new(),
                extremes: Vec::new(),
                error: None,
            });
        check.starts.extend(
            report
                .violations
                .iter()
                .map(|violation| violation.start.unwrap_or(0.0)),
        );
        check.extremes.extend(
            report
                .violations
                .iter()
                .map(|violation| violation.extreme.filter(|value| value.is_finite())),
        );
        if check.error.is_none() {
            check.error.clone_from(&report.error);
        }
    }
}

/// Record the deck's `.MEASURE` results on `observation`.
fn record_measures(measurements: &[rspice_core::MeasureResult], observation: &mut Observation) {
    for measurement in measurements {
        observation.measures.insert(
            measurement.name.to_ascii_lowercase(),
            MeasureRecord::of(measurement.value, measurement.error.as_deref()),
        );
    }
}

/// Whether the deck, with every file it includes, states a statistical
/// function outside comments. An include that cannot be resolved is an
/// error, not "no statistics".
pub fn deck_states_statistics(source: &str, origin: &Path) -> Result<bool, String> {
    let directory = origin.parent().unwrap_or(Path::new("."));
    let expanded = rspice_core::netlist::IncludeProcessor::new(directory)
        .expand_content(source, origin)
        .map_err(|error| error.to_string())?;
    Ok(rspice_simulation::transient_family::deck_states_statistics(
        &expanded,
    ))
}

/// How a family's member observations gather into the case's one family
/// observation: the trials of a Monte Carlo transient, the points of a
/// `.STEP` transient, or an uninterrupted transient and its resumes.
#[derive(Default)]
struct Family {
    /// `mc`, `step` or `ckpt`; empty for a case that is not a family.
    kind: &'static str,
    /// Each member task's label, in member order.
    members: Vec<String>,
    /// Each trial's or point's resolved top-level `.param` values, as
    /// `param:` scalars, by label: the draw the trial makes or the coordinate
    /// the point takes, materialized without solving. The member the run
    /// solved must agree with them.
    parameters: HashMap<String, BTreeMap<String, f64>>,
}

impl Family {
    fn gather(mut self, results: &mut Vec<Observation>) {
        if self.kind.is_empty() {
            return;
        }
        let mut by_label: HashMap<String, Observation> = std::mem::take(results)
            .into_iter()
            .map(|observation| (observation.label.clone(), observation))
            .collect();
        let mut members: Vec<Observation> = self
            .members
            .iter()
            .filter_map(|label| by_label.remove(label))
            .collect();
        for member in &mut members {
            if let Some(parameters) = self.parameters.remove(&member.label) {
                member.scalars.extend(parameters);
            }
        }
        if self.kind == "ckpt" {
            let mut members = members.into_iter();
            if let Some(mut uninterrupted) = members.next() {
                uninterrupted.kind = "ckpt".into();
                uninterrupted.trials = members.collect();
                results.push(uninterrupted);
            }
            return;
        }
        let noun = if self.kind == "mc" {
            "trials"
        } else {
            "points"
        };
        results.push(Observation {
            kind: self.kind.into(),
            label: format!("{} {noun}", members.len()),
            trials: members,
            ..Default::default()
        });
    }
}

/// Lower a Monte Carlo transient, a `.STEP` transient or a checkpoint resume
/// into one prepared task per member.
///
/// The deck's `.TRAN` card lowers as every other card does. A trial or point
/// task carries it and the deck's own `.MC` or `.STEP` card, and names its
/// member; the member count and each member's parameter values are read off
/// the deck by the engine's own sampler and sweep materializer, without
/// solving. A checkpoint case is the transient with the design's checkpoint
/// schedule and one resume per scheduled time, each bound to it.
#[allow(clippy::too_many_arguments)]
fn family_tasks(
    request: &CaseRequest<'_>,
    kind: &str,
    circuit: &str,
    origin: &Path,
    source: &str,
    cards: &[String],
    runtimes: &PreparedVerilogARuntimeSet,
) -> Result<(Vec<HeadlessTaskRequest>, Family), CaseError> {
    use rspice_simulation::transient_family;
    let family_word = match kind {
        "mc-tran" => Some(".mc"),
        "step-tran" => Some(".step"),
        _ => None,
    };
    let is_family_card = |card: &str| {
        family_word.is_some_and(|word| {
            card.split_whitespace()
                .next()
                .is_some_and(|first| first.eq_ignore_ascii_case(word))
        })
    };
    let family_card = cards.iter().find(|card| is_family_card(card)).cloned();
    let rest: Vec<String> = cards
        .iter()
        .filter(|card| !is_family_card(card))
        .cloned()
        .collect();
    let base = match <[HeadlessTaskRequest; 1]>::try_from(headless_tasks(
        circuit, origin, &rest, runtimes,
    )?) {
        Ok([base]) if matches!(base.analysis.spec, AnalysisSpec::Transient { .. }) => base,
        _ => {
            return Err(harness_error(format!(
                "a {kind} deck carries one .tran card besides its family card"
            ))
            .into());
        }
    };
    let member_task = |label: String| {
        let mut task = base.clone();
        task.instance_id = AnalysisInstanceId::new();
        task.label = label;
        task
    };
    if kind == "ckpt" {
        let times = request
            .design
            .analysis(request.analysis)
            .map(|analysis| analysis.checkpoint_at.clone())
            .unwrap_or_default();
        let mut producer = member_task("uninterrupted".into());
        producer.analysis.spec_options.tran_checkpoint = Some(
            rspice_simulation::transient_checkpoint::TransientCheckpointRequest {
                times: times.clone(),
                resume: None,
            },
        );
        let mut family = Family {
            kind: "ckpt",
            members: vec![producer.label.clone()],
            ..Default::default()
        };
        let mut tasks = Vec::with_capacity(times.len() + 1);
        for time in times {
            let mut resume = member_task(format!("resumed@{time:e}"));
            resume.analysis.spec_options.resume_at = Some(time);
            resume.dependencies = vec![producer.instance_id];
            family.members.push(resume.label.clone());
            tasks.push(resume);
        }
        tasks.insert(0, producer);
        return Ok((tasks, family));
    }
    let Some(family_card) = family_card else {
        return Err(harness_error(format!("a {kind} deck carries its family card")).into());
    };
    let netlist = rspice_core::Netlist::parse_with_path(source, origin)
        .map_err(|error| harness_error(format!("the deck could not be read: {error}")))?;
    let mut config = rspice_core::engine::SimulationConfig::default();
    if let Some(dialect) = dialect(&request.design.file.dialect) {
        config.spice_dialect = dialect;
    }
    let engine = rspice_core::Engine::new(config);
    let members = if kind == "mc-tran" {
        // The corpus decks state their variation as agauss/aunif expressions,
        // in the deck or in a file it includes.
        let statistical = deck_states_statistics(source, origin).map_err(|error| {
            harness_error(format!(
                "the deck's includes could not be resolved to find its statistics: {error}"
            ))
        })?;
        transient_family::monte_carlo_members(&netlist, statistical)
    } else {
        transient_family::step_members(&netlist, &engine.config().resource_limits)
    }
    .map_err(harness_error)?;
    let drawn =
        transient_family::member_parameters(&engine, &netlist, &members).map_err(harness_error)?;
    let parameters = members
        .iter()
        .zip(drawn)
        .map(|(member, values)| {
            let scalars = values
                .into_iter()
                .map(|(name, value)| (format!("param:{name}"), value))
                .collect();
            (member.member.label(), scalars)
        })
        .collect();
    let members: Vec<_> = members.into_iter().map(|member| member.member).collect();
    let mut family = Family {
        kind: if kind == "mc-tran" { "mc" } else { "step" },
        members: Vec::with_capacity(members.len()),
        parameters,
    };
    let tasks = members
        .into_iter()
        .map(|member| {
            let mut task = member_task(member.label());
            task.analysis.analysis_line = format!("{}\n{family_card}", base.analysis.analysis_line);
            task.analysis.spec_options.member = Some(member);
            family.members.push(task.label.clone());
            task
        })
        .collect();
    Ok((tasks, family))
}

#[cfg(test)]
mod prepared_route;

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
    fn a_state_keyword_splits_from_its_card() {
        assert_eq!(
            split_state(".ac dec 10 10 100meg ACTIMES=550u"),
            Some((".ac dec 10 10 100meg".into(), "ACTIMES=550u".into()))
        );
        assert_eq!(
            split_state(".noise v(out) v1 dec 3 1k 1meg checkpoint=\"s.chk\""),
            Some((
                ".noise v(out) v1 dec 3 1k 1meg".into(),
                "checkpoint=\"s.chk\"".into()
            ))
        );
        assert_eq!(split_state(".ac dec 10 10 100meg"), None);
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

    /// A file of connect rules and connect modules alone is a connection
    /// library: it seals, and `.options connectrules=` selects a rule from it
    /// on the headless path as the engine selects one. A file with neither
    /// modules nor rules is still refused.
    #[test]
    fn a_connect_only_source_contributes_its_rules_on_the_headless_path() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("deck")).unwrap();
        std::fs::write(
            dir.path().join("drv.vams"),
            "module drv(p, q);\n  inout p;\n  electrical p;\n  output q;\n  reg q;\n  initial q = 1;\n  analog I(p) <+ 0;\nendmodule\n",
        )
        .unwrap();
        let mut rules = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
            .iter()
            .map(|(_, source)| *source)
            .collect::<String>();
        rules.push_str(
            "\nconnectrules Low; connect d2a #(.vsup(1.0)); endconnectrules\n\
             connectrules High; connect d2a #(.vsup(5.0)); endconnectrules\n",
        );
        std::fs::write(dir.path().join("rules.vams"), rules).unwrap();
        std::fs::write(dir.path().join("empty.vams"), "// no modules, no rules\n").unwrap();
        let design = Design {
            dir: dir.path().to_path_buf(),
            slug: "cxx".into(),
            file: toml::from_str(
                "schema = \"rspice-mixed-design/1\"\nid = \"C99\"\ntitle = \"t\"\nexit_phases = [\"P1\"]\ndialect = \"d\"\n\
                 [[variant]]\nname = \"mx\"\nfamilies = [\"spice\"]\ntoday = \"runs\"\n\
                 [[analysis]]\nid = \"tran\"\nvariants = [\"mx\"]\noracles = [\"O6\"]\ntimeout_s = 60\nphase = \"P1\"\n",
            )
            .unwrap(),
            tolerance: toml::from_str("schema = \"rspice-mixed-tolerance/1\"\ntime_precision = 1e-12\n")
                .unwrap(),
        };
        let request = CaseRequest {
            design: &design,
            variant: "mx",
            analysis: "tran",
            o5: false,
            budget: Duration::from_secs(60),
        };
        let origin = dir.path().join("deck").join("mx.tran.cir");
        let deck = |rule: &str| {
            format!(
                "connect-only rules\n.hdl \"../drv.vams\"\n.hdl \"../rules.vams\"\nV1 p 0 1\nX1 p q drv\nR1 q 0 1meg\n\
                 .options connectrules={rule} connectrules_source=rules\n.tran 0.2n 2n\n.end\n"
            )
        };
        for (rule, supply) in [("Low", 1.0), ("High", 5.0)] {
            let result = headless_case(&request, "tran", &origin, &deck(rule));
            assert_eq!(result.outcome, "completed", "{rule}: {:?}", result.error);
            let q = result.results[0].real.get("v(q)").unwrap_or_else(|| {
                panic!("{rule}: no v(q) in {:?}", result.results[0].real.keys())
            });
            let last = *q.last().unwrap();
            assert!(
                (last - supply).abs() < 1e-3 * supply,
                "{rule}: the rule's d2a drives q to {supply} V, saw {last}"
            );
        }
        let empty = headless_case(
            &request,
            "op",
            &origin,
            "t\n.hdl \"../empty.vams\"\n.op\n.end\n",
        );
        assert_eq!(empty.outcome, "error", "{:?}", empty.error);
        let error = empty.error.expect("a refusal carries its error");
        assert_eq!(error.token.as_deref(), Some("unknown_module"), "{error:?}");
        assert_eq!(error.module.as_deref(), Some("empty.vams"), "{error:?}");
    }

    #[test]
    fn flattened_capability_refusals_are_recovered_from_their_display() {
        let text = "Circuit error: unsupported capability [analysis.mixed_signal.discrete_state] in 0d6c-uuid: AC analysis cannot run with 'xq'";
        let (token, analysis, detail) = prose_capability(text).unwrap();
        assert_eq!(token, "analysis.mixed_signal.discrete_state");
        assert_eq!(analysis.as_deref(), Some("0d6c-uuid"));
        assert_eq!(detail, "AC analysis cannot run with 'xq'");
        assert!(prose_capability("Circuit error: singular matrix").is_none());
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

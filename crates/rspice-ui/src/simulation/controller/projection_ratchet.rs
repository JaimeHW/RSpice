//! Every editable field of every analysis draft must reach the engine.
//!
//! This studio has grown the same defect three separate times: a control that
//! is rendered, validated and persisted, and then read by nothing. First it
//! was solver options pinned to literals, then create-only registries, then
//! thirteen per-analysis fields whose values stopped at their `*Config`.
//! Each was found by hand, months apart.
//!
//! The check here is mechanical instead. For every analysis kind it takes the
//! default draft, serializes it, and perturbs one field at a time. A field
//! that changes nothing about the *engine-facing projection* — the typed
//! [`AnalysisSpec`], the SPICE directive, and the runner's execution options —
//! is a field the engine cannot see, and the test fails naming it.
//!
//! The projection deliberately excludes the intermediate `*Config` types. A
//! dead control usually does reach its config; that is exactly how the last
//! thirteen hid.
//!
//! Two details make the result trustworthy rather than merely green:
//!
//! - **Gated fields are tried un-gated.** A field that only matters when some
//!   checkbox is on would look dead against the default draft alone, so each
//!   field is judged against three starting points: the default, every boolean
//!   forced on, and every boolean forced off.
//! - **A field that cannot be perturbed fails too.** If no candidate value
//!   deserializes — an enum spelling the pool does not know, say — the field is
//!   reported as unperturbable rather than quietly counted as live. The
//!   fix is to teach [`ENUM_CANDIDATES`] the spelling, never to ignore it.
//!
//! ## Adding to [`INERT_FIELDS`]
//!
//! An entry is a claim that a field legitimately cannot move the projection,
//! with the reason. It is not a place to park a control that should work.
//! Prefer deleting the control, or wiring it, over adding a line here.

use serde_json::{Map, Value};

use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::model_library::ModelLibraryManager;
use crate::workbench::app_state::AppState;

use super::SimulationController;

/// Fields that cannot move the engine-facing projection, and why.
///
/// Each entry is `(kind, dotted field path, reason)`.
const INERT_FIELDS: &[(AnalysisKind, &str, &str)] = &[
    // Touchstone export happens after the solve, from the retained result. It
    // is a real contract with its own tests; it is simply not part of what the
    // engine is asked to compute.
    (
        AnalysisKind::SParameter,
        "touchstone_export",
        "post-run export of the retained result, not an engine input",
    ),
    (
        AnalysisKind::SParameter,
        "touchstone_version",
        "post-run export format, not an engine input",
    ),
    // Six `Corner` entries stood here, every one of them a `run_set.*` path.
    // They are gone rather than re-justified: the Corner draft no longer holds
    // a run set, so those paths are not draft fields and this ratchet — which
    // perturbs draft fields — is no longer the thing that judges them. The run
    // set is the plan's, and it is judged by `run_set::validate`, by the two
    // page/prepared parity derivations, and by the participation ratchet below.
];

/// Fields that name something in the user's own circuit, and the fixture name
/// this ratchet gives them.
///
/// A draft opens without an output node, an input source, or a tone: a default
/// cannot know one, and inventing one is how the studio used to hand out names
/// the design did not carry. The consequence here is that such a draft refuses
/// to build a spec at all, and every *other* field of that analysis then
/// projects as the same refusal — the ratchet would report a whole form as
/// dead because one name is missing.
///
/// This is the same fixture role [`with_bound_corner_supply`] and
/// [`with_sealed_process_library`] play: supply the context an empty project
/// cannot, then judge the fields. The named fields are still judged on their
/// own, from the fixture value rather than from empty.
const DESIGN_NAMED_FIELDS: &[(AnalysisKind, &str, &str)] = &[
    (AnalysisKind::Noise, "output", "n_out"),
    (AnalysisKind::Noise, "input", "VSRC"),
    (AnalysisKind::TransferFunction, "input_source", "VSRC"),
    (
        AnalysisKind::TransferFunction,
        "output_expression",
        "V(n_out)",
    ),
    (AnalysisKind::Pss, "tone_sources", "VSRC"),
];

/// Extra string values to try, for fields whose type is an enum.
///
/// Serde spells enums however the type asked to be spelled, and a test cannot
/// enumerate variants without reflection. When a new enum appears, the test
/// fails as *unperturbable* and the spelling is added here.
const ENUM_CANDIDATES: &[&str] = &[
    "decade",
    "octave",
    "linear",
    "explicit_frequency_list",
    "Decade",
    "Octave",
    "Linear",
    "enabled",
    "disabled",
    "none",
    "all",
    "top20",
    "top50",
    "all_contributors",
    "summary_only",
    "cartesian",
    "zipped",
    "filtered",
    "Cartesian",
    "Zipped",
    "Filtered",
    "conditional",
    "adaptive",
    "nested",
    "Conditional",
    "Adaptive",
    "Nested",
    "Placed",
    "Entered",
];

/// One valid element for a list-valued field that defaults to empty.
///
/// A list the user has not populated yet cannot be perturbed by duplicating
/// its first entry, so the shape of an entry has to be supplied. This is a
/// fixture for building a legal value — the field must still move the
/// projection on its own merit.
const ARRAY_SEEDS: &[(AnalysisKind, &str, &str)] = &[
    (
        AnalysisKind::HarmonicBalance,
        "additional_tones",
        r#"{"frequency": "2G", "harmonics": "3", "name": "tone2", "source": ""}"#,
    ),
    // The `Corner` composition seeds that stood here are gone with the run set
    // they addressed: a composition is declared on the plan, not on the draft
    // this ratchet perturbs.
];

/// Object-valued fields that default to absent.
///
/// Empty, and legitimately so: the only entries were the Corner draft's
/// adaptive-policy composition, which is now the plan's. The list stays because
/// the shape it declares is how the next such field is seeded.
const OBJECT_SEEDS: &[(AnalysisKind, &str, &str)] = &[];

/// The engine-facing projection of one draft.
///
/// Three artifacts, because a field can legitimately reach any one of them:
/// the typed specification the runner dispatches on, the directive written
/// into a generated deck, and the execution options a service runner takes.
/// Errors are part of the projection — a value that makes the plan refuse to
/// build is still a value the engine saw.
///
/// The order mirrors the production dispatch in `analysis_plan`: the draft's
/// own typed spec first, and the legacy slot projection only where a kind has
/// no draft-shaped builder.
/// The `AppState` a draft is projected against.
///
/// A periodic consumer reads its prerequisite PSS out of the legacy singleton
/// slot, which opens without a tone because only the design can name one. Left
/// that way, every PAC/PNOISE/PXF/PSTB field projects as the same missing-tone
/// refusal and the whole form reads as dead. This is the prerequisite's
/// configuration, not a field under test — the same fixture role
/// [`with_sealed_process_library`] plays. A Pss draft overwrites it, so a
/// perturbation of the tone list itself is still judged.
pub(super) fn engine_facing_state(draft: &AnalysisDraft) -> AppState {
    let mut state = AppState::default();
    state.sim_setup.pss.ensure_initialized();
    if state.sim_setup.pss.tone_sources.trim().is_empty() {
        state.sim_setup.pss.tone_sources = "VSRC".to_owned();
    }
    state.sim_setup.apply_analysis_draft_projection(draft);
    state
}

/// The default draft body for `kind`, with the fixture context an empty
/// project cannot invent already filled in.
pub(super) fn fixture_body(
    kind: AnalysisKind,
    draft: &AnalysisDraft,
) -> Option<Map<String, Value>> {
    let body = draft_body(draft)?;
    Some(with_design_named_fields(kind, body))
}

/// The default draft for `kind`, carrying that fixture context.
pub(super) fn fixture_draft(kind: AnalysisKind) -> AnalysisDraft {
    let draft = AnalysisDraft::for_kind(kind);
    match fixture_body(kind, &draft).and_then(|body| rebuild(&draft, body)) {
        Some(prepared) => prepared,
        None => draft,
    }
}

fn projection(kind: AnalysisKind, draft: &AnalysisDraft) -> String {
    let controller = SimulationController::new();
    let state = engine_facing_state(draft);

    let spec = match controller.build_manifest_preview_spec(&state, draft) {
        Ok(Some(spec)) => Ok(spec),
        Ok(None) => controller.build_analysis_spec_for_index(&state, kind.legacy_index()),
        Err(error) => Err(error),
    };

    let spec = match spec {
        Ok(spec) => spec,
        Err(error) => return format!("spec-error: {error}"),
    };

    let command = controller.analysis_spec_to_spice_line(&state, &spec);
    let options = with_sealed_process_library(|sealed| {
        controller.analysis_spec_execution_options(&state, &spec, sealed)
    });

    format!("{spec:?}\u{1f}{command:?}\u{1f}{options:?}")
}

/// A sealed model library that defines every process section the corner run
/// set can name.
///
/// Without one, `corner_model_bindings` refuses before the run configuration
/// is assembled, and *every* corner field whose only route to the engine is
/// through `SpecExecutionOptions` — the resolved point list among them —
/// projects as the same refusal whatever it is set to. The ratchet would then
/// report those fields as dead when they are merely unreachable from an empty
/// project. Sealing is done once per thread because it is the expensive part;
/// the sections carry different `KP` values so a binding that resolves to the
/// wrong one is still visible in the projection.
fn with_sealed_process_library<T>(
    build: impl FnOnce(&crate::state::model_library::SealedModelExecutionSources) -> Result<T, String>,
) -> Result<T, String> {
    use std::cell::RefCell;

    thread_local! {
        static SEALED: RefCell<Option<
            Result<crate::state::model_library::SealedModelExecutionSources, String>,
        >> = const { RefCell::new(None) };
    }

    SEALED.with(|cell| {
        let mut slot = cell.borrow_mut();
        let sealed = slot.get_or_insert_with(|| {
            let mut manager = ModelLibraryManager::new();
            let mut library = String::new();
            for (section, kp) in [
                ("TT", "1e-3"),
                ("SS", "0.8e-3"),
                ("FF", "1.2e-3"),
                ("SF", "0.9e-3"),
                ("FS", "1.1e-3"),
            ] {
                library.push_str(&format!(
                    ".lib {section}\n.model nch NMOS (LEVEL=1 KP={kp})\n.endl {section}\n"
                ));
            }
            manager.load_library_bytes("ratchet-process.lib", library.into_bytes(), None)?;
            manager.seal_execution_sources()
        });
        match sealed {
            Ok(sealed) => build(sealed),
            Err(error) => Err(error.clone()),
        }
    })
}

/// Candidate replacements for one JSON value.
///
/// A field is live if *any* candidate moves the projection, so the pool only
/// has to contain one value the field actually distinguishes.
fn perturbations(kind: AnalysisKind, path: &str, current: &Value) -> Vec<Value> {
    let mut candidates = match current {
        Value::Bool(value) => vec![Value::Bool(!value)],
        Value::Number(_) => vec![
            Value::from(2u64),
            Value::from(3u64),
            Value::from(1.5f64),
            Value::from(0u64),
        ],
        Value::String(_) => {
            let mut strings = vec![
                Value::String("7.5k".to_owned()),
                Value::String("2".to_owned()),
                Value::String("NPRB".to_owned()),
                Value::String(String::new()),
            ];
            strings.extend(
                ENUM_CANDIDATES
                    .iter()
                    .map(|candidate| Value::String((*candidate).to_owned())),
            );
            strings
        }
        Value::Array(items) => {
            let mut arrays = vec![Value::Array(Vec::new())];
            let seed = items.first().cloned().or_else(|| {
                ARRAY_SEEDS
                    .iter()
                    .find(|(seed_kind, seed_path, _)| *seed_kind == kind && *seed_path == path)
                    .and_then(|(_, _, literal)| serde_json::from_str(literal).ok())
            });
            if let Some(seed) = seed {
                let mut extended = items.clone();
                extended.push(seed);
                arrays.push(Value::Array(extended));
            }
            arrays
        }
        Value::Null | Value::Object(_) => OBJECT_SEEDS
            .iter()
            .filter(|(seed_kind, seed_path, _)| *seed_kind == kind && *seed_path == path)
            .filter_map(|(_, _, literal)| serde_json::from_str(literal).ok())
            .collect(),
    };
    candidates.retain(|candidate| candidate != current);
    candidates
}

/// The draft is adjacently tagged: `{"kind": "...", "draft": {...}}`. Only the
/// content object holds editable fields; the tag names the analysis.
const DRAFT_CONTENT_KEY: &str = "draft";

/// The draft's own fields, unwrapped from the adjacently-tagged envelope.
fn draft_body(draft: &AnalysisDraft) -> Option<Map<String, Value>> {
    serde_json::to_value(draft)
        .ok()?
        .as_object()?
        .get(DRAFT_CONTENT_KEY)?
        .as_object()
        .cloned()
}

/// Rebuild a draft from a mutated body, preserving the tag.
///
/// `prepare_after_restore` is the production path a project load takes; without
/// it the lazy-initialization sentinel stays false and the first `to_config`
/// would reset the draft to defaults, hiding every perturbation.
fn rebuild(draft: &AnalysisDraft, body: Map<String, Value>) -> Option<AnalysisDraft> {
    let mut envelope = serde_json::to_value(draft).ok()?.as_object()?.clone();
    envelope.insert(DRAFT_CONTENT_KEY.to_owned(), Value::Object(body));
    let mut rebuilt: AnalysisDraft = serde_json::from_value(Value::Object(envelope)).ok()?;
    rebuilt.prepare_after_restore();
    Some(rebuilt)
}

/// Every scalar leaf in the body, as dotted paths.
///
/// Nested objects are walked because several drafts embed a shared sweep
/// editor as a sub-object; its fields are as configurable as any other.
fn leaf_paths(body: &Map<String, Value>, prefix: &str, out: &mut Vec<String>) {
    for (key, value) in body {
        let path = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        match value {
            Value::Object(nested) => leaf_paths(nested, &path, out),
            _ => out.push(path),
        }
    }
}

fn value_at<'a>(body: &'a Map<String, Value>, path: &str) -> Option<&'a Value> {
    let mut current = body.get(path.split('.').next()?)?;
    for segment in path.split('.').skip(1) {
        current = current.as_object()?.get(segment)?;
    }
    Some(current)
}

fn with_value_at(body: &Map<String, Value>, path: &str, replacement: Value) -> Map<String, Value> {
    let mut updated = body.clone();
    let segments: Vec<&str> = path.split('.').collect();
    fn assign(target: &mut Map<String, Value>, segments: &[&str], replacement: Value) {
        let Some((head, rest)) = segments.split_first() else {
            return;
        };
        if rest.is_empty() {
            target.insert((*head).to_owned(), replacement);
        } else if let Some(Value::Object(nested)) = target.get_mut(*head) {
            assign(nested, rest, replacement);
        }
    }
    assign(&mut updated, &segments, replacement);
    updated
}

/// Force every boolean in the body to one setting, un-gating conditional
/// fields so they can be judged at all.
/// Fill in the [`DESIGN_NAMED_FIELDS`] this kind declares.
fn with_design_named_fields(kind: AnalysisKind, body: Map<String, Value>) -> Map<String, Value> {
    DESIGN_NAMED_FIELDS
        .iter()
        .filter(|(named_kind, _, _)| *named_kind == kind)
        .fold(body, |body, (_, path, name)| {
            with_value_at(&body, path, Value::String((*name).to_owned()))
        })
}

fn with_all_booleans(body: &Map<String, Value>, setting: bool) -> Map<String, Value> {
    body.iter()
        .map(|(key, value)| {
            let value = match value {
                Value::Bool(_) => Value::Bool(setting),
                Value::Object(nested) => Value::Object(with_all_booleans(nested, setting)),
                other => other.clone(),
            };
            (key.clone(), value)
        })
        .collect()
}

// `with_bound_corner_supply` and `with_corner_composition_fixture` stood here.
// Both reached into a corner draft body's `run_set` to supply a supply
// authority and a composition mode. A corner draft body has neither now — the
// plan declares them — so both had become no-ops that returned their argument
// unchanged, which is worse than absent: a fixture that silently does nothing
// makes the fields it was supposed to enable look legitimately inert.

/// One outcome for a field, against one starting body.
enum FieldOutcome {
    Moved,
    Inert,
    Unperturbable,
}

fn judge(
    kind: AnalysisKind,
    draft: &AnalysisDraft,
    body: &Map<String, Value>,
    path: &str,
) -> FieldOutcome {
    let Some(current) = value_at(body, path) else {
        return FieldOutcome::Unperturbable;
    };
    let Some(baseline_draft) = rebuild(draft, body.clone()) else {
        return FieldOutcome::Unperturbable;
    };
    let baseline = projection(kind, &baseline_draft);

    let mut any_deserialized = false;
    for candidate in perturbations(kind, path, current) {
        let Some(mutated) = rebuild(draft, with_value_at(body, path, candidate)) else {
            continue;
        };
        any_deserialized = true;
        if projection(kind, &mutated) != baseline {
            return FieldOutcome::Moved;
        }
    }

    if any_deserialized {
        FieldOutcome::Inert
    } else {
        FieldOutcome::Unperturbable
    }
}

/// Re-judge a field with one sibling control moved first.
///
/// Some fields only reach the engine in a particular mode — an explicit
/// frequency list matters only under a list sweep, a corner's base analysis
/// only once the process set resolves. Rather than hand-enumerate those
/// modes, this walks the siblings and retries the field under each of them.
/// It runs only for a field that already looks inert, so the cost is bounded
/// by the number of failures rather than by the size of the catalog.
///
/// The search starts from each of the caller's base bodies, which is what
/// lets a field behind *two* gates be reached: the all-off body supplies one,
/// the sibling under test supplies the other.
fn judge_under_sibling_modes(
    kind: AnalysisKind,
    draft: &AnalysisDraft,
    bodies: &[Map<String, Value>],
    path: &str,
    siblings: &[String],
) -> Option<FieldOutcome> {
    for body in bodies {
        for sibling in siblings {
            if sibling == path {
                continue;
            }
            let Some(current) = value_at(body, sibling) else {
                continue;
            };
            // Lists are included because a whole axis can be the gate: a
            // corner run set whose process dimension is empty is what makes
            // the rest of its configuration reachable at all.
            if !matches!(current, Value::Bool(_) | Value::String(_) | Value::Array(_)) {
                continue;
            }
            for candidate in perturbations(kind, sibling, current) {
                let gated = with_value_at(body, sibling, candidate);
                if rebuild(draft, gated.clone()).is_none() {
                    continue;
                }
                if matches!(judge(kind, draft, &gated, path), FieldOutcome::Moved) {
                    return Some(FieldOutcome::Moved);
                }
            }
        }
    }
    None
}

#[test]
fn every_draft_field_moves_the_engine_facing_projection() {
    let mut inert: Vec<String> = Vec::new();
    let mut unperturbable: Vec<String> = Vec::new();

    for kind in AnalysisKind::ALL {
        let draft = AnalysisDraft::for_kind(kind);
        let Some(body) = fixture_body(kind, &draft) else {
            continue;
        };

        // Three starting points, so a field gated behind a checkbox is judged
        // in the state where it can actually matter.
        let bodies = vec![
            body.clone(),
            with_all_booleans(&body, true),
            with_all_booleans(&body, false),
        ];

        let mut paths = Vec::new();
        leaf_paths(&body, "", &mut paths);
        let paths_for_escalation = paths.clone();

        for path in &paths {
            let path = path.clone();
            if INERT_FIELDS
                .iter()
                .any(|(inert_kind, inert_path, _)| *inert_kind == kind && *inert_path == path)
            {
                continue;
            }

            let mut outcomes: Vec<FieldOutcome> = bodies
                .iter()
                .map(|body| judge(kind, &draft, body, &path))
                .collect();

            if outcomes
                .iter()
                .any(|outcome| matches!(outcome, FieldOutcome::Moved))
            {
                continue;
            }
            if let Some(escalated) =
                judge_under_sibling_modes(kind, &draft, &bodies, &path, &paths_for_escalation)
            {
                outcomes.push(escalated);
                continue;
            }
            if outcomes
                .iter()
                .all(|outcome| matches!(outcome, FieldOutcome::Unperturbable))
            {
                unperturbable.push(format!("{} · {path}", kind.label()));
            } else {
                inert.push(format!("{} · {path}", kind.label()));
            }
        }
    }

    assert!(
        inert.is_empty() && unperturbable.is_empty(),
        "editor fields that reach no engine-facing projection — wire them, delete them, or \
         justify them in INERT_FIELDS:\n  {}\n\nfields the ratchet could not construct an \
         alternate value for — teach ENUM_CANDIDATES their spelling, do not skip them:\n  {}",
        if inert.is_empty() {
            "(none)".to_owned()
        } else {
            inert.join("\n  ")
        },
        if unperturbable.is_empty() {
            "(none)".to_owned()
        } else {
            unperturbable.join("\n  ")
        },
    );
}

/// A per-analysis advanced option is not a draft field either.
///
/// Like participation, it lives on the [`crate::simulation::plan::
/// AnalysisInstance`] rather than in a draft body, so perturbing draft fields
/// cannot reach it — and it must not be parked in [`INERT_FIELDS`], because it
/// reaches the engine by the most direct route any editor has: a second
/// `.OPTIONS` card in the task's own deck.
///
/// What it moves is that deck and the task's configuration identity, so that
/// is what this judges, through the production queue rather than a fixture.
/// Two other ratchets cover the rest of the same seam: `numeric_override`'s
/// own proves each option moves the *resolved* engine configuration, and
/// `execution::snapshot` proves each one survives the splice into a deck the
/// engine's parser reads. This one proves the plan actually carries them
/// there.
#[test]
fn every_authored_advanced_option_moves_the_prepared_task_identity() {
    use crate::simulation::plan::{AnalysisInstance, AnalysisKind, NumericOverrideOption};

    let mut state = super::prepared_run::tests::runnable_state();
    let id = state
        .sim_setup
        .enabled_analysis_instances()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
        .map(AnalysisInstance::id)
        .expect("a fresh plan holds one enabled transient");

    let digest_of = |state: &AppState| -> Vec<u8> {
        let controller = SimulationController::new();
        let plan = controller
            .build_analysis_plan(state)
            .unwrap_or_else(|errors| panic!("the fixture plan compiles: {}", errors.join("; ")));
        let sealed = state
            .model_library_manager
            .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
            .expect("the fixture library seals");
        controller
            .build_queue_from_plan(state, &plan, &sealed)
            .unwrap_or_else(|errors| panic!("the fixture queue builds: {}", errors.join("; ")))
            .iter()
            .flat_map(|task| task.config_digest().as_bytes().to_vec())
            .collect()
    };

    let baseline = digest_of(&state);
    let mut inert = Vec::new();

    for option in NumericOverrideOption::applicable_to(AnalysisKind::Transient) {
        use crate::simulation::plan::OverrideValueKind as K;
        let authored = match option.value_kind() {
            K::PositiveReal | K::NonNegativeReal => "3.25e-7",
            K::IterationCount => "37",
            K::Flag => "on",
            K::Method => "GEAR2",
            K::Damping => "BANKROSE",
            K::Solver => "KLU",
        };
        let mut record = crate::simulation::plan::AnalysisNumericOverride::default();
        record
            .set(AnalysisKind::Transient, option, authored)
            .unwrap_or_else(|error| panic!("{} is authorable: {error}", option.key()));
        state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("stable plan")
            .set_numeric_override(id, Some(record))
            .unwrap_or_else(|error| panic!("{} commits: {error}", option.key()));

        if digest_of(&state) == baseline {
            inert.push(option.key());
        }
    }

    assert!(
        inert.is_empty(),
        "advanced options the prepared queue cannot see — they are authored, persisted and \
         reported, and then run the same solve as an analysis that stated nothing:\n  {}",
        inert.join("\n  ")
    );
}

/// Run-set participation is not a draft field, and this is where it is judged.
///
/// The ratchet above walks [`AnalysisDraft`] bodies, and participation is not
/// one: it lives on the [`crate::simulation::plan::AnalysisInstance`], because
/// it is a property every kind has and none of them spells differently. So it
/// cannot be covered by perturbing a draft, and it must not be parked in
/// [`INERT_FIELDS`] either — that list is for fields that legitimately reach no
/// engine-facing projection, and participation reaches the most consequential
/// one there is.
///
/// What it moves is the *set* of dispatched tasks rather than any one task's
/// spec, directive or options. So the projection judged here is the dispatched
/// task count, and every variant has to move it away from the others: three
/// settings that produced the same queue would be exactly the dead control this
/// module exists to catch.
///
/// The fixture is `prepared_run`'s own runnable state rather than a whole
/// application: this module sits below the shell, and a test that reached up
/// for one would be the layer inversion the module-order gate exists to stop.
#[test]
fn every_run_set_participation_moves_the_dispatched_task_set() {
    use crate::simulation::plan::{AnalysisInstance, AnalysisKind};
    use crate::simulation::run_set::{AnalysisRunAt, RunSetDimensionKind, RunSetPoint};

    let mut state = super::prepared_run::tests::runnable_state();
    // One axis, and a reference the axis actually declares, so all three
    // settings resolve rather than one of them refusing for its own reasons.
    for dimension in &mut state.sim_setup.run_set.dimensions {
        dimension.enabled = dimension.kind == RunSetDimensionKind::Temperature;
    }
    state
        .sim_setup
        .set_reference_pvt(crate::product::ProcessCorner::TT, 25.0)
        .expect("25 °C is a valid reference temperature");

    let points: Vec<String> = crate::simulation::run_set::resolve(&state.sim_setup.run_set)
        .expect("the fixture space expands exactly")
        .iter()
        .map(RunSetPoint::point_key)
        .collect();
    assert!(
        points.len() > 2,
        "the fixture space must be able to distinguish all, one, and some"
    );
    let id = state
        .sim_setup
        .enabled_analysis_instances()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
        .map(AnalysisInstance::id)
        .expect("a fresh plan holds one enabled transient");

    let mut controller = SimulationController::new();
    let mut counts = Vec::new();
    for run_at in [
        AnalysisRunAt::AllPoints,
        AnalysisRunAt::NominalPoint,
        AnalysisRunAt::SelectedPoints(points[..2].to_vec()),
    ] {
        let label = run_at.label();
        state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("stable plan")
            .set_run_at(id, run_at)
            .unwrap_or_else(|error| panic!("{label} commits: {error}"));
        let prepared = controller
            .prepare_run_set_for_preflight(&state)
            .unwrap_or_else(|error| panic!("{label} prepares: {error}"))
            .task_count;
        counts.push((label, prepared));
    }

    assert_eq!(
        counts,
        vec![
            ("All run-set points", points.len()),
            ("Nominal point only", 1),
            ("Selected points", 2),
        ],
        "each participation must dispatch exactly the points it declares"
    );
}

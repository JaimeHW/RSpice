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
    // Run-set editing bookkeeping. These carry the shape of the edit, not the
    // shape of the run: they let the editor detect stale snapshots and hand
    // out dimension identities.
    (
        AnalysisKind::Corner,
        "run_set.revision",
        "monotonic edit counter used to reject stale snapshots",
    ),
    (
        AnalysisKind::Corner,
        "run_set.sequence",
        "next dimension identity to hand out; never dispatched",
    ),
    // Cost model for the pre-dispatch forecast. The two coefficients are not
    // inert for the same reason, so they do not share a justification.
    //
    // `cost_per_point_ms` scales `RunSetForecast::cost_ms`, which is displayed
    // and nothing else: no budget is declared against elapsed time, so no
    // value of it can refuse a run.
    //
    // `bytes_per_point` does multiply into an admission gate —
    // `run_set::validate` compares `task_count * bytes_per_point` against the
    // declared `maximum_storage_bytes` and refuses with RUNSET-STORAGE-BUDGET.
    // It reads as inert here only because the fixture's task count is small
    // enough that none of the candidate values the ratchet perturbs it to
    // crosses that limit; a value that did would move the projection, and this
    // entry would have to go rather than be widened.
    (
        AnalysisKind::Corner,
        "run_set.budgets.cost_per_point_ms",
        "scales the displayed elapsed-time forecast; no budget is declared against it",
    ),
    (
        AnalysisKind::Corner,
        "run_set.budgets.bytes_per_point",
        "feeds the storage admission gate, but no candidate value crosses the fixture's limit",
    ),
    (
        AnalysisKind::Corner,
        "run_set.preview",
        "frozen output of the last validate-and-preview, not an input to the next run",
    ),
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
    // One point of the default PVT space, keyed the way
    // `RunSetPoint::point_key` builds it: the sorted value identities of
    // every enabled dimension, joined by `+`. An exclusion only bites on a
    // point that exists, so an invented key would prove nothing.
    (
        AnalysisKind::Corner,
        "run_set.composition.excluded_points",
        r#""dimension-process-value-001+dimension-supply-value-001+dimension-temperature-value-001""#,
    ),
    (
        AnalysisKind::Corner,
        "run_set.composition.upstream_dimension_ids",
        r#""dimension-temperature""#,
    ),
];

const OBJECT_SEEDS: &[(AnalysisKind, &str, &str)] = &[
    (
        AnalysisKind::Corner,
        "run_set.composition.adaptive_policy",
        r#"{"id":"space-filling-v1","objective":"minimize:error","seed":7,"bounds":"{\"dimension-temperature\":[-40,125]}","stop_rule":"maximum-proposals","maximum_proposals":4}"#,
    ),
    (
        AnalysisKind::Corner,
        "run_set.composition.adaptive_policy",
        r#"{"id":"space-filling-v2","objective":"minimize:error","seed":11,"bounds":"{\"dimension-temperature\":[-20,85]}","stop_rule":"maximum-proposals","maximum_proposals":6}"#,
    ),
];

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
pub(super) fn fixture_body(kind: AnalysisKind, draft: &AnalysisDraft) -> Option<Map<String, Value>> {
    let mut body = draft_body(draft)?;
    if kind == AnalysisKind::Corner {
        body = with_bound_corner_supply(body);
    }
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

/// Give the corner ratchet a reachable supply contract. Production defaults
/// deliberately do not guess a supply source, but this test must start from a
/// valid point so edits such as point exclusions can reach the executor.
fn with_bound_corner_supply(mut body: Map<String, Value>) -> Map<String, Value> {
    let Some(Value::Object(run_set)) = body.get_mut("run_set") else {
        return body;
    };
    let Some(Value::Array(dimensions)) = run_set.get_mut("dimensions") else {
        return body;
    };
    for dimension in dimensions {
        let Some(dimension) = dimension.as_object_mut() else {
            continue;
        };
        if dimension.get("kind").and_then(Value::as_str) == Some("supply") {
            dimension.insert(
                "source".to_owned(),
                Value::String("netlist-source:VDD".to_owned()),
            );
        }
    }
    body
}

fn with_corner_composition_fixture(mut body: Map<String, Value>, mode: &str) -> Map<String, Value> {
    let Some(Value::Object(run_set)) = body.get_mut("run_set") else {
        return body;
    };
    let Some(Value::Object(composition)) = run_set.get_mut("composition") else {
        return body;
    };
    composition.insert("mode".to_owned(), Value::String(mode.to_owned()));
    match mode {
        "conditional" => {
            composition.insert(
                "predicate".to_owned(),
                Value::String("dimension-temperature == 25".to_owned()),
            );
            composition.insert(
                "upstream_dimension_ids".to_owned(),
                Value::Array(vec![Value::String("dimension-temperature".to_owned())]),
            );
        }
        "adaptive" => {
            composition.insert(
                "adaptive_policy".to_owned(),
                serde_json::from_str(OBJECT_SEEDS[0].2).expect("adaptive fixture is valid JSON"),
            );
        }
        _ => {}
    }
    body
}

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
        let mut bodies = vec![
            body.clone(),
            with_all_booleans(&body, true),
            with_all_booleans(&body, false),
        ];
        if kind == AnalysisKind::Corner {
            bodies.push(with_corner_composition_fixture(body.clone(), "conditional"));
            bodies.push(with_corner_composition_fixture(body.clone(), "adaptive"));
            bodies.push(with_corner_composition_fixture(body.clone(), "nested"));
        }

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

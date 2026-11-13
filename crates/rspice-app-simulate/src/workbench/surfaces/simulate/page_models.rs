//! Models & sections.
//!
//! The model closure this plan binds against. The page reads the loaded
//! libraries and states, per library, which corner section a run will resolve
//! to and whether its source is pinned well enough to be reproducible.
//!
//! Authoring belongs to Models & PDKs; this page is the plan's read of that
//! closure plus the one binding decision the plan owns — the corner.

use egui::Ui;

use crate::state::model_library::{ModelSourceAuthority, SimulationPlanModelBinding, short_digest};
use crate::ui::accessibility::{counted, plural_suffix};
use crate::ui::icons::Icon;
use crate::ui::widgets::{Button, IconButton, select};
use crate::workbench::app_state::Model;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::ports::{ModelGateFact, QualificationGate};
use crate::workbench::simulate::{SimulateFeature, invalidate_preflight, record_plan_refusal};

use super::page_kit::{
    RowPress, Tone, card, card_body, card_head_row, card_note, card_row, card_with_head, cell_ui,
    ledger_head, ledger_row, ledger_row_cells, paint_text, paint_toned_text, rule_row,
};
use crate::workbench::app_state::model::DefinitionRow;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

const CLOSURE_COLUMNS: [f32; 7] = [0.05, 0.18, 0.15, 0.12, 0.16, 0.20, 0.14];

/// How many of this library's names another library of the plan also provides,
/// by the manager's own index.
///
/// Never recomputed here —
/// [`Model::definition_index`](crate::workbench::app_state::Model::definition_index)
/// is the models workspace's one answer to "who provides this name", and the
/// studio states it rather than deriving a second opinion that would
/// eventually disagree with the workspace an operator repairs it in. A provider
/// the plan does not bind is not one its run reads, so it contests nothing here.
fn contested_names(
    definitions: &[DefinitionRow],
    bindings: &[SimulationPlanModelBinding],
    library: &str,
) -> usize {
    definitions
        .iter()
        .filter(|row| {
            let bound = |provider: &String| {
                bindings
                    .iter()
                    .any(|binding| binding.library_name.eq_ignore_ascii_case(provider))
            };
            row.contested()
                && row.providers.iter().filter(|name| bound(name)).count() > 1
                && row
                    .providers
                    .iter()
                    .any(|provider| provider.eq_ignore_ascii_case(library))
        })
        .count()
}
const GATE_COLUMNS: [f32; 3] = [0.30, 0.46, 0.24];

/// How many models with findings the gate card lists before it stops.
///
/// The gate is decided over every vector of every model in the closure. A plan
/// page that printed them all would be a log, not a read; the remainder is
/// reported as a count and never dropped.
const GATE_FINDING_ROWS: usize = 8;

pub(super) fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    closure(ui, cx);
    qualification_gate(ui, cx);
    card_row(
        ui,
        cx,
        |ui, cx| binding_policy(ui, cx.model),
        |ui, cx| reproducibility(ui, cx.model),
    );
}

fn closure(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    // The scan the models workspace takes, taken here too, because this page is
    // reachable without ever opening that workspace and a drift report nobody
    // requested reads exactly like a closure with no drift. Event-driven, not
    // per frame: it rehashes every pinned source, and `needs_scan` is false
    // until the project revision or the catalog actually moves.
    if cx.model.source_drift_needs_scan() {
        cx.model.scan_source_drift();
    }
    let bindings = cx.model.sim_setup.model_bindings.clone();
    // One name cannot be contested by one library, so a closure that loaded a
    // single library skips the index entirely. That check is free and the index
    // walks every model of every loaded library.
    let definitions: Vec<DefinitionRow> = if cx.model.model_library_manager.library_count() > 1 {
        cx.model.definition_index()
    } else {
        Vec::new()
    };
    let invalid = bindings
        .iter()
        .filter(|binding| {
            cx.model
                .model_library_manager
                .validate_simulation_plan_bindings(std::slice::from_ref(binding))
                .is_err()
        })
        .count();
    let drifted = bindings
        .iter()
        .filter(|binding| {
            !cx.model
                .source_drift_findings(&binding.library_name)
                .is_empty()
        })
        .count();
    let contested = bindings
        .iter()
        .map(|binding| contested_names(&definitions, &bindings, &binding.library_name))
        .sum::<usize>();
    // Ordered by how badly each stops a reproducible run: an invalid binding
    // cannot resolve at all, a contested name fails closed at bind time, and
    // drift still runs but against bytes nobody accepted.
    let (status, tone) = if invalid > 0 {
        (
            format!(
                "{invalid} stale or invalid binding{}",
                plural_suffix(invalid)
            ),
            Tone::Error,
        )
    } else if contested > 0 {
        (
            format!(
                "{contested} contested definition{}",
                plural_suffix(contested)
            ),
            Tone::Error,
        )
    } else if drifted > 0 {
        (
            format!(
                "{drifted} librar{} drifted from its pin",
                if drifted == 1 { "y" } else { "ies" }
            ),
            Tone::Warn,
        )
    } else {
        (
            format!(
                "{} binding{}",
                bindings.len(),
                plural_suffix(bindings.len())
            ),
            Tone::Neutral,
        )
    };
    // Resolved before the card is drawn because the control that offers these
    // belongs in the card head, and the head is drawn first.
    let bound_names = bindings
        .iter()
        .map(|binding| binding.library_name.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>();
    let available = cx
        .model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| {
            library.source_authority.has_execution_source()
                && !bound_names.contains(&library.name.to_ascii_lowercase())
        })
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    let mut requested = None;
    let mut attach = None;
    card_with_head(
        ui,
        |ui| {
            card_head_row(ui, "Model closure", Some((status.as_str(), tone)), |ui| {
                // One control naming what can be attached, in the head where
                // the card's commands live. The strip this replaces was a raw
                // label at the card's left border with the libraries beside it
                // as buttons: it sat four columns left of every inset row on
                // the card, and it grew a row per library until it truncated
                // itself at four and said "+N more".
                let choices = available
                    .iter()
                    .map(|name| super::page_kit::PopupChoice {
                        label: name.clone(),
                        unavailable: None,
                    })
                    .collect::<Vec<_>>();
                if let Some(index) = super::page_kit::command_popup(
                    ui,
                    "simulation.models.attach-library",
                    Button::new("Add library\u{2026}")
                        .icon(Icon::Add)
                        .enabled(!available.is_empty()),
                    "Every executable library is already bound to this plan.",
                    &choices,
                ) && let Some(name) = available.get(index)
                {
                    attach = Some(ModelBindingAction::Attach(name.clone()));
                }
            });
        },
        |ui| {
            ledger_head(
                ui,
                &CLOSURE_COLUMNS,
                &[
                    "Order",
                    "Library",
                    "Provides",
                    "Digest",
                    "Source",
                    "Corner section",
                    "Actions",
                ],
            );
            if bindings.is_empty() {
                ledger_row(
                    ui,
                    &CLOSURE_COLUMNS,
                    &[
                        ("—", Tone::Neutral),
                        ("No libraries attached.", Tone::Neutral),
                        ("", Tone::Neutral),
                        ("", Tone::Neutral),
                        ("", Tone::Neutral),
                        ("", Tone::Neutral),
                        ("", Tone::Neutral),
                    ],
                    false,
                    RowPress::Ignored,
                );
            }
            for (index, binding) in bindings.iter().enumerate() {
                let library = cx
                    .model
                    .model_library_manager
                    .get_library(&binding.library_name);
                let corners = corner_options(cx.model, &binding.library_name);
                let (rect, cells) = ledger_row_cells(ui, &CLOSURE_COLUMNS);
                let t = Tokens::get(ui.ctx());
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom(),
                    egui::Stroke::new(1.0, t.color.border),
                );
                let font = theme::mono(tokens::FS_0, FontWeight::Regular);
                paint_text(
                    ui,
                    cells[0].shrink2(egui::vec2(8.0, 0.0)),
                    &(index + 1).to_string(),
                    font.clone(),
                    t.color.text_dim,
                );
                paint_toned_text(
                    ui,
                    cells[1].shrink2(egui::vec2(8.0, 0.0)),
                    &binding.library_name,
                    font.clone(),
                    if library.is_some() {
                        Tone::Text
                    } else {
                        Tone::Error
                    },
                );
                // What this library actually offers a reference, and how much of it
                // more than one library claims. A contested name fails closed at
                // bind time, so it belongs beside the count it spoils rather than
                // in a separate advisory the reader has to correlate.
                let contested_here =
                    contested_names(&definitions, &bindings, &binding.library_name);
                let provides = match (library, contested_here) {
                    (None, _) => "—".to_owned(),
                    (Some(library), 0) => format!(
                        "{} model{}",
                        library.models.len(),
                        plural_suffix(library.models.len())
                    ),
                    (Some(library), contested) => format!(
                        "{} model{} · {contested} contested",
                        library.models.len(),
                        plural_suffix(library.models.len())
                    ),
                };
                paint_toned_text(
                    ui,
                    cells[2].shrink2(egui::vec2(8.0, 0.0)),
                    &provides,
                    font.clone(),
                    if contested_here > 0 {
                        Tone::Error
                    } else {
                        Tone::Dim
                    },
                );
                // The digest the binding pinned, and whether the bytes still hash
                // to it. Drift is the models workspace's finding, read here rather
                // than recomputed: this page cannot repair it, so it must not be
                // able to disagree about whether there is anything to repair.
                let drift = cx.model.source_drift_findings(&binding.library_name);
                paint_toned_text(
                    ui,
                    cells[3].shrink2(egui::vec2(8.0, 0.0)),
                    &short_digest(&binding.source_digest.to_string()),
                    font.clone(),
                    if drift.is_empty() {
                        Tone::Neutral
                    } else {
                        Tone::Warn
                    },
                );
                if !drift.is_empty() {
                    let detail = drift
                        .iter()
                        .map(|finding| {
                            format!(
                                "{} · pinned {} → now {}",
                                finding.path.display(),
                                finding.pinned,
                                finding.on_disk.as_deref().unwrap_or("unreadable"),
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    ui.interact(
                    cells[3],
                    ui.id().with(("closure-drift", &binding.library_name)),
                    egui::Sense::hover(),
                )
                .on_hover_text(format!(
                    "{} pinned source{} no longer hash to the digest this binding accepted:\n{detail}",
                    drift.len(),
                    plural_suffix(drift.len()),
                ));
                }
                let validation = cx
                    .model
                    .model_library_manager
                    .validate_simulation_plan_bindings(std::slice::from_ref(binding));
                let (source_text, source_tone) = match (library, validation) {
                    (_, Err(_)) => ("stale".to_owned(), Tone::Error),
                    (Some(library), Ok(())) => (
                        format!(
                            "{} · {} pinned",
                            match library.source_authority {
                                ModelSourceAuthority::External => "external",
                                _ => "project",
                            },
                            library.source_closure.len()
                        ),
                        Tone::Dim,
                    ),
                    (None, Ok(())) => ("missing".to_owned(), Tone::Error),
                };
                paint_toned_text(
                    ui,
                    cells[4].shrink2(egui::vec2(8.0, 0.0)),
                    &source_text,
                    font,
                    source_tone,
                );
                if library.is_none() || corners.is_empty() {
                    paint_text(
                        ui,
                        cells[5].shrink2(egui::vec2(8.0, 0.0)),
                        if library.is_some() {
                            "reference-process fallback"
                        } else {
                            "library unavailable"
                        },
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_faint,
                    );
                } else {
                    let mut choices = Vec::with_capacity(corners.len() + 1);
                    choices.push("Automatic (reference process)".to_owned());
                    choices.extend(corners);
                    let selected = binding
                        .selected_corner
                        .clone()
                        .unwrap_or_else(|| choices[0].clone());
                    let cell_rect = cells[5].shrink2(egui::vec2(6.0, 4.0));
                    let mut cell = cell_ui(ui, cell_rect);
                    if let Some(choice) = select(
                        &mut cell,
                        &format!("simulation.models.corner.{}", binding.library_name),
                        "Corner section",
                        &selected,
                        &choices,
                        cell_rect.width(),
                    ) {
                        requested = Some(ModelBindingAction::SetCorner {
                            index,
                            corner: (choice != 0).then(|| choices[choice].clone()),
                        });
                    }
                }
                let action_rect = cells[6].shrink2(egui::vec2(3.0, 4.0));
                let mut actions = cell_ui(ui, action_rect);
                actions.horizontal(|ui| {
                    if IconButton::new(Icon::ChevronUp)
                        .enabled(index > 0)
                        .tooltip("Move earlier in model precedence")
                        .show(ui)
                        .clicked()
                    {
                        requested = Some(ModelBindingAction::MoveUp(index));
                    }
                    if IconButton::new(Icon::ChevronDown)
                        .enabled(index + 1 < bindings.len())
                        .tooltip("Move later in model precedence")
                        .show(ui)
                        .clicked()
                    {
                        requested = Some(ModelBindingAction::MoveDown(index));
                    }
                    if IconButton::new(Icon::Trash)
                        .tooltip("Remove from this simulation plan")
                        .show(ui)
                        .clicked()
                    {
                        requested = Some(ModelBindingAction::Remove(index));
                    }
                });
            }

            card_note(
                ui,
                "This ordered list is owned by the active simulation plan. Earlier libraries have \
             higher precedence. Every entry is pinned to the source digest accepted when it was \
             attached; replacement or refresh requires an explicit review before another run.",
            );
        },
    );

    if let Some(action) = requested.or(attach) {
        apply_model_binding_action(cx, action);
    }
}

pub enum ModelBindingAction {
    SetCorner {
        index: usize,
        corner: Option<String>,
    },
    MoveUp(usize),
    MoveDown(usize),
    Remove(usize),
    Attach(String),
}

fn corner_options(model: &Model, library: &str) -> Vec<String> {
    model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|candidate| candidate.name == library)
        .map(|candidate| {
            let mut names: Vec<String> = candidate.corners.keys().cloned().collect();
            names.sort();
            names
        })
        .unwrap_or_default()
}

pub fn apply_model_binding_action(cx: &mut Cx<'_, SimulateFeature>, action: ModelBindingAction) {
    let mut candidate = cx.model.sim_setup.clone();
    let detail = match action {
        ModelBindingAction::SetCorner { index, corner } => {
            let Some(binding) = candidate.model_bindings.get_mut(index) else {
                return record_binding_refusal(
                    cx,
                    "The model binding changed before the edit committed.",
                );
            };
            binding.selected_corner = corner;
            format!("Changed model section for {}.", binding.library_name)
        }
        ModelBindingAction::MoveUp(index)
            if index > 0 && index < candidate.model_bindings.len() =>
        {
            candidate.model_bindings.swap(index - 1, index);
            "Changed model-library precedence.".to_owned()
        }
        ModelBindingAction::MoveDown(index) if index + 1 < candidate.model_bindings.len() => {
            candidate.model_bindings.swap(index, index + 1);
            "Changed model-library precedence.".to_owned()
        }
        ModelBindingAction::Remove(index) if index < candidate.model_bindings.len() => {
            let removed = candidate.model_bindings.remove(index);
            format!(
                "Removed model library {} from the plan.",
                removed.library_name
            )
        }
        ModelBindingAction::Attach(name) => {
            let binding = match cx
                .model
                .model_library_manager
                .simulation_plan_binding(&name)
            {
                Ok(binding) => binding,
                Err(error) => return record_binding_refusal(cx, &error),
            };
            candidate.model_bindings.push(binding);
            format!("Attached model library {name} to the plan.")
        }
        _ => return,
    };
    if let Err(error) = cx
        .model
        .model_library_manager
        .validate_simulation_plan_bindings(&candidate.model_bindings)
    {
        return record_binding_refusal(cx, &error);
    }
    match candidate.commit_active_plan_configuration_change(detail) {
        Ok(receipt) => {
            cx.model.sim_setup = candidate;
            invalidate_preflight(cx);
            cx.model.record_plan_receipt(receipt.status_line());
        }
        Err(error) => record_binding_refusal(cx, &error.to_string()),
    }
}

fn record_binding_refusal(cx: &mut Cx<'_, SimulateFeature>, error: &str) {
    record_plan_refusal(cx, error);
}

/// One model's contribution to the release gate.
///
/// A borrow of the gate fact the Models workspace publishes, carried across
/// field for field. It exists so the aggregation below can be stated — and
/// tested — without this page reaching into a qualification record, which is
/// the shape of mistake that once had this page reporting its own gate: a
/// different definition count, a different passing total, and findings
/// against models the catalog recorded as clean.
#[derive(Debug, Clone, Copy)]
pub struct ModelGate<'a> {
    pub library: &'a str,
    pub model: &'a str,
    pub vectors: usize,
    pub evidenced_vectors: usize,
    pub passing_vectors: usize,
    pub open_dispositions: usize,
    /// The workspace could not read this model's gate at all.
    pub unreadable: bool,
    pub gate: QualificationGate,
}

impl<'a> From<&'a ModelGateFact> for ModelGate<'a> {
    fn from(fact: &'a ModelGateFact) -> Self {
        Self {
            library: &fact.library,
            model: &fact.model,
            vectors: fact.vectors,
            evidenced_vectors: fact.evidenced_vectors,
            passing_vectors: fact.passing_vectors,
            open_dispositions: fact.open_dispositions,
            unreadable: fact.unreadable,
            gate: fact.gate,
        }
    }
}

impl ModelGate<'_> {
    /// The worst thing the gate has to say about this model, or nothing.
    ///
    /// One row per model. Which vector failed, and why, is the qualification
    /// page's subject; the plan only needs to know the model is not clear.
    fn finding(&self) -> Option<GateFinding> {
        // Asked before every test below, all of which read an absence of
        // project evidence as this model's failure to produce it. An
        // engine-owned card is the simulator's own equation default and is not
        // a subject of the source-owned gate, so the plan has nothing to
        // report about it — an exemption stated as a finding is still a row in
        // a table of things that are wrong.
        if self.gate == QualificationGate::EngineOwned {
            return None;
        }
        let (rank, finding, tone) = if self.unreadable {
            (0, "gate could not be read".to_owned(), Tone::Error)
        } else if self.evidenced_vectors > self.passing_vectors {
            (
                1,
                format!(
                    "{} failing",
                    counted(
                        self.evidenced_vectors - self.passing_vectors,
                        "evidenced vector",
                        "evidenced vectors"
                    )
                ),
                Tone::Error,
            )
        } else if self.open_dispositions > 0 {
            (
                2,
                format!(
                    "{} open",
                    counted(self.open_dispositions, "disposition", "dispositions")
                ),
                Tone::Warn,
            )
        } else if self.vectors > self.evidenced_vectors {
            (
                3,
                format!(
                    "{} without results",
                    counted(self.vectors - self.evidenced_vectors, "vector", "vectors")
                ),
                Tone::Warn,
            )
        } else if self.gate == QualificationGate::Unqualified {
            (4, "no qualification suite".to_owned(), Tone::Warn)
        } else if self.gate != QualificationGate::Qualified {
            // Platform parity and measurement correlation also hold the gate,
            // and neither reason is carried on the summary. Saying which one
            // it was would be a guess; the qualification page holds it.
            (5, "held for review".to_owned(), Tone::Warn)
        } else {
            return None;
        };
        Some(GateFinding {
            rank,
            model: format!("{}/{}", self.library, self.model),
            finding,
            state: self.gate.label(),
            tone,
        })
    }
}

/// A model the gate has something to say about.
#[derive(Debug, Clone)]
pub struct GateFinding {
    /// Severity, so the rows the card has room for are the ones that matter.
    rank: u8,
    pub model: String,
    pub finding: String,
    pub state: &'static str,
    pub tone: Tone,
}

/// The release gate, summed over the closure.
#[derive(Debug, Default)]
pub struct GateReading {
    pub models: usize,
    /// Vectors a *readable* gate records as passing.
    pub passing: usize,
    /// Every vector the closure's qualification suites declare.
    pub vectors: usize,
    pub open_dispositions: usize,
    /// Models whose gate could not be read.
    pub unreadable: usize,
    /// Models the source-owned gate does not govern at all.
    ///
    /// Counted rather than skipped: a closure of nothing but engine-owned
    /// cards has no findings, and "no findings" said about no subjects is how
    /// the card would come to sign off a release on an empty gate.
    pub engine_owned: usize,
    pub findings: Vec<GateFinding>,
}

/// Sum the gate across the closure.
///
/// A model whose gate could not be read contributes its vectors to the total
/// and nothing to the passing count. The alternative — skipping it, or taking
/// the outcomes recorded against a source identity that no longer validates —
/// reports a clean release on evidence nobody can open, and a gate that fails
/// open is worse than no gate.
pub fn gate_reading<'a>(models: impl IntoIterator<Item = ModelGate<'a>>) -> GateReading {
    let mut reading = GateReading::default();
    for model in models {
        reading.models += 1;
        reading.vectors += model.vectors;
        reading.open_dispositions += model.open_dispositions;
        if model.gate == QualificationGate::EngineOwned {
            reading.engine_owned += 1;
        }
        if model.unreadable {
            reading.unreadable += 1;
        } else {
            reading.passing += model.passing_vectors;
        }
        if let Some(finding) = model.finding() {
            reading.findings.push(finding);
        }
    }
    reading.findings.sort_by(|left, right| {
        left.rank
            .cmp(&right.rank)
            .then_with(|| left.model.cmp(&right.model))
    });
    reading
}

/// The card's status line: open dispositions, and whether any gate was
/// unreadable at all. A clear gate states nothing.
pub fn gate_status(reading: &GateReading) -> Option<(String, Tone)> {
    if reading.models == 0 {
        return Some(("no models to gate".to_owned(), Tone::Neutral));
    }
    // Every model here is one the gate does not govern, so there is no verdict
    // to report — neither a clean one nor a failing one.
    if reading.engine_owned == reading.models {
        return Some((
            format!(
                "{} · nothing under the release gate",
                counted(
                    reading.engine_owned,
                    "engine-owned model",
                    "engine-owned models"
                )
            ),
            Tone::Neutral,
        ));
    }
    let open = counted(reading.open_dispositions, "disposition", "dispositions");
    if reading.unreadable > 0 {
        return Some((
            format!(
                "{open} open · {} unreadable",
                counted(reading.unreadable, "gate", "gates")
            ),
            Tone::Error,
        ));
    }
    if reading.open_dispositions > 0 {
        return Some((format!("{open} open"), Tone::Warn));
    }
    None
}

/// The plan's read of the model qualification gate.
///
/// Every number here is summed from the Models workspace's own summaries and
/// none is recomputed, so the two surfaces cannot report different gates for
/// the same closure.
fn qualification_gate(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let bound = cx
        .model
        .sim_setup
        .model_bindings
        .iter()
        .map(|binding| binding.library_name.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>();
    let summaries = cx
        .peers
        .models()
        .model_gate_facts(cx.model)
        .into_iter()
        .filter(|fact| bound.contains(&fact.library.to_ascii_lowercase()))
        .collect::<Vec<_>>();
    let reading = gate_reading(summaries.iter().map(ModelGate::from));
    let status = gate_status(&reading);
    let mut open = false;
    card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Qualification gate",
                status.as_ref().map(|(text, tone)| (text.as_str(), *tone)),
                |ui| {
                    open = Button::new("Open qualification…").show(ui).clicked();
                },
            );
        },
        |ui| {
            card_body(ui, |ui| {
                rule_row(
                    ui,
                    "Release-gating vectors",
                    &format!("{} of {} passing", reading.passing, reading.vectors),
                );
                rule_row(
                    ui,
                    "Models with findings",
                    &format!("{} of {}", reading.findings.len(), reading.models),
                );
                rule_row(
                    ui,
                    "Gates that could not be read",
                    &if reading.unreadable == 0 {
                        "none".to_owned()
                    } else {
                        format!("{} · not counted as passing", reading.unreadable)
                    },
                );
            });
            // No findings is no table: the head's status already says whether
            // there was anything to gate.
            if !reading.findings.is_empty() {
                ledger_head(ui, &GATE_COLUMNS, &["Model", "Finding", "Gate"]);
            }
            for finding in reading.findings.iter().take(GATE_FINDING_ROWS) {
                ledger_row(
                    ui,
                    &GATE_COLUMNS,
                    &[
                        (finding.model.as_str(), Tone::Text),
                        (finding.finding.as_str(), finding.tone),
                        (finding.state, Tone::Neutral),
                    ],
                    false,
                    RowPress::Ignored,
                );
            }
            let remainder = reading.findings.len().saturating_sub(GATE_FINDING_ROWS);
            if remainder > 0 {
                ledger_row(
                    ui,
                    &GATE_COLUMNS,
                    &[
                        (format!("{remainder} more").as_str(), Tone::Neutral),
                        ("listed in full on the qualification page", Tone::Neutral),
                        ("—", Tone::Neutral),
                    ],
                    false,
                    RowPress::Ignored,
                );
            }
            card_note(
                ui,
                "A run may proceed with open dispositions; a release may not. The gate is the \
                 Models & PDKs workspace's own count of vector outcomes against a source-owned \
                 model revision, read here and never recomputed — a model whose gate cannot be \
                 read is reported as unreadable rather than counted as passing. Vectors, \
                 dispositions and release promotion are authored there.",
            );
        },
    );
    if open {
        // Route to the surface that owns the qualification record.
        // Dispositioning a vector from the plan would give one record two
        // authors, so the page hands off instead.
        cx.execute(ids::MODEL_QUALIFICATION);
    }
}

fn binding_policy(ui: &mut Ui, model: &Model) {
    let bindings = &model.sim_setup.model_bindings;
    let with_corner = bindings
        .iter()
        .filter(|binding| binding.selected_corner.is_some())
        .count();
    let subcircuits: usize = bindings
        .iter()
        .filter_map(|binding| {
            model
                .model_library_manager
                .get_library(&binding.library_name)
        })
        .map(|library| library.subcircuits.len())
        .sum();
    card(ui, "Binding policy", None, |ui| {
        card_body(ui, |ui| {
            rule_row(
                ui,
                "Libraries with a corner bound",
                &format!("{with_corner} of {}", bindings.len()),
            );
            rule_row(ui, "Addressable subcircuits", &subcircuits.to_string());
            rule_row(
                ui,
                "Unresolved instance",
                "refused at preflight · the instance and the name it wanted are reported",
            );
            rule_row(
                ui,
                "Duplicate definition",
                "resolved by an explicit provider decision; otherwise preflight refuses it",
            );
        });
        card_note(
            ui,
            "Resolution order is the active plan's binding order above. The project model \
                 workspace owns source content; this plan owns which sources participate, their \
                 precedence, and each nominal section.",
        );
    });
}

fn reproducibility(ui: &mut Ui, model: &Model) {
    let libraries = model
        .sim_setup
        .model_bindings
        .iter()
        .filter_map(|binding| {
            model
                .model_library_manager
                .get_library(&binding.library_name)
        })
        .collect::<Vec<_>>();
    // Only an external library needs a pinned closure: a project-owned one
    // already carries its bytes in the project file. Counting every library
    // as needing a pin made this card contradict the closure card above it,
    // which reports the same fact.
    let external: Vec<_> = libraries
        .iter()
        .filter(|library| matches!(library.source_authority, ModelSourceAuthority::External))
        .collect();
    let pinned = external
        .iter()
        .filter(|library| !library.source_closure.is_empty())
        .count();
    let records: usize = libraries
        .iter()
        .map(|library| library.source_contents.len())
        .sum();
    let complete = pinned == external.len();
    card(
        ui,
        "Reproducibility",
        (!complete).then_some(("incomplete", Tone::Error)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(
                    ui,
                    "External libraries needing a pin",
                    &format!("{pinned} of {} pinned", external.len()),
                );
                rule_row(
                    ui,
                    "Project-owned libraries",
                    &format!(
                        "{} · bytes already retained in the project",
                        libraries.len() - external.len()
                    ),
                );
                rule_row(ui, "Retained source records", &records.to_string());
                rule_row(
                    ui,
                    "Digest",
                    "each retained source carries its own content digest into the run manifest",
                );
            });
            card_note(
                ui,
                "Pinning a library saves the file contents that were read, not just their path, \
                 so the project alone can reproduce a result. A library read from a path whose \
                 file has since changed would otherwise give a different answer under the same \
                 run settings.",
            );
        },
    );
}

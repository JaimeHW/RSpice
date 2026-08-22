//! Models & sections.
//!
//! The model closure this plan binds against. The page reads the loaded
//! libraries and states, per library, which corner section a run will resolve
//! to and whether its source is pinned well enough to be reproducible.
//!
//! Authoring belongs to Models & PDKs; this page is the plan's read of that
//! closure plus the one binding decision the plan owns — the corner.

use egui::Ui;

use crate::state::model_library::{ModelSourceAuthority, short_digest};
use crate::ui::icons::Icon;
use crate::ui::widgets::{Button, IconButton, select};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::ModelsPage;

use super::super::models::closure::{
    DefinitionRow, definition_index, scan_source_drift, source_drift_findings,
    source_drift_needs_scan,
};
use super::super::models::{ModelGateFact, QualificationGate, model_gate_facts};
use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, card_with_head, cell_ui,
    ledger_head, ledger_row, ledger_row_cells, paint_text, rule_row,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

const CLOSURE_COLUMNS: [f32; 7] = [0.05, 0.18, 0.15, 0.12, 0.16, 0.20, 0.14];

/// How many contested names this library provides, by the manager's own index.
///
/// Never recomputed here — [`definition_index`] is the models workspace's one
/// answer to "who provides this name", and the studio states it rather than
/// deriving a second opinion that would eventually disagree with the workspace
/// an operator repairs it in.
fn contested_names(definitions: &[DefinitionRow], library: &str) -> usize {
    definitions
        .iter()
        .filter(|row| {
            row.contested()
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

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    closure(ui, app);
    qualification_gate(ui, app);
    card_row(
        ui,
        app,
        |ui, app| binding_policy(ui, app),
        |ui, app| reproducibility(ui, app),
    );
    super::pages::plan_configuration_receipts(ui, app);
}

fn closure(ui: &mut Ui, app: &mut RSpiceApp) {
    // The scan the models workspace takes, taken here too, because this page is
    // reachable without ever opening that workspace and a drift report nobody
    // requested reads exactly like a closure with no drift. Event-driven, not
    // per frame: it rehashes every pinned source, and `needs_scan` is false
    // until the project revision or the catalog actually moves.
    if source_drift_needs_scan(&app.state) {
        scan_source_drift(&mut app.state);
    }
    let bindings = app.state.sim_setup.model_bindings.clone();
    // One name cannot be contested by one library, so a closure that loaded a
    // single library skips the index entirely. That check is free and the index
    // walks every model of every loaded library.
    let definitions: Vec<DefinitionRow> = if app.state.model_library_manager.library_count() > 1 {
        definition_index(&app.state)
    } else {
        Vec::new()
    };
    let invalid = bindings
        .iter()
        .filter(|binding| {
            app.state
                .model_library_manager
                .validate_simulation_plan_bindings(std::slice::from_ref(binding))
                .is_err()
        })
        .count();
    let drifted = bindings
        .iter()
        .filter(|binding| !source_drift_findings(&app.state, &binding.library_name).is_empty())
        .count();
    let contested = bindings
        .iter()
        .map(|binding| contested_names(&definitions, &binding.library_name))
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
                "{} ordered binding{}",
                bindings.len(),
                plural_suffix(bindings.len())
            ),
            Tone::Ok,
        )
    };
    // Resolved before the card is drawn because the control that offers these
    // belongs in the card head, and the head is drawn first.
    let bound_names = bindings
        .iter()
        .map(|binding| binding.library_name.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>();
    let available = app
        .state
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
                        ("No model libraries bound", Tone::Neutral),
                        ("0", Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("explicit empty closure", Tone::Neutral),
                        ("attach a library below", Tone::Warn),
                        ("—", Tone::Neutral),
                    ],
                    false,
                );
            }
            for (index, binding) in bindings.iter().enumerate() {
                let library = app
                    .state
                    .model_library_manager
                    .get_library(&binding.library_name);
                let corners = corner_options(app, &binding.library_name);
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
                paint_text(
                    ui,
                    cells[1].shrink2(egui::vec2(8.0, 0.0)),
                    &binding.library_name,
                    font.clone(),
                    if library.is_some() {
                        t.color.accent
                    } else {
                        t.color.err
                    },
                );
                // What this library actually offers a reference, and how much of it
                // more than one library claims. A contested name fails closed at
                // bind time, so it belongs beside the count it spoils rather than
                // in a separate advisory the reader has to correlate.
                let contested_here = contested_names(&definitions, &binding.library_name);
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
                paint_text(
                    ui,
                    cells[2].shrink2(egui::vec2(8.0, 0.0)),
                    &provides,
                    font.clone(),
                    if contested_here > 0 {
                        t.color.err
                    } else {
                        t.color.text_dim
                    },
                );
                // The digest the binding pinned, and whether the bytes still hash
                // to it. Drift is the models workspace's finding, read here rather
                // than recomputed: this page cannot repair it, so it must not be
                // able to disagree about whether there is anything to repair.
                let drift = source_drift_findings(&app.state, &binding.library_name);
                paint_text(
                    ui,
                    cells[3].shrink2(egui::vec2(8.0, 0.0)),
                    &short_digest(&binding.source_digest.to_string()),
                    font.clone(),
                    if drift.is_empty() {
                        t.color.text_faint
                    } else {
                        t.color.warn
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
                let validation = app
                    .state
                    .model_library_manager
                    .validate_simulation_plan_bindings(std::slice::from_ref(binding));
                let (source_text, source_color) = match (library, validation) {
                    (_, Err(_)) => ("stale · review required".to_owned(), t.color.err),
                    (Some(library), Ok(())) => (
                        format!(
                            "{} · {} pinned",
                            match library.source_authority {
                                ModelSourceAuthority::External => "external",
                                _ => "project",
                            },
                            library.source_closure.len()
                        ),
                        t.color.text_dim,
                    ),
                    (None, Ok(())) => ("missing".to_owned(), t.color.err),
                };
                paint_text(
                    ui,
                    cells[4].shrink2(egui::vec2(8.0, 0.0)),
                    &source_text,
                    font,
                    source_color,
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
        apply_model_binding_action(app, action);
    }
}

const fn plural_suffix(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

enum ModelBindingAction {
    SetCorner {
        index: usize,
        corner: Option<String>,
    },
    MoveUp(usize),
    MoveDown(usize),
    Remove(usize),
    Attach(String),
}

fn corner_options(app: &RSpiceApp, library: &str) -> Vec<String> {
    app.state
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

fn apply_model_binding_action(app: &mut RSpiceApp, action: ModelBindingAction) {
    let mut candidate = app.state.sim_setup.clone();
    let detail = match action {
        ModelBindingAction::SetCorner { index, corner } => {
            let Some(binding) = candidate.model_bindings.get_mut(index) else {
                return record_binding_refusal(
                    app,
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
            let binding = match app
                .state
                .model_library_manager
                .simulation_plan_binding(&name)
            {
                Ok(binding) => binding,
                Err(error) => return record_binding_refusal(app, &error),
            };
            candidate.model_bindings.push(binding);
            format!("Attached model library {name} to the plan.")
        }
        _ => return,
    };
    if let Err(error) = app
        .state
        .model_library_manager
        .validate_simulation_plan_bindings(&candidate.model_bindings)
    {
        return record_binding_refusal(app, &error);
    }
    match candidate.commit_active_plan_configuration_change(detail) {
        Ok(receipt) => {
            app.state.sim_setup = candidate;
            app.invalidate_simulation_preflight();
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_receipt(receipt.status_line());
        }
        Err(error) => record_binding_refusal(app, &error.to_string()),
    }
}

fn record_binding_refusal(app: &mut RSpiceApp, error: &str) {
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal(error.to_owned());
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
struct ModelGate<'a> {
    library: &'a str,
    model: &'a str,
    vectors: usize,
    evidenced_vectors: usize,
    passing_vectors: usize,
    open_dispositions: usize,
    /// The workspace could not read this model's gate at all.
    unreadable: bool,
    gate: QualificationGate,
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
        let (rank, finding, tone) = if self.unreadable {
            (0, "gate could not be read".to_owned(), Tone::Error)
        } else if self.evidenced_vectors > self.passing_vectors {
            (
                1,
                format!(
                    "{} failing",
                    plural(
                        self.evidenced_vectors - self.passing_vectors,
                        "evidenced vector"
                    )
                ),
                Tone::Error,
            )
        } else if self.open_dispositions > 0 {
            (
                2,
                format!("{} open", plural(self.open_dispositions, "disposition")),
                Tone::Warn,
            )
        } else if self.vectors > self.evidenced_vectors {
            (
                3,
                format!(
                    "{} without evidence",
                    plural(self.vectors - self.evidenced_vectors, "vector")
                ),
                Tone::Warn,
            )
        } else if self.gate == QualificationGate::Unqualified {
            (4, "no qualification suite".to_owned(), Tone::Warn)
        } else if self.gate != QualificationGate::Qualified {
            // Platform parity and measurement correlation also hold the gate,
            // and neither reason is carried on the summary. Saying which one
            // it was would be a guess, so the card says where the reason is.
            (
                5,
                "held for review · the reason is recorded in Models & PDKs".to_owned(),
                Tone::Warn,
            )
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
struct GateFinding {
    /// Severity, so the rows the card has room for are the ones that matter.
    rank: u8,
    model: String,
    finding: String,
    state: &'static str,
    tone: Tone,
}

/// The release gate, summed over the closure.
#[derive(Debug, Default)]
struct GateReading {
    models: usize,
    /// Vectors a *readable* gate records as passing.
    passing: usize,
    /// Every vector the closure's qualification suites declare.
    vectors: usize,
    open_dispositions: usize,
    /// Models whose gate could not be read.
    unreadable: usize,
    findings: Vec<GateFinding>,
}

/// Sum the gate across the closure.
///
/// A model whose gate could not be read contributes its vectors to the total
/// and nothing to the passing count. The alternative — skipping it, or taking
/// the outcomes recorded against a source identity that no longer validates —
/// reports a clean release on evidence nobody can open, and a gate that fails
/// open is worse than no gate.
fn gate_reading<'a>(models: impl IntoIterator<Item = ModelGate<'a>>) -> GateReading {
    let mut reading = GateReading::default();
    for model in models {
        reading.models += 1;
        reading.vectors += model.vectors;
        reading.open_dispositions += model.open_dispositions;
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
/// unreadable at all.
fn gate_status(reading: &GateReading) -> (String, Tone) {
    if reading.models == 0 {
        return ("no models to gate".to_owned(), Tone::Neutral);
    }
    let open = plural(reading.open_dispositions, "disposition");
    if reading.unreadable > 0 {
        return (
            format!(
                "{open} open · {} unreadable",
                plural(reading.unreadable, "gate")
            ),
            Tone::Error,
        );
    }
    if reading.open_dispositions > 0 {
        return (format!("{open} open"), Tone::Warn);
    }
    ("no open dispositions".to_owned(), Tone::Ok)
}

fn plural(count: usize, singular: &str) -> String {
    format!("{count} {singular}{}", if count == 1 { "" } else { "s" })
}

/// The plan's read of the model qualification gate.
///
/// Every number here is summed from the Models workspace's own summaries and
/// none is recomputed, so the two surfaces cannot report different gates for
/// the same closure.
fn qualification_gate(ui: &mut Ui, app: &mut RSpiceApp) {
    let bound = app
        .state
        .sim_setup
        .model_bindings
        .iter()
        .map(|binding| binding.library_name.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>();
    let summaries = model_gate_facts(app)
        .into_iter()
        .filter(|fact| bound.contains(&fact.library.to_ascii_lowercase()))
        .collect::<Vec<_>>();
    let reading = gate_reading(summaries.iter().map(ModelGate::from));
    let (status, tone) = gate_status(&reading);
    let mut open = false;
    card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Qualification gate",
                Some((status.as_str(), tone)),
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
            ledger_head(ui, &GATE_COLUMNS, &["Model", "Finding", "Gate"]);
            if reading.findings.is_empty() {
                let (subject, finding, tone) = if reading.models == 0 {
                    ("No models loaded", "nothing to gate", Tone::Warn)
                } else {
                    (
                        "No findings",
                        "every gate in the closure is qualified",
                        Tone::Ok,
                    )
                };
                ledger_row(
                    ui,
                    &GATE_COLUMNS,
                    &[
                        (subject, Tone::Neutral),
                        (finding, tone),
                        ("—", Tone::Neutral),
                    ],
                    false,
                );
            }
            for finding in reading.findings.iter().take(GATE_FINDING_ROWS) {
                ledger_row(
                    ui,
                    &GATE_COLUMNS,
                    &[
                        (finding.model.as_str(), Tone::Accent),
                        (finding.finding.as_str(), finding.tone),
                        (finding.state, finding.tone),
                    ],
                    false,
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
        Command::ModelsPage(ModelsPage::Qualification).execute(app);
    }
}

fn binding_policy(ui: &mut Ui, app: &RSpiceApp) {
    let bindings = &app.state.sim_setup.model_bindings;
    let with_corner = bindings
        .iter()
        .filter(|binding| binding.selected_corner.is_some())
        .count();
    let subcircuits: usize = bindings
        .iter()
        .filter_map(|binding| {
            app.state
                .model_library_manager
                .get_library(&binding.library_name)
        })
        .map(|library| library.subcircuits.len())
        .sum();
    card(
        ui,
        "Binding policy",
        Some(("resolved at elaboration", Tone::Ok)),
        |ui| {
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
        },
    );
}

fn reproducibility(ui: &mut Ui, app: &RSpiceApp) {
    let libraries = app
        .state
        .sim_setup
        .model_bindings
        .iter()
        .filter_map(|binding| {
            app.state
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
        Some((
            if complete {
                "every external source pinned"
            } else {
                "incomplete"
            },
            if complete { Tone::Ok } else { Tone::Error },
        )),
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
                "A pinned closure is what makes a result reproducible from the project alone: the \
                 exact bytes that were read, not the path they were read from. A library resolved \
                 from a live path that has since changed would otherwise produce a different \
                 answer under the same manifest.",
            );
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn active_plan_revision(app: &RSpiceApp) -> crate::product::ObjectRevision {
        app.state
            .sim_setup
            .analysis_plan
            .as_ref()
            .expect("test instance has an active plan")
            .revision()
    }

    fn add_sectioned_library(app: &mut RSpiceApp, source_name: &str, model: &str) -> String {
        app.state
            .model_library_manager
            .load_library_bytes(
                source_name,
                format!(
                    ".lib TT\n.model {model} NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n\
                     .lib FF\n.model {model} NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n"
                )
                .into_bytes(),
                None,
            )
            .expect("sectioned test library is executable")
    }

    #[test]
    fn model_binding_actions_commit_exact_order_corner_and_removal() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let first = add_sectioned_library(&mut app, "models-first.lib", "first_nch");
        let second = add_sectioned_library(&mut app, "models-second.lib", "second_nch");
        let initial_revision = active_plan_revision(&app);
        app.state.workbench.preflight.open = true;

        apply_model_binding_action(&mut app, ModelBindingAction::Attach(first.clone()));
        assert_eq!(app.state.sim_setup.model_bindings[0].library_name, first);
        assert_ne!(active_plan_revision(&app), initial_revision);
        assert!(!app.state.workbench.preflight.open);

        apply_model_binding_action(&mut app, ModelBindingAction::Attach(second.clone()));
        assert_eq!(
            app.state
                .sim_setup
                .model_bindings
                .iter()
                .map(|binding| binding.library_name.as_str())
                .collect::<Vec<_>>(),
            [first.as_str(), second.as_str()]
        );

        apply_model_binding_action(&mut app, ModelBindingAction::MoveUp(1));
        assert_eq!(
            app.state
                .sim_setup
                .model_bindings
                .iter()
                .map(|binding| binding.library_name.as_str())
                .collect::<Vec<_>>(),
            [second.as_str(), first.as_str()]
        );

        apply_model_binding_action(
            &mut app,
            ModelBindingAction::SetCorner {
                index: 0,
                corner: Some("FF".to_owned()),
            },
        );
        assert_eq!(
            app.state.sim_setup.model_bindings[0]
                .selected_corner
                .as_deref(),
            Some("FF")
        );

        apply_model_binding_action(&mut app, ModelBindingAction::Remove(1));
        assert_eq!(app.state.sim_setup.model_bindings.len(), 1);
        assert_eq!(app.state.sim_setup.model_bindings[0].library_name, second);
        assert!(!app.state.workbench.analysis_lifecycle_status.is_refusal());
    }

    #[test]
    fn refused_model_binding_actions_preserve_plan_state_and_revision() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let library = add_sectioned_library(&mut app, "models-valid.lib", "valid_nch");
        apply_model_binding_action(&mut app, ModelBindingAction::Attach(library));
        let before_bindings = app.state.sim_setup.model_bindings.clone();
        let before_revision = active_plan_revision(&app);

        apply_model_binding_action(
            &mut app,
            ModelBindingAction::SetCorner {
                index: 0,
                corner: Some("NONEXISTENT".to_owned()),
            },
        );
        assert_eq!(app.state.sim_setup.model_bindings, before_bindings);
        assert_eq!(active_plan_revision(&app), before_revision);
        assert!(app.state.workbench.analysis_lifecycle_status.is_refusal());

        apply_model_binding_action(
            &mut app,
            ModelBindingAction::Attach("missing-library".to_owned()),
        );
        assert_eq!(app.state.sim_setup.model_bindings, before_bindings);
        assert_eq!(active_plan_revision(&app), before_revision);
        assert!(app.state.workbench.analysis_lifecycle_status.is_refusal());
    }

    const fn model_gate<'a>(
        model: &'a str,
        vectors: usize,
        evidenced_vectors: usize,
        passing_vectors: usize,
        open_dispositions: usize,
        unreadable: bool,
        gate: QualificationGate,
    ) -> ModelGate<'a> {
        ModelGate {
            library: "pdk",
            model,
            vectors,
            evidenced_vectors,
            passing_vectors,
            open_dispositions,
            unreadable,
            gate,
        }
    }

    #[test]
    fn open_dispositions_are_reported_and_never_read_as_clean() {
        let reading = gate_reading([model_gate(
            "nch",
            12,
            12,
            12,
            3,
            false,
            QualificationGate::Review,
        )]);

        assert_eq!(reading.open_dispositions, 3);
        let (status, tone) = gate_status(&reading);
        assert_eq!(status, "3 dispositions open");
        assert_ne!(tone, Tone::Ok);
        let finding = reading
            .findings
            .first()
            .expect("a model with open dispositions has a finding");
        assert_eq!(finding.model, "pdk/nch");
        assert_eq!(finding.finding, "3 dispositions open");
        assert_eq!(finding.state, "review");
        assert_ne!(finding.tone, Tone::Ok);
    }

    #[test]
    fn an_unreadable_gate_is_reported_rather_than_counted_as_passing() {
        // The unreadable model carries outcomes that say every one of its
        // vectors passed. They are recorded against a source identity that no
        // longer validates, so the card must not spend them.
        let reading = gate_reading([
            model_gate("readable", 4, 4, 4, 0, false, QualificationGate::Qualified),
            model_gate("stale", 6, 6, 6, 0, true, QualificationGate::Blocked),
        ]);

        assert_eq!(reading.passing, 4);
        assert_eq!(reading.vectors, 10);
        assert_eq!(reading.unreadable, 1);
        let (status, tone) = gate_status(&reading);
        assert_eq!(status, "0 dispositions open · 1 gate unreadable");
        assert_eq!(tone, Tone::Error);
        let finding = reading
            .findings
            .first()
            .expect("an unreadable gate is a finding");
        assert_eq!(finding.model, "pdk/stale");
        assert_eq!(finding.finding, "gate could not be read");
    }

    #[test]
    fn a_qualified_closure_reports_no_findings() {
        let reading = gate_reading([model_gate(
            "nch",
            8,
            8,
            8,
            0,
            false,
            QualificationGate::Qualified,
        )]);

        assert!(reading.findings.is_empty());
        assert_eq!(
            gate_status(&reading),
            ("no open dispositions".to_owned(), Tone::Ok)
        );
    }

    /// An empty closure has nothing to gate. Reporting it as clean would be the
    /// card's one chance to sign off a release on no evidence at all.
    #[test]
    fn an_empty_closure_never_reads_as_a_clean_gate() {
        assert_eq!(
            gate_status(&gate_reading([])),
            ("no models to gate".to_owned(), Tone::Neutral)
        );
    }

    /// The card shows the first [`GATE_FINDING_ROWS`] findings, so they have to
    /// be the worst ones.
    #[test]
    fn findings_are_ordered_worst_first() {
        let reading = gate_reading([
            model_gate("review", 2, 2, 2, 0, false, QualificationGate::Review),
            model_gate("nosuite", 0, 0, 0, 0, false, QualificationGate::Unqualified),
            model_gate("noevid", 5, 1, 1, 0, false, QualificationGate::Review),
            model_gate("disp", 3, 3, 3, 1, false, QualificationGate::Review),
            model_gate("fail", 4, 4, 3, 0, false, QualificationGate::Review),
            model_gate("unread", 9, 9, 9, 0, true, QualificationGate::Blocked),
        ]);

        let order: Vec<&str> = reading
            .findings
            .iter()
            .map(|finding| finding.model.as_str())
            .collect();
        assert_eq!(
            order,
            [
                "pdk/unread",
                "pdk/fail",
                "pdk/disp",
                "pdk/noevid",
                "pdk/nosuite",
                "pdk/review",
            ]
        );
        assert_eq!(reading.findings[1].finding, "1 evidenced vector failing");
        assert_eq!(reading.findings[2].finding, "1 disposition open");
        assert_eq!(reading.findings[3].finding, "4 vectors without evidence");
        assert_eq!(reading.findings[4].finding, "no qualification suite");
        assert_eq!(reading.findings[4].state, "unqualified");
    }

    /// The card and the Models workspace must not be able to disagree, so the
    /// totals it shows are asserted against the summaries themselves.
    #[test]
    fn gate_totals_equal_the_sum_over_the_workspace_summaries() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let definition = crate::state::model_library::ProjectModelDefinition {
            name: "nch_gated".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Plan-page qualification gate fixture".to_owned(),
            numeric_parameters: std::collections::BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("vth0".to_owned(), 0.48),
            ]),
            string_parameters: std::collections::BTreeMap::new(),
        };
        app.state
            .model_library_manager
            .create_project_model("owned-models", &definition)
            .expect("create project model");
        crate::workbench::documents::model_editor::open_project_model(
            &mut app,
            "owned-models",
            "nch_gated",
        )
        .expect("open the project model for qualification authoring");
        let editor = &mut app.state.workbench.model_editor;
        editor.begin_qualification_suite();
        let authoring = &mut editor.qualification_authoring;
        authoring.suite_id = "dc-op".to_owned();
        authoring.suite_name = "DC operating point".to_owned();
        authoring.vector_id = "nominal".to_owned();
        authoring.vector_name = "Nominal bias".to_owned();
        authoring.executable_input =
            "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_gated\n.op\n.end\n".to_owned();
        authoring.quantity = "v(out)".to_owned();
        authoring.probe_target = "out".to_owned();
        authoring.expected = "1".to_owned();
        authoring.absolute_tolerance = "1e-9".to_owned();
        authoring.relative_tolerance = "1e-9".to_owned();
        assert!(
            editor.commit_qualification_suite(),
            "{:?}",
            editor.qualification_authoring.error
        );

        let summaries = model_gate_facts(&app);
        let reading = gate_reading(summaries.iter().map(ModelGate::from));

        assert!(
            reading.vectors > 0,
            "the fixture must declare at least one release-gating vector"
        );
        assert_eq!(reading.models, summaries.len());
        assert_eq!(
            reading.vectors,
            summaries
                .iter()
                .map(|summary| summary.vectors)
                .sum::<usize>()
        );
        assert_eq!(
            reading.passing,
            summaries
                .iter()
                .filter(|summary| !summary.unreadable)
                .map(|summary| summary.passing_vectors)
                .sum::<usize>()
        );
        assert_eq!(
            reading.open_dispositions,
            summaries
                .iter()
                .map(|summary| summary.open_dispositions)
                .sum::<usize>()
        );
        assert_eq!(
            reading.unreadable,
            summaries
                .iter()
                .filter(|summary| summary.unreadable)
                .count()
        );
    }
}

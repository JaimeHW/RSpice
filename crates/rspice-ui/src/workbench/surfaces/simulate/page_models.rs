//! Models & sections.
//!
//! The model closure this plan binds against. The page reads the loaded
//! libraries and states, per library, which corner section a run will resolve
//! to and whether its source is pinned well enough to be reproducible.
//!
//! Authoring belongs to Models & PDKs; this page is the plan's read of that
//! closure plus the one binding decision the plan owns — the corner.

use egui::Ui;

use crate::state::model_library::ModelSourceAuthority;
use crate::ui::icons::Icon;
use crate::ui::widgets::{Button, IconButton, select};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::ModelsPage;

use super::super::models::{ModelGateFact, QualificationGate, model_gate_facts};
use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, card_with_head, cell_ui,
    ledger_head, ledger_row, ledger_row_cells, paint_text, rule_row,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

const CLOSURE_COLUMNS: [f32; 6] = [0.07, 0.21, 0.12, 0.18, 0.27, 0.15];
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
}

fn closure(ui: &mut Ui, app: &mut RSpiceApp) {
    let bindings = app.state.sim_setup.model_bindings.clone();
    let invalid = bindings
        .iter()
        .filter(|binding| {
            app.state
                .model_library_manager
                .validate_simulation_plan_bindings(std::slice::from_ref(binding))
                .is_err()
        })
        .count();
    let status = if invalid == 0 {
        format!(
            "{} ordered binding{}",
            bindings.len(),
            plural_suffix(bindings.len())
        )
    } else {
        format!(
            "{invalid} stale or invalid binding{}",
            plural_suffix(invalid)
        )
    };
    let tone = if invalid == 0 { Tone::Ok } else { Tone::Error };
    let mut requested = None;
    card(ui, "Model closure", Some((status.as_str(), tone)), |ui| {
        ledger_head(
            ui,
            &CLOSURE_COLUMNS,
            &[
                "Order",
                "Library",
                "Models",
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
            paint_text(
                ui,
                cells[2].shrink2(egui::vec2(8.0, 0.0)),
                &library
                    .map_or(0, |library| library.models.len())
                    .to_string(),
                font.clone(),
                t.color.text_dim,
            );
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
                cells[3].shrink2(egui::vec2(8.0, 0.0)),
                &source_text,
                font,
                source_color,
            );
            if library.is_none() || corners.is_empty() {
                paint_text(
                    ui,
                    cells[4].shrink2(egui::vec2(8.0, 0.0)),
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
                let cell_rect = cells[4].shrink2(egui::vec2(6.0, 4.0));
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
            let action_rect = cells[5].shrink2(egui::vec2(3.0, 4.0));
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
        if !available.is_empty() {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Available executable libraries:");
                for name in available.iter().take(4) {
                    if Button::new(name).show(ui).clicked() {
                        requested = Some(ModelBindingAction::Attach(name.clone()));
                    }
                }
                if available.len() > 4 {
                    ui.label(format!("+{} more", available.len() - 4));
                }
            });
        }
        card_note(
            ui,
            "This ordered list is owned by the active simulation plan. Earlier libraries have \
             higher precedence. Every entry is pinned to the source digest accepted when it was \
             attached; replacement or refresh requires an explicit review before another run.",
        );
    });

    if let Some(action) = requested {
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

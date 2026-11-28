//! The rows an analysis form draws for the analyses it builds on: the
//! prerequisite bindings the contract card shows, the operating point a
//! periodic solve starts from, and the base-analysis choice of the study
//! forms.

use egui::Ui;

pub(super) fn study_base_row(
    ui: &mut Ui,
    selected_base: &mut Option<crate::product::AnalysisInstanceId>,
    bases: &[(crate::product::AnalysisInstanceId, String)],
    default_label: &str,
) {
    let mut choices = vec![(None, default_label.to_owned())];
    choices.extend(bases.iter().map(|(id, label)| (Some(*id), label.clone())));
    if let Some(id) = *selected_base
        && !choices.iter().any(|(candidate, _)| *candidate == Some(id))
    {
        choices.push((Some(id), "Missing analysis".to_owned()));
    }
    let mut selected = choices
        .iter()
        .position(|(id, _)| *id == *selected_base)
        .unwrap_or(0);
    let labels = choices
        .iter()
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();
    if super::choice_row(ui, "Base analysis", &labels, &mut selected) {
        *selected_base = choices[selected].0;
    }
}

pub(in crate::workbench::surfaces::simulate) fn prerequisite_rows(
    ui: &mut Ui,
    selected: &super::super::SelectedAnalysis,
) -> Option<super::super::AnalysisAction> {
    let mut requested = None;
    let optional = selected.kind.optional_prerequisites();
    for prerequisite in selected
        .prerequisite_roles
        .iter()
        .filter(|prerequisite| !optional.contains(prerequisite))
    {
        let field = uses_field(selected, *prerequisite);
        if field.options.is_empty() {
            // A switched-off analysis blocks nothing yet, so its empty field
            // is stated, not warned about.
            if field.bound || !selected.enabled {
                super::super::property_row(ui, "Uses", &field.current);
            } else {
                super::super::property_row_problem(
                    ui,
                    "Uses",
                    &field.current,
                    Some(super::super::Problem::Warning),
                );
            }
            continue;
        }
        let labels = field
            .options
            .iter()
            .map(|(label, _)| label.clone())
            .collect::<Vec<_>>();
        if let Some(index) =
            select_row(ui, selected, *prerequisite, "Uses", &field.current, &labels)
            && let Some((_, asked)) = field.options.get(index)
        {
            requested = Some(*asked);
        }
    }
    for prerequisite in optional {
        if let Some(asked) = start_from_row(ui, selected, *prerequisite) {
            requested = Some(asked);
        }
    }
    requested
}

/// The "Uses" field of one role an analysis reads: what it shows, and what
/// each of its options asks for.
pub(in crate::workbench::surfaces::simulate) struct UsesField {
    /// The analysis it uses, by name, or the reason it cannot run yet.
    pub(in crate::workbench::surfaces::simulate) current: String,
    pub(in crate::workbench::surfaces::simulate) bound: bool,
    pub(in crate::workbench::surfaces::simulate) options:
        Vec<(String, super::super::AnalysisAction)>,
}

/// The compatible analyses in this test by name, each rebinding the role,
/// and the one action that makes the analysis runnable where picking one of
/// them does not ("Add Periodic steady state", "Enable PSS 1").
pub(in crate::workbench::surfaces::simulate) fn uses_field(
    selected: &super::super::SelectedAnalysis,
    prerequisite: crate::simulation::plan::AnalysisKind,
) -> UsesField {
    // A binding whose analysis is gone reads as no binding: the field is empty
    // and says what the analysis needs.
    let bound = selected
        .dependencies
        .iter()
        .find(|dependency| dependency.prerequisite() == prerequisite)
        .and_then(|dependency| {
            selected
                .dependency_names
                .iter()
                .find(|(id, _)| *id == dependency.target())
        })
        .map(|(_, name)| name.clone());
    let current = bound
        .clone()
        .unwrap_or_else(|| format!("needs {}", super::super::prerequisite_need(prerequisite)));
    let candidates = selected
        .prerequisite_candidates
        .iter()
        .find_map(|(kind, candidates)| (*kind == prerequisite).then_some(candidates.as_slice()))
        .unwrap_or_default();
    let mut options = candidates
        .iter()
        .map(|candidate| {
            (
                candidate.label.clone(),
                super::super::AnalysisAction::BindDependency {
                    prerequisite,
                    target: candidate.id,
                },
            )
        })
        .collect::<Vec<_>>();
    // Picking a listed analysis is the fix while none is bound; otherwise the
    // repair is offered beside them: add, enable or move one, or switch on
    // what the one it uses needs itself.
    if (bound.is_some() || candidates.is_empty())
        && let Some(label) = &selected.repair_label
    {
        options.push((
            label.clone(),
            super::super::AnalysisAction::RepairDependencies,
        ));
    }
    UsesField {
        current,
        bound: bound.is_some(),
        options,
    }
}

/// The DC point a PSS, HB or QPSS starts from: its own solve, or the solution
/// of an operating point earlier in this test.
fn start_from_row(
    ui: &mut Ui,
    selected: &super::super::SelectedAnalysis,
    prerequisite: crate::simulation::plan::AnalysisKind,
) -> Option<super::super::AnalysisAction> {
    const OWN: &str = "Own DC solve";
    let candidates = selected
        .prerequisite_candidates
        .iter()
        .find_map(|(kind, candidates)| (*kind == prerequisite).then_some(candidates.as_slice()))
        .unwrap_or_default();
    let bound = selected
        .dependencies
        .iter()
        .find(|dependency| dependency.prerequisite() == prerequisite);
    if candidates.is_empty() && bound.is_none() {
        super::super::property_row(ui, "Start from", OWN).on_hover_text(
            "It finds its own DC point. An operating point earlier in this test can seed it instead.",
        );
        return None;
    }
    let current = bound.map_or(OWN, |dependency| {
        selected
            .dependency_names
            .iter()
            .find(|(id, _)| *id == dependency.target())
            .map_or("missing analysis", |(_, name)| name.as_str())
    });
    let options = std::iter::once(OWN.to_owned())
        .chain(candidates.iter().map(|candidate| candidate.label.clone()))
        .collect::<Vec<_>>();
    match select_row(ui, selected, prerequisite, "Start from", current, &options)? {
        0 => bound.map(|_| super::super::AnalysisAction::StartFromOwnSolve { prerequisite }),
        index => candidates.get(index - 1).map(|candidate| {
            super::super::AnalysisAction::BindDependency {
                prerequisite,
                target: candidate.id,
            }
        }),
    }
}

/// One labelled select in the property block, answering the option picked.
fn select_row(
    ui: &mut Ui,
    selected: &super::super::SelectedAnalysis,
    prerequisite: crate::simulation::plan::AnalysisKind,
    label: &str,
    current: &str,
    options: &[String],
) -> Option<usize> {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let width = ui.available_width().max(1.0);
        let trailing = crate::workbench::design_system::panel_inset(ui);
        // The property rows above and below this one are the same block,
        // so the control starts where their values do. The share this
        // replaces was a second account of the same column that collapsed
        // to the label's own text width, opening a second label column a
        // hundred points left of the first.
        super::super::paint_control_row_label(ui, label, width);
        let select_width = (ui.available_width() - trailing).max(1.0);
        let salt = format!(
            "analysis.{}.prerequisite.{}",
            selected.id,
            prerequisite.stable_id()
        );
        crate::ui::widgets::select(
            ui,
            &salt,
            &format!("Select {label}"),
            current,
            options,
            select_width,
        )
    })
    .inner
}

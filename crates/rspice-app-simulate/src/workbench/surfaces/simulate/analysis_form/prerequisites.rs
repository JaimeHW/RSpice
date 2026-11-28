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
        choices.push((Some(id), format!("Unavailable analysis ({id})")));
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
        let bound = selected
            .dependencies
            .iter()
            .find(|dependency| dependency.prerequisite() == *prerequisite);
        let target = bound.map_or_else(
            || {
                if selected.enabled {
                    "unbound · preflight blocked".to_owned()
                } else {
                    "unbound · required when enabled".to_owned()
                }
            },
            |dependency| {
                selected
                    .dependency_names
                    .iter()
                    .find(|(id, _)| *id == dependency.target())
                    .map_or_else(|| "missing analysis".to_owned(), |(_, name)| name.clone())
            },
        );
        let label = format!("{} prerequisite", prerequisite.stable_id().to_uppercase());
        let candidates = selected
            .prerequisite_candidates
            .iter()
            .find_map(|(kind, candidates)| (*kind == *prerequisite).then_some(candidates));
        let Some(candidates) = candidates.filter(|candidates| !candidates.is_empty()) else {
            super::super::property_row(ui, &label, &target);
            continue;
        };
        let options = candidates
            .iter()
            .map(|candidate| candidate.label.clone())
            .collect::<Vec<_>>();
        let current = bound
            .and_then(|dependency| {
                candidates
                    .iter()
                    .find(|candidate| candidate.id == dependency.target())
            })
            .map_or("Select compatible instance", |candidate| {
                candidate.label.as_str()
            });
        if let Some(index) = select_row(ui, selected, *prerequisite, &label, current, &options)
            && let Some(candidate) = candidates.get(index)
        {
            requested = Some(super::super::AnalysisAction::BindDependency {
                prerequisite: *prerequisite,
                target: candidate.id,
            });
        }
    }
    for prerequisite in optional {
        if let Some(asked) = start_from_row(ui, selected, *prerequisite) {
            requested = Some(asked);
        }
    }
    requested
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

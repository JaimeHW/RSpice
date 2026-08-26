//! Specialist Models & PDKs page: bins.

use super::*;

/// The height one [`egui::Ui::selectable_label`] takes at the current style.
///
/// A virtualized list is told its row height up front, and every table on this
/// workspace allocates its rows at a height it names. The bin family list uses
/// the plain selectable label instead, whose height belongs to egui, so it is
/// derived the same way egui derives it rather than guessed. A guess that runs
/// short leaves the last families unreachable below the fold.
fn selectable_label_height(ui: &Ui) -> f32 {
    (ui.text_style_height(&egui::TextStyle::Button) + ui.spacing().button_padding.y * 2.0)
        .max(ui.spacing().interact_size.y)
}

/// Geometry findings the audit column lists before reporting the remainder.
const FINDING_ROWS: usize = 10;

/// One card's place in the simulator's exact executable bin family.
#[derive(Clone)]
struct BinCard {
    model: String,
    geometry: rspice_core::engine::ModelBinCardGeometry,
    declaration_order: usize,
}

/// The set of cards the engine would consider for one instance reference.
///
/// Keyed by library and by the family base name — `nch` for `nch.1`, `nch.2` —
/// because that is the set core's `resolve_binned_model_def` collects
/// candidates from. Grouping by device type instead, which this page used to
/// do, put every NMOS card in every attached foundry library into one family
/// and reported them as overlapping each other.
struct BinFamily {
    library: String,
    family: String,
    cards: Vec<BinCard>,
}

impl BinFamily {
    fn key(&self) -> String {
        format!("{} · {}", self.library, self.family)
    }
}

fn bin_families(
    app: &ManagerRenderContext<'_>,
    inspection: &rspice_core::engine::ModelBinInspection,
) -> Result<Vec<BinFamily>, String> {
    let mut families = BTreeMap::<(String, String), Vec<BinCard>>::new();
    for card in &inspection.cards {
        let provider = app
            .state
            .model_library_manager
            .effective_definition_provider(ModelConsumerScope::PrimitiveModel, &card.model)?
            .ok_or_else(|| {
                format!(
                    "Prepared model-bin card '{}' has no executable provider in the project catalog",
                    card.model
                )
            })?;
        families
            .entry((provider.library, card.family.clone()))
            .or_default()
            .push(BinCard {
                model: card.model.clone(),
                geometry: card.geometry,
                declaration_order: card.declaration_order,
            });
    }
    Ok(families
        .into_iter()
        .map(|((library, family), mut cards)| {
            cards.sort_by_key(|card| card.declaration_order);
            BinFamily {
                library,
                family,
                cards,
            }
        })
        .collect::<Vec<_>>())
}

#[derive(Clone)]
struct ModelBinInspectionCache {
    input_digest: crate::product::ContentDigest,
    result: Result<rspice_core::engine::ModelBinInspection, String>,
}

/// Inspect the exact deck prepared for the active design and simulation plan.
/// The engine owns expression evaluation, NFIN handling, hierarchy flattening,
/// tolerance, declaration order, and instance selection; this surface only
/// presents the resulting immutable receipt.
///
/// Preparing that deck is far too expensive to repeat per frame, hence the
/// cache — but the key is asked for on every frame whether the cache hits or
/// not, so it has to be cheaper than the miss it prevents. It used to include
/// the execution catalogue digest, which serializes every library in the corpus
/// through `serde_json::Value`; at production scale this page paid the corpus
/// on every frame to decide it did not need to. See
/// [`ModelLibraryManager::design_inspection_catalog_key`][key] for the cheap
/// key that replaced it and for why hashing content — rather than a revision
/// counter a wholesale replacement could carry in with it — is what makes the
/// cached verdict expire when it must.
///
/// [key]: crate::state::model_library::ModelLibraryManager::design_inspection_catalog_key
fn authoritative_bin_inspection(
    ui: &Ui,
    app: &ManagerRenderContext<'_>,
) -> Result<rspice_core::engine::ModelBinInspection, String> {
    let input_digest =
        crate::simulation::controller::prepared_run::design_inspection_input_digest(app.state);
    let cache_id = egui::Id::new("models-authoritative-bin-inspection");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<ModelBinInspectionCache>(cache_id))
        && cached.input_digest == input_digest
    {
        return cached.result;
    }

    let result = (|| {
        let source = crate::simulation::controller::SimulationController::
            prepare_design_netlist_for_inspection(app.state)
            .map_err(|error| error.to_string())?;
        let netlist = rspice_core::Netlist::parse(&source).map_err(|error| error.to_string())?;
        rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .inspect_model_bins(&netlist)
            .map_err(|error| error.to_string())
    })();
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            cache_id,
            ModelBinInspectionCache {
                input_digest,
                result: result.clone(),
            },
        );
    });
    result
}

pub(super) fn bins_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let inspection = authoritative_bin_inspection(ui, app);
    let families = inspection
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|inspection| bin_families(app, inspection));
    let (inspection, families) = match (inspection, families) {
        (Ok(inspection), Ok(families)) => (inspection, families),
        (Err(error), _) | (_, Err(error)) => {
            section_title(
                ui,
                "Bins & geometry",
                "authoritative design inspection blocked",
                |_| {},
            );
            page_empty_state(ui, "Exact model-bin inspection is unavailable", &error);
            return;
        }
    };
    let findings = geometry_findings(&families);
    section_title(
        ui,
        "Bins & geometry",
        &format!(
            "{} binned families · {} cards · {} findings",
            families.len(),
            families
                .iter()
                .map(|family| family.cards.len())
                .sum::<usize>(),
            findings.len()
        ),
        |ui| {
            if ui
                .add_enabled(
                    !app.state.workbench.models_view.model_import_in_progress,
                    egui::Button::new("Import bin map"),
                )
                .on_hover_text(
                    "Import authenticated binned model cards; L, W, and NFIN bounds are evaluated by the simulator from the retained source.",
                )
                .clicked()
            {
                app.queue_model_source_import();
            }
            if ui.button("Edit cards…").clicked() {
                app.queue_command(Command::ModelEditor);
            }
            if ui.button("Audit all families").clicked() {
                receipt(
                    app,
                    if findings.is_empty() {
                        Ok(
                            "Geometry audit completed with no overlapping card envelopes."
                                .to_owned(),
                        )
                    } else {
                        Err(format!(
                            "Geometry audit found {} overlapping or incomplete envelopes.",
                            findings.len()
                        ))
                    },
                );
            }
            // Gated on the fact it traces — the selected model — rather than
            // on a schematic selection it never reads, and it says which.
            let traced = app.state.workbench.selected_model.clone();
            if ui
                .add_enabled(traced.is_some(), egui::Button::new("Trace schematic"))
                .on_disabled_hover_text("Select a model in the catalog first.")
                .clicked()
                && let Some(model) = traced
            {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::BindingTrace {
                        consumers: effective_model_consumers(app, &model),
                        model,
                    });
            }
        },
    );
    if families.is_empty() {
        page_empty_state(
            ui,
            "No geometry envelopes are loaded",
            "Attach a binned PDK library or author L/W bounds in Model Editor.",
        );
        return;
    }
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(
                ui,
                "BIN FAMILIES",
                Some(&format!("{} · library · family", families.len())),
            );
            let selected = app
                .state
                .workbench
                .models_view
                .selected_bin_family
                .clone()
                .unwrap_or_default();
            let mut picked = None;
            let row_height = selectable_label_height(ui);
            ScrollArea::vertical()
                .id_salt("models-bin-families")
                .max_height(ui.available_height().max(120.0))
                .show_rows(ui, row_height, families.len(), |ui, range| {
                    for family in &families[range] {
                        let key = family.key();
                        if ui
                            .selectable_label(
                                selected == key,
                                format!("{key}  ·  {} cards", family.cards.len()),
                            )
                            .clicked()
                        {
                            picked = Some(key);
                        }
                    }
                });
            if let Some(picked) = picked {
                app.state.workbench.models_view.selected_bin_family = Some(picked);
            }
        });
        card(&mut columns[1], |ui| {
            card_title(
                ui,
                "AUDIT FINDINGS",
                Some(&format!("{} · fail closed on overlap", findings.len())),
            );
            if findings.is_empty() {
                ui.label(
                    RichText::new("Every declared envelope is non-overlapping.")
                        .color(Tokens::get(ui.ctx()).color.ok),
                );
            } else {
                for finding in findings.iter().take(FINDING_ROWS) {
                    ui.label(
                        RichText::new(format!("Review: {finding}"))
                            .small()
                            .color(Tokens::get(ui.ctx()).color.err),
                    );
                }
                if findings.len() > FINDING_ROWS {
                    ui.label(
                        RichText::new(format!(
                            "…and {} more, all counted above",
                            findings.len() - FINDING_ROWS
                        ))
                        .small()
                        .color(Tokens::get(ui.ctx()).color.text_faint),
                    );
                }
            }
        });
    });
    let selected = app
        .state
        .workbench
        .models_view
        .selected_bin_family
        .clone()
        .filter(|selected| families.iter().any(|family| family.key() == *selected))
        .or_else(|| families.first().map(BinFamily::key));
    if let Some(selected) = selected {
        app.state.workbench.models_view.selected_bin_family = Some(selected.clone());
        if let Some(family) = families.iter().find(|family| family.key() == selected) {
            geometry_map(ui, &selected, &family.cards);
            geometry_instance_table(ui, family, &inspection);
        }
    }
}

/// Findings the audit reports, comparing only cards the engine would compare.
///
/// The pairwise sweep is quadratic in a family, which is why the family has to
/// be the engine's — a few dozen bins of one device — and not every card of one
/// device type in the whole corpus.
fn geometry_findings(families: &[BinFamily]) -> Vec<String> {
    let mut findings = Vec::new();
    for family in families {
        for (index, left) in family.cards.iter().enumerate() {
            for right in family.cards.iter().skip(index + 1) {
                if left.geometry.overlaps_with_positive_area(right.geometry) {
                    findings.push(format!(
                        "{}/{} overlaps {} across L/W/NFIN geometry",
                        family.family, left.model, right.model
                    ));
                }
            }
        }
    }
    findings
}

fn geometry_map(ui: &mut Ui, family: &str, cards: &[BinCard]) {
    card(ui, |ui| {
        card_title(ui, "LOG L/W MAP", Some(&format!("{family} · NFIN audited")));
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 210.0), Sense::hover());
        let t = Tokens::get(ui.ctx());
        ui.painter().rect(
            rect,
            2.0,
            t.color.bg_inset,
            Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        let plot = rect.shrink(20.0);
        ui.painter().line_segment(
            [plot.left_bottom(), plot.right_bottom()],
            Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().line_segment(
            [plot.left_bottom(), plot.left_top()],
            Stroke::new(1.0, t.color.border_strong),
        );
        let finite = cards
            .iter()
            .filter_map(|card| {
                Some((
                    card.geometry.length.min?.max(f64::MIN_POSITIVE).log10(),
                    card.geometry.length.max?.max(f64::MIN_POSITIVE).log10(),
                    card.geometry.width.min?.max(f64::MIN_POSITIVE).log10(),
                    card.geometry.width.max?.max(f64::MIN_POSITIVE).log10(),
                    card.model.as_str(),
                ))
            })
            .collect::<Vec<_>>();
        let min_l = finite
            .iter()
            .map(|value| value.0)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_l = finite
            .iter()
            .map(|value| value.1)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let min_w = finite
            .iter()
            .map(|value| value.2)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_w = finite
            .iter()
            .map(|value| value.3)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let map_x = |value: f64| {
            plot.left() + plot.width() * ((value - min_l) / (max_l - min_l).max(1e-12)) as f32
        };
        let map_y = |value: f64| {
            plot.bottom() - plot.height() * ((value - min_w) / (max_w - min_w).max(1e-12)) as f32
        };
        for (index, (l0, l1, w0, w1, name)) in finite.iter().enumerate() {
            let bin = egui::Rect::from_min_max(
                egui::pos2(map_x(*l0), map_y(*w1)),
                egui::pos2(map_x(*l1), map_y(*w0)),
            );
            let color = [
                t.color.accent,
                t.color.info,
                t.color.ok,
                t.color.warn,
                t.color.text_dim,
            ][index % 5];
            ui.painter().rect(
                bin,
                1.0,
                color.linear_multiply(0.12),
                Stroke::new(1.2, color),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                bin.center(),
                egui::Align2::CENTER_CENTER,
                *name,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                color,
            );
        }
    });
}

fn geometry_instance_table(
    ui: &mut Ui,
    family: &BinFamily,
    inspection: &rspice_core::engine::ModelBinInspection,
) {
    card(ui, |ui| {
        card_title(
            ui,
            "INSTANCE RESOLUTION",
            Some("prepared design · simulator receipt"),
        );
        let names = family
            .cards
            .iter()
            .map(|card| card.model.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let matches = inspection
            .instances
            .iter()
            .filter(|instance| {
                names.contains(&instance.selected_model.to_ascii_lowercase())
                    || instance
                        .requested_model
                        .eq_ignore_ascii_case(&family.family)
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            empty_state(
                ui,
                "No placed instance resolves to this family.",
                "The exact prepared design contains no simulator-resolved instance for this family.",
            );
            return;
        }
        let row_height = ui.spacing().interact_size.y;
        ScrollArea::vertical()
            .id_salt("models-bin-instances")
            .max_height(ui.available_height().max(120.0))
            .show_rows(ui, row_height, matches.len(), |ui, range| {
                for instance in &matches[range] {
                    let selection = match instance.selection {
                        rspice_core::engine::ModelBinSelectionKind::ExactCard => "exact card",
                        rspice_core::engine::ModelBinSelectionKind::FamilyMatch => "family match",
                        rspice_core::engine::ModelBinSelectionKind::SharedBoundary => {
                            "shared boundary · declaration order"
                        }
                    };
                    let value =
                        format!("{} → {}", instance.requested_model, instance.selected_model);
                    let origin = format!(
                        "{selection} · L={} · W={} · NFIN={} · M={}",
                        optional_bin_value(instance.length),
                        optional_bin_value(instance.width),
                        optional_bin_value(instance.nfin),
                        optional_bin_value(instance.multiplier),
                    );
                    property(ui, &instance.element, &value, &origin);
                }
            });
    });
}

fn optional_bin_value(value: Option<f64>) -> String {
    value.map_or_else(|| "—".to_owned(), |value| format!("{value:.6e}"))
}

#[cfg(test)]
mod tests;

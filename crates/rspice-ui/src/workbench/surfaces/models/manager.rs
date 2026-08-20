//! Mounted Models & PDKs manager composition.
//!
//! The legacy surface remains in the parent module only as reusable rendering
//! and qualification code. This module owns the current six-page workbench,
//! corpus scopes, guarded source/pack actions, and model detail composition.

mod corner_ops;
mod dialogs;
mod hub;
mod specialist_pages;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use egui::{Align, Color32, Layout, RichText, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::state::model_library::{
    ClosureFacts, CornerSectionBinding, CornerSectionDomain, DeviceModel, ModelConsumerScope,
    ModelLibrary, ModelSourceAuthority, PackModelHit, ProcessCorner, closure_facts,
    envelope_is_invalid, model_library_source_digest, short_digest,
};
use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, SymbolDocument, ViewType, explicit_component_model,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app::open_create_subcircuit_bound_symbol_dialog;
use crate::workbench::app_state::design_history::{
    publish_model_library_candidate, publish_model_library_set_candidate,
    publish_model_resolution_candidate, publish_symbol_definition_candidate,
};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::{
    ModelPackFacet, ModelsCatalogScope, ModelsOperationalState, ModelsPage, ModelsWorkbenchDialog,
    ProjectModelFacet, RSpicePartFacet,
};
use crate::workbench::{AppState, RSpiceApp};

const ROW_H: f32 = 24.0;
const HEADER_H: f32 = 27.0;
const CATALOG_FOOT_H: f32 = 26.0;
const CATALOG_LIMIT: usize = 160;
/// Parameter rows a detail column lists before reporting the remainder.
const PARAMETER_ROWS: usize = 24;
/// Consumers the "where used" column lists before reporting the remainder.
const USAGE_ROWS: usize = 12;
const PAGE_TABS_H: f32 = 38.0;
const CATALOG_BAR_H: f32 = 34.0;

enum ManagerAction {
    Command(Command),
    ImportModelSource,
    RefreshLibrary {
        library: String,
    },
    AttachPack {
        pack_id: String,
    },
    AddPart {
        pack_id: String,
        part_name: String,
    },
    #[cfg(target_arch = "wasm32")]
    SelectBrowserImportRoot {
        root: String,
    },
    #[cfg(target_arch = "wasm32")]
    CancelBrowserImportRoot,
    BindComponentModel {
        component_id: u64,
        library: String,
        model: String,
    },
    CreateSubcircuitSymbol {
        library: String,
        subcircuit: String,
    },
    ModelHub(crate::workbench::app::ModelHubRequest),
}

struct ManagerRenderContext<'a> {
    state: &'a mut AppState,
    pending_actions: &'a mut Vec<ManagerAction>,
}

impl ManagerRenderContext<'_> {
    fn queue_command(&mut self, command: Command) {
        self.pending_actions.push(ManagerAction::Command(command));
    }

    fn queue_model_source_import(&mut self) {
        self.pending_actions.push(ManagerAction::ImportModelSource);
    }

    fn queue_library_refresh(&mut self, library: &str) {
        self.pending_actions.push(ManagerAction::RefreshLibrary {
            library: library.to_owned(),
        });
    }

    fn queue_pack_attach(&mut self, pack_id: &str) {
        self.pending_actions.push(ManagerAction::AttachPack {
            pack_id: pack_id.to_owned(),
        });
    }

    fn queue_part_add(&mut self, pack_id: &str, part_name: &str) {
        self.pending_actions.push(ManagerAction::AddPart {
            pack_id: pack_id.to_owned(),
            part_name: part_name.to_owned(),
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn queue_browser_import_root(&mut self, root: String) {
        self.pending_actions
            .push(ManagerAction::SelectBrowserImportRoot { root });
    }

    #[cfg(target_arch = "wasm32")]
    fn queue_cancel_browser_import_root(&mut self) {
        self.pending_actions
            .push(ManagerAction::CancelBrowserImportRoot);
    }

    fn queue_model_binding(&mut self, component_id: u64, library: &str, model: &str) {
        self.pending_actions
            .push(ManagerAction::BindComponentModel {
                component_id,
                library: library.to_owned(),
                model: model.to_owned(),
            });
    }

    fn queue_model_hub(&mut self, request: crate::workbench::app::ModelHubRequest) {
        self.pending_actions.push(ManagerAction::ModelHub(request));
    }

    fn queue_subcircuit_symbol(&mut self, library: &str, subcircuit: &str) {
        self.pending_actions
            .push(ManagerAction::CreateSubcircuitSymbol {
                library: library.to_owned(),
                subcircuit: subcircuit.to_owned(),
            });
    }
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let context = ui.ctx().clone();
    let t = Tokens::get(ui.ctx());
    // The approved manager is one continuous document surface. Individual
    // panes own their padding; the composition itself uses one-pixel dividers.
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    ui.visuals_mut().widgets.noninteractive.bg_fill = t.color.bg_panel;

    let mut pending_actions = Vec::new();
    {
        let mut render = ManagerRenderContext {
            state: &mut app.state,
            pending_actions: &mut pending_actions,
        };
        page_tabs(ui, &mut render);
    }

    if app.state.workbench.models_view.model_import_in_progress {
        ui.horizontal(|ui| {
            ui.add(egui::Spinner::new().size(14.0));
            ui.label(
                app.state
                    .workbench
                    .models_view
                    .model_import_label
                    .as_deref()
                    .unwrap_or("Authenticating and parsing model sources…"),
            );
        });
    }

    let page = app.state.workbench.models_page;
    let include_diagnostics = (page == ModelsPage::Include)
        .then(|| closure_facts(app.state.model_library_manager.libraries_sorted()));
    // Projected before the render borrow, and refreshed on open when the
    // cached catalog is old enough that showing it without checking would be
    // reporting last week's answer to this week's question.
    let hub_catalog = hub::hub_catalog(&app.model_hub);
    if page == ModelsPage::Models
        && hub_catalog.unavailable.is_none()
        && hub_catalog.stale
        && !app.state.workbench.models_view.model_import_in_progress
        && !app.state.workbench.models_view.catalog_refresh_requested
    {
        app.state.workbench.models_view.catalog_refresh_requested = true;
        pending_actions.push(ManagerAction::ModelHub(
            crate::workbench::app::ModelHubRequest::FetchSnapshot,
        ));
    }
    if page == ModelsPage::Qualification {
        qualification_page(ui, app);
    } else {
        let mut render = ManagerRenderContext {
            state: &mut app.state,
            pending_actions: &mut pending_actions,
        };
        match page {
            ModelsPage::Models => catalog_page(ui, &mut render, &hub_catalog),
            ModelsPage::Symbols => specialist_pages::symbols_page(ui, &mut render),
            ModelsPage::Corners => specialist_pages::corners_page(ui, &mut render),
            ModelsPage::Bins => specialist_pages::bins_page(ui, &mut render),
            ModelsPage::Include => specialist_pages::include_page(
                ui,
                &mut render,
                include_diagnostics
                    .as_ref()
                    .expect("include diagnostics are prepared for the include page"),
            ),
            ModelsPage::Qualification => unreachable!("qualification renders above"),
        }
    }

    {
        let mut render = ManagerRenderContext {
            state: &mut app.state,
            pending_actions: &mut pending_actions,
        };
        dialogs::render_dialog(ui, &mut render);
    }
    for action in pending_actions {
        match action {
            ManagerAction::Command(command) => command.execute(app),
            ManagerAction::ImportModelSource => app.start_model_source_import(&context),
            ManagerAction::RefreshLibrary { library } => {
                app.start_model_library_refresh(&context, library);
            }
            ManagerAction::AttachPack { pack_id } => {
                app.start_model_pack_attach(&context, pack_id);
            }
            ManagerAction::AddPart { pack_id, part_name } => {
                app.start_model_part_add(&context, pack_id, part_name);
            }
            #[cfg(target_arch = "wasm32")]
            ManagerAction::SelectBrowserImportRoot { root } => {
                app.select_browser_model_import_root(&context, root);
            }
            #[cfg(target_arch = "wasm32")]
            ManagerAction::CancelBrowserImportRoot => {
                app.cancel_browser_model_import_root();
            }
            ManagerAction::BindComponentModel {
                component_id,
                library,
                model,
            } => {
                let result = crate::workbench::docks::bind_component_model_from_catalog(
                    app,
                    component_id,
                    &library,
                    &model,
                )
                .map(|()| format!("Bound selected instance to model '{library} / {model}'."));
                apply_receipt(&mut app.state, result);
            }
            ManagerAction::ModelHub(request) => {
                app.start_model_hub_operation(&context, request);
            }
            ManagerAction::CreateSubcircuitSymbol {
                library,
                subcircuit,
            } => {
                let result = open_subcircuit_symbol_workflow(&mut app.state, &library, &subcircuit)
                    .map(|()| {
                        format!(
                            "Opened governed symbol creation for subcircuit '{library} / {subcircuit}'."
                        )
                    });
                apply_receipt(&mut app.state, result);
            }
        }
    }
}

fn open_subcircuit_symbol_workflow(
    state: &mut AppState,
    library_name: &str,
    subcircuit_name: &str,
) -> Result<(), String> {
    let provider = state
        .model_library_manager
        .effective_definition_provider(ModelConsumerScope::Subcircuit, subcircuit_name)?
        .ok_or_else(|| {
            format!("Subcircuit '{subcircuit_name}' has no executable catalog provider")
        })?;
    if !provider.library.eq_ignore_ascii_case(library_name) {
        return Err(format!(
            "Subcircuit '{subcircuit_name}' executes from project-global provider '{}', not selected library '{library_name}'",
            provider.library
        ));
    }
    let library = state
        .model_library_manager
        .get_library(&provider.library)
        .ok_or_else(|| {
            format!(
                "Subcircuit provider '{}' no longer exists",
                provider.library
            )
        })?;
    let active_sections = library.active_section_names();
    let interfaces = library
        .subcircuits
        .values()
        .filter(|interface| interface.name.eq_ignore_ascii_case(&provider.definition))
        .filter(|interface| {
            interface.section.as_deref().is_none_or(|section| {
                active_sections
                    .iter()
                    .any(|active| active.eq_ignore_ascii_case(section))
            })
        })
        .cloned()
        .collect::<Vec<_>>();
    let interface = match interfaces.as_slice() {
        [interface] => interface,
        [] => {
            return Err(format!(
                "Subcircuit '{}/{}' is not active in the selected model corner",
                provider.library, provider.definition
            ));
        }
        _ => {
            return Err(format!(
                "Subcircuit '{}/{}' has multiple active interfaces; repair its section contract before symbol creation",
                provider.library, provider.definition
            ));
        }
    };
    let source_path = interface
        .file_path
        .clone()
        .or_else(|| library.root_path.clone())
        .ok_or_else(|| {
            format!(
                "Subcircuit '{}/{}' has no retained implementation source",
                provider.library, provider.definition
            )
        })?;
    open_create_subcircuit_bound_symbol_dialog(
        state,
        provider.library,
        interface.name.clone(),
        source_path,
        interface.ports.clone(),
        interface.section.clone(),
        interface.parameter_defaults.clone(),
    )
}

fn qualification_page(ui: &mut Ui, app: &mut RSpiceApp) {
    let summaries = super::qualification_summaries(app);
    if super::selected_qualification_summary(app, &summaries).is_none()
        && let Some(first) = summaries.first()
    {
        app.state
            .model_library_manager
            .select_library(&first.library);
        app.state.workbench.selected_model = Some(first.model.clone());
    }
    let selected = super::selected_qualification_summary(app, &summaries).cloned();
    let total_vectors = summaries
        .iter()
        .map(|summary| summary.vectors)
        .sum::<usize>();
    let subtitle = format!(
        "{} model families · {} vectors · source-owned release evidence",
        summaries.len(),
        total_vectors
    );
    let mut requested_action = None;
    let compare_blocker = super::qualification_action_block_reason(
        app,
        selected.as_ref(),
        super::QualificationPageAction::CompareRelease,
    );
    let release_blocker = super::qualification_action_block_reason(
        app,
        selected.as_ref(),
        super::QualificationPageAction::ReviewReleaseBinding,
    );
    let run_blocker = super::qualification_action_block_reason(
        app,
        selected.as_ref(),
        super::QualificationPageAction::RunSuite,
    );

    section_title(ui, "Model qualification", &subtitle, |ui| {
        let compare = ui.add_enabled(
            compare_blocker.is_none(),
            egui::Button::new("Compare approved"),
        );
        if let Some(reason) = compare_blocker.as_deref() {
            compare.on_disabled_hover_text(reason);
        } else if compare.clicked() {
            requested_action = Some(super::QualificationPageAction::CompareRelease);
        }

        let release = ui.add_enabled(
            release_blocker.is_none(),
            egui::Button::new("Release closure"),
        );
        if let Some(reason) = release_blocker.as_deref() {
            release.on_disabled_hover_text(reason);
        } else if release.clicked() {
            requested_action = Some(super::QualificationPageAction::ReviewReleaseBinding);
        }

        let run = ui.add_enabled(run_blocker.is_none(), egui::Button::new("Run suite"));
        if let Some(reason) = run_blocker.as_deref() {
            run.on_disabled_hover_text(reason);
        } else if run.clicked() {
            requested_action = Some(super::QualificationPageAction::RunSuite);
        }
    });

    qualification_metric_strip(ui, &summaries);
    qualification_gate_banner(ui, selected.as_ref());
    if summaries.is_empty() {
        page_empty_state(
            ui,
            "No qualification suites are loaded",
            "Attach a project-owned model source and retain versioned vectors before making release claims.",
        );
    } else {
        let available = ui.available_size();
        let left_width = (available.x * 0.34).clamp(220.0, 310.0);
        let right_width = (available.x - left_width - 1.0).max(260.0);
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(left_width, available.y),
                Layout::top_down(Align::Min),
                |ui| {
                    qualification_suite_rail(
                        ui,
                        app,
                        &summaries,
                        selected.as_ref(),
                        &mut requested_action,
                    );
                },
            );
            ui.separator();
            ui.allocate_ui_with_layout(
                egui::vec2(right_width, available.y),
                Layout::top_down(Align::Min),
                |ui| {
                    qualification_selected_contract(
                        ui,
                        app,
                        selected.as_ref(),
                        &mut requested_action,
                    )
                },
            );
        });
    }

    if let Some(action) = requested_action {
        super::execute_qualification_action(app, action);
    }
}

fn qualification_metric_strip(ui: &mut Ui, summaries: &[super::QualificationModelSummary]) {
    let t = Tokens::get(ui.ctx());
    let total_vectors = summaries
        .iter()
        .map(|summary| summary.vectors)
        .sum::<usize>();
    let passing_vectors = summaries
        .iter()
        .map(|summary| summary.passing_vectors)
        .sum::<usize>();
    let evidenced_vectors = summaries
        .iter()
        .map(|summary| summary.evidenced_vectors)
        .sum::<usize>();
    let open_dispositions = summaries
        .iter()
        .map(|summary| summary.open_dispositions)
        .sum::<usize>();
    let qualified = summaries
        .iter()
        .filter(|summary| summary.gate == super::QualificationGate::Qualified)
        .count();
    let parity = summaries
        .iter()
        .filter(|summary| summary.suites > 0 && summary.parity_suites == summary.suites)
        .count();
    let metrics = [
        (
            "VECTORS PASSING",
            format!("{passing_vectors} / {total_vectors}"),
            format!("{open_dispositions} open dispositions"),
            if total_vectors > 0 && passing_vectors == total_vectors {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        (
            "REFERENCE COVERAGE",
            format!("{evidenced_vectors} / {total_vectors}"),
            "exact retained evidence".to_owned(),
            if total_vectors > 0 && evidenced_vectors == total_vectors {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        (
            "QUALIFIED MODELS",
            format!("{qualified} / {}", summaries.len()),
            "source-owned release gates".to_owned(),
            if !summaries.is_empty() && qualified == summaries.len() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        (
            "RUNTIME PARITY",
            format!("{parity} / {}", summaries.len()),
            "desktop · WebAssembly".to_owned(),
            if !summaries.is_empty() && parity == summaries.len() {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
    ];
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 62.0), Sense::hover());
    let width = rect.width() / metrics.len() as f32;
    for (index, (label, value, detail, color)) in metrics.iter().enumerate() {
        let cell = egui::Rect::from_min_max(
            egui::pos2(rect.left() + width * index as f32, rect.top()),
            egui::pos2(rect.left() + width * (index + 1) as f32, rect.bottom()),
        );
        ui.painter().rect(
            cell,
            0.0,
            t.color.bg_panel,
            Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            egui::pos2(cell.left() + 9.0, cell.top() + 12.0),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_faint,
        );
        ui.painter().text(
            egui::pos2(cell.left() + 9.0, cell.top() + 33.0),
            egui::Align2::LEFT_CENTER,
            value,
            theme::mono(tokens::FS_1, FontWeight::SemiBold),
            *color,
        );
        ui.painter().text(
            egui::pos2(cell.left() + 9.0, cell.bottom() - 9.0),
            egui::Align2::LEFT_CENTER,
            elide(ui, detail, (cell.width() - 18.0).max(1.0), false),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
}

fn qualification_gate_banner(ui: &mut Ui, selected: Option<&super::QualificationModelSummary>) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 54.0), Sense::hover());
    ui.painter().rect(
        rect,
        0.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.top() + 16.0),
        egui::Align2::LEFT_CENTER,
        "Gate ownership",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.bottom() - 13.0),
        egui::Align2::LEFT_CENTER,
        elide(
            ui,
            "Release closure consumes exact source revisions, retained references, runtime parity, and governed dispositions.",
            (rect.width() - 150.0).max(1.0),
            false,
        ),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let gate = selected.map(|summary| summary.gate);
    let color = gate.map_or(t.color.text_faint, |gate| {
        qualification_gate_color(gate, &t)
    });
    let label = gate.map_or("NO SELECTION", super::QualificationGate::label);
    ui.painter().text(
        egui::pos2(rect.right() - 12.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        label.to_uppercase(),
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        color,
    );
}

fn qualification_suite_rail(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    summaries: &[super::QualificationModelSummary],
    selected: Option<&super::QualificationModelSummary>,
    requested_action: &mut Option<super::QualificationPageAction>,
) {
    let selected_key = selected.map(|summary| summary.key.as_str());
    detail_pane(
        ui,
        "MODEL SUITES",
        Some(&format!("{} source revisions", summaries.len())),
        |ui| {
            let list_height = (ui.available_height() - 154.0).max(130.0);
            // The rail lists every model in every loaded library, which on a
            // foundry corpus is thousands of fixed-height rows.
            const RAIL_ROW_H: f32 = 38.0;
            ScrollArea::vertical()
                .id_salt("models-qualification-suite-rail")
                .max_height(list_height)
                .show_rows(ui, RAIL_ROW_H, summaries.len(), |ui, range| {
                    for summary in &summaries[range] {
                        let t = Tokens::get(ui.ctx());
                        let selected = selected_key == Some(summary.key.as_str());
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), RAIL_ROW_H),
                            Sense::click(),
                        );
                        if selected {
                            ui.painter().rect_filled(
                                rect,
                                0.0,
                                t.color.accent.linear_multiply(0.14),
                            );
                            ui.painter().vline(
                                rect.left(),
                                rect.y_range(),
                                Stroke::new(2.0, t.color.accent),
                            );
                        } else if response.hovered() {
                            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                        }
                        ui.painter().text(
                            egui::pos2(rect.left() + 8.0, rect.top() + 12.0),
                            egui::Align2::LEFT_CENTER,
                            elide(ui, &summary.model, rect.width() - 92.0, true),
                            theme::mono(tokens::FS_0, FontWeight::SemiBold),
                            t.color.text,
                        );
                        ui.painter().text(
                            egui::pos2(rect.left() + 8.0, rect.bottom() - 9.0),
                            egui::Align2::LEFT_CENTER,
                            elide(
                                ui,
                                &format!("{} · {} vectors", summary.library, summary.vectors),
                                rect.width() - 92.0,
                                false,
                            ),
                            theme::sans(tokens::FS_0, FontWeight::Regular),
                            t.color.text_faint,
                        );
                        ui.painter().text(
                            egui::pos2(rect.right() - 8.0, rect.center().y),
                            egui::Align2::RIGHT_CENTER,
                            summary.gate.label(),
                            theme::mono(tokens::FS_0, FontWeight::SemiBold),
                            qualification_gate_color(summary.gate, &t),
                        );
                        let row_label = format!(
                            "{} in {}, {} vectors, {}",
                            summary.model,
                            summary.library,
                            summary.vectors,
                            summary.gate.label()
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::SelectableLabel,
                                ui.is_enabled(),
                                selected,
                                row_label.clone(),
                            )
                        });
                        theme::paint_focus_ring(ui, &response, rect);
                        if response.clicked() {
                            app.state
                                .model_library_manager
                                .select_library(&summary.library);
                            app.state.workbench.selected_model = Some(summary.model.clone());
                        }
                    }
                });
            ui.separator();
            if let Some(selected) = selected {
                property(ui, "Selected", &selected.model, &selected.library);
                property(
                    ui,
                    "Source",
                    &selected.source_revision,
                    if selected.source_error.is_none() {
                        "retained"
                    } else {
                        "review"
                    },
                );
                if ui.button("Review qualification").clicked() {
                    *requested_action = Some(super::QualificationPageAction::ReviewVectors);
                }
                if ui.button("Measurement correlation").clicked() {
                    *requested_action = Some(super::QualificationPageAction::OpenCorrelation);
                }
            }
        },
    );
}

fn qualification_selected_contract(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&super::QualificationModelSummary>,
    requested_action: &mut Option<super::QualificationPageAction>,
) {
    let Some(selected) = selected else {
        page_empty_state(
            ui,
            "Select a qualification suite",
            "Choose a model family to inspect its dispositions, tolerance policy, and retained evidence.",
        );
        return;
    };
    ScrollArea::vertical()
        .id_salt("models-qualification-selected-contract")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            detail_pane(
                ui,
                "OPEN DISPOSITIONS",
                Some(&format!("{} pending", selected.open_dispositions)),
                |ui| {
                    if selected.domains.is_empty() {
                        property(ui, "Domains", "not configured", "suite contract");
                    } else {
                        for domain in &selected.domains {
                            property(
                                ui,
                                domain.domain.label(),
                                &domain.disposition,
                                &domain.reference_coverage,
                            );
                        }
                        let blocker = super::qualification_action_block_reason(
                            app,
                            Some(selected),
                            super::QualificationPageAction::ReviewVectors,
                        );
                        let review = ui.add_enabled(
                            blocker.is_none(),
                            egui::Button::new("Review dispositions"),
                        );
                        if let Some(reason) = blocker.as_deref() {
                            review.on_disabled_hover_text(reason);
                        } else if review.clicked() {
                            *requested_action = Some(super::QualificationPageAction::ReviewVectors);
                        }
                    }
                },
            );
            detail_pane(
                ui,
                "TOLERANCE POLICY",
                Some("domain-owned contracts"),
                |ui| {
                    if selected.domains.is_empty() {
                        property(ui, "Policy", "not declared", "fail closed");
                    } else {
                        for domain in &selected.domains {
                            property(
                                ui,
                                domain.domain.label(),
                                &domain.tolerance,
                                &format!("{} vectors", domain.vectors),
                            );
                        }
                    }
                },
            );
            detail_pane(
                ui,
                "QUALIFICATION CONTRACT",
                Some(selected.gate.label()),
                |ui| {
                    property(
                        ui,
                        "Model revision",
                        &selected.source_revision,
                        "exact source",
                    );
                    property(
                        ui,
                        "Runtime parity",
                        &format!(
                            "desktop {}/{} · WASM {}/{}",
                            selected.desktop_passing,
                            selected.vectors,
                            selected.wasm_passing,
                            selected.vectors
                        ),
                        &format!("{} suites", selected.parity_suites),
                    );
                    property(
                        ui,
                        "Evidence set",
                        selected
                            .evidence_digest
                            .as_deref()
                            .unwrap_or("not retained"),
                        &format!("{} references", selected.references),
                    );
                    property(
                        ui,
                        "Correlation",
                        &selected.correlation_status,
                        selected
                            .correlation_evidence_digest
                            .as_deref()
                            .unwrap_or("not retained"),
                    );
                    property(
                        ui,
                        "Approved releases",
                        &selected.releases.to_string(),
                        "source-owned",
                    );
                },
            );
        });
}

fn qualification_gate_color(gate: super::QualificationGate, t: &Tokens) -> Color32 {
    match gate {
        super::QualificationGate::Qualified => t.color.ok,
        super::QualificationGate::Review | super::QualificationGate::Unqualified => t.color.warn,
        super::QualificationGate::Blocked => t.color.err,
    }
}

fn page_tabs(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::ZERO)
        .show(ui, |ui| {
            ui.set_height(PAGE_TABS_H);
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.max_rect().bottom() - 0.5,
                Stroke::new(1.0, t.color.border),
            );
            ScrollArea::horizontal()
                .id_salt("models-pdks-page-tabs")
                .max_height(PAGE_TABS_H)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        for page in ModelsPage::ALL {
                            let selected = app.state.workbench.models_page == page;
                            let font = theme::sans(tokens::FS_1, FontWeight::Regular);
                            let text_width = ui
                                .painter()
                                .layout_no_wrap(page.label().to_owned(), font.clone(), t.color.text)
                                .size()
                                .x;
                            let response = ui.add_sized(
                                [text_width + 24.0, PAGE_TABS_H - 1.0],
                                egui::Button::new(RichText::new(page.label()).font(font).color(
                                    if selected {
                                        t.color.text
                                    } else {
                                        t.color.text_dim
                                    },
                                ))
                                .frame(false),
                            );
                            if selected {
                                ui.painter().hline(
                                    (response.rect.left() + 9.0)..=(response.rect.right() - 9.0),
                                    response.rect.bottom() - 1.0,
                                    Stroke::new(2.0, t.color.accent),
                                );
                            }
                            if response.clicked() {
                                app.state.workbench.models_page = page;
                            }
                        }
                    });
                });
        });
}

fn catalog_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &hub::HubCatalog) {
    // The project scope is the only one derived from the loaded corpus, so it
    // is the only one that scans; the pack scopes ask the pack index. Deriving
    // it here means the facet chips above the table and the table itself are
    // one pass over the corpus, not six.
    let scan = matches!(
        app.state.workbench.models_view.catalog_scope,
        ModelsCatalogScope::Project
    )
    .then(|| project_catalog_scan(app, &ConsumerIndex::build(app)));
    catalog_bar(ui, app, scan.as_ref());
    match app.state.workbench.models_view.catalog_scope {
        ModelsCatalogScope::Project => {
            project_catalog(ui, app, scan.as_ref().expect("the project scope scans"));
        }
        ModelsCatalogScope::InstalledPacks => hub::packs_page(ui, app, hub),
        ModelsCatalogScope::RSpiceLibrary => parts_catalog(ui, app),
    }
}

fn catalog_bar(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, scan: Option<&ProjectCatalogScan>) {
    let loaded = app.state.model_library_manager.total_model_count();
    let pack_rows = app
        .state
        .model_library_manager
        .spice_packs()
        .map(|index| index.packs().to_vec())
        .unwrap_or_default();
    let packs = pack_rows.len();
    let parts = app.state.model_library_manager.pack_definition_count();
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(7, 0))
        .show(ui, |ui| {
            ui.set_height(CATALOG_BAR_H);
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                egui::Frame::NONE
                    .stroke(Stroke::new(1.0, t.color.border))
                    .corner_radius(5.0)
                    .inner_margin(egui::Margin::ZERO)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::ZERO;
                            for (scope, count) in [
                                (ModelsCatalogScope::Project, loaded),
                                (ModelsCatalogScope::InstalledPacks, packs),
                                (ModelsCatalogScope::RSpiceLibrary, parts),
                            ] {
                                let selected =
                                    app.state.workbench.models_view.catalog_scope == scope;
                                let response = ui.add_sized(
                                    [scope_segment_width(scope), 20.0],
                                    egui::Button::new(
                                        RichText::new(format!("{}  {count}", scope.label()))
                                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                            .color(if selected {
                                                t.color.text
                                            } else {
                                                t.color.text_dim
                                            }),
                                    )
                                    .fill(if selected {
                                        t.color.bg_active
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .stroke(Stroke::NONE)
                                    .corner_radius(4.0),
                                );
                                if response.clicked() {
                                    app.state.workbench.models_view.catalog_scope = scope;
                                    app.state.workbench.models_view.catalog_query.clear();
                                    app.state.workbench.models_view.part_catalog_offset = 0;
                                    app.state.workbench.models_view.selected_part = None;
                                }
                            }
                        });
                    });

                // The source mockup drops the facet rail before it can squeeze
                // the scope switcher or search field in the document column.
                if ui.available_width() >= 560.0 {
                    ScrollArea::horizontal()
                        .id_salt("models-catalog-facets")
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            match app.state.workbench.models_view.catalog_scope {
                                ModelsCatalogScope::Project => {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 4.0;
                                        for (index, facet) in
                                            ProjectModelFacet::ALL.into_iter().enumerate()
                                        {
                                            let count =
                                                scan.map(|scan| scan.facets[index]).unwrap_or(0);
                                            if facet_button(
                                                ui,
                                                app.state.workbench.models_view.project_facet
                                                    == facet,
                                                facet.label(),
                                                Some(count),
                                            )
                                            .clicked()
                                            {
                                                app.state.workbench.models_view.project_facet =
                                                    facet;
                                            }
                                        }
                                    });
                                }
                                ModelsCatalogScope::InstalledPacks => {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 4.0;
                                        for facet in ModelPackFacet::ALL {
                                            let count =
                                                hub::pack_facet_count(app, &pack_rows, facet);
                                            if facet_button(
                                                ui,
                                                app.state.workbench.models_view.pack_facet == facet,
                                                facet.label(),
                                                Some(count),
                                            )
                                            .clicked()
                                            {
                                                app.state.workbench.models_view.pack_facet = facet;
                                            }
                                        }
                                    });
                                }
                                ModelsCatalogScope::RSpiceLibrary => {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 4.0;
                                        for facet in RSpicePartFacet::ALL {
                                            if facet_button(
                                                ui,
                                                app.state.workbench.models_view.part_facet == facet,
                                                facet.label(),
                                                None,
                                            )
                                            .clicked()
                                            {
                                                app.state.workbench.models_view.part_facet = facet;
                                                app.state.workbench.models_view.selected_part =
                                                    None;
                                                app.state
                                                    .workbench
                                                    .models_view
                                                    .part_catalog_offset = 0;
                                            }
                                        }
                                    });
                                }
                            }
                        });
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let hint = match app.state.workbench.models_view.catalog_scope {
                        ModelsCatalogScope::Project => {
                            "Search models, parameters or consumers…".to_owned()
                        }
                        ModelsCatalogScope::InstalledPacks => "Search installed packs…".to_owned(),
                        ModelsCatalogScope::RSpiceLibrary => {
                            format!("Search {} parts by name, class or pack…", parts)
                        }
                    };
                    let query = ui.add_sized(
                        [ui.available_width().clamp(170.0, 340.0), 24.0],
                        egui::TextEdit::singleline(
                            &mut app.state.workbench.models_view.catalog_query,
                        )
                        .hint_text(hint),
                    );
                    if query.changed() {
                        app.state.workbench.models_view.part_catalog_offset = 0;
                        app.state.workbench.models_view.selected_part = None;
                    }
                });
            });
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.max_rect().bottom() - 0.5,
                Stroke::new(1.0, t.color.border),
            );
        });
}

fn scope_segment_width(scope: ModelsCatalogScope) -> f32 {
    match scope {
        ModelsCatalogScope::Project => 74.0,
        ModelsCatalogScope::InstalledPacks => 112.0,
        ModelsCatalogScope::RSpiceLibrary => 108.0,
    }
}

fn facet_button(ui: &mut Ui, selected: bool, label: &str, count: Option<usize>) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let label = count.map_or_else(|| label.to_owned(), |count| format!("{label}  {count}"));
    ui.add_sized(
        [
            (ui.painter()
                .layout_no_wrap(
                    label.clone(),
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_dim,
                )
                .size()
                .x
                + 16.0)
                .max(44.0),
            22.0,
        ],
        egui::Button::new(
            RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(if selected {
                    t.color.text
                } else {
                    t.color.text_dim
                }),
        )
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::new(
            1.0,
            if selected {
                t.color.accent
            } else {
                t.color.border
            },
        ))
        .corner_radius(11.0),
    )
}

struct ProjectModelRow {
    library: String,
    model: String,
    family: &'static str,
    source: String,
    pinned: bool,
    review: bool,
    /// The first consumer, which is all the row column shows.
    usage: Option<String>,
    vectors: usize,
}

/// Which schematic instances name which model.
///
/// One pass over the placed components per frame instead of one per model.
/// Deriving it per model made the catalog cost the product of the corpus and
/// the schematic, and the facet chips paid it again for every facet.
struct ConsumerIndex {
    by_provider: BTreeMap<(String, String, String), Vec<String>>,
    diagnostics: Vec<String>,
}

impl ConsumerIndex {
    fn build(app: &ManagerRenderContext<'_>) -> Self {
        let mut by_provider = BTreeMap::<(String, String, String), Vec<String>>::new();
        let mut diagnostics = Vec::new();
        let Some(schematic) = app.state.workspace.active_schematic() else {
            return Self {
                by_provider,
                diagnostics,
            };
        };
        for component in &schematic.components {
            let Some(model) = explicit_component_model(component) else {
                continue;
            };
            let label = format!(
                "{} · {} · ({}, {})",
                component.name,
                component.kind.display_name(),
                component.pos.x,
                component.pos.y
            );
            let declared_provider = crate::state::parse_params_string(&component.params)
                .get("model_library")
                .map(|provider| provider.trim().to_owned())
                .filter(|provider| !provider.is_empty());
            match app
                .state
                .model_library_manager
                .effective_definition_provider(ModelConsumerScope::PrimitiveModel, &model)
            {
                Ok(Some(provider))
                    if declared_provider.as_deref().is_none_or(|declared| {
                        declared.eq_ignore_ascii_case(&provider.library)
                    }) =>
                {
                    by_provider
                        .entry((
                            provider.library.to_ascii_lowercase(),
                            provider.definition.to_ascii_lowercase(),
                            provider.source_digest.to_string(),
                        ))
                        .or_default()
                        .push(label);
                }
                Ok(Some(provider)) => diagnostics.push(format!(
                    "{} declares provider '{}' for model '{}', but the executable provider is '{}'",
                    component.name,
                    declared_provider.as_deref().unwrap_or_default(),
                    model,
                    provider.library
                )),
                Ok(None) => diagnostics.push(format!(
                    "{} references model '{}', which has no executable provider",
                    component.name, model
                )),
                Err(error) => diagnostics.push(format!("{}: {error}", component.name)),
            }
        }
        for consumers in by_provider.values_mut() {
            consumers.sort();
        }
        diagnostics.sort();
        Self {
            by_provider,
            diagnostics,
        }
    }

    fn of(&self, library: &ModelLibrary, model_name: &str) -> &[String] {
        self.by_provider
            .get(&(
                library.name.to_ascii_lowercase(),
                model_name.to_ascii_lowercase(),
                model_library_source_digest(library).to_string(),
            ))
            .map_or(&[], Vec::as_slice)
    }
}

/// Everything the catalog page derives from the corpus, derived once.
///
/// The facet chips and the table used to walk the corpus separately, which is
/// how a chip could count rows the table did not show. They read this instead,
/// so the count and the rows are the same pass.
struct ProjectCatalogScan {
    rows: Vec<ProjectModelRow>,
    facets: [usize; ProjectModelFacet::ALL.len()],
    review: usize,
    consumer_diagnostics: Vec<String>,
}

fn project_catalog_scan(
    app: &ManagerRenderContext<'_>,
    consumers: &ConsumerIndex,
) -> ProjectCatalogScan {
    let query = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    let facet = app.state.workbench.models_view.project_facet;
    let mut rows = Vec::new();
    let mut facets = [0usize; ProjectModelFacet::ALL.len()];
    let mut review_shown = 0usize;
    for library in app.state.model_library_manager.libraries_sorted() {
        let pinned = model_is_pinned(library);
        let protected = matches!(library.source_authority, ModelSourceAuthority::BuiltIn);
        let library_source = library.root_path.as_deref().map(path_label);
        for model in library.models.values() {
            let usages = consumers.of(library, &model.name);
            let review = model_needs_review(library, model);
            let matches = |facet: ProjectModelFacet| match facet {
                ProjectModelFacet::All => true,
                ProjectModelFacet::Bound => !usages.is_empty(),
                ProjectModelFacet::Pinned => pinned,
                ProjectModelFacet::Review => review,
                ProjectModelFacet::Protected => protected,
            };
            for (index, candidate) in ProjectModelFacet::ALL.into_iter().enumerate() {
                facets[index] += usize::from(matches(candidate));
            }
            if !matches(facet) {
                continue;
            }
            if !query.is_empty() && !model_matches_query(library, model, &query) {
                continue;
            }
            if review {
                review_shown += 1;
            }
            rows.push(ProjectModelRow {
                library: library.name.clone(),
                model: model.name.clone(),
                family: model.model_type.display_name(),
                source: model
                    .file_path
                    .as_deref()
                    .map(path_label)
                    .or_else(|| library_source.clone())
                    .unwrap_or_else(|| match library.source_authority {
                        ModelSourceAuthority::BuiltIn => "RSpice built-in".to_owned(),
                        ModelSourceAuthority::External => "external source".to_owned(),
                        ModelSourceAuthority::RetainedImport { .. } => "retained import".to_owned(),
                        ModelSourceAuthority::ProjectOwned { .. } => "project source".to_owned(),
                    }),
                pinned,
                review,
                usage: usages.first().cloned(),
                vectors: library
                    .model_qualification
                    .get(&model.name)
                    .map_or(0, |qualification| {
                        qualification
                            .suites
                            .iter()
                            .map(|suite| suite.vectors.len())
                            .sum()
                    }),
            });
        }
    }
    rows.sort_by(|left, right| {
        case_folded_cmp(&left.model, &right.model).then_with(|| left.library.cmp(&right.library))
    });
    ProjectCatalogScan {
        rows,
        facets,
        review: review_shown,
        consumer_diagnostics: consumers.diagnostics.clone(),
    }
}

/// Order two names exactly as comparing their `to_ascii_lowercase` would,
/// without allocating either one.
///
/// The catalog lower-cased both sides inside the comparator, which allocates
/// two strings per comparison — well over a hundred thousand allocations to
/// order one frame of a corpus-sized catalog. Folding ASCII per byte, rather
/// than reaching for `char::to_lowercase`, keeps the ordering identical to the
/// one this replaces; full Unicode folding would be both slower and a
/// different order.
fn case_folded_cmp(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

/// Whether a search term appears anywhere a catalog row reports.
///
/// Matched field by field rather than by joining the card into one haystack:
/// the join allocated a string per model per frame, including the whole
/// parameter map, and was thrown away immediately.
fn model_matches_query(library: &ModelLibrary, model: &DeviceModel, query: &str) -> bool {
    let contains = |field: &str| {
        field.len() >= query.len()
            && field
                .as_bytes()
                .windows(query.len())
                .any(|window| window.eq_ignore_ascii_case(query.as_bytes()))
    };
    contains(&model.name)
        || contains(&model.description)
        || contains(model.model_type.display_name())
        || contains(&library.name)
        || model.parameters.keys().any(|parameter| contains(parameter))
}

fn project_catalog(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, scan: &ProjectCatalogScan) {
    let rows = &scan.rows;
    if let Some(first) = scan.consumer_diagnostics.first() {
        let suffix = if scan.consumer_diagnostics.len() > 1 {
            format!(
                " · {} additional binding diagnostics",
                scan.consumer_diagnostics.len() - 1
            )
        } else {
            String::new()
        };
        ui.label(
            RichText::new(format!("Binding unresolved: {first}{suffix}"))
                .small()
                .color(Tokens::get(ui.ctx()).color.warn),
        );
    }
    let selected_visible = rows.iter().any(|row| {
        app.state.model_library_manager.selected_library.as_deref() == Some(row.library.as_str())
            && app.state.workbench.selected_model.as_deref() == Some(row.model.as_str())
    });
    if !selected_visible && let Some(row) = rows.first() {
        app.state.model_library_manager.select_library(&row.library);
        app.state.workbench.selected_model = Some(row.model.clone());
    }

    let table_h = (ui.available_height() * 0.36).max(120.0);
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
        table_header(
            ui,
            &[
                ("MODEL", 0.20),
                ("FAMILY", 0.17),
                ("SOURCE", 0.22),
                ("USED BY", 0.16),
                ("VECTORS", 0.10),
                ("STATUS", 0.15),
            ],
        );
        let table = ScrollArea::vertical()
            .id_salt("models-project-table")
            .max_height(table_h);
        if rows.is_empty() {
            table.show(ui, |ui| {
                empty_state(
                    ui,
                    "No models match the current catalog filter.",
                    "Search covers names, families, sources, libraries, consumers and resolved parameters.",
                );
                if ui.button("Clear filter").clicked() {
                    app.state.workbench.models_view.catalog_query.clear();
                    app.state.workbench.models_view.project_facet = ProjectModelFacet::All;
                }
            });
        } else {
            // A corpus-sized catalog builds a widget per row otherwise, and a
            // foundry PDK has thousands. Rows here are a fixed height, which
            // is what lets the scroll area place the scrollbar correctly while
            // building only the rows on screen.
            table.show_rows(ui, ROW_H, rows.len(), |ui, range| {
                for row in &rows[range] {
                    project_model_row(ui, app, row);
                }
            });
        }
        });
    catalog_footer(
        ui,
        rows.len(),
        app.state.model_library_manager.total_model_count(),
        scan.review,
        "project models",
    );

    selected_model_detail(ui, app);
}

/// Whether a model's source is retained well enough to reproduce a run.
fn model_is_pinned(library: &ModelLibrary) -> bool {
    !library.source_closure.is_empty() || library.source_authority.is_project_owned()
}

/// Whether a model has something an engineer must actually look at.
///
/// Both the catalog rows and the facet counts read this, because when they
/// each carried their own copy the chip could count rows the table did not
/// show. A missing `description` is deliberately *not* a finding: prose is
/// optional on a `.model` card, nearly every real PDK card omits it, and
/// flagging it turned the whole catalog amber and made the flag mean nothing.
fn model_needs_review(library: &ModelLibrary, model: &DeviceModel) -> bool {
    envelope_is_invalid(model) || (library.root_path.is_some() && library.source_closure.is_empty())
}

fn project_model_row(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, row: &ProjectModelRow) {
    let selected = app.state.model_library_manager.selected_library.as_deref()
        == Some(row.library.as_str())
        && app.state.workbench.selected_model.as_deref() == Some(row.model.as_str());
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let row_label = format!("{} in {}", row.model, row.library);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            row_label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
    if response.clicked() {
        app.state.model_library_manager.select_library(&row.library);
        app.state.workbench.selected_model = Some(row.model.clone());
    }
    let status = if row.review {
        "review"
    } else if row.pinned {
        "pinned"
    } else {
        ""
    };
    paint_columns(
        ui,
        rect,
        &[
            (&row.model, 0.20, true),
            (row.family, 0.17, false),
            (&row.source, 0.22, false),
            (row.usage.as_deref().unwrap_or(""), 0.16, false),
            (&row.vectors.to_string(), 0.10, true),
            (status, 0.15, true),
        ],
    );
}

fn catalog_footer(ui: &mut Ui, shown: usize, total: usize, review: usize, noun: &str) {
    catalog_footer_capped(ui, shown, total, review, noun, false);
}

/// The catalog footer, told whether the result set was cut off.
///
/// "160 shown · 260000 parts" reads as a filter that matched 160. When the
/// number is a search cap instead, a reader has no way to tell whether the
/// part they want is one of the ones dropped, so the cap says so.
fn catalog_footer_capped(
    ui: &mut Ui,
    shown: usize,
    total: usize,
    review: usize,
    noun: &str,
    capped: bool,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), CATALOG_FOOT_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.top() + 0.5,
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        if capped {
            format!("first {shown} of {total} {noun} · narrow the search to see the rest")
        } else {
            format!("{shown} shown · {total} {noun}")
        },
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if capped {
            t.color.warn
        } else {
            t.color.text_faint
        },
    );
    let state = if review == 0 {
        "No open review"
    } else if review == 1 {
        "1 item needs review"
    } else {
        ""
    };
    let state = if state.is_empty() {
        format!("{review} items need review")
    } else {
        state.to_owned()
    };
    ui.painter().text(
        egui::pos2(rect.right() - 12.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        state,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if review == 0 {
            t.color.ok
        } else {
            t.color.warn
        },
    );
}

fn parts_catalog_footer(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    shown: usize,
    total: usize,
    review: usize,
) {
    let t = Tokens::get(ui.ctx());
    let offset = app.state.workbench.models_view.part_catalog_offset;
    let start = if shown == 0 { 0 } else { offset + 1 };
    let end = offset.saturating_add(shown).min(total);
    let page_count = total.div_ceil(CATALOG_LIMIT).max(1);
    let page = (offset / CATALOG_LIMIT).saturating_add(1).min(page_count);
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(12, 3))
        .show(ui, |ui| {
            ui.set_min_height(CATALOG_FOOT_H - 6.0);
            ui.horizontal_centered(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Showing {start}–{end} of {total} addressable parts"
                    ))
                    .small()
                    .color(t.color.text_faint),
                );
                if review > 0 {
                    ui.label(
                        RichText::new(format!("· {review} on this page need review"))
                            .small()
                            .color(t.color.warn),
                    );
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add_enabled(end < total, egui::Button::new("Next"))
                        .clicked()
                    {
                        app.state.workbench.models_view.part_catalog_offset =
                            offset.saturating_add(CATALOG_LIMIT);
                        app.state.workbench.models_view.selected_part = None;
                    }
                    ui.label(
                        RichText::new(format!("Page {page} of {page_count}"))
                            .monospace()
                            .small()
                            .color(t.color.text_dim),
                    );
                    if ui
                        .add_enabled(offset > 0, egui::Button::new("Previous"))
                        .clicked()
                    {
                        app.state.workbench.models_view.part_catalog_offset =
                            offset.saturating_sub(CATALOG_LIMIT);
                        app.state.workbench.models_view.selected_part = None;
                    }
                });
            });
        });
}

fn compact_button(label: &str) -> egui::Button<'_> {
    egui::Button::new(RichText::new(label).font(theme::sans(tokens::FS_0, FontWeight::Regular)))
        .min_size(egui::vec2(0.0, 24.0))
}

fn selected_model_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let Some(library_name) = app.state.model_library_manager.selected_library.clone() else {
        empty_state(
            ui,
            "Select a model to inspect its exact source and resolved contract.",
            "The detail area never invents a model when the catalog selection is empty.",
        );
        return;
    };
    let Some(model_name) = app.state.workbench.selected_model.clone() else {
        empty_state(
            ui,
            "Select a model to inspect its exact source and resolved contract.",
            "Use the table above or choose a model source in the Navigator.",
        );
        return;
    };
    let Some((library, model)) = app
        .state
        .model_library_manager
        .get_library(&library_name)
        .and_then(|library| library.get_model(&model_name).map(|model| (library, model)))
        .map(|(library, model)| (library.clone(), model.clone()))
    else {
        empty_state(
            ui,
            "The selected model no longer resolves.",
            "Rescan or clear the selection; stale identities are never retargeted automatically.",
        );
        return;
    };
    let usages = model_consumers_for_provider(app, &library, &model.name);
    let selected_component = exactly_one_selected_component(app);
    let binding_block_reason = selected_component.map_or_else(
        || Some("Select exactly one compatible schematic instance first.".to_owned()),
        |component_id| {
            crate::workbench::docks::validate_component_model_catalog_binding(
                app.state,
                component_id,
                &library.name,
                &model.name,
            )
            .err()
        },
    );
    let source_available = !library.source_contents.is_empty() || model.file_path.is_some();

    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(
                    RichText::new(&model.name)
                        .monospace()
                        .font(theme::mono(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    RichText::new(format!(
                        "{} · {} · {}",
                        model.model_type.display_name(),
                        model.level.display_name(),
                        library.name
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let blocked = pin_source_block_reason(
                    library.root_path.is_some(),
                    app.state.workbench.models_view.model_import_in_progress,
                );
                if ui
                    .add_enabled(
                        blocked.is_none(),
                        compact_button(if library.source_closure.is_empty() {
                            "Pin source"
                        } else {
                            "Refresh pin"
                        }),
                    )
                    .on_disabled_hover_text(blocked.unwrap_or_default())
                    .clicked()
                {
                    refresh_library(app, &library);
                }
                if ui
                    .add_enabled(source_available, compact_button("Open source"))
                    .on_disabled_hover_text("This built-in definition has no source document.")
                    .clicked()
                {
                    open_model_source(app, &library, &model);
                }
                if ui.add(compact_button("Compare…")).clicked() {
                    open_model_compare(app, &library.name, &model.name);
                }
                let project_owned = library.source_authority.is_project_owned();
                if ui
                    .add(compact_button(if project_owned {
                        "Model editor…"
                    } else {
                        "Author project copy…"
                    }))
                    .clicked()
                {
                    if project_owned {
                        app.queue_command(Command::ModelEditor);
                    } else {
                        app.queue_command(Command::ModelCreateProjectCopy);
                    }
                }
                if ui.add(compact_button("Qualification")).clicked() {
                    app.state.workbench.models_page = ModelsPage::Qualification;
                }
                let bind = ui.add_enabled(
                    selected_component.is_some() && binding_block_reason.is_none(),
                    compact_button("Bind to selection…"),
                );
                let bind = if let Some(reason) = binding_block_reason.as_deref() {
                    bind.on_disabled_hover_text(reason)
                } else {
                    bind
                };
                if bind.clicked()
                    && let Some(component_id) = selected_component
                {
                    app.queue_model_binding(component_id, &library.name, &model.name);
                }
            });
        });
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.min_rect().bottom(),
        Stroke::new(1.0, t.color.border),
    );

    ScrollArea::vertical()
        .id_salt("models-selected-detail")
        .show(ui, |ui| {
            let detail_width = ui.available_width();
            if detail_width > 1100.0 {
                ui.columns(4, |columns| {
                    parameter_card(&mut columns[0], &library, &model);
                    characteristic_card(&mut columns[1], &model);
                    qualification_card(&mut columns[2], &library, &model);
                    usage_card(&mut columns[3], &model, &usages, app);
                });
            } else if detail_width > 650.0 {
                ui.columns(2, |columns| {
                    parameter_card(&mut columns[0], &library, &model);
                    characteristic_card(&mut columns[1], &model);
                });
                ui.columns(2, |columns| {
                    qualification_card(&mut columns[0], &library, &model);
                    usage_card(&mut columns[1], &model, &usages, app);
                });
            } else {
                parameter_card(ui, &library, &model);
                characteristic_card(ui, &model);
                qualification_card(ui, &library, &model);
                usage_card(ui, &model, &usages, app);
            }
        });
}

fn parameter_card(ui: &mut Ui, library: &ModelLibrary, model: &DeviceModel) {
    detail_pane(
        ui,
        "RESOLVED PARAMETERS",
        Some(&format!(
            "{} values",
            model.parameters.len() + model.string_parameters.len()
        )),
        |ui| {
            let mut values = model
                .parameters
                .iter()
                .map(|(name, value)| (name.clone(), value.to_string(), "source card"))
                .collect::<Vec<_>>();
            values.extend(
                model
                    .string_parameters
                    .iter()
                    .map(|(name, value)| (name.clone(), value.clone(), "source string")),
            );
            values.sort_by(|left, right| left.0.cmp(&right.0));
            if values.is_empty() {
                empty_state(
                    ui,
                    "No parameter card is attached to this model.",
                    "Built-in equation defaults remain owned by the engine.",
                );
            } else {
                // A BSIM4 card carries several hundred parameters and this is
                // a column in a four-pane row. What is not listed is counted;
                // a list that stops silently reads as the whole card.
                let hidden = values.len().saturating_sub(PARAMETER_ROWS);
                for (name, value, origin) in values.into_iter().take(PARAMETER_ROWS) {
                    property(ui, &name, &value, origin);
                }
                if hidden > 0 {
                    property(
                        ui,
                        "…",
                        &format!("{hidden} more"),
                        "open source for the full card",
                    );
                }
            }
            if let Some(metadata) = library.model_definition_metadata.get(&model.name) {
                ui.separator();
                property(
                    ui,
                    "Schema",
                    &format!("{} typed fields", metadata.parameters.len()),
                    "project definition",
                );
            }
        },
    );
}

/// What the card declares about the device's operating envelope.
///
/// This pane used to draw a curve: a square-law `(V − VTH0)²` sketch, plotted
/// for any card carrying a `vth0` — which every BSIM4, BSIM-CMG and PSP card
/// does, and for none of which is the square law the model. Normalised, with no
/// axis units and a hard-coded 1.8 V supply, it looked like an I-V
/// characteristic and was an unrelated equation. A plot that a reader can
/// mistake for the device's behaviour has to come from the engine evaluating
/// the actual model; until it does, this states only what the card itself
/// declares.
fn characteristic_card(ui: &mut Ui, model: &DeviceModel) {
    detail_pane(ui, "DECLARED ENVELOPE", Some("from the card"), |ui| {
        let mut declared = false;
        if let Some(vth) = model.vth0.or_else(|| model.parameters.get("vth0").copied()) {
            property(ui, "VTH0", &format!("{vth:.6} V"), "source card");
            declared = true;
        }
        if let Some(vdd) = model.vdd {
            property(ui, "Supply", &format!("{vdd:.6} V"), "source card");
            declared = true;
        }
        for (label, low, high) in [
            ("Length", model.l_min, model.l_max),
            ("Width", model.w_min, model.w_max),
        ] {
            match (low, high) {
                (Some(low), Some(high)) => {
                    property(ui, label, &format!("{low:.4e} … {high:.4e} m"), "bin range");
                    declared = true;
                }
                (Some(low), None) => {
                    property(ui, label, &format!("≥ {low:.4e} m"), "bin range");
                    declared = true;
                }
                (None, Some(high)) => {
                    property(ui, label, &format!("≤ {high:.4e} m"), "bin range");
                    declared = true;
                }
                (None, None) => {}
            }
        }
        if let (Some(low), Some(high)) = (model.spice_level, model.model_version) {
            property(
                ui,
                "Level",
                &format!("{low} · version {high}"),
                "source card",
            );
            declared = true;
        }
        if !declared {
            empty_state(
                ui,
                "This card declares no operating envelope.",
                "Bin ranges, threshold and supply are read from the card; nothing is inferred.",
            );
        }
    });
}

fn qualification_card(ui: &mut Ui, library: &ModelLibrary, model: &DeviceModel) {
    detail_pane(ui, "QUALIFICATION", Some("source-owned evidence"), |ui| {
        if let Some(state) = library.model_qualification.get(&model.name) {
            property(ui, "Suites", &state.suites.len().to_string(), "retained");
            property(
                ui,
                "Vectors",
                &state
                    .suites
                    .iter()
                    .map(|suite| suite.vectors.len())
                    .sum::<usize>()
                    .to_string(),
                "declared",
            );
            property(
                ui,
                "Evidence",
                &state.evidence.len().to_string(),
                "immutable",
            );
            property(
                ui,
                "Releases",
                &state.releases.len().to_string(),
                "promoted",
            );
            let open = state
                .vector_dispositions
                .iter()
                .filter(|disposition| disposition.is_open())
                .count();
            property(
                ui,
                "Open dispositions",
                &open.to_string(),
                if open == 0 {
                    "clean"
                } else {
                    "review required"
                },
            );
        } else {
            empty_state(
                ui,
                "This model has no qualification suite.",
                "Qualification claims remain empty until a retained suite and exact-source evidence exist.",
            );
        }
    });
}

fn usage_card(
    ui: &mut Ui,
    model: &DeviceModel,
    usages: &[String],
    app: &mut ManagerRenderContext<'_>,
) {
    detail_pane(
        ui,
        "WHERE USED",
        Some(&format!("{} consumers", usages.len())),
        |ui| {
            if usages.is_empty() {
                empty_state(
                    ui,
                    "Not bound in the active project.",
                    "Place an instance or select one and use Bind to selection.",
                );
            } else {
                for usage in usages.iter().take(USAGE_ROWS) {
                    if ui.link(usage).clicked() {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::BindingTrace {
                                model: model.name.clone(),
                                consumers: usages.to_vec(),
                            });
                    }
                }
                if usages.len() > USAGE_ROWS {
                    // The trace dialog lists all of them; only this column stops.
                    if ui
                        .link(format!("{} more…", usages.len() - USAGE_ROWS))
                        .clicked()
                    {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::BindingTrace {
                                model: model.name.clone(),
                                consumers: usages.to_vec(),
                            });
                    }
                }
            }
        },
    );
}

fn parts_catalog(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    if app.state.model_library_manager.pack_definition_count() == 0 {
        page_empty_state(
            ui,
            "No addressable parts are installed",
            "Install the versioned model-pack corpus to browse licensed models and macromodel definitions.",
        );
        return;
    }
    let facet = app.state.workbench.models_view.part_facet;
    let pack_filter = app.state.workbench.models_view.selected_pack.clone();
    let mut offset = app.state.workbench.models_view.part_catalog_offset;
    let (mut total, mut hits) = app
        .state
        .model_library_manager
        .browse_pack_models(
            &app.state.workbench.models_view.catalog_query,
            pack_filter.as_deref(),
            facet.device_filters(),
            offset,
            CATALOG_LIMIT,
        )
        .unwrap_or_else(|error| {
            receipt(app, Err(error));
            (0, Vec::new())
        });
    if total > 0 && offset >= total {
        offset = ((total - 1) / CATALOG_LIMIT) * CATALOG_LIMIT;
        app.state.workbench.models_view.part_catalog_offset = offset;
        (total, hits) = app
            .state
            .model_library_manager
            .browse_pack_models(
                &app.state.workbench.models_view.catalog_query,
                pack_filter.as_deref(),
                facet.device_filters(),
                offset,
                CATALOG_LIMIT,
            )
            .unwrap_or_else(|error| {
                receipt(app, Err(error));
                (0, Vec::new())
            });
    }
    let table_h = (ui.available_height() * 0.40).max(120.0);
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
        table_header(
            ui,
            &[
                ("PART", 0.20),
                ("DESCRIPTION", 0.22),
                ("CLASS", 0.14),
                ("KIND", 0.10),
                ("PACK", 0.19),
                ("AVAILABILITY", 0.15),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-parts-table")
            .max_height(table_h)
            .show(ui, |ui| {
                if hits.is_empty() {
                    empty_state(
                        ui,
                        "No addressable part matches the current search and class.",
                        "Private helper models declared inside macromodel bodies are intentionally excluded.",
                    );
                    if ui.button("Clear search").clicked() {
                        app.state.workbench.models_view.catalog_query.clear();
                        app.state.workbench.models_view.part_facet = RSpicePartFacet::All;
                        app.state.workbench.models_view.selected_pack = None;
                        app.state.workbench.models_view.part_catalog_offset = 0;
                        app.state.workbench.models_view.selected_part = None;
                    }
                }
                for hit in &hits {
                    let key = part_key(hit);
                    let selected = app.state.workbench.models_view.selected_part.as_deref()
                        == Some(key.as_str());
                    let availability = if hit.restricted {
                        "restricted"
                    } else if hit.source.as_ref().is_some_and(|path| path.is_file()) {
                        "on disk"
                    } else {
                        "sync required"
                    };
                    if selectable_data_row(
                        ui,
                        selected,
                        &[
                            (&hit.name, 0.20, true),
                            (&hit.pack_name, 0.22, false),
                            (&hit.device, 0.14, false),
                            (&hit.kind, 0.10, true),
                            (&hit.pack, 0.19, true),
                            (availability, 0.15, true),
                        ],
                    )
                    .clicked()
                    {
                        app.state.workbench.models_view.selected_part = Some(key);
                    }
                }
            });
        });
    parts_catalog_footer(
        ui,
        app,
        hits.len(),
        total,
        hits.iter()
            .filter(|hit| hit.restricted || !hit.redistributable)
            .count(),
    );
    selected_part_detail(ui, app, &hits);
}

fn selected_part_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hits: &[PackModelHit]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_part
        .as_deref()
        .and_then(|key| hits.iter().find(|hit| part_key(hit) == key))
        .cloned()
        .or_else(|| hits.first().cloned());
    let Some(hit) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_part = Some(part_key(&hit));
    let built_in = is_builtin_pack(app, &hit.pack);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(&hit.name).monospace().strong());
        ui.label(format!("{} · {} · {}", hit.device, hit.kind, hit.pack_name));
        if ui.button("Show pack").clicked() {
            app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::InstalledPacks;
            app.state.workbench.models_view.selected_pack = Some(hit.pack.clone());
            app.state.workbench.models_view.catalog_query.clear();
        }
        if built_in {
            ui.label(RichText::new("Built in").small());
        } else if ui
            .add_enabled(
                hit.source.as_ref().is_some_and(|path| path.is_file())
                    && hit.redistributable
                    && !hit.restricted
                    && !app.state.workbench.models_view.model_import_in_progress,
                egui::Button::new("Add to project…"),
            )
            .on_disabled_hover_text(if hit.restricted || !hit.redistributable {
                "The source is not licensed for embedding in a project."
            } else {
                "The card is not present on disk; rescan or sync the corpus."
            })
            .clicked()
        {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPart {
                pack_id: hit.pack.clone(),
                part_name: hit.name.clone(),
            });
        }
        if ui.button("Open qualification").clicked() {
            app.state.workbench.models_page = ModelsPage::Qualification;
        }
        if ui
            .add_enabled(
                hit.source.as_ref().is_some_and(|path| path.is_file()),
                egui::Button::new("Open card"),
            )
            .clicked()
            && let Some(source) = hit.source.as_ref()
        {
            match std::fs::read_to_string(source) {
                Ok(body) => {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::SourcePreview {
                            title: hit.name.clone(),
                            subtitle: format!(
                                "{}:{} · read-only corpus source",
                                source.display(),
                                hit.line
                            ),
                            source: body,
                            editable: false,
                        });
                }
                Err(error) => receipt(
                    app,
                    Err(format!("Could not open '{}': {error}", source.display())),
                ),
            }
        }
    });
    card(ui, |ui| {
        card_title(ui, "DEFINITION", Some(&hit.kind));
        property(ui, "Name", &hit.name, "catalog identity");
        property(ui, "Device class", &hit.device, "canonical");
        property(ui, "Pack", &hit.pack_name, &hit.pack);
        property(
            ui,
            "Source",
            hit.source
                .as_deref()
                .map(path_label)
                .as_deref()
                .unwrap_or("not on disk"),
            &format!("line {}", hit.line),
        );
        property(
            ui,
            "Project eligibility",
            if hit.redistributable && !hit.restricted {
                "eligible"
            } else {
                "blocked"
            },
            "license policy",
        );
    });
}

fn compare_models(
    ui: &mut Ui,
    app: &ManagerRenderContext<'_>,
    left_library: &str,
    left_model: &str,
    right_library: &str,
    right_model: &str,
) {
    let left = app
        .state
        .model_library_manager
        .get_library(left_library)
        .and_then(|library| library.get_model(left_model));
    let right = app
        .state
        .model_library_manager
        .get_library(right_library)
        .and_then(|library| library.get_model(right_model));
    let (Some(left), Some(right)) = (left, right) else {
        empty_state(
            ui,
            "One comparison definition no longer resolves.",
            "Close this comparison and select live catalog rows again.",
        );
        return;
    };
    ui.label(
        RichText::new(format!(
            "{left_library}/{left_model}  ↔  {right_library}/{right_model}"
        ))
        .monospace(),
    );
    // Numeric and string parameters both, because a comparison that reads only
    // the numeric map calls two cards identical when they differ on a string —
    // and the detail pane beside it lists strings, so the two disagreed.
    let rows = model_comparison_rows(left, right);
    let differences = rows.iter().filter(|row| row.state != "same").count();
    ui.label(
        RichText::new(if differences == 0 {
            "No parameter differs.".to_owned()
        } else {
            format!("{differences} of {} parameters differ.", rows.len())
        })
        .small()
        .color(if differences == 0 {
            Tokens::get(ui.ctx()).color.ok
        } else {
            Tokens::get(ui.ctx()).color.warn
        }),
    );
    table_header(
        ui,
        &[
            ("PARAMETER", 0.26),
            ("LEFT", 0.25),
            ("RIGHT", 0.25),
            ("KIND", 0.10),
            ("STATE", 0.14),
        ],
    );
    ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
        for row in &rows {
            selectable_data_row(
                ui,
                false,
                &[
                    (&row.key, 0.26, true),
                    (row.left.as_deref().unwrap_or("not set"), 0.25, true),
                    (row.right.as_deref().unwrap_or("not set"), 0.25, true),
                    (row.kind, 0.10, false),
                    (row.state, 0.14, true),
                ],
            );
        }
    });
}

struct ModelComparisonRow {
    key: String,
    left: Option<String>,
    right: Option<String>,
    kind: &'static str,
    state: &'static str,
}

fn model_comparison_rows(left: &DeviceModel, right: &DeviceModel) -> Vec<ModelComparisonRow> {
    let mut rows = Vec::new();
    let numeric = left
        .parameters
        .keys()
        .chain(right.parameters.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for key in numeric {
        let left_value = left.parameters.get(&key).map(ToString::to_string);
        let right_value = right.parameters.get(&key).map(ToString::to_string);
        rows.push(ModelComparisonRow {
            state: comparison_state(left_value.as_ref(), right_value.as_ref()),
            key,
            left: left_value,
            right: right_value,
            kind: "value",
        });
    }
    let strings = left
        .string_parameters
        .keys()
        .chain(right.string_parameters.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for key in strings {
        let left_value = left.string_parameters.get(&key).cloned();
        let right_value = right.string_parameters.get(&key).cloned();
        rows.push(ModelComparisonRow {
            state: comparison_state(left_value.as_ref(), right_value.as_ref()),
            key,
            left: left_value,
            right: right_value,
            kind: "string",
        });
    }
    rows.sort_by(|left, right| left.key.cmp(&right.key));
    rows
}

fn comparison_state(left: Option<&String>, right: Option<&String>) -> &'static str {
    match (left, right) {
        (None, None) => "same",
        (None, Some(_)) => "right only",
        (Some(_), None) => "left only",
        (Some(left), Some(right)) if left == right => "same",
        (Some(_), Some(_)) => "changed",
    }
}

fn open_model_compare(app: &mut ManagerRenderContext<'_>, left_library: &str, left_model: &str) {
    let right = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library
                .models
                .values()
                .map(move |model| (library.name.clone(), model.name.clone()))
        })
        .find(|(library, model)| library != left_library || model != left_model);
    if right.is_none() {
        receipt(
            app,
            Err("A comparison requires at least two loaded model definitions.".to_owned()),
        );
        return;
    }
    // Opened on a chosen counterpart, never on whichever definition the
    // catalog happens to iterate first: comparing a MOS card against an
    // unrelated BJT because it sorted first is not a comparison.
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::CompareModels {
        left_library: left_library.to_owned(),
        left_model: left_model.to_owned(),
        right: default_comparison_counterpart(app, left_library, left_model),
    });
}

/// The counterpart a comparison opens on, or `None` to make the user choose.
///
/// A model of the same name in another library is the comparison an engineer
/// almost always wants — the same card across two PDK revisions or corners.
/// Anything less specific is a guess, and the picker asks instead.
fn default_comparison_counterpart(
    app: &ManagerRenderContext<'_>,
    left_library: &str,
    left_model: &str,
) -> Option<(String, String)> {
    app.state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| !library.name.eq_ignore_ascii_case(left_library))
        .find_map(|library| {
            library
                .models
                .values()
                .find(|model| model.name.eq_ignore_ascii_case(left_model))
                .map(|model| (library.name.clone(), model.name.clone()))
        })
}

/// Choose the right-hand definition of a comparison.
fn comparison_counterpart_picker(
    ui: &mut Ui,
    app: &ManagerRenderContext<'_>,
    left_library: &str,
    left_model: &str,
    selected: &mut Option<(String, String)>,
) {
    let candidates = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            let mut models = library.models.values().collect::<Vec<_>>();
            models.sort_by(|left, right| left.name.cmp(&right.name));
            models
                .into_iter()
                .map(move |model| (library.name.clone(), model.name.clone()))
        })
        .filter(|(library, model)| {
            !(library.eq_ignore_ascii_case(left_library) && model.eq_ignore_ascii_case(left_model))
        })
        .collect::<Vec<_>>();
    ui.horizontal(|ui| {
        ui.label("Compare against");
        let label = selected.as_ref().map_or_else(
            || "Choose a definition…".to_owned(),
            |(library, model)| format!("{library} / {model}"),
        );
        egui::ComboBox::from_id_salt("models-compare-counterpart")
            .selected_text(label)
            .width(320.0)
            .show_ui(ui, |ui| {
                for (library, model) in &candidates {
                    let chosen = selected.as_ref() == Some(&(library.clone(), model.clone()));
                    if ui
                        .selectable_label(chosen, format!("{library} / {model}"))
                        .clicked()
                    {
                        *selected = Some((library.clone(), model.clone()));
                    }
                }
            });
    });
}

fn open_model_source(
    app: &mut ManagerRenderContext<'_>,
    library: &ModelLibrary,
    model: &DeviceModel,
) {
    let content = model
        .file_path
        .as_ref()
        .and_then(|path| {
            library
                .source_contents
                .iter()
                .find(|source| source.path == *path)
        })
        .or_else(|| library.source_contents.first());
    if let Some(content) = content {
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::SourcePreview {
            title: model.name.clone(),
            subtitle: format!(
                "{} · {}",
                content.path.display(),
                match library.source_authority {
                    ModelSourceAuthority::ProjectOwned { .. } => "project revision",
                    ModelSourceAuthority::RetainedImport { .. } => "retained import bytes",
                    ModelSourceAuthority::External => "pinned external bytes",
                    ModelSourceAuthority::BuiltIn => "built-in",
                }
            ),
            source: String::from_utf8_lossy(&content.bytes).into_owned(),
            editable: library.source_authority.is_project_owned(),
        });
    } else if let Some(path) = model.file_path.as_ref().or(library.root_path.as_ref()) {
        match std::fs::read_to_string(path) {
            Ok(source) => {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::SourcePreview {
                        title: model.name.clone(),
                        subtitle: format!("{} · live unpinned source", path.display()),
                        source,
                        editable: false,
                    });
            }
            Err(error) => receipt(
                app,
                Err(format!(
                    "Could not read model source '{}': {error}",
                    path.display()
                )),
            ),
        }
    }
}

fn refresh_library(app: &mut ManagerRenderContext<'_>, library: &ModelLibrary) {
    if library.root_path.is_none() {
        receipt(
            app,
            Err(format!(
                "Library '{}' has no external root to refresh.",
                library.name
            )),
        );
        return;
    }
    app.queue_library_refresh(&library.name);
}

fn attach_pack(app: &mut ManagerRenderContext<'_>, pack_id: &str) {
    app.queue_pack_attach(pack_id);
}

fn author_technology_symbol_variant(
    app: &mut ManagerRenderContext<'_>,
    package_id: &str,
    source_cell: &str,
    target_library: &str,
    target_cell: &str,
) -> Result<String, String> {
    let target_cell = target_cell.trim();
    let mut characters = target_cell.chars();
    if target_cell.is_empty()
        || target_cell.len() > 128
        || !characters
            .next()
            .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
        || characters.any(|character| character != '_' && !character.is_ascii_alphanumeric())
    {
        return Err(
            "Target cell must start with a letter or underscore and contain only ASCII letters, digits, and underscores."
                .to_owned(),
        );
    }
    let package = app
        .state
        .project_signed_technology_package()?
        .ok_or_else(|| "This project has no exact signed technology package.".to_owned())?;
    if !package
        .manifest()
        .package_id
        .eq_ignore_ascii_case(package_id)
    {
        return Err(format!(
            "The selected technology symbol belongs to '{package_id}', but the project now pins '{}'.",
            package.manifest().package_id
        ));
    }
    let source = package
        .symbol_definitions()
        .iter()
        .find(|definition| definition.identity.cell.eq_ignore_ascii_case(source_cell))
        .cloned()
        .ok_or_else(|| {
            format!("Signed technology symbol '{package_id}/{source_cell}' no longer resolves.")
        })?;
    let target = app
        .state
        .library_manager
        .get_library(target_library)
        .ok_or_else(|| format!("Target library '{target_library}' no longer exists."))?;
    if target.read_only {
        return Err(format!("Target library '{target_library}' is read-only."));
    }
    if target.get_cell(target_cell).is_some() {
        return Err(format!(
            "Target cell '{target_library}/{target_cell}' already exists. Choose a new project cell so the signed source is never overwritten implicitly."
        ));
    }

    let mut variant = source;
    variant.identity = crate::state::SymbolIdentity::new(
        target_library,
        target_cell,
        1,
        uuid::Uuid::new_v5(
            &uuid::Uuid::NAMESPACE_OID,
            format!(
                "rspice:signed-pdk-symbol-variant:{}:{}:{target_library}/{target_cell}",
                package.archive_digest(),
                variant.identity.binding_id
            )
            .as_bytes(),
        )
        .to_string(),
    );
    variant.parameter_form.revision = 1;
    variant.generated_views.symbol = true;
    variant.generated_views.parameter_form = true;
    variant.generated_views.simulation_test_fixture = false;
    variant.validate().map_err(|error| {
        format!("Signed technology symbol cannot seed a project variant: {error}")
    })?;

    let mut candidate = app.state.library_manager.clone();
    let plan = variant
        .build_plan(
            candidate
                .get_library(target_library)
                .expect("target library was resolved above"),
        )
        .map_err(|error| error.to_string())?;
    plan.commit(
        candidate
            .get_library_mut(target_library)
            .expect("target library remains present in candidate"),
    )
    .map_err(|error| error.to_string())?;
    let revision = publish_symbol_definition_candidate(
        app.state,
        candidate,
        target_library,
        target_cell,
        format!("author project variant of signed symbol {package_id}/{source_cell}"),
    )?;
    app.state.workbench.models_view.selected_symbol =
        Some(format!("{target_library}\u{1f}{target_cell}\u{1f}symbol"));
    Ok(format!(
        "Authored '{target_library}/{target_cell}' from signed technology symbol '{package_id}/{source_cell}' at project revision {}.",
        revision.get()
    ))
}

fn detach_pack(app: &mut ManagerRenderContext<'_>, pack_id: &str) {
    let libraries = attached_libraries_for_pack(app, pack_id);
    if libraries.is_empty() {
        receipt(
            app,
            Err(format!("Pack '{pack_id}' is not attached to this project.")),
        );
        return;
    }
    let mut candidate = app.state.model_library_manager.clone();
    for library in &libraries {
        candidate.remove_library(library);
    }
    let result = publish_model_library_set_candidate(
        app.state,
        candidate,
        format!("detach model pack {pack_id}"),
    )
    .and_then(|revision| {
        let revision = revision
            .ok_or_else(|| format!("Pack '{pack_id}' detach produced no project change."))?;
        Ok(format!(
            "Detached pack '{pack_id}' from {} libraries ({}) at project revision {}.",
            libraries.len(),
            libraries.join(", "),
            revision.get()
        ))
    });
    receipt(app, result);
}

fn add_part(app: &mut ManagerRenderContext<'_>, pack_id: &str, part_name: &str) {
    app.queue_part_add(pack_id, part_name);
}

fn publish_definition_provider(
    app: &mut ManagerRenderContext<'_>,
    scope: ModelConsumerScope,
    definition: &str,
    provider_library: &str,
    reason: &str,
) {
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate
        .resolve_definition_provider(scope, definition, provider_library, reason.trim())
        .and_then(|record| {
            publish_model_resolution_candidate(
                app.state,
                candidate,
                format!(
                    "resolve {} {} to {}",
                    scope.label(),
                    record.normalized_name,
                    record.provider_library
                ),
            )
            .map(|revision| {
                format!(
                    "Resolved {} '{}' to authenticated provider '{}' (source {}) at project revision {}.",
                    scope.label(),
                    record.normalized_name,
                    record.provider_library,
                    record.provider_source_digest,
                    revision.get()
                )
            })
        });
    receipt(app, result);
}

fn clear_definition_provider(
    app: &mut ManagerRenderContext<'_>,
    scope: ModelConsumerScope,
    definition: &str,
) {
    let mut candidate = app.state.model_library_manager.clone();
    if !candidate.clear_definition_provider(scope, definition) {
        receipt(
            app,
            Err(format!(
                "No provider decision exists for {} '{}'.",
                scope.label(),
                definition
            )),
        );
        return;
    }
    let result = publish_model_resolution_candidate(
        app.state,
        candidate,
        format!("clear {} provider decision {definition}", scope.label()),
    )
    .map(|revision| {
        format!(
            "Cleared the provider decision for {} '{}' at project revision {}; execution now fails closed while the name remains contested.",
            scope.label(),
            definition,
            revision.get()
        )
    });
    receipt(app, result);
}

fn attached_libraries_for_pack(app: &ManagerRenderContext<'_>, pack_id: &str) -> Vec<String> {
    let mut libraries = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| library.pack_id.as_deref() == Some(pack_id))
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    libraries.sort();
    libraries
}

fn is_builtin_pack(app: &ManagerRenderContext<'_>, pack_id: &str) -> bool {
    app.state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .any(|library| {
            library.pack_id.as_deref() == Some(pack_id)
                && library.source_authority == ModelSourceAuthority::BuiltIn
        })
}

fn export_include_manifest(app: &mut ManagerRenderContext<'_>) {
    let manifest = build_include_manifest(app);
    let result = crate::workbench::workflows::export_workflow::publish_generated_bytes(
        "model closure manifest",
        crate::workbench::workflows::export_workflow::SaveDialogConfig {
            title: "Export model closure manifest",
            default_name: "rspice-model-closure.json",
            filter_name: "JSON",
            filter_extensions: &["json"],
        },
        manifest.as_bytes(),
        "application/json;charset=utf-8",
    );
    match result {
        Ok(Some(message)) => receipt(app, Ok(message)),
        Ok(None) => {}
        Err(error) => receipt(
            app,
            Err(format!(
                "Could not export the model closure manifest: {error}"
            )),
        ),
    }
}

fn build_include_manifest(app: &ManagerRenderContext<'_>) -> String {
    let libraries = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            serde_json::json!({
                "library": library.name,
                "authority": match library.source_authority {
                    ModelSourceAuthority::BuiltIn => "built-in",
                    ModelSourceAuthority::External => "external",
                    ModelSourceAuthority::RetainedImport { .. } => "retained-import",
                    ModelSourceAuthority::ProjectOwned { .. } => "project-owned",
                },
                "root": library.root_path.as_ref().map(|path| path.to_string_lossy()),
                "selected_section": library.selected_corner,
                "sources": library.source_closure.iter().map(|source| serde_json::json!({
                    "path": source.path.to_string_lossy(),
                    "digest": source.digest.to_string(),
                })).collect::<Vec<_>>(),
                "edges": library.source_edges.iter().map(|edge| serde_json::json!({
                    "owner": edge.owner.to_string_lossy(),
                    "requested": edge.requested_path,
                    "target": edge.target.to_string_lossy(),
                })).collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&serde_json::json!({
        "schema": 1,
        "project_revision": app.state.workspace.project.revision(),
        "libraries": libraries,
    }))
    .unwrap_or_else(|error| format!("{{\"error\":\"{error}\"}}"))
}

fn model_consumers_for_provider(
    app: &ManagerRenderContext<'_>,
    library: &ModelLibrary,
    model_name: &str,
) -> Vec<String> {
    ConsumerIndex::build(app).of(library, model_name).to_vec()
}

fn effective_model_consumers(app: &ManagerRenderContext<'_>, model_name: &str) -> Vec<String> {
    app.state
        .model_library_manager
        .effective_definition_provider(ModelConsumerScope::PrimitiveModel, model_name)
        .ok()
        .flatten()
        .and_then(|provider| {
            app.state
                .model_library_manager
                .get_library(&provider.library)
                .map(|library| model_consumers_for_provider(app, library, model_name))
        })
        .unwrap_or_default()
}

/// Why the pin action is unavailable, or `None` when it can run.
///
/// Two conditions disable it and the hover must name the one that applies. It
/// named only the first for both, so a user who started an import was told the
/// external library they were looking at was built in. The permanent reason
/// wins when both hold: a built-in source never becomes pinnable, so telling
/// someone to wait would be telling them to wait forever.
const fn pin_source_block_reason(external: bool, importing: bool) -> Option<&'static str> {
    if !external {
        Some("Built-in sources do not have an external file to pin.")
    } else if importing {
        Some("Another model-source operation is still running.")
    } else {
        None
    }
}

fn exactly_one_selected_component(app: &ManagerRenderContext<'_>) -> Option<u64> {
    let selection = &app.state.schematic.selection.components;
    (selection.len() == 1).then(|| *selection.iter().next().expect("one selected component"))
}

fn path_label(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

fn part_key(hit: &PackModelHit) -> String {
    format!("{}\u{1f}{}\u{1f}{}", hit.pack, hit.name, hit.line)
}

fn receipt(app: &mut ManagerRenderContext<'_>, result: Result<String, String>) {
    apply_receipt(app.state, result);
}

fn apply_receipt(state: &mut AppState, result: Result<String, String>) {
    match &result {
        Ok(message) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(message.clone()))
        }
        Err(message) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message.clone()))
        }
    }
    state.workbench.models_view.operational_state = match &result {
        Ok(message) if message.to_ascii_lowercase().contains("recover") => {
            ModelsOperationalState::Recovered
        }
        Ok(_) => ModelsOperationalState::Ready,
        Err(message) => ModelsOperationalState::from_failure(message),
    };
    state.workbench.models_view.action_receipt = Some(result);
}

fn navigate_specialist(app: &mut ManagerRenderContext<'_>, surface: crate::workbench::SurfaceId) {
    if let Err(error) = app.state.workbench.navigate(
        crate::workbench::SurfaceRoute::surface(surface),
        crate::workbench::RouteTransitionSource::User,
    ) {
        receipt(
            app,
            Err(format!("Cannot open {}: {error}", surface.label())),
        );
    }
}

fn section_title(ui: &mut Ui, title: &str, subtitle: &str, actions: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, 0))
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 30.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 12.0;
                    ui.label(
                        RichText::new(title)
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        RichText::new(subtitle)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                },
            );
        });
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, 2))
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 32.0),
                Layout::left_to_right(Align::Center),
                actions,
            );
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.max_rect().bottom() - 0.5,
                Stroke::new(1.0, t.color.border),
            );
        });
}

fn card(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(7))
        .show(ui, |ui| {
            // Cards are structural panes, not shrink-to-fit labels.
            ui.set_min_width(ui.available_width().max(1.0));
            content(ui);
        });
}

fn detail_pane(ui: &mut Ui, title: &str, meta: Option<&str>, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::ZERO)
        .show(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 6))
                .show(ui, |ui| card_title(ui, title, meta));
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, content);
        });
}

fn card_title(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text_dim),
        );
        if let Some(meta) = meta {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(meta)
                        .small()
                        .monospace()
                        .color(t.color.text_faint),
                );
            });
        }
    });
    ui.separator();
}

fn property(ui: &mut Ui, name: &str, value: &str, origin: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 22.0), Sense::hover());
    let name_width = rect.width() * 0.30;
    let value_width = rect.width() * 0.34;
    let origin_width = (rect.width() - name_width - value_width).max(1.0);
    let inset = 3.0;

    let name = elide(ui, name, (name_width - inset * 2.0).max(1.0), false);
    let value = elide(ui, value, (value_width - inset * 2.0).max(1.0), true);
    let origin = elide(ui, origin, (origin_width - inset * 2.0).max(1.0), false);
    ui.painter().text(
        egui::pos2(rect.left() + inset, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.left() + name_width + inset, rect.center().y),
        egui::Align2::LEFT_CENTER,
        value,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.right() - inset, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        origin,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

fn empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(title).strong().color(t.color.text_dim));
        ui.label(RichText::new(detail).small().color(t.color.text_faint));
    });
    ui.add_space(10.0);
}

fn page_empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let size = egui::vec2(
        ui.available_width().max(1.0),
        ui.available_height().max(180.0),
    );
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let panel = rect.shrink2(egui::vec2(12.0, 12.0));
    ui.painter().rect(
        panel,
        3.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let accent = egui::Rect::from_center_size(
        egui::pos2(panel.center().x, panel.center().y - 34.0),
        egui::vec2(34.0, 3.0),
    );
    ui.painter().rect_filled(accent, 2.0, t.color.accent);
    ui.painter().text(
        egui::pos2(panel.center().x, panel.center().y - 12.0),
        egui::Align2::CENTER_CENTER,
        title,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(panel.center().x, panel.center().y + 14.0),
        egui::Align2::CENTER_CENTER,
        elide(ui, detail, (panel.width() - 48.0).max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

fn table_header(ui: &mut Ui, columns: &[(&str, f32)]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), HEADER_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    let mut x = rect.left() + 5.0;
    for (label, fraction) in columns {
        let width = rect.width() * fraction;
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            *label,
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_faint,
        );
        x += width;
    }
}

fn selectable_data_row(
    ui: &mut Ui,
    selected: bool,
    columns: &[(&str, f32, bool)],
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    paint_columns(ui, rect, columns);
    // The first column is the row's identifier in every caller, so it is the
    // name a screen reader should announce for the selection.
    let row_label = columns
        .first()
        .map(|(value, _, _)| (*value).to_owned())
        .unwrap_or_default();
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            row_label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn paint_columns(ui: &Ui, rect: egui::Rect, columns: &[(&str, f32, bool)]) {
    let t = Tokens::get(ui.ctx());
    let mut x = rect.left() + 5.0;
    for (value, fraction, mono) in columns {
        let width = rect.width() * fraction;
        let clipped = elide(ui, value, width - 9.0, *mono);
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            clipped,
            if *mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            t.color.text_dim,
        );
        x += width;
    }
}

/// Clip a cell's text to its column.
///
/// This module carried its own copy that dropped one character at a time and
/// laid the whole string out again after each, so a name that had to lose
/// thirty characters cost thirty text layouts — paid per cell, per row, on a
/// table the size of the corpus. The design system's owner bisects instead,
/// and cuts on grapheme boundaries rather than `char`s.
fn elide(ui: &Ui, value: &str, max_width: f32, mono: bool) -> String {
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    crate::workbench::design_system::elide_text(ui, value, &font, max_width)
}

#[cfg(test)]
mod tests;

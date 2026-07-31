//! Corner and PDK section binding workspace.
//!
//! Status on this surface is produced by `ModelLibraryManager` from the same
//! sealed source bundle used by prepared simulation runs. Editable rows are
//! durable corner contracts; view selection remains transient.

use super::*;
use crate::state::model_library::{
    CornerBindingInspection, CornerBindingInspectionRow, CornerSectionBinding, CornerSectionDomain,
    CornerSectionInspection, ProcessCorner,
};
use crate::workbench::state::VerificationPage;

const CORNER_TABLE_MIN_W: f32 = 1_180.0;
const CORNER_TABLE_H: f32 = 278.0;
const CORNER_DETAILS_BREAKPOINT: f32 = 940.0;

#[derive(Debug, Clone)]
struct CornerInspectionCache {
    report: CornerBindingInspection,
}

#[derive(Debug, Clone)]
struct CornerEditorDraft {
    library_name: String,
    original_name: Option<String>,
    name: String,
    description: String,
    nmos_corner: String,
    pmos_corner: String,
    nominal_temperature_c: String,
    supply_factor: String,
    minimum_temperature_c: String,
    maximum_temperature_c: String,
    is_default: bool,
    bindings: BTreeMap<CornerSectionDomain, String>,
    required: BTreeSet<CornerSectionDomain>,
    error: Option<String>,
}

impl CornerEditorDraft {
    fn new(library: &ModelLibrary) -> Self {
        Self {
            library_name: library.name.clone(),
            original_name: None,
            name: String::new(),
            description: String::new(),
            nmos_corner: "typical".to_owned(),
            pmos_corner: "typical".to_owned(),
            nominal_temperature_c: "27".to_owned(),
            supply_factor: "1".to_owned(),
            minimum_temperature_c: String::new(),
            maximum_temperature_c: String::new(),
            is_default: false,
            bindings: BTreeMap::new(),
            required: BTreeSet::from([CornerSectionDomain::Composite]),
            error: None,
        }
    }

    fn from_corner(library: &ModelLibrary, corner: &ProcessCorner) -> Self {
        Self {
            library_name: library.name.clone(),
            original_name: Some(corner.name.clone()),
            name: corner.name.clone(),
            description: corner.description.clone(),
            nmos_corner: corner.nmos_corner.clone(),
            pmos_corner: corner.pmos_corner.clone(),
            nominal_temperature_c: corner.temperature.to_string(),
            supply_factor: corner.vdd_factor.to_string(),
            minimum_temperature_c: corner
                .minimum_temperature_c
                .map_or_else(String::new, |value| value.to_string()),
            maximum_temperature_c: corner
                .maximum_temperature_c
                .map_or_else(String::new, |value| value.to_string()),
            is_default: corner.is_default,
            bindings: corner
                .effective_section_bindings()
                .into_iter()
                .map(|binding| (binding.domain, binding.section))
                .collect(),
            required: corner.effective_required_domains().into_iter().collect(),
            error: None,
        }
    }
}

#[derive(Debug, Clone)]
struct SelectedCorner {
    library: ModelLibrary,
    corner: ProcessCorner,
    inspection: CornerBindingInspectionRow,
}

pub(super) fn corners(ui: &mut Ui, app: &mut RSpiceApp) {
    let ctx = ui.ctx().clone();
    let mut inspection = cached_corner_inspection(ui, app);
    let source_library_count = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| library.root_path.is_some())
        .count();
    let mut import_requested = false;
    let mut add_requested = false;
    let mut validate_requested = false;

    surface_title_with_action_reserve(
        ui,
        &format!(
            "{} authenticated sources · {} resolved · {} unresolved",
            source_library_count,
            inspection.resolved_count(),
            inspection.unresolved_count()
        ),
        "Corner & section binding",
        "Compose process, statistical, and aging sections; prove every binding against the exact sealed source used for execution.",
        true,
        500.0,
        |ui| {
            if Button::new("Import section map…").show(ui).clicked() {
                import_requested = true;
            }
            ui.add_enabled_ui(source_library_count > 0, |ui| {
                if Button::new("Add corner…").show(ui).clicked() {
                    add_requested = true;
                }
            });
            if Button::new("Validate all bindings")
                .accent()
                .show(ui)
                .clicked()
            {
                validate_requested = true;
            }
        },
    );

    if import_requested {
        Command::PdkSettings.execute(app);
    }
    if add_requested {
        if let Some(library) = preferred_source_library(app) {
            set_corner_editor(&ctx, Some(CornerEditorDraft::new(&library)));
        }
    }
    if validate_requested {
        inspection = app.state.model_library_manager.inspect_corner_bindings();
        set_corner_inspection_cache(&ctx, inspection.clone());
        announce_corner_validation(app, &inspection);
    }

    corner_source_strip(ui, app, &inspection);

    let selected_key = selected_corner_key(ui, app, &inspection);
    let table_tokens = Tokens::get(ui.ctx());
    let (table_rows, targets) =
        corner_table_rows(app, &inspection, selected_key.as_deref(), &table_tokens);
    let columns = [
        ("Library", 0.11),
        ("Corner", 0.08),
        ("MOS", 0.10),
        ("BJT", 0.09),
        ("Passives", 0.10),
        ("Macro", 0.09),
        ("Statistical", 0.13),
        ("Aging", 0.09),
        ("Temp °C", 0.10),
        ("Status", 0.11),
    ];

    let remaining = ui.available_size().max(Vec2::splat(1.0));
    let (viewport, _) = ui.allocate_exact_size(remaining, Sense::hover());
    let mut body = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    body.spacing_mut().item_spacing = Vec2::ZERO;
    ScrollArea::vertical()
        .id_salt("models.corners.body")
        .auto_shrink([false, false])
        .show(&mut body, |ui| {
            ui.set_min_width(viewport.width());
            let event = data_table(
                ui,
                "models.corners.matrix",
                CORNER_TABLE_MIN_W,
                &columns,
                &table_rows,
                egui::vec2(ui.available_width(), CORNER_TABLE_H),
                "No authenticated PDK corner sections are loaded. Import a .lib or .scs source to begin.",
            );
            if let Some(event) = event
                && targets.contains_key(&event.key)
            {
                set_selected_corner_key(ui, Some(event.key));
            }
            corner_fail_closed_footer(ui, &inspection);
            ui.add_space(8.0);

            let current_key = selected_corner_key(ui, app, &inspection);
            let selected = current_key
                .as_deref()
                .and_then(|key| selected_corner(app, &inspection, key));
            match selected {
                Some(selected) => selected_corner_workspace(ui, app, &selected),
                None => corner_empty_details(ui),
            }
        });

    render_corner_editor(&ctx, app);
}

fn cached_corner_inspection(ui: &Ui, app: &RSpiceApp) -> CornerBindingInspection {
    let id = corner_inspection_id();
    let digest = app.state.model_library_manager.execution_catalog_digest();
    if let Some(cache) = ui
        .ctx()
        .data_mut(|data| data.get_temp::<CornerInspectionCache>(id))
        && cache.report.catalog_digest == digest
    {
        return cache.report;
    }
    let report = app.state.model_library_manager.inspect_corner_bindings();
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            id,
            CornerInspectionCache {
                report: report.clone(),
            },
        );
    });
    report
}

fn set_corner_inspection_cache(ctx: &egui::Context, report: CornerBindingInspection) {
    let id = corner_inspection_id();
    ctx.data_mut(|data| data.insert_temp(id, CornerInspectionCache { report }));
}

fn corner_inspection_id() -> egui::Id {
    egui::Id::new("models.corners.inspection")
}

fn selected_corner_key(
    ui: &Ui,
    app: &RSpiceApp,
    inspection: &CornerBindingInspection,
) -> Option<String> {
    let id = ui.make_persistent_id("models.corners.selected");
    let retained = ui
        .ctx()
        .data_mut(|data| data.get_temp::<String>(id))
        .filter(|key| {
            inspection
                .rows
                .iter()
                .any(|row| model_key(&row.library_name, &row.corner_name) == *key)
        });
    retained.or_else(|| {
        let reference = app.state.sim_setup.reference_pvt.process.short_name();
        inspection
            .rows
            .iter()
            .find(|row| row.corner_name.eq_ignore_ascii_case(reference))
            .or_else(|| inspection.rows.first())
            .map(|row| model_key(&row.library_name, &row.corner_name))
    })
}

fn set_selected_corner_key(ui: &Ui, key: Option<String>) {
    let id = ui.make_persistent_id("models.corners.selected");
    ui.ctx().data_mut(|data| match key {
        Some(key) => {
            data.insert_temp(id, key);
        }
        None => {
            data.remove::<String>(id);
        }
    });
}

fn corner_table_rows(
    app: &RSpiceApp,
    inspection: &CornerBindingInspection,
    selected_key: Option<&str>,
    t: &Tokens,
) -> (Vec<DataRow>, HashMap<String, (String, String)>) {
    let reference = app.state.sim_setup.reference_pvt.process.short_name();
    let mut rows = Vec::with_capacity(inspection.rows.len());
    let mut targets = HashMap::new();
    for inspected in &inspection.rows {
        let Some(library) = app
            .state
            .model_library_manager
            .get_library(&inspected.library_name)
        else {
            continue;
        };
        let Some(corner) = library.corners.get(&inspected.corner_name) else {
            continue;
        };
        let key = model_key(&inspected.library_name, &inspected.corner_name);
        targets.insert(
            key.clone(),
            (
                inspected.library_name.clone(),
                inspected.corner_name.clone(),
            ),
        );
        let status = if inspected.is_resolved() {
            if corner.name.eq_ignore_ascii_case(reference) {
                "reference"
            } else {
                "resolved"
            }
        } else {
            "unresolved"
        };
        let status_color = if inspected.is_resolved() {
            if corner.name.eq_ignore_ascii_case(reference) {
                t.color.accent
            } else {
                t.color.ok
            }
        } else {
            t.color.err
        };
        rows.push(DataRow {
            key: key.clone(),
            selected: selected_key == Some(key.as_str()),
            cells: vec![
                DataCell::mono(&library.name),
                DataCell::mono(corner.name.to_uppercase()),
                DataCell::mono(domain_binding(corner, CornerSectionDomain::Mos)),
                DataCell::mono(domain_binding(corner, CornerSectionDomain::Bjt)),
                DataCell::mono(domain_binding(corner, CornerSectionDomain::Passives)),
                DataCell::mono(domain_binding(corner, CornerSectionDomain::MacroModels)),
                DataCell::mono(statistical_binding(corner)),
                DataCell::mono(domain_binding(corner, CornerSectionDomain::Aging)),
                DataCell::mono(temperature_range_label(corner)),
                DataCell::mono_colored(status, status_color),
            ],
        });
    }
    (rows, targets)
}

fn domain_binding(corner: &ProcessCorner, domain: CornerSectionDomain) -> String {
    let bindings = corner.effective_section_bindings();
    bindings
        .iter()
        .find(|binding| binding.domain == domain)
        .or_else(|| {
            bindings
                .iter()
                .find(|binding| binding.domain == CornerSectionDomain::Composite)
        })
        .map_or_else(|| "—".to_owned(), |binding| binding.section.clone())
}

fn statistical_binding(corner: &ProcessCorner) -> String {
    let global = domain_binding(corner, CornerSectionDomain::StatisticalGlobal);
    let local = domain_binding(corner, CornerSectionDomain::StatisticalLocal);
    match (global.as_str(), local.as_str()) {
        ("—", "—") => domain_binding(corner, CornerSectionDomain::Composite),
        ("—", local) => local.to_owned(),
        (global, "—") => global.to_owned(),
        (global, local) if global == local => global.to_owned(),
        (global, local) => format!("{global} · {local}"),
    }
}

fn temperature_range_label(corner: &ProcessCorner) -> String {
    match (corner.minimum_temperature_c, corner.maximum_temperature_c) {
        (Some(minimum), Some(maximum)) => format!("{minimum}…{maximum}"),
        _ => corner.temperature.to_string(),
    }
}

fn corner_source_strip(ui: &mut Ui, app: &RSpiceApp, inspection: &CornerBindingInspection) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(11, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let status = if inspection.is_resolved() {
                    ("VALIDATED", t.color.ok)
                } else if inspection.rows.is_empty() {
                    ("NO PDK SECTIONS", t.color.text_faint)
                } else {
                    ("BLOCKED", t.color.err)
                };
                ui.label(
                    egui::RichText::new(status.0)
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(status.1),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!(
                        "catalog {} · {} pinned members · {} search paths",
                        short_digest(inspection.catalog_digest),
                        app.state
                            .model_library_manager
                            .libraries_sorted()
                            .iter()
                            .map(|library| library.source_closure.len())
                            .sum::<usize>(),
                        app.state.pdk_config.library_paths.len()
                    ))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
                let card_count = inspection
                    .rows
                    .iter()
                    .map(|row| inspected_model_card_count(&row.resolved_sections))
                    .sum::<usize>();
                if card_count > 0 {
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!("{card_count} materialized cards"))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                }
                if let Some(issue) = inspection.global_issues.first() {
                    ui.separator();
                    ui.label(
                        egui::RichText::new(issue)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.err),
                    );
                }
            });
        });
}

fn inspected_model_card_count(sections: &[CornerSectionInspection]) -> usize {
    sections
        .iter()
        .map(|section| section.model_card_count)
        .sum()
}

fn corner_fail_closed_footer(ui: &mut Ui, inspection: &CornerBindingInspection) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(11, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("FAIL CLOSED")
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(if inspection.unresolved_count() == 0 {
                            t.color.ok
                        } else {
                            t.color.err
                        }),
                );
                ui.label(
                    egui::RichText::new(
                        "A corner with any missing or unmaterializable required section cannot expand into simulation tasks. No typical fallback or alias guessing is performed.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
        });
}

fn selected_corner(
    app: &RSpiceApp,
    inspection: &CornerBindingInspection,
    key: &str,
) -> Option<SelectedCorner> {
    let inspected = inspection
        .rows
        .iter()
        .find(|row| model_key(&row.library_name, &row.corner_name) == key)?
        .clone();
    let library = app
        .state
        .model_library_manager
        .get_library(&inspected.library_name)?
        .clone();
    let corner = library.corners.get(&inspected.corner_name)?.clone();
    Some(SelectedCorner {
        library,
        corner,
        inspection: inspected,
    })
}

fn selected_corner_workspace(ui: &mut Ui, app: &mut RSpiceApp, selected: &SelectedCorner) {
    let wide = corner_details_use_columns(
        ui.available_width(),
        Tokens::get(ui.ctx()).metrics.ctl_h >= 44.0,
    );
    if wide {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let total = ui.available_width();
            let main_width = (total * 0.64).max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(main_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| selected_source_card(ui, app, selected),
            );
            ui.allocate_ui_with_layout(
                egui::vec2((total - main_width - 8.0).max(1.0), 0.0),
                Layout::top_down(Align::Min),
                |ui| selected_corner_side(ui, app, selected),
            );
        });
    } else {
        selected_source_card(ui, app, selected);
        ui.add_space(8.0);
        selected_corner_side(ui, app, selected);
    }
}

fn corner_details_use_columns(width: f32, touch: bool) -> bool {
    width >= CORNER_DETAILS_BREAKPOINT && !touch
}

fn selected_source_card(ui: &mut Ui, app: &mut RSpiceApp, selected: &SelectedCorner) {
    let t = Tokens::get(ui.ctx());
    let (source_label, section_label, mut preview) = corner_source_preview(selected);
    let mut edit_requested = false;
    let mut reference_requested = false;
    let mut copy_requested = false;
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(11))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(format!("{source_label}({section_label})"))
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    egui::RichText::new(format!("bound by {}", selected.corner.name))
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                if selected.inspection.is_resolved() {
                    ui.label(
                        egui::RichText::new("READ-ONLY · SEALED")
                            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.ok),
                    );
                }
            });
            ui.add_space(6.0);
            ui.add(
                egui::TextEdit::multiline(&mut preview)
                    .code_editor()
                    .desired_rows(12)
                    .interactive(false)
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if Button::new("Edit bindings…").show(ui).clicked() {
                    edit_requested = true;
                }
                ui.add_enabled_ui(selected.inspection.is_resolved(), |ui| {
                    if Button::new("Copy section").show(ui).clicked() {
                        copy_requested = true;
                    }
                });
                let process = corner_process(&selected.corner.name);
                let executable = selected.inspection.is_resolved() && process.is_some();
                ui.add_enabled_ui(executable, |ui| {
                    if Button::new("Set as reference").accent().show(ui).clicked() {
                        reference_requested = true;
                    }
                });
                if process.is_none() {
                    ui.label(
                        egui::RichText::new(
                            "Custom corner is retained as a PDK contract but is not yet available in the standard PVT axis.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.warn),
                    );
                }
            });
        });

    if edit_requested {
        set_corner_editor(
            ui.ctx(),
            Some(CornerEditorDraft::from_corner(
                &selected.library,
                &selected.corner,
            )),
        );
    }
    if copy_requested {
        ui.ctx().copy_text(preview);
        app.state.push_user_message(ConsoleMessage::info(
            "Copied sealed corner section preview.",
        ));
    }
    if reference_requested && let Some(process) = corner_process(&selected.corner.name) {
        match set_reference_corner(app, process, selected.corner.temperature) {
            Ok(changed) => {
                if changed {
                    app.state.push_user_message(ConsoleMessage::info(format!(
                        "Reference PVT changed to {} at {} °C.",
                        process.short_name(),
                        selected.corner.temperature
                    )));
                }
            }
            Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
                "Reference corner change blocked: {error}"
            ))),
        }
    }
}

fn selected_corner_side(ui: &mut Ui, app: &mut RSpiceApp, selected: &SelectedCorner) {
    statistical_and_aging_card(ui, app, selected);
    ui.add_space(8.0);
    environment_axes_card(ui, app);
    ui.add_space(8.0);
    validity_card(ui, app, selected);
}

fn statistical_and_aging_card(ui: &mut Ui, app: &mut RSpiceApp, selected: &SelectedCorner) {
    let variable_count = selected
        .library
        .model_definition_metadata
        .values()
        .map(|metadata| metadata.statistics.variables.len())
        .sum::<usize>();
    let correlation_count = selected
        .library
        .model_definition_metadata
        .values()
        .map(|metadata| metadata.statistics.correlation_matrices.len())
        .sum::<usize>();
    let mut open_model = false;
    let mut open_evidence = false;
    property_card(ui, "Statistical & aging sections", |ui| {
        property_row(
            ui,
            "Process (global)",
            &domain_binding(&selected.corner, CornerSectionDomain::StatisticalGlobal),
        );
        property_row(
            ui,
            "Mismatch (local)",
            &domain_binding(&selected.corner, CornerSectionDomain::StatisticalLocal),
        );
        property_row(
            ui,
            "Aging",
            &domain_binding(&selected.corner, CornerSectionDomain::Aging),
        );
        property_row(ui, "Statistical variables", &variable_count.to_string());
        property_row(ui, "Correlation groups", &correlation_count.to_string());
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_enabled_ui(variable_count > 0, |ui| {
                if Button::new("Inspect statistical model…").show(ui).clicked() {
                    open_model = true;
                }
            });
            if Button::new("Monte Carlo evidence").show(ui).clicked() {
                open_evidence = true;
            }
        });
    });
    if open_model {
        app.state
            .model_library_manager
            .select_library(&selected.library.name);
        if let Some(model) = selected
            .library
            .model_definition_metadata
            .keys()
            .min()
            .cloned()
        {
            app.state.workbench.selected_model = Some(model);
        }
        Command::ModelEditor.execute(app);
    }
    if open_evidence {
        Command::VerificationPage(VerificationPage::Yield).execute(app);
    }
}

fn environment_axes_card(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut corner_state = app.state.sim_setup.corner.clone();
    corner_state.ensure_initialized();
    let config = corner_state.to_config();
    let mut edit_requested = false;
    property_card(ui, "Environment axes · run-set owned", |ui| {
        match &config {
            Ok(config) => {
                property_row(
                    ui,
                    "Temperatures",
                    &config
                        .temperatures
                        .iter()
                        .map(|value| format!("{value} °C"))
                        .collect::<Vec<_>>()
                        .join(" · "),
                );
                property_row(
                    ui,
                    "Supplies",
                    &config
                        .voltages
                        .iter()
                        .map(|value| format!("{value} V"))
                        .collect::<Vec<_>>()
                        .join(" · "),
                );
                property_row(
                    ui,
                    "Process points",
                    &config
                        .process_corners
                        .iter()
                        .map(|process| process.short_name())
                        .collect::<Vec<_>>()
                        .join(" · "),
                );
                property_row(
                    ui,
                    "Expansion",
                    if config.full_matrix {
                        "full Cartesian matrix"
                    } else {
                        "diagonal pairing"
                    },
                );
            }
            Err(error) => {
                property_row_toned(
                    ui,
                    "Run-set contract",
                    &format!("invalid · {error}"),
                    Tokens::get(ui.ctx()).color.err,
                );
            }
        }
        ui.add_space(6.0);
        if Button::new("Edit in run plan…").show(ui).clicked() {
            edit_requested = true;
        }
    });
    if edit_requested {
        Command::OpenWorkspace(Workspace::Simulate).execute(app);
    }
}

fn validity_card(ui: &mut Ui, app: &mut RSpiceApp, selected: &SelectedCorner) {
    let mut laws = 0usize;
    let mut qualified_minimum = f64::NEG_INFINITY;
    let mut qualified_maximum = f64::INFINITY;
    for metadata in selected.library.model_definition_metadata.values() {
        for law in &metadata.temperature_laws {
            laws += 1;
            qualified_minimum = qualified_minimum.max(law.valid_range.minimum_c.get());
            qualified_maximum = qualified_maximum.min(law.valid_range.maximum_c.get());
        }
    }
    let mut run_state = app.state.sim_setup.corner.clone();
    run_state.ensure_initialized();
    let requested = run_state
        .to_config()
        .map(|config| config.temperatures)
        .unwrap_or_default();
    let outside = if laws == 0 || qualified_minimum > qualified_maximum {
        0
    } else {
        requested
            .iter()
            .filter(|temperature| {
                **temperature < qualified_minimum || **temperature > qualified_maximum
            })
            .count()
    };
    let t = Tokens::get(ui.ctx());
    property_card(ui, "Validity against model cards", |ui| {
        if laws == 0 {
            property_row_toned(
                ui,
                "Temperature qualification",
                "not declared",
                t.color.warn,
            );
            ui.label(
                egui::RichText::new(
                    "No typed model temperature law declares a qualified range. RSpice will not infer one from a corner name.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        } else if qualified_minimum <= qualified_maximum {
            property_row(
                ui,
                "Common qualified range",
                &format!("{qualified_minimum}…{qualified_maximum} °C"),
            );
            property_row_toned(
                ui,
                "Requested axis",
                if outside == 0 {
                    "inside declared range"
                } else {
                    "outside declared range"
                },
                if outside == 0 {
                    t.color.ok
                } else {
                    t.color.err
                },
            );
            property_row(ui, "Out-of-range points", &outside.to_string());
        } else {
            property_row_toned(
                ui,
                "Temperature qualification",
                "model ranges have no common intersection",
                t.color.err,
            );
        }
    });
}

fn corner_empty_details(ui: &mut Ui) {
    property_card(ui, "Bound section source", |ui| {
        ui.label(
            "Select an authenticated corner row to inspect its exact section source, environment axes, and validity.",
        );
    });
}

fn corner_source_preview(selected: &SelectedCorner) -> (String, String, String) {
    let binding = selected
        .corner
        .effective_section_bindings()
        .into_iter()
        .next();
    let section = binding
        .as_ref()
        .map_or_else(|| "unbound".to_owned(), |binding| binding.section.clone());
    let source_path = selected
        .corner
        .file_path
        .as_deref()
        .or(selected.library.root_path.as_deref());
    let source_label = source_path
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "no authenticated source".to_owned());
    let preview = source_path
        .and_then(|path| {
            selected
                .library
                .source_contents
                .iter()
                .find(|content| content.path == path)
        })
        .and_then(|content| rspice_core::netlist::decode_source_bytes(&content.bytes).ok())
        .map(|source| extract_section_preview(&source, &section))
        .unwrap_or_else(|| {
            selected
                .inspection
                .issues
                .first()
                .cloned()
                .unwrap_or_else(|| {
                    "No retained source preview is available for this binding.".to_owned()
                })
        });
    (source_label, section, preview)
}

fn extract_section_preview(source: &str, section: &str) -> String {
    let lines = source.lines().collect::<Vec<_>>();
    let mut start = None;
    let mut end = None;
    for (index, line) in lines.iter().enumerate() {
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        if start.is_none()
            && tokens.len() >= 2
            && tokens[0].eq_ignore_ascii_case(".lib")
            && tokens[1].eq_ignore_ascii_case(section)
        {
            start = Some(index);
            continue;
        }
        if start.is_some()
            && tokens
                .first()
                .is_some_and(|token| token.eq_ignore_ascii_case(".endl"))
        {
            end = Some(index);
            break;
        }
    }
    let Some(start) = start else {
        return format!(
            "Section '{section}' is not present in the retained source preview. Run Validate all bindings for executable-source diagnostics."
        );
    };
    let end = end.unwrap_or_else(|| lines.len().saturating_sub(1));
    lines[start..=end]
        .iter()
        .take(160)
        .enumerate()
        .map(|(offset, line)| format!("{:>5}  {line}", start + offset + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

fn preferred_source_library(app: &RSpiceApp) -> Option<ModelLibrary> {
    app.state
        .model_library_manager
        .current_library()
        .filter(|library| library.root_path.is_some())
        .or_else(|| {
            app.state
                .model_library_manager
                .libraries_sorted()
                .into_iter()
                .find(|library| library.root_path.is_some())
        })
        .cloned()
}

fn announce_corner_validation(app: &mut RSpiceApp, inspection: &CornerBindingInspection) {
    let message = if inspection.is_resolved() {
        ConsoleMessage::info(format!(
            "Corner binding validation passed: {} corners resolved against sealed source catalogue {}.",
            inspection.resolved_count(),
            short_digest(inspection.catalog_digest)
        ))
    } else {
        ConsoleMessage::error(format!(
            "Corner binding validation blocked: {} unresolved corners and {} source-seal errors.",
            inspection.unresolved_count(),
            inspection.global_issues.len()
        ))
    };
    app.state.push_user_message(message);
    for issue in inspection.global_issues.iter().take(8) {
        app.state
            .push_user_message(ConsoleMessage::error(issue.clone()));
    }
}

fn corner_process(name: &str) -> Option<crate::simulation::dialog::corner::ProcessCorner> {
    use crate::simulation::dialog::corner::ProcessCorner as RunCorner;
    if name.eq_ignore_ascii_case("TT") {
        Some(RunCorner::TT)
    } else if name.eq_ignore_ascii_case("SS") {
        Some(RunCorner::SS)
    } else if name.eq_ignore_ascii_case("FF") {
        Some(RunCorner::FF)
    } else if name.eq_ignore_ascii_case("SF") {
        Some(RunCorner::SF)
    } else if name.eq_ignore_ascii_case("FS") {
        Some(RunCorner::FS)
    } else {
        None
    }
}

fn set_reference_corner(
    app: &mut RSpiceApp,
    process: crate::simulation::dialog::corner::ProcessCorner,
    temperature_celsius: f64,
) -> Result<bool, String> {
    app.state
        .model_library_manager
        .reference_process_model_cards(process)?;
    crate::workbench::chrome::toolbar::commit_reference_pvt(app, process, temperature_celsius)
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    format!("{digest}").chars().take(12).collect()
}

fn corner_editor_id() -> egui::Id {
    egui::Id::new("models.corners.editor")
}

fn set_corner_editor(ctx: &egui::Context, draft: Option<CornerEditorDraft>) {
    ctx.data_mut(|data| match draft {
        Some(draft) => {
            data.insert_temp(corner_editor_id(), draft);
        }
        None => {
            data.remove::<CornerEditorDraft>(corner_editor_id());
        }
    });
}

fn render_corner_editor(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(mut draft) =
        ctx.data_mut(|data| data.get_temp::<CornerEditorDraft>(corner_editor_id()))
    else {
        return;
    };
    let mut open = true;
    let mut save_requested = false;
    let mut cancel_requested = false;
    egui::Window::new(if draft.original_name.is_some() {
        "Edit corner section contract"
    } else {
        "Add corner section contract"
    })
    .id(corner_editor_id().with("window"))
    .open(&mut open)
    .collapsible(false)
    .resizable(true)
    .default_width(680.0)
    .max_height((ctx.content_rect().height() - 48.0).max(320.0))
    .show(ctx, |ui| {
        ui.label(
            egui::RichText::new(format!("Authenticated library · {}", draft.library_name))
                .font(theme::mono(tokens::FS_0, FontWeight::Medium)),
        );
        ui.add_space(8.0);
        egui::Grid::new("models.corners.editor.identity")
            .num_columns(2)
            .spacing(egui::vec2(12.0, 7.0))
            .show(ui, |ui| {
                ui.label("Corner name");
                ui.text_edit_singleline(&mut draft.name);
                ui.end_row();
                ui.label("Description");
                ui.text_edit_singleline(&mut draft.description);
                ui.end_row();
                ui.label("NMOS speed");
                ui.text_edit_singleline(&mut draft.nmos_corner);
                ui.end_row();
                ui.label("PMOS speed");
                ui.text_edit_singleline(&mut draft.pmos_corner);
                ui.end_row();
                ui.label("Nominal temperature °C");
                ui.text_edit_singleline(&mut draft.nominal_temperature_c);
                ui.end_row();
                ui.label("Supply factor");
                ui.text_edit_singleline(&mut draft.supply_factor);
                ui.end_row();
                ui.label("Qualified minimum °C");
                ui.text_edit_singleline(&mut draft.minimum_temperature_c);
                ui.end_row();
                ui.label("Qualified maximum °C");
                ui.text_edit_singleline(&mut draft.maximum_temperature_c);
                ui.end_row();
            });
        ui.checkbox(&mut draft.is_default, "Make this library's default corner");
        ui.separator();
        ui.label(
            egui::RichText::new("SECTION COMPOSITION")
                .font(theme::mono(tokens::FS_0, FontWeight::SemiBold)),
        );
        ui.label(
            "Required domains fail closed when unbound. Multiple domains may intentionally point to the same composite section.",
        );
        ScrollArea::vertical()
            .id_salt("models.corners.editor.sections")
            .max_height(290.0)
            .show(ui, |ui| {
                egui::Grid::new("models.corners.editor.section-grid")
                    .num_columns(3)
                    .spacing(egui::vec2(12.0, 7.0))
                    .striped(true)
                    .show(ui, |ui| {
                        ui.strong("Domain");
                        ui.strong("Required");
                        ui.strong("Section name");
                        ui.end_row();
                        for domain in CornerSectionDomain::ALL {
                            ui.label(domain.label());
                            let mut required = draft.required.contains(&domain);
                            if ui.checkbox(&mut required, "").changed() {
                                if required {
                                    draft.required.insert(domain);
                                } else {
                                    draft.required.remove(&domain);
                                }
                            }
                            let section = draft.bindings.entry(domain).or_default();
                            ui.text_edit_singleline(section);
                            ui.end_row();
                        }
                    });
            });
        if let Some(error) = &draft.error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        ui.separator();
        ui.horizontal(|ui| {
            if Button::new("Save contract").accent().show(ui).clicked() {
                save_requested = true;
            }
            if Button::new("Cancel").show(ui).clicked() {
                cancel_requested = true;
            }
        });
    });

    if save_requested {
        match commit_corner_editor(app, &draft) {
            Ok(issues) => {
                if issues.is_empty() {
                    app.state.push_user_message(ConsoleMessage::info(format!(
                        "Corner '{}' saved and validated structurally.",
                        draft.name.trim()
                    )));
                } else {
                    app.state.push_user_message(ConsoleMessage::warning(format!(
                        "Corner '{}' saved as an unresolved draft: {}",
                        draft.name.trim(),
                        issues.join("; ")
                    )));
                }
                set_corner_editor(ctx, None);
                ctx.data_mut(|data| {
                    data.remove::<CornerInspectionCache>(corner_inspection_id());
                });
                return;
            }
            Err(error) => draft.error = Some(error),
        }
    }
    if cancel_requested || !open {
        set_corner_editor(ctx, None);
    } else {
        set_corner_editor(ctx, Some(draft));
    }
}

fn commit_corner_editor(
    app: &mut RSpiceApp,
    draft: &CornerEditorDraft,
) -> Result<Vec<String>, String> {
    let name = draft.name.trim();
    if name.is_empty() {
        return Err("Corner name cannot be empty.".to_owned());
    }
    let temperature = parse_finite(&draft.nominal_temperature_c, "nominal temperature")?;
    if temperature <= -273.15 {
        return Err("Nominal temperature must be above absolute zero.".to_owned());
    }
    let vdd_factor = parse_finite(&draft.supply_factor, "supply factor")?;
    if vdd_factor <= 0.0 {
        return Err("Supply factor must be greater than zero.".to_owned());
    }
    let (minimum_temperature_c, maximum_temperature_c) =
        parse_optional_range(&draft.minimum_temperature_c, &draft.maximum_temperature_c)?;

    let mut candidate = app.state.model_library_manager.clone();
    let library = candidate
        .get_library_mut(&draft.library_name)
        .ok_or_else(|| "The selected model library no longer exists.".to_owned())?;
    let source_path = library
        .root_path
        .clone()
        .ok_or_else(|| "Corner contracts require an authenticated library source.".to_owned())?;
    if library.corners.values().any(|corner| {
        corner.name.eq_ignore_ascii_case(name)
            && draft.original_name.as_deref() != Some(corner.name.as_str())
    }) {
        return Err(format!(
            "Corner '{name}' conflicts with an existing case-insensitive name."
        ));
    }

    let bindings = draft
        .bindings
        .iter()
        .filter_map(|(domain, section)| {
            let section = section.trim();
            (!section.is_empty()).then(|| CornerSectionBinding::new(*domain, section))
        })
        .collect::<Vec<_>>();
    let mut corner = ProcessCorner {
        name: name.to_owned(),
        description: draft.description.trim().to_owned(),
        nmos_corner: draft.nmos_corner.trim().to_owned(),
        pmos_corner: draft.pmos_corner.trim().to_owned(),
        temperature,
        vdd_factor,
        file_path: Some(source_path),
        is_default: draft.is_default,
        section_bindings: bindings,
        required_domains: draft.required.iter().copied().collect(),
        minimum_temperature_c,
        maximum_temperature_c,
    };
    let issues = corner.validate_contract().err().unwrap_or_default();
    let blocking_editor_errors = issues
        .iter()
        .filter(|issue| !issue.contains("section is required but not bound"))
        .cloned()
        .collect::<Vec<_>>();
    if !blocking_editor_errors.is_empty() {
        return Err(blocking_editor_errors.join("; "));
    }
    if corner.description.is_empty() {
        corner.description = format!("Process corner {name}");
    }

    if let Some(original_name) = draft.original_name.as_deref() {
        library.corners.remove(original_name);
    }
    if draft.is_default {
        for existing in library.corners.values_mut() {
            existing.is_default = false;
        }
        library.selected_corner = Some(name.to_owned());
    } else if library.selected_corner.as_deref() == draft.original_name.as_deref() {
        library.selected_corner = Some(name.to_owned());
    }
    library.corners.insert(name.to_owned(), corner);
    candidate.rebuild_active_model_projection(&draft.library_name)?;
    app.publish_model_library_candidate(candidate)?;
    Ok(issues)
}

fn parse_finite(text: &str, field: &str) -> Result<f64, String> {
    let value = text
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("{field} is not a valid number."))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(format!("{field} must be finite."))
    }
}

fn parse_optional_range(
    minimum: &str,
    maximum: &str,
) -> Result<(Option<f64>, Option<f64>), String> {
    match (minimum.trim(), maximum.trim()) {
        ("", "") => Ok((None, None)),
        ("", _) | (_, "") => Err("Qualified temperature range requires both bounds.".to_owned()),
        (minimum, maximum) => {
            let minimum = parse_finite(minimum, "qualified minimum temperature")?;
            let maximum = parse_finite(maximum, "qualified maximum temperature")?;
            if minimum > maximum {
                return Err("Qualified minimum temperature exceeds the maximum.".to_owned());
            }
            Ok((Some(minimum), Some(maximum)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_preview_is_exact_and_line_attributed() {
        let source = "* header\n.lib TT\n.model nch NMOS (LEVEL=1)\n.endl TT\n.lib FF\n.endl FF\n";
        let preview = extract_section_preview(source, "TT");
        assert!(preview.contains("    2  .lib TT"));
        assert!(preview.contains("    4  .endl TT"));
        assert!(!preview.contains(".lib FF"));
    }

    #[test]
    fn editor_range_parser_requires_a_complete_finite_interval() {
        assert_eq!(parse_optional_range("", "").unwrap(), (None, None));
        assert!(parse_optional_range("-40", "").is_err());
        assert!(parse_optional_range("125", "-40").is_err());
        assert_eq!(
            parse_optional_range("-40", "125").unwrap(),
            (Some(-40.0), Some(125.0))
        );
    }

    #[test]
    fn custom_corner_is_not_misrepresented_as_standard_pvt_process() {
        assert!(corner_process("TT").is_some());
        assert!(corner_process("hot_5v5").is_none());
    }

    #[test]
    fn corner_details_stack_for_phone_tablet_and_touch_profiles() {
        assert!(!corner_details_use_columns(390.0, true));
        assert!(!corner_details_use_columns(900.0, false));
        assert!(!corner_details_use_columns(1_200.0, true));
        assert!(corner_details_use_columns(1_200.0, false));
    }

    #[test]
    fn corner_contract_commit_is_revisioned_and_invalidates_execution() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model_library_manager
            .load_library_bytes(
                "corners.lib",
                b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n".to_vec(),
                Some("TT"),
            )
            .expect("fixture imports");
        app.state.project_lifecycle.project_open = true;
        app.state.ui.netlist.current_generation_input_digest =
            Some(crate::product::ContentDigest::from_bytes([0x42; 32]));
        let revision_before = app.state.workspace.project.revision();
        let epoch_before = app.state.design_execution_epoch;
        let library = app
            .state
            .model_library_manager
            .get_library("corners")
            .expect("library exists")
            .clone();
        let corner = library.corners.get("TT").expect("TT exists");
        let mut draft = CornerEditorDraft::from_corner(&library, corner);
        draft
            .bindings
            .insert(CornerSectionDomain::Aging, "TT".to_owned());
        draft.required.insert(CornerSectionDomain::Aging);
        draft.minimum_temperature_c = "-40".to_owned();
        draft.maximum_temperature_c = "150".to_owned();

        let issues = commit_corner_editor(&mut app, &draft).expect("contract commits");

        assert!(issues.is_empty());
        assert_eq!(
            app.state.workspace.project.revision(),
            revision_before.next().expect("revision advances")
        );
        assert_eq!(
            app.state.design_execution_epoch,
            epoch_before.wrapping_add(1)
        );
        assert!(
            app.state
                .ui
                .netlist
                .current_generation_input_digest
                .is_none()
        );
        assert!(
            app.state
                .model_library_manager
                .inspect_corner_bindings()
                .is_resolved()
        );
    }

    #[test]
    fn unresolved_custom_draft_does_not_poison_resolved_reference_corner() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model_library_manager
            .load_library_bytes(
                "corners.lib",
                b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n".to_vec(),
                Some("TT"),
            )
            .expect("fixture imports");
        let library = app
            .state
            .model_library_manager
            .get_library("corners")
            .expect("library exists")
            .clone();
        let mut draft = CornerEditorDraft::new(&library);
        draft.name = "HOT_5V5".to_owned();
        draft.required = BTreeSet::from([
            CornerSectionDomain::Mos,
            CornerSectionDomain::Bjt,
            CornerSectionDomain::Aging,
        ]);

        let issues = commit_corner_editor(&mut app, &draft).expect("draft persists");

        assert_eq!(issues.len(), 3);
        let inspection = app.state.model_library_manager.inspect_corner_bindings();
        assert_eq!(inspection.unresolved_count(), 1);
        assert!(
            app.state
                .model_library_manager
                .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
                .is_ok()
        );
    }
}

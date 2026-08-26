//! Specialist Models & PDKs page: corners.

use super::*;

use crate::state::model_library::RetainedClosure;

pub(super) fn corners_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let rows = corner_rows(app);
    let unresolved = rows.iter().filter(|row| !row.resolved()).count();
    section_title(
        ui,
        "Corners & sections",
        &format!(
            "{} bindings · {} unresolved · fail closed before run expansion",
            rows.len(),
            unresolved
        ),
        |ui| {
            if ui
                .add_enabled(
                    !app.state.workbench.models_view.model_import_in_progress,
                    egui::Button::new("Import section map"),
                )
                .on_hover_text(
                    "Import an authenticated SPICE model source whose .lib sections define the map.",
                )
                .clicked()
            {
                app.queue_model_source_import();
            }
            if ui.button("Add corner…").clicked() {
                if let Some(library) = app
                    .state
                    .model_library_manager
                    .selected_library
                    .clone()
                    .or_else(|| {
                        app.state
                            .model_library_manager
                            .libraries_sorted()
                            .first()
                            .map(|library| library.name.clone())
                    })
                {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::AddCorner {
                            library,
                            name: String::new(),
                            temperature_c: "27".to_owned(),
                            supply_factor: "1.0".to_owned(),
                        });
                } else {
                    receipt(
                        app,
                        Err("Attach a model library before adding a corner.".to_owned()),
                    );
                }
            }
            if ui.button("Validate bindings").clicked() {
                let result = validate_current_model_execution_plan(app, unresolved);
                receipt(app, result);
            }
        },
    );
    if rows.is_empty() {
        page_empty_state(
            ui,
            "No corner bindings are loaded",
            "Import a PDK section map or attach a sectioned model library to publish executable corner bindings.",
        );
        return;
    }
    temperature_validity_findings(ui, &rows);
    let table_h = (ui.available_height() * 0.34).clamp(150.0, 240.0);
    card(ui, |ui| {
        table_header(
            ui,
            &[
                ("CORNER", 0.12),
                ("MOS", 0.13),
                ("BJT", 0.11),
                ("PASSIVES", 0.13),
                ("STAT", 0.10),
                ("AGING", 0.10),
                ("TEMP", 0.13),
                ("STATUS", 0.18),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-corner-matrix")
            .max_height(table_h)
            .show(ui, |ui| {
                for row in &rows {
                    let selected = app.state.workbench.models_view.selected_corner.as_deref()
                        == Some(row.key.as_str());
                    if selectable_data_row(
                        ui,
                        selected,
                        &[
                            (&row.corner.name.to_uppercase(), 0.12, true),
                            (
                                &format!("{}/{}", row.corner.nmos_corner, row.corner.pmos_corner),
                                0.13,
                                true,
                            ),
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::Bjt),
                                0.11,
                                true,
                            ),
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::Passives),
                                0.13,
                                true,
                            ),
                            (&statistical_cell(&row.corner), 0.10, true),
                            (
                                &domain_cell(&row.corner, CornerSectionDomain::Aging),
                                0.10,
                                true,
                            ),
                            (&format!("{:.1} °C", row.corner.temperature), 0.13, true),
                            (
                                if row.active && row.resolved() {
                                    "active"
                                } else if row.active {
                                    "active · blocked"
                                } else if row.resolved() {
                                    "resolved"
                                } else {
                                    "unresolved"
                                },
                                0.18,
                                true,
                            ),
                        ],
                    )
                    .clicked()
                    {
                        inspect_corner(app, row);
                    }
                }
            });
    });
    corner_detail(ui, app, &rows);
}

/// What one corner binds one domain to, as the matrix cell states it.
///
/// The cells this replaces were not per-corner facts at all: BJT and passives
/// painted the literal word "section" for every corner whose *composite*
/// binding happened to resolve, so a PDK with independently selectable device
/// sections showed identical cells for corners that bound different sections —
/// and for corners that bound none. An unbound domain is now blank, which is
/// the answer most PDKs give and the one the run expansion acts on.
fn domain_cell(corner: &ProcessCorner, domain: CornerSectionDomain) -> String {
    corner.section_for_domain(domain).unwrap_or_default()
}

/// The statistical cell, which two domains can answer.
///
/// A PDK may publish global and local statistics as separate sections, and a
/// corner may bind either or both. Naming both is the only reading that does
/// not hide one of them behind the other.
fn statistical_cell(corner: &ProcessCorner) -> String {
    [
        CornerSectionDomain::StatisticalGlobal,
        CornerSectionDomain::StatisticalLocal,
    ]
    .into_iter()
    .filter_map(|domain| corner.section_for_domain(domain))
    .collect::<Vec<_>>()
    .join(" · ")
}

/// Findings the page lists before it stops and counts the rest.
const TEMPERATURE_FINDING_ROWS: usize = 3;

/// Corners the run set asks to run outside the range the PDK qualified them for.
///
/// A finding rather than a refusal: a run at 150 °C against a corner qualified
/// to 125 °C is a run the foundry does not vouch for, which is an engineer's
/// judgement rather than a verdict the tool can reach. The page's job is that
/// the judgement is made knowingly. It renders only when there is something to
/// say — a project whose corners cover its run set sees nothing here.
fn temperature_validity_findings(ui: &mut Ui, rows: &[CornerRow]) {
    let findings = rows
        .iter()
        .filter(|row| !row.unqualified_temperatures.is_empty())
        .collect::<Vec<_>>();
    if findings.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.warn))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.spacing_mut().item_spacing.y = 2.0;
            for row in findings.iter().take(TEMPERATURE_FINDING_ROWS) {
                let line = format!(
                    "{} is qualified {}; this run set requests {}",
                    row.corner.name.to_uppercase(),
                    row.corner.qualified_range_label(),
                    crate::state::model_library::stated_temperatures(&row.unqualified_temperatures)
                );
                super::hub::announced(ui, RichText::new(&line).small().color(t.color.warn), &line);
            }
            if findings.len() > TEMPERATURE_FINDING_ROWS {
                let more = format!(
                    "{} more corner{} are qualified outside this run set",
                    findings.len() - TEMPERATURE_FINDING_ROWS,
                    if findings.len() - TEMPERATURE_FINDING_ROWS == 1 {
                        ""
                    } else {
                        "s"
                    }
                );
                super::hub::announced(
                    ui,
                    RichText::new(&more).small().color(t.color.text_faint),
                    &more,
                );
            }
        });
}

fn validate_current_model_execution_plan(
    app: &mut ManagerRenderContext<'_>,
    unresolved: usize,
) -> Result<String, String> {
    if unresolved > 0 {
        return Err(format!(
            "Corner validation found {unresolved} bindings without an exact source section."
        ));
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err(
            "A durable model-validation receipt cannot be published while the project is read-only."
                .to_owned(),
        );
    }
    let has_project_technology = app.state.project_technology_in_effect();
    if has_project_technology {
        app.state.technology_gate_block_reason()?;
    }
    let sealed = if has_project_technology {
        app.state.seal_project_execution_model_sources()?
    } else {
        app.state.model_library_manager.seal_execution_sources()?
    };
    let plan = sealed.reference_model_execution_plan(app.state.sim_setup.reference_pvt.process)?;
    let mut findings = vec![
        crate::state::model_library::ModelValidationFinding {
            code: "SOURCE_CLOSURE_AUTHENTICATED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: "Every executable SPICE source and transitive dependency matched its accepted content digest.".to_owned(),
        },
        crate::state::model_library::ModelValidationFinding {
            code: "SPICE_NAMESPACE_COMPILED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: format!(
                "The frozen SPICE namespace compiled with {} bindings and {} explicit provider decisions.",
                plan.bindings().len(),
                plan.applied_resolutions().len()
            ),
        },
    ];
    let mut veriloga_count = 0_usize;
    if let Some((package, archive_digest, artifacts, bindings)) = sealed.pdk_veriloga_authority() {
        for binding in bindings {
            crate::simulation::veriloga::compile_signed_pdk_source_runtime(
                package,
                archive_digest,
                artifacts,
                binding,
            )?;
            veriloga_count += 1;
        }
        findings.push(crate::state::model_library::ModelValidationFinding {
            code: "VERILOGA_RUNTIME_COMPILED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: format!(
                "Compiled and validated {veriloga_count} authenticated signed-PDK Verilog-A runtime bindings."
            ),
        });
    }
    let pdk_archive_digest = sealed
        .pdk_model_identity()
        .map(|(_, archive_digest)| archive_digest);
    if pdk_archive_digest.is_some() {
        findings.push(crate::state::model_library::ModelValidationFinding {
            code: "SIGNED_PDK_TRUST_VERIFIED".to_owned(),
            severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
            message: "The exact project-pinned signed PDK archive, platform contract, and trust chain were verified.".to_owned(),
        });
    }
    let receipt = app
        .state
        .model_library_manager
        .issue_model_validation_receipt(
            app.state.workspace.project.revision(),
            plan.digest(),
            pdk_archive_digest,
            crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            findings,
        )?;
    app.state
        .model_library_manager
        .validate_model_validation_receipt(
            app.state.workspace.project.revision(),
            plan.digest(),
            pdk_archive_digest,
            crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
        )?;
    app.state.workspace.project_metadata_dirty = true;
    Ok(format!(
        "Published durable model-validation receipt {} for exact plan {} with {} authenticated bindings, {} source-qualified provider decisions, and {veriloga_count} Verilog-A runtimes.",
        receipt.receipt_digest,
        plan.digest(),
        plan.bindings().len(),
        plan.applied_resolutions().len()
    ))
}

#[derive(Clone)]
struct CornerRow {
    key: String,
    library: String,
    corner: ProcessCorner,
    /// Why a run cannot expand this corner, in the words the run itself uses.
    /// `None` means the corner resolves.
    blocker: Option<String>,
    has_statistics: bool,
    source: Option<String>,
    source_digest: Option<String>,
    active: bool,
    /// Temperatures this run set asks for that the corner's qualified range
    /// excludes. Empty is the healthy state and the usual one.
    unqualified_temperatures: Vec<f64>,
}

impl CornerRow {
    const fn resolved(&self) -> bool {
        self.blocker.is_none()
    }
}

/// Why a corner cannot be expanded into a run, or `None` if it can.
///
/// The verdict is the run's own, asked rather than restated: `RetainedClosure`
/// holds the one acceptance rule, and `io::project_execution`'s
/// `persisted_active_model_section_names` calls the same function. Restating it
/// here is what let the page and the run disagree twice — the page had no
/// counterpart for the run's project-owned escape, and it asked whether a
/// section *defined* anything where the run asks only whether the closure
/// carries it.
///
/// The corner contract is checked first because a malformed corner is a
/// finding about the corner, not about run expansion.
fn corner_blocker(library: &ModelLibrary, corner: &ProcessCorner) -> Option<String> {
    if let Err(errors) = corner.validate_contract() {
        return Some(errors.join("; "));
    }
    RetainedClosure::from(library).expansion_blocker(corner)
}

fn corner_rows(app: &ManagerRenderContext<'_>) -> Vec<CornerRow> {
    let mut rows = Vec::new();
    // Every corner on this page is one a reader is authoring, so every one is
    // compared against the run set — not only the corner that happens to be
    // active, which is the narrower question the preflight report asks.
    let requested = app.state.sim_setup.requested_temperatures_celsius();
    let libraries = app.state.model_library_manager.libraries_sorted();
    let active_library = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()
        .and_then(|selected| {
            libraries
                .iter()
                .find(|library| library.name.eq_ignore_ascii_case(selected))
        })
        .or_else(|| libraries.iter().find(|library| !library.corners.is_empty()))
        .map(|library| library.name.clone());
    for library in libraries {
        if active_library.as_deref() != Some(library.name.as_str()) {
            continue;
        }
        let has_statistics = library
            .model_definition_metadata
            .values()
            .any(|metadata| !metadata.statistics.variables.is_empty());
        for corner in library.corners.values() {
            let source_path = corner.file_path.as_deref().or(library.root_path.as_deref());
            let source = source_path.map(|path| path.display().to_string());
            let source_digest = source_path.and_then(|path| {
                library
                    .source_closure
                    .iter()
                    .find(|pin| pin.path == path)
                    .map(|pin| short_digest(&pin.digest.to_string()))
            });
            let blocker = if source.is_none() {
                Some(format!(
                    "corner '{}' is not bound to a retained source",
                    corner.name
                ))
            } else {
                corner_blocker(library, corner)
            };
            rows.push(CornerRow {
                key: format!("{}\u{1f}{}", library.name, corner.name),
                library: library.name.clone(),
                unqualified_temperatures: corner.temperatures_outside_qualified_range(&requested),
                corner: corner.clone(),
                blocker,
                has_statistics,
                source,
                source_digest,
                active: library
                    .selected_corner
                    .as_deref()
                    .is_some_and(|active| active.eq_ignore_ascii_case(&corner.name)),
            });
        }
    }
    rows.sort_by(|left, right| {
        left.corner
            .name
            .to_ascii_lowercase()
            .cmp(&right.corner.name.to_ascii_lowercase())
            .then_with(|| left.library.cmp(&right.library))
    });
    rows
}

fn corner_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, rows: &[CornerRow]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_corner
        .as_deref()
        .and_then(|key| rows.iter().find(|row| row.key == key))
        .cloned()
        .or_else(|| rows.first().cloned());
    let Some(row) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_corner = Some(row.key.clone());
    let mut open_editor = false;
    let mut duplicate = false;
    let mut make_default = false;
    let mut activate = false;
    let mut delete = false;
    let mut unbind = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        ui.label(
            RichText::new(format!(
                "{} / {}",
                row.library,
                row.corner.name.to_uppercase()
            ))
            .monospace()
            .strong(),
        );
        if let Some(blocker) = row.blocker.as_deref() {
            ui.label(
                RichText::new(format!("run expansion blocked · {blocker}"))
                    .small()
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        if row.active {
            ui.label(
                RichText::new("ACTIVE FOR EXECUTION")
                    .small()
                    .strong()
                    .color(Tokens::get(ui.ctx()).color.accent),
            );
        }
        if ui
            .add_enabled(
                !row.active && row.resolved(),
                egui::Button::new("Use for execution"),
            )
            .on_disabled_hover_text(if row.active {
                "This corner is already active for this library's executable model projection."
            } else {
                "Resolve every required source-section binding before activating this corner."
            })
            .clicked()
        {
            activate = true;
        }
        if ui.button("Edit corner…").clicked() {
            open_editor = true;
        }
        if ui.button("Duplicate…").clicked() {
            duplicate = true;
        }
        if ui
            .add_enabled(!row.corner.is_default, egui::Button::new("Set default"))
            .clicked()
        {
            make_default = true;
        }
        if ui.button("Delete corner…").clicked() {
            delete = true;
        }
        if ui.button("Bind section…").clicked() {
            open_corner_binding_dialog(app, &row);
        }
        for binding in row.corner.effective_section_bindings() {
            if ui
                .button(format!("Unbind {}", binding.domain.label()))
                .clicked()
            {
                unbind = Some(binding.domain);
            }
        }
        // The corner's own retained file, not whichever model the library
        // happens to iterate first.
        if ui
            .add_enabled(row.source.is_some(), egui::Button::new("Open source"))
            .clicked()
        {
            open_corner_source(app, &row);
        }
        if ui.button("View include graph").clicked() {
            app.state.workbench.models_page = ModelsPage::Include;
        }
        if ui.button("Model editor…").clicked() {
            app.queue_command(Command::ModelEditor);
        }
    });
    if activate {
        activate_corner(app, &row.library, &row.corner.name);
    } else if open_editor || duplicate {
        open_corner_editor(app, &row, duplicate);
    } else if make_default {
        corner_ops::set_default_corner(app, &row.library, &row.corner.name);
    } else if delete {
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmDeleteCorner {
            library: row.library.clone(),
            corner: row.corner.name.clone(),
        });
    } else if let Some(domain) = unbind {
        corner_ops::unbind_corner_section(app, &row.library, &row.corner.name, domain);
    }
    detail_pane(
        ui,
        "CORNER BINDING DETAILS",
        Some("section, environment, statistics, and aging"),
        |ui| {
            property(
                ui,
                "Description",
                &row.corner.description,
                "project metadata",
            );
            property(
                ui,
                "Default",
                if row.corner.is_default { "yes" } else { "no" },
                "new-plan fallback",
            );
            property(
                ui,
                "Execution active",
                if row.active { "yes" } else { "no" },
                "executable model projection",
            );
            property(ui, "NMOS", &row.corner.nmos_corner, "exact section axis");
            property(ui, "PMOS", &row.corner.pmos_corner, "exact section axis");
            property(
                ui,
                "Source",
                row.source.as_deref().unwrap_or("not bound"),
                if row.resolved() {
                    "retained"
                } else {
                    "unresolved"
                },
            );
            property(
                ui,
                "Source digest",
                row.source_digest.as_deref().unwrap_or("not pinned"),
                "authenticated content identity",
            );
            property(
                ui,
                "Supply factor",
                &format!("{:.6}", row.corner.vdd_factor),
                "environment axis",
            );
            property(
                ui,
                "Temperature",
                &format!("{:.3} °C", row.corner.temperature),
                "environment axis",
            );
            property(
                ui,
                "Qualified range",
                &row.corner.qualified_range_label(),
                &if row.unqualified_temperatures.is_empty() {
                    "temperature validity".to_owned()
                } else {
                    format!(
                        "excludes {}, which this run set requests",
                        crate::state::model_library::stated_temperatures(
                            &row.unqualified_temperatures
                        )
                    )
                },
            );
            property(
                ui,
                "Required domains",
                &row.corner
                    .effective_required_domains()
                    .into_iter()
                    .map(CornerSectionDomain::label)
                    .collect::<Vec<_>>()
                    .join(", "),
                "execution contract",
            );
            for binding in row.corner.effective_section_bindings() {
                property(
                    ui,
                    binding.domain.label(),
                    &binding.section,
                    "authenticated section",
                );
            }
            ui.separator();
            property(
                ui,
                "Statistical variables",
                if row.has_statistics {
                    "declared"
                } else {
                    "none"
                },
                "model schema",
            );
            // No "Aging evidence" row: the one this replaces read
            // `model_qualification.evidence` — reviewer evidence for whatever a
            // suite qualified — and printed it under an aging label. Nothing in
            // this project owns aging data yet, so the aging column above
            // states the binding fact and this pane states nothing at all.
            property(
                ui,
                "Binding policy",
                if row.resolved() {
                    "executable"
                } else {
                    "fail closed"
                },
                "run expansion",
            );
            if let Some(receipt) = app.state.model_library_manager.model_validation_receipt() {
                let current_revision = app.state.workspace.project.revision();
                let receipt_state = if receipt.project_revision == current_revision {
                    "current revision"
                } else {
                    "stale revision"
                };
                property(
                    ui,
                    "Validation receipt",
                    &format!(
                        "{} ({receipt_state})",
                        short_digest(&receipt.receipt_digest.to_string())
                    ),
                    &format!(
                        "project revision {} · plan {} · {} authenticated sources · {}",
                        receipt.project_revision.get(),
                        short_digest(&receipt.model_execution_plan_digest.to_string()),
                        receipt.source_count,
                        receipt.platform
                    ),
                );
            }
        },
    );
}

fn open_corner_binding_dialog(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    let section = app
        .state
        .model_library_manager
        .get_library(&row.library)
        .and_then(|library| library.section_index().into_iter().next())
        .unwrap_or_default();
    let bindings = row.corner.effective_section_bindings();
    let domain = row
        .corner
        .effective_required_domains()
        .into_iter()
        .find(|required| !bindings.iter().any(|binding| binding.domain == *required))
        .or_else(|| bindings.first().map(|binding| binding.domain))
        .unwrap_or(CornerSectionDomain::Composite);
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::BindCornerSection {
        library: row.library.clone(),
        corner: row.corner.name.clone(),
        domain,
        section,
    });
}

fn open_corner_editor(app: &mut ManagerRenderContext<'_>, row: &CornerRow, duplicate: bool) {
    let name = if duplicate {
        let base = format!("{}_copy", row.corner.name);
        let mut candidate = base.clone();
        let mut suffix = 2_u32;
        if let Some(library) = app.state.model_library_manager.get_library(&row.library) {
            while library
                .corners
                .keys()
                .any(|existing| existing.eq_ignore_ascii_case(&candidate))
            {
                candidate = format!("{base}_{suffix}");
                suffix += 1;
            }
        }
        candidate
    } else {
        row.corner.name.clone()
    };
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::EditCorner {
        library: row.library.clone(),
        original_name: row.corner.name.clone(),
        duplicate,
        name,
        description: row.corner.description.clone(),
        nmos_corner: row.corner.nmos_corner.clone(),
        pmos_corner: row.corner.pmos_corner.clone(),
        temperature_c: row.corner.temperature.to_string(),
        supply_factor: row.corner.vdd_factor.to_string(),
        minimum_temperature_c: row
            .corner
            .minimum_temperature_c
            .map_or_else(String::new, |value| value.to_string()),
        maximum_temperature_c: row
            .corner
            .maximum_temperature_c
            .map_or_else(String::new, |value| value.to_string()),
        required_domains: row.corner.effective_required_domains(),
        make_default: !duplicate && row.corner.is_default,
    });
}

/// Show the retained bytes of the file this corner is bound to.
fn open_corner_source(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(&row.library)
        .cloned()
    else {
        receipt(
            app,
            Err(format!("Library '{}' no longer exists.", row.library)),
        );
        return;
    };
    let path = row
        .corner
        .file_path
        .as_deref()
        .or(library.root_path.as_deref());
    let Some(path) = path else {
        receipt(
            app,
            Err(format!(
                "Corner '{}' is not bound to a retained source.",
                row.corner.name
            )),
        );
        return;
    };
    let retained = library
        .source_contents
        .iter()
        .find(|content| content.path == path);
    match retained {
        Some(content) => {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::SourcePreview {
                title: format!("{} / {}", library.name, row.corner.name.to_uppercase()),
                subtitle: format!("{} · retained closure member", content.path.display()),
                source: String::from_utf8_lossy(&content.bytes).into_owned(),
                editable: false,
            });
        }
        None => match std::fs::read_to_string(path) {
            Ok(source) => {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::SourcePreview {
                        title: format!("{} / {}", library.name, row.corner.name.to_uppercase()),
                        subtitle: format!("{} · live unpinned source", path.display()),
                        source,
                        editable: false,
                    });
            }
            Err(error) => receipt(
                app,
                Err(format!(
                    "Could not read corner source '{}': {error}",
                    path.display()
                )),
            ),
        },
    }
}

fn inspect_corner(app: &mut ManagerRenderContext<'_>, row: &CornerRow) {
    app.state.workbench.models_view.selected_corner = Some(row.key.clone());
}

fn activate_corner(app: &mut ManagerRenderContext<'_>, library_name: &str, corner_name: &str) {
    app.state.select_model_library(library_name);
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate
        .get_library_mut(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))
        .and_then(|library| {
            library
                .activate_corner(corner_name)
                .then_some(())
                .ok_or_else(|| {
                    format!(
                        "Corner '{corner_name}' no longer exists in library '{library_name}'."
                    )
                })
        })
        .and_then(|()| {
            publish_model_library_candidate(
                app.state,
                candidate,
                library_name,
                format!("activate model corner {corner_name}"),
            )
        })
        .map(|revision| {
            format!(
                "Activated exact corner '{corner_name}' for '{library_name}' at project revision {}.",
                revision.get()
            )
        });
    receipt(app, result);
}

#[cfg(test)]
mod tests;

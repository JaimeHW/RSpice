//! Mockup-owned rendering for the custom sheet-size workflows.

use std::collections::BTreeMap;

use egui::{ComboBox, Grid, RichText, ScrollArea, TextEdit, Ui, vec2};

use crate::state::{AuthoredDrawingSheetSize, DrawingSheetPreset, DrawingSheetPresetScope};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::model::{
    ImportCandidate, ImportResolution, PresetEditorDraft, PresetEditorMode, PresetEditorUnit,
    PresetPackageFormat, PresetTransferState, StartingFrame, TransferMode, format_dimension_um,
    unavailable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PresetKey {
    pub(super) id: String,
    pub(super) scope: DrawingSheetPresetScope,
}

impl From<&DrawingSheetPreset> for PresetKey {
    fn from(value: &DrawingSheetPreset) -> Self {
        Self {
            id: value.id.clone(),
            scope: value.scope,
        }
    }
}

pub(super) fn usage_key(scope: DrawingSheetPresetScope, id: &str) -> String {
    format!("{}:{}", scope_label(scope), id.to_lowercase())
}

#[derive(Debug, Clone)]
pub(super) enum LibraryBodyAction {
    New,
    Import,
    Export,
    Use(PresetKey),
    Duplicate(PresetKey),
    Rename(PresetKey),
    Delete(PresetKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EditorBodyAction {
    RefreshPreview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TransferBodyAction {
    ChooseImportFile,
    SelectAllExport(bool),
}

pub(super) fn library_body(
    ui: &mut Ui,
    search: &mut String,
    presets: &[DrawingSheetPreset],
    usage: &BTreeMap<String, usize>,
    project_writable: bool,
    personal_fresh: bool,
    authority_notice: Option<&str>,
) -> Option<LibraryBodyAction> {
    let t = Tokens::get(ui.ctx());
    let mut action = None;

    concept_banner(
        ui,
        "A sheet never depends on a preset the next reader will not have.",
        "Project presets travel with the project. A personal preset is copied into the project the first time a sheet uses it. Dimensions are stored on every using sheet as well as in the preset, so a missing library never changes page geometry.",
    );
    ui.add_space(10.0);
    ui.horizontal_wrapped(|ui| {
        if Button::new("New custom size\u{2026}")
            .accent()
            .show(ui)
            .clicked()
        {
            action = Some(LibraryBodyAction::New);
        }
        if Button::new("Import\u{2026}").show(ui).clicked() {
            action = Some(LibraryBodyAction::Import);
        }
        let exportable = presets.iter().any(|preset| !unavailable(preset));
        if Button::new("Export\u{2026}")
            .enabled(exportable)
            .show(ui)
            .clicked()
        {
            action = Some(LibraryBodyAction::Export);
        }
        ui.add_space(8.0);
        let available = ui.available_width().clamp(180.0, 280.0);
        ui.add_sized(
            vec2(available, t.metrics.ctl_h),
            TextEdit::singleline(search).hint_text("Filter custom sizes\u{2026}"),
        );
    });
    if let Some(notice) = authority_notice {
        ui.add_space(8.0);
        warning_notice(ui, notice);
    }
    ui.add_space(10.0);
    let query = search.trim().to_lowercase();
    let filtered = presets
        .iter()
        .filter(|preset| {
            query.is_empty()
                || preset.name.to_lowercase().contains(&query)
                || scope_label(preset.scope).contains(&query)
                || preset.id.to_lowercase().contains(&query)
        })
        .collect::<Vec<_>>();

    if filtered.is_empty() {
        empty_state(
            ui,
            if presets.is_empty() {
                "No custom sheet sizes"
            } else {
                "No custom sizes match this filter"
            },
            if presets.is_empty() {
                "Create a project or personal preset, or import a reviewed schema-1 package."
            } else {
                "Change the filter to return to the library."
            },
        );
    } else {
        ScrollArea::both()
            .id_salt("custom-sheet-size-library")
            .max_height(425.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Grid::new("custom-sheet-size-library-grid")
                    .num_columns(6)
                    .striped(true)
                    .spacing([14.0, 8.0])
                    .min_col_width(70.0)
                    .show(ui, |ui| {
                        table_header(ui, "Preset");
                        table_header(ui, "Portrait dimensions");
                        table_header(ui, "Scope");
                        table_header(ui, "Used by");
                        table_header(ui, "State");
                        table_header(ui, "Actions");
                        ui.end_row();

                        for preset in filtered {
                            let key = PresetKey::from(preset);
                            let unavailable = unavailable(preset);
                            let used_by = usage
                                .get(&usage_key(preset.scope, &preset.id))
                                .copied()
                                .unwrap_or(0);
                            ui.label(
                                RichText::new(&preset.name)
                                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                    .color(t.color.text),
                            );
                            ui.label(portrait_dimensions(preset));
                            scope_badge(ui, preset.scope);
                            ui.label(if used_by == 0 {
                                "unused".to_owned()
                            } else {
                                format!(
                                    "{used_by} reference{}",
                                    if used_by == 1 { "" } else { "s" }
                                )
                            });
                            state_badge(ui, unavailable);

                            ui.horizontal(|ui| {
                                if Button::new("Use\u{2026}")
                                    .enabled(!unavailable)
                                    .show(ui)
                                    .clicked()
                                {
                                    action = Some(LibraryBodyAction::Use(key.clone()));
                                }
                                if Button::new("Duplicate")
                                    .enabled(!unavailable && project_writable)
                                    .show(ui)
                                    .clicked()
                                {
                                    action = Some(LibraryBodyAction::Duplicate(key.clone()));
                                }
                                let owner_writable = match preset.scope {
                                    DrawingSheetPresetScope::Project => project_writable,
                                    DrawingSheetPresetScope::User => personal_fresh,
                                    DrawingSheetPresetScope::Organization => false,
                                };
                                if Button::new("Rename")
                                    .enabled(owner_writable)
                                    .show(ui)
                                    .clicked()
                                {
                                    action = Some(LibraryBodyAction::Rename(key.clone()));
                                }
                                let delete = Button::new("Delete")
                                    .enabled(owner_writable && used_by == 0)
                                    .show(ui);
                                let delete = if used_by > 0 {
                                    delete.on_disabled_hover_text(
                                        "In use by a sheet or drawing-sheet default. Change every \
                                     reference first \u{2014} deleting a depended-on preset \
                                     would leave that reference unresolvable.",
                                    )
                                } else {
                                    delete
                                };
                                if delete.clicked() {
                                    action = Some(LibraryBodyAction::Delete(key.clone()));
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
    }
    ui.add_space(10.0);
    ui.columns(3, |columns| {
        workflow_note(
            &mut columns[0],
            "Renaming",
            "A preset name is only a label. Sheets reference its stable identity, so renaming changes no geometry.",
        );
        workflow_note(
            &mut columns[1],
            "Deleting",
            "Only a preset with no sheet or drawing-sheet default references can be deleted. Former using sheets retain their exact dimensions.",
        );
        workflow_note(
            &mut columns[2],
            "Migration",
            "A project with a missing preset library recovers each sheet from its embedded dimensions and reports the dependency as unavailable.",
        );
    });
    action
}

pub(super) fn editor_body(
    ui: &mut Ui,
    draft: &mut PresetEditorDraft,
    validation_error: Option<&str>,
    project_error: Option<&str>,
    personal_stale: bool,
) -> Option<EditorBodyAction> {
    let mut changed = false;
    let editing_name_only = draft.mode == PresetEditorMode::Edit;

    let t = Tokens::get(ui.ctx());
    let available = ui.available_width();
    let gap = 12.0;
    let form_width = ((available - gap) * 0.58).max(360.0);
    let preview_width = (available - gap - form_width).max(280.0);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(
            vec2(form_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                egui::Frame::NONE
                    .fill(t.color.bg_panel)
                    .stroke(egui::Stroke::new(1.0, t.color.border))
                    .corner_radius(t.radius)
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |form| {
        section_heading(
            form,
            if editing_name_only {
                "Preset identity"
            } else {
                "Identity and scope"
            },
        );
        field_label(form, "Name");
        changed |= form
            .add(
                TextEdit::singleline(&mut draft.name)
                    .char_limit(crate::state::MAX_DRAWING_SHEET_PRESET_NAME_CHARS)
                    .desired_width(f32::INFINITY),
            )
            .changed();

        form.add_space(10.0);
        field_label(form, "Scope");
        form.add_enabled_ui(!editing_name_only, |ui| {
            ComboBox::from_id_salt("custom-sheet-editor-scope")
                .width(ui.available_width())
                .selected_text(scope_label(draft.scope))
                .show_ui(ui, |ui| {
                    changed |= ui
                        .selectable_value(
                            &mut draft.scope,
                            DrawingSheetPresetScope::Project,
                            "Project",
                        )
                        .changed();
                    changed |= ui
                        .selectable_value(
                            &mut draft.scope,
                            DrawingSheetPresetScope::User,
                            "Personal",
                        )
                        .changed();
                });
        });
        form.weak(match draft.scope {
            DrawingSheetPresetScope::Project => {
                "Saved with this project and available to collaborators."
            }
            DrawingSheetPresetScope::User => {
                "Saved in personal Preferences. A project captures an exact copy before use."
            }
            DrawingSheetPresetScope::Organization => {
                "Organization definitions are managed and read-only."
            }
        });

        if !editing_name_only {
            form.add_space(14.0);
            section_heading(form, "Physical size");
            form.columns(2, |columns| {
                field_label(&mut columns[0], "Portrait width");
                changed |= columns[0]
                        .add(
                            TextEdit::singleline(&mut draft.width)
                                .desired_width(f32::INFINITY),
                        )
                        .changed();
                field_label(&mut columns[1], "Portrait height");
                changed |= columns[1]
                        .add(
                            TextEdit::singleline(&mut draft.height)
                                .desired_width(f32::INFINITY),
                        )
                        .changed();
            });
            field_label(form, "Units");
            ComboBox::from_id_salt("custom-sheet-editor-unit")
                .width(form.available_width())
                .selected_text(unit_label(draft.unit))
                .show_ui(form, |ui| {
                    for unit in [
                        PresetEditorUnit::Millimetres,
                        PresetEditorUnit::Centimetres,
                        PresetEditorUnit::Inches,
                    ] {
                        changed |= ui
                            .selectable_value(
                                &mut draft.unit,
                                unit,
                                unit_label(unit),
                            )
                            .changed();
                    }
                });
            form.weak(format!(
                "50.8 to 2540 mm per edge \u{00b7} maximum {}:1 \u{00b7} stored to 1 \u{00b5}m; {} is only the editor unit.",
                crate::state::DRAWING_SHEET_MAX_ASPECT_RATIO,
                draft.unit.suffix(),
            ));

            form.add_space(14.0);
            section_heading(form, "Starting frame");
            field_label(form, "Margins and frame");
            ComboBox::from_id_salt("custom-sheet-editor-frame")
                .width(form.available_width())
                .selected_text(frame_label(draft.frame))
                .show_ui(form, |ui| {
                    for frame in [
                        StartingFrame::IsoA,
                        StartingFrame::AnsiA,
                        StartingFrame::Plain,
                        StartingFrame::None,
                    ] {
                        changed |= ui
                            .selectable_value(
                                &mut draft.frame,
                                frame,
                                frame_label(frame),
                            )
                            .changed();
                    }
                });
        } else {
            form.add_space(14.0);
            key_value(form, "Exact portrait size", &portrait_dimensions_from_format(
                &draft.last_valid_preview,
            ));
            key_value(form, "Stable identity", draft.source_id.as_deref().unwrap_or("\u{2014}"));
            form.weak(
                "Rename preserves the stable identity, exact physical dimensions, border, zones, title block, and every authored snapshot.",
            );
        }

        form.add_space(12.0);
        form.allocate_ui_with_layout(
            vec2(form.available_width(), 96.0),
            egui::Layout::top_down(egui::Align::Min),
            |form| {
                if let Some(error) = validation_error {
                    error_notice(form, error);
                } else if draft.unavailable {
                    warning_notice(
                        form,
                        "This definition retains an unavailable managed dependency and cannot be edited or used.",
                    );
                } else if draft.scope == DrawingSheetPresetScope::Project {
                    if let Some(error) = project_error {
                        warning_notice(form, error);
                    }
                } else if draft.scope == DrawingSheetPresetScope::User && personal_stale {
                    warning_notice(
                        form,
                        "Personal drawing-sheet preferences changed. Close and reopen this library.",
                    );
                }
                if let Some(error) = draft.error.as_deref() {
                    form.add_space(8.0);
                    error_notice(form, error);
                }
            },
        );
                    });
            },
        );
        ui.allocate_ui_with_layout(
            vec2(preview_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                egui::Frame::NONE
                    .fill(t.color.bg_panel)
                    .stroke(egui::Stroke::new(1.0, t.color.border))
                    .corner_radius(t.radius)
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |preview| {
                        let preview_caption = custom_preview_caption(draft);
                        super::super::drawing_sheet_preview::drawing_sheet_preview(
                            preview,
                            &draft.last_valid_preview,
                            230.0,
                            &preview_caption,
                        );
                        preview.add_space(8.0);
                        section_heading(preview, "Preset contract");
                        key_value(preview, "Current sheet", "unchanged");
                        key_value(preview, "Identity", "stable preset ID");
                        key_value(
                            preview,
                            "Portability",
                            "dimensions copied onto every using sheet",
                        );
                        key_value(
                            preview,
                            "Undo",
                            "preset creation remains reversible until project save",
                        );
                    });
            },
        );
    });

    changed.then_some(EditorBodyAction::RefreshPreview)
}

pub(super) fn transfer_body(
    ui: &mut Ui,
    transfer: &mut PresetTransferState,
    visible: &[DrawingSheetPreset],
) -> Option<TransferBodyAction> {
    match transfer.mode {
        TransferMode::Import => import_body(ui, transfer),
        TransferMode::Export => export_body(ui, transfer, visible),
    }
}

fn import_body(ui: &mut Ui, transfer: &mut PresetTransferState) -> Option<TransferBodyAction> {
    let mut action = None;
    let ready = transfer
        .import_candidates
        .iter()
        .filter(|candidate| candidate.selected)
        .count();
    let conflicts = transfer
        .import_candidates
        .iter()
        .filter(|candidate| candidate.selected && candidate.existing_id.is_some())
        .count();
    let banner_body = if transfer.import_candidates.is_empty() {
        "RSpice validates schema, digest, identities, exact dimensions, and template dependencies before any project-owned candidate can be selected."
            .to_owned()
    } else {
        format!(
            "{ready} preset{} selected; {conflicts} require identity resolution. Import creates project-owned candidates and never replaces a preset silently.",
            if ready == 1 { " is" } else { "s are" }
        )
    };
    concept_banner(
        ui,
        if transfer.import_candidates.is_empty() {
            "Choose a preset package for reviewed import."
        } else {
            "The source package remains untouched."
        },
        &banner_body,
    );
    ui.add_space(10.0);
    field_label(ui, "Preset package");
    ui.horizontal(|ui| {
        if Button::new("Choose package\u{2026}").show(ui).clicked() {
            action = Some(TransferBodyAction::ChooseImportFile);
        }
        ui.label(if transfer.package_name.is_empty() {
            "No file selected"
        } else {
            transfer.package_name.as_str()
        });
    });
    if let Some(digest) = transfer.reviewed_digest.as_deref() {
        ui.label(
            RichText::new(format!("Reviewed \u{00b7} {digest}"))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(Tokens::get(ui.ctx()).color.ok),
        );
    }
    ui.add_space(12.0);
    section_heading(ui, "Import review");
    if transfer.import_candidates.is_empty() {
        empty_state(
            ui,
            "No reviewed definitions",
            "Review a package before choosing identity and dependency resolutions.",
        );
    } else {
        ScrollArea::vertical()
            .id_salt("sheet-preset-import-candidates")
            .max_height(260.0)
            .show(ui, |ui| {
                Grid::new("sheet-preset-import-grid")
                    .num_columns(5)
                    .striped(true)
                    .spacing([14.0, 8.0])
                    .show(ui, |ui| {
                        table_header(ui, "Import");
                        table_header(ui, "Candidate");
                        table_header(ui, "Dimensions");
                        table_header(ui, "Dependencies");
                        table_header(ui, "Resolution");
                        ui.end_row();
                        for candidate in &mut transfer.import_candidates {
                            import_candidate_row(ui, candidate);
                        }
                    });
            });
    }
    if let Some(error) = transfer.error.as_deref() {
        ui.add_space(8.0);
        error_notice(ui, error);
    }
    ui.add_space(10.0);
    ui.columns(3, |columns| {
        workflow_note(
            &mut columns[0],
            "Units",
            "Every candidate shows normalized dimensions; exact micrometre values survive conversion.",
        );
        workflow_note(
            &mut columns[1],
            "Missing templates",
            "Managed dependencies require an explicit replacement or a retained unavailable dependency.",
        );
        workflow_note(
            &mut columns[2],
            "Receipt",
            "Import records the source digest, schema, selected candidates, mappings, conflicts, and skips.",
        );
    });
    action
}

fn import_candidate_row(ui: &mut Ui, candidate: &mut ImportCandidate) {
    ui.checkbox(&mut candidate.selected, "");
    ui.label(&candidate.portable.name);
    ui.label(format!(
        "{} \u{00d7} {} mm",
        format_mm(candidate.portable.portrait_width_um),
        format_mm(candidate.portable.portrait_height_um)
    ));
    if candidate.missing_managed_dependency {
        ui.colored_label(
            Tokens::get(ui.ctx()).color.warn,
            "organization block \u{00b7} unavailable",
        );
    } else {
        ui.label(match candidate.portable.format.border {
            crate::state::DrawingSheetBorderTemplate::Standard => "standard border",
            crate::state::DrawingSheetBorderTemplate::Plain => "plain border",
            crate::state::DrawingSheetBorderTemplate::None => "no border",
            crate::state::DrawingSheetBorderTemplate::OrganizationManaged => "organization border",
        });
    }
    ui.add_enabled_ui(candidate.selected, |ui| {
        ComboBox::from_id_salt((
            "sheet-preset-import-resolution",
            &candidate.portable.stable_id,
        ))
        .selected_text(resolution_label(candidate.resolution))
        .show_ui(ui, |ui| {
            if candidate.existing_id.is_none() && !candidate.missing_managed_dependency {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::NewIdentity,
                    resolution_label(ImportResolution::NewIdentity),
                );
            }
            if candidate.existing_id.is_some() {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::MapExisting,
                    resolution_label(ImportResolution::MapExisting),
                );
                if !candidate.missing_managed_dependency {
                    ui.selectable_value(
                        &mut candidate.resolution,
                        ImportResolution::KeepBothRename,
                        resolution_label(ImportResolution::KeepBothRename),
                    );
                }
            }
            if candidate.resolution == ImportResolution::MatchesByDigest {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::MatchesByDigest,
                    resolution_label(ImportResolution::MatchesByDigest),
                );
            }
            if candidate.missing_managed_dependency {
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::ReplaceManagedDependencies,
                    resolution_label(ImportResolution::ReplaceManagedDependencies),
                );
                ui.selectable_value(
                    &mut candidate.resolution,
                    ImportResolution::RetainUnavailableDependency,
                    resolution_label(ImportResolution::RetainUnavailableDependency),
                );
            }
            ui.selectable_value(
                &mut candidate.resolution,
                ImportResolution::Skip,
                resolution_label(ImportResolution::Skip),
            );
        });
    });
    ui.end_row();
}

fn export_body(
    ui: &mut Ui,
    transfer: &mut PresetTransferState,
    visible: &[DrawingSheetPreset],
) -> Option<TransferBodyAction> {
    let mut action = None;
    concept_banner(
        ui,
        "The package contains format definitions only.",
        "No schematic geometry, title-block field values, project identity, user paths, credentials, or results are included.",
    );
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        if Button::new("Select all").show(ui).clicked() {
            action = Some(TransferBodyAction::SelectAllExport(true));
        }
        if Button::new("Clear").show(ui).clicked() {
            action = Some(TransferBodyAction::SelectAllExport(false));
        }
        let references = if transfer.include_builtin_frame_references {
            "built-in references"
        } else {
            "no reference manifest"
        };
        let metadata = if transfer.include_source_metadata {
            "source metadata"
        } else {
            "no source metadata"
        };
        ui.weak(format!(
            "Schema 1 \u{00b7} exact micrometres \u{00b7} stable ids \u{00b7} {references} \u{00b7} {metadata} \u{00b7} SHA-256 digest"
        ));
    });
    ui.add_space(10.0);
    section_heading(ui, "Definitions");
    ScrollArea::vertical()
        .id_salt("sheet-preset-export-definitions")
        .max_height(360.0)
        .show(ui, |ui| {
            Grid::new("sheet-preset-export-grid")
                .num_columns(5)
                .striped(true)
                .spacing([14.0, 8.0])
                .show(ui, |ui| {
                    table_header(ui, "Export");
                    table_header(ui, "Preset");
                    table_header(ui, "Scope");
                    table_header(ui, "Portrait dimensions");
                    table_header(ui, "Dependencies");
                    ui.end_row();
                    for preset in visible {
                        let identity = super::transfer_identity(preset);
                        let mut selected = transfer.export_ids.contains(&identity);
                        let available = !unavailable(preset);
                        let response =
                            ui.add_enabled(available, egui::Checkbox::new(&mut selected, ""));
                        if response.changed() {
                            if selected {
                                transfer.export_ids.insert(identity);
                            } else {
                                transfer.export_ids.remove(&identity);
                            }
                        }
                        ui.vertical(|ui| {
                            ui.label(&preset.name);
                            if !available {
                                ui.colored_label(
                                    Tokens::get(ui.ctx()).color.warn,
                                    "Unavailable dependency",
                                );
                            }
                        });
                        ui.label(scope_label(preset.scope));
                        ui.label(portrait_dimensions(preset));
                        ui.label("margins \u{00b7} frame contract");
                        ui.end_row();
                    }
                });
        });
    ui.add_space(10.0);
    field_label(ui, "Package format");
    ComboBox::from_id_salt("sheet-preset-export-format")
        .width(ui.available_width())
        .selected_text(transfer.package_format.label())
        .show_ui(ui, |ui| {
            for format in [
                PresetPackageFormat::CanonicalSchema1,
                PresetPackageFormat::HumanReviewJson,
            ] {
                ui.selectable_value(&mut transfer.package_format, format, format.label());
            }
        });
    ui.checkbox(
        &mut transfer.include_builtin_frame_references,
        "Include compatible built-in border and title-block references",
    );
    ui.checkbox(
        &mut transfer.include_source_metadata,
        "Include digest and source metadata",
    );
    if let Some(error) = transfer.error.as_deref() {
        ui.add_space(8.0);
        error_notice(ui, error);
    }
    action
}

pub(super) fn delete_body(ui: &mut Ui, preset: Option<&DrawingSheetPreset>, used_by: usize) {
    if let Some(preset) = preset {
        key_value(ui, "Preset", &preset.name);
        key_value(ui, "Stable identity", &preset.id);
        key_value(ui, "Authority", scope_label(preset.scope));
        key_value(ui, "Portrait size", &portrait_dimensions(preset));
        key_value(
            ui,
            "Used by",
            &format!(
                "{used_by} drawing-sheet reference{}",
                if used_by == 1 { "" } else { "s" }
            ),
        );
    } else {
        error_notice(ui, "The selected definition is no longer available.");
    }
}

pub(super) fn error_notice(ui: &mut Ui, message: &str) {
    notice(ui, message, NoticeTone::Error);
}

fn warning_notice(ui: &mut Ui, message: &str) {
    notice(ui, message, NoticeTone::Warning);
}

fn concept_banner(ui: &mut Ui, title: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(title)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.weak(body);
        });
}

fn workflow_note(ui: &mut Ui, title: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.label(RichText::new(title).strong());
            ui.weak(body);
        });
}

#[derive(Debug, Clone, Copy)]
enum NoticeTone {
    Warning,
    Error,
}

fn notice(ui: &mut Ui, message: &str, tone: NoticeTone) {
    let t = Tokens::get(ui.ctx());
    let color = match tone {
        NoticeTone::Warning => t.color.warn,
        NoticeTone::Error => t.color.err,
    };
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .stroke(egui::Stroke::new(1.0, color))
        .inner_margin(egui::Margin::same(9))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(message).color(color));
        });
}

fn empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.canvas_bg)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(title)
                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.weak(detail);
            });
        });
}

fn section_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(egui::Margin::symmetric(9, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
    ui.add_space(6.0);
}

fn table_header(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn field_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_dim),
    );
}

fn key_value(ui: &mut Ui, key: &str, value: &str) {
    Grid::new(("custom-sheet-key-value", key))
        .num_columns(2)
        .spacing([16.0, 6.0])
        .show(ui, |ui| {
            ui.weak(key);
            ui.label(
                RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text),
            );
            ui.end_row();
        });
}

fn scope_badge(ui: &mut Ui, scope: DrawingSheetPresetScope) {
    let t = Tokens::get(ui.ctx());
    let color = match scope {
        DrawingSheetPresetScope::Project => t.color.accent,
        DrawingSheetPresetScope::User => t.color.ok,
        DrawingSheetPresetScope::Organization => t.color.text_dim,
    };
    ui.label(
        RichText::new(scope_label(scope).to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn state_badge(ui: &mut Ui, unavailable: bool) {
    let t = Tokens::get(ui.ctx());
    let (label, color) = if unavailable {
        ("unavailable", t.color.warn)
    } else {
        ("resolved", t.color.ok)
    };
    ui.label(
        RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn custom_preview_caption(draft: &PresetEditorDraft) -> String {
    let (width_um, height_um) = draft.last_valid_preview.portrait_dimensions_um();
    format!(
        "Preview \u{00b7} {} \u{00d7} {} {} portrait",
        format_dimension_um(width_um, draft.unit),
        format_dimension_um(height_um, draft.unit),
        draft.unit.suffix(),
    )
}

fn scope_label(scope: DrawingSheetPresetScope) -> &'static str {
    match scope {
        DrawingSheetPresetScope::Project => "Project",
        DrawingSheetPresetScope::User => "Personal",
        DrawingSheetPresetScope::Organization => "Organization",
    }
}

fn unit_label(unit: PresetEditorUnit) -> &'static str {
    match unit {
        PresetEditorUnit::Millimetres => "Millimetres (mm)",
        PresetEditorUnit::Centimetres => "Centimetres (cm)",
        PresetEditorUnit::Inches => "Inches (in)",
    }
}

fn frame_label(frame: StartingFrame) -> &'static str {
    match frame {
        StartingFrame::IsoA => "Copy ISO A defaults",
        StartingFrame::AnsiA => "Copy ANSI A defaults",
        StartingFrame::Plain => "Plain 10 mm margins",
        StartingFrame::None => "No border or title block",
    }
}

fn resolution_label(resolution: ImportResolution) -> &'static str {
    match resolution {
        ImportResolution::NewIdentity => "Import as project preset",
        ImportResolution::MatchesByDigest => "Map exact existing definition",
        ImportResolution::KeepBothRename => "Keep both and rename import",
        ImportResolution::MapExisting => "Map to existing definition",
        ImportResolution::ReplaceManagedDependencies => "Replace managed dependencies",
        ImportResolution::RetainUnavailableDependency => "Retain as unavailable",
        ImportResolution::Skip => "Skip",
    }
}

fn portrait_dimensions(preset: &DrawingSheetPreset) -> String {
    portrait_dimensions_from_format(&preset.format)
}

fn portrait_dimensions_from_format(format: &crate::state::SchematicSheetFormat) -> String {
    let AuthoredDrawingSheetSize::Custom { snapshot } = &format.authored_size else {
        return "Not a custom size".to_owned();
    };
    format!(
        "{} \u{00d7} {} mm",
        format_mm(snapshot.portrait_width_um),
        format_mm(snapshot.portrait_height_um)
    )
}

fn format_mm(value_um: u64) -> String {
    let whole = value_um / 1_000;
    let fraction = value_um % 1_000;
    if fraction == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{fraction:03}")
            .trim_end_matches('0')
            .to_owned()
    }
}

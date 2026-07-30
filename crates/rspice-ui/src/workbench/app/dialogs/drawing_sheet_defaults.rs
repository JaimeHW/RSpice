//! Project and personal defaults for future drawing sheets.
//!
//! This surface edits isolated candidates. Project changes publish through the
//! design-management revision/history authority; personal changes publish
//! through `UserPreferences`. Existing authored sheet records are never
//! rewritten by this workflow.

use egui::{ComboBox, Context, Frame, RichText, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    AuthoredDrawingSheetSize, DrawingSheetBorderTemplate, DrawingSheetDisplayUnit,
    DrawingSheetNewSheetPolicy, DrawingSheetProjectSettings, DrawingSheetStandard,
    DrawingSheetTitleBlockTemplate, SchematicPageOrientation, SchematicSheetFormat,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::DrawingSheetPersonalPreferences;
use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};

use super::drawing_sheet_preview::drawing_sheet_preview;

const EYEBROW: &str = "NEW PROJECTS \u{00b7} NEW SHEETS \u{00b7} INHERITANCE";
const TITLE: &str = "Drawing-sheet defaults";
const DESCRIPTION: &str = "Choose what future projects and sheets start with. Existing authored sheets are never rewritten by changing a default.";

#[derive(Debug, Clone)]
pub(crate) struct DrawingSheetDefaultsDialogState {
    pub(crate) open: bool,
    edit: Option<SchematicEditAuthority>,
    catalog_revision: u64,
    baseline_project: DrawingSheetProjectSettings,
    draft_project: DrawingSheetProjectSettings,
    baseline_personal: DrawingSheetPersonalPreferences,
    draft_personal: DrawingSheetPersonalPreferences,
    error: Option<String>,
}

impl Default for DrawingSheetDefaultsDialogState {
    fn default() -> Self {
        Self {
            open: false,
            edit: None,
            catalog_revision: 0,
            baseline_project: DrawingSheetProjectSettings::default(),
            draft_project: DrawingSheetProjectSettings::default(),
            baseline_personal: DrawingSheetPersonalPreferences::default(),
            draft_personal: DrawingSheetPersonalPreferences::default(),
            error: None,
        }
    }
}

pub(crate) fn open_drawing_sheet_defaults(state: &mut AppState) -> bool {
    if state.dialogs.drawing_sheet_defaults.open {
        return false;
    }
    let project = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .clone();
    let personal = state.ui.preferences.drawing_sheet_personal_preferences();
    state.dialogs.drawing_sheet_defaults = DrawingSheetDefaultsDialogState {
        open: true,
        edit: Some(SchematicEditAuthority::capture(state)),
        catalog_revision: state.workspace.design_management.revision(),
        baseline_project: project.clone(),
        draft_project: project,
        baseline_personal: personal.clone(),
        draft_personal: personal,
        error: None,
    };
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_defaults_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.drawing_sheet_defaults.open {
            return;
        }
        let transaction = &self.state.dialogs.drawing_sheet_defaults;
        let project_changed = transaction.draft_project != transaction.baseline_project;
        let personal_changed = transaction.draft_personal != transaction.baseline_personal;
        let project_authority = project_changed
            .then(|| validate_project_authority(&self.state, transaction).err())
            .flatten();
        let project_editable = validate_project_authority(&self.state, transaction).is_ok();
        let valid = transaction.draft_project.validate().is_ok()
            && transaction.draft_personal.validate().is_ok();
        let enabled = valid && project_authority.is_none() && (project_changed || personal_changed);

        let choice = Dialog::new(EYEBROW, TITLE, "Save sheet defaults")
            .description(DESCRIPTION)
            .size(DialogSize::DrawingSheetWorkflow)
            // The complete defaults contract is taller than a 720 pt desktop
            // viewport. Keep the authored surface stable and scroll only its
            // body beneath the persistent footer.
            .fixed_height(760.0)
            .ghost("Cancel")
            .primary_enabled(enabled)
            .show(ctx, |ui| {
                drawing_sheet_defaults_body(
                    ui,
                    &mut self.state.dialogs.drawing_sheet_defaults,
                    project_authority.as_deref(),
                    project_editable,
                );
            });
        match choice {
            DialogChoice::Primary => match apply_drawing_sheet_defaults(self) {
                Ok(message) => {
                    self.state.push_user_message(ConsoleMessage::info(message));
                    self.state.dialogs.drawing_sheet_defaults = Default::default();
                }
                Err(error) => self.state.dialogs.drawing_sheet_defaults.error = Some(error),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_defaults = Default::default();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn drawing_sheet_defaults_body(
    ui: &mut Ui,
    state: &mut DrawingSheetDefaultsDialogState,
    project_authority_error: Option<&str>,
    project_editable: bool,
) {
    let t = Tokens::get(ui.ctx());
    let project_authority = state.baseline_project.default_format.clone();
    let personal_authority = state.baseline_personal.default_format.clone();
    concept_banner(
        ui,
        "Defaults seed new work; they do not migrate existing work.",
        "The personal default starts a new project. The project default starts a new sheet in this project. A sheet may follow the project default or save an explicit override.",
    );
    ui.add_space(10.0);
    ui.columns(2, |columns| {
        default_card(
            &mut columns[0],
            "Project default",
            "project",
            &mut state.draft_project.default_format,
            &project_authority,
            project_editable,
            "Saved with this project. New schematic sheets inherit it unless their creation workflow chooses an explicit format.",
        );
        default_card(
            &mut columns[1],
            "Personal default",
            "personal",
            &mut state.draft_personal.default_format,
            &personal_authority,
            true,
            "Saved in personal preferences. Used only when a new project has no organization or template default.",
        );
    });
    ui.add_space(10.0);
    ui.group(|ui| {
        group_heading(ui, "New-sheet behavior");
        ui.label("When a new sheet is created");
        ComboBox::from_id_salt("drawing-sheet-new-policy")
            .width(ui.available_width())
            .selected_text(new_sheet_policy_label(state.draft_project.new_sheet_policy))
            .show_ui(ui, |ui| {
                for value in [
                    DrawingSheetNewSheetPolicy::ProjectDefault,
                    DrawingSheetNewSheetPolicy::Ask,
                    DrawingSheetNewSheetPolicy::MatchCurrent,
                ] {
                    ui.selectable_value(
                        &mut state.draft_project.new_sheet_policy,
                        value,
                        new_sheet_policy_label(value),
                    );
                }
            });
        ui.add_space(8.0);
        ui.checkbox(
            &mut state.draft_project.remember_last_explicit_format,
            "Remember the last explicit choice within this project",
        );
    });

    ui.add_space(10.0);
    ui.columns(3, |columns| {
        note(
            &mut columns[0],
            "Inheritance",
            "A following sheet moves when the project default changes. A sheet override does not. Page Setup states the source before Apply.",
        );
        note(
            &mut columns[1],
            "Organization policy",
            "Managed projects may replace either default and lock selected border or title-block properties while leaving dimensions and orientation editable.",
        );
        columns[2].group(|ui| {
            group_heading(ui, "Reset");
            if Button::new("Reset project default")
                .enabled(project_editable)
                .show(ui)
                .clicked()
            {
                let reset = DrawingSheetProjectSettings::default();
                state.draft_project.default_format = reset.default_format;
                enforce_managed_default_authority(
                    &mut state.draft_project.default_format,
                    &state.baseline_project.default_format,
                );
            }
            if Button::new("Reset personal default").show(ui).clicked() {
                state.draft_personal.default_format =
                    DrawingSheetPersonalPreferences::default().default_format;
                enforce_managed_default_authority(
                    &mut state.draft_personal.default_format,
                    &state.baseline_personal.default_format,
                );
            }
        });
    });

    ui.add_space(10.0);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 42.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            if let Some(error) = project_authority_error.or(state.error.as_deref()) {
                Frame::NONE
                    .fill(t.color.bg_panel)
                    .stroke(egui::Stroke::new(1.0, t.color.err.gamma_multiply(0.5)))
                    .inner_margin(egui::Margin::symmetric(10, 7))
                    .show(ui, |ui| {
                        ui.colored_label(t.color.err, error);
                    });
            }
        },
    );
}

fn default_card(
    ui: &mut Ui,
    title: &str,
    scope: &str,
    format: &mut SchematicSheetFormat,
    managed_authority: &SchematicSheetFormat,
    enabled: bool,
    help: &str,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            group_heading(ui, title);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let t = Tokens::get(ui.ctx());
                Frame::NONE
                    .fill(if scope == "project" {
                        t.color.bg_active
                    } else {
                        t.color.bg_inset
                    })
                    .stroke(egui::Stroke::new(1.0, t.color.border))
                    .inner_margin(egui::Margin::symmetric(7, 3))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(scope)
                                .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                                .color(if scope == "project" {
                                    t.color.ok
                                } else {
                                    t.color.text_dim
                                }),
                        );
                    });
            });
        });
        let label = format_summary(format);
        drawing_sheet_preview(ui, format, 118.0, &label);
        ui.add_space(8.0);
        ui.add_enabled_ui(enabled, |ui| {
            let current_standard = match format.authored_size {
                AuthoredDrawingSheetSize::Standard { standard } => Some(standard),
                AuthoredDrawingSheetSize::Custom { .. } => None,
            };
            let mut selected = current_standard;
            ui.label("Format");
            ComboBox::from_id_salt(("drawing-sheet-default-format", scope))
                .width(ui.available_width())
                .selected_text(current_standard.map_or_else(
                    || format!("Custom \u{00b7} {}", format_summary(format)),
                    |standard| default_format_option_label(standard, format.display_unit),
                ))
                .show_ui(ui, |ui| {
                    for standard in DrawingSheetStandard::ALL {
                        ui.selectable_value(
                            &mut selected,
                            Some(standard),
                            default_format_option_label(standard, format.display_unit),
                        );
                    }
                });
            if selected != current_standard
                && let Some(selected) = selected
            {
                *format = retarget_default_to_standard(format, selected);
            }
            ui.add_space(8.0);
            ui.columns(2, |columns| {
                columns[0].label("Orientation");
                columns[1].label("Units");
                columns[0].add_space(3.0);
                columns[1].add_space(3.0);
                {
                    let previous_orientation = format.orientation;
                    let mut selected_orientation = previous_orientation;
                    enum_combo(
                        &mut columns[0],
                        ("orientation", scope),
                        &mut selected_orientation,
                        &[
                            (SchematicPageOrientation::Landscape, "Landscape"),
                            (SchematicPageOrientation::Portrait, "Portrait"),
                        ],
                    );
                    if selected_orientation != previous_orientation
                        && let Ok(updated) = format.try_update(|draft| {
                            draft.orientation = selected_orientation;
                        })
                    {
                        *format = updated;
                    }
                }
                {
                    enum_combo(
                        &mut columns[1],
                        ("unit", scope),
                        &mut format.display_unit,
                        &[
                            (DrawingSheetDisplayUnit::Millimetres, "mm"),
                            (DrawingSheetDisplayUnit::Centimetres, "cm"),
                            (DrawingSheetDisplayUnit::Inches, "in"),
                        ],
                    );
                }
            });
            ui.add_space(8.0);
            ui.label("Border");
            let border_locked =
                managed_authority.border == DrawingSheetBorderTemplate::OrganizationManaged;
            ui.add_enabled_ui(!border_locked, |ui| {
                let previous_border = format.border;
                let mut selected_border = previous_border;
                if border_locked {
                    enum_combo(
                        ui,
                        ("border", scope),
                        &mut selected_border,
                        &[(
                            DrawingSheetBorderTemplate::OrganizationManaged,
                            "Organization border · managed",
                        )],
                    );
                } else {
                    enum_combo(
                        ui,
                        ("border", scope),
                        &mut selected_border,
                        &[
                            (
                                DrawingSheetBorderTemplate::Standard,
                                "Standard border with zones",
                            ),
                            (DrawingSheetBorderTemplate::Plain, "Plain border"),
                            (DrawingSheetBorderTemplate::None, "No border"),
                        ],
                    );
                }
                if selected_border != previous_border
                    && let Ok(updated) =
                        format.try_update(|draft| draft.apply_border_template(selected_border))
                {
                    *format = updated;
                }
            });
            ui.add_space(8.0);
            ui.label("Title block");
            let title_locked = managed_authority.title_block.template
                == DrawingSheetTitleBlockTemplate::OrganizationManaged;
            ui.add_enabled_ui(!title_locked, |ui| {
                let previous_title_block = format.title_block.template;
                let mut selected_title_block = previous_title_block;
                if title_locked {
                    enum_combo(
                        ui,
                        ("title-block", scope),
                        &mut selected_title_block,
                        &[(
                            DrawingSheetTitleBlockTemplate::OrganizationManaged,
                            "Organization block · managed",
                        )],
                    );
                } else {
                    enum_combo(
                        ui,
                        ("title-block", scope),
                        &mut selected_title_block,
                        &[
                            (DrawingSheetTitleBlockTemplate::Compact, "RSpice compact"),
                            (DrawingSheetTitleBlockTemplate::Standard, "RSpice standard"),
                            (DrawingSheetTitleBlockTemplate::Wide, "RSpice wide"),
                            (DrawingSheetTitleBlockTemplate::None, "No title block"),
                        ],
                    );
                }
                if selected_title_block != previous_title_block
                    && let Ok(updated) = format.try_update(|draft| {
                        draft.title_block.template = selected_title_block;
                        if selected_title_block == DrawingSheetTitleBlockTemplate::None {
                            draft.title_block.offset_x_um = 0;
                            draft.title_block.offset_y_um = 0;
                            draft.title_block.rotation =
                                crate::state::DrawingSheetTitleBlockRotation::Upright;
                        }
                    })
                {
                    *format = updated;
                }
            });
        });
        ui.add_space(6.0);
        ui.weak(help);
    });
}

/// Move a reusable default to another standard without trusting placement
/// offsets that were authored for a differently sized sheet.
///
/// Keep the requested presentation whenever it still fits. If only the
/// retained offsets make the new standard invalid, normalize those offsets
/// to the template anchor. The final fallback is the new standard's own valid
/// title block; format selection must never panic or leave a corrupt draft.
fn retarget_default_to_standard(
    source: &SchematicSheetFormat,
    standard: DrawingSheetStandard,
) -> SchematicSheetFormat {
    let base = SchematicSheetFormat::from_standard(standard, source.orientation);
    let rebuild = |title_block: crate::state::DrawingSheetTitleBlock| {
        base.try_update(|draft| {
            draft.display_unit = source.display_unit;
            draft.apply_border_template(source.border);
            draft.title_block = title_block;
            draft.inheritance = source.inheritance;
        })
    };

    if let Ok(candidate) = rebuild(source.title_block.clone()) {
        return candidate;
    }

    let mut anchored = source.title_block.clone();
    anchored.offset_x_um = 0;
    anchored.offset_y_um = 0;
    if let Ok(candidate) = rebuild(anchored) {
        return candidate;
    }

    rebuild(base.title_block.clone()).unwrap_or(base)
}

fn enum_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    value: &mut T,
    options: &[(T, &str)],
) {
    let selected = options
        .iter()
        .find_map(|(candidate, label)| (*candidate == *value).then_some(*label))
        .unwrap_or("Unavailable");
    ComboBox::from_id_salt(id)
        .width(ui.available_width())
        .selected_text(selected)
        .show_ui(ui, |ui| {
            for (candidate, label) in options {
                ui.selectable_value(value, *candidate, *label);
            }
        });
}

fn format_summary(format: &SchematicSheetFormat) -> String {
    let (width, height) = format.oriented_dimensions_um();
    format!(
        "{} \u{00b7} {}",
        format.authored_size.label(),
        format.display_unit.format_size_um(width, height)
    )
}

fn default_format_option_label(
    standard: DrawingSheetStandard,
    unit: DrawingSheetDisplayUnit,
) -> String {
    let (width, height) = standard.portrait_dimensions_um();
    format!(
        "{} \u{00b7} {}",
        standard.label(),
        unit.format_size_um(width, height)
    )
}

fn new_sheet_policy_label(value: DrawingSheetNewSheetPolicy) -> &'static str {
    match value {
        DrawingSheetNewSheetPolicy::ProjectDefault => "Follow the project default",
        DrawingSheetNewSheetPolicy::Ask => "Ask in the New Sheet dialog",
        DrawingSheetNewSheetPolicy::MatchCurrent => "Copy the current sheet's format",
    }
}

fn validate_project_authority(
    app: &AppState,
    state: &DrawingSheetDefaultsDialogState,
) -> Result<(), String> {
    state
        .edit
        .as_ref()
        .ok_or_else(|| "Drawing-sheet Defaults has no project edit authority.".to_owned())?
        .validate(app, "Drawing-sheet Defaults")?;
    if app.workspace.design_management.revision() != state.catalog_revision {
        return Err(
            "Project drawing-sheet settings changed. Close and reopen Defaults.".to_owned(),
        );
    }
    Ok(())
}

fn apply_drawing_sheet_defaults(app: &mut RSpiceApp) -> Result<String, String> {
    let mut transaction = app.state.dialogs.drawing_sheet_defaults.clone();
    enforce_managed_default_authority(
        &mut transaction.draft_project.default_format,
        &transaction.baseline_project.default_format,
    );
    enforce_managed_default_authority(
        &mut transaction.draft_personal.default_format,
        &transaction.baseline_personal.default_format,
    );
    transaction.draft_project.default_format = transaction
        .draft_project
        .default_format
        .as_drawing_sheet_default();
    transaction.draft_project.last_explicit_format = transaction
        .draft_project
        .last_explicit_format
        .as_ref()
        .map(SchematicSheetFormat::as_drawing_sheet_default);
    transaction.draft_personal.default_format = transaction
        .draft_personal
        .default_format
        .as_drawing_sheet_default();
    if !transaction.draft_project.remember_last_explicit_format {
        transaction.draft_project.last_explicit_format = None;
    }
    transaction
        .draft_project
        .validate()
        .map_err(|error| error.to_string())?;
    transaction.draft_personal.validate()?;
    if app
        .state
        .ui
        .preferences
        .drawing_sheet_personal_preferences()
        != transaction.baseline_personal
    {
        return Err(
            "Personal drawing-sheet preferences changed. Close and reopen Defaults.".to_owned(),
        );
    }
    let project_changed = transaction.draft_project != transaction.baseline_project;
    let personal_changed = transaction.draft_personal != transaction.baseline_personal;
    if !project_changed && !personal_changed {
        return Ok("Drawing-sheet defaults already matched the saved values.".to_owned());
    }

    let mut personal_candidate = app.state.ui.preferences.clone();
    personal_candidate
        .set_drawing_sheet_personal_preferences(transaction.draft_personal.clone())?;

    if project_changed {
        validate_project_authority(&app.state, &transaction)?;
        let before = app.state.workspace.design_management.clone();
        let mut candidate = before.clone();
        candidate
            .update_drawing_sheet_settings(candidate.revision(), transaction.draft_project.clone())
            .map_err(|error| error.to_string())?;
        commit_project_candidate(app, "Drawing-sheet defaults", before, candidate)?;
    }
    app.state.ui.preferences = personal_candidate;

    Ok(match (project_changed, personal_changed) {
        (true, true) => {
            "Project and personal drawing-sheet defaults saved through their owning authorities."
                .to_owned()
        }
        (true, false) => "Project drawing-sheet defaults saved.".to_owned(),
        (false, true) => "Personal drawing-sheet defaults saved in Preferences.".to_owned(),
        (false, false) => unreachable!(),
    })
}

fn enforce_managed_default_authority(
    candidate: &mut SchematicSheetFormat,
    authority: &SchematicSheetFormat,
) {
    let updated = candidate.clone().try_update(|draft| {
        if authority.border == DrawingSheetBorderTemplate::OrganizationManaged {
            draft.border = authority.border;
            draft.zones.mode = authority.zones.mode;
            draft.zones.custom_columns = authority.zones.custom_columns;
            draft.zones.custom_rows = authority.zones.custom_rows;
        }
        if authority.title_block.template == DrawingSheetTitleBlockTemplate::OrganizationManaged {
            draft.title_block.template = authority.title_block.template;
        }
    });
    if let Ok(updated) = updated {
        *candidate = updated;
    }
}

pub(super) fn commit_project_candidate(
    app: &mut RSpiceApp,
    description: &str,
    before: crate::state::DesignManagementCatalog,
    candidate: crate::state::DesignManagementCatalog,
) -> Result<(), String> {
    candidate.validate().map_err(|error| error.to_string())?;
    let schematic_tx = app
        .state
        .prepare_design_management_schematic_transaction(&candidate)?;
    let owner = app.state.workspace.active_schematic_reference();
    let committed_revision = app
        .state
        .workspace
        .replace_design_management(candidate)
        .map_err(|error| error.to_string())?;
    app.state
        .apply_design_management_schematic_transaction(&schematic_tx);
    let after = app.state.workspace.design_management.clone();
    app.state
        .record_design_management_transaction(DesignManagementHistoryEntry {
            description: description.to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            committed_revision,
        });
    Ok(())
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
                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                RichText::new(body)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
}

fn group_heading(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(title)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text),
    );
}

fn note(ui: &mut Ui, title: &str, body: &str) {
    ui.group(|ui| {
        group_heading(ui, title);
        ui.weak(body);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_defaults_are_a_real_no_op() {
        let mut app = RSpiceApp::test_instance();
        open_drawing_sheet_defaults(&mut app.state);
        let revision = app.state.workspace.design_management.revision();

        assert_eq!(
            apply_drawing_sheet_defaults(&mut app).unwrap(),
            "Drawing-sheet defaults already matched the saved values."
        );
        assert_eq!(app.state.workspace.design_management.revision(), revision);
    }

    #[test]
    fn project_default_is_one_history_transaction_and_personal_uses_preferences() {
        let mut app = RSpiceApp::test_instance();
        open_drawing_sheet_defaults(&mut app.state);
        app.state
            .dialogs
            .drawing_sheet_defaults
            .draft_project
            .default_format = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::AnsiA,
            SchematicPageOrientation::Landscape,
        )
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
        })
        .unwrap();
        app.state
            .dialogs
            .drawing_sheet_defaults
            .draft_personal
            .default_format = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::JisB4,
            SchematicPageOrientation::Portrait,
        )
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::UserDefault;
        })
        .unwrap();

        let before = app.state.workspace.design_management.revision();
        apply_drawing_sheet_defaults(&mut app).unwrap();

        assert_eq!(app.state.workspace.design_management.revision(), before + 1);
        assert!(app.state.can_undo_project_design());
        assert_eq!(
            app.state
                .ui
                .preferences
                .drawing_sheet_personal_preferences()
                .default_format
                .authored_size
                .label(),
            "JIS B4"
        );
    }

    #[test]
    fn managed_default_properties_are_reapplied_at_commit_without_locking_orientation() {
        let mut app = RSpiceApp::test_instance();
        let mut settings = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .clone();
        settings.default_format = settings
            .default_format
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::OrganizationManaged);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            })
            .unwrap();
        let revision = app.state.workspace.design_management.revision();
        app.state
            .workspace
            .design_management
            .update_drawing_sheet_settings(revision, settings)
            .unwrap();
        open_drawing_sheet_defaults(&mut app.state);
        let draft = &mut app
            .state
            .dialogs
            .drawing_sheet_defaults
            .draft_project
            .default_format;
        *draft = draft
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::None);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                draft.orientation = SchematicPageOrientation::Portrait;
            })
            .unwrap();

        apply_drawing_sheet_defaults(&mut app).unwrap();

        let saved = &app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings()
            .default_format;
        assert_eq!(
            saved.border,
            DrawingSheetBorderTemplate::OrganizationManaged
        );
        assert_eq!(
            saved.title_block.template,
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        );
        assert_eq!(saved.orientation, SchematicPageOrientation::Portrait);
    }

    #[test]
    fn managed_default_locks_are_independent_per_property() {
        let border_authority = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::OrganizationManaged);
            })
            .unwrap();
        let mut border_candidate = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::None);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::Wide;
            })
            .unwrap();
        enforce_managed_default_authority(&mut border_candidate, &border_authority);
        assert_eq!(
            border_candidate.border,
            DrawingSheetBorderTemplate::OrganizationManaged
        );
        assert_eq!(
            border_candidate.title_block.template,
            DrawingSheetTitleBlockTemplate::Wide
        );

        let title_authority = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            })
            .unwrap();
        let mut title_candidate = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.apply_border_template(DrawingSheetBorderTemplate::Plain);
                draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
            })
            .unwrap();
        enforce_managed_default_authority(&mut title_candidate, &title_authority);
        assert_eq!(title_candidate.border, DrawingSheetBorderTemplate::Plain);
        assert_eq!(
            title_candidate.title_block.template,
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        );
    }

    #[test]
    fn changing_default_standard_normalizes_an_offset_that_no_longer_fits() {
        let source = SchematicSheetFormat::from_standard(
            DrawingSheetStandard::IsoA0,
            SchematicPageOrientation::Landscape,
        )
        .try_update(|draft| {
            draft.title_block.offset_x_um = -300_000;
        })
        .unwrap();

        let retargeted = retarget_default_to_standard(&source, DrawingSheetStandard::IsoA5);

        assert_eq!(
            retargeted.authored_size,
            AuthoredDrawingSheetSize::Standard {
                standard: DrawingSheetStandard::IsoA5,
            }
        );
        assert_eq!(retargeted.title_block.template, source.title_block.template);
        assert_eq!(retargeted.title_block.offset_x_um, 0);
        assert_eq!(retargeted.title_block.offset_y_um, 0);
        retargeted.validate().unwrap();
    }
}

//! Report editor dialogs over local drafts and app-owned source/transaction authority.
use super::{
    page_marker,
    session::{ReportAuthoringState, page_update_policy_index, valid_title},
};
use egui::{Ui, Vec2};
use rspice_results::report_document::ReportDocument;
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Dialog, DialogChoice, DialogInitialFocus, input_row, select},
};

pub trait DialogHost {
    type Figure;
    fn editor(&mut self) -> &mut ReportAuthoringState;
    fn writable(&self) -> bool;
    fn blocked_reason(&self) -> &'static str;
    fn document_snapshot(&self) -> Option<ReportDocument>;
    fn run_options(&self) -> Vec<String>;
    fn removal_target(&self) -> (String, bool);
    fn figure_options(&self) -> Vec<Self::Figure>;
    fn figure_label(figure: &Self::Figure) -> &str;
    fn select_figure(&mut self, index: usize, figure: Option<&Self::Figure>);
    fn commit_create_document(&mut self);
    fn commit_add_page(&mut self);
    fn commit_page_properties(&mut self);
    fn commit_remove_report_block(&mut self);
    fn commit_insert_result_document(&mut self);
    fn commit_add_report_element(&mut self);
}

pub fn show(ctx: &egui::Context, host: &mut impl DialogHost) {
    create_document_dialog(ctx, host);
    add_page_dialog(ctx, host);
    page_properties_dialog(ctx, host);
    remove_report_block_dialog(ctx, host);
    insert_result_document_dialog(ctx, host);
    add_report_element_dialog(ctx, host);
}

fn create_document_dialog<H: DialogHost>(ctx: &egui::Context, host: &mut H) {
    if !host.editor().create_document_open {
        return;
    }
    const TEMPLATE_LABELS: [&str; 3] = [
        "Release verification 4.2",
        "Design review",
        "Model qualification",
    ];
    let valid = valid_title(&host.editor().create_document_title);
    let writable = host.writable();
    let error = host.editor().transaction_error.clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · TRACEABLE DERIVED EVIDENCE",
        "Plan report artifact",
        "Create report document",
    )
    .description(
        "Create one explicit project-owned report source and its mockup-specified page outline.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let response = input_row(
            ui,
            "Report title",
            &mut host.editor()
                .create_document_title,
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            let label_width = 130.0_f32.min(ui.available_width() * 0.32);
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Template"),
            );
            let selected = host.editor()
                .create_document_template
                .min(TEMPLATE_LABELS.len() - 1);
            let options = TEMPLATE_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-document-template",
                "Report document template",
                TEMPLATE_LABELS[selected],
                &options,
                ui.available_width(),
            ) {
                host.editor()
                    .create_document_template = index;
            }
        });
        ui.add_space(8.0);
        ui.label(
            "The document is created only when this transaction commits; opening Report Authoring never changes the project.",
        );
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                host.blocked_reason(),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid && writable => host.commit_create_document(),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            host.editor().create_document_open = false;
            host.editor().transaction_error = None;
        }
        _ => {}
    }
}

fn add_page_dialog<H: DialogHost>(ctx: &egui::Context, host: &mut H) {
    if !host.editor().add_page_open {
        return;
    }
    let valid = valid_title(&host.editor().add_page_title);
    let error = host.editor().transaction_error.clone();
    let choice = Dialog::new(
        "REPORTING · DOCUMENT COMPOSITION",
        "Add report page",
        "Add page",
    )
    .description("Add one versioned page to the project-owned report document.")
    .ghost("Cancel")
    .primary_enabled(valid)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let response = input_row(ui, "Page title", &mut host.editor().add_page_title);
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid => host.commit_add_page(),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            host.editor().add_page_open = false;
            host.editor().transaction_error = None;
        }
        _ => {}
    }
}

fn page_properties_dialog<H: DialogHost>(ctx: &egui::Context, host: &mut H) {
    if !host.editor().page_properties_open {
        return;
    }
    let valid = valid_title(&host.editor().page_title_draft);
    let error = host.editor().transaction_error.clone();
    let Some(document) = host.document_snapshot() else {
        host.editor().page_properties_open = false;
        return;
    };
    let page_options = document
        .pages()
        .iter()
        .enumerate()
        .map(|(index, page)| format!("{} · {}", page_marker(index, page.title()), page.title()))
        .collect::<Vec<_>>();
    let page_index = host
        .editor()
        .page_properties_page
        .and_then(|page_id| {
            document
                .pages()
                .iter()
                .position(|page| page.id() == page_id)
        })
        .unwrap_or_default();
    const TEMPLATE_LABELS: [&str; 3] = [
        "Release verification 4.2",
        "Design review",
        "Model qualification",
    ];
    const UPDATE_POLICY_LABELS: [&str; 2] = [
        "Refresh linked figures automatically",
        "Freeze selected figure revision",
    ];
    let choice = Dialog::new(
        "REPORTING · DOCUMENT COMPOSITION",
        "Report page properties",
        "Save page properties",
    )
    .description("Edit the selected page through one revision-checked report transaction.")
    .ghost("Cancel")
    .primary_enabled(valid)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 130.0_f32.min(ui.available_width() * 0.32);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Template"),
            );
            let selected_index = host
                .editor()
                .report_template_draft
                .min(TEMPLATE_LABELS.len() - 1);
            let options = TEMPLATE_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-page-template",
                "Report template",
                TEMPLATE_LABELS[selected_index],
                &options,
                ui.available_width(),
            ) {
                host.editor().report_template_draft = index;
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Page"),
            );
            if let Some(index) = select(
                ui,
                "report-page-selection",
                "Report page",
                page_options
                    .get(page_index)
                    .map_or("No page", String::as_str),
                &page_options,
                ui.available_width(),
            ) && let Some(page) = document.pages().get(index)
            {
                let editor = host.editor();
                editor.page_properties_page = Some(page.id());
                editor.page_title_draft = page.title().to_owned();
                editor.page_update_policy_draft = page_update_policy_index(page.update_policy());
                editor.transaction_error = None;
            }
        });
        ui.add_space(8.0);
        let response = input_row(ui, "Page title", &mut host.editor().page_title_draft);
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Update policy"),
            );
            let selected_index = host
                .editor()
                .page_update_policy_draft
                .min(UPDATE_POLICY_LABELS.len() - 1);
            let options = UPDATE_POLICY_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-page-update-policy",
                "Report page update policy",
                UPDATE_POLICY_LABELS[selected_index],
                &options,
                ui.available_width(),
            ) {
                host.editor().page_update_policy_draft = index;
            }
        });
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid => host.commit_page_properties(),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            host.editor().page_properties_open = false;
            host.editor().transaction_error = None;
        }
        _ => {}
    }
}

fn remove_report_block_dialog<H: DialogHost>(ctx: &egui::Context, host: &mut H) {
    if !host.editor().remove_report_block_open {
        return;
    }
    let (block_title, valid) = host.removal_target();
    let writable = host.writable();
    let error = host.editor().transaction_error.clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · PAGE ELEMENT",
        "Remove page element",
        "Remove",
    )
    .description(
        "Remove the selected element from this report revision. Its stable identity is retained as a tombstone in the report audit history.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::Primary)
    .show(ctx, |ui| {
        ui.label(format!("Element: {block_title}"));
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                host.blocked_reason(),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
    });
    match choice {
        DialogChoice::Primary if valid && writable => host.commit_remove_report_block(),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            host.editor().remove_report_block_open = false;
            host.editor().transaction_error = None;
        }
        _ => {}
    }
}

fn insert_result_document_dialog<H: DialogHost>(ctx: &egui::Context, host: &mut H) {
    if !host.editor().insert_result_document_open {
        return;
    }
    const SIZING_LABELS: [&str; 3] = ["Fit width", "Fit page", "Natural size"];
    let figure_options = host.figure_options();
    let source_options = figure_options
        .iter()
        .map(|option| <H as DialogHost>::figure_label(option).to_owned())
        .collect::<Vec<_>>();
    let source_index = host
        .editor()
        .insert_result_document_index
        .min(source_options.len().saturating_sub(1));
    let editor = host.editor();
    let caption = editor.insert_result_caption.trim();
    let alternative_text = editor.insert_result_alternative_text.trim();
    let valid = !source_options.is_empty()
        && !caption.is_empty()
        && caption.len() <= 2_048
        && !caption.chars().any(char::is_control)
        && !alternative_text.is_empty()
        && alternative_text.len() <= 8_192
        && !alternative_text
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t');
    let writable = host.writable();
    let error = host.editor().transaction_error.clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · IMMUTABLE RESULT SOURCE",
        "Insert result document",
        "Insert result document",
    )
    .description(
        "Insert one exact visualization-document revision with all immutable dataset bindings retained for publication audit.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 132.0_f32.min(ui.available_width() * 0.34);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Result document"),
            );
            let current = source_options
                .get(source_index)
                .map_or("No retained result documents", String::as_str);
            if let Some(index) = select(
                ui,
                "report-insert-result-source",
                "Immutable result document",
                current,
                &source_options,
                ui.available_width(),
            ) {
                host.select_figure(index,figure_options.get(index));
            }
        });
        ui.add_space(8.0);
        let focus = input_row(
            ui,
            "Caption",
            &mut host.editor()
                .insert_result_caption,
        );
        ui.add_space(8.0);
        input_row(
            ui,
            "Alternative text",
            &mut host.editor()
                .insert_result_alternative_text,
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Sizing"),
            );
            let sizing_index = host.editor()
                .insert_result_sizing
                .min(SIZING_LABELS.len() - 1);
            let labels = SIZING_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-insert-result-sizing",
                "Result figure sizing",
                SIZING_LABELS[sizing_index],
                &labels,
                ui.available_width(),
            ) {
                host.editor()
                    .insert_result_sizing = index;
            }
        });
        ui.add_space(8.0);
        ui.checkbox(
            &mut host.editor()
                .insert_result_frozen,
            "Freeze self-contained source payload in this report revision",
        );
        ui.label(if host.editor()
            .insert_result_frozen
        {
            "The exact source snapshot and deterministic publication PNG are embedded and digest-authenticated."
        } else {
            "The block remains linked to one exact immutable visualization revision."
        });
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                host.blocked_reason(),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(focus.id)
    });
    match choice {
        DialogChoice::Primary if valid && writable => host.commit_insert_result_document(),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            host.editor().insert_result_document_open = false;
            host.editor().transaction_error = None;
        }
        _ => {}
    }
}

fn add_report_element_dialog<H: DialogHost>(ctx: &egui::Context, host: &mut H) {
    if !host.editor().add_report_element_open {
        return;
    }
    const KIND_LABELS: [&str; 7] = [
        "Authored prose",
        "Data table",
        "Datasheet field",
        "Requirement statement",
        "Specification result",
        "Review note",
        "Verification evidence",
    ];
    const PROSE_STYLE_LABELS: [&str; 5] = [
        "Body",
        "Executive summary",
        "Method",
        "Conclusion",
        "Warning",
    ];
    let run_options = host.run_options();
    let kind_index = host
        .editor()
        .add_report_element_kind
        .min(KIND_LABELS.len() - 1);
    let source_required = matches!(kind_index, 1 | 2 | 3 | 4 | 6);
    let valid = host
        .editor()
        .valid_add_report_element_draft(!run_options.is_empty());
    let writable = host.writable();
    let error = host.editor().transaction_error.clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · PAGE ELEMENT CATALOG",
        "Add report element",
        "Add element",
    )
    .description(
        "Create one validated report element. Source-derived elements bind to one exact immutable dataset; result plots use Insert result document.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 134.0_f32.min(ui.available_width() * 0.34);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Element type"),
            );
            let options = KIND_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-add-element-kind",
                "Report element type",
                KIND_LABELS[kind_index],
                &options,
                ui.available_width(),
            ) {
                host.editor().reset_add_report_element_kind(index);
            }
        });
        ui.add_space(8.0);
        let focus = input_row(
            ui,
            if kind_index == 5 { "Author" } else { "Element title" },
            &mut host.editor()
                .add_report_element_title,
        );

        let (primary_label, secondary_label, tertiary_label) =
            add_report_element_field_labels(kind_index);
        ui.add_space(8.0);
        if matches!(kind_index, 0 | 3 | 5 | 6) {
            dialog_text_area(
                ui,
                primary_label,
                &mut host.editor()
                    .add_report_element_primary,
            );
        } else {
            input_row(
                ui,
                primary_label,
                &mut host.editor()
                    .add_report_element_primary,
            );
        }
        if let Some(label) = secondary_label {
            ui.add_space(8.0);
            input_row(
                ui,
                label,
                &mut host.editor()
                    .add_report_element_secondary,
            );
        }
        if let Some(label) = tertiary_label {
            ui.add_space(8.0);
            input_row(
                ui,
                label,
                &mut host.editor()
                    .add_report_element_tertiary,
            );
        }

        if kind_index == 0 {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_sized(
                    Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                    egui::Label::new("Prose style"),
                );
                let style_index = host.editor()
                    .add_report_element_style
                    .min(PROSE_STYLE_LABELS.len() - 1);
                let labels = PROSE_STYLE_LABELS
                    .iter()
                    .map(|label| (*label).to_owned())
                    .collect::<Vec<_>>();
                if let Some(index) = select(
                    ui,
                    "report-add-prose-style",
                    "Report prose style",
                    PROSE_STYLE_LABELS[style_index],
                    &labels,
                    ui.available_width(),
                ) {
                    host.editor()
                        .add_report_element_style = index;
                }
            });
        }
        if matches!(kind_index, 3..=5) {
            ui.add_space(8.0);
            let status_labels: &[&str] = match kind_index {
                3 => &["Not evaluated", "Passed", "Failed", "Waived"],
                4 => &[
                    "Not evaluated",
                    "In specification",
                    "Out of specification",
                    "Informational",
                ],
                _ => &["Open", "Addressed", "Accepted"],
            };
            ui.horizontal(|ui| {
                ui.add_sized(
                    Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                    egui::Label::new("Status"),
                );
                let status_index = host.editor()
                    .add_report_element_status
                    .min(status_labels.len() - 1);
                let labels = status_labels
                    .iter()
                    .map(|label| (*label).to_owned())
                    .collect::<Vec<_>>();
                if let Some(index) = select(
                    ui,
                    "report-add-element-status",
                    "Report element status",
                    status_labels[status_index],
                    &labels,
                    ui.available_width(),
                ) {
                    host.editor()
                        .add_report_element_status = index;
                }
            });
        }
        if source_required {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_sized(
                    Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                    egui::Label::new("Bound source"),
                );
                let run_index = host.editor()
                    .add_report_element_source_run
                    .min(run_options.len().saturating_sub(1));
                let current = run_options
                    .get(run_index)
                    .map_or("No immutable dataset retained", String::as_str);
                if let Some(index) = select(
                    ui,
                    "report-add-element-source",
                    "Immutable dataset source",
                    current,
                    &run_options,
                    ui.available_width(),
                ) {
                    host.editor()
                        .add_report_element_source_run = index;
                }
            });
            if run_options.is_empty() {
                ui.colored_label(
                    Tokens::get(ui.ctx()).color.warn,
                    "Run and retain an analysis before adding this source-derived element.",
                );
            }
        }
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                host.blocked_reason(),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(focus.id)
    });
    match choice {
        DialogChoice::Primary if valid && writable => host.commit_add_report_element(),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            host.editor().add_report_element_open = false;
            host.editor().transaction_error = None;
        }
        _ => {}
    }
}

fn dialog_text_area(ui: &mut Ui, label: &str, value: &mut String) -> egui::Response {
    let label_width = 134.0_f32.min(ui.available_width() * 0.34);
    ui.horizontal_top(|ui| {
        ui.add_sized(Vec2::new(label_width, 82.0), egui::Label::new(label));
        ui.add_sized(
            Vec2::new(ui.available_width(), 82.0),
            egui::TextEdit::multiline(value)
                .desired_rows(4)
                .font(theme::mono(tokens::FS_1, FontWeight::Regular)),
        )
    })
    .inner
}

fn add_report_element_field_labels(
    kind_index: usize,
) -> (&'static str, Option<&'static str>, Option<&'static str>) {
    match kind_index {
        1 => (
            "Column heading",
            Some("Cell value"),
            Some("Unit (optional)"),
        ),
        2 => ("Field label", Some("Field value"), Some("Unit (optional)")),
        3 => (
            "Requirement statement",
            Some("Requirement ID"),
            Some("Evidence label (optional)"),
        ),
        4 => (
            "Expression",
            Some("Limit"),
            Some("Measured value (optional)"),
        ),
        5 => ("Review message", None, None),
        6 => ("Evidence summary", None, None),
        _ => ("Report text", None, None),
    }
}

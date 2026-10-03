//! Worksheet properties, pane order, link groups, and report-page controls.

use super::super::widgets::{dock_intro, empty_note};
use egui::Ui;
use rspice_results::studio_presentation::{MAX_REPORT_PAGE_TITLE_BYTES, REPORT_PAGE_TEMPLATES};
use rspice_ui_kit::{panels::property_row, widgets::Button};

pub trait PropertiesHost {
    fn current(&self) -> (u8, bool);
    fn significant_digits(&mut self) -> &mut Option<u8>;
    fn phase_continuous(&mut self) -> &mut Option<bool>;
    fn save(&mut self, significant_digits: u8, phase_continuous: bool);
}

pub trait LinksHost {
    fn prepare(&mut self) -> Option<u64>;
    fn x_link(&mut self) -> &mut u64;
    fn cursor_group(&mut self) -> &mut u64;
    fn canonical(&self) -> bool;
    fn linked_cursors(&self) -> bool;
    fn set_cursor_links(&mut self, linked: bool);
    fn save(&mut self, pane_id: u64);
}

pub struct PageDraft<'a> {
    pub page: &'a mut String,
    pub template: &'a mut String,
    pub freeze: &'a mut bool,
}

pub trait PageHost {
    fn prepare(&mut self) -> Option<u64>;
    fn draft(&mut self) -> PageDraft<'_>;
    fn save(&mut self, pane_id: u64, page: String, template: String, freeze: bool);
}

pub fn properties(ui: &mut Ui, host: &mut impl PropertiesHost) -> bool {
    dock_intro(
        ui,
        "RESULT DOCUMENT · PRESENTATION POLICY",
        "Edit the current worksheet's retained display properties.",
    );
    let (current_significant_digits, current_phase_continuous) = host.current();
    let significant_digits = host
        .significant_digits()
        .get_or_insert(current_significant_digits);
    ui.add(egui::Slider::new(significant_digits, 3..=17).text("Significant digits"));
    property_row(ui, "Engineering grid", "Renderer-managed major grid");
    property_row(ui, "Legend placement", "Inside plot · compact");
    let phase_continuous = host
        .phase_continuous()
        .get_or_insert(current_phase_continuous);
    ui.checkbox(phase_continuous, "Continuous (unwrapped) phase display");
    let save = Button::new("Save properties").accent().show(ui).clicked();
    if save {
        let significant_digits = *host.significant_digits();
        let phase_continuous = *host.phase_continuous();
        let significant_digits = significant_digits.unwrap_or(current_significant_digits);
        let phase_continuous = phase_continuous.unwrap_or(current_phase_continuous);
        host.save(significant_digits, phase_continuous);
    }
    save
}

pub fn reorder(ui: &mut Ui, active: Option<u64>, order: &mut [u64]) -> bool {
    dock_intro(
        ui,
        "RESULTS · WORKSHEET LAYOUT",
        "Move panes while preserving their stable identity, traces, and link groups.",
    );
    let index = active.and_then(|id| order.iter().position(|pane_id| *pane_id == id));
    property_row(
        ui,
        "Selected pane",
        &active.map_or_else(|| "none".to_owned(), |id| format!("Pane {id:02}")),
    );
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                index.is_some_and(|index| index > 0),
                egui::Button::new("Move before"),
            )
            .clicked()
            && let Some(index) = index
        {
            order.swap(index, index - 1);
        }
        if ui
            .add_enabled(
                index.is_some_and(|index| index + 1 < order.len()),
                egui::Button::new("Move after"),
            )
            .clicked()
            && let Some(index) = index
        {
            order.swap(index, index + 1);
        }
    });
    Button::new("Apply pane order").accent().show(ui).clicked()
}

pub fn links(ui: &mut Ui, host: &mut impl LinksHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · SYNCHRONIZED NAVIGATION",
        "Define which panes share X ranges and cursor positions.",
    );
    let active_pane = host.prepare();
    let Some(pane_id) = active_pane else {
        empty_note(ui, "Select a pane before editing link groups.");
        return Button::new("Close").show(ui).clicked();
    };
    ui.horizontal(|ui| {
        ui.label("X range group");
        ui.add(egui::DragValue::new(host.x_link()).range(0..=999));
    });
    let canonical = host.canonical();
    if canonical {
        property_row(
            ui,
            "Retained A/B links",
            if host.linked_cursors() {
                "linked"
            } else {
                "independent"
            },
        );
        let (link, unlink) = ui
            .horizontal_wrapped(|ui| {
                (
                    Button::new("Link matching A/B cursors").show(ui).clicked(),
                    Button::new("Unlink A/B cursors").show(ui).clicked(),
                )
            })
            .inner;
        if link {
            host.set_cursor_links(true);
        } else if unlink {
            host.set_cursor_links(false);
        }
    } else {
        ui.horizontal(|ui| {
            ui.label("Cursor group");
            ui.add(egui::DragValue::new(host.cursor_group()).range(0..=999));
        });
    }
    let apply = Button::new("Save link groups").accent().show(ui).clicked();
    if apply {
        host.save(pane_id);
    }
    apply
}

pub fn page(ui: &mut Ui, host: &mut impl PageHost) -> bool {
    dock_intro(
        ui,
        "REPORTING · DOCUMENT COMPOSITION",
        "Compose versioned pages from linked plots and immutable result evidence.",
    );
    let Some(pane_id) = host.prepare() else {
        empty_note(ui, "Select a pane before editing its page.");
        return Button::new("Close").show(ui).clicked();
    };
    let draft = host.draft();
    ui.label("Template");
    egui::ComboBox::from_id_salt("report.page.template")
        .selected_text(draft.template.clone())
        .show_ui(ui, |ui| {
            for template in REPORT_PAGE_TEMPLATES {
                ui.selectable_value(draft.template, template.to_owned(), template);
            }
        });
    ui.label("Page");
    ui.text_edit_singleline(draft.page);
    ui.label("Update policy");
    ui.radio_value(draft.freeze, false, "Refresh linked figures automatically");
    ui.radio_value(draft.freeze, true, "Freeze selected figure revision");
    let page = draft.page.trim().to_owned();
    let template = draft.template.clone();
    let freeze = *draft.freeze;
    let valid = !page.is_empty()
        && page.len() <= MAX_REPORT_PAGE_TITLE_BYTES
        && !page.chars().any(char::is_control)
        && REPORT_PAGE_TEMPLATES.contains(&template.as_str());
    let apply = ui
        .add_enabled(valid, egui::Button::new("Save report document"))
        .clicked();
    if apply {
        host.save(pane_id, page, template, freeze);
    }
    apply
}

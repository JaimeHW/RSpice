//! Create Result Document presentation with immediate host validation.

use egui::{Color32, RichText, Sense, Ui, UiBuilder, vec2};
use rspice_app_types::product::DatasetId;
use rspice_results::{
    document_creation::{ResultDocumentFamily, ResultDocumentLayout},
    viewer_catalog::{
        VIEWER_DOCUMENTS, ViewerCompatibility, ViewerDocumentDefinition, ViewerReleaseClass,
    },
};
use rspice_ui_kit::tokens::Tokens;
use std::borrow::Cow;

/// Runtime draft for the dataset-driven Create Result Document transaction.
///
/// IDs, rather than translated labels or row positions, cross the modal
/// boundary. The project-owned document is created only after the workflow
/// module revalidates every selection against current retained datasets.
#[derive(Debug, Clone)]
pub struct CreateResultDocumentDialogState {
    pub open: bool,
    pub name: String,
    pub name_touched: bool,
    pub dataset_id: Option<DatasetId>,
    pub family_id: String,
    pub viewer_id: String,
    pub layout_id: String,
    pub validation_error: Option<String>,
}

impl Default for CreateResultDocumentDialogState {
    fn default() -> Self {
        Self {
            open: false,
            name: String::new(),
            name_touched: false,
            dataset_id: None,
            family_id: "waveform-worksheet".to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: "two-linked-panes".to_owned(),
            validation_error: None,
        }
    }
}

/// Source-qualified catalog entry; renderer admission remains with the host.
pub struct ViewerChoice {
    pub viewer: &'static ViewerDocumentDefinition,
    pub compatibility: ViewerCompatibility,
    pub renderer_available: bool,
}

/// Read current sources and reconcile selections before rendering dependent controls.
pub trait CreateDocumentHost {
    fn validation_message(&self, draft: &CreateResultDocumentDialogState) -> Option<String>;
    fn can_create(&self, draft: &CreateResultDocumentDialogState) -> bool;
    fn select_family(
        &self,
        draft: &mut CreateResultDocumentDialogState,
        family: ResultDocumentFamily,
    );
    fn select_dataset(&self, draft: &mut CreateResultDocumentDialogState, dataset_id: DatasetId);
    fn dataset_label(&self, dataset_id: Option<DatasetId>) -> String;
    fn datasets(&self) -> impl Iterator<Item = (DatasetId, String)>;
    fn viewer_catalog(&self, dataset_id: Option<DatasetId>) -> impl Iterator<Item = ViewerChoice>;
}

/// Return a create request; the host revalidates and commits the current draft.
pub fn show(
    ctx: &egui::Context,
    draft: &mut CreateResultDocumentDialogState,
    host: &impl CreateDocumentHost,
) -> bool {
    if !draft.open {
        return false;
    }
    let mut window_open = true;
    let mut submit = false;
    let mut cancel = false;
    let t = Tokens::get(ctx);
    let validation_message = host.validation_message(draft);
    if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
        cancel = true;
    }
    let available = ctx.content_rect().size();
    let max_window_width = (available.x - 24.0).max(320.0);
    let max_window_height = (available.y - 24.0).max(320.0);

    egui::Window::new("New result document")
        .id(egui::Id::new("rspice.create-result-document"))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(true)
        .default_width(920.0_f32.min(max_window_width))
        .min_width(720.0_f32.min(max_window_width))
        .max_width(max_window_width)
        .min_height(620.0_f32.min(max_window_height))
        .max_height(max_window_height)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            let body_height = (max_window_height - 82.0).max(220.0);
            egui::ScrollArea::vertical()
                .id_salt("rspice.create-result-document.body")
                .max_height(body_height)
                .auto_shrink([false, false])
                .show(ui, |ui| {
            ui.label(
                RichText::new("RESULTS · DATASET-DRIVEN DOCUMENT")
                    .monospace()
                    .color(t.color.text_dim),
            );
            ui.add_space(4.0);
            ui.label(
                "Choose the engineering question first. RSpice selects a compatible viewer family from the immutable dataset and keeps specialist tools available inside the resulting document.",
            );
            ui.add_space(10.0);

            ui.label(RichText::new("Result document type").strong());
            for row in ResultDocumentFamily::ALL.chunks(2) {
                ui.columns(2, |columns| {
                    for (column, family) in row.iter().copied().enumerate() {
                        let selected = draft.family_id == family.id();
                        let response = family_card(&mut columns[column], family, selected);
                        if response.clicked() && !selected {
                            host.select_family(draft, family);
                        }
                    }
                });
                ui.add_space(6.0);
            }

            result_setting_row(
                ui,
                "Document name",
                "A project-owned name; duplicates and invalid byte lengths are rejected on commit.",
                |ui| {
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut draft.name)
                            .id_salt("rspice.create-result-document.name")
                            .desired_width(ui.available_width()),
                    );
                    if response.changed() {
                        draft.name_touched = true;
                        draft.validation_error = None;
                    }
                },
            );
            result_setting_row(
                ui,
                "Dataset",
                "Document types and viewers update with the selected immutable result.",
                |ui| {
                let selected_dataset_label = host.dataset_label(draft.dataset_id);
                egui::ComboBox::from_id_salt("rspice.create-result-document.dataset")
                    .selected_text(selected_dataset_label)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for (dataset_id, label) in host.datasets() {
                            let selected = draft.dataset_id == Some(dataset_id);
                            if ui
                                .selectable_label(selected, label)
                                .clicked()
                            {
                                host.select_dataset(draft, dataset_id);
                            }
                        }
                    });
                },
            );
            result_setting_row(
                ui,
                "Document layout",
                "The document stays editable without changing source samples.",
                |ui| {
                let layout = ResultDocumentLayout::from_id(&draft.layout_id)
                    .unwrap_or(ResultDocumentLayout::TwoLinkedPanes);
                egui::ComboBox::from_id_salt("rspice.create-result-document.layout")
                    .selected_text(layout.label())
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for candidate in ResultDocumentLayout::ALL {
                            ui.selectable_value(
                                &mut draft.layout_id,
                                candidate.id().to_owned(),
                                candidate.label(),
                            );
                        }
                    });
                },
            );

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Complete viewer catalog").strong());
                ui.separator();
                ui.label(
                    RichText::new(format!("{} canonical viewers", VIEWER_DOCUMENTS.len()))
                        .small()
                        .color(t.color.text_dim),
                );
            });

            let catalog = host.viewer_catalog(draft.dataset_id);
            egui::Grid::new("rspice.create-result-document.viewer-table")
                        .num_columns(3)
                        .striped(true)
                        .min_col_width(120.0)
                        .show(ui, |ui| {
                            ui.strong("Viewer / family");
                            ui.strong("Required result");
                            ui.strong("Status");
                            ui.end_row();

                            for ViewerChoice { viewer, compatibility, renderer_available } in catalog {
                                let family = ResultDocumentFamily::from_id(&draft.family_id)
                                    .unwrap_or(ResultDocumentFamily::WaveformWorksheet);
                                let belongs_to_family = family.includes(viewer);
                                let selectable = belongs_to_family
                                    && viewer.release == ViewerReleaseClass::ReleaseTarget
                                    && compatibility.is_compatible()
                                    && renderer_available;
                                let selected = draft.viewer_id == viewer.id;
                                ui.vertical(|ui| {
                                    let response = ui.add_enabled(
                                        selectable,
                                        egui::Button::selectable(selected, viewer.title),
                                    );
                                    if response.clicked() {
                                        draft.viewer_id = viewer.id.to_owned();
                                        draft.validation_error = None;
                                    }
                                    response.on_disabled_hover_text(if !belongs_to_family {
                                        "Choose a document family that includes this viewer."
                                            .to_owned()
                                    } else if viewer.release != ViewerReleaseClass::ReleaseTarget {
                                        viewer.unavailable_reason().into_owned()
                                    } else if !compatibility.is_compatible() {
                                        viewer_requirement(viewer)
                                    } else {
                                        "This viewer has no renderer for the selected dataset yet."
                                            .to_owned()
                                    });
                                    ui.label(
                                        RichText::new(viewer.group.label())
                                            .small()
                                            .color(t.color.text_dim),
                                    );
                                });
                                ui.label(viewer_requirement(viewer));
                                let (status, color) = if belongs_to_family {
                                    viewer_status(viewer, compatibility, renderer_available, &t)
                                } else {
                                    (Cow::Borrowed("other document family"), t.color.text_dim)
                                };
                                ui.label(RichText::new(status).color(color));
                                ui.end_row();
                            }
                        });

            ui.add_space(8.0);
            if let Some(message) = draft
                .validation_error
                .as_deref()
                .or(validation_message.as_deref())
            {
                ui.label(RichText::new(message).color(t.color.err));
            }
                });

            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                submit = ui
                    .add_enabled(
                        host.can_create(draft),
                        egui::Button::new("Create result document"),
                    )
                    .clicked();
                cancel = ui.button("Cancel").clicked();
            });
        });

    if !window_open || cancel {
        draft.open = false;
        false
    } else {
        submit
    }
}

fn family_card(ui: &mut Ui, family: ResultDocumentFamily, selected: bool) -> egui::Response {
    use rspice_ui_kit::theme::mix;

    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 52.0), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::RadioButton,
            ui.is_enabled(),
            selected,
            family.label(),
        )
    });
    if ui.is_rect_visible(rect) {
        let fill = if selected {
            t.color.accent_dim
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_panel
        };
        let border = if selected {
            t.color.accent
        } else {
            t.color.border
        };
        ui.painter().rect_filled(rect, 2.0, fill);
        ui.painter().rect_stroke(
            rect,
            2.0,
            egui::Stroke::new(1.0, border),
            egui::StrokeKind::Inside,
        );
        if selected {
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    rect.left_top(),
                    egui::pos2(rect.left() + 3.0, rect.bottom()),
                ),
                1.0,
                t.color.accent,
            );
        }
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(rect.shrink2(vec2(12.0, 7.0)))
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                // Painted, not labels: a label over the card takes the presses on its text.
                rspice_ui_kit::panels::painted_label(
                    ui,
                    RichText::new(family.label()).strong().color(if selected {
                        t.color.text
                    } else {
                        mix(t.color.text, t.color.text_dim, 0.15)
                    }),
                    egui::TextWrapMode::Wrap,
                );
                rspice_ui_kit::panels::painted_label(
                    ui,
                    RichText::new(family.description())
                        .small()
                        .color(t.color.text_dim),
                    egui::TextWrapMode::Wrap,
                );
            },
        );
    }
    rspice_ui_kit::theme::paint_focus_ring(ui, &response, rect);
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(family.description())
}

fn result_setting_row(ui: &mut Ui, title: &str, detail: &str, value: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let label_width = (width * 0.46).clamp(250.0, 390.0);
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            vec2(label_width, 48.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(RichText::new(title).strong());
                ui.label(RichText::new(detail).small().color(t.color.text_dim));
            },
        );
        ui.add_space(12.0);
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 48.0),
            egui::Layout::top_down_justified(egui::Align::Center),
            value,
        );
    });
}

fn viewer_status(
    viewer: &ViewerDocumentDefinition,
    compatibility: ViewerCompatibility,
    renderer_available: bool,
    tokens: &Tokens,
) -> (Cow<'static, str>, Color32) {
    if viewer.release != ViewerReleaseClass::ReleaseTarget {
        return (viewer.unavailable_reason(), tokens.color.warn);
    }
    if compatibility.is_compatible() && !renderer_available {
        return (
            Cow::Borrowed("viewer integration required"),
            tokens.color.warn,
        );
    }
    match compatibility {
        ViewerCompatibility::Compatible => (Cow::Borrowed("available"), tokens.color.ok),
        ViewerCompatibility::MissingAnalysis { .. } => {
            (Cow::Borrowed("analysis required"), tokens.color.warn)
        }
        ViewerCompatibility::MissingExternalCapability { .. } => (
            Cow::Borrowed("specialist dataset required"),
            tokens.color.warn,
        ),
        ViewerCompatibility::UnknownDocument => (Cow::Borrowed("unregistered"), tokens.color.err),
    }
}

fn viewer_requirement(viewer: &ViewerDocumentDefinition) -> String {
    if let Some(capability) = viewer.external_capability {
        format!("{capability} result contract")
    } else if viewer.analysis_ids.is_empty() {
        "any completed dataset".to_owned()
    } else {
        viewer.analysis_ids.join(" or ").to_ascii_uppercase()
    }
}

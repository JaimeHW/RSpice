//! Report composition, outline and page-setting controls over app-owned authority.
use super::{
    PaneSeparators,
    inspector::{self, InspectorHost},
    page_marker, paint_pane_separators,
    preview::{self, PreviewHost},
};
use egui::{Align, Align2, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};
use rspice_results::report_document::{
    ReportBlockedGateTextPolicy, ReportDocument, ReportPage, ReportPageEvidenceBinding,
    ReportPageId, ReportPageInclusion,
};
use rspice_ui_kit::{
    panels::{
        WorkbenchIcon, code_inspector_section, code_workspace_heading, icon_button,
        workspace_title_row,
    },
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, select},
};

pub trait ComposerHost: PreviewHost + InspectorHost {
    fn project_open(&self) -> bool;
    fn synchronize_selection(&mut self);
    fn document_snapshot(&self) -> Option<ReportDocument>;
    fn selected_page(&self, document: &ReportDocument) -> Option<ReportPageId>;
    fn create_document(&mut self);
    fn add_page(&mut self);
    fn page_properties(&mut self);
    fn can_move_page(&self, direction: PageMoveDirection) -> bool;
    fn move_page(&mut self, direction: PageMoveDirection);
    fn select_page(&mut self, page: ReportPageId);
    fn prepare_page_settings(&mut self, page: &ReportPage);
    fn title_draft(&mut self) -> &mut String;
    fn commit_page_setting(&mut self, page: ReportPageId, setting: PageSettingEdit);
    fn evidence_options(
        &self,
        binding: ReportPageEvidenceBinding,
    ) -> Vec<(String, ReportPageEvidenceBinding)>;
    fn evidence_label(&self, binding: ReportPageEvidenceBinding) -> String;
}

const DESKTOP_BREAKPOINT: f32 = 1_020.0;
const STACK_BREAKPOINT: f32 = 820.0;
const OUTLINE_DESKTOP_WIDTH: f32 = 250.0;
const OUTLINE_TABLET_WIDTH: f32 = 180.0;
const INSPECTOR_WIDTH: f32 = 300.0;
const PANEL_GAP: f32 = 0.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;
const OUTLINE_HEADER_HEIGHT: f32 = 39.0;
const OUTLINE_ROW_HEIGHT: f32 = 34.0;
const PREVIEW_MIN_HEIGHT: f32 = 420.0;
const INSPECTOR_PUBLICATION_CONTENT_HEIGHT: f32 = 820.0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComposerLayout {
    ThreeColumn,
    TwoColumnInspectorBelow,
    Stacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageMoveDirection {
    Earlier,
    Later,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageSettingEdit {
    Title(String),
    Inclusion(ReportPageInclusion),
    EvidenceBinding(ReportPageEvidenceBinding),
    BlockedGateText(ReportBlockedGateTextPolicy),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ComposerPaneHeights {
    outline: f32,
    preview: f32,
    inspector: f32,
}

impl ComposerLayout {
    fn resolve(width: f32) -> Self {
        if width > DESKTOP_BREAKPOINT {
            Self::ThreeColumn
        } else if width > STACK_BREAKPOINT {
            Self::TwoColumnInspectorBelow
        } else {
            Self::Stacked
        }
    }

    fn separators(self) -> [PaneSeparators; 3] {
        match self {
            Self::ThreeColumn => [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ],
            Self::TwoColumnInspectorBelow => [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
                PaneSeparators {
                    top: true,
                    ..PaneSeparators::default()
                },
            ],
            Self::Stacked => [
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ],
        }
    }
}

fn composer_pane_heights(
    layout: ComposerLayout,
    available_height: f32,
    page_count: usize,
) -> ComposerPaneHeights {
    let viewport_height = if available_height.is_finite() {
        available_height.max(1.0)
    } else {
        PREVIEW_MIN_HEIGHT
    };
    let outline_content = OUTLINE_HEADER_HEIGHT + OUTLINE_ROW_HEIGHT * page_count as f32;
    let inspector_content = INSPECTOR_PUBLICATION_CONTENT_HEIGHT;

    match layout {
        ComposerLayout::ThreeColumn => ComposerPaneHeights {
            outline: viewport_height,
            preview: viewport_height,
            inspector: viewport_height,
        },
        ComposerLayout::TwoColumnInspectorBelow => {
            let top_content = outline_content.max(PREVIEW_MIN_HEIGHT);
            let base_height = top_content + inspector_content;
            let surplus = (viewport_height - base_height).max(0.0);
            ComposerPaneHeights {
                outline: top_content + surplus * 0.65,
                preview: top_content + surplus * 0.65,
                inspector: inspector_content + surplus * 0.35,
            }
        }
        ComposerLayout::Stacked => {
            let base_height = outline_content + PREVIEW_MIN_HEIGHT + inspector_content;
            let surplus = (viewport_height - base_height).max(0.0);
            ComposerPaneHeights {
                outline: outline_content,
                preview: PREVIEW_MIN_HEIGHT + surplus * 0.70,
                inspector: inspector_content + surplus * 0.30,
            }
        }
    }
}

pub fn show(ui: &mut Ui, host: &mut impl ComposerHost) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        ui.set_width(ui.available_width());

        if !host.project_open() {
            workspace_title_row(ui, |ui| {
                code_workspace_heading(
                    ui,
                    "REPORT AUTHORING · NO OPEN PROJECT",
                    "Engineering report composer",
                    "Open a project before creating a project-owned report document.",
                );
            });
            return;
        }

        host.synchronize_selection();

        report_title_row(ui, host);

        let Some(document) = host.document_snapshot() else {
            empty_report_workspace(ui, host);
            return;
        };
        let selected_page = host.selected_page(&document);
        let available = ui.available_size();
        let layout = ComposerLayout::resolve(available.x);
        let heights = composer_pane_heights(layout, available.y, document.pages().len());
        let [outline_separators, preview_separators, inspector_separators] = layout.separators();
        match layout {
            ComposerLayout::ThreeColumn => {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = PANEL_GAP;
                    ui.allocate_ui_with_layout(
                        Vec2::new(OUTLINE_DESKTOP_WIDTH, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| outline(ui, host, &document, selected_page, outline_separators),
                    );
                    let preview_width =
                        (available.x - OUTLINE_DESKTOP_WIDTH - INSPECTOR_WIDTH - PANEL_GAP * 2.0)
                            .max(1.0);
                    ui.allocate_ui_with_layout(
                        Vec2::new(preview_width, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| preview::show(ui, host, &document, selected_page, preview_separators),
                    );
                    ui.allocate_ui_with_layout(
                        Vec2::new(INSPECTOR_WIDTH, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| inspector::show(ui, host, &document, inspector_separators),
                    );
                });
            }
            ComposerLayout::TwoColumnInspectorBelow => {
                ScrollArea::vertical()
                    .id_salt("report-authoring.tablet")
                    .show(ui, |ui| {
                        let local_width = ui.available_width().max(1.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = PANEL_GAP;
                            ui.allocate_ui_with_layout(
                                Vec2::new(OUTLINE_TABLET_WIDTH, heights.outline),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    outline(ui, host, &document, selected_page, outline_separators)
                                },
                            );
                            ui.allocate_ui_with_layout(
                                Vec2::new(
                                    (local_width - OUTLINE_TABLET_WIDTH).max(1.0),
                                    heights.preview,
                                ),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    preview::show(
                                        ui,
                                        host,
                                        &document,
                                        selected_page,
                                        preview_separators,
                                    )
                                },
                            );
                        });
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.inspector),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| inspector::show(ui, host, &document, inspector_separators),
                        );
                    });
            }
            ComposerLayout::Stacked => {
                ScrollArea::vertical()
                    .id_salt("report-authoring.compact")
                    .show(ui, |ui| {
                        let local_width = ui.available_width().max(1.0);
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.outline),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| outline(ui, host, &document, selected_page, outline_separators),
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.preview),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                preview::show(
                                    ui,
                                    host,
                                    &document,
                                    selected_page,
                                    preview_separators,
                                )
                            },
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.inspector),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| inspector::show(ui, host, &document, inspector_separators),
                        );
                    });
            }
        }
    });
}

fn report_title_row(ui: &mut Ui, host: &mut impl ComposerHost) {
    workspace_title_row(ui, |ui| {
        if ui.available_width() <= TITLE_ACTION_STACK_BREAKPOINT {
            report_title_heading(ui);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let width = ((ui.available_width() - 6.0) * 0.5).max(1.0);
                report_release_button(ui, host, width);
                report_plan_button(ui, host, width);
            });
        } else {
            let release_width = report_title_button_width(ui, "Release closure", false);
            let plan_width = report_title_button_width(ui, "Plan report artifact…", true);
            let actions_width = release_width + 6.0 + plan_width;
            let heading_width = (ui.available_width() - actions_width - 12.0).max(1.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.allocate_ui_with_layout(
                    Vec2::new(heading_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_width(heading_width);
                        report_title_heading(ui);
                    },
                );
                ui.add_space(6.0);
                report_release_button(ui, host, 0.0);
                report_plan_button(ui, host, 0.0);
            });
        }
    });
}

fn report_title_heading(ui: &mut Ui) {
    code_workspace_heading(
        ui,
        "REPORT AUTHORING · ENGINEERING DRAFT",
        "Engineering report composer",
        "Author traceable pages, locked run references, publication plots and machine-readable appendices. Release closure alone owns package assembly and promotion.",
    );
}

fn report_release_button(ui: &mut Ui, host: &mut impl ComposerHost, width: f32) {
    let mut button = Button::new("Release closure");
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    if button.show(ui).clicked() {
        host.open_release();
    }
}

fn report_plan_button(ui: &mut Ui, host: &mut impl ComposerHost, width: f32) {
    let writable = PreviewHost::writable(host);
    let mut button = Button::new("Plan report artifact…")
        .accent()
        .enabled(writable);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    let response = button
        .show(ui)
        .on_disabled_hover_text(PreviewHost::blocked_reason(host));
    if response.clicked() {
        host.create_document();
    }
}

fn report_title_button_width(ui: &Ui, label: &str, accent: bool) -> f32 {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(
        tokens::FS_0,
        if accent {
            FontWeight::SemiBold
        } else {
            FontWeight::Regular
        },
    );
    let content = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, t.color.text)
        .size()
        .x
        + 20.0;
    content.max(if t.metrics.ctl_h >= 44.0 { 44.0 } else { 0.0 })
}

fn empty_report_workspace(ui: &mut Ui, host: &mut impl ComposerHost) {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_size();
    let writable = PreviewHost::writable(host);
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .show(ui, |ui| {
            ui.set_min_size(available.max(Vec2::new(1.0, 1.0)));
            ui.add_space((available.y * 0.24).clamp(36.0, 180.0));
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("No report document")
                        .font(theme::sans(18.0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.add_space(7.0);
                ui.label(
                    egui::RichText::new(
                        "Plan an explicit project-owned report artifact before authoring pages and evidence.",
                    )
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
                ui.add_space(16.0);
                if Button::new("Plan report artifact...")
                    .accent()
                    .enabled(writable)
                    .show(ui)
                    .clicked()
                {
                    host.create_document();
                }
                if !writable {
                    ui.add_space(8.0);
                    ui.colored_label(
                        t.color.err,
                        PreviewHost::blocked_reason(host),
                    );
                }
            });
        });
}

fn outline(
    ui: &mut Ui,
    host: &mut impl ComposerHost,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
        let (head, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 39.0), Sense::hover());
        ui.painter().hline(
            head.x_range(),
            head.bottom(),
            Stroke::new(1.0, t.color.border),
        );
        ui.painter().text(
            head.left_center() + Vec2::new(10.0, 0.0),
            Align2::LEFT_CENTER,
            "Report outline",
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
        const CONTROL_SIZE: f32 = 29.0;
        const CONTROL_GAP: f32 = 2.0;
        const CONTROL_COUNT: f32 = 4.0;
        let controls_width = CONTROL_SIZE * CONTROL_COUNT + CONTROL_GAP * (CONTROL_COUNT - 1.0);
        let controls_rect = Rect::from_center_size(
            head.right_center() - Vec2::new(5.0 + controls_width * 0.5, 0.0),
            Vec2::new(controls_width, CONTROL_SIZE),
        );
        let mut controls = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(controls_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        controls.spacing_mut().item_spacing.x = CONTROL_GAP;
        let writable = PreviewHost::writable(host);
        let add_response = controls
            .add_enabled_ui(writable, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::Add,
                    "Add report page",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(PreviewHost::blocked_reason(host));
        if add_response.clicked() {
            host.add_page();
        }

        let move_earlier_enabled = host.can_move_page(PageMoveDirection::Earlier);
        let move_earlier_response = controls
            .add_enabled_ui(move_earlier_enabled, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::ArrowLeft,
                    "Move page earlier",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(if writable {
                "The selected page is already first."
            } else {
                PreviewHost::blocked_reason(host)
            });
        if move_earlier_response.clicked() {
            host.move_page(PageMoveDirection::Earlier);
        }

        let move_later_enabled = host.can_move_page(PageMoveDirection::Later);
        let move_later_response = controls
            .add_enabled_ui(move_later_enabled, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::ArrowRight,
                    "Move page later",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(if writable {
                "The selected page is already last."
            } else {
                PreviewHost::blocked_reason(host)
            });
        if move_later_response.clicked() {
            host.move_page(PageMoveDirection::Later);
        }

        let properties_enabled = writable && selected_page.is_some();
        let properties_response = controls
            .add_enabled_ui(properties_enabled, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::Sliders,
                    "Page properties",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(PreviewHost::blocked_reason(host));
        if properties_response.clicked() {
            host.page_properties();
        }

        ScrollArea::vertical()
            .id_salt("report-authoring.outline")
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for (index, page) in document.pages().iter().enumerate() {
                    let marker = page_marker(index, page.title());
                    let selected = Some(page.id()) == selected_page;
                    if outline_row(ui, marker, page.title(), selected).clicked() {
                        host.select_page(page.id());
                    }
                }
                page_settings(ui, host, document, selected_page);
            });
    });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn outline_row(ui: &mut Ui, marker: &str, label: &str, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), selected, label)
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            Rect::from_min_max(rect.left_top(), rect.left_bottom() + Vec2::new(2.0, 0.0)),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().text(
        rect.left_center() + Vec2::new(10.0, 0.0),
        Align2::LEFT_CENTER,
        marker,
        theme::mono(tokens::FS_1, FontWeight::Medium),
        if selected {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        rect.left_center() + Vec2::new(37.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn page_settings(
    ui: &mut Ui,
    host: &mut impl ComposerHost,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
) {
    let Some(page) = selected_page.and_then(|page_id| document.page(page_id)) else {
        return;
    };
    let page_id = page.id();
    host.prepare_page_settings(page);
    let t = Tokens::get(ui.ctx());
    let page_marker = document
        .pages()
        .iter()
        .position(|candidate| candidate.id() == page_id)
        .map(|index| page_marker(index, page.title()))
        .unwrap_or("+");
    let section_title = format!("Page settings · {page_marker}");
    code_inspector_section(ui, &section_title, None, |ui| {
        egui::Frame::new()
            .inner_margin(egui::Margin {
                left: 10,
                right: 10,
                top: 8,
                bottom: 10,
            })
            .show(ui, |ui| {
                ui.set_width(ui.available_width().max(1.0));
                report_form_label(ui, "Page title");
                let title_response = ui.add_sized(
                    Vec2::new(ui.available_width(), t.metrics.ctl_h),
                    egui::TextEdit::singleline(host.title_draft())
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular)),
                );
                if title_response.has_focus()
                    && ui.input(|input| input.key_pressed(egui::Key::Escape))
                {
                    *host.title_draft() = page.title().to_owned();
                    title_response.surrender_focus();
                } else if title_response.lost_focus() {
                    let title = host.title_draft().trim().to_owned();
                    *host.title_draft() = title.clone();
                    host.commit_page_setting(page_id, PageSettingEdit::Title(title));
                }

                ui.add_space(8.0);
                report_form_label(ui, "Include in artifact");
                const INCLUSION_OPTIONS: [(ReportPageInclusion, &str); 3] = [
                    (ReportPageInclusion::Included, "Included"),
                    (
                        ReportPageInclusion::ExcludedFromDraft,
                        "Excluded from draft",
                    ),
                    (ReportPageInclusion::AppendixOnly, "Appendix only"),
                ];
                let inclusion_labels = INCLUSION_OPTIONS
                    .iter()
                    .map(|(_, label)| (*label).to_owned())
                    .collect::<Vec<_>>();
                let inclusion_current = report_page_inclusion_label(page.inclusion());
                if let Some(index) = select(
                    ui,
                    "report-page-inclusion",
                    "Include report page in artifact",
                    inclusion_current,
                    &inclusion_labels,
                    ui.available_width(),
                ) && let Some((inclusion, _)) = INCLUSION_OPTIONS.get(index)
                {
                    host.commit_page_setting(page_id, PageSettingEdit::Inclusion(*inclusion));
                }

                ui.add_space(8.0);
                report_form_label(ui, "Evidence binding");
                let evidence_options = host.evidence_options(page.evidence_binding());
                let evidence_labels = evidence_options
                    .iter()
                    .map(|(label, _)| label.clone())
                    .collect::<Vec<_>>();
                let evidence_current = host.evidence_label(page.evidence_binding());
                if let Some(index) = select(
                    ui,
                    "report-page-evidence-binding",
                    "Report page evidence binding",
                    &evidence_current,
                    &evidence_labels,
                    ui.available_width(),
                ) && let Some((_, evidence_binding)) = evidence_options.get(index)
                {
                    host.commit_page_setting(
                        page_id,
                        PageSettingEdit::EvidenceBinding(*evidence_binding),
                    );
                }

                ui.add_space(8.0);
                report_form_label(ui, "Blocked-gate text");
                const GATE_TEXT_OPTIONS: [(ReportBlockedGateTextPolicy, &str); 2] = [
                    (
                        ReportBlockedGateTextPolicy::VerbatimFromSource,
                        "State verbatim from source",
                    ),
                    (
                        ReportBlockedGateTextPolicy::SummarizeWithLink,
                        "Summarize with link",
                    ),
                ];
                let gate_text_labels = GATE_TEXT_OPTIONS
                    .iter()
                    .map(|(_, label)| (*label).to_owned())
                    .collect::<Vec<_>>();
                let gate_text_current =
                    report_blocked_gate_text_policy_label(page.blocked_gate_text_policy());
                if let Some(index) = select(
                    ui,
                    "report-page-gate-text",
                    "Blocked-gate text policy",
                    gate_text_current,
                    &gate_text_labels,
                    ui.available_width(),
                ) && let Some((policy, _)) = GATE_TEXT_OPTIONS.get(index)
                {
                    host.commit_page_setting(page_id, PageSettingEdit::BlockedGateText(*policy));
                }

                if let Some(error) = host.transaction_error() {
                    ui.add_space(8.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(error)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.err),
                        )
                        .wrap(),
                    );
                }
            });
    });
}

fn report_form_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(4.0);
}

fn report_page_inclusion_label(inclusion: ReportPageInclusion) -> &'static str {
    match inclusion {
        ReportPageInclusion::Included => "Included",
        ReportPageInclusion::ExcludedFromDraft => "Excluded from draft",
        ReportPageInclusion::AppendixOnly => "Appendix only",
    }
}

fn report_blocked_gate_text_policy_label(policy: ReportBlockedGateTextPolicy) -> &'static str {
    match policy {
        ReportBlockedGateTextPolicy::VerbatimFromSource => "State verbatim from source",
        ReportBlockedGateTextPolicy::SummarizeWithLink => "Summarize with link",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::INITIAL_PAGES;
    #[test]
    fn responsive_report_builder_matches_mockup_breakpoints() {
        assert_eq!(OUTLINE_DESKTOP_WIDTH, 250.0);
        assert_eq!(OUTLINE_TABLET_WIDTH, 180.0);
        assert_eq!(INSPECTOR_WIDTH, 300.0);
        assert_eq!(
            ComposerLayout::resolve(1_280.0),
            ComposerLayout::ThreeColumn
        );
        assert_eq!(
            ComposerLayout::resolve(1_020.0),
            ComposerLayout::TwoColumnInspectorBelow
        );
        assert_eq!(
            ComposerLayout::resolve(821.0),
            ComposerLayout::TwoColumnInspectorBelow
        );
        assert_eq!(ComposerLayout::resolve(820.0), ComposerLayout::Stacked);
        assert_eq!(ComposerLayout::resolve(390.0), ComposerLayout::Stacked);
    }

    #[test]
    fn every_report_layout_assigns_each_internal_seam_to_one_pane() {
        assert_eq!(
            ComposerLayout::ThreeColumn.separators(),
            [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ]
        );
        assert_eq!(
            ComposerLayout::TwoColumnInspectorBelow.separators(),
            [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
                PaneSeparators {
                    top: true,
                    ..PaneSeparators::default()
                },
            ]
        );
        assert_eq!(
            ComposerLayout::Stacked.separators(),
            [
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ]
        );
    }

    #[test]
    fn tablet_and_stacked_pane_heights_follow_local_space_and_document_content() {
        let tablet_short = composer_pane_heights(
            ComposerLayout::TwoColumnInspectorBelow,
            640.0,
            INITIAL_PAGES.len(),
        );
        let tablet_tall = composer_pane_heights(
            ComposerLayout::TwoColumnInspectorBelow,
            1_600.0,
            INITIAL_PAGES.len(),
        );
        assert!(tablet_tall.preview > tablet_short.preview);
        assert!(tablet_tall.inspector > tablet_short.inspector);
        assert!(tablet_short.preview + tablet_short.inspector + 0.01 >= 640.0);
        assert!(tablet_tall.preview + tablet_tall.inspector + 0.01 >= 1_600.0);

        let compact_seven =
            composer_pane_heights(ComposerLayout::Stacked, 720.0, INITIAL_PAGES.len());
        let compact_twelve = composer_pane_heights(ComposerLayout::Stacked, 720.0, 12);
        assert!(compact_twelve.outline > compact_seven.outline);
        assert_eq!(compact_seven.preview, PREVIEW_MIN_HEIGHT);
        assert!(compact_seven.inspector > 300.0);
    }
}

//! The settings column of the Print and Export dialogs: the choices that make
//! the page, in five groups of label-left rows beside the page they lay out.
//!
//! The column holds the same rows, at the same places, whatever is chosen. A
//! row that only means something some of the time (a page range where there
//! is more than one page, the tiling of a drawing too large for one page, the
//! four margins of a custom page) keeps its place while it is hidden, so
//! nothing moves under the pointer when it shows. None of these rows
//! refreshes the preview itself: the body compares the whole draft before and
//! after the pass.

use super::*;

/// The settings column's width: the label column, and a control column wide
/// enough for a paper name and the button beside a printer.
pub(super) const SETTINGS_WIDTH: f32 = 420.0;
const LABEL_WIDTH: f32 = 88.0;
const LABEL_GAP: f32 = 8.0;
const ROW_GAP: f32 = 4.0;
const GROUP_GAP: f32 = 8.0;
const GROUP_CAPTION_HEIGHT: f32 = 18.0;
const INSET_X: i8 = 16;
const INSET_Y: i8 = 10;
/// One line of a set of checks.
const CHECK_LINE_HEIGHT: f32 = 22.0;
/// The space between a check's label and the next column's box.
const CHECK_COLUMN_GAP: f32 = 16.0;
/// A whole-number field: four digits and its drag affordance.
const NUMBER_WIDTH: f32 = 64.0;

/// Margins a little inside the default ones, for a drawing that wants the
/// paper. Most office printers print to within a quarter inch of the edge.
const NARROW_MARGIN_UM: u64 = 6_350;

/// The settings column, exactly [`SETTINGS_WIDTH`] wide and `height` tall
/// whatever its rows ask for, so nothing in it can widen the dialog.
pub(super) fn settings_column(
    ui: &mut Ui,
    draft: &mut HardcopyDialogState,
    height: f32,
) -> (Rect, BodyAction) {
    let t = Tokens::get(ui.ctx());
    let mut action = BodyAction::None;
    let (rect, _) = ui.allocate_exact_size(vec2(SETTINGS_WIDTH, height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    let mut column = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("hardcopy-settings-column")
            .max_rect(rect)
            .layout(Layout::top_down(Align::Min)),
    );
    column.set_clip_rect(rect.intersect(ui.clip_rect()));
    // The rows are authored to fit the column. The scroll area is the
    // fallback for a hostile font scale, not the layout.
    egui::ScrollArea::vertical()
        .id_salt("hardcopy-settings")
        .auto_shrink([false, false])
        .max_height(height)
        .show(&mut column, |ui| {
            Frame::NONE
                .inner_margin(Margin::symmetric(INSET_X, INSET_Y))
                .show(ui, |ui| {
                    ui.set_width(SETTINGS_WIDTH - 2.0 * f32::from(INSET_X));
                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                    let mut rows = Rows::new(ui);
                    action = settings_rows(&mut rows, draft);
                });
        });
    (rect, action)
}

fn settings_rows(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) -> BodyAction {
    let mut action = BodyAction::None;
    let total_pages = draft
        .preview_plan
        .as_ref()
        .map_or(1, |plan| plan.pagination().total_page_count());
    clamp_page_range(draft, total_pages);
    if draft.workflow == HardcopyWorkflow::Export {
        rows.group("Format");
        format_row(rows, draft);
        resolution_row(rows, draft);
    } else {
        rows.group("Printer");
        printer_row(rows, draft, &mut action);
        copies_row(rows, draft);
    }
    page_range_row(rows, draft, total_pages);

    rows.group("Paper");
    paper_row(rows, draft, &mut action);
    orientation_row(rows, draft);
    margins_rows(rows, draft);

    rows.group("Layout");
    scale_row(rows, draft);
    let tiled = total_pages > 1 || draft.tiling != TilingMode::Automatic;
    tiling_rows(rows, draft, tiled);

    rows.group("Content");
    content_rows(rows, draft, &mut action);

    rows.group("Output");
    color_row(rows, draft, &mut action);
    background_row(rows, draft);
    action
}

/// Label-left rows on one label column, grouped under captions.
struct Rows<'u> {
    ui: &'u mut Ui,
    started: bool,
}

impl<'u> Rows<'u> {
    fn new(ui: &'u mut Ui) -> Self {
        Self { ui, started: false }
    }

    /// A caption over a hairline, opening the group of rows after it.
    fn group(&mut self, title: &str) {
        if self.started {
            self.ui.add_space(GROUP_GAP);
        }
        self.started = true;
        let t = Tokens::get(self.ui.ctx());
        let width = self.ui.available_width();
        let (_, rect) = self.ui.allocate_space(vec2(width, GROUP_CAPTION_HEIGHT));
        let response = self.ui.interact(
            rect,
            self.ui.id().with(("hardcopy-settings-group", title)),
            Sense::hover(),
        );
        self.ui.painter().text(
            egui::pos2(rect.left(), rect.top()),
            egui::Align2::LEFT_TOP,
            title,
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        );
        self.ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border),
        );
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, title));
    }

    /// One row: its label, then its control in the rest of the width.
    fn row<R>(&mut self, label: &str, control: impl FnOnce(&mut Ui, f32) -> R) -> R {
        self.lines(label, Tokens::get(self.ui.ctx()).metrics.ctl_h, control)
    }

    /// A row `height` tall, its label level with its first line.
    ///
    /// Every row is its own id scope named by its label, so a row that shows
    /// or hides above it never renames the widgets in it.
    fn lines<R>(&mut self, label: &str, height: f32, control: impl FnOnce(&mut Ui, f32) -> R) -> R {
        self.ui.add_space(ROW_GAP);
        let t = Tokens::get(self.ui.ctx());
        let width = self.ui.available_width();
        let first_line = t.metrics.ctl_h.min(height);
        // One line centres its controls on the label; several hang from it.
        let cross = if height > t.metrics.ctl_h {
            Align::Min
        } else {
            Align::Center
        };
        let id = self.ui.id().with(("hardcopy-settings-row", label));
        self.ui
            .scope_builder(egui::UiBuilder::new().id(id), |ui| {
                ui.allocate_ui_with_layout(
                    vec2(width, height),
                    Layout::left_to_right(Align::Min),
                    |ui| {
                        ui.set_min_height(height);
                        ui.spacing_mut().item_spacing.x = LABEL_GAP;
                        let (label_rect, _) =
                            ui.allocate_exact_size(vec2(LABEL_WIDTH, first_line), Sense::hover());
                        ui.painter().text(
                            egui::pos2(label_rect.left(), label_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            label,
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            t.color.text_dim,
                        );
                        let control_width = ui.available_width();
                        ui.allocate_ui_with_layout(
                            vec2(control_width, height),
                            Layout::left_to_right(cross),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = LABEL_GAP;
                                ui.set_min_height(first_line);
                                control(ui, control_width)
                            },
                        )
                        .inner
                    },
                )
                .inner
            })
            .inner
    }

    /// A row that holds its place whether or not it is shown.
    fn held<R>(
        &mut self,
        shown: bool,
        label: &str,
        control: impl FnOnce(&mut Ui, f32) -> R,
    ) -> Option<R> {
        if shown {
            return Some(self.row(label, control));
        }
        let height = Tokens::get(self.ui.ctx()).metrics.ctl_h;
        self.ui.add_space(ROW_GAP + height);
        None
    }
}

/// A control of the row's height, in the row's control column.
fn sized<R>(ui: &mut Ui, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let height = Tokens::get(ui.ctx()).metrics.ctl_h;
    ui.allocate_ui_with_layout(
        vec2(width, height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_width(width);
            add(ui)
        },
    )
    .inner
}

fn faint(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        )
        .truncate(),
    );
}

const fn unit_label(unit: LengthUnit) -> &'static str {
    match unit {
        LengthUnit::Inches => "in",
        LengthUnit::Millimetres => "mm",
    }
}

/// A short typed number with its unit inside the field's end, and a faint
/// lead-in letter inside its start where the row holds several.
fn number_field(
    ui: &mut Ui,
    id: egui::Id,
    value: &mut String,
    width: f32,
    (lead, unit): (&str, &str),
    name: &str,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::field_mono();
    let measure = |ui: &mut Ui, text: &str| {
        if text.is_empty() {
            0.0
        } else {
            ui.fonts_mut(|fonts| {
                fonts
                    .layout_no_wrap(text.to_owned(), font.clone(), t.color.text_dim)
                    .size()
                    .x
            }) + 4.0
        }
    };
    let lead_width = measure(ui, lead);
    let unit_width = measure(ui, unit);
    let response = ui.add_sized(
        vec2(width, t.metrics.ctl_h),
        egui::TextEdit::singleline(value)
            .id(id)
            .font(font.clone())
            .text_color(t.color.text)
            .vertical_align(Align::Center)
            .margin(Margin {
                left: (6.0 + lead_width).ceil() as i8,
                right: (6.0 + unit_width).ceil() as i8,
                top: 4,
                bottom: 4,
            }),
    );
    if !lead.is_empty() {
        ui.painter().text(
            egui::pos2(response.rect.left() + 6.0, response.rect.center().y),
            egui::Align2::LEFT_CENTER,
            lead,
            font.clone(),
            t.color.text_faint,
        );
    }
    if !unit.is_empty() {
        ui.painter().text(
            egui::pos2(response.rect.right() - 6.0, response.rect.center().y),
            egui::Align2::RIGHT_CENTER,
            unit,
            font,
            t.color.text_dim,
        );
    }
    crate::ui::widgets::name_control(ui, &response, name);
    response.on_hover_text(name)
}

fn whole_number(
    ui: &mut Ui,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
    name: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let response = ui
        .scope(|ui| {
            ui.visuals_mut().override_text_color = Some(t.color.text);
            ui.add_sized(
                vec2(NUMBER_WIDTH, t.metrics.ctl_h),
                egui::DragValue::new(value).range(range).speed(0.1),
            )
        })
        .inner;
    crate::ui::widgets::name_control(ui, &response, name);
    response.changed()
}

// ---- Printer, or the format an export is written in -------------------------

/// What a print goes to: a system printer, found by discovery, or a file.
#[derive(Clone, PartialEq)]
enum Destination {
    Printer(String),
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    RuntimePrint,
    File(OutputFormat),
}

fn printer_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState, action: &mut BodyAction) {
    let selected_text = match draft.format {
        OutputFormat::NativePrinter => draft
            .printer_capabilities
            .as_ref()
            .map(|printer| printer.display_name().to_owned())
            .unwrap_or_else(|| {
                if draft.printer_discovery_busy {
                    "Looking for printers…".to_owned()
                } else {
                    "No printer".to_owned()
                }
            }),
        OutputFormat::BrowserPrintDocument => runtime_print_label().to_owned(),
        OutputFormat::PdfVector => "PDF file".to_owned(),
        OutputFormat::SvgVector => "SVG file".to_owned(),
        format => format_label(format).to_owned(),
    };
    let mut picked = None;
    let properties = rows.row("Print to", |ui, width| {
        let properties_reachable =
            printer_properties_reachable() && draft.format == OutputFormat::NativePrinter;
        let button = Button::new("Properties…")
            .enabled(properties_reachable && !draft.printer_id.is_empty());
        let button_width = button.measured_width(ui);
        let combo_width = (width - button_width - LABEL_GAP).max(120.0);
        crate::ui::widgets::Combo::from_id_salt("hardcopy-printer-destination")
            .selected_text(selected_text.as_str())
            .width(combo_width)
            .truncate()
            .show_ui(ui, |ui| {
                printer_entries(ui, draft, &mut picked);
                ui.separator();
                for (format, label) in [
                    (OutputFormat::PdfVector, "PDF file"),
                    (OutputFormat::SvgVector, "SVG file"),
                ] {
                    if ui
                        .add(egui::Button::selectable(draft.format == format, label))
                        .clicked()
                    {
                        picked = Some(Destination::File(format));
                    }
                }
            });
        button.show(ui).clicked()
    });
    if properties {
        *action = BodyAction::PrinterProperties;
    }
    match picked {
        Some(Destination::Printer(device_id)) => {
            draft.format = OutputFormat::NativePrinter;
            *action = BodyAction::SelectPrinter(device_id);
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        Some(Destination::RuntimePrint) => draft.format = runtime_print_format_choice(),
        Some(Destination::File(format)) => draft.format = format,
        None => {}
    }
}

/// The system printers discovery found, or what stands in for them.
fn printer_entries(ui: &mut Ui, draft: &HardcopyDialogState, picked: &mut Option<Destination>) {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        let printers = draft
            .printer_report
            .as_ref()
            .map(|report| report.printers())
            .unwrap_or_default();
        for entry in printers {
            let capabilities = entry.capabilities();
            let selected = draft.format == OutputFormat::NativePrinter
                && draft.printer_id == capabilities.device_id();
            if ui
                .add(egui::Button::selectable(
                    selected,
                    capabilities.display_name(),
                ))
                .clicked()
            {
                *picked = Some(Destination::Printer(capabilities.device_id().to_owned()));
            }
        }
        if printers.is_empty() {
            let text = if draft.printer_discovery_busy {
                "Looking for printers…"
            } else {
                "No printers found"
            };
            ui.add_enabled(false, egui::Button::selectable(false, text));
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    if ui
        .add(egui::Button::selectable(
            draft.format == OutputFormat::BrowserPrintDocument,
            runtime_print_label(),
        ))
        .clicked()
    {
        *picked = Some(Destination::RuntimePrint);
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
const fn runtime_print_format_choice() -> OutputFormat {
    OutputFormat::BrowserPrintDocument
}

const fn runtime_print_label() -> &'static str {
    if cfg!(target_arch = "wasm32") {
        "Browser print dialog"
    } else {
        "Print-ready document"
    }
}

fn copies_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) {
    let job = draft
        .printer_job
        .clone()
        .filter(|_| draft.format == OutputFormat::NativePrinter);
    let mut copies = job.as_ref().map_or(1, |job| u32::from(job.copies()));
    let changed = rows.row("Copies", |ui, _| {
        ui.add_enabled_ui(job.is_some(), |ui| {
            whole_number(ui, &mut copies, 1..=999, "Copies")
        })
        .inner
    });
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    if changed && let (Some(job), Some(capabilities)) = (job, draft.printer_capabilities.clone()) {
        let copies = u16::try_from(copies).unwrap_or(1);
        apply_printer_job(
            draft,
            &capabilities,
            job.selected_paper_id(),
            job.media_source().clone(),
            job.resolution_dpi(),
            job.duplex(),
            copies,
            copies > 1,
        );
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let _ = changed;
}

/// A page range only where there is more than one page to choose from.
fn page_range_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState, total: u32) {
    rows.held(total > 1, "Pages", |ui, width| {
        let mut ranged = usize::from(draft.page_range.is_some());
        let changed = sized(ui, (width * 0.4).min(120.0), |ui| {
            segmented(
                ui,
                "hardcopy-page-range",
                &["All", "Range"],
                &mut ranged,
                SegmentedWidth::Equal,
            )
        });
        if changed {
            draft.page_range = (ranged == 1).then_some((1, total));
        }
        if let Some((mut first, mut last)) = draft.page_range {
            let first_changed = whole_number(ui, &mut first, 1..=total, "First page");
            ui.label("to");
            let last_changed = whole_number(ui, &mut last, 1..=total, "Last page");
            if first_changed {
                last = last.max(first);
            }
            if last_changed {
                first = first.min(last);
            }
            draft.page_range = Some((first, last));
        } else {
            faint(ui, &format!("{total} pages"));
        }
    });
}

/// Keep the draft's range inside the pages the drawing takes now. A change
/// of paper or scale can take the last page away from under it.
fn clamp_page_range(draft: &mut HardcopyDialogState, total: u32) {
    if total <= 1 {
        draft.page_range = None;
    } else if let Some((first, last)) = draft.page_range {
        let last = last.clamp(1, total);
        draft.page_range = Some((first.clamp(1, last), last));
    }
}

fn format_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) {
    // The raster entries carry whatever resolution is currently in play, so
    // leaving PNG for SVG and coming back does not silently reset it.
    let dpi = draft.format.raster_dpi().unwrap_or(DEFAULT_RASTER_DPI);
    let png = bounded_raster(draft, OutputFormat::Png { dpi });
    let tiff = bounded_raster(draft, OutputFormat::Tiff { dpi });
    rows.row("Save as", |ui, width| {
        crate::ui::widgets::Combo::from_id_salt("hardcopy-format")
            .selected_text(format_label(draft.format))
            .width(width)
            .show_ui(ui, |ui| {
                for (format, label) in [
                    (OutputFormat::PdfVector, "PDF · vector"),
                    (OutputFormat::PdfA, "PDF/A · vector"),
                    (OutputFormat::SvgVector, "SVG · vector"),
                    (png, "PNG · raster"),
                    (tiff, "TIFF · raster"),
                ] {
                    ui.selectable_value(&mut draft.format, format, label);
                }
            });
    });
}

/// The resolution of a raster file, bounded by what this host can raster
/// for the page; a vector file has none, and the row keeps its place.
fn resolution_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) {
    let raster = draft.format.raster_dpi().is_some();
    rows.held(raster, "Resolution", |ui, _| {
        let ceiling = raster_dpi_ceiling(draft);
        let response = number_field(
            ui,
            raster_resolution_id(),
            &mut draft.raster_dpi_draft,
            96.0,
            ("", "dpi"),
            "Resolution",
        );
        // Commit on Enter or on leaving the field. Never per keystroke: `60`
        // is a legal prefix of `600` and an order of magnitude away from it.
        if response.lost_focus() {
            commit_raster_resolution(draft, ceiling);
        }
        let (message, tone) = raster_resolution_note(&draft.raster_dpi_draft, ceiling);
        ui.add(
            egui::Label::new(
                egui::RichText::new(message)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(tone(ui)),
            )
            .truncate(),
        );
    });
}

// ---- Paper -------------------------------------------------------------------

fn paper_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState, action: &mut BodyAction) {
    let sheet_media = schematic_sheet_relationship(draft).is_some();
    let selected = paper_label(&draft.paper);
    rows.row("Size", |ui, width| {
        crate::ui::widgets::Combo::from_id_salt("hardcopy-paper")
            .selected_text(selected)
            .width(width)
            .truncate()
            .show_ui(ui, |ui| {
                if sheet_media {
                    ui.selectable_value(
                        &mut draft.paper,
                        PaperDraft::MatchAuthoredSheets,
                        "Match each drawing sheet",
                    )
                    .on_hover_text(
                        "Use each schematic sheet's own size and orientation. A set of mixed sheets keeps each page's own paper size.",
                    );
                    ui.separator();
                }
                for paper in [
                    StandardPaper::Letter,
                    StandardPaper::Legal,
                    StandardPaper::A4,
                    StandardPaper::A3,
                    StandardPaper::Tabloid,
                ] {
                    ui.selectable_value(
                        &mut draft.paper,
                        PaperDraft::Standard(paper),
                        standard_paper_label(paper),
                    );
                }
                if ui
                    .selectable_label(matches!(draft.paper, PaperDraft::Custom { .. }), "Custom…")
                    .clicked()
                {
                    if !matches!(draft.paper, PaperDraft::Custom { .. }) {
                        draft.paper = custom_from_current(draft);
                    }
                    *action = BodyAction::CustomPaper;
                }
            });
    });
}

fn orientation_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) {
    let options = [
        (Orientation::Landscape, "Landscape"),
        (Orientation::Portrait, "Portrait"),
        (Orientation::AutomaticPerPage, "Automatic"),
    ];
    let mut index = options
        .iter()
        .position(|(orientation, _)| *orientation == draft.orientation)
        .unwrap_or(0);
    let labels = options.map(|(_, label)| label);
    let changed = rows.row("Orientation", |ui, width| {
        sized(ui, width, |ui| {
            segmented(
                ui,
                "hardcopy-orientation",
                &labels,
                &mut index,
                SegmentedWidth::Equal,
            )
        })
    });
    if changed {
        draft.orientation = options[index].0;
    }
}

/// The margins a preset names, top, right, bottom and left. Narrow margins
/// are as near the edge as the chosen printer prints, where it says; a
/// quarter inch otherwise.
fn preset_margins(draft: &HardcopyDialogState, narrow: bool) -> [Length; 4] {
    if narrow {
        let printer = draft
            .printer_job
            .as_ref()
            .filter(|job| {
                draft.format == OutputFormat::NativePrinter
                    && draft.printer_capabilities.as_ref().is_some_and(|printer| {
                        printer.states_printable_area(job.selected_paper_id())
                    })
            })
            .map(|job| {
                super::super::unsaved_sheet::widest_unprintable_margin(
                    job.raster_geometry(),
                    job.resolution_dpi(),
                )
            });
        [printer.unwrap_or(Length::from_micrometres(NARROW_MARGIN_UM)); 4]
    } else {
        let margins = rspice_hardcopy_contract::HardcopySetup::default()
            .physical_page()
            .margins();
        [margins.top, margins.right, margins.bottom, margins.left]
    }
}

fn draft_margins(draft: &HardcopyDialogState) -> Option<[Length; 4]> {
    let parse = |value: &str| Length::parse_decimal(value, draft.display_unit).ok();
    Some([
        parse(&draft.margin_top)?,
        parse(&draft.margin_right)?,
        parse(&draft.margin_bottom)?,
        parse(&draft.margin_left)?,
    ])
}

fn set_margins(draft: &mut HardcopyDialogState, margins: [Length; 4]) {
    let [top, right, bottom, left] =
        margins.map(|margin| format_length_local(margin, draft.display_unit));
    draft.margin_top = top;
    draft.margin_right = right;
    draft.margin_bottom = bottom;
    draft.margin_left = left;
}

fn margins_rows(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) {
    let current = draft_margins(draft);
    let mut index = if draft.margins_custom {
        2
    } else if current == Some(preset_margins(draft, false)) {
        0
    } else if current == Some(preset_margins(draft, true)) {
        1
    } else {
        2
    };
    let changed = rows.row("Margins", |ui, width| {
        sized(ui, width, |ui| {
            segmented(
                ui,
                "hardcopy-margins",
                &["Normal", "Narrow", "Custom"],
                &mut index,
                SegmentedWidth::Equal,
            )
        })
    });
    if changed {
        draft.margins_custom = index == 2;
        if index < 2 {
            set_margins(draft, preset_margins(draft, index == 1));
        }
    }
    let unit = unit_label(draft.display_unit);
    rows.held(index == 2, "", |ui, width| {
        ui.spacing_mut().item_spacing.x = 4.0;
        let field = ((width - 3.0 * 4.0) / 4.0).floor();
        for (lead, name, value) in [
            ("T", "Top margin", &mut draft.margin_top),
            ("R", "Right margin", &mut draft.margin_right),
            ("B", "Bottom margin", &mut draft.margin_bottom),
            ("L", "Left margin", &mut draft.margin_left),
        ] {
            number_field(
                ui,
                egui::Id::new(("hardcopy-margin", lead)),
                value,
                field,
                (lead, unit),
                name,
            );
        }
    });
}

// ---- Layout ------------------------------------------------------------------

fn scale_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) {
    let options = [
        (0_u8, "Fit to page"),
        (3, "Fit to width"),
        (1, "Actual size"),
        (2, "Percent"),
    ];
    let mut kind = scale_key(draft.scale);
    let selected = options
        .iter()
        .find(|(key, _)| *key == kind)
        .map_or("Fit to page", |(_, label)| *label);
    let derived = resolved_percent(draft);
    rows.row("Scale", |ui, width| {
        let combo_width = (width * 0.55).floor();
        crate::ui::widgets::Combo::from_id_salt("hardcopy-scale")
            .selected_text(selected)
            .width(combo_width)
            .show_ui(ui, |ui| {
                for (key, label) in options {
                    ui.selectable_value(&mut kind, key, label);
                }
            });
        if kind == 2 {
            number_field(
                ui,
                egui::Id::new("hardcopy-scale-percent"),
                &mut draft.custom_scale_percent,
                84.0,
                ("", "%"),
                "Scale percent",
            );
        } else if let Some(derived) = derived {
            faint(ui, &derived);
        }
    });
    draft.scale = match kind {
        0 => ScaleMode::FitPrintableArea,
        1 => ScaleMode::EngineeringOneToOne,
        2 => ScaleMode::CustomPercent {
            hundredths_percent: 10_000,
        },
        _ => ScaleMode::FitWidth,
    };
}

/// The number a fitted drawing comes to, so a fit never reads as 1:1.
fn resolved_percent(draft: &HardcopyDialogState) -> Option<String> {
    if !matches!(
        draft.scale,
        ScaleMode::FitPrintableArea | ScaleMode::FitWidth
    ) {
        return None;
    }
    let tenths = draft
        .preview_plan
        .as_ref()?
        .pagination()
        .scale()
        .hundredths_percent()
        / 10;
    Some(format!("{}.{}%", tenths / 10, tenths % 10))
}

/// How a drawing larger than its page is split, where it is: the grid, the
/// overlap at each seam and the marks that line the pages up.
fn tiling_rows(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState, tiled: bool) {
    let mut kind = tiling_key(draft.tiling);
    rows.held(tiled, "Tiling", |ui, width| {
        let labels = ["Automatic", "Columns × rows", "One page"];
        let selected = match kind {
            0 => labels[0],
            1 => labels[2],
            _ => labels[1],
        };
        crate::ui::widgets::Combo::from_id_salt("hardcopy-tiling")
            .selected_text(selected)
            .width((width * 0.55).floor())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut kind, 0, labels[0]);
                ui.selectable_value(&mut kind, 2, labels[1]).on_hover_text(
                    "Print on exactly this many pages. Asking for more than the drawing needs widens every overlap between pages.",
                );
                ui.selectable_value(&mut kind, 1, labels[2]).on_hover_text(
                    "Print on one page or not at all. Choose Fit to page to make the drawing fit.",
                );
            });
        if kind == 2 {
            ui.spacing_mut().item_spacing.x = 4.0;
            number_field(
                ui,
                egui::Id::new("hardcopy-tiling-columns"),
                &mut draft.manual_columns,
                44.0,
                ("", ""),
                "Columns",
            );
            ui.label("×");
            number_field(
                ui,
                egui::Id::new("hardcopy-tiling-rows"),
                &mut draft.manual_rows,
                44.0,
                ("", ""),
                "Rows",
            );
        }
    });
    draft.tiling = match kind {
        0 => TilingMode::Automatic,
        1 => TilingMode::SinglePage,
        _ => TilingMode::Manual {
            columns: 2,
            rows: 1,
        },
    };
    let unit = unit_label(draft.display_unit);
    rows.held(tiled, "Overlap", |ui, _| {
        number_field(
            ui,
            egui::Id::new("hardcopy-overlap"),
            &mut draft.overlap,
            96.0,
            ("", unit),
            "Page overlap",
        );
        ui.checkbox(&mut draft.registration_marks, "Registration marks");
    });
}

// ---- Content -----------------------------------------------------------------

fn content_rows(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState, action: &mut BodyAction) {
    let sheet = schematic_sheet_relationship(draft);
    document_rows(rows, draft, action, sheet);
    let plot = draft
        .source
        .as_ref()
        .is_some_and(|source| source.document_kind() == HardcopyDocumentKind::PlotOrWorksheet);
    let mut title_block = draft.include_header;
    let mut watermark = !matches!(draft.watermark, Watermark::None);
    let mut checks: Vec<(&str, &mut bool)> = vec![
        ("Title block", &mut title_block),
        ("Legend", &mut draft.include_legends),
    ];
    if plot {
        checks.push(("Cursors and markers", &mut draft.include_cursors_markers));
        checks.push(("Notes", &mut draft.include_notes));
    }
    if sheet.is_some() {
        checks.push(("Border", &mut draft.include_sheet_border));
        checks.push(("Reference zones", &mut draft.include_sheet_zones));
        checks.push(("Schematic grid", &mut draft.include_schematic_grid));
        checks.push(("Paper and bleed", &mut draft.include_sheet_paper));
        checks.push(("Crop marks", &mut draft.crop_marks));
    }
    checks.push(("Draft watermark", &mut watermark));
    check_grid(rows, "Show", &mut checks);
    if title_block != draft.include_header {
        // One switch for the identity a page carries: its header, the footer
        // naming what made it, and a drawing sheet's own title block.
        draft.include_header = title_block;
        draft.include_provenance = title_block;
        draft.include_sheet_title_block = title_block;
    }
    draft.watermark = if watermark {
        if matches!(draft.watermark, Watermark::None) {
            Watermark::Draft
        } else {
            draft.watermark.clone()
        }
    } else {
        Watermark::None
    };
}

/// Checks two to a line, under one label. The second column starts past the
/// widest label in the first, so a long name never runs into its neighbour.
fn check_grid(rows: &mut Rows<'_>, label: &str, checks: &mut [(&str, &mut bool)]) {
    let ctl_h = Tokens::get(rows.ui.ctx()).metrics.ctl_h;
    let lines = checks.len().div_ceil(2);
    // The first line is centred on the label like any single-line row.
    let lead = ((ctl_h - CHECK_LINE_HEIGHT) * 0.5).max(0.0);
    let height = (lead + lines as f32 * CHECK_LINE_HEIGHT).max(ctl_h);
    rows.lines(label, height, |ui, width| {
        let (grid, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let spacing = ui.spacing();
        let box_width = spacing.icon_width + spacing.icon_spacing;
        let font = egui::TextStyle::Button.resolve(ui.style());
        let first_column = checks
            .iter()
            .step_by(2)
            .map(|(name, _)| {
                ui.fonts_mut(|fonts| {
                    fonts
                        .layout_no_wrap((*name).to_owned(), font.clone(), egui::Color32::WHITE)
                        .size()
                        .x
                })
            })
            .fold(0.0_f32, f32::max);
        let second = (box_width + first_column + CHECK_COLUMN_GAP)
            .ceil()
            .min(width * 0.6);
        for (index, (name, value)) in checks.iter_mut().enumerate() {
            let (left, right) = if index % 2 == 0 {
                (0.0, second)
            } else {
                (second, width)
            };
            let top = grid.top() + lead + (index / 2) as f32 * CHECK_LINE_HEIGHT;
            let cell = Rect::from_min_max(
                egui::pos2(grid.left() + left, top),
                egui::pos2(grid.left() + right, top + CHECK_LINE_HEIGHT),
            );
            ui.scope_builder(
                egui::UiBuilder::new()
                    .id_salt(("hardcopy-check", *name))
                    .max_rect(cell)
                    .layout(Layout::left_to_right(Align::Center)),
                |ui| ui.checkbox(value, *name),
            );
        }
    });
}

/// Which open document, and how much of it, where there is a choice. A
/// drawing sheet's own extent shares the row with the document's.
fn document_rows(
    rows: &mut Rows<'_>,
    draft: &mut HardcopyDialogState,
    action: &mut BodyAction,
    sheet: Option<bool>,
) {
    let active_kind = draft.source.as_ref().map(|source| source.document_kind());
    let active_key = draft
        .resolved_document
        .as_ref()
        .map(|source| source.source_key().to_owned());
    let kinds = [
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyDocumentKind::LayoutWithLayerLegend,
        HardcopyDocumentKind::PlotOrWorksheet,
        HardcopyDocumentKind::Report,
    ]
    .into_iter()
    .filter_map(|kind| {
        draft
            .source_candidates
            .iter()
            .find(|candidate| {
                candidate.document_kind == kind && candidate.availability.is_available()
            })
            .map(|candidate| (kind, candidate))
    })
    .collect::<Vec<_>>();
    if kinds.len() > 1 {
        rows.row("Document", |ui, width| {
            crate::ui::widgets::Combo::from_id_salt("hardcopy-document-type")
                .selected_text(active_kind.map_or("Nothing to print", document_kind_label))
                .width(width)
                .show_ui(ui, |ui| {
                    for (kind, candidate) in &kinds {
                        if ui
                            .add(egui::Button::selectable(
                                active_kind == Some(*kind),
                                document_kind_label(*kind),
                            ))
                            .clicked()
                            && active_key.as_deref() != Some(candidate.source_key.as_str())
                            && let Some((source_key, scope)) = source_choice_for_active_extent(
                                &draft.source_candidates,
                                Some(candidate),
                            )
                        {
                            *action = BodyAction::SelectSource { source_key, scope };
                        }
                    }
                });
        });
    }

    let active_scope = draft.source.as_ref().map(|source| source.scope().clone());
    let active_candidate = active_key.as_deref().and_then(|key| {
        draft
            .source_candidates
            .iter()
            .find(|candidate| candidate.source_key == key)
    });
    let mut choices = vec![
        (
            active_scope
                .as_ref()
                .map_or("This document", scope_label)
                .to_owned(),
            source_choice_for_active_extent(&draft.source_candidates, active_candidate),
        ),
        (
            "Selection".to_owned(),
            source_choice_for_scope(
                &draft.source_candidates,
                active_candidate,
                HardcopyScope::Selection,
            ),
        ),
        (
            "Every sheet or pane".to_owned(),
            source_choice_for_scope(
                &draft.source_candidates,
                active_candidate,
                HardcopyScope::AllSheetsOrPanes,
            ),
        ),
    ];
    for (source_key, scope, name) in
        named_source_choices(&draft.source_candidates, active_candidate)
    {
        choices.push((format!("Print set · {name}"), Some((source_key, scope))));
    }
    let scoped = choices
        .iter()
        .filter(|(_, choice)| choice.is_some())
        .count()
        >= 2;
    if !scoped && sheet.is_none() {
        return;
    }
    let selected = active_scope
        .as_ref()
        .map_or("Nothing to print", scope_label)
        .to_owned();
    rows.row("Extent", |ui, width| {
        let sheet_width = match (scoped, sheet.is_some()) {
            (true, true) => ((width - LABEL_GAP) / 2.0).floor(),
            (false, true) => width,
            _ => 0.0,
        };
        if scoped {
            let scope_width = if sheet.is_some() {
                width - LABEL_GAP - sheet_width
            } else {
                width
            };
            crate::ui::widgets::Combo::from_id_salt("hardcopy-scope")
                .selected_text(selected)
                .width(scope_width)
                .truncate()
                .show_ui(ui, |ui| {
                    for (label, choice) in choices {
                        let Some((source_key, scope)) = choice else {
                            continue;
                        };
                        let active = active_key.as_deref() == Some(source_key.as_str())
                            && active_scope.as_ref() == Some(&scope);
                        if ui.add(egui::Button::selectable(active, label)).clicked() && !active {
                            *action = BodyAction::SelectSource { source_key, scope };
                        }
                    }
                });
        }
        if let Some(has_outside_content) = sheet {
            sheet_extent_combo(ui, draft, has_outside_content, sheet_width);
        }
    });
}

/// How much of an authored drawing sheet is printed, as one choice: the sheet,
/// or everything drawn. Where something is drawn past the sheet's edge, the
/// sheet is offered clipped at that edge or extended to take it, and nothing
/// is chosen for the operator until they choose.
fn sheet_extent_combo(
    ui: &mut Ui,
    draft: &mut HardcopyDialogState,
    has_outside_content: bool,
    width: f32,
) {
    use OutsideSheetContentPolicy::{Ask, ClipToAuthoredSheet, ExtendOutput};
    use SchematicHardcopyExtent::{AuthoredDrawingSheet, CompleteSchematicContent};
    let kept = draft.outside_sheet_content;
    let options: &[(
        SchematicHardcopyExtent,
        OutsideSheetContentPolicy,
        &str,
        &str,
    )] = if has_outside_content {
        &[
            (
                AuthoredDrawingSheet,
                ClipToAuthoredSheet,
                "Sheet, clipped at its edge",
                "Print the drawing sheet and leave out what is drawn past its edge.",
            ),
            (
                AuthoredDrawingSheet,
                ExtendOutput,
                "Sheet, extended past its edge",
                "Print the drawing sheet and grow the page to take what is drawn past its edge.",
            ),
            (
                CompleteSchematicContent,
                ExtendOutput,
                "Everything drawn",
                "Print everything drawn, without the drawing sheet's bounds.",
            ),
        ]
    } else {
        &[
            (
                AuthoredDrawingSheet,
                Ask,
                "Drawing sheet",
                "Print the drawing sheet as it is drawn.",
            ),
            (
                CompleteSchematicContent,
                ExtendOutput,
                "Everything drawn",
                "Print everything drawn, without the drawing sheet's bounds.",
            ),
        ]
    };
    let chosen = |extent, policy| {
        extent == draft.schematic_extent
            && (extent == CompleteSchematicContent || !has_outside_content || policy == kept)
    };
    let selected = options
        .iter()
        .find(|(extent, policy, _, _)| chosen(*extent, *policy))
        .map_or("Clip or extend the sheet?", |(_, _, label, _)| *label);
    let mut picked = None;
    crate::ui::widgets::Combo::from_id_salt("hardcopy-sheet-extent")
        .selected_text(selected)
        .width(width)
        .truncate()
        .show_ui(ui, |ui| {
            for (extent, policy, label, hint) in options {
                if ui
                    .add(egui::Button::selectable(chosen(*extent, *policy), *label))
                    .on_hover_text(*hint)
                    .clicked()
                {
                    picked = Some((*extent, *policy));
                }
            }
        });
    if let Some((extent, policy)) = picked {
        draft.schematic_extent = extent;
        if extent == CompleteSchematicContent || has_outside_content {
            draft.outside_sheet_content = policy;
        }
    }
    if draft.schematic_extent == CompleteSchematicContent {
        draft.outside_sheet_content = ExtendOutput;
    }
}

// ---- Output ------------------------------------------------------------------

fn color_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState, action: &mut BodyAction) {
    draft.fit_colors_to_printer();
    let color_supported = !draft.prints_black_only();
    let mapping = rows.row("Color", |ui, width| {
        let button = Button::new("Mapping…");
        let button_width = button.measured_width(ui);
        crate::ui::widgets::Combo::from_id_salt("hardcopy-color")
            .selected_text(color_mapping_short_label(draft.color_mapping))
            .width((width - button_width - LABEL_GAP).max(120.0))
            .truncate()
            .show_ui(ui, |ui| {
                for (value, label, needs_color) in [
                    (
                        ColorMapping::PrintSafeEngineeringPalette,
                        "Print-safe engineering palette",
                        true,
                    ),
                    (ColorMapping::ScreenColors, "Screen colors", true),
                    (
                        ColorMapping::GrayscaleWithDashMarkerRedundancy,
                        "Grayscale, told apart by dashes and markers",
                        false,
                    ),
                    (ColorMapping::Monochrome, "Monochrome", false),
                ] {
                    let response = ui.add_enabled(
                        color_supported || !needs_color,
                        egui::Button::selectable(draft.color_mapping == value, label),
                    );
                    if response.clicked() {
                        draft.color_mapping = value;
                    } else if needs_color && !color_supported {
                        response
                            .on_disabled_hover_text("This printer prints in black and white only.");
                    }
                }
            });
        button
            .show(ui)
            .on_hover_text("How each layer, trace and marker is told apart on paper")
            .clicked()
    });
    if mapping {
        *action = BodyAction::PrintMapping;
    }
}

fn background_row(rows: &mut Rows<'_>, draft: &mut HardcopyDialogState) {
    let transparent_available = draft.format.is_vector()
        && !matches!(
            draft.format,
            OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument
        );
    if draft.background == BackgroundMode::Transparent && !transparent_available {
        draft.background = BackgroundMode::White;
    }
    rows.row("Background", |ui, width| {
        crate::ui::widgets::Combo::from_id_salt("hardcopy-background")
            .selected_text(background_label(draft.background))
            .width((width * 0.55).floor())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut draft.background, BackgroundMode::White, "White");
                let transparent = ui.add_enabled(
                    transparent_available,
                    egui::Button::selectable(
                        draft.background == BackgroundMode::Transparent,
                        "Transparent",
                    ),
                );
                if transparent.clicked() {
                    draft.background = BackgroundMode::Transparent;
                } else if !transparent_available {
                    transparent.on_disabled_hover_text(
                        "Only a PDF or SVG file can have a transparent background.",
                    );
                }
                ui.selectable_value(
                    &mut draft.background,
                    BackgroundMode::WorkspaceBackground,
                    "Workspace background",
                );
            });
        ui.checkbox(&mut draft.soft_proof, "Soft proof")
            .on_hover_text("Preview the colors as uncoated paper prints them");
    });
}

/// A color mapping as its field shows it; its list says the whole of it.
const fn color_mapping_short_label(mapping: ColorMapping) -> &'static str {
    match mapping {
        ColorMapping::PrintSafeEngineeringPalette => "Print-safe colors",
        ColorMapping::ScreenColors => "Screen colors",
        ColorMapping::GrayscaleWithDashMarkerRedundancy => "Grayscale",
        ColorMapping::Monochrome => "Monochrome",
    }
}

impl HardcopyDialogState {
    /// What the footer says when the page asks the printer for more than it
    /// can do: marks nearer the paper's edge than it prints, or an edge it
    /// does not state.
    pub(super) fn printer_margin_warning(&self) -> Option<String> {
        let unit = unit_label(self.display_unit);
        if let Some(shortfall) = self.preview_printer_shortfall() {
            let length = |micrometres: u64| {
                format_length_local(Length::from_micrometres(micrometres), self.display_unit)
            };
            return Some(format!(
                "Marks fall {} {unit} from the paper's edge, and this printer can't print within {} {unit}. Choose Narrow margins and Fit to page.",
                length(shortfall.ink_um()),
                length(shortfall.margin_um())
            ));
        }
        self.printer_area_unstated().then(|| {
            "This printer does not say how near its edge it prints, so an actual-size page cannot be checked against it.".to_owned()
        })
    }
}

//! Workspace-scoped document tabs. Stable project identities drive every tab;
//! the shell never fabricates datasets, source files, or verification records.

use egui::containers::menu::MenuButton;
use egui::{Frame, Layout, Panel, Rect, Sense, Ui, Vec2};

use crate::state::{AssemblyVariant, AssemblyVariantCatalog, AssemblyVariantId};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::Model;
use crate::workbench::framework::host::DocumentHost;
use crate::workbench::framework::workbench_host::{ConsoleHost, WorkbenchHost};
use rspice_app_state::workbench::app_state::model::BASE_DESIGN;

use super::super::layout::LayoutSpec;
use super::super::state::{WorkbenchSession, Workspace, WorkspaceDocumentId};
use crate::ui::panels::{WorkbenchIcon, elide_text};
use crate::workbench::state::workbench_session::WorkspaceDocument;

pub const TAB_MAX_WIDTH: f32 = 190.0;
/// The narrowest a tab shrinks to before the strip scrolls instead: its icon,
/// about eleven characters of its name, and its close button.
pub const TAB_MIN_WIDTH: f32 = 140.0;
const TAB_PADDING_X: f32 = 10.0;
const TAB_ICON_SIZE: f32 = 16.0;
const TAB_CONTENT_GAP: f32 = 7.0;
const TAB_CLOSE_SIZE: f32 = 16.0;
const TAB_DIRTY_SIZE: f32 = 6.0;
const TAB_ACTIVE_EDGE: f32 = 2.0;
const STRIP_FADE_WIDTH: f32 = 24.0;
const HIDDEN_BUTTON_WIDTH: f32 = 40.0;
/// Past this many hidden tabs, their menu offers a filter.
const HIDDEN_MENU_FILTER_THRESHOLD: usize = 8;

/// The session the bar and its commands are handed: the workbench session's
/// record of the open documents, and the shell's answers for the documents
/// each workspace offers. Any session that is both is one.
pub trait DocumentSession:
    ConsoleHost
    + DocumentHost<
        WorkspaceKey = Workspace,
        DocumentId = WorkspaceDocumentId,
        Document = WorkspaceDocument,
    >
{
}

impl<S> DocumentSession for S where
    S: ConsoleHost
        + DocumentHost<
            WorkspaceKey = Workspace,
            DocumentId = WorkspaceDocumentId,
            Document = WorkspaceDocument,
        >
{
}

pub fn show(root: &mut Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    reconcile_document_registry(app.session_mut());
    let documents = visible_documents(app.session());
    if !document_strip_visible(app.workbench().workspace, documents.len()) {
        return;
    }

    let t = Tokens::get(ctx);
    // The strip is window chrome and wears the toolbar's tone. The active tab
    // wears the page's and covers the hairline, so it opens onto the document
    // beneath it; painted in the page's tone, the strip ran into the page.
    Panel::top("workbench.document_bar")
        .exact_size(layout.document_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let rect = ui.max_rect();
            let height = layout.document_bar_height;
            ui.painter().hline(
                rect.x_range(),
                (rect.bottom() - 0.5).max(rect.top()),
                egui::Stroke::new(1.0, t.color.border),
            );
            let closable = closable_documents(app.session(), &documents);
            let natural = documents
                .iter()
                .zip(&closable)
                .map(|(document, closable)| {
                    natural_tab_width(ui.painter(), &document.label, document.dirty, *closable)
                })
                .collect::<Vec<_>>();
            let hidden_button_width = HIDDEN_BUTTON_WIDTH.max(t.metrics.ctl_h);
            let chooser = VariantChooser::read(app);
            let chooser_width = chooser
                .as_ref()
                .map_or(0.0, |chooser| chooser.width(ui.painter()));
            // Results' strip ends in a "+" that makes a new result document,
            // right after its last tab.
            let create = (app.workbench().workspace == Workspace::Results).then(|| {
                let key = crate::state::command_catalog::CommandKey::new(CREATE_RESULT_DOCUMENT);
                match app.availability(key) {
                    crate::workbench::framework::command_registry::CommandAvailability::Disabled(
                        reason,
                    ) => Some(reason),
                    _ => None,
                }
            });
            let create_width = if create.is_some() {
                hidden_button_width
            } else {
                0.0
            };
            let fit = StripFit::resolve(
                &natural,
                (rect.width() - chooser_width - create_width).max(0.0),
                hidden_button_width,
            );
            // The tabs' track: the strip's width while tabs scroll in it, else
            // just the tabs, so the "+" stands right after the last one.
            let track = if create.is_some() && !fit.overflowing {
                let cap = fit.cap.map_or(TAB_MAX_WIDTH, |cap| cap.min(TAB_MAX_WIDTH));
                natural
                    .iter()
                    .map(|width| width.min(cap))
                    .sum::<f32>()
                    .min(fit.tabs_width)
            } else {
                fit.tabs_width
            };
            let tab_menu = (app.workbench().workspace == Workspace::Results)
                .then(|| result_document_tab_menu_rows(app, ctx));
            let mut request = StripRequest::default();
            ui.horizontal(|ui| {
                let scroll = ui
                    .allocate_ui_with_layout(
                        Vec2::new(track, height),
                        Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            // The track, not the content: the button after it
                            // must sit at the strip's right end.
                            ui.set_min_width(track);
                            egui::ScrollArea::horizontal()
                                .id_salt("workbench.document_tabs.scroll")
                                .scroll_bar_visibility(
                                    egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                                )
                                .show(ui, |ui| {
                                    let tabs = ui.horizontal(|ui| {
                                        document_tabs(
                                            ui,
                                            app.session_mut(),
                                            StripTabs {
                                                documents: &documents,
                                                closable: &closable,
                                                natural: &natural,
                                                menu: tab_menu.as_deref(),
                                            },
                                            height,
                                            fit.cap,
                                        )
                                    });
                                    ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
                                        node.set_role(egui::accesskit::Role::TabList);
                                        node.set_label("Open documents");
                                    });
                                    let (rects, menu) = tabs.inner;
                                    request.menu = menu;
                                    rects
                                })
                        },
                    )
                    .inner;
                paint_scroll_fades(ui, &scroll, t.color.bg_panel);
                if let Some(refusal) = &create {
                    request.create = create_document_button(
                        ui,
                        refusal.as_deref(),
                        Vec2::new(create_width, height),
                    );
                }
                if fit.overflowing {
                    let hidden = documents
                        .iter()
                        .zip(&scroll.inner)
                        .filter(|(_, tab)| !in_view(scroll.inner_rect, **tab))
                        .map(|(document, _)| document)
                        .collect::<Vec<_>>();
                    hidden_documents_button(
                        ui,
                        app.session_mut(),
                        &hidden,
                        Vec2::new(hidden_button_width, height),
                    );
                }
                if let Some(chooser) = chooser {
                    let rest = ui.available_width();
                    ui.add_space((rest - chooser_width).max(0.0));
                    chooser.show(ui, app, Vec2::new(chooser_width, height));
                }
            });
            run_strip_request(app, request, &documents);
        });
}

/// Do what the strip was asked for. A tab's Rename and Delete act on the
/// document shown, so the tab's document is shown first, as Close others
/// keeps the one shown.
fn run_strip_request(
    app: &mut impl WorkbenchHost,
    request: StripRequest,
    documents: &[WorkspaceDocument],
) {
    use crate::state::command_catalog::CommandKey;
    if request.create {
        app.execute(CommandKey::new(CREATE_RESULT_DOCUMENT));
    }
    let Some((document, choice)) = request.menu else {
        return;
    };
    match choice {
        TabMenuChoice::Close => {
            close_document(app.session_mut(), &document, documents);
        }
        TabMenuChoice::CloseOthers => {
            if activate_document(app.session_mut(), &document) {
                close_other_documents(app.session_mut());
            }
        }
        TabMenuChoice::Rename => {
            if activate_document(app.session_mut(), &document) {
                app.execute(CommandKey::new(RENAME_RESULT_DOCUMENT));
            }
        }
        TabMenuChoice::Delete => {
            if activate_document(app.session_mut(), &document) {
                app.execute(CommandKey::new(DELETE_RESULT_DOCUMENT));
            }
        }
    }
}

/// The "+" at the end of Results' strip, which opens the New result
/// document form; greyed, saying why, while no document can be made. True
/// when it was pressed.
fn create_document_button(ui: &mut Ui, refusal: Option<&str>, size: Vec2) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        size,
        if refusal.is_none() {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            refusal.is_none(),
            NEW_RESULT_DOCUMENT,
        )
    });
    if response.hovered() && refusal.is_none() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    WorkbenchIcon::Add.paint(
        ui.painter(),
        Rect::from_center_size(rect.center(), Vec2::splat(TAB_ICON_SIZE)),
        match (refusal, response.hovered()) {
            (Some(_), _) => t.color.text_faint,
            (None, true) => t.color.text,
            (None, false) => t.color.text_dim,
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    let response = match refusal {
        Some(refusal) => {
            let mut reason = refusal.chars();
            let reason = reason.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(reason).collect()
            });
            response.on_hover_text(format!("{NEW_RESULT_DOCUMENT}\n{reason}"))
        }
        None => response.on_hover_text(NEW_RESULT_DOCUMENT),
    };
    response.clicked()
}

/// The command that opens the New result document form, which Results'
/// strip offers at its end.
const CREATE_RESULT_DOCUMENT: &str = "create-result-document";
/// The commands that rename and delete the saved result document shown,
/// which a result document's tab offers in its menu.
const RENAME_RESULT_DOCUMENT: &str = "rename-result-document";
const DELETE_RESULT_DOCUMENT: &str = "delete-result-document";
/// The commands that close the document in front and every other one, whose
/// keys a result document's tab menu shows beside its Close rows.
const CLOSE_DOCUMENT: &str = "close-document";
const CLOSE_OTHER_DOCUMENTS: &str = "close-other-documents";
/// What Results' strip offers at its end.
const NEW_RESULT_DOCUMENT: &str = "New result document\u{2026}";

/// What a tab's menu asked for, of the tab it was opened on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TabMenuChoice {
    Rename,
    Close,
    CloseOthers,
    Delete,
}

/// What the reader asked of the strip that only its host can do: a tab's
/// menu choice, or the strip's "+".
#[derive(Default)]
struct StripRequest {
    menu: Option<(WorkspaceDocumentId, TabMenuChoice)>,
    create: bool,
}

/// The command that opens Design management on its Variants tab.
const MANAGE_VARIANTS: &str = "design-variants";
/// The command that opens Design management's New variant form.
const NEW_VARIANT: &str = "design-new-variant";

/// The variant chooser at the bar's right end: which variant the design is
/// netlisted, drawn and inspected as. It is there only in the Design
/// workspace of a project that has a variant.
struct VariantChooser {
    /// The variant chosen; `None` is the base design.
    chosen: Option<AssemblyVariantId>,
    /// What the field shows: the chosen variant's name.
    shown: String,
    /// Every variant, parents before their children, with its depth.
    variants: Vec<(AssemblyVariantId, String, usize)>,
}

/// Every variant of `catalog` as the chooser lists it: each parent before
/// its children, children in catalog order, with each one's depth below a
/// variant that has no parent.
fn variant_rows(catalog: &AssemblyVariantCatalog) -> Vec<(AssemblyVariantId, String, usize)> {
    let parent_of = |variant: &AssemblyVariant| {
        variant
            .definition()
            .parent
            .as_ref()
            .map(|parent| parent.id)
            .filter(|id| catalog.find(*id).is_some())
    };
    let mut rows: Vec<(AssemblyVariantId, String, usize)> =
        Vec::with_capacity(catalog.variants().len());
    let mut stack: Vec<(&AssemblyVariant, usize)> = catalog
        .variants()
        .iter()
        .rev()
        .filter(|variant| parent_of(variant).is_none())
        .map(|variant| (variant, 0))
        .collect();
    while let Some((variant, depth)) = stack.pop() {
        if rows.iter().any(|(id, ..)| *id == variant.id()) {
            continue;
        }
        rows.push((variant.id(), variant.name().to_owned(), depth));
        stack.extend(
            catalog
                .variants()
                .iter()
                .rev()
                .filter(|child| parent_of(child) == Some(variant.id()))
                .map(|child| (child, depth + 1)),
        );
    }
    rows
}

impl VariantChooser {
    const PADDING_X: f32 = 10.0;
    const GAP: f32 = 6.0;
    const CHEVRON: f32 = 12.0;

    fn read(app: &impl WorkbenchHost) -> Option<Self> {
        if app.workbench().workspace != Workspace::Design {
            return None;
        }
        let catalog = app.model().workspace.content.design_management.variants();
        if catalog.variants().is_empty() {
            return None;
        }
        let variants = variant_rows(catalog);
        let chosen = catalog.active_variant_id();
        let shown = catalog.active().map_or_else(
            || BASE_DESIGN.to_owned(),
            |variant| variant.name().to_owned(),
        );
        Some(Self {
            chosen,
            shown,
            variants,
        })
    }

    fn caption(painter: &egui::Painter, text: &str, weight: FontWeight) -> f32 {
        painter
            .layout_no_wrap(
                text.to_owned(),
                theme::sans(tokens::FS_1, weight),
                egui::Color32::WHITE,
            )
            .size()
            .x
    }

    fn width(&self, painter: &egui::Painter) -> f32 {
        let name = Self::caption(painter, &self.shown, FontWeight::Medium).min(TAB_MAX_WIDTH);
        Self::PADDING_X * 2.0
            + Self::caption(painter, "Variant", FontWeight::Regular)
            + Self::GAP
            + name
            + Self::GAP
            + Self::CHEVRON
    }

    fn show(self, ui: &mut Ui, app: &mut impl WorkbenchHost, size: Vec2) {
        let t = Tokens::get(ui.ctx());
        let mut picked: Option<Option<AssemblyVariantId>> = None;
        let mut command = None;
        let response = ui
            .scope(|ui| {
                ui.spacing_mut().button_padding = Vec2::ZERO;
                MenuButton::from_button(egui::Button::new("").frame(false).min_size(size))
                    .ui(ui, |ui| {
                        ui.set_min_width(size.x.max(200.0));
                        let row = |ui: &mut Ui, selected: bool, label: &str| {
                            ui.add_sized(
                                [ui.available_width(), t.metrics.ctl_h],
                                egui::Button::selectable(selected, label),
                            )
                            .clicked()
                        };
                        if row(ui, self.chosen.is_none(), BASE_DESIGN) {
                            picked = Some(None);
                        }
                        for (id, name, depth) in &self.variants {
                            let label = format!("{}{name}", "\u{2003}".repeat(*depth));
                            if row(ui, self.chosen == Some(*id), &label) {
                                picked = Some(Some(*id));
                            }
                        }
                        ui.separator();
                        if row(ui, false, "New variant\u{2026}") {
                            command = Some(NEW_VARIANT);
                        }
                        if row(ui, false, "Manage variants\u{2026}") {
                            command = Some(MANAGE_VARIANTS);
                        }
                        if picked.is_some() || command.is_some() {
                            ui.close();
                        }
                    })
                    .0
            })
            .inner;
        let spoken = format!("Variant {}", self.shown);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::ComboBox, ui.is_enabled(), &spoken)
        });
        let rect = response.rect;
        let name_font = theme::sans(tokens::FS_1, FontWeight::Medium);
        let caption_width = Self::caption(ui.painter(), "Variant", FontWeight::Regular);
        let name_x = rect.left() + Self::PADDING_X + caption_width + Self::GAP;
        let name_width =
            (rect.right() - Self::PADDING_X - Self::CHEVRON - Self::GAP - name_x).max(1.0);
        let name = elide_text(ui, &self.shown, &name_font, name_width);
        let painter = ui.painter();
        painter.rect_filled(
            rect,
            0.0,
            if response.hovered() {
                t.color.bg_hover
            } else {
                t.color.bg_panel
            },
        );
        painter.vline(
            rect.left(),
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
        painter.hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );
        painter.text(
            egui::pos2(rect.left() + Self::PADDING_X, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Variant",
            theme::sans(tokens::FS_1, FontWeight::Regular),
            t.color.text_dim,
        );
        painter.text(
            egui::pos2(name_x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            name_font,
            t.color.text,
        );
        WorkbenchIcon::ChevronDown.paint(
            painter,
            Rect::from_center_size(
                egui::pos2(
                    rect.right() - Self::PADDING_X - Self::CHEVRON * 0.5,
                    rect.center().y,
                ),
                Vec2::splat(Self::CHEVRON),
            ),
            t.color.text_dim,
        );
        theme::paint_focus_ring(ui, &response, rect);
        response.on_hover_text("The variant the design is netlisted, drawn and inspected as");
        if let Some(choice) = picked {
            let outcome = app.model_mut().choose_assembly_variant(choice);
            if choice != self.chosen && outcome.is_ok() {
                app.simulation_input_changed();
            }
            let (Ok(message) | Err(message)) = outcome;
            app.workbench_mut().prompt(message);
        }
        if let Some(command) = command {
            app.execute(crate::state::command_catalog::CommandKey::new(command));
        }
    }
}

/// How the strip's tabs share its width.
///
/// Tabs keep their own widths while they fit. Past that they shrink evenly,
/// the narrow ones keeping theirs, down to [`TAB_MIN_WIDTH`]. Only past that
/// does the strip scroll, and only then does it give up room for the button
/// that lists the tabs scrolled out of view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripFit {
    /// The widest a tab may be, when the tabs do not fit at their own widths.
    pub cap: Option<f32>,
    /// The width the tabs scroll within.
    pub tabs_width: f32,
    /// Whether some tabs are out of view even at the narrowest a tab may be.
    pub overflowing: bool,
}

impl StripFit {
    pub fn resolve(natural: &[f32], strip_width: f32, hidden_button_width: f32) -> Self {
        let narrowest: f32 = natural.iter().map(|width| width.min(TAB_MIN_WIDTH)).sum();
        let overflowing = narrowest > strip_width + 0.5;
        let tabs_width = if overflowing {
            (strip_width - hidden_button_width).max(1.0)
        } else {
            strip_width
        };
        Self {
            cap: shared_width_cap(natural, tabs_width),
            tabs_width,
            overflowing,
        }
    }
}

/// The widest a tab may be for the tabs to fill `available`, or `None` when
/// they fit at their own widths. Tabs narrower than the cap keep their width,
/// and the cap never goes below [`TAB_MIN_WIDTH`].
pub fn shared_width_cap(natural: &[f32], available: f32) -> Option<f32> {
    if natural.iter().sum::<f32>() <= available + 0.5 {
        return None;
    }
    let mut widths = natural.to_vec();
    widths.sort_by(f32::total_cmp);
    let mut remaining = available;
    for (index, width) in widths.iter().enumerate() {
        let share = remaining / (widths.len() - index) as f32;
        if *width > share {
            return Some(share.max(TAB_MIN_WIDTH));
        }
        remaining -= width;
    }
    None
}

/// Fade the strip's ends where tabs carry on out of view, above the hairline.
fn paint_scroll_fades(
    ui: &Ui,
    scroll: &egui::scroll_area::ScrollAreaOutput<Vec<Rect>>,
    color: egui::Color32,
) {
    let band = scroll
        .inner_rect
        .with_max_y(scroll.inner_rect.bottom() - 1.0);
    let before = scroll.state.offset.x;
    let after = scroll.content_size.x - band.width() - before;
    let width = STRIP_FADE_WIDTH.min(band.width() * 0.5);
    if before > 0.5 {
        let edge = Rect::from_min_size(band.left_top(), Vec2::new(width, band.height()));
        paint_horizontal_fade(ui.painter(), edge, color, egui::Color32::TRANSPARENT);
    }
    if after > 0.5 {
        let edge = Rect::from_min_max(egui::pos2(band.right() - width, band.top()), band.max);
        paint_horizontal_fade(ui.painter(), edge, egui::Color32::TRANSPARENT, color);
    }
}

fn paint_horizontal_fade(
    painter: &egui::Painter,
    rect: Rect,
    left: egui::Color32,
    right: egui::Color32,
) {
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(rect.left_top(), left);
    mesh.colored_vertex(rect.right_top(), right);
    mesh.colored_vertex(rect.right_bottom(), right);
    mesh.colored_vertex(rect.left_bottom(), left);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(mesh);
}

/// "+N" at the strip's right end while tabs are out of view; its menu lists
/// those tabs and only those, and a choice opens the document, which scrolls
/// its tab into view.
fn hidden_documents_button(
    ui: &mut Ui,
    state: &mut impl DocumentSession,
    hidden: &[&WorkspaceDocument],
    size: Vec2,
) {
    let t = Tokens::get(ui.ctx());
    let active = active_document_id(state);
    let description = match hidden.len() {
        1 => "1 more open document".to_owned(),
        count => format!("{count} more open documents"),
    };
    let filter_id = egui::Id::new("workbench.document_tabs.hidden_filter");
    let mut chosen = None;
    let response = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            MenuButton::from_button(egui::Button::new("").frame(false).min_size(size))
                .ui(ui, |ui| {
                    ui.set_min_width(240.0);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    let mut filter = String::new();
                    if hidden.len() > HIDDEN_MENU_FILTER_THRESHOLD {
                        filter = ui
                            .data(|data| data.get_temp::<String>(filter_id))
                            .unwrap_or_default();
                        ui.add(
                            egui::TextEdit::singleline(&mut filter)
                                .hint_text("Filter")
                                .desired_width(f32::INFINITY),
                        );
                        ui.data_mut(|data| data.insert_temp(filter_id, filter.clone()));
                        ui.add_space(4.0);
                    }
                    let needle = filter.trim().to_lowercase();
                    let row_height = if size.y >= 44.0 { 44.0 } else { 29.0 };
                    for document in hidden
                        .iter()
                        .filter(|document| document.label.to_lowercase().contains(&needle))
                    {
                        let row = ui.add_sized(
                            [ui.available_width(), row_height],
                            egui::Button::selectable(
                                active.as_ref() == Some(&document.id),
                                &document.label,
                            ),
                        );
                        if row.clicked() {
                            chosen = Some(document.id.clone());
                            ui.close();
                        }
                    }
                })
                .0
        })
        .inner;
    if let Some(document) = chosen {
        ui.data_mut(|data| data.remove::<String>(filter_id));
        activate_document(state, &document);
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &description)
    });
    ui.painter().rect_filled(
        response.rect,
        0.0,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_panel
        },
    );
    ui.painter().vline(
        response.rect.left(),
        response.rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        response.rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("+{}", hidden.len()),
        theme::sans(tokens::FS_1, FontWeight::Medium),
        if response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_text(description);
}

pub fn is_visible(app: &impl WorkbenchHost) -> bool {
    document_strip_visible(
        app.workbench().workspace,
        visible_documents(app.session()).len(),
    )
}

/// Read-only document metadata used by shell-owned document/session managers.
///
/// The descriptor is deliberately derived from the same authoritative source
/// as the tab strip. Consumers cannot mutate project documents by editing a
/// cached presentation model.
#[derive(Debug, Clone)]
pub struct WorkspaceDocumentDescriptor {
    pub id: WorkspaceDocumentId,
    pub label: String,
    pub dirty: bool,
    pub active: bool,
    pub open: bool,
    pub closable: bool,
    pub(crate) owner: crate::workbench::lifecycle::window_session::ApplicationWindowId,
}

pub fn document_descriptors(state: &impl DocumentSession) -> Vec<WorkspaceDocumentDescriptor> {
    let available = available_documents(state);
    let active = authoritative_active_document(state, &available);
    available
        .into_iter()
        .enumerate()
        .map(|(index, document)| WorkspaceDocumentDescriptor {
            open: index == 0
                || document.id.is_pinned()
                || !state.workbench().documents.is_closed(&document.id)
                || active.as_ref() == Some(&document.id),
            active: active.as_ref() == Some(&document.id),
            closable: index > 0 && !document.id.is_pinned(),
            owner: state.workbench().window_session.owner(&document.id),
            id: document.id,
            label: document.label,
            dirty: document.dirty,
        })
        .collect()
}

pub fn all_document_descriptors(state: &impl DocumentSession) -> Vec<WorkspaceDocumentDescriptor> {
    Workspace::ALL
        .into_iter()
        .flat_map(|workspace| {
            available_documents_for_workspace(state, workspace)
                .into_iter()
                .enumerate()
                .map(move |(index, document)| {
                    let owner = state.workbench().window_session.owner(&document.id);
                    let active = state
                        .workbench()
                        .window_session
                        .state(owner)
                        .and_then(|window| window.active_document.as_ref())
                        == Some(&document.id);
                    WorkspaceDocumentDescriptor {
                        open: index == 0
                            || document.id.is_pinned()
                            || !state.workbench().documents.is_closed(&document.id)
                            || active,
                        active,
                        closable: index > 0 && !document.id.is_pinned(),
                        owner,
                        id: document.id,
                        label: document.label,
                        dirty: document.dirty,
                    }
                })
        })
        .collect()
}

pub fn active_document_id(state: &impl DocumentSession) -> Option<WorkspaceDocumentId> {
    let available = owned_available_documents(state);
    authoritative_active_document(state, &available)
}

/// The active document of the workspace `workbench` shows, as
/// [`active_document_id`] resolves it, for a caller that holds the session,
/// the model, and that workspace's `documents` and `page_document` rather
/// than the whole host.
pub fn active_document_among(
    workbench: &WorkbenchSession,
    model: &Model,
    documents: Vec<WorkspaceDocument>,
    page_document: Option<WorkspaceDocumentId>,
) -> Option<WorkspaceDocumentId> {
    let available = owned_documents(workbench, documents);
    resolve_active_document(workbench, model, page_document, &available)
}

pub fn activate_document_by_id(
    state: &mut impl DocumentSession,
    document: &WorkspaceDocumentId,
) -> bool {
    activate_document(state, document)
}

pub fn cycle_document(state: &mut impl DocumentSession, previous: bool) -> bool {
    reconcile_document_registry(state);
    let documents = visible_documents(state);
    if documents.len() < 2 {
        return false;
    }
    let active = authoritative_active_document(state, &documents);
    let current = active
        .as_ref()
        .and_then(|active| documents.iter().position(|document| document.id == *active))
        .unwrap_or_default();
    let next = if previous {
        (current + documents.len() - 1) % documents.len()
    } else {
        (current + 1) % documents.len()
    };
    activate_document(state, &documents[next].id)
}

pub fn close_other_documents(state: &mut impl DocumentSession) -> usize {
    reconcile_document_registry(state);
    let documents = owned_available_documents(state);
    let active = authoritative_active_document(state, &documents);
    let mut closed = 0;
    for document in &documents {
        if document_is_closable(state, &document.id) && active.as_ref() != Some(&document.id) {
            let was_closed = state.workbench().documents.is_closed(&document.id);
            state.workbench_mut().documents.close(&document.id);
            closed += usize::from(!was_closed);
        }
    }
    closed
}

pub fn close_all_documents(state: &mut impl DocumentSession) -> usize {
    if !state.commit_pending_edit() {
        return 0;
    }
    reconcile_document_registry(state);
    let documents = owned_available_documents(state);
    let root = documents
        .iter()
        .find(|document| !document_is_closable(state, &document.id))
        .or_else(|| documents.first())
        .map(|document| document.id.clone());
    let Some(root) = root else {
        return 0;
    };
    if !activate_document(state, &root) {
        return 0;
    }
    let mut closed = 0;
    for document in &documents {
        if !document_is_closable(state, &document.id) {
            continue;
        }
        let was_closed = state.workbench().documents.is_closed(&document.id);
        state.workbench_mut().documents.close(&document.id);
        closed += usize::from(!was_closed);
    }
    closed
}

fn available_documents(state: &impl DocumentSession) -> Vec<WorkspaceDocument> {
    available_documents_for_workspace(state, state.workbench().workspace)
}

pub fn available_documents_for_workspace(
    state: &impl DocumentSession,
    workspace: Workspace,
) -> Vec<WorkspaceDocument> {
    #[cfg(any(test, feature = "test-support"))]
    DOCUMENT_LISTS.with(|lists| lists.set(lists.get() + 1));
    state.workspace_documents(workspace)
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Times this thread listed a workspace's documents.
    pub static DOCUMENT_LISTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Tabs this thread laid out and painted.
    pub static TABS_LAID_OUT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn owned_available_documents(state: &impl DocumentSession) -> Vec<WorkspaceDocument> {
    owned_documents(state.workbench(), available_documents(state))
}

/// The `documents` the current window holds, in its order, the pinned ones
/// first: the generated netlist and the Added lines lead the strip however
/// late they were built.
fn owned_documents(
    workbench: &WorkbenchSession,
    documents: Vec<WorkspaceDocument>,
) -> Vec<WorkspaceDocument> {
    let current = workbench.window_session.current();
    let order = workbench
        .window_session
        .state(current)
        .map(|window| window.documents.clone())
        .unwrap_or_default();
    let mut documents = documents
        .into_iter()
        .filter(|document| workbench.window_session.belongs_to_current(&document.id))
        .collect::<Vec<_>>();
    documents.sort_by_key(|document| {
        (
            !document.id.is_pinned(),
            order
                .iter()
                .position(|candidate| candidate == &document.id)
                .unwrap_or(usize::MAX),
        )
    });
    documents
}

fn document_is_closable(state: &impl DocumentSession, document: &WorkspaceDocumentId) -> bool {
    !document.is_pinned()
        && available_documents_for_workspace(state, document.workspace())
            .first()
            .is_some_and(|root| root.id != *document)
}

/// Whether each of `documents` can be closed, as [`document_is_closable`]
/// answers it, with each workspace's documents listed once rather than once
/// a tab: a strip can hold hundreds of retained runs.
fn closable_documents(state: &impl DocumentSession, documents: &[WorkspaceDocument]) -> Vec<bool> {
    let mut roots = Vec::<(Workspace, Option<WorkspaceDocumentId>)>::new();
    documents
        .iter()
        .map(|document| {
            let workspace = document.id.workspace();
            let at = roots
                .iter()
                .position(|(listed, _)| *listed == workspace)
                .unwrap_or_else(|| {
                    let root = available_documents_for_workspace(state, workspace)
                        .into_iter()
                        .next()
                        .map(|root| root.id);
                    roots.push((workspace, root));
                    roots.len() - 1
                });
            roots[at]
                .1
                .as_ref()
                .is_some_and(|root| *root != document.id)
        })
        .collect()
}

fn authoritative_active_document(
    state: &impl DocumentSession,
    available: &[WorkspaceDocument],
) -> Option<WorkspaceDocumentId> {
    resolve_active_document(
        state.workbench(),
        state.model(),
        state.page_document(state.workbench().workspace),
        available,
    )
}

/// Which of the `available` documents of the workspace `workbench` shows is
/// active: the page's own document, then the window's choice, then the
/// workspace's own candidate, then the window's first.
fn resolve_active_document(
    workbench: &WorkbenchSession,
    model: &Model,
    page_document: Option<WorkspaceDocumentId>,
    available: &[WorkspaceDocument],
) -> Option<WorkspaceDocumentId> {
    if let Some(document) = page_document
        && available.iter().any(|candidate| candidate.id == document)
    {
        return Some(document);
    }
    let current_window = workbench.window_session.current();
    if let Some(document) = workbench
        .window_session
        .active_document(current_window, workbench.workspace)
        .filter(|document| {
            available.iter().any(|candidate| candidate.id == **document)
                && workbench.window_session.owner(document) == current_window
        })
    {
        return Some(document.clone());
    }
    let candidate = match workbench.workspace {
        Workspace::Design => Some(WorkspaceDocumentId::CellView(
            model.workspace.content.active_view.clone(),
        )),
        Workspace::Results => model
            .simulation
            .active_run()
            .map(|run| WorkspaceDocumentId::ResultDataset(run.dataset_id)),
        workspace => workbench.documents.active(workspace).cloned(),
    };
    candidate
        .filter(|candidate| {
            available.iter().any(|document| document.id == *candidate)
                && workbench.window_session.owner(candidate) == current_window
        })
        .or_else(|| {
            available
                .iter()
                .find(|document| workbench.window_session.owner(&document.id) == current_window)
                .map(|document| document.id.clone())
        })
}

pub fn visible_documents(state: &impl DocumentSession) -> Vec<WorkspaceDocument> {
    let available = owned_available_documents(state);
    let active = authoritative_active_document(state, &available);
    available
        .into_iter()
        .enumerate()
        .filter(|(index, document)| {
            *index == 0
                || !state.workbench().documents.is_closed(&document.id)
                || active.as_ref() == Some(&document.id)
        })
        .map(|(_, document)| document)
        .collect()
}

fn reconcile_document_registry(state: &mut impl DocumentSession) {
    let available = available_documents(state);
    let workspace = state.workbench().workspace;
    state.workbench_mut().documents.retain_available(
        workspace,
        available.iter().map(|document| document.id.clone()),
    );
    if let Some(active) = authoritative_active_document(state, &available) {
        state.workbench_mut().documents.activate(active);
    }
    let all_available = Workspace::ALL
        .into_iter()
        .flat_map(|workspace| available_documents_for_workspace(state, workspace))
        .map(|document| document.id)
        .collect::<Vec<_>>();
    state
        .workbench_mut()
        .window_session
        .retain_documents(all_available);
}

/// Whether `workspace`'s strip shows with `document_count` documents open.
/// Results' shows from its first tab, the run the data browser picked: its
/// "+" is where a new result document is made.
pub fn document_strip_visible(workspace: Workspace, document_count: usize) -> bool {
    match workspace {
        Workspace::Design | Workspace::Models | Workspace::Results => document_count > 0,
        _ => document_count > 1,
    }
}

/// The tabs a strip draws: its documents, whether each can be closed, and
/// the width each takes when nothing asks it to share.
struct StripTabs<'a> {
    documents: &'a [WorkspaceDocument],
    closable: &'a [bool],
    natural: &'a [f32],
    /// The rows of a result document's tab menu, in Results.
    menu: Option<&'a [super::title_bar::menu_entries::MenuRow]>,
}

/// Draw every tab, at most `cap` wide when the strip has asked them to
/// share it, and return where each one went, in document order.
///
/// A tab wholly outside the strip's view is given its room and nothing
/// else: no pointer can reach it and no key can focus it, since only the
/// active tab takes focus. The active tab is always drawn, and so is every
/// tab while a reader of the accessibility tree is listening, which must
/// hear them all.
fn document_tabs(
    ui: &mut egui::Ui,
    state: &mut impl DocumentSession,
    tabs: StripTabs<'_>,
    height: f32,
    cap: Option<f32>,
) -> (Vec<Rect>, Option<(WorkspaceDocumentId, TabMenuChoice)>) {
    let StripTabs {
        documents,
        closable,
        natural,
        menu: tab_menu,
    } = tabs;
    let active = authoritative_active_document(state, documents);
    let mut menu = None;
    let max_width = cap.map_or(TAB_MAX_WIDTH, |cap| cap.min(TAB_MAX_WIDTH));
    let listened = ui.ctx().accesskit_node_builder(ui.id(), |_| ()).is_some();
    let view = ui.clip_rect().x_range();
    let mut open = None;
    let mut close = None;
    let mut focus_index = None;
    let mut tab_ids = Vec::with_capacity(documents.len());
    let mut tab_rects = Vec::with_capacity(documents.len());

    for (index, document) in documents.iter().enumerate() {
        let selected = active.as_ref() == Some(&document.id);
        let size = Vec2::new(natural[index].min(max_width), height);
        let left = ui.cursor().min.x;
        if !selected && !listened && (left + size.x < view.min || left > view.max) {
            let (rect, response) = ui
                .push_id(("document", &document.id), |ui| {
                    ui.allocate_exact_size(size, Sense::hover())
                })
                .inner;
            tab_ids.push(response.id);
            tab_rects.push(rect);
            continue;
        }
        let tab = ui
            .push_id(("document", &document.id), |ui| {
                document_tab(
                    ui,
                    DocumentTabPresentation {
                        icon: document.icon,
                        label: &document.label,
                        short_label: document.short_label(),
                        selected,
                        dirty: document.dirty,
                        closable: closable[index],
                        height,
                        max_width,
                    },
                )
            })
            .inner;
        tab_ids.push(tab.response.id);
        tab_rects.push(tab.response.rect);
        if selected {
            reveal_newly_active_tab(ui, &tab.response);
        }

        // A pointer press activates a tab without taking keyboard focus, so
        // the focus ring appears only for a reader who is using the keys.
        // The middle button closes a tab, as its close does.
        if tab.close_clicked || (tab.response.middle_clicked() && closable[index]) {
            close = Some((index, document.id.clone(), tab.response.has_focus()));
        } else if tab.response.clicked() && !selected {
            open = Some(document.id.clone());
        }
        if let WorkspaceDocumentId::VisualizationDocument(_) = document.id
            && let Some(rows) = tab_menu
            && let Some(choice) = result_document_tab_menu(&tab.response, rows)
        {
            menu = Some((document.id.clone(), choice));
        }

        if tab.response.has_focus() {
            let key = ui.input_mut(|input| {
                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                    Some(TabKey::Previous)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                    Some(TabKey::Next)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                    Some(TabKey::First)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                    Some(TabKey::Last)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Delete) {
                    Some(TabKey::Close)
                } else {
                    None
                }
            });
            match key {
                Some(TabKey::Previous) => {
                    focus_index = Some((index + documents.len() - 1) % documents.len())
                }
                Some(TabKey::Next) => focus_index = Some((index + 1) % documents.len()),
                Some(TabKey::First) => focus_index = Some(0),
                Some(TabKey::Last) => focus_index = Some(documents.len() - 1),
                Some(TabKey::Close) if closable[index] => {
                    close = Some((index, document.id.clone(), true))
                }
                Some(TabKey::Close) | None => {}
            }
        }
    }

    if let Some(index) = focus_index {
        open = Some(documents[index].id.clone());
        ui.memory_mut(|memory| memory.request_focus(tab_ids[index]));
    }
    if let Some(document) = open {
        activate_document(state, &document);
    }
    if let Some((closed_index, document, had_focus)) = close {
        let focus_after_close = if closed_index + 1 < documents.len() {
            closed_index + 1
        } else {
            closed_index.saturating_sub(1)
        };
        if close_document(state, &document, documents) && had_focus {
            ui.memory_mut(|memory| memory.request_focus(tab_ids[focus_after_close]));
        }
    }
    (tab_rects, menu)
}

/// The rows of a saved result document's tab menu: its name, closing it or
/// the others, and deleting it, each with the key its command is bound to.
/// Closing never deletes; deleting asks nothing, since Undo puts the
/// document back, and neither renaming nor deleting is offered on a project
/// open read-only.
fn result_document_tab_menu_rows(
    app: &impl WorkbenchHost,
    ctx: &egui::Context,
) -> Vec<super::title_bar::menu_entries::MenuRow> {
    use super::title_bar::menu_entries::{CommandRow, MenuRow};
    use crate::state::command_catalog::CommandKey;
    use crate::workbench::framework::command_registry::CommandAvailability;
    let platform = crate::workbench::app_state::session::shortcuts::runtime_command_platform(ctx);
    let editable = if app.model().safe_mode.project_read_only() {
        CommandAvailability::Disabled("the project is open read-only")
    } else {
        CommandAvailability::Available
    };
    let row = |command: &'static str, label: &'static str, availability: CommandAvailability| {
        let command = CommandKey::new(command);
        MenuRow::Command(CommandRow::new(
            command,
            label,
            app.shortcut_label(command, platform, ctx.os()),
            availability,
        ))
    };
    vec![
        row(RENAME_RESULT_DOCUMENT, "Rename\u{2026}", editable),
        MenuRow::Separator,
        row(CLOSE_DOCUMENT, "Close", CommandAvailability::Available),
        row(
            CLOSE_OTHER_DOCUMENTS,
            "Close others",
            CommandAvailability::Available,
        ),
        MenuRow::Separator,
        row(DELETE_RESULT_DOCUMENT, "Delete", editable),
    ]
}

/// A saved result document's tab menu of `rows`, drawn as the menu bar
/// draws its menus and hung below the tab as they hang below their names,
/// and what the reader chose from it.
fn result_document_tab_menu(
    tab: &egui::Response,
    rows: &[super::title_bar::menu_entries::MenuRow],
) -> Option<TabMenuChoice> {
    let choice = super::title_bar::context_menu_below(tab, "Document tab menu", |_| rows.to_vec())?;
    match choice.command().as_str() {
        RENAME_RESULT_DOCUMENT => Some(TabMenuChoice::Rename),
        CLOSE_DOCUMENT => Some(TabMenuChoice::Close),
        CLOSE_OTHER_DOCUMENTS => Some(TabMenuChoice::CloseOthers),
        DELETE_RESULT_DOCUMENT => Some(TabMenuChoice::Delete),
        _ => None,
    }
}

/// Scroll the active tab into view when it becomes active, whether by a
/// press, Ctrl+Tab or the navigator. Only until it is in view: revealing it
/// every frame would undo every turn of the wheel over the strip, and
/// revealing it once can stop short while the scroll is still animating.
fn reveal_newly_active_tab(ui: &Ui, tab: &egui::Response) {
    let revealed = egui::Id::new("workbench.document_tabs.revealed");
    if ui.ctx().data(|data| data.get_temp::<egui::Id>(revealed)) == Some(tab.id) {
        return;
    }
    tab.scroll_to_me(None);
    let viewport = ui.clip_rect();
    if in_view(viewport, tab.rect) || tab.rect.width() >= viewport.width() {
        ui.ctx().data_mut(|data| data.insert_temp(revealed, tab.id));
    }
}

/// Whether all of `tab` is inside the strip's `viewport` across. Only the
/// horizontal extent counts: the strip scrolls one way, and its viewport can
/// be a fraction of a point shorter than the tabs it holds.
fn in_view(viewport: Rect, tab: Rect) -> bool {
    tab.left() >= viewport.left() - 0.5 && tab.right() <= viewport.right() + 0.5
}

pub fn activate_document(state: &mut impl DocumentSession, document: &WorkspaceDocumentId) -> bool {
    if document.workspace() != state.workbench().workspace {
        return false;
    }
    let activated = state.activate_owned_document(document);
    if activated {
        record_active_document(state.workbench_mut(), document);
    }
    activated
}

/// Make `document`, which its owner has just shown, the active document of
/// its workspace and of the window that holds it.
pub fn record_active_document(workbench: &mut WorkbenchSession, document: &WorkspaceDocumentId) {
    workbench.documents.activate(document.clone());
    let window = workbench.window_session.current();
    let _ = workbench
        .window_session
        .set_active_document(window, document.clone());
}

pub fn close_document(
    state: &mut impl DocumentSession,
    document: &WorkspaceDocumentId,
    documents: &[WorkspaceDocument],
) -> bool {
    let Some(closed_index) = documents
        .iter()
        .position(|candidate| candidate.id == *document)
    else {
        return false;
    };
    if !document_is_closable(state, document) || document.workspace() != state.workbench().workspace
    {
        return false;
    }
    if !state.commit_pending_edit() {
        return false;
    }
    let was_active = authoritative_active_document(state, documents).as_ref() == Some(document);
    let fallback = documents
        .get(closed_index + 1)
        .or_else(|| documents.get(closed_index.saturating_sub(1)))
        .map(|document| document.id.clone());

    let closed = state.close_owned_document(document).unwrap_or_else(|| {
        state.workbench_mut().documents.close(document);
        true
    });
    if closed
        && (was_active || matches!(document, WorkspaceDocumentId::CellView(_)))
        && let Some(fallback) = fallback
    {
        let source = state.workbench().window_session.owner(document);
        let primary = state.workbench().window_session.primary();
        if source != primary {
            let _ = state.workbench_mut().window_session.move_document(
                document.clone(),
                source,
                primary,
            );
        }
        activate_document(state, &fallback);
    }
    closed
}

#[derive(Clone, Copy)]
enum TabKey {
    Previous,
    Next,
    First,
    Last,
    Close,
}

struct DocumentTabResponse {
    response: egui::Response,
    close_clicked: bool,
}

struct DocumentTabPresentation<'a> {
    icon: WorkbenchIcon,
    label: &'a str,
    short_label: Option<&'a str>,
    selected: bool,
    dirty: bool,
    closable: bool,
    height: f32,
    max_width: f32,
}

/// The text a tab paints at `width`: its label while the label fits, else the
/// short form when it has one. A squeezed tab drops what its icon already
/// says before it cuts the name.
pub fn tab_caption<'a>(
    label: &'a str,
    short: Option<&'a str>,
    natural: f32,
    width: f32,
) -> &'a str {
    match short {
        Some(short) if natural > width + 0.5 => short,
        _ => label,
    }
}

/// The width a tab takes when nothing asks it to share: its whole name, up
/// to [`TAB_MAX_WIDTH`].
fn natural_tab_width(painter: &egui::Painter, label: &str, dirty: bool, closable: bool) -> f32 {
    let label_width = painter
        .layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            egui::Color32::WHITE,
        )
        .size()
        .x;
    (TAB_PADDING_X
        + TAB_ICON_SIZE
        + TAB_CONTENT_GAP
        + label_width
        + suffix_width(dirty, closable)
        + TAB_PADDING_X)
        .min(TAB_MAX_WIDTH)
}

fn document_tab(
    ui: &mut egui::Ui,
    presentation: DocumentTabPresentation<'_>,
) -> DocumentTabResponse {
    let DocumentTabPresentation {
        icon,
        label,
        short_label,
        selected,
        dirty,
        closable,
        height,
        max_width,
    } = presentation;
    #[cfg(any(test, feature = "test-support"))]
    TABS_LAID_OUT.with(|tabs| tabs.set(tabs.get() + 1));
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let suffix_width = suffix_width(dirty, closable);
    let natural = natural_tab_width(ui.painter(), label, dirty, closable);
    let width = natural.min(max_width);
    let shown = tab_caption(label, short_label, natural, width);
    let mut sense = Sense::click();
    if !selected {
        sense = sense.difference(Sense::focusable_noninteractive());
    }
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
        let description = match (dirty, closable) {
            (true, true) => "Unsaved changes. Press Delete to close.",
            (true, false) => "Unsaved changes.",
            (false, true) => "Press Delete to close.",
            (false, false) => "Pinned primary document.",
        };
        node.set_description(description);
    });

    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.right(), rect.top() + TAB_ACTIVE_EDGE),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );

    let icon_center_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE * 0.5;
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(icon_center_x, rect.center().y),
            Vec2::splat(TAB_ICON_SIZE),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let label_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE + TAB_CONTENT_GAP;
    let suffix_left = rect.right() - TAB_PADDING_X - suffix_width;
    let label_clip = egui::Rect::from_min_max(
        egui::pos2(label_x, rect.top()),
        egui::pos2(suffix_left.max(label_x), rect.bottom()),
    );
    let painted_label = elide_text(ui, shown, &font, (suffix_left - label_x).max(0.0));
    let shortened = painted_label != label;
    let galley = ui
        .painter()
        .layout_no_wrap(painted_label, font, t.color.text);
    ui.painter().with_clip_rect(label_clip).galley(
        egui::pos2(label_x, rect.center().y - galley.size().y * 0.5),
        galley,
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );

    let mut suffix_x = suffix_left;
    if suffix_width > 0.0 {
        suffix_x += TAB_CONTENT_GAP;
    }
    // The tab's unsaved marker is a live state, not a warning, so it is
    // painted in the secondary text tone rather than in amber.
    let paint_dirty = |center: egui::Pos2| {
        ui.painter()
            .circle_filled(center, TAB_DIRTY_SIZE * 0.5, t.color.text_dim);
    };
    if dirty && !closable {
        paint_dirty(egui::pos2(suffix_x + TAB_DIRTY_SIZE * 0.5, rect.center().y));
    }
    let mut close_clicked = false;
    if closable {
        let close_rect = egui::Rect::from_min_size(
            egui::pos2(suffix_x + 2.0, rect.center().y - TAB_CLOSE_SIZE * 0.5),
            Vec2::splat(TAB_CLOSE_SIZE),
        );
        // accessibility-pointer-shim: the close glyph is a pointer shortcut;
        // the semantic tab exposes the equivalent Delete-key action.
        let close_response = ui.interact(
            close_rect,
            response.id.with("close"),
            Sense::click().difference(Sense::focusable_noninteractive()),
        );
        // An unsaved tab's marker holds the close's place and turns into
        // the close while the pointer is over the tab, as an editor's tab
        // does, so a tab never shows both.
        if dirty && !response.contains_pointer() && !close_response.contains_pointer() {
            paint_dirty(close_rect.center());
        } else {
            if close_response.hovered() {
                ui.painter().rect_filled(close_rect, 2.0, t.color.bg_active);
            }
            ui.painter().text(
                close_rect.center(),
                egui::Align2::CENTER_CENTER,
                "×",
                theme::sans(14.0, FontWeight::Regular),
                if close_response.hovered() {
                    t.color.text
                } else {
                    t.color.text_faint
                },
            );
        }
        close_clicked = close_response.clicked();
    }
    theme::paint_focus_ring(ui, &response, rect);
    let response = if shortened {
        response.on_hover_text(label)
    } else {
        response
    };
    DocumentTabResponse {
        response,
        close_clicked,
    }
}

pub fn suffix_width(dirty: bool, closable: bool) -> f32 {
    match (dirty, closable) {
        (false, false) => 0.0,
        (true, false) => TAB_CONTENT_GAP + TAB_DIRTY_SIZE,
        // An unsaved tab that closes shows its marker in the close's place.
        (_, true) => TAB_CONTENT_GAP + 2.0 + TAB_CLOSE_SIZE,
    }
}

#[cfg(test)]
mod variant_chooser_tests {
    use super::*;
    use crate::state::{AssemblyVariantDraft, VariantInheritance, VariantQualificationPlan};

    fn create(
        catalog: &mut AssemblyVariantCatalog,
        name: &str,
        parent_id: Option<AssemblyVariantId>,
    ) -> AssemblyVariantId {
        catalog
            .create(AssemblyVariantDraft {
                name: name.to_owned(),
                parent_id,
                inheritance: VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
                overrides: std::collections::BTreeMap::new(),
            })
            .unwrap()
    }

    #[test]
    fn the_chooser_lists_each_child_indented_under_its_parent() {
        let mut catalog = AssemblyVariantCatalog::default();
        let industrial = create(&mut catalog, "Industrial", None);
        create(&mut catalog, "Low power", None);
        let automotive = create(&mut catalog, "Automotive", Some(industrial));
        create(&mut catalog, "Racing", Some(automotive));
        let rows = variant_rows(&catalog)
            .into_iter()
            .map(|(_, name, depth)| (name, depth))
            .collect::<Vec<_>>();
        assert_eq!(
            rows,
            [
                ("Industrial".to_owned(), 0),
                ("Automotive".to_owned(), 1),
                ("Racing".to_owned(), 2),
                ("Low power".to_owned(), 0),
            ]
        );
    }
}

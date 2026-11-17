//! The Overview's Design card: the register of what the project is made of.

use std::collections::BTreeSet;

use egui::{Align2, Color32, Rect, Sense, Stroke, Ui, pos2, vec2};

use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::workspace_state::ProjectPage;

use super::super::page::{elide_text, paint_elided};
use super::snapshot::{OpenDocumentSnapshot, OverviewSnapshot};
use super::{
    OverviewIntent, Tone, command_gate, overview_card, paint_tone_mark, singular_or_plural,
};

const REGISTER_ROW_HEIGHT: f32 = 17.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignObjectSnapshot {
    pub key: String,
    pub name: String,
    pub kind: String,
    pub(crate) detail: String,
    pub(crate) tone: Tone,
    pub top: bool,
}

pub(super) fn design_card(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    snapshot: &OverviewSnapshot,
) -> Option<OverviewIntent> {
    let objects = design_objects(snapshot);
    overview_card(ui, "Design", "", |ui| design_register(ui, cx, &objects))
}

fn design_register(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    objects: &[DesignObjectSnapshot],
) -> Option<OverviewIntent> {
    ui.add_space(7.0);
    let mut requested = None;
    // The reference register sizes its name and kind columns to their widest
    // content — `minmax(110px, auto) auto minmax(0, 1fr)` — so a row's detail
    // starts just past its neighbours' names rather than at a fixed fraction
    // of the card that leaves gulfs between related columns.
    let width = ui.available_width().max(1.0);
    let name_column = objects
        .iter()
        .map(|object| {
            let font = theme::mono(
                tokens::FS_0,
                if object.top {
                    FontWeight::SemiBold
                } else {
                    FontWeight::Regular
                },
            );
            let indent = if object.top { 0.0 } else { 16.0 };
            ui.painter()
                .layout_no_wrap(object.name.clone(), font, Color32::WHITE)
                .size()
                .x
                + indent
        })
        .fold(110.0_f32, f32::max)
        .min(width * 0.40);
    let kind_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let kind_column = objects
        .iter()
        .map(|object| {
            ui.painter()
                .layout_no_wrap(object.kind.clone(), kind_font.clone(), Color32::WHITE)
                .size()
                .x
                + 10.0
        })
        .fold(40.0_f32, f32::max)
        .min(width * 0.32);
    for (index, object) in objects.iter().enumerate() {
        design_summary_row(ui, object, name_column, kind_column);
        if index + 1 < objects.len() {
            ui.add_space(5.0);
        }
    }
    ui.add_space(5.0);
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: 14,
            right: 14,
            top: 5,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                let library_command = ProjectCommand::Page(ProjectPage::Library);
                let (library_enabled, library_disabled_reason) =
                    command_gate(cx, library_command.key());
                let library = Button::new("Browse library…")
                    .enabled(library_enabled)
                    .show(ui);
                let library = library_disabled_reason.map_or(library.clone(), |reason| {
                    library.on_disabled_hover_text(reason)
                });
                if library.clicked() {
                    requested = Some(OverviewIntent::Command(library_command.key()));
                }
                let configuration_command = ids::CONFIGURATION_SETS;
                let (configuration_enabled, configuration_disabled_reason) =
                    command_gate(cx, configuration_command);
                let configuration = Button::new("Configurations")
                    .enabled(configuration_enabled)
                    .show(ui);
                let configuration = configuration_disabled_reason
                    .map_or(configuration.clone(), |reason| {
                        configuration.on_disabled_hover_text(reason)
                    });
                if configuration.clicked() {
                    requested = Some(OverviewIntent::Command(configuration_command));
                }
            });
        });
    requested
}

pub fn design_objects(snapshot: &OverviewSnapshot) -> Vec<DesignObjectSnapshot> {
    fn design_key(identity: &str) -> String {
        let identity = identity.trim().replace('\\', "/");
        for suffix in ["/schematic", "/testbench", "/config", "/symbol"] {
            if let Some(stripped) = identity.strip_suffix(suffix) {
                return stripped.to_ascii_lowercase();
            }
        }
        identity.to_ascii_lowercase()
    }

    fn push_unique(
        rows: &mut Vec<DesignObjectSnapshot>,
        keys: &mut BTreeSet<String>,
        row: DesignObjectSnapshot,
    ) {
        if keys.insert(row.key.to_ascii_lowercase()) {
            rows.push(row);
        }
    }

    let mut rows = Vec::with_capacity(5);
    let mut keys = BTreeSet::new();
    if snapshot.netlist_first {
        if let Some(deck) = &snapshot.source_deck {
            push_unique(
                &mut rows,
                &mut keys,
                DesignObjectSnapshot {
                    key: format!("source-deck:{}", deck.name.to_ascii_lowercase()),
                    name: deck.name.clone(),
                    kind: format!("SPICE deck · {}", deck.ownership),
                    detail: format!(
                        "revision {} · {} {} · {} {}{}",
                        deck.revision,
                        deck.line_count,
                        singular_or_plural(deck.line_count, "line", "lines"),
                        deck.dependency_count,
                        singular_or_plural(deck.dependency_count, "include", "includes"),
                        if deck.dependencies_sealed || deck.dependency_count == 0 {
                            ""
                        } else {
                            ", not all found"
                        }
                    ),
                    tone: deck.validation_tone,
                    top: true,
                },
            );
        }
    } else {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: design_key(&snapshot.descriptor_root),
                name: snapshot.descriptor_root.clone(),
                kind: format!(
                    "schematic · {} {}",
                    snapshot.root_sheet_count,
                    singular_or_plural(snapshot.root_sheet_count, "sheet", "sheets")
                ),
                detail: format!(
                    "{} {} · {} {}",
                    snapshot.root_instance_count,
                    singular_or_plural(snapshot.root_instance_count, "instance", "instances"),
                    snapshot.root_net_count,
                    singular_or_plural(snapshot.root_net_count, "net", "nets")
                ),
                tone: Tone::Neutral,
                top: true,
            },
        );
        if snapshot.configuration.configured {
            push_unique(
                &mut rows,
                &mut keys,
                DesignObjectSnapshot {
                    key: design_key(&snapshot.simulation_root),
                    name: snapshot.simulation_root.clone(),
                    kind: "schematic + config".to_owned(),
                    detail: format!("active testbench · {}", snapshot.configuration.state),
                    tone: snapshot.configuration.tone,
                    top: false,
                },
            );
            push_unique(
                &mut rows,
                &mut keys,
                DesignObjectSnapshot {
                    key: design_key(&snapshot.configuration.dut),
                    name: snapshot.configuration.dut.clone(),
                    kind: "design under test".to_owned(),
                    detail: snapshot.configuration.detail.clone(),
                    tone: snapshot.configuration.tone,
                    top: false,
                },
            );
        }
    }
    for document in snapshot.open_documents.iter().filter(|document| {
        matches!(
            document.view_type,
            ViewType::Verilog | ViewType::VerilogA | ViewType::VerilogAms | ViewType::Spice
        )
    }) {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: design_key(&document.reference.display_path()),
                name: document.reference.display_path(),
                kind: document.view_type.display_name().to_owned(),
                detail: document_state(document).to_owned(),
                tone: if document.dirty {
                    Tone::Warn
                } else {
                    Tone::Neutral
                },
                top: false,
            },
        );
        if rows.len() >= 4 {
            break;
        }
    }
    if snapshot.technology_attached {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: format!("technology:{}", snapshot.technology),
                name: snapshot.technology.clone(),
                kind: "technology + model set".to_owned(),
                detail: format!(
                    "{} model {}",
                    snapshot.model_library_count,
                    singular_or_plural(snapshot.model_library_count, "library", "libraries")
                ),
                tone: Tone::Neutral,
                top: false,
            },
        );
    }
    if rows.len() < 5 {
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: format!("design-library:{}", snapshot.descriptor_root),
                name: "Project design libraries".to_owned(),
                kind: "libraries · cells · views".to_owned(),
                detail: format!(
                    "{} {} · {} {} · {} {}",
                    snapshot.library_count,
                    singular_or_plural(snapshot.library_count, "library", "libraries"),
                    snapshot.cell_count,
                    singular_or_plural(snapshot.cell_count, "cell", "cells"),
                    snapshot.view_count,
                    singular_or_plural(snapshot.view_count, "view", "views")
                ),
                tone: Tone::Neutral,
                top: false,
            },
        );
    }
    for document in &snapshot.open_documents {
        if rows.len() >= 5 {
            break;
        }
        // A netlist-first project deliberately leaves its bootstrap schematic
        // and symbol views pristine; naming them here would present documents
        // the project's own contract says do not carry design content.
        if snapshot.netlist_first
            && matches!(
                document.view_type,
                ViewType::Schematic | ViewType::Symbol | ViewType::Testbench
            )
        {
            continue;
        }
        push_unique(
            &mut rows,
            &mut keys,
            DesignObjectSnapshot {
                key: design_key(&document.reference.display_path()),
                name: document.reference.display_path(),
                kind: document.view_type.display_name().to_owned(),
                detail: document_state(document).to_owned(),
                // An unsaved document is the one state a reader acts on; an
                // active one is simply the document they are in.
                tone: if document.dirty {
                    Tone::Warn
                } else {
                    Tone::Neutral
                },
                top: false,
            },
        );
    }
    rows.truncate(5);
    rows
}

/// One row of the design register.
///
/// The reference surface reads this register rather than navigating from it —
/// the destinations it names are reached from the actions below it — so the row
/// is an annotation, not a control.
fn design_summary_row(
    ui: &mut Ui,
    object: &DesignObjectSnapshot,
    name_column: f32,
    kind_column: f32,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, REGISTER_ROW_HEIGHT), Sense::hover());
    let indent = if object.top { 0.0 } else { 16.0 };
    paint_elided(
        ui,
        pos2(
            rect.left() + 14.0 + indent,
            rect.center().y - tokens::FS_0 * 0.5,
        ),
        &object.name,
        theme::mono(
            tokens::FS_0,
            if object.top {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        ),
        t.color.text,
        (name_column - indent).max(1.0),
    );
    // The reference kind chip is an outline only — micro type, 3px radius,
    // no fill — so it reads as an annotation, not a control.
    let kind_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let kind_text = elide_text(ui, &object.kind, &kind_font, (kind_column - 10.0).max(1.0));
    let kind_text_width = ui
        .painter()
        .layout_no_wrap(kind_text.clone(), kind_font.clone(), Color32::WHITE)
        .size()
        .x;
    let kind_rect = Rect::from_min_size(
        pos2(
            rect.left() + 14.0 + name_column + 9.0,
            rect.center().y - (tokens::FS_MICRO + 5.0) * 0.5,
        ),
        vec2(kind_text_width + 10.0, tokens::FS_MICRO + 5.0),
    );
    ui.painter().rect_stroke(
        kind_rect,
        3.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        kind_rect.center(),
        Align2::CENTER_CENTER,
        kind_text,
        kind_font,
        t.color.text_dim,
    );
    // Detail is faint annotation text in the reference. A genuine warning or
    // error takes a mark in front of it so it stands out without the words
    // themselves being coloured.
    let detail_left = rect.left() + 14.0 + name_column + 9.0 + kind_column + 9.0;
    let mark = paint_tone_mark(ui, object.tone, detail_left, rect.center().y);
    paint_elided(
        ui,
        pos2(detail_left + mark, rect.center().y - tokens::FS_MICRO * 0.5),
        &object.detail,
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
        (rect.right() - 14.0 - detail_left - mark).max(1.0),
    );
    // The row paints its three columns straight to the painter, so the label
    // has to be assembled here or a screen reader announces nothing.
    let row_label = format!("{}, {}, {}", object.name, object.kind, object.detail);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), row_label.clone())
    });
    response.on_hover_text(format!(
        "{}\n{}\n{}",
        object.name, object.kind, object.detail
    ));
}

fn document_state(document: &OpenDocumentSnapshot) -> &'static str {
    if document.dirty {
        "modified"
    } else if document.active {
        "active"
    } else {
        "saved"
    }
}

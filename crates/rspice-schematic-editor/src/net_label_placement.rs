//! Net-label and off-sheet connector fields over the current placement view.

use crate::view::grid::snap_label;
use egui::{Frame, Response, Stroke, TextEdit, Ui, Vec2};
use rspice_design::schematic::{
    document_policy::{NetNamingPolicy, SchematicGridPitch},
    net_label::NetLabelKind,
};
use rspice_design_model::{Point, design_management::CrossSheetPortDirection};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{field_label, read_only_value, select};

/// Read-only context resolved by the application; edits affect only name/kind.
pub struct NetLabelPlacementView<'a> {
    pub anchor: Option<Point>,
    pub grid_pitch: SchematicGridPitch,
    pub naming_policy: NetNamingPolicy,
    pub validation_message: Option<&'a str>,
    pub validation_is_error: bool,
}

pub fn name_id() -> egui::Id {
    egui::Id::new(FIELD_ID)
}

const FIELD_ID: &str = "place-net-label-name";
const DIRECTION_FIELD_ID: &str = "place-net-label-direction";
const DIRECTION_LABEL: &str = "Direction";

pub fn show(
    ui: &mut Ui,
    view: NetLabelPlacementView<'_>,
    name: &mut String,
    kind: &mut NetLabelKind,
) -> (Option<egui::Id>, bool) {
    let NetLabelPlacementView {
        anchor,
        grid_pitch,
        naming_policy,
        validation_message,
        validation_is_error,
    } = view;
    let anchor_text = anchor.map_or_else(
        || "anchor unavailable".to_owned(),
        |anchor| {
            format!(
                "({}, {}) \u{00b7} snapped {}",
                anchor.x,
                anchor.y,
                snap_label(grid_pitch)
            )
        },
    );
    read_only_value(ui, "Anchor", &anchor_text);
    ui.add_space(10.0);

    let t = Tokens::get(ui.ctx());
    let response = field_label(ui, "Net name", |ui| {
        ui.add_sized(
            Vec2::new(ui.available_width(), t.metrics.ctl_h),
            TextEdit::singleline(name)
                .id(name_id())
                .font(egui::TextStyle::Monospace)
                .hint_text("for example: vout or DATA[7]")
                .margin(egui::Margin::symmetric(8, 4)),
        )
    });
    configure_name_accessibility(ui, &response, validation_message, validation_is_error);
    let mut changed = response.changed();
    if let Some(direction) = kind.off_sheet_direction() {
        ui.add_space(10.0);
        changed |= direction_field(ui, direction, kind);
    }

    // Reserve a stable validation row so typing never moves the remaining
    // controls or footer.
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), 32.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            if let Some(message) = validation_message {
                ui.add_space(4.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(message)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(if validation_is_error {
                                t.color.err
                            } else {
                                t.color.text_dim
                            }),
                    )
                    .wrap(),
                );
            }
        },
    );

    let policy = match naming_policy {
        NetNamingPolicy::StrictCaseSensitive => "Strict case-sensitive SPICE syntax",
        NetNamingPolicy::SpiceCompatibleRelaxed => "SPICE-compatible relaxed syntax",
    };
    read_only_value(ui, "Document naming policy", policy);
    ui.add_space(10.0);
    let note = if kind.off_sheet_direction().is_some() {
        "Connectors with the same accepted name form one electrical net across every sheet of this cellview. Cancel or Escape leaves the document unchanged."
    } else {
        "Labels with the same accepted name form one electrical net. Cancel or Escape leaves the document unchanged."
    };
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(5.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_min_width((ui.available_width() - 16.0).max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(note)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });

    (Some(response.id), changed)
}

/// The one control the connector adds. It is offered only when the armed tool
/// declared an off-sheet kind, so the plain label transaction is unchanged.
fn direction_field(
    ui: &mut Ui,
    selected: CrossSheetPortDirection,
    kind: &mut NetLabelKind,
) -> bool {
    let options = NetLabelKind::DIRECTIONS
        .map(|direction| NetLabelKind::direction_label(direction).to_owned());
    field_label(ui, DIRECTION_LABEL, |ui| {
        select(
            ui,
            DIRECTION_FIELD_ID,
            DIRECTION_LABEL,
            NetLabelKind::direction_label(selected),
            &options,
            ui.available_width(),
        )
    })
    .is_some_and(|index| {
        *kind = NetLabelKind::OffSheet {
            direction: NetLabelKind::DIRECTIONS[index],
        };
        true
    })
}

fn configure_name_accessibility(
    ui: &Ui,
    response: &Response,
    message: Option<&str>,
    invalid: bool,
) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Net name");
        node.set_description(match message {
            Some(message) => format!("Required electrical net name. {message}"),
            None => "Required electrical net name".to_owned(),
        });
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
}

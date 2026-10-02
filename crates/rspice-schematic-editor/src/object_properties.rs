//! Isolated object-property drafts and form presentation over canonical design values.
//!
//! The host owns document context, live validation and commits; these drafts
//! retain expected values solely for preview, semantic dirty state and validation.

use egui::{Frame, Response, Stroke, TextEdit, Ui, Vec2};
use rspice_design::schematic::{
    bus::{Bus, BusDeclaration, BusPropertyImpact, BusSlice, BusTap, BusTapOrientation},
    design_note::{DesignNote, DesignNoteKind, DesignNoteLayer, DesignReviewState},
    documentation_shape::{
        DocumentationShape, DocumentationShapeGeometry, DocumentationShapeKind,
        DocumentationShapeLayer, arc_parameters, geometry_from_points,
    },
    net_label::NetLabel,
    owned::named_net::NamedNetTarget,
};
use rspice_design_model::Point;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{
    NotePreviewStyle, PreviewPoint, SelectionImpact, SelectionPreview, ShapePreviewStroke, select,
    select_with_response, selection_command_workflow, workflow_preview_status,
};

pub const BUS_DECLARATION_FIELD: &str = "bus-declaration";
pub const TAP_SOURCE_FIELD: &str = "tap-source-bus";
pub const TAP_SLICE_FIELD: &str = "tap-slice";
pub const LABEL_NAME_FIELD: &str = "net-label-name";
pub const LABEL_X_FIELD: &str = "net-label-x";
pub const LABEL_Y_FIELD: &str = "net-label-y";
pub const NAMED_NET_NAME_FIELD: &str = "named-net-name";
const NOTE_KIND_FIELD: &str = "design-note-kind";
pub const NOTE_TEXT_FIELD: &str = "design-note-text";

/// Exact, isolated draft for a selected bus. The durable baseline is retained
/// to guard the eventual commit against stale-object overwrite.
#[derive(Debug, Clone)]
pub struct BusObjectPropertiesDraft {
    pub original: Bus,
    pub declaration: String,
}

/// Exact, isolated draft for a selected typed bus tap.
#[derive(Debug, Clone)]
pub struct BusTapObjectPropertiesDraft {
    pub original: BusTap,
    pub source_bus_id: u64,
    pub slice: String,
    pub orientation: BusTapOrientation,
}

/// Exact, isolated draft for a selected net label. Coordinates are retained as
/// text until Primary so incomplete or out-of-range edits never partially move
/// the electrical attachment point.
#[derive(Debug, Clone)]
pub struct NetLabelObjectPropertiesDraft {
    pub original: NetLabel,
    pub name: String,
    pub x: String,
    pub y: String,
}

/// Exact naming authority for a logical net selected through conductor
/// geometry. Unlike a label draft, this has no glyph position to edit: the
/// transaction updates every captured label/port name while retaining IDs.
#[derive(Debug, Clone)]
pub struct NamedNetObjectPropertiesDraft {
    pub original: NamedNetTarget,
    pub name: String,
}

/// Exact isolated draft for one non-electrical schematic documentation object.
#[derive(Debug, Clone)]
pub struct DesignNoteObjectPropertiesDraft {
    pub original: DesignNote,
    pub kind: DesignNoteKind,
    pub text: String,
    pub review_state: Option<DesignReviewState>,
}

#[derive(Debug, Clone)]
pub struct DocumentationShapeObjectPropertiesDraft {
    pub original: DocumentationShape,
    pub points: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum ObjectPropertiesDraft {
    Bus(BusObjectPropertiesDraft),
    BusTap(BusTapObjectPropertiesDraft),
    NetLabel(NetLabelObjectPropertiesDraft),
    NamedNet(NamedNetObjectPropertiesDraft),
    DesignNote(DesignNoteObjectPropertiesDraft),
    DocumentationShape(DocumentationShapeObjectPropertiesDraft),
}

impl ObjectPropertiesDraft {
    pub fn is_modified(&self) -> bool {
        match self {
            Self::Bus(draft) => {
                let text = draft.declaration.trim();
                let candidate = if text.is_empty() {
                    Some(None)
                } else {
                    BusDeclaration::parse(text).ok().map(Some)
                };
                candidate.map_or_else(
                    || {
                        draft.declaration
                            != draft
                                .original
                                .declaration
                                .as_ref()
                                .map_or_else(String::new, ToString::to_string)
                    },
                    |candidate| candidate != draft.original.declaration,
                )
            }
            Self::BusTap(draft) => {
                let selector_changed = BusSlice::parse(draft.slice.trim()).map_or_else(
                    |_| draft.slice != draft.original.slice.to_string(),
                    |slice| slice != draft.original.slice,
                );
                draft.source_bus_id != draft.original.bus_id
                    || selector_changed
                    || draft.orientation != draft.original.orientation
            }
            Self::NetLabel(draft) => {
                let candidate = draft
                    .x
                    .trim()
                    .parse::<i32>()
                    .ok()
                    .zip(draft.y.trim().parse::<i32>().ok())
                    .map(|(x, y)| {
                        NetLabel::new(draft.original.id, Point::new(x, y), draft.name.trim())
                    });
                candidate.map_or_else(
                    || {
                        draft.name != draft.original.name
                            || draft.x != draft.original.pos.x.to_string()
                            || draft.y != draft.original.pos.y.to_string()
                    },
                    |candidate| candidate != draft.original,
                )
            }
            Self::NamedNet(draft) => draft.name.trim() != draft.original.name,
            Self::DesignNote(draft) => {
                let mut candidate = draft.original.clone();
                candidate
                    .update(draft.kind, draft.text.clone())
                    .map_or_else(
                        |_| true,
                        |_| {
                            if let Some(review_state) = draft.review_state
                                && candidate.set_review_state(review_state).is_err()
                            {
                                return true;
                            }
                            candidate != draft.original
                        },
                    )
            }
            Self::DocumentationShape(draft) => {
                let points: Option<Vec<_>> = draft
                    .points
                    .iter()
                    .map(|(x, y)| {
                        x.trim()
                            .parse::<i32>()
                            .ok()
                            .zip(y.trim().parse::<i32>().ok())
                            .map(|(x, y)| Point::new(x, y))
                    })
                    .collect();
                points
                    .and_then(|points| geometry_from_points(draft.original.kind(), &points).ok())
                    .is_none_or(|geometry| geometry != draft.original.geometry)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ObjectSummary {
    object: String,
    preview: SelectionPreview,
    scope: String,
    effect: String,
    recovery: String,
    geometry: String,
}

fn object_summary(
    draft: &ObjectPropertiesDraft,
    legal: bool,
    bus_impact: Option<BusPropertyImpact>,
) -> ObjectSummary {
    match draft {
        ObjectPropertiesDraft::Bus(draft) => {
            let candidate_label = if draft.declaration.trim().is_empty() {
                "untyped".to_owned()
            } else {
                draft.declaration.trim().to_owned()
            };
            ObjectSummary {
                object: format!("BUS-{} \u{00b7} {candidate_label}", draft.original.id),
                preview: SelectionPreview::Bus {
                    points: preview_points(&draft.original.points),
                    label: candidate_label,
                },
                scope: match bus_impact {
                    Some(impact) if impact.connected_buses == 1 => "one selected bus".to_owned(),
                    Some(impact) => {
                        format!("{} electrically connected buses", impact.connected_buses)
                    }
                    None => "selected connected-bus network".to_owned(),
                },
                effect: if legal {
                    let impact = bus_impact.unwrap_or_default();
                    format!(
                        "{} bus declaration(s) + {} dependent selector(s)",
                        impact.buses_changed, impact.taps_changed
                    )
                } else {
                    "no electrical change until validation passes".to_owned()
                },
                recovery: "one semantic undo record".to_owned(),
                geometry: format!(
                    "{} route vertices remain bit-exact",
                    draft.original.points.len()
                ),
            }
        }
        ObjectPropertiesDraft::BusTap(draft) => ObjectSummary {
            object: format!("TAP-{} \u{00b7} {}", draft.original.id, draft.slice.trim()),
            preview: SelectionPreview::BusTap {
                bus_point: preview_point(draft.original.bus_point),
                connection_point: preview_point(draft.original.connection_point),
                label: draft.slice.trim().to_owned(),
            },
            scope: "one selected typed bus tap".to_owned(),
            effect: if legal {
                "source, selector, and orientation transaction".to_owned()
            } else {
                "no electrical change until validation passes".to_owned()
            },
            recovery: "one semantic undo record".to_owned(),
            geometry: format!(
                "anchors ({}, {}) \u{2192} ({}, {}) remain bit-exact",
                draft.original.bus_point.x,
                draft.original.bus_point.y,
                draft.original.connection_point.x,
                draft.original.connection_point.y
            ),
        },
        ObjectPropertiesDraft::NetLabel(draft) => {
            let position = draft
                .x
                .trim()
                .parse::<i32>()
                .ok()
                .zip(draft.y.trim().parse::<i32>().ok())
                .map_or(draft.original.pos, |(x, y)| Point::new(x, y));
            ObjectSummary {
                object: format!("LABEL-{} \u{00b7} {}", draft.original.id, draft.name.trim()),
                preview: SelectionPreview::NetLabel {
                    position: preview_point(position),
                    label: draft.name.trim().to_owned(),
                },
                scope: "one selected electrical net label".to_owned(),
                effect: if legal {
                    "net identity and attachment-point transaction".to_owned()
                } else {
                    "no electrical change until validation passes".to_owned()
                },
                recovery: "one semantic undo record".to_owned(),
                geometry: format!(
                    "Stable label ID {} remains unchanged at grid coordinate ({}, {})",
                    draft.original.id, position.x, position.y
                ),
            }
        }
        ObjectPropertiesDraft::NamedNet(draft) => ObjectSummary {
            object: format!(
                "NET \u{00b7} {} \u{00b7} {} naming authorit{}",
                draft.name.trim(),
                draft.original.authority_count(),
                if draft.original.authority_count() == 1 {
                    "y"
                } else {
                    "ies"
                }
            ),
            preview: SelectionPreview::NetLabel {
                position: preview_point(draft.original.preview_position),
                label: draft.name.trim().to_owned(),
            },
            scope: "one resolved logical named net".to_owned(),
            effect: if legal {
                format!(
                    "{} label name(s) + {} interface-port name(s)",
                    draft.original.labels.len(),
                    draft.original.ports.len()
                )
            } else {
                "no electrical change until validation passes".to_owned()
            },
            recovery: "one semantic undo record".to_owned(),
            geometry: format!(
                "{} conductor segment ID(s), {} label ID(s), and {} port ID(s) remain unchanged",
                draft.original.wire_ids.len(),
                draft.original.labels.len(),
                draft.original.ports.len()
            ),
        },
        ObjectPropertiesDraft::DesignNote(draft) => ObjectSummary {
            object: format!("NOTE-{} · {}", draft.original.id, draft.kind.label()),
            preview: SelectionPreview::DesignNote {
                position: preview_point(draft.original.pos),
                label: draft.text.trim().to_owned(),
                style: note_preview_style(draft.kind),
            },
            scope: "one selected non-electrical design note".to_owned(),
            effect: if legal {
                "documentation type, content, and review metadata transaction".to_owned()
            } else {
                "no document change until validation passes".to_owned()
            },
            recovery: "one semantic undo record".to_owned(),
            geometry: format!(
                "Stable note ID {} and anchor ({}, {}) remain unchanged on {}",
                draft.original.id,
                draft.original.pos.x,
                draft.original.pos.y,
                DesignNoteLayer::DrawingAnnotation.label()
            ),
        },
        ObjectPropertiesDraft::DocumentationShape(draft) => {
            let geometry =
                shape_candidate_geometry(draft).unwrap_or_else(|| draft.original.geometry.clone());
            ObjectSummary {
                object: format!(
                    "SHAPE-{} \u{b7} {}",
                    draft.original.id,
                    draft.original.kind().label()
                ),
                preview: {
                    let (outline, stroke) = shape_preview_outline(&geometry);
                    SelectionPreview::DocumentationShape {
                        outline,
                        stroke,
                        label: format!("{} documentation shape", draft.original.kind().label()),
                    }
                },
                scope: "one selected non-electrical documentation shape".to_owned(),
                effect: if legal {
                    "exact presentation geometry transaction".to_owned()
                } else {
                    "no document change until validation passes".to_owned()
                },
                recovery: "one semantic undo record".to_owned(),
                geometry: format!(
                    "Stable shape ID {} remains unchanged on {}",
                    draft.original.id,
                    DocumentationShapeLayer::DrawingDocumentation.label()
                ),
            }
        }
    }
}

fn shape_candidate_geometry(
    draft: &DocumentationShapeObjectPropertiesDraft,
) -> Option<DocumentationShapeGeometry> {
    let points: Option<Vec<_>> = draft
        .points
        .iter()
        .map(|(x, y)| {
            x.trim()
                .parse::<i32>()
                .ok()
                .zip(y.trim().parse::<i32>().ok())
                .map(|(x, y)| Point::new(x, y))
        })
        .collect();
    geometry_from_points(draft.original.kind(), &points?).ok()
}

pub fn show(
    ui: &mut Ui,
    draft: &mut ObjectPropertiesDraft,
    bus_impact: Option<BusPropertyImpact>,
    bus_choices: &[(u64, String)],
    validation_message: Option<&str>,
    invalid_field: Option<&'static str>,
) -> (Option<egui::Id>, bool) {
    let summary = object_summary(draft, validation_message.is_none(), bus_impact);
    selection_command_workflow(
        ui,
        "PROP",
        &summary.preview,
        SelectionImpact {
            scope: &summary.scope,
            effect: &summary.effect,
            recovery: &summary.recovery,
        },
        if validation_message.is_some() {
            "attention required"
        } else {
            "scope resolved"
        },
        validation_message.is_none(),
        |ui| {
            read_only_value(ui, "Object", &summary.object);
            ui.add_space(9.0);
            let output = fields_pane(ui, draft, bus_choices, invalid_field);
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(&summary.geometry)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
            ui.add_space(8.0);
            workflow_preview_status(
                ui,
                validation_message.is_none(),
                if validation_message.is_some() {
                    "Transaction blocked"
                } else {
                    "One explicit editor transaction"
                },
                validation_message.unwrap_or(
                    "Locked, hidden, protected, and out-of-hierarchy objects are excluded and reported.",
                ),
            );
            output
        },
    )
}

fn fields_pane(
    ui: &mut Ui,
    draft: &mut ObjectPropertiesDraft,
    bus_choices: &[(u64, String)],
    invalid_field: Option<&'static str>,
) -> (Option<egui::Id>, bool) {
    let focus;
    let mut edited = false;
    match draft {
        ObjectPropertiesDraft::Bus(draft) => {
            let response = text_field(
                ui,
                BUS_DECLARATION_FIELD,
                "Bus declaration",
                &mut draft.declaration,
                "DATA[15:0] or DATA<15:0>",
                false,
                invalid_field == Some(BUS_DECLARATION_FIELD),
            );
            focus = Some(response.id);
            edited |= response.changed();
            field_note(
                ui,
                "Renaming or changing notation rebases attached selectors atomically. Empty is allowed only when no tap depends on the bus.",
            );
        }
        ObjectPropertiesDraft::BusTap(draft) => {
            let (source_id, changed) = source_bus_field(
                ui,
                &mut draft.source_bus_id,
                bus_choices,
                invalid_field == Some(TAP_SOURCE_FIELD),
            );
            focus = Some(source_id);
            edited |= changed;
            let response = text_field(
                ui,
                TAP_SLICE_FIELD,
                "Scalar member or slice",
                &mut draft.slice,
                "DATA[3] or DATA[7:0]",
                true,
                invalid_field == Some(TAP_SLICE_FIELD),
            );
            edited |= response.changed();
            edited |= orientation_field(ui, &mut draft.orientation);
            field_note(
                ui,
                "Stored route anchors are preserved exactly; this transaction changes only typed connectivity and display orientation.",
            );
        }
        ObjectPropertiesDraft::NetLabel(draft) => {
            let response = text_field(
                ui,
                LABEL_NAME_FIELD,
                "Net name",
                &mut draft.name,
                "vout or DATA[7]",
                true,
                invalid_field == Some(LABEL_NAME_FIELD),
            );
            focus = Some(response.id);
            edited |= response.changed();
            edited |= text_field(
                ui,
                LABEL_X_FIELD,
                "Grid X",
                &mut draft.x,
                "0",
                true,
                invalid_field == Some(LABEL_X_FIELD),
            )
            .changed();
            edited |= text_field(
                ui,
                LABEL_Y_FIELD,
                "Grid Y",
                &mut draft.y,
                "0",
                true,
                invalid_field == Some(LABEL_Y_FIELD),
            )
            .changed();
            field_note(
                ui,
                "The stable label ID is preserved. Name and attachment coordinates publish together as one connectivity edit.",
            );
        }
        ObjectPropertiesDraft::NamedNet(draft) => {
            let response = text_field(
                ui,
                NAMED_NET_NAME_FIELD,
                "Net name",
                &mut draft.name,
                "vout or DATA[7]",
                true,
                invalid_field == Some(NAMED_NET_NAME_FIELD),
            );
            focus = Some(response.id);
            edited |= response.changed();
            field_note(
                ui,
                "Every captured label and interface-port stable ID is retained. The logical net is renamed atomically; this command never merges two existing nets.",
            );
        }
        ObjectPropertiesDraft::DesignNote(draft) => {
            field_label(ui, "Type");
            let options = DesignNoteKind::ALL.map(|kind| kind.label().to_owned());
            edited |= select(
                ui,
                NOTE_KIND_FIELD,
                "Design note type",
                draft.kind.label(),
                &options,
                ui.available_width(),
            )
            .is_some_and(|index| {
                let next = DesignNoteKind::ALL[index];
                if next == draft.kind {
                    false
                } else {
                    draft.kind = next;
                    draft.review_state =
                        (next == DesignNoteKind::ReviewNote).then_some(DesignReviewState::Open);
                    true
                }
            });
            ui.add_space(9.0);
            let response =
                design_note_text_field(ui, &mut draft.text, invalid_field == Some(NOTE_TEXT_FIELD));
            focus = Some(response.id);
            edited |= response.changed();
            if draft.kind == DesignNoteKind::ReviewNote {
                ui.add_space(9.0);
                field_label(ui, "Review state");
                let current = draft.review_state.unwrap_or(DesignReviewState::Open);
                let options = DesignReviewState::ALL.map(|state| state.label().to_owned());
                edited |= select(
                    ui,
                    "design-note-review-state",
                    "Review state",
                    current.label(),
                    &options,
                    ui.available_width(),
                )
                .is_some_and(|index| {
                    let next = DesignReviewState::ALL[index];
                    if draft.review_state == Some(next) {
                        false
                    } else {
                        draft.review_state = Some(next);
                        true
                    }
                });
            }
            read_only_value(ui, "Layer", DesignNoteLayer::DrawingAnnotation.label());
            field_note(
                ui,
                "This object remains on the non-electrical annotation layer. Type, text, and governed review state publish together without changing connectivity.",
            );
        }
        ObjectPropertiesDraft::DocumentationShape(draft) => {
            read_only_value(ui, "Type", draft.original.kind().label());
            read_only_value(
                ui,
                "Layer",
                DocumentationShapeLayer::DrawingDocumentation.label(),
            );
            read_only_value(ui, "Electrical connectivity", "none");
            let mut first_focus = None;
            let kind = draft.original.kind();
            for (index, (x, y)) in draft.points.iter_mut().enumerate() {
                let point_name = documentation_point_name(kind, index);
                field_label(ui, &point_name);
                ui.columns(2, |columns| {
                    let x_response = shape_coordinate_field(&mut columns[0], index, "X", x);
                    let y_response = shape_coordinate_field(&mut columns[1], index, "Y", y);
                    first_focus.get_or_insert(x_response.id);
                    edited |= x_response.changed() || y_response.changed();
                });
                ui.add_space(9.0);
            }
            focus = first_focus;
            field_note(
                ui,
                "Coordinates publish together as one exact non-electrical geometry transaction. Type, layer, and connectivity remain invariant.",
            );
        }
    }
    (focus, edited)
}

fn documentation_point_name(kind: DocumentationShapeKind, index: usize) -> String {
    match (kind, index) {
        (DocumentationShapeKind::Rectangle, 0) => "First corner".to_owned(),
        (DocumentationShapeKind::Rectangle, _) => "Opposite corner".to_owned(),
        (DocumentationShapeKind::Line, 0) => "Start".to_owned(),
        (DocumentationShapeKind::Line, _) => "End".to_owned(),
        (DocumentationShapeKind::Arc, 0) => "Start".to_owned(),
        (DocumentationShapeKind::Arc, 1) => "Through".to_owned(),
        (DocumentationShapeKind::Arc, _) => "End".to_owned(),
        (DocumentationShapeKind::Callout, 0) => "Target".to_owned(),
        (DocumentationShapeKind::Callout, 1) => "First box corner".to_owned(),
        (DocumentationShapeKind::Callout, _) => "Opposite box corner".to_owned(),
        (DocumentationShapeKind::Polygon, _) => format!("Vertex {}", index + 1),
    }
}

fn shape_coordinate_field(
    ui: &mut Ui,
    index: usize,
    axis: &'static str,
    value: &mut String,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        Vec2::new(ui.available_width(), t.metrics.ctl_h),
        TextEdit::singleline(value)
            .id_source(("documentation-shape-point", index, axis))
            .font(egui::TextStyle::Monospace)
            .hint_text("0")
            .margin(egui::Margin::symmetric(8, 4)),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(format!("Point {} {axis}", index + 1));
        node.set_description("Exact signed 32-bit schematic coordinate");
    });
    response
}

fn read_only_value(ui: &mut Ui, label: &str, value: &str) {
    field_label(ui, label);
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_app)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(3.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                    .color(t.color.text),
            );
        });
}

fn text_field(
    ui: &mut Ui,
    stable_id: &'static str,
    label: &str,
    value: &mut String,
    hint: &str,
    required: bool,
    invalid: bool,
) -> Response {
    field_label(ui, label);
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        Vec2::new(ui.available_width(), t.metrics.ctl_h),
        TextEdit::singleline(value)
            .id_source(("object-properties", stable_id))
            .font(egui::TextStyle::Monospace)
            .hint_text(hint)
            .margin(egui::Margin::symmetric(8, 4)),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_description(if invalid && required {
            "Required typed engineering value with a validation error"
        } else if invalid {
            "Typed engineering value with a validation error"
        } else if required {
            "Required typed engineering value"
        } else {
            "Typed engineering value"
        });
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
    ui.add_space(9.0);
    response
}

fn design_note_text_field(ui: &mut Ui, value: &mut String, invalid: bool) -> Response {
    field_label(ui, "Text");
    let response = ui.add_sized(
        Vec2::new(ui.available_width(), 72.0),
        TextEdit::multiline(value)
            .id_source(("object-properties", NOTE_TEXT_FIELD))
            .font(egui::TextStyle::Monospace)
            .hint_text("Bias network")
            .margin(egui::Margin::symmetric(8, 6)),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Text");
        node.set_description(if invalid {
            "Required design-note text with a validation error"
        } else {
            "Required design-note text"
        });
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
    ui.add_space(9.0);
    response
}

fn source_bus_field(
    ui: &mut Ui,
    source_bus_id: &mut u64,
    choices: &[(u64, String)],
    invalid: bool,
) -> (egui::Id, bool) {
    field_label(ui, "Source bus");
    let labels: Vec<String> = choices.iter().map(|(_, label)| label.clone()).collect();
    let selected = choices
        .iter()
        .find(|(id, _)| id == source_bus_id)
        .map_or_else(
            || "Source bus unavailable".to_owned(),
            |(_, label)| label.clone(),
        );
    let output = select_with_response(
        ui,
        "object-properties-tap-source-bus",
        "Source bus",
        &selected,
        &labels,
        ui.available_width(),
    );
    ui.ctx().accesskit_node_builder(output.response.id, |node| {
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
            node.set_description("Source bus requires attention");
        } else {
            node.clear_invalid();
        }
    });
    let changed = output
        .picked
        .and_then(|index| choices.get(index))
        .is_some_and(|(id, _)| {
            if *source_bus_id == *id {
                false
            } else {
                *source_bus_id = *id;
                true
            }
        });
    if invalid {
        ui.label(
            egui::RichText::new("Source bus requires attention")
                .color(Tokens::get(ui.ctx()).color.err)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
        );
    }
    ui.add_space(9.0);
    (output.response.id, changed)
}

fn orientation_field(ui: &mut Ui, orientation: &mut BusTapOrientation) -> bool {
    field_label(ui, "Orientation");
    let options = ["Automatic", "Left", "Right", "Up", "Down"].map(str::to_owned);
    let selected = match orientation {
        BusTapOrientation::Automatic => "Automatic",
        BusTapOrientation::Left => "Left",
        BusTapOrientation::Right => "Right",
        BusTapOrientation::Up => "Up",
        BusTapOrientation::Down => "Down",
    };
    let changed = select(
        ui,
        "object-properties-tap-orientation",
        "Bus tap orientation",
        selected,
        &options,
        ui.available_width(),
    )
    .is_some_and(|index| {
        let next = match index {
            1 => BusTapOrientation::Left,
            2 => BusTapOrientation::Right,
            3 => BusTapOrientation::Up,
            4 => BusTapOrientation::Down,
            _ => BusTapOrientation::Automatic,
        };
        if *orientation == next {
            false
        } else {
            *orientation = next;
            true
        }
    });
    ui.add_space(9.0);
    changed
}

fn field_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(4.0);
}

fn field_note(ui: &mut Ui, note: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(note)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

// =============================================================================
// Design-system preview adapters
// =============================================================================
//
// The selection-command widget draws previews from plain coordinate pairs and
// a small presentation vocabulary. Translating the schematic model into that
// vocabulary belongs to the editor; the UI kit stays independent of design data.

/// Project a schematic point into the preview's coordinate pair.
fn preview_point(point: Point) -> PreviewPoint {
    (point.x, point.y)
}

fn preview_points(points: &[Point]) -> Vec<PreviewPoint> {
    points.iter().copied().map(preview_point).collect()
}

/// Map a design-note kind onto how the preview should read it.
fn note_preview_style(kind: DesignNoteKind) -> NotePreviewStyle {
    match kind {
        DesignNoteKind::PlainText => NotePreviewStyle::Muted,
        DesignNoteKind::PropertyDisplay => NotePreviewStyle::Accent,
        DesignNoteKind::RequirementLink => NotePreviewStyle::AccentUnderlined,
        DesignNoteKind::ReviewNote => NotePreviewStyle::Warning,
    }
}

/// Tessellate a documentation shape for preview and say how to stroke it.
///
/// Arcs are flattened to a polyline here so the widget never needs the curve
/// solver. A degenerate arc falls back to its three defining points, which is
/// what the previous in-widget helper did.
fn shape_preview_outline(
    geometry: &DocumentationShapeGeometry,
) -> (Vec<PreviewPoint>, ShapePreviewStroke) {
    match geometry {
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => {
            let Some((cx, cy, radius, start_angle, sweep)) = arc_parameters(*start, *through, *end)
            else {
                return (
                    preview_points(&[*start, *through, *end]),
                    ShapePreviewStroke::Polyline,
                );
            };
            let outline = (0..=48)
                .map(|index| {
                    let angle = start_angle + sweep * f64::from(index) / 48.0;
                    (
                        (cx + radius * angle.cos()).round() as i32,
                        (cy + radius * angle.sin()).round() as i32,
                    )
                })
                .collect();
            (outline, ShapePreviewStroke::Polyline)
        }
        DocumentationShapeGeometry::Rectangle { .. } => (
            preview_points(&geometry.points()),
            ShapePreviewStroke::Rectangle,
        ),
        DocumentationShapeGeometry::Polygon { .. } => (
            preview_points(&geometry.points()),
            ShapePreviewStroke::ClosedPolyline,
        ),
        DocumentationShapeGeometry::Callout { .. } => (
            preview_points(&geometry.points()),
            ShapePreviewStroke::Callout,
        ),
        DocumentationShapeGeometry::Line { .. } => (
            preview_points(&geometry.points()),
            ShapePreviewStroke::Polyline,
        ),
    }
}

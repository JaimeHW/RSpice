//! Mockup-owned Object properties transaction for typed schematic objects.
//!
//! Components use the schema-driven property editor. Buses and bus taps use
//! this host because their editable values participate in cross-object
//! connectivity invariants. Every field lives in an isolated draft and the
//! primary action publishes exactly one guarded undo transaction.

use egui::Context;
use rspice_schematic_editor::requests::EditorRequestSource;

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    Bus, BusDeclaration, BusPropertyImpact, BusSlice, BusTap, BusTapOrientation, DesignNote,
    DesignNoteKind, DesignReviewState, DocumentationShape, DocumentationShapeGeometry, NetLabel,
    Point, SchematicState,
};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::app::RSpiceApp;
use rspice_schematic_editor::object_properties::{
    self, BUS_DECLARATION_FIELD, BusObjectPropertiesDraft, BusTapObjectPropertiesDraft,
    DesignNoteObjectPropertiesDraft, DocumentationShapeObjectPropertiesDraft, LABEL_NAME_FIELD,
    LABEL_X_FIELD, LABEL_Y_FIELD, NAMED_NET_NAME_FIELD, NOTE_TEXT_FIELD,
    NamedNetObjectPropertiesDraft, NetLabelObjectPropertiesDraft, ObjectPropertiesDraft,
    TAP_SLICE_FIELD, TAP_SOURCE_FIELD,
};

const EYEBROW: &str = "EDIT \u{00b7} TYPED PARAMETERS";
const TITLE: &str = "Object properties";
const PRIMARY: &str = "Apply object properties";
const BODY: &str = "Edit identity, model, parameters, orientation, connectivity, display, constraints, and review metadata.";
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No project or result data has been changed.";
const FAILURE_TITLE: &str = "Properties were not applied";
const DIALOG_SIZE: DialogSize = DialogSize::SimulationWorkflow;

#[derive(Debug, Clone)]
enum PropertyCommit {
    Bus {
        expected: Bus,
        declaration: Option<BusDeclaration>,
    },
    BusTap {
        expected: BusTap,
        bus_id: u64,
        slice: BusSlice,
        orientation: BusTapOrientation,
    },
    NetLabel {
        expected: NetLabel,
        name: String,
        position: Point,
    },
    NamedNet {
        expected: crate::workbench::app::NamedNetTarget,
        name: String,
    },
    DesignNote {
        expected: DesignNote,
        kind: DesignNoteKind,
        text: String,
        review_state: Option<DesignReviewState>,
    },
    DocumentationShape {
        expected: DocumentationShape,
        geometry: DocumentationShapeGeometry,
    },
}

#[derive(Debug, Clone)]
enum DraftValidation {
    Incomplete {
        field: &'static str,
        message: String,
    },
    Invalid {
        field: Option<&'static str>,
        message: String,
    },
    /// Boxed: the commit dwarfs the two message-only variants.
    Valid(Box<PropertyCommit>, Option<BusPropertyImpact>),
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_, _))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Incomplete { message, .. } | Self::Invalid { message, .. } => Some(message),
            Self::Valid(_, _) => None,
        }
    }

    fn field(&self) -> Option<&'static str> {
        match self {
            Self::Incomplete { field, .. } => Some(*field),
            Self::Invalid { field, .. } => *field,
            Self::Valid(_, _) => None,
        }
    }

    fn bus_impact(&self) -> Option<BusPropertyImpact> {
        match self {
            Self::Valid(_, impact) => *impact,
            Self::Incomplete { .. } | Self::Invalid { .. } => None,
        }
    }
}

/// Per-dialog derived resolution. Bus validation intentionally exercises the
/// complete connected-network transaction on detached design data; retaining
/// that result by document generation and exact draft source keeps repaints
/// cheap without weakening the fresh validation performed by Primary.
#[derive(Debug, Clone)]
struct CachedDraftResolution {
    key: DraftResolutionKey,
    validation: DraftValidation,
    bus_choices: Vec<(u64, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DraftResolutionKey {
    source: EditorRequestSource,
    view_path: String,
    target_matches_baseline: bool,
    draft_source: DraftResolutionSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DraftResolutionSource {
    Bus {
        original: Bus,
        declaration: String,
    },
    BusTap {
        original: BusTap,
        source_bus_id: u64,
        slice: String,
        orientation: BusTapOrientation,
    },
    NetLabel {
        original: NetLabel,
        name: String,
        x: String,
        y: String,
    },
    NamedNet {
        original_name: String,
        label_ids: Vec<u64>,
        port_ids: Vec<u64>,
        wire_ids: Vec<u64>,
        name: String,
    },
    DesignNote {
        original: DesignNote,
        kind: DesignNoteKind,
        text: String,
        review_state: Option<DesignReviewState>,
    },
    DocumentationShape {
        original: DocumentationShape,
        points: Vec<(String, String)>,
    },
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_object_properties_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.object_properties.open {
            return;
        }
        let Some(draft) = self.state.dialogs.object_properties.draft.as_ref() else {
            // Fail closed instead of retaining an invisible modal owner.
            self.state.dialogs.object_properties.close();
            self.state.push_user_message(ConsoleMessage::warning(
                "Object properties closed because its typed draft was unavailable.".to_owned(),
            ));
            return;
        };

        let authority_error = object_property_session_error(&self.state);
        let (validation, bus_choices) = authority_error.map_or_else(
            || resolve_draft_for_repaint(ctx, &self.state, draft),
            |message| {
                (
                    DraftValidation::Invalid {
                        field: None,
                        message,
                    },
                    Vec::new(),
                )
            },
        );
        let validation_message = validation.message().map(str::to_owned);
        let invalid_field = validation.field();
        let retained_error = self
            .state
            .dialogs
            .object_properties
            .validation_error
            .clone();
        let discard_confirm = self.state.dialogs.object_properties.discard_confirm;
        let can_commit = self.state.dialogs.object_properties.dirty
            && !self.state.schematic_edit_read_only()
            && validation.can_commit();
        let primary_on_enter = !matches!(draft, ObjectPropertiesDraft::DesignNote(_));

        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(BODY)
            .size(DIALOG_SIZE)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(can_commit)
            .primary_on_enter(primary_on_enter)
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        } else if let Some(error) = retained_error.as_deref() {
            dialog = dialog.transaction_state(DialogTransactionTone::Error, FAILURE_TITLE, error);
        }

        let mut edited = false;
        let mut response = dialog.show_transaction(ctx, |ui| {
            let draft = self
                .state
                .dialogs
                .object_properties
                .draft
                .as_mut()
                .expect("open object-properties workflow retains a typed draft");
            let (focus, changed) = object_properties::show(
                ui,
                draft,
                validation.bus_impact(),
                &bus_choices,
                validation_message.as_deref(),
                invalid_field,
            );
            edited = changed;
            focus
        });
        if edited {
            self.state.dialogs.object_properties.mark_edited();
        }

        match response.choice {
            DialogChoice::Primary => {
                let Some(draft) = self.state.dialogs.object_properties.draft.as_ref() else {
                    return;
                };
                let validation = object_property_session_error(&self.state).map_or_else(
                    || validate_draft(&self.state.schematic, draft),
                    |message| DraftValidation::Invalid {
                        field: None,
                        message,
                    },
                );
                match validation {
                    DraftValidation::Valid(commit, _) => {
                        match apply_commit(&mut self.state.schematic, *commit) {
                            Ok(changed) => {
                                let message = if changed {
                                    "Object properties were applied as one undoable transaction."
                                } else {
                                    "Object properties already matched the stored object."
                                };
                                self.state
                                    .push_user_message(ConsoleMessage::info(message.to_owned()));
                                self.state.ui.toasts.success(
                                    ctx,
                                    if changed {
                                        "Properties applied"
                                    } else {
                                        "No changes"
                                    },
                                    message,
                                );
                                self.state.dialogs.object_properties.close();
                            }
                            Err(error) => {
                                self.state.dialogs.object_properties.validation_error =
                                    Some(error.to_string());
                            }
                        }
                    }
                    DraftValidation::Incomplete { message, .. }
                    | DraftValidation::Invalid { message, .. } => {
                        self.state.dialogs.object_properties.validation_error = Some(message);
                    }
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.object_properties.attempt_close();
                if self.state.dialogs.object_properties.open {
                    response.retain_cancel_focus(DialogInitialFocus::Ghost);
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn resolve_draft_for_repaint(
    ctx: &Context,
    state: &crate::workbench::app_state::AppState,
    draft: &ObjectPropertiesDraft,
) -> (DraftValidation, Vec<(u64, String)>) {
    let key = draft_resolution_key(state, draft);
    let cache_id = egui::Id::new("object-properties-draft-resolution-cache");
    if let Some(cached) = ctx.data(|data| data.get_temp::<CachedDraftResolution>(cache_id))
        && cached.key == key
    {
        return (cached.validation, cached.bus_choices);
    }

    let validation = validate_draft(&state.schematic, draft);
    let bus_choices = match draft {
        ObjectPropertiesDraft::Bus(_)
        | ObjectPropertiesDraft::NetLabel(_)
        | ObjectPropertiesDraft::NamedNet(_)
        | ObjectPropertiesDraft::DesignNote(_)
        | ObjectPropertiesDraft::DocumentationShape(_) => Vec::new(),
        ObjectPropertiesDraft::BusTap(draft) => {
            bus_choices(&state.schematic, draft.original.bus_point)
        }
    };
    ctx.data_mut(|data| {
        data.insert_temp(
            cache_id,
            CachedDraftResolution {
                key,
                validation: validation.clone(),
                bus_choices: bus_choices.clone(),
            },
        );
    });
    (validation, bus_choices)
}

fn draft_resolution_key(
    state: &crate::workbench::app_state::AppState,
    draft: &ObjectPropertiesDraft,
) -> DraftResolutionKey {
    let (target_matches_baseline, draft_source) = match draft {
        ObjectPropertiesDraft::Bus(draft) => (
            state
                .schematic
                .document()
                .buses
                .iter()
                .find(|bus| bus.id == draft.original.id)
                == Some(&draft.original),
            DraftResolutionSource::Bus {
                original: draft.original.clone(),
                declaration: draft.declaration.clone(),
            },
        ),
        ObjectPropertiesDraft::BusTap(draft) => (
            state
                .schematic
                .document()
                .bus_taps
                .iter()
                .find(|tap| tap.id == draft.original.id)
                == Some(&draft.original),
            DraftResolutionSource::BusTap {
                original: draft.original.clone(),
                source_bus_id: draft.source_bus_id,
                slice: draft.slice.clone(),
                orientation: draft.orientation,
            },
        ),
        ObjectPropertiesDraft::NetLabel(draft) => (
            state
                .schematic
                .document()
                .net_labels
                .iter()
                .find(|label| label.id == draft.original.id)
                == Some(&draft.original),
            DraftResolutionSource::NetLabel {
                original: draft.original.clone(),
                name: draft.name.clone(),
                x: draft.x.clone(),
                y: draft.y.clone(),
            },
        ),
        ObjectPropertiesDraft::NamedNet(draft) => (
            crate::workbench::app::validate_named_net_rename(
                &state.schematic,
                &draft.original,
                &draft.original.name,
            )
            .is_ok(),
            DraftResolutionSource::NamedNet {
                original_name: draft.original.name.clone(),
                label_ids: draft.original.labels.iter().map(|label| label.id).collect(),
                port_ids: draft.original.ports.iter().map(|port| port.id).collect(),
                wire_ids: draft.original.wire_ids.clone(),
                name: draft.name.clone(),
            },
        ),
        ObjectPropertiesDraft::DesignNote(draft) => (
            state
                .schematic
                .document()
                .design_notes
                .iter()
                .find(|note| note.id == draft.original.id)
                == Some(&draft.original),
            DraftResolutionSource::DesignNote {
                original: draft.original.clone(),
                kind: draft.kind,
                text: draft.text.clone(),
                review_state: draft.review_state,
            },
        ),
        ObjectPropertiesDraft::DocumentationShape(draft) => (
            state
                .schematic
                .document()
                .documentation_shapes
                .iter()
                .find(|shape| shape.id == draft.original.id)
                == Some(&draft.original),
            DraftResolutionSource::DocumentationShape {
                original: draft.original.clone(),
                points: draft.points.clone(),
            },
        ),
    };
    DraftResolutionKey {
        source: crate::workbench::app::schematic_editor_request_source(state),
        view_path: state.workspace.content.active_view.display_path(),
        target_matches_baseline,
        draft_source,
    }
}

fn object_property_session_error(state: &crate::workbench::app_state::AppState) -> Option<String> {
    let dialog = &state.dialogs.object_properties;
    if state.schematic_edit_read_only() {
        return Some("The active schematic is read-only; no properties can be applied.".to_owned());
    }
    let Some(source) = dialog.source.as_ref() else {
        return Some(
            "The object-properties context is unavailable. Close and reopen the current object."
                .to_owned(),
        );
    };
    let current = crate::workbench::app::schematic_editor_request_source(state);
    if source.design_epoch != current.design_epoch {
        return Some(
            "The design document changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if source.document_epoch != current.document_epoch {
        return Some(
            "The active schematic buffer changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if source.topology_version != current.topology_version {
        return Some(
            "Schematic connectivity changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.view_path != state.workspace.content.active_view.display_path() {
        return Some(
            "The active cell/view changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    (source != &current).then(|| {
        "The schematic source or editing scope changed while properties were open. Close and reopen the current object."
            .to_owned()
    })
}

fn apply_commit(schematic: &mut SchematicState, commit: PropertyCommit) -> Result<bool, String> {
    match commit {
        PropertyCommit::Bus {
            expected,
            declaration,
        } => schematic
            .edit_bus_properties(&expected, declaration)
            .map_err(|error| error.to_string()),
        PropertyCommit::BusTap {
            expected,
            bus_id,
            slice,
            orientation,
        } => schematic
            .edit_bus_tap_properties(
                &expected,
                bus_id,
                expected.bus_point,
                expected.connection_point,
                slice,
                orientation,
            )
            .map_err(|error| error.to_string()),
        PropertyCommit::NetLabel {
            expected,
            name,
            position,
        } => schematic.edit_net_label_properties(expected, name, position),
        PropertyCommit::NamedNet { expected, name } => {
            crate::workbench::app::apply_named_net_rename(schematic, expected, name)
        }
        PropertyCommit::DesignNote {
            expected,
            kind,
            text,
            review_state,
        } => schematic.edit_design_note_properties(expected, kind, text, review_state),
        PropertyCommit::DocumentationShape { expected, geometry } => {
            schematic.edit_documentation_shape_properties(expected, geometry)
        }
    }
}

fn validate_draft(schematic: &SchematicState, draft: &ObjectPropertiesDraft) -> DraftValidation {
    if schematic.session.read_only {
        return DraftValidation::Invalid {
            field: None,
            message: "The active schematic is read-only.".to_owned(),
        };
    }
    match draft {
        ObjectPropertiesDraft::Bus(draft) => validate_bus_draft(schematic, draft),
        ObjectPropertiesDraft::BusTap(draft) => validate_tap_draft(schematic, draft),
        ObjectPropertiesDraft::NetLabel(draft) => validate_net_label_draft(schematic, draft),
        ObjectPropertiesDraft::NamedNet(draft) => validate_named_net_draft(schematic, draft),
        ObjectPropertiesDraft::DesignNote(draft) => validate_design_note_draft(schematic, draft),
        ObjectPropertiesDraft::DocumentationShape(draft) => {
            validate_documentation_shape_draft(schematic, draft)
        }
    }
}

fn validate_named_net_draft(
    schematic: &SchematicState,
    draft: &NamedNetObjectPropertiesDraft,
) -> DraftValidation {
    let candidate = match crate::workbench::app::validate_named_net_rename(
        schematic,
        &draft.original,
        draft.name.trim(),
    ) {
        Ok(candidate) => candidate,
        Err(message) if draft.name.trim().is_empty() => {
            return DraftValidation::Incomplete {
                field: NAMED_NET_NAME_FIELD,
                message,
            };
        }
        Err(message) => {
            return DraftValidation::Invalid {
                field: Some(NAMED_NET_NAME_FIELD),
                message,
            };
        }
    };
    DraftValidation::Valid(
        Box::new(PropertyCommit::NamedNet {
            expected: draft.original.clone(),
            name: candidate,
        }),
        None,
    )
}

fn validate_documentation_shape_draft(
    schematic: &SchematicState,
    draft: &DocumentationShapeObjectPropertiesDraft,
) -> DraftValidation {
    let Some(current) = schematic
        .document()
        .documentation_shapes
        .iter()
        .find(|shape| shape.id == draft.original.id)
    else {
        return stale_validation("The selected documentation shape no longer exists.");
    };
    if current != &draft.original {
        return stale_validation(
            "The selected documentation shape changed while properties were open.",
        );
    }
    let mut points = Vec::with_capacity(draft.points.len());
    for (index, (x, y)) in draft.points.iter().enumerate() {
        let Ok(x) = x.trim().parse::<i32>() else {
            return DraftValidation::Invalid {
                field: None,
                message: format!(
                    "Point {} X must be a whole number in the signed 32-bit coordinate range.",
                    index + 1
                ),
            };
        };
        let Ok(y) = y.trim().parse::<i32>() else {
            return DraftValidation::Invalid {
                field: None,
                message: format!(
                    "Point {} Y must be a whole number in the signed 32-bit coordinate range.",
                    index + 1
                ),
            };
        };
        points.push(Point::new(x, y));
    }
    match crate::state::geometry_from_points(draft.original.kind(), &points) {
        Ok(geometry) => DraftValidation::Valid(
            Box::new(PropertyCommit::DocumentationShape {
                expected: draft.original.clone(),
                geometry,
            }),
            None,
        ),
        Err(error) => DraftValidation::Invalid {
            field: None,
            message: error.to_string(),
        },
    }
}

fn validate_design_note_draft(
    schematic: &SchematicState,
    draft: &DesignNoteObjectPropertiesDraft,
) -> DraftValidation {
    let Some(current) = schematic
        .document()
        .design_notes
        .iter()
        .find(|note| note.id == draft.original.id)
    else {
        return stale_validation("The selected design note no longer exists.");
    };
    if current != &draft.original {
        return stale_validation("The selected design note changed while properties were open.");
    }
    let mut candidate = draft.original.clone();
    match candidate.update(draft.kind, draft.text.clone()) {
        Ok(()) => {
            if let Some(review_state) = draft.review_state
                && let Err(error) = candidate.set_review_state(review_state)
            {
                return DraftValidation::Invalid {
                    field: None,
                    message: error.to_string(),
                };
            }
            DraftValidation::Valid(
                Box::new(PropertyCommit::DesignNote {
                    expected: draft.original.clone(),
                    kind: draft.kind,
                    text: draft.text.trim().to_owned(),
                    review_state: draft.review_state,
                }),
                None,
            )
        }
        Err(error) => DraftValidation::Invalid {
            field: Some(NOTE_TEXT_FIELD),
            message: error.to_string(),
        },
    }
}

fn validate_bus_draft(
    schematic: &SchematicState,
    draft: &BusObjectPropertiesDraft,
) -> DraftValidation {
    let Some(current) = schematic
        .document()
        .buses
        .iter()
        .find(|bus| bus.id == draft.original.id)
    else {
        return stale_validation("The selected bus no longer exists.");
    };
    if current != &draft.original {
        return stale_validation("The selected bus changed while properties were open.");
    }
    let declaration_text = draft.declaration.trim();
    if declaration_text.is_empty() {
        return match schematic.validate_bus_properties(&draft.original, None) {
            Ok(impact) => DraftValidation::Valid(
                Box::new(PropertyCommit::Bus {
                    expected: draft.original.clone(),
                    declaration: None,
                }),
                Some(impact),
            ),
            Err(crate::state::BusParseError::UndeclaredBus) => DraftValidation::Incomplete {
                field: BUS_DECLARATION_FIELD,
                message:
                    "A bus with typed source or destination dependencies must retain a declaration."
                        .to_owned(),
            },
            Err(error) => DraftValidation::Invalid {
                field: Some(BUS_DECLARATION_FIELD),
                message: format!("Bus declaration: {error}."),
            },
        };
    }
    let declaration = match BusDeclaration::parse(declaration_text) {
        Ok(declaration) => declaration,
        Err(error) => {
            return DraftValidation::Invalid {
                field: Some(BUS_DECLARATION_FIELD),
                message: format!("Bus declaration: {error}."),
            };
        }
    };

    // Exercise the domain transaction on a detached clone. This keeps the
    // primary enabled only when all dependent selectors can be rebased.
    match schematic.validate_bus_properties(&draft.original, Some(&declaration)) {
        Ok(impact) => DraftValidation::Valid(
            Box::new(PropertyCommit::Bus {
                expected: draft.original.clone(),
                declaration: Some(declaration),
            }),
            Some(impact),
        ),
        Err(error) => DraftValidation::Invalid {
            field: Some(BUS_DECLARATION_FIELD),
            message: format!("Bus declaration: {error}."),
        },
    }
}

fn validate_tap_draft(
    schematic: &SchematicState,
    draft: &BusTapObjectPropertiesDraft,
) -> DraftValidation {
    let Some(current) = schematic
        .document()
        .bus_taps
        .iter()
        .find(|tap| tap.id == draft.original.id)
    else {
        return stale_validation("The selected bus tap no longer exists.");
    };
    if current != &draft.original {
        return stale_validation("The selected bus tap changed while properties were open.");
    }
    if !schematic
        .document()
        .buses
        .iter()
        .any(|bus| bus.id == draft.source_bus_id)
    {
        return DraftValidation::Invalid {
            field: Some(TAP_SOURCE_FIELD),
            message: "Select a source bus that still exists in this schematic.".to_owned(),
        };
    }
    if draft.slice.trim().is_empty() {
        return DraftValidation::Incomplete {
            field: TAP_SLICE_FIELD,
            message: "Enter one scalar member or contiguous bus slice.".to_owned(),
        };
    }
    let slice = match BusSlice::parse(draft.slice.trim()) {
        Ok(slice) => slice,
        Err(error) => {
            return DraftValidation::Invalid {
                field: Some(TAP_SLICE_FIELD),
                message: format!("Bus tap selector: {error}."),
            };
        }
    };
    match schematic.validate_bus_tap_properties(
        &draft.original,
        draft.source_bus_id,
        draft.original.bus_point,
        draft.original.connection_point,
        slice.clone(),
        draft.orientation,
    ) {
        Ok(_) => DraftValidation::Valid(
            Box::new(PropertyCommit::BusTap {
                expected: draft.original.clone(),
                bus_id: draft.source_bus_id,
                slice,
                orientation: draft.orientation,
            }),
            None,
        ),
        Err(error) => DraftValidation::Invalid {
            field: Some(
                if matches!(error, crate::state::BusParseError::InvalidBusReference) {
                    TAP_SOURCE_FIELD
                } else {
                    TAP_SLICE_FIELD
                },
            ),
            message: format!("Bus tap properties: {error}."),
        },
    }
}

fn validate_net_label_draft(
    schematic: &SchematicState,
    draft: &NetLabelObjectPropertiesDraft,
) -> DraftValidation {
    let Some(current) = schematic
        .document()
        .net_labels
        .iter()
        .find(|label| label.id == draft.original.id)
    else {
        return stale_validation("The selected net label no longer exists.");
    };
    if current != &draft.original {
        return stale_validation("The selected net label changed while properties were open.");
    }

    let name = draft.name.trim();
    if name.is_empty() {
        return DraftValidation::Incomplete {
            field: LABEL_NAME_FIELD,
            message: "Enter the electrical net name assigned at this attachment point.".to_owned(),
        };
    }
    if let Err(reason) =
        NetLabel::validate_name(name, schematic.document().document_policy.net_naming)
    {
        return DraftValidation::Invalid {
            field: Some(LABEL_NAME_FIELD),
            message: format!("Net name: {reason}."),
        };
    }
    let x = match draft.x.trim().parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            return DraftValidation::Invalid {
                field: Some(LABEL_X_FIELD),
                message: "Grid X must be a whole number in the signed 32-bit coordinate range."
                    .to_owned(),
            };
        }
    };
    let y = match draft.y.trim().parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            return DraftValidation::Invalid {
                field: Some(LABEL_Y_FIELD),
                message: "Grid Y must be a whole number in the signed 32-bit coordinate range."
                    .to_owned(),
            };
        }
    };

    DraftValidation::Valid(
        Box::new(PropertyCommit::NetLabel {
            expected: draft.original.clone(),
            name: name.to_owned(),
            position: Point::new(x, y),
        }),
        None,
    )
}

fn stale_validation(message: &str) -> DraftValidation {
    DraftValidation::Invalid {
        field: None,
        message: format!("{message} Close this editor and reopen the current object."),
    }
}

fn bus_choices(
    schematic: &SchematicState,
    retained_anchor: crate::state::Point,
) -> Vec<(u64, String)> {
    let mut choices: Vec<_> = schematic
        .document()
        .buses
        .iter()
        .filter(|bus| bus.contains_point(retained_anchor))
        .filter_map(|bus| {
            bus.declaration
                .as_ref()
                .map(|declaration| (bus.id, format!("{} \u{00b7} BUS-{}", declaration, bus.id)))
        })
        .collect();
    choices.sort_by_key(|(id, _)| *id);
    choices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{BusParseError, BusTargetKind, NetLabel, Point, Wire};

    fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 850.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn declared_bus(id: u64, y: i32, declaration: &str) -> Bus {
        Bus::segment(
            id,
            Point::new(0, y),
            Point::new(20, y),
            Some(BusDeclaration::parse(declaration).unwrap()),
        )
        .unwrap()
    }

    fn open_bus_dialog(app: &mut RSpiceApp, bus: &Bus) {
        app.state.dialogs.object_properties.open_bus(
            bus,
            crate::workbench::app::schematic_editor_request_source(&app.state),
            app.state.workspace.content.active_view.display_path(),
        );
    }

    #[test]
    fn named_net_properties_publish_one_guarded_identity_preserving_undo() {
        let mut schematic = SchematicState::default();
        schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(91, vec![Point::new(0, 0), Point::new(40, 0)]));
        let label = NetLabel::new(92, Point::new(20, 0), "sense");
        schematic
            .document_mut_for_test()
            .net_labels
            .push(label.clone());
        schematic.init_undo_history();
        let draft = NamedNetObjectPropertiesDraft {
            original: crate::workbench::app::NamedNetTarget {
                name: "sense".to_owned(),
                labels: vec![label],
                ports: Vec::new(),
                wire_ids: vec![91],
                preview_position: Point::new(0, 0),
            },
            name: "sense_filtered".to_owned(),
        };

        let DraftValidation::Valid(commit, None) = validate_named_net_draft(&schematic, &draft)
        else {
            panic!("valid named-net properties were rejected")
        };
        assert!(apply_commit(&mut schematic, *commit).unwrap());
        assert_eq!(schematic.document().net_labels[0].id, 92);
        assert_eq!(schematic.document().net_labels[0].name, "sense_filtered");
        assert_eq!(schematic.undo_description(), Some("rename named net"));
        assert!(schematic.undo());
        assert_eq!(schematic.document().net_labels[0].name, "sense");
        assert!(!schematic.undo(), "named-net edit must be one undo step");
    }

    #[test]
    fn repaint_resolution_cache_key_tracks_draft_topology_and_baseline_authority() {
        let mut state = crate::workbench::app_state::AppState::default();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        state
            .schematic
            .document_mut_for_test()
            .buses
            .push(bus.clone());
        let mut draft = ObjectPropertiesDraft::Bus(BusObjectPropertiesDraft {
            original: bus.clone(),
            declaration: "DATA[7:0]".to_owned(),
        });

        let initial = draft_resolution_key(&state, &draft);
        if let ObjectPropertiesDraft::Bus(bus_draft) = &mut draft {
            bus_draft.declaration = "ADDR[7:0]".to_owned();
        }
        assert_ne!(draft_resolution_key(&state, &draft), initial);

        if let ObjectPropertiesDraft::Bus(bus_draft) = &mut draft {
            bus_draft.declaration = "DATA[7:0]".to_owned();
        }
        state.schematic.bump_topology_version();
        let topology_changed = draft_resolution_key(&state, &draft);
        assert_ne!(topology_changed, initial);

        state.schematic.document_mut_for_test().buses[0].declaration =
            Some(BusDeclaration::parse("OTHER[7:0]").unwrap());
        let baseline_changed = draft_resolution_key(&state, &draft);
        assert!(!baseline_changed.target_matches_baseline);
        assert_ne!(baseline_changed, topology_changed);
    }

    #[test]
    fn bus_draft_validation_is_stale_safe_and_rebases_taps() {
        let mut schematic = SchematicState::default();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        schematic.document_mut_for_test().buses.push(bus.clone());
        schematic
            .document_mut_for_test()
            .buses
            .push(declared_bus(3, 5, "DATA[6:4]"));
        schematic.document_mut_for_test().bus_taps.push(
            BusTap::new(
                2,
                &bus,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[6:4]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap(),
        );
        let draft = BusObjectPropertiesDraft {
            original: bus.clone(),
            declaration: "ADDR<0:15>".to_owned(),
        };
        let DraftValidation::Valid(commit, impact) = validate_bus_draft(&schematic, &draft) else {
            panic!("valid retype was rejected")
        };
        let impact = impact.expect("bus impact");
        assert_eq!(impact.connected_buses, 2);
        assert_eq!(impact.buses_changed, 2);
        assert_eq!(impact.taps_changed, 1);
        assert!(apply_commit(&mut schematic, *commit).unwrap());
        assert_eq!(
            schematic.document().bus_taps[0].slice,
            BusSlice::parse("ADDR<4:6>").unwrap()
        );

        let stale = validate_bus_draft(&schematic, &draft);
        assert!(matches!(
            stale,
            DraftValidation::Invalid { field: None, .. }
        ));
    }

    #[test]
    fn tap_draft_preserves_geometry_and_validates_source_and_selector() {
        let mut schematic = SchematicState::default();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        let tap = BusTap::new(
            2,
            &bus,
            Point::new(5, 0),
            Point::new(5, 5),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        schematic.document_mut_for_test().buses.push(bus);
        schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::segment(
                90,
                Point::new(0, 5),
                Point::new(20, 5),
            ));
        schematic.document_mut_for_test().bus_taps.push(tap.clone());
        let draft = BusTapObjectPropertiesDraft {
            original: tap.clone(),
            source_bus_id: 1,
            slice: "DATA[2]".to_owned(),
            orientation: BusTapOrientation::Left,
        };
        let DraftValidation::Valid(commit, None) = validate_tap_draft(&schematic, &draft) else {
            panic!("valid tap edit was rejected")
        };
        assert!(apply_commit(&mut schematic, *commit).unwrap());
        assert_eq!(schematic.document().bus_taps[0].bus_point, tap.bus_point);
        assert_eq!(
            schematic.document().bus_taps[0].connection_point,
            tap.connection_point
        );
        assert_eq!(
            schematic.document().bus_taps[0].target_kind(),
            BusTargetKind::Wire
        );

        let mut invalid = draft;
        invalid.original = schematic.document().bus_taps[0].clone();
        invalid.slice = "DATA[99]".to_owned();
        assert!(matches!(
            validate_tap_draft(&schematic, &invalid),
            DraftValidation::Invalid {
                field: Some(TAP_SLICE_FIELD),
                ..
            }
        ));
    }

    #[test]
    fn net_label_properties_validate_and_commit_name_and_anchor_as_one_undo_step() {
        let original = NetLabel::new(17, Point::new(10, 20), "afe.out");
        let mut schematic = SchematicState::default();
        schematic
            .document_mut_for_test()
            .net_labels
            .push(original.clone());
        schematic.init_undo_history();
        let draft = NetLabelObjectPropertiesDraft {
            original: original.clone(),
            name: "DATA[7]".to_owned(),
            x: "-30".to_owned(),
            y: "45".to_owned(),
        };

        let DraftValidation::Valid(commit, None) = validate_net_label_draft(&schematic, &draft)
        else {
            panic!("valid net-label properties were rejected")
        };
        assert!(apply_commit(&mut schematic, *commit).unwrap());
        assert_eq!(
            schematic.document().net_labels,
            vec![NetLabel::new(17, Point::new(-30, 45), "DATA[7]")]
        );
        assert_eq!(
            schematic.undo_description(),
            Some("edit net label properties")
        );
        assert!(schematic.undo());
        assert_eq!(schematic.document().net_labels, vec![original.clone()]);
        assert!(!schematic.can_undo());

        let mut invalid_name = draft.clone();
        invalid_name.name = "two nodes".to_owned();
        assert!(matches!(
            validate_net_label_draft(&schematic, &invalid_name),
            DraftValidation::Invalid {
                field: Some(LABEL_NAME_FIELD),
                ..
            }
        ));
        let mut invalid_x = draft;
        invalid_x.x = "2147483648".to_owned();
        assert!(matches!(
            validate_net_label_draft(&schematic, &invalid_x),
            DraftValidation::Invalid {
                field: Some(LABEL_X_FIELD),
                ..
            }
        ));
    }

    #[test]
    fn design_note_properties_commit_typed_text_without_changing_topology() {
        let original = DesignNote::new(
            18,
            Point::new(10, 20),
            DesignNoteKind::ReviewNote,
            "Review bias path",
        )
        .unwrap();
        let review_id = original.review.as_ref().unwrap().record_id.clone();
        let mut schematic = SchematicState::default();
        schematic
            .document_mut_for_test()
            .design_notes
            .push(original.clone());
        schematic.init_undo_history();
        let topology = schematic.topology_version();
        let draft = DesignNoteObjectPropertiesDraft {
            original: original.clone(),
            kind: DesignNoteKind::ReviewNote,
            text: "Review updated bias path".to_owned(),
            review_state: Some(DesignReviewState::Resolved),
        };

        let DraftValidation::Valid(commit, None) = validate_design_note_draft(&schematic, &draft)
        else {
            panic!("valid design-note properties were rejected")
        };
        assert!(apply_commit(&mut schematic, *commit).unwrap());
        assert_eq!(
            schematic.document().design_notes[0].text,
            "Review updated bias path"
        );
        assert_eq!(
            schematic.document().design_notes[0]
                .review
                .as_ref()
                .unwrap()
                .state,
            DesignReviewState::Resolved
        );
        assert_eq!(
            schematic.document().design_notes[0]
                .review
                .as_ref()
                .unwrap()
                .record_id,
            review_id
        );
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(
            schematic.undo_description(),
            Some("edit design note properties")
        );
        assert!(schematic.undo());
        assert_eq!(schematic.document().design_notes, vec![original]);

        let invalid = DesignNoteObjectPropertiesDraft {
            original: schematic.document().design_notes[0].clone(),
            kind: DesignNoteKind::RequirementLink,
            text: "REQ 19".to_owned(),
            review_state: None,
        };
        assert!(matches!(
            validate_design_note_draft(&schematic, &invalid),
            DraftValidation::Invalid {
                field: Some(NOTE_TEXT_FIELD),
                ..
            }
        ));
    }

    #[test]
    fn documentation_shape_properties_commit_exact_geometry_as_one_non_electrical_undo_step() {
        let original = DocumentationShape::new(
            19,
            DocumentationShapeGeometry::Arc {
                start: Point::new(0, 10),
                through: Point::new(10, 0),
                end: Point::new(20, 10),
            },
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic
            .document_mut_for_test()
            .documentation_shapes
            .push(original.clone());
        schematic.init_undo_history();
        let topology = schematic.topology_version();
        let draft = DocumentationShapeObjectPropertiesDraft {
            original: original.clone(),
            points: vec![
                ("-5".to_owned(), "12".to_owned()),
                ("10".to_owned(), "-3".to_owned()),
                ("25".to_owned(), "12".to_owned()),
            ],
        };

        let DraftValidation::Valid(commit, None) =
            validate_documentation_shape_draft(&schematic, &draft)
        else {
            panic!("valid documentation-shape properties were rejected")
        };
        assert!(apply_commit(&mut schematic, *commit).unwrap());
        assert_eq!(schematic.document().documentation_shapes[0].id, original.id);
        assert_eq!(
            schematic.document().documentation_shapes[0].layer,
            original.layer
        );
        assert_eq!(
            schematic.document().documentation_shapes[0].geometry,
            DocumentationShapeGeometry::Arc {
                start: Point::new(-5, 12),
                through: Point::new(10, -3),
                end: Point::new(25, 12),
            }
        );
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(
            schematic.undo_description(),
            Some("edit documentation shape properties")
        );
        assert!(schematic.undo());
        assert_eq!(schematic.document().documentation_shapes, vec![original]);
        assert_eq!(schematic.topology_version(), topology);
        assert!(!schematic.can_undo());

        let invalid = DocumentationShapeObjectPropertiesDraft {
            original: schematic.document().documentation_shapes[0].clone(),
            points: vec![
                ("0".to_owned(), "0".to_owned()),
                ("10".to_owned(), "10".to_owned()),
                ("20".to_owned(), "20".to_owned()),
            ],
        };
        assert!(matches!(
            validate_documentation_shape_draft(&schematic, &invalid),
            DraftValidation::Invalid { field: None, .. }
        ));
    }

    #[test]
    fn apply_commit_propagates_read_only_without_mutation() {
        let mut schematic = SchematicState::default();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        schematic.document_mut_for_test().buses.push(bus.clone());
        schematic.session.read_only = true;
        let result = apply_commit(
            &mut schematic,
            PropertyCommit::Bus {
                expected: bus.clone(),
                declaration: Some(BusDeclaration::parse("ADDR[7:0]").unwrap()),
            },
        );
        assert_eq!(result, Err(BusParseError::ReadOnly.to_string()));
        assert_eq!(schematic.document().buses[0], bus);
    }

    #[test]
    fn retained_properties_reject_changed_context_without_committing() {
        for change in [
            "project",
            "view",
            "design",
            "buffer",
            "occurrence",
            "sheet",
            "sheet-revision",
            "content",
            "topology",
            "symbol-context",
            "read-only",
            "safe-mode",
            "missing",
        ] {
            let ctx = Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut app = RSpiceApp::test_instance();
            let master = crate::state::CellViewRef::new("work", "property_parent", "schematic");
            app.state.workspace.descend_into(
                "X1".to_owned(),
                master.clone(),
                crate::state::ViewType::Schematic,
            );
            let first = app
                .state
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
                .unwrap();
            let bus = declared_bus(1, 0, "DATA[7:0]");
            let note = DesignNote::new(
                99,
                Point::new(0, 20),
                DesignNoteKind::PlainText,
                "Original note",
            )
            .unwrap();
            app.state
                .schematic
                .document_mut_for_test()
                .buses
                .push(bus.clone());
            app.state
                .schematic
                .document_mut_for_test()
                .design_notes
                .push(note.clone());
            app.state.schematic.init_undo_history();
            open_bus_dialog(&mut app, &bus);
            assert!(object_property_session_error(&app.state).is_none());
            let Some(ObjectPropertiesDraft::Bus(draft)) =
                app.state.dialogs.object_properties.draft.as_mut()
            else {
                panic!("bus draft")
            };
            draft.declaration = "ADDR[7:0]".to_owned();
            app.state.dialogs.object_properties.mark_edited();
            let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
                app.render_object_properties_dialog(ctx)
            });
            let cached = ctx
                .data(|data| {
                    data.get_temp::<CachedDraftResolution>(egui::Id::new(
                        "object-properties-draft-resolution-cache",
                    ))
                })
                .expect("valid initial draft warms the repaint cache");
            assert!(cached.validation.can_commit());
            match change {
                "project" => app.state.workspace.content.project = Default::default(),
                "view" => {
                    app.state.workspace.content.active_view =
                        crate::state::CellViewRef::new("work", "replacement", "schematic")
                }
                "design" => app.state.design_execution_epoch += 1,
                "buffer" => app.state.active_schematic_epoch += 1,
                "occurrence" => {
                    app.state.workspace.ascend_one().unwrap();
                    app.state.workspace.descend_into(
                        "X2".to_owned(),
                        master.clone(),
                        crate::state::ViewType::Schematic,
                    );
                }
                "sheet" | "sheet-revision" => {
                    let catalog = app
                        .state
                        .workspace
                        .content
                        .design_management
                        .sheet_catalog_mut(&master.key())
                        .unwrap();
                    let second = catalog
                        .create_sheet(
                            crate::state::SheetDefinition {
                                name: "Sheet 2".to_owned(),
                                template: crate::state::SheetTemplate::AnalogSchematic,
                                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                                explicit_page_number: Some(2),
                            },
                            Some(first),
                        )
                        .unwrap();
                    if change == "sheet" {
                        catalog.set_active(second).unwrap();
                    }
                }
                "content" => {
                    let topology = app.state.schematic.topology_version();
                    assert!(
                        app.state
                            .schematic
                            .edit_design_note_properties(
                                note,
                                DesignNoteKind::PlainText,
                                "Changed note".to_owned(),
                                None,
                            )
                            .unwrap()
                    );
                    assert_eq!(app.state.schematic.topology_version(), topology);
                }
                "topology" => app.state.schematic.bump_topology_version(),
                "symbol-context" => {
                    app.state
                        .workspace
                        .content
                        .schematic_buffers
                        .insert("work/other/schematic".to_owned(), Default::default());
                }
                "read-only" => app.state.schematic.session.read_only = true,
                "safe-mode" => app.state.workbench.safe_mode.activate(
                    crate::workbench::state::LocalSafeModeOptions {
                        open_project_read_only: true,
                        ..Default::default()
                    },
                    "object-properties test".to_owned(),
                ),
                "missing" => app.state.dialogs.object_properties.source = None,
                _ => unreachable!(),
            }
            assert!(
                object_property_session_error(&app.state).is_some(),
                "{change}"
            );
            if !matches!(change, "read-only" | "safe-mode" | "missing") {
                let draft = app.state.dialogs.object_properties.draft.as_ref().unwrap();
                assert_ne!(
                    draft_resolution_key(&app.state, draft),
                    cached.key,
                    "{change}"
                );
            }
            let before = crate::state::SchematicSnapshot::capture(app.state.schematic.document());
            let content = app.state.schematic.content_version();
            let topology = app.state.schematic.topology_version();
            let undo = app.state.schematic.undo_description().map(str::to_owned);
            let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
                app.render_object_properties_dialog(ctx)
            });
            assert!(app.state.dialogs.object_properties.open, "{change}");
            assert!(
                before.is_equal_document(app.state.schematic.document()),
                "{change}"
            );
            assert_eq!(app.state.schematic.content_version(), content, "{change}");
            assert_eq!(app.state.schematic.topology_version(), topology, "{change}");
            assert_eq!(
                app.state.schematic.undo_description(),
                undo.as_deref(),
                "{change}"
            );
        }
    }

    #[test]
    fn rendered_primary_revalidates_and_commits_one_undoable_bus_transaction() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        app.state
            .schematic
            .document_mut_for_test()
            .buses
            .push(bus.clone());
        app.state.schematic.clear_undo_history();
        open_bus_dialog(&mut app, &bus);
        let Some(ObjectPropertiesDraft::Bus(draft)) =
            app.state.dialogs.object_properties.draft.as_mut()
        else {
            panic!("bus draft")
        };
        draft.declaration = "ADDR<0:15>".to_owned();
        app.state.dialogs.object_properties.mark_edited();

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_object_properties_dialog(ctx)
        });

        assert!(!app.state.dialogs.object_properties.open);
        assert!(app.state.dialogs.object_properties.source.is_none());
        assert_eq!(
            app.state.schematic.document().buses[0].declaration,
            Some(BusDeclaration::parse("ADDR<0:15>").unwrap())
        );
        assert!(app.state.schematic.undo());
        assert_eq!(app.state.schematic.document().buses[0], bus);
        open_bus_dialog(&mut app, &bus);
        assert!(object_property_session_error(&app.state).is_none());
    }

    #[test]
    fn rendered_escape_requires_explicit_second_discard() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        app.state
            .schematic
            .document_mut_for_test()
            .buses
            .push(bus.clone());
        open_bus_dialog(&mut app, &bus);
        let Some(ObjectPropertiesDraft::Bus(draft)) =
            app.state.dialogs.object_properties.draft.as_mut()
        else {
            panic!("bus draft")
        };
        draft.declaration = "ADDR[7:0]".to_owned();
        app.state.dialogs.object_properties.mark_edited();

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        assert!(app.state.dialogs.object_properties.open);
        assert!(app.state.dialogs.object_properties.discard_confirm);

        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        assert!(!app.state.dialogs.object_properties.open);
    }

    #[test]
    fn accessibility_tree_exposes_modal_fields_and_actions() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        app.state
            .schematic
            .document_mut_for_test()
            .buses
            .push(bus.clone());
        open_bus_dialog(&mut app, &bus);

        let output = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("object properties access tree")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog && node.label() == Some(TITLE)
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::TextInput
                && node.label() == Some("Bus declaration")
        }));
        for label in [PRIMARY, "Cancel"] {
            assert!(nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }));
        }
    }
}

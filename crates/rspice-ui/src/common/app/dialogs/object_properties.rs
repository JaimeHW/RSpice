//! Mockup-owned Object properties transaction for typed schematic objects.
//!
//! Components use the schema-driven property editor. Buses and bus taps use
//! this host because their editable values participate in cross-object
//! connectivity invariants. Every field lives in an isolated draft and the
//! primary action publishes exactly one guarded undo transaction.

use egui::{Context, Frame, Response, Stroke, TextEdit, Ui, Vec2};

use crate::state::{
    Bus, BusDeclaration, BusPropertyImpact, BusSlice, BusTap, BusTapOrientation, DesignNote,
    DesignNoteKind, DesignNoteLayer, DesignReviewState, DocumentationShape,
    DocumentationShapeGeometry, DocumentationShapeLayer, NetLabel, Point, SchematicState,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, SelectionImpact,
    SelectionPreview, select, select_with_response, selection_command_workflow,
    workflow_preview_status,
};

use crate::common::app::{
    BusObjectPropertiesDraft, BusTapObjectPropertiesDraft, ConsoleMessage,
    DesignNoteObjectPropertiesDraft, DocumentationShapeObjectPropertiesDraft,
    NamedNetObjectPropertiesDraft, NetLabelObjectPropertiesDraft, ObjectPropertiesDraft, RSpiceApp,
};

const EYEBROW: &str = "EDIT \u{00b7} TYPED PARAMETERS";
const TITLE: &str = "Object properties";
const PRIMARY: &str = "Apply object properties";
const BODY: &str = "Edit identity, model, parameters, orientation, connectivity, display, constraints, and review metadata.";
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No project or result data has been changed.";
const FAILURE_TITLE: &str = "Properties were not applied";
const BUS_DECLARATION_FIELD: &str = "bus-declaration";
const TAP_SOURCE_FIELD: &str = "tap-source-bus";
const TAP_SLICE_FIELD: &str = "tap-slice";
const LABEL_NAME_FIELD: &str = "net-label-name";
const LABEL_X_FIELD: &str = "net-label-x";
const LABEL_Y_FIELD: &str = "net-label-y";
const NAMED_NET_NAME_FIELD: &str = "named-net-name";
const NOTE_KIND_FIELD: &str = "design-note-kind";
const NOTE_TEXT_FIELD: &str = "design-note-text";
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
        expected: crate::common::app::NamedNetTarget,
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
    Valid(PropertyCommit, Option<BusPropertyImpact>),
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

#[derive(Debug, Clone)]
struct ObjectSummary {
    object: String,
    preview: SelectionPreview,
    scope: String,
    effect: String,
    recovery: String,
    geometry: String,
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
    design_execution_epoch: u64,
    active_schematic_epoch: u64,
    topology_version: u64,
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
    pub(in crate::common::app) fn render_object_properties_dialog(&mut self, ctx: &Context) {
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
        let summary = object_summary(
            &self.state.schematic,
            draft,
            validation.can_commit(),
            validation.bus_impact(),
        );
        let retained_error = self
            .state
            .dialogs
            .object_properties
            .validation_error
            .clone();
        let discard_confirm = self.state.dialogs.object_properties.discard_confirm;
        let can_commit = self.state.dialogs.object_properties.dirty
            && !self.state.schematic.read_only
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
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let draft = self
                .state
                .dialogs
                .object_properties
                .draft
                .as_mut()
                .expect("open object-properties workflow retains a typed draft");
            let (focus, changed) = object_properties_body(
                ui,
                draft,
                &summary,
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

        match choice {
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
                        match apply_commit(&mut self.state.schematic, commit) {
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
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn resolve_draft_for_repaint(
    ctx: &Context,
    state: &crate::common::app::AppState,
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
    state: &crate::common::app::AppState,
    draft: &ObjectPropertiesDraft,
) -> DraftResolutionKey {
    let (target_matches_baseline, draft_source) = match draft {
        ObjectPropertiesDraft::Bus(draft) => (
            state
                .schematic
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
            crate::common::app::validate_named_net_rename(
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
        design_execution_epoch: state.design_execution_epoch,
        active_schematic_epoch: state.active_schematic_epoch,
        topology_version: state.schematic.topology_version(),
        view_path: state.workspace.active_view.display_path(),
        target_matches_baseline,
        draft_source,
    }
}

fn object_property_session_error(state: &crate::common::app::AppState) -> Option<String> {
    let dialog = &state.dialogs.object_properties;
    if state.schematic.read_only || state.active_view_read_only() {
        return Some("The active schematic is read-only; no properties can be applied.".to_owned());
    }
    if dialog.design_execution_epoch != state.design_execution_epoch {
        return Some(
            "The design document changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.active_schematic_epoch != state.active_schematic_epoch {
        return Some(
            "The active schematic buffer changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.topology_version != state.schematic.topology_version() {
        return Some(
            "Schematic connectivity changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    (dialog.view_path != state.workspace.active_view.display_path()).then(|| {
        "The active cell/view changed while properties were open. Close and reopen the current object."
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
        } => {
            let Some(current) = schematic
                .net_labels
                .iter()
                .find(|label| label.id == expected.id)
            else {
                return Err("The selected net label no longer exists.".to_owned());
            };
            if current != &expected {
                return Err("The selected net label changed before commit.".to_owned());
            }
            if expected.name == name && expected.pos == position {
                return Ok(false);
            }
            Ok(
                schematic.with_undo("edit net label properties", move |schematic| {
                    if let Some(label) = schematic
                        .net_labels
                        .iter_mut()
                        .find(|label| label.id == expected.id)
                    {
                        label.name = name;
                        label.pos = position;
                        schematic.is_dirty = true;
                        schematic.bump_topology_version();
                    }
                }),
            )
        }
        PropertyCommit::NamedNet { expected, name } => {
            crate::common::app::apply_named_net_rename(schematic, expected, name)
        }
        PropertyCommit::DesignNote {
            expected,
            kind,
            text,
            review_state,
        } => {
            let Some(current) = schematic
                .design_notes
                .iter()
                .find(|note| note.id == expected.id)
            else {
                return Err("The selected design note no longer exists.".to_owned());
            };
            if current != &expected {
                return Err("The selected design note changed before commit.".to_owned());
            }
            let mut candidate = expected.clone();
            candidate
                .update(kind, text)
                .map_err(|error| error.to_string())?;
            if let Some(review_state) = review_state {
                candidate
                    .set_review_state(review_state)
                    .map_err(|error| error.to_string())?;
            }
            if candidate == expected {
                return Ok(false);
            }
            Ok(
                schematic.with_undo("edit design note properties", move |schematic| {
                    if let Some(note) = schematic
                        .design_notes
                        .iter_mut()
                        .find(|note| note.id == expected.id)
                    {
                        *note = candidate;
                        schematic.is_dirty = true;
                    }
                }),
            )
        }
        PropertyCommit::DocumentationShape { expected, geometry } => {
            let Some(current) = schematic
                .documentation_shapes
                .iter()
                .find(|shape| shape.id == expected.id)
            else {
                return Err("The selected documentation shape no longer exists.".to_owned());
            };
            if current != &expected {
                return Err("The selected documentation shape changed before commit.".to_owned());
            }
            let candidate = DocumentationShape::new(expected.id, geometry)
                .map_err(|error| error.to_string())?;
            if candidate == expected {
                return Ok(false);
            }
            Ok(
                schematic.with_undo("edit documentation shape properties", move |schematic| {
                    if let Some(shape) = schematic
                        .documentation_shapes
                        .iter_mut()
                        .find(|shape| shape.id == expected.id)
                    {
                        *shape = candidate;
                        schematic.is_dirty = true;
                    }
                }),
            )
        }
    }
}

fn validate_draft(schematic: &SchematicState, draft: &ObjectPropertiesDraft) -> DraftValidation {
    if schematic.read_only {
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
    let candidate = match crate::common::app::validate_named_net_rename(
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
        PropertyCommit::NamedNet {
            expected: draft.original.clone(),
            name: candidate,
        },
        None,
    )
}

fn validate_documentation_shape_draft(
    schematic: &SchematicState,
    draft: &DocumentationShapeObjectPropertiesDraft,
) -> DraftValidation {
    let Some(current) = schematic
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
            PropertyCommit::DocumentationShape {
                expected: draft.original.clone(),
                geometry,
            },
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
                PropertyCommit::DesignNote {
                    expected: draft.original.clone(),
                    kind: draft.kind,
                    text: draft.text.trim().to_owned(),
                    review_state: draft.review_state,
                },
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
                PropertyCommit::Bus {
                    expected: draft.original.clone(),
                    declaration: None,
                },
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
            PropertyCommit::Bus {
                expected: draft.original.clone(),
                declaration: Some(declaration),
            },
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
            PropertyCommit::BusTap {
                expected: draft.original.clone(),
                bus_id: draft.source_bus_id,
                slice,
                orientation: draft.orientation,
            },
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
    if let Err(reason) = NetLabel::validate_name(name, schematic.document_policy.net_naming) {
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
        PropertyCommit::NetLabel {
            expected: draft.original.clone(),
            name: name.to_owned(),
            position: Point::new(x, y),
        },
        None,
    )
}

fn stale_validation(message: &str) -> DraftValidation {
    DraftValidation::Invalid {
        field: None,
        message: format!("{message} Close this editor and reopen the current object."),
    }
}

fn object_summary(
    _schematic: &SchematicState,
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
                    points: draft.original.points.clone(),
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
                bus_point: draft.original.bus_point,
                connection_point: draft.original.connection_point,
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
                    position,
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
                position: draft.original.preview_position,
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
                position: draft.original.pos,
                label: draft.text.trim().to_owned(),
                kind: draft.kind,
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
                preview: SelectionPreview::DocumentationShape {
                    geometry,
                    label: format!("{} documentation shape", draft.original.kind().label()),
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
    crate::state::geometry_from_points(draft.original.kind(), &points?).ok()
}

fn bus_choices(
    schematic: &SchematicState,
    retained_anchor: crate::state::Point,
) -> Vec<(u64, String)> {
    let mut choices: Vec<_> = schematic
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

fn object_properties_body(
    ui: &mut Ui,
    draft: &mut ObjectPropertiesDraft,
    summary: &ObjectSummary,
    bus_choices: &[(u64, String)],
    validation_message: Option<&str>,
    invalid_field: Option<&'static str>,
) -> (Option<egui::Id>, bool) {
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
                "afe_out or DATA[7]",
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
                "afe_out or DATA[7]",
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

fn documentation_point_name(kind: crate::state::DocumentationShapeKind, index: usize) -> String {
    match (kind, index) {
        (crate::state::DocumentationShapeKind::Rectangle, 0) => "First corner".to_owned(),
        (crate::state::DocumentationShapeKind::Rectangle, _) => "Opposite corner".to_owned(),
        (crate::state::DocumentationShapeKind::Line, 0) => "Start".to_owned(),
        (crate::state::DocumentationShapeKind::Line, _) => "End".to_owned(),
        (crate::state::DocumentationShapeKind::Arc, 0) => "Start".to_owned(),
        (crate::state::DocumentationShapeKind::Arc, 1) => "Through".to_owned(),
        (crate::state::DocumentationShapeKind::Arc, _) => "End".to_owned(),
        (crate::state::DocumentationShapeKind::Callout, 0) => "Target".to_owned(),
        (crate::state::DocumentationShapeKind::Callout, 1) => "First box corner".to_owned(),
        (crate::state::DocumentationShapeKind::Callout, _) => "Opposite box corner".to_owned(),
        (crate::state::DocumentationShapeKind::Polygon, _) => format!("Vertex {}", index + 1),
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
            app.state.design_execution_epoch,
            app.state.active_schematic_epoch,
            app.state.schematic.topology_version(),
            app.state.workspace.active_view.display_path(),
        );
    }

    #[test]
    fn mockup_shell_contract_is_exact() {
        assert_eq!(TITLE, "Object properties");
        assert_eq!(EYEBROW, "EDIT · TYPED PARAMETERS");
        assert_eq!(PRIMARY, "Apply object properties");
        assert_eq!(DIALOG_SIZE, DialogSize::SimulationWorkflow);
        assert_eq!(
            BODY,
            "Edit identity, model, parameters, orientation, connectivity, display, constraints, and review metadata."
        );
    }

    #[test]
    fn named_net_properties_publish_one_guarded_identity_preserving_undo() {
        let mut schematic = SchematicState::default();
        schematic
            .wires
            .push(Wire::new(91, vec![Point::new(0, 0), Point::new(40, 0)]));
        let label = NetLabel::new(92, Point::new(20, 0), "sense");
        schematic.net_labels.push(label.clone());
        schematic.init_undo_history();
        let draft = NamedNetObjectPropertiesDraft {
            original: crate::common::app::NamedNetTarget {
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
        assert!(apply_commit(&mut schematic, commit).unwrap());
        assert_eq!(schematic.net_labels[0].id, 92);
        assert_eq!(schematic.net_labels[0].name, "sense_filtered");
        assert_eq!(schematic.undo_description(), Some("rename named net"));
        assert!(schematic.undo());
        assert_eq!(schematic.net_labels[0].name, "sense");
        assert!(!schematic.undo(), "named-net edit must be one undo step");
    }

    #[test]
    fn repaint_resolution_cache_key_tracks_draft_topology_and_baseline_authority() {
        let mut state = crate::common::app::AppState::default();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        state.schematic.buses.push(bus.clone());
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

        state.schematic.buses[0].declaration = Some(BusDeclaration::parse("OTHER[7:0]").unwrap());
        let baseline_changed = draft_resolution_key(&state, &draft);
        assert!(!baseline_changed.target_matches_baseline);
        assert_ne!(baseline_changed, topology_changed);
    }

    #[test]
    fn bus_draft_validation_is_stale_safe_and_rebases_taps() {
        let mut schematic = SchematicState::default();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        schematic.buses.push(bus.clone());
        schematic.buses.push(declared_bus(3, 5, "DATA[6:4]"));
        schematic.bus_taps.push(
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
        assert!(apply_commit(&mut schematic, commit).unwrap());
        assert_eq!(
            schematic.bus_taps[0].slice,
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
        schematic.buses.push(bus);
        schematic.wires.push(crate::state::Wire::segment(
            90,
            Point::new(0, 5),
            Point::new(20, 5),
        ));
        schematic.bus_taps.push(tap.clone());
        let draft = BusTapObjectPropertiesDraft {
            original: tap.clone(),
            source_bus_id: 1,
            slice: "DATA[2]".to_owned(),
            orientation: BusTapOrientation::Left,
        };
        let DraftValidation::Valid(commit, None) = validate_tap_draft(&schematic, &draft) else {
            panic!("valid tap edit was rejected")
        };
        assert!(apply_commit(&mut schematic, commit).unwrap());
        assert_eq!(schematic.bus_taps[0].bus_point, tap.bus_point);
        assert_eq!(schematic.bus_taps[0].connection_point, tap.connection_point);
        assert_eq!(schematic.bus_taps[0].target_kind(), BusTargetKind::Wire);

        let mut invalid = draft;
        invalid.original = schematic.bus_taps[0].clone();
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
        schematic.net_labels.push(original.clone());
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
        assert!(apply_commit(&mut schematic, commit).unwrap());
        assert_eq!(
            schematic.net_labels,
            vec![NetLabel::new(17, Point::new(-30, 45), "DATA[7]")]
        );
        assert_eq!(
            schematic.undo_description(),
            Some("edit net label properties")
        );
        assert!(schematic.undo());
        assert_eq!(schematic.net_labels, vec![original.clone()]);
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
        schematic.design_notes.push(original.clone());
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
        assert!(apply_commit(&mut schematic, commit).unwrap());
        assert_eq!(schematic.design_notes[0].text, "Review updated bias path");
        assert_eq!(
            schematic.design_notes[0].review.as_ref().unwrap().state,
            DesignReviewState::Resolved
        );
        assert_eq!(
            schematic.design_notes[0].review.as_ref().unwrap().record_id,
            review_id
        );
        assert_eq!(schematic.topology_version(), topology);
        assert_eq!(
            schematic.undo_description(),
            Some("edit design note properties")
        );
        assert!(schematic.undo());
        assert_eq!(schematic.design_notes, vec![original]);

        let invalid = DesignNoteObjectPropertiesDraft {
            original: schematic.design_notes[0].clone(),
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
        schematic.documentation_shapes.push(original.clone());
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
        assert!(apply_commit(&mut schematic, commit).unwrap());
        assert_eq!(schematic.documentation_shapes[0].id, original.id);
        assert_eq!(schematic.documentation_shapes[0].layer, original.layer);
        assert_eq!(
            schematic.documentation_shapes[0].geometry,
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
        assert_eq!(schematic.documentation_shapes, vec![original]);
        assert_eq!(schematic.topology_version(), topology);
        assert!(!schematic.can_undo());

        let invalid = DocumentationShapeObjectPropertiesDraft {
            original: schematic.documentation_shapes[0].clone(),
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
        schematic.buses.push(bus.clone());
        schematic.read_only = true;
        let result = apply_commit(
            &mut schematic,
            PropertyCommit::Bus {
                expected: bus.clone(),
                declaration: Some(BusDeclaration::parse("ADDR[7:0]").unwrap()),
            },
        );
        assert_eq!(result, Err(BusParseError::ReadOnly.to_string()));
        assert_eq!(schematic.buses[0], bus);
    }

    #[test]
    fn dialog_session_guard_rejects_replaced_document_and_view() {
        let mut app = RSpiceApp::test_instance();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        app.state.schematic.buses.push(bus.clone());
        open_bus_dialog(&mut app, &bus);
        assert!(object_property_session_error(&app.state).is_none());

        app.state.design_execution_epoch = app.state.design_execution_epoch.wrapping_add(1);
        assert!(object_property_session_error(&app.state).is_some());
        app.state.design_execution_epoch =
            app.state.dialogs.object_properties.design_execution_epoch;

        app.state.dialogs.object_properties.view_path = "replacement/view".to_owned();
        assert!(object_property_session_error(&app.state).is_some());
    }

    #[test]
    fn rendered_primary_revalidates_and_commits_one_undoable_bus_transaction() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        app.state.schematic.buses.push(bus.clone());
        app.state.schematic.clear_undo_history();
        open_bus_dialog(&mut app, &bus);
        let Some(ObjectPropertiesDraft::Bus(draft)) =
            app.state.dialogs.object_properties.draft.as_mut()
        else {
            panic!("bus draft")
        };
        draft.declaration = "ADDR<0:15>".to_owned();
        app.state.dialogs.object_properties.mark_edited();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_object_properties_dialog(ctx)
        });

        assert!(!app.state.dialogs.object_properties.open);
        assert_eq!(
            app.state.schematic.buses[0].declaration,
            Some(BusDeclaration::parse("ADDR<0:15>").unwrap())
        );
        assert!(app.state.schematic.undo());
        assert_eq!(app.state.schematic.buses[0], bus);
    }

    #[test]
    fn rendered_escape_requires_explicit_second_discard() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let bus = declared_bus(1, 0, "DATA[7:0]");
        app.state.schematic.buses.push(bus.clone());
        open_bus_dialog(&mut app, &bus);
        let Some(ObjectPropertiesDraft::Bus(draft)) =
            app.state.dialogs.object_properties.draft.as_mut()
        else {
            panic!("bus draft")
        };
        draft.declaration = "ADDR[7:0]".to_owned();
        app.state.dialogs.object_properties.mark_edited();

        let _ = ctx.run(dialog_input(Vec::new()), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_object_properties_dialog(ctx)
        });
        assert!(app.state.dialogs.object_properties.open);
        assert!(app.state.dialogs.object_properties.discard_confirm);

        let _ = ctx.run(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
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
        app.state.schematic.buses.push(bus.clone());
        open_bus_dialog(&mut app, &bus);

        let output = ctx.run(dialog_input(Vec::new()), |ctx| {
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

//! The design feature: the schematic and symbol canvases, the stimulus
//! library, and the dialogs that edit what they draw.
//!
//! What it has not registered yet, the shell's legacy tables still serve.

pub(crate) mod commands;

use egui::Context;

use crate::workbench::app::actions::stimulus;
use crate::workbench::app::dialogs::connectivity_manager::{
    ConnectivityManager, open_connectivity_manager,
};
use crate::workbench::app::dialogs::design_review_comments::{
    ReviewComments, open_design_review_comments,
};
use crate::workbench::app::dialogs::engineering_table::{
    EngineeringTable, open_engineering_table_dialog,
};
use crate::workbench::app::dialogs::grid_snap_routing::open_grid_snap_routing_dialog;
use crate::workbench::app::dialogs::hierarchy::create::{
    CreateHierarchy, open_create_hierarchy_dialog,
};
use crate::workbench::app::dialogs::hierarchy::descend;
use crate::workbench::app::dialogs::placement::builtin_xspice::BuiltinXspicePlacement;
use crate::workbench::app::dialogs::property_dialog::PropertyDialog;
use crate::workbench::app::dialogs::replace_instance::{
    ReplaceInstance, open_replace_instance_dialog, replace_instance_available,
};
use crate::workbench::app::dialogs::selection::{array, bulk_edit, rename, shared_properties};
use crate::workbench::app::dialogs::{
    create_model_bound_symbol, find_replace, object_properties, placement, rule_setup,
    stimulus_link, symbol_definition, visibility_options,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureRegistry, NoHandles, Reads,
};
use crate::workbench::slots::DesignState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use commands::DesignCommand;

/// The design feature.
pub(crate) struct DesignFeature;

impl Feature for DesignFeature {
    type State = DesignState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<DesignCommand>();
        registry.dialogs(DIALOGS);
    }
}

impl CommandSet for DesignCommand {
    type Feature = DesignFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// The schematic dialogs need a schematic or testbench in front of the
    /// reader; Design rule setup needs only the project, which the registry
    /// already asks for. Rename names what it can act on; Replace instance and
    /// Create hierarchy name the first thing missing: the schematic, an
    /// editable one, then something in it to act on.
    ///
    /// Every library verb needs the instrument open, because they all act on
    /// the definition it is reading; New and Validate need only that,
    /// because a library with nothing in it is exactly where New belongs.
    /// Placing is an edit of the schematic, so a read-only view is the one
    /// extra thing it needs beyond a definition to place.
    fn availability(self, reads: &Reads<'_, DesignFeature>) -> CommandAvailability {
        let schematic = schematic_editor(reads);
        let selected = reads.state.selected_stimulus_definition.is_some();
        let refusal = match self {
            Self::GridSnapRouting | Self::VisibilityOptions | Self::EngineeringTableView => {
                (!schematic).then_some("command is unavailable in this context")
            }
            Self::ConnectivityManager => (!schematic).then_some("open a schematic or testbench"),
            Self::SelectionBulkEdit | Self::ReviewComments => {
                (!schematic).then_some("open a schematic project")
            }
            Self::RuleSetup => None,
            Self::RenameSelection
                if schematic && rename::rename_selection_available(reads.model) =>
            {
                None
            }
            Self::RenameSelection => {
                Some("select one editable component, net label, or declared bus")
            }
            Self::ReplaceInstance | Self::CreateHierarchy if !schematic => {
                Some("open an editable schematic or testbench")
            }
            Self::ReplaceInstance
                if replace_instance_available(&reads.state.property_registry, reads.model) =>
            {
                None
            }
            Self::CreateHierarchy if reads.model.create_hierarchy_available() => None,
            Self::ReplaceInstance | Self::CreateHierarchy
                if reads.model.schematic_edit_read_only() =>
            {
                Some("the active schematic is read-only")
            }
            Self::ReplaceInstance | Self::CreateHierarchy => Some("select an editable object"),
            _ if reads.wb.workspace != Workspace::Stimulus => Some("open the Stimulus Library"),
            Self::NewStimulusDefinition | Self::ValidateStimulusLibrary => None,
            Self::DuplicateStimulusDefinition | Self::DeleteStimulusDefinition => {
                (!selected).then_some("select a stimulus definition")
            }
            Self::PlaceSourceFromDefinition => {
                if reads.model.schematic_edit_read_only() {
                    Some("the active schematic view is read-only")
                } else {
                    (!selected).then_some("select a stimulus definition")
                }
            }
            Self::ApplyStimulusDraft | Self::RevertStimulusDraft => {
                (!stimulus::selected_draft_is_dirty(
                    reads.state.selected_stimulus_definition.as_deref(),
                    &reads.state.stimulus_editor,
                ))
                .then_some("this definition has no draft to apply")
            }
            Self::ShowAdopterOnSchematic => (!stimulus::selected_definition_has_adopter(
                reads.state.selected_stimulus_definition.as_deref(),
                reads.model,
            ))
            .then_some("no placed source has adopted this definition"),
        };
        refusal.map_or(
            CommandAvailability::Available,
            CommandAvailability::Disabled,
        )
    }

    fn execute(self, cx: &mut Cx<'_, DesignFeature>) {
        match self {
            Self::NewStimulusDefinition => stimulus::new_definition(
                &mut cx.state.selected_stimulus_definition,
                &mut cx.state.stimulus_editor,
                cx.model,
                cx.wb,
            ),
            Self::DuplicateStimulusDefinition => {
                stimulus::duplicate_definition(
                    &mut cx.state.selected_stimulus_definition,
                    cx.model,
                    cx.wb,
                );
            }
            Self::DeleteStimulusDefinition => stimulus::delete_definition(
                &mut cx.state.selected_stimulus_definition,
                &mut cx.state.stimulus_editor,
                cx.model,
            ),
            Self::ApplyStimulusDraft => stimulus::apply_draft(
                &mut cx.state.selected_stimulus_definition,
                &mut cx.state.stimulus_editor,
                cx.model,
                cx.wb,
            ),
            Self::RevertStimulusDraft => stimulus::revert_draft(
                cx.state.selected_stimulus_definition.as_deref(),
                &mut cx.state.stimulus_editor,
                cx.model,
            ),
            Self::ValidateStimulusLibrary => stimulus::validate_library(
                &cx.state.stimulus_editor,
                &cx.state.property_registry,
                cx.model,
            ),
            Self::PlaceSourceFromDefinition => {
                stimulus::place_selected_definition(
                    cx.state.selected_stimulus_definition.as_deref(),
                    &cx.state.stimulus_editor,
                    cx.model,
                    cx.wb,
                );
            }
            Self::GridSnapRouting => {
                open_grid_snap_routing_dialog(
                    &mut cx.state.grid_snap_routing,
                    cx.state.grid,
                    cx.model,
                    cx.wb,
                );
            }
            Self::VisibilityOptions => {
                visibility_options::open_schematic_visibility_options(
                    &mut cx.state.schematic_visibility_dialog,
                    cx.state.schematic_visibility,
                    cx.state.drawing_sheet_layers,
                );
            }
            Self::EngineeringTableView => {
                open_engineering_table_dialog(
                    &mut cx.state.engineering_table,
                    &cx.state.engineering_table_views,
                    cx.model,
                );
            }
            Self::ConnectivityManager => open_connectivity_manager(
                &mut cx.state.connectivity_manager,
                cx.model,
                cx.wb,
                cx.platform,
            ),
            Self::SelectionBulkEdit => bulk_edit::open_selection_bulk_edit_dialog(
                &mut cx.state.selection_bulk_edit,
                &cx.state.schematic_bulk_edit_filters,
                cx.model,
            ),
            Self::RuleSetup => {
                rule_setup::open_rule_setup(&mut cx.state.rule_setup, cx.model, cx.wb, cx.platform)
            }
            Self::ReviewComments => open_design_review_comments(
                &mut cx.state.design_review_comments,
                cx.model,
                cx.platform,
            ),
            Self::RenameSelection => {
                rename::open_selected_object_rename(
                    &mut cx.state.rename_selection,
                    cx.state.object_properties.open || cx.state.tabbed_property_dialog.open,
                    cx.model,
                );
            }
            Self::ReplaceInstance => open_replace_instance_dialog(
                &mut cx.state.replace_instance,
                &cx.state.property_registry,
                cx.model,
                cx.wb,
            ),
            Self::CreateHierarchy => {
                open_create_hierarchy_dialog(&mut cx.state.create_hierarchy, cx.model, cx.wb);
            }
            Self::ShowAdopterOnSchematic => {
                stimulus::show_adopter_on_schematic(
                    cx.state.selected_stimulus_definition.as_deref(),
                    cx.model,
                    cx.wb,
                );
                cx.wb.activate(Workspace::Design);
            }
        }
    }
}

/// Whether a schematic or testbench is the document in front of the reader
/// in the Design workspace.
fn schematic_editor(reads: &Reads<'_, DesignFeature>) -> bool {
    reads.wb.workspace == Workspace::Design
        && matches!(
            reads.model.workspace.content.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
}

/// The dialog steps the feature draws, under the keys the shell's dialog
/// order lists them by and in that order. Each is open while its form is;
/// Import symbol definition while either of its two forms is.
const DIALOGS: &[FeatureDialog<DesignFeature>] = &[
    FeatureDialog {
        key: DialogKey("design.property"),
        is_open: |design| design.tabbed_property_dialog.open,
        render: DesignFeature::render_property_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.bus_tap"),
        is_open: |design| design.bus_tap.open,
        render: DesignFeature::render_bus_tap_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.builtin_xspice_placement"),
        is_open: |design| design.builtin_xspice_placement.open,
        render: DesignFeature::render_builtin_xspice_placement_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.net_label"),
        is_open: |design| design.net_label_placement.open,
        render: DesignFeature::render_net_label_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.grid_snap_routing"),
        is_open: |design| design.grid_snap_routing.open,
        render: DesignFeature::render_grid_snap_routing_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.schematic_visibility"),
        is_open: |design| design.schematic_visibility_dialog.open,
        render: DesignFeature::render_schematic_visibility_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.descend_hierarchy"),
        is_open: |design| design.descend_hierarchy.open,
        render: DesignFeature::render_descend_hierarchy_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.engineering_table"),
        is_open: |design| design.engineering_table.open,
        render: DesignFeature::render_engineering_table_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.pin_port"),
        is_open: |design| design.pin_port.open,
        render: DesignFeature::render_pin_port_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.design_review_comments"),
        is_open: |design| design.design_review_comments.open,
        render: DesignFeature::render_design_review_comments_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.design_note"),
        is_open: |design| design.design_note.open,
        render: DesignFeature::render_design_note_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.documentation_shape"),
        is_open: |design| design.documentation_shape.open,
        render: DesignFeature::render_documentation_shape_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.array_selection"),
        is_open: |design| design.array_selection.open,
        render: DesignFeature::render_array_selection_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.replace_instance"),
        is_open: |design| design.replace_instance.open,
        render: DesignFeature::render_replace_instance_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.stimulus_link"),
        is_open: |design| design.stimulus_link.open,
        render: DesignFeature::render_stimulus_link_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.create_hierarchy"),
        is_open: |design| design.create_hierarchy.open,
        render: DesignFeature::render_create_hierarchy_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.connectivity_manager"),
        is_open: |design| design.connectivity_manager.open,
        render: DesignFeature::render_connectivity_manager_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.selection_bulk_edit"),
        is_open: |design| design.selection_bulk_edit.open,
        render: DesignFeature::render_selection_bulk_edit_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.create_model_bound_symbol"),
        is_open: |design| design.create_model_bound_symbol.open,
        render: DesignFeature::render_create_model_bound_symbol_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.symbol_definition"),
        is_open: |design| design.symbol_import.open || design.symbol_parameter_form.open,
        render: DesignFeature::render_symbol_definition_dialogs,
    },
    FeatureDialog {
        key: DialogKey("design.rule_setup"),
        is_open: |design| design.rule_setup.open,
        render: DesignFeature::render_rule_setup_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.object_properties"),
        is_open: |design| design.object_properties.open,
        render: DesignFeature::render_object_properties_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.rename_selection"),
        is_open: |design| design.rename_selection.open,
        render: DesignFeature::render_rename_selection_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.find_replace"),
        is_open: |design| design.find_replace.open,
        render: DesignFeature::render_find_replace_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.shared_properties"),
        is_open: |design| design.shared_properties.open,
        render: DesignFeature::render_shared_properties_dialog,
    },
];

/// The feature's dialog steps, each over the parts of the design slot it
/// owns.
impl DesignFeature {
    /// Place bus tap.
    fn render_bus_tap_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::bus_tap::render_bus_tap_dialog(&mut cx.state.bus_tap, cx.model, ctx);
    }

    /// Net label, or the off-sheet connector form.
    fn render_net_label_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::net_label::render_net_label_dialog(
            &mut cx.state.net_label_placement,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Create pins.
    fn render_pin_port_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::pin_port::render_pin_port_dialog(&mut cx.state.pin_port, cx.model, ctx);
    }

    /// Place text or design note.
    fn render_design_note_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::design_note::render_design_note_dialog(&mut cx.state.design_note, cx.model, ctx);
    }

    /// Draw documentation shape.
    fn render_documentation_shape_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::documentation_shape::render_documentation_shape_dialog(
            &mut cx.state.documentation_shape,
            cx.model,
            ctx,
        );
    }

    /// Create array.
    fn render_array_selection_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        array::render_array_selection_dialog(&mut cx.state.array_selection, cx.model, ctx);
    }

    /// Object properties.
    fn render_object_properties_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        object_properties::render_object_properties_dialog(
            &mut cx.state.object_properties,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Rename.
    fn render_rename_selection_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        rename::render_rename_selection_dialog(
            &mut cx.state.rename_selection,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Find and replace.
    fn render_find_replace_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let edit_in_progress = cx.state.schematic_edit_in_progress(cx.model);
        find_replace::render_find_replace_dialog(
            &mut cx.state.find_replace,
            edit_in_progress,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Properties, for the instances selected together.
    fn render_shared_properties_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        shared_properties::render_shared_properties_dialog(
            &mut cx.state.shared_properties,
            &cx.state.property_registry,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Grid, snap and wire routing.
    fn render_grid_snap_routing_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        cx.state
            .render_grid_snap_routing_dialog(cx.model, cx.wb, ctx);
    }

    /// Display options.
    fn render_schematic_visibility_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        cx.state
            .render_schematic_visibility_dialog(cx.model, cx.wb, ctx);
    }

    /// Descend into hierarchy. A checked choice goes to the shell, which
    /// answers it straight after the dialog's step draws.
    fn render_descend_hierarchy_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        descend::render_descend_hierarchy_dialog(
            &mut cx.state.descend_hierarchy,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Import symbol definition, then the symbol's parameter form.
    fn render_symbol_definition_dialogs(cx: &mut Cx<'_, Self>, ctx: &Context) {
        symbol_definition::render_symbol_definition_dialogs(
            &mut cx.state.symbol_import,
            &mut cx.state.symbol_parameter_form,
            cx.model,
            ctx,
        );
    }

    /// Design review comments, signed with the name the cloud gives the
    /// reviewer.
    fn render_design_review_comments_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        ReviewComments {
            dialog: &mut cx.state.design_review_comments,
            design_note: &mut cx.state.design_note,
            model: cx.model,
            workbench: cx.wb,
            cloud: cx.peers.cloud(),
            session: cx.peers.cloud_session(),
        }
        .render_design_review_comments_dialog(ctx);
    }

    /// Design rule setup. A new deck changes what blocks a run, so an applied
    /// one drops the run prepared under the old deck with its preflight.
    fn render_rule_setup_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let review_markers = cx.state.schematic_visibility.review_markers;
        if rule_setup::render_rule_setup_dialog(
            &mut cx.state.rule_setup,
            review_markers,
            cx.model,
            cx.runs,
            ctx,
        ) {
            rspice_app_runs::workbench::preflight::invalidate(
                &mut cx.runs.preflight,
                cx.handles.run_controller,
            );
        }
    }

    /// Bulk edit, over its draft and the saved filters.
    fn render_selection_bulk_edit_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        bulk_edit::render_selection_bulk_edit_dialog(
            &mut cx.state.selection_bulk_edit,
            &mut cx.state.schematic_bulk_edit_filters,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Engineering table view, over its draft and the views saved on this
    /// device.
    fn render_engineering_table_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        EngineeringTable {
            dialog: &mut cx.state.engineering_table,
            views: &mut cx.state.engineering_table_views,
            model: cx.model,
            platform: cx.platform,
            workbench: cx.wb,
        }
        .render_engineering_table_dialog(ctx);
    }

    /// Adopt a stimulus definition onto a placed source, or save its card as
    /// one, over the draft and the definition the Stimulus Library selects.
    fn render_stimulus_link_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        stimulus_link::render_stimulus_link_dialog(
            &mut cx.state.stimulus_link,
            &mut cx.state.selected_stimulus_definition,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Create model-bound symbol, over its draft. A created symbol asks for
    /// its view to open in the Design workspace.
    fn render_create_model_bound_symbol_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        create_model_bound_symbol::render_create_model_bound_symbol_dialog(
            &mut cx.state.create_model_bound_symbol,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// The schematic component editor, over its draft, the property sheets
    /// and the stimulus link it hands an instance to. A re-adopted stimulus
    /// asks for the editor to reopen on the instance's new card.
    fn render_property_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        PropertyDialog {
            dialog: &mut cx.state.tabbed_property_dialog,
            registry: &mut cx.state.property_registry,
            stimulus_link: &mut cx.state.stimulus_link,
            selected_stimulus_definition: &mut cx.state.selected_stimulus_definition,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_property_dialog(ctx);
    }

    /// The connectivity and bus manager, over its draft. Revealing a finding,
    /// opening the source map and applying reviewed repairs go to the shell.
    fn render_connectivity_manager_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        ConnectivityManager {
            dialog: &mut cx.state.connectivity_manager,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_connectivity_manager_dialog(ctx);
    }

    /// Create hierarchy, over its draft and the model it checks the draft
    /// against. A checked extraction goes to the shell.
    fn render_create_hierarchy_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        CreateHierarchy {
            dialog: &mut cx.state.create_hierarchy,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_create_hierarchy_dialog(ctx);
    }

    /// Replace instance, over its draft, the property sheets its catalog reads,
    /// the model and the session.
    fn render_replace_instance_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        ReplaceInstance {
            dialog: &mut cx.state.replace_instance,
            registry: &cx.state.property_registry,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_replace_instance_dialog(ctx);
    }

    /// The built-in XSPICE placement form, over its draft, the model it arms
    /// the placement on and the session.
    fn render_builtin_xspice_placement_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        BuiltinXspicePlacement {
            dialog: &mut cx.state.builtin_xspice_placement,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_builtin_xspice_placement_dialog(ctx);
    }
}

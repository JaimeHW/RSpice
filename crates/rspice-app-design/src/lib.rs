//! The RSpice application's design feature: the schematic canvas and symbol
//! editor, the design workspace's navigator, inspector, surfaces and toolbar,
//! the stimulus editor, and the design dialogs and actions.
//!
//! The feature's files keep the paths they had inside `rspice-app`. This root
//! declares them there and re-exports, at the same paths, the lower crates'
//! items they name.

#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#![allow(clippy::too_many_arguments)]
#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(
    test,
    allow(
        clippy::assertions_on_constants,
        clippy::bool_assert_comparison,
        clippy::cloned_ref_to_slice_refs,
        clippy::default_constructed_unit_structs,
        clippy::expect_fun_call,
        clippy::field_reassign_with_default,
        clippy::len_zero,
        clippy::manual_range_contains,
        clippy::manual_repeat_n,
        clippy::needless_range_loop,
        clippy::unnecessary_cast,
        clippy::unnecessary_get_then_check,
        clippy::unnecessary_unwrap,
        clippy::useless_vec
    )
)]

#[cfg(any(not(target_arch = "wasm32"), test))]
use rspice_app_services::io;
use rspice_app_state::{diagnostics, product, quantity, state, time_compat};
use rspice_app_workbench::ui;

pub mod properties;
pub mod schematic;

mod services {
    pub(crate) use rspice_app_services::services::model_hub;

    #[cfg(test)]
    mod drc_tests;
}

mod simulation {
    pub(crate) use rspice_app_runs::simulation::controller;
    pub(crate) use rspice_app_state::simulation::{
        placed_sources, plan, stimulus_realize, table_route,
    };

    pub(crate) use controller::SimulationController;
}

pub mod workbench {
    pub(crate) use rspice_app_state::workbench::preferences;
    pub(crate) use rspice_app_workbench::workbench::{framework, layout, localization, routing};
    pub(crate) use rspice_ui_kit::panels as design_system;

    pub(crate) use localization::{MessageCatalog, MessageId};
    #[cfg(test)]
    pub(crate) use preferences::UserPreferences;
    pub(crate) use preferences::{ChoicePreference, TogglePreference};
    pub(crate) use routing::surface_catalog::SurfaceId;
    pub(crate) use rspice_design::symbol::edit::{SymbolCommitIntent, SymbolDocumentSnapshot};
    pub(crate) use rspice_schematic_editor::symbol_editor::session::{
        SymbolGridSpacing, SymbolTool,
    };

    pub mod cross_probe;
    pub mod design;

    pub mod app {
        pub(crate) use rspice_app_state::workbench::app::schematic::placement::arm_pack_part;

        pub(crate) use actions::property_edit::{
            PropertyFormParts, authoritative_component_property_sheet,
            open_selected_object_properties_in, selected_object_properties_available_in,
        };
        pub(crate) use actions::stimulus_placement::arm_placement_from_definition;
        pub(crate) use actions::workspace::{
            copy_active_schematic_selection, deny_read_only_edit, hierarchy_master_for_component,
            selected_hierarchy_master, selected_instance_veriloga_source,
            symbol_document::{
                active_symbol_ports, finish_symbol_parent_edit, generate_active_symbol_document,
                load_active_symbol_editor_metadata, record_symbol_edit, store_symbol_canvas_frame,
                symbol_editor_copy_available, symbol_editor_lock_message,
                symbol_editor_request_source,
            },
            symbol_in_place::{
                SymbolInPlace, SymbolInPlaceRefusal, in_place_bounds, in_place_placement,
                in_place_session, in_place_session_mut, selected_symbol_in_place_target,
                sync_in_place_layout,
            },
            sync_active_schematic, veriloga_master_of_component, veriloga_source_for_component,
        };
        pub(crate) use dialogs::connectivity_manager::ConnectivityManagerDialogState;
        pub(crate) use dialogs::create_model_bound_symbol::CreateModelBoundSymbolDialogState;
        pub(crate) use dialogs::design_review_comments::DesignReviewCommentsDialogState;
        pub(crate) use dialogs::find_replace::find_replace_matches;
        pub(crate) use dialogs::hierarchy::create::open_create_hierarchy_dialog;
        pub(crate) use dialogs::placement::net_label::{
            open_net_label_placement_in, place_sequenced_net_label_in,
        };
        pub(crate) use dialogs::replace_instance::{
            open_replace_instance_dialog, replace_instance_available,
        };
        pub(crate) use dialogs::selection::array::{
            ArraySelectionDialogState, ArraySelectionPreviewCache, armed_array_selection_authority,
            armed_array_selection_plan, cancel_armed_array_selection_in,
            open_array_selection_dialog_in,
        };
        pub(crate) use dialogs::selection::bulk_edit::SelectionBulkEditDialogState;
        pub(crate) use dialogs::selection::commands::floating::{
            FloatingCopyParts, cancel_floating_paste_in, floating_paste_scope, on_grid,
            place_floating_paste, turn_floating_paste,
        };
        pub(crate) use dialogs::selection::commands::{
            cut_schematic_selection, delete_schematic_selection, duplicate_schematic_selection,
            duplicate_schematic_selection_at, join_collinear_wires_on_active_sheet,
            paste_schematic_clipboard, paste_schematic_clipboard_at, schematic_grid_step,
            select_all_schematic_objects,
        };
        pub(crate) use dialogs::selection::move_selection::{
            arm_move_selection_in, armed_move_selection_authority, cancel_armed_move_selection_in,
        };
        pub(crate) use dialogs::selection::stretch::{
            arm_stretch_selection_in, armed_stretch_selection_authority,
            cancel_armed_stretch_selection_in,
        };
        pub(crate) use dialogs::stimulus_link::{
            StimulusLinkMode, open_stimulus_link, show_stimulus_definition,
        };
        pub(crate) use dialogs::symbol_definition::{
            SymbolImportDialogState, SymbolParameterFormDialogState,
            open_symbol_parameter_form_dialog,
        };
        pub(crate) use schematic::conductor_routes::{
            reconcile_conductor_route, start_canvas_bus, start_canvas_wire,
        };
        pub use schematic::edit_authority::schematic_editor_request_source;
        pub(crate) use schematic::edit_authority::{
            SchematicEditAuthority, symbol_context_revision,
        };
        pub(crate) use schematic::named_net::{
            NamedNetTarget, apply_named_net_rename, selected_named_net_target,
            validate_named_net_rename,
        };
        pub(crate) use schematic::placement::arm_schematic_tool;

        pub mod schematic;

        pub mod actions {
            pub(crate) use symbol::{
                activate_symbol_tool, apply_symbol_edit_request, copy_selected_symbol_shape,
                delete_selected_symbol_item, duplicate_selected_symbol_shape,
                edit_selected_symbol_items, paste_symbol_shape, select_all_symbol_items,
                symbol_pin_name_request, symbol_text_request,
            };

            pub mod hierarchy_net_highlight;
            pub mod property_edit;
            pub mod stimulus;
            pub mod stimulus_placement;
            pub mod symbol;
            pub mod workspace;

            pub(crate) mod reveal {
                pub(crate) use rspice_app_workbench::workbench::app::reveal::{
                    placed_instance_in, reaches_in,
                };
            }
        }

        pub mod dialogs {
            pub(crate) use rspice_app_workbench::workbench::app::dialogs::review_primitives;

            pub mod connectivity_manager;
            pub mod create_model_bound_symbol;
            pub mod design_request;
            pub mod design_review_comments;
            pub mod engineering_table;
            pub mod find_replace;
            pub mod grid_snap_routing;
            pub mod hierarchy;
            pub mod object_properties;
            pub mod placement;
            pub mod property_dialog;
            pub mod replace_instance;
            pub mod rule_setup;
            pub mod selection;
            pub mod stimulus_link;
            pub mod symbol_definition;
            pub mod visibility_options;
        }
    }

    pub mod app_state {
        #[cfg(test)]
        pub(in crate::workbench) use rspice_app_state::workbench::app_state::sim_setup;
        pub(crate) use rspice_app_state::workbench::app_state::{
            design_checks, design_history, diagnostic_store, model, waivers,
        };
        pub(crate) use rspice_app_workbench::workbench::app_state::session::shortcuts::{
            accessibility_shortcut_summary, report_engineering_canvas_focus,
            request_engineering_canvas_focus, return_engineering_canvas_focus,
            runtime_command_platform,
        };

        pub(in crate::workbench) use crate::workbench::slots::PlatformState;
        pub(crate) use design_checks::{model_design_view, model_location_anchor};
        pub(crate) use design_history::{
            SymbolDefinitionFixtureDelta, publish_symbol_definition_candidate,
            publish_symbol_definition_candidate_with_fixture,
        };
        pub(crate) use interaction_state::{ContextTarget, DragType, SchematicKeyboardFocus};
        pub(crate) use model::{Model, SlotFacts};
        #[cfg(test)]
        pub(crate) use sim_setup::TranSetup;

        pub mod interaction_state;
    }

    pub(crate) mod browser {
        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::accessibility;
        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::file_import;
    }

    pub mod chrome {
        pub use rspice_app_workbench::workbench::chrome::*;

        pub mod document_bar {
            pub use rspice_app_workbench::workbench::chrome::document_bar::*;

            pub mod design;
        }
        pub mod status_bar {
            pub use rspice_app_workbench::workbench::chrome::status_bar::*;

            pub mod design;
        }
        pub mod title_bar {
            pub use rspice_app_workbench::workbench::chrome::title_bar::*;

            pub mod design;
        }
    }

    pub mod docks {
        pub use rspice_app_workbench::workbench::docks::*;

        pub(crate) use navigator::stimulus_browser_state::StimulusBrowserState;

        pub mod navigator {
            pub use rspice_app_workbench::workbench::docks::navigator::*;

            pub mod stimulus_browser_state;
        }
    }

    mod lifecycle {
        pub(crate) use rspice_app_workbench::workbench::lifecycle::window_session;

        pub(crate) mod session {
            pub(crate) use rspice_schematic_editor::symbol_editor::session::SymbolTool;
        }
    }

    pub mod slots {
        pub(crate) use rspice_app_runs::workbench::slots::runs;
        pub(in crate::workbench) use rspice_app_services::workbench::slots::platform;

        pub(crate) use design::DesignState;
        pub(crate) use platform::PlatformState;
        pub(crate) use runs::RunsState;

        pub mod design;
    }

    pub mod state {
        use rspice_app_workbench::workbench::state::console_views;
        pub(in crate::workbench) use rspice_app_workbench::workbench::state::navigator_tree;
        pub(crate) use rspice_app_workbench::workbench::state::workbench_session;

        pub(crate) use console_views::{InlineEditAuthority, InlineEditField, InlineEditSession};
        pub(crate) use navigator_tree::*;
        pub(crate) use rspice_app_state::workbench::workspace::Workspace;
        pub(crate) use stimulus_editor::*;
        pub(crate) use workbench_session::{
            ConsolePage, Drawer, WorkbenchSession, WorkspaceDocumentId,
        };

        pub mod stimulus_editor;
    }

    pub mod surfaces {
        pub use rspice_app_workbench::workbench::surfaces::*;

        pub mod stimulus;
    }

    mod workflows {
        pub(crate) use rspice_app_services::workbench::workflows::export_workflow;
    }
}

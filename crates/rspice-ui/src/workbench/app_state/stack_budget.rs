//! How big the application state is allowed to be *inline*.
//!
//! [`AppState`](crate::workbench::app_state::AppState) travels by value: test
//! fixtures return it through helper chains, `Default` builds it through
//! forwarding frames, and the session deserializer constructs one next to its
//! wire form. A debug build gives every one of those frames its own full-size
//! slot, so the stack cost of the state is its inline size times the depth of
//! whatever moved it — and Windows test threads run on 2 MiB. At 105 KiB
//! inline, one accessibility sweep's fixture chain overflowed exactly that
//! stack (`STATUS_STACK_OVERFLOW`, 2026-08-26) while the same suite passed on
//! Linux CI's larger stacks.
//!
//! The fix was boxing the sub-states that carried the weight — `dialogs`,
//! `ui`, `workbench`, `project_lifecycle` — and this budget keeps the class of
//! defect dead: growth has to land behind a `Box`, or consciously raise the
//! number here with the multiplication redone. The application binaries
//! reserve 64 MiB (`build.rs`), but that covers neither test harnesses nor
//! worker threads, so the reservation is a backstop, not a licence.

use std::mem::size_of;

/// ~10 frames of by-value movement must stay well under a 2 MiB thread stack.
/// The state sits near 22 KiB today; the headroom is for ordinary field
/// growth, not for the next inline dialog aggregate.
const APP_STATE_INLINE_BUDGET: usize = 32 * 1024;

/// The application root adds its own inline members (controllers, runtimes)
/// on top of the state and moves through the same fixture chains.
const APP_INLINE_BUDGET: usize = 40 * 1024;

fn field_sizes() -> Vec<(&'static str, usize)> {
    use crate::workbench::app_state as a;
    vec![
        ("schematic", size_of::<crate::state::SchematicState>()),
        ("simulation", size_of::<crate::state::SimulationState>()),
        (
            "project_design_history",
            size_of::<a::design_history::ProjectDesignHistory>(),
        ),
        ("design_checks", size_of::<a::DesignCheckRuntime>()),
        (
            "dialogs",
            size_of::<Box<crate::workbench::app::DialogState>>(),
        ),
        ("sim_setup", size_of::<a::SimSetupState>()),
        ("log_buffer", size_of::<crate::diagnostics::LogBuffer>()),
        (
            "script_console",
            size_of::<a::session::script_console::ScriptConsoleState>(),
        ),
        ("library_manager", size_of::<crate::state::LibraryManager>()),
        (
            "library_edit_locks",
            size_of::<crate::state::ProjectLibraryLockAuthority>(),
        ),
        ("workspace", size_of::<crate::state::ProjectWorkspace>()),
        (
            "project_lifecycle",
            size_of::<Box<crate::workbench::lifecycle::project_lifecycle::ProjectLifecycleState>>(),
        ),
        (
            "tabbed_property_dialog",
            size_of::<crate::properties::TabbedPropertyDialogState>(),
        ),
        (
            "property_registry",
            size_of::<crate::state::PropertyRegistry>(),
        ),
        (
            "calculator_panel",
            size_of::<a::session::calculator::CalculatorPanel>(),
        ),
        (
            "pdk_settings_dialog",
            size_of::<a::session::pdk_settings::PdkSettingsDialogState>(),
        ),
        (
            "pdk_config",
            size_of::<crate::state::pdk_config::PdkConfig>(),
        ),
        (
            "model_library_manager",
            size_of::<crate::state::model_library::ModelLibraryManager>(),
        ),
        (
            "model_browser_state",
            size_of::<crate::properties::model_browser::ModelBrowserState>(),
        ),
        ("analysis", size_of::<a::AnalysisWorkspaceState>()),
        ("ui", size_of::<Box<crate::workbench::UiSessionState>>()),
        (
            "workbench",
            size_of::<Box<crate::workbench::WorkbenchState>>(),
        ),
        (
            "shortcut_resolver",
            size_of::<a::session::shortcuts::ShortcutResolverState>(),
        ),
        (
            "shortcut_library_persistence",
            size_of::<a::session::shortcut_library::ShortcutLibraryPersistenceRuntime>(),
        ),
    ]
}

fn weight_table() -> String {
    let mut rows = field_sizes();
    rows.sort_by_key(|&(_, size)| std::cmp::Reverse(size));
    rows.iter()
        .map(|(name, size)| format!("{size:>9}  {name}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The state must stay cheap to move by value on a default thread stack.
#[test]
fn app_state_stays_inside_its_inline_stack_budget() {
    let state = size_of::<crate::workbench::app_state::AppState>();
    assert!(
        state <= APP_STATE_INLINE_BUDGET,
        "AppState is {state} bytes inline (budget {APP_STATE_INLINE_BUDGET}); every by-value \
         move carries this on the stack, and debug fixture chains hold ten of them on a 2 MiB \
         Windows test thread. Box the field that grew:\n{}",
        weight_table()
    );
    let app = size_of::<crate::workbench::app::RSpiceApp>();
    assert!(
        app <= APP_INLINE_BUDGET,
        "RSpiceApp is {app} bytes inline (budget {APP_INLINE_BUDGET}); test fixtures move it by \
         value through helper chains. Box the member that grew:\n{}",
        weight_table()
    );
}

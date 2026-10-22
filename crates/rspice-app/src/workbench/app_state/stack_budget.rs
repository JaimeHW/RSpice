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
//! The fix was boxing the sub-states that carried the weight — the dialogs,
//! now the feature slots, `ui`, `workbench`, `project_lifecycle` — and this
//! budget keeps the class of defect dead: growth has to land behind a `Box`,
//! or consciously raise the number here with the multiplication redone. A
//! boxed slot is still built on the stack before it is boxed, so each one
//! has its own budget as well. The application binaries
//! reserve 64 MiB (`build.rs`), but that covers neither test harnesses nor
//! worker threads, so the reservation is a backstop, not a licence.

use std::mem::size_of;

/// ~10 frames of by-value movement must stay well under a 2 MiB thread stack.
/// The state sits near 22 KiB today; the headroom is for ordinary field
/// growth, not for the next inline dialog aggregate.
const APP_STATE_INLINE_BUDGET: usize = 32 * 1024;

/// The model is held inline in the state, so its size is spent from the
/// state's budget. It sits near 21 KiB today; the headroom is for ordinary
/// field growth: a model that needs more boxes the field that grew rather
/// than taking the rest of the state's headroom.
const MODEL_INLINE_BUDGET: usize = 24 * 1024;

/// Each boxed feature slot, by its field: what the slot holds today plus
/// room for ordinary growth. `Default` and `Clone` build a slot by value
/// before it reaches the heap, so a debug frame holds the whole slot; a slot
/// that needs more boxes the field that grew.
fn slot_budgets() -> Vec<(&'static str, usize, usize)> {
    vec![
        ("platform", held(|state| &*state.platform), 1024),
        ("design", held(|state| &*state.design), 24 * 1024),
        ("code", held(|state| &*state.code), 2 * 1024),
        ("simulate", held(|state| &*state.simulate), 2 * 1024),
        ("project", held(|state| &*state.project), 8 * 1024),
        ("results", held(|state| &*state.results), 1024),
        ("hardcopy", held(|state| &*state.hardcopy), 4 * 1024),
        ("settings", held(|state| &*state.settings), 4 * 1024),
        ("cloud", held(|state| &*state.cloud), 1024),
        ("sheets", held(|state| &*state.sheets), 10 * 1024),
    ]
}

/// The size of what one of the state's fields holds, named by the field
/// rather than by its type.
fn held<T>(_field: fn(&crate::workbench::app_state::AppState) -> &T) -> usize {
    size_of::<T>()
}

/// The inline fields only. The boxed sub-states (the feature slots, `ui`,
/// `workbench`, `project_lifecycle`) contribute pointer size by construction
/// and can never explain inline growth, so listing them would only add the
/// upward module references this layer is retiring.
fn field_sizes() -> Vec<(&'static str, usize)> {
    use crate::workbench::app_state as a;
    vec![
        ("model", size_of::<a::Model>()),
        ("shell", held(|state| &state.shell)),
        (
            "tabbed_property_dialog",
            size_of::<crate::properties::TabbedPropertyDialogState>(),
        ),
        (
            "property_registry",
            size_of::<crate::properties::PropertyEditorSchema>(),
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
            "model_browser_state",
            size_of::<crate::properties::model_browser::ModelBrowserState>(),
        ),
        (
            "analysis",
            size_of::<a::viewer_capabilities::AnalysisWorkspaceState>(),
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

/// The model's inline fields, listed the same way: its boxed fields
/// (`netlist_artifacts`, `preferences`) are pointer size by construction.
fn model_field_sizes() -> Vec<(&'static str, usize)> {
    use crate::workbench::app_state as a;
    vec![
        ("schematic", size_of::<crate::state::SchematicState>()),
        ("simulation", size_of::<crate::state::SimulationState>()),
        (
            "project_design_history",
            size_of::<a::design_history::ProjectDesignHistory>(),
        ),
        ("design_checks", size_of::<a::DesignCheckRuntime>()),
        ("sim_setup", size_of::<a::SimSetupState>()),
        ("log_buffer", size_of::<crate::diagnostics::LogBuffer>()),
        ("library_manager", size_of::<crate::state::LibraryManager>()),
        (
            "library_edit_locks",
            size_of::<crate::state::ProjectLibraryLockAuthority>(),
        ),
        ("workspace", size_of::<crate::state::ProjectWorkspace>()),
        (
            "pdk_config",
            size_of::<crate::state::pdk_config::PdkConfig>(),
        ),
        (
            "model_library_manager",
            size_of::<crate::state::model_library::ModelLibraryManager>(),
        ),
        (
            "safe_mode",
            size_of::<crate::state::write_policy::LocalSafeModeState>(),
        ),
        (
            "live_write_locks",
            size_of::<crate::state::write_policy::LiveWriteLocks>(),
        ),
    ]
}

fn weight_table(mut rows: Vec<(&'static str, usize)>) -> String {
    rows.sort_by_key(|&(_, size)| std::cmp::Reverse(size));
    rows.iter()
        .map(|(name, size)| format!("{size:>9}  {name}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The state must stay cheap to move by value on a default thread stack.
///
/// The application root that embeds this state carries its own budget in its
/// own module's tests, beside the members it measures.
#[test]
fn app_state_stays_inside_its_inline_stack_budget() {
    let state = size_of::<crate::workbench::app_state::AppState>();
    assert!(
        state <= APP_STATE_INLINE_BUDGET,
        "AppState is {state} bytes inline (budget {APP_STATE_INLINE_BUDGET}); every by-value \
         move carries this on the stack, and debug fixture chains hold ten of them on a 2 MiB \
         Windows test thread. Box the field that grew:\n{}",
        weight_table(field_sizes())
    );
}

/// The model rides inline in every by-value move of the state, so it is
/// held to its own share of the state's budget.
#[test]
fn the_model_stays_inside_its_inline_stack_budget() {
    let model = size_of::<crate::workbench::app_state::Model>();
    assert!(
        model <= MODEL_INLINE_BUDGET,
        "Model is {model} bytes inline (budget {MODEL_INLINE_BUDGET}); it moves inline with \
         every AppState. Box the field that grew:\n{}",
        weight_table(model_field_sizes())
    );
}

/// A slot is boxed in the state, but built and cloned by value on its way
/// there, so each is held to its own budget.
#[test]
fn each_feature_slot_stays_inside_its_stack_budget() {
    for (slot, size, budget) in slot_budgets() {
        assert!(
            size <= budget,
            "the `{slot}` slot is {size} bytes (budget {budget}); `Default` and `Clone` build it \
             by value before it is boxed. Box the field that grew."
        );
    }
}

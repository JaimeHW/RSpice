//! The features the shell registers, and how a command key reaches the
//! feature that declares it.
//!
//! [`features!`] lists every registered feature with the slot of the
//! application state it owns. Each feature's registry is built once. A call
//! into one of its commands borrows the feature's slot, the model, the
//! session and the peers it reads from the application state for that one
//! call: a context is references only, so building one allocates nothing.

use std::sync::LazyLock;

use crate::state::command_catalog::CommandKey;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{Cx, Feature, FeatureRegistry, Peers, Reads, Session};
use crate::workbench::framework::requests::Outbox;
use crate::workbench::sheets::SheetsFeature;
use crate::workbench::state::WorkbenchSession;

impl Session for WorkbenchSession {
    fn outbox(&mut self) -> &mut Outbox {
        &mut self.outbox
    }
}

/// A feature the shell registers: its registry, and its contexts borrowed
/// from the application state.
pub(super) trait Registered: Feature {
    /// Everything the feature registered, built on first use.
    fn registry() -> &'static FeatureRegistry<Self>;

    /// What the feature reads to answer whether a command can run.
    fn reads(state: &AppState) -> Reads<'_, Self>;

    /// The context the feature's command runs in.
    fn cx(state: &mut AppState) -> Cx<'_, Self>;
}

/// Register each `slot: Feature` pair: the feature `Feature`, whose state is
/// the application state's field `slot`.
macro_rules! features {
    ($($slot:ident: $feature:ty),+ $(,)?) => {
        $(
            impl Registered for $feature {
                fn registry() -> &'static FeatureRegistry<Self> {
                    static REGISTRY: LazyLock<FeatureRegistry<$feature>> =
                        LazyLock::new(FeatureRegistry::build);
                    &REGISTRY
                }

                fn reads(state: &AppState) -> Reads<'_, Self> {
                    Reads::new(&*state.$slot, &state.model, &*state.workbench)
                }

                fn cx(state: &mut AppState) -> Cx<'_, Self> {
                    Cx::new(
                        &mut *state.$slot,
                        &mut state.model,
                        &mut *state.workbench,
                        Peers::new(&*state.design),
                    )
                }
            }
        )+

        /// What the feature command `key` names declares; `None` for a key
        /// no registered feature declares.
        pub(super) fn spec(key: CommandKey) -> Option<&'static CommandSpec> {
            $(
                if let Some(command) = <$feature as Registered>::registry().command(key) {
                    return Some(&command.spec);
                }
            )+
            None
        }

        /// Whether the feature command `key` names can run here, as its
        /// feature answers once the shell's own guards have passed; `None`
        /// for a key no registered feature declares.
        pub(super) fn availability(state: &AppState, key: CommandKey) -> Option<CommandAvailability> {
            $(
                if let Some(command) = <$feature as Registered>::registry().command(key) {
                    return Some(command.availability(&<$feature as Registered>::reads(state)));
                }
            )+
            None
        }

        /// Whether what the feature dialog step `key` names draws is open;
        /// `false` for a key no registered feature draws.
        pub(super) fn dialog_open(state: &AppState, key: DialogKey) -> bool {
            $(
                if let Some(dialog) = <$feature as Registered>::registry().dialog(key) {
                    return (dialog.is_open)(&*state.$slot);
                }
            )+
            false
        }

        /// Every dialog step each registered feature draws, with the slot of
        /// the feature that draws it.
        #[cfg(test)]
        pub(super) fn registered_dialogs() -> Vec<(&'static str, DialogKey)> {
            let mut dialogs = Vec::new();
            $(
                for dialog in <$feature as Registered>::registry().registered_dialogs() {
                    dialogs.push((stringify!($slot), dialog.key));
                }
            )+
            dialogs
        }

        /// Draw the feature dialog step `key` names for this frame, then
        /// answer what it raised for straight after.
        pub(super) fn render_dialog(app: &mut RSpiceApp, ctx: &egui::Context, key: DialogKey) {
            $(
                if let Some(dialog) = <$feature as Registered>::registry().dialog(key) {
                    (dialog.render)(&mut <$feature as Registered>::cx(&mut app.state), ctx);
                }
            )else+
            app.after_contribution();
        }

        /// Carry out the feature command `key` names, once the shell's
        /// prologue has let it through.
        pub(super) fn execute(state: &mut AppState, key: CommandKey) {
            $(
                if let Some(command) = <$feature as Registered>::registry().command(key) {
                    command.execute(&mut <$feature as Registered>::cx(state));
                }
            )else+
        }
    };
}

features! {
    sheets: SheetsFeature,
}

/// Draw `F`'s inspector section in the container's `ui`. The caller answers
/// what it raised straight after.
pub(super) fn show_inspector<F: Registered>(state: &mut AppState, ui: &mut egui::Ui) {
    if let Some(inspector) = F::registry().inspector_section() {
        (inspector.show)(&mut F::cx(state), ui);
    }
}

/// Commit what `F`'s inspector section holds typed and not yet committed.
/// The caller answers what that raised straight after.
pub(super) fn commit_inspector<F: Registered>(state: &mut AppState, ctx: &egui::Context) {
    if let Some(inspector) = F::registry().inspector_section() {
        (inspector.commit_pending_edit)(&mut F::cx(state), ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::vocabulary::command_registry;

    /// The registry the shell orders commands by declares each feature
    /// command exactly as its feature does.
    #[test]
    fn every_feature_command_is_ordered_as_its_feature_declares_it() {
        let registry = command_registry();
        let declared = <SheetsFeature as Registered>::registry();
        for key in registry.order() {
            let Some(spec) = spec(*key) else { continue };
            assert_eq!(
                format!("{spec:?}"),
                format!("{:?}", registry.spec(*key).expect("ordered")),
            );
            assert!(declared.command(*key).is_some(), "{key:?}");
        }
        assert_eq!(
            registry
                .order()
                .iter()
                .filter(|key| spec(**key).is_some())
                .count(),
            crate::workbench::sheets::commands::SheetsCommand::ALL.len()
        );
    }
}

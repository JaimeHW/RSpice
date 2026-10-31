//! The features the shell registers, and how a command key reaches the
//! feature that declares it.
//!
//! [`features!`] lists every registered feature with the slot of the
//! application state it owns, and the application's field holding what the
//! feature runs, for a feature that runs services of its own. Each
//! feature's registry is built once. A call into one of its commands borrows
//! the feature's slot, the model, the session, the platform services' state
//! and the peers it reads from the application state, and what the feature
//! runs and the application's handles on the services below the workbench
//! beside it, for that one call: a context is references only, so building
//! one allocates nothing.

use std::sync::LazyLock;

use crate::state::command_catalog::CommandKey;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::cloud::CloudFeature;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    Cx, Feature, FeatureRegistry, LowerHandles, Peers, Reads, ToolbarTool,
};
use crate::workbench::hardcopy::HardcopyFeature;
use crate::workbench::models::ModelsFeature;
use crate::workbench::project::ProjectFeature;
use crate::workbench::results::ResultsFeature;
use crate::workbench::settings::SettingsFeature;
use crate::workbench::sheets::SheetsFeature;

/// A feature the shell registers: its registry, and its contexts borrowed
/// from the application.
pub(crate) trait Registered: Feature {
    /// Everything the feature registered, built on first use.
    fn registry() -> &'static FeatureRegistry<Self>;

    /// What the feature reads to answer whether a command can run.
    fn reads(app: &RSpiceApp) -> Reads<'_, Self>;

    /// The context the feature's command runs in, borrowed from `app`.
    fn cx(app: &mut RSpiceApp) -> Cx<'_, Self>;
}

/// The context `F`'s call runs in, borrowed from `app`: its state, what it
/// runs, and the application's handles on the services below the workbench.
pub(crate) fn cx<F: Registered>(app: &mut RSpiceApp) -> Cx<'_, F> {
    F::cx(app)
}

/// The results peer a context over `state` holds for the feature whose slot
/// is `slot`: none for the results feature itself, whose slot it is.
macro_rules! results_peer {
    (results, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&*$state.results)
    };
}

/// The cloud peer a context over `state` holds for the feature whose slot is
/// `slot`: none for the cloud feature itself, whose slot it is.
macro_rules! cloud_peer {
    (cloud, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&*$state.cloud)
    };
}

/// The hardcopy peer a context over `state` holds for the feature whose slot
/// is `slot`: none for the hardcopy feature itself, whose slot it is.
macro_rules! hardcopy_peer {
    (hardcopy, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&*$state.hardcopy)
    };
}

/// The models peer a context over `state` holds for the feature whose slot
/// is `slot`: none for the models feature itself, whose slot it is.
macro_rules! models_peer {
    (models, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&*$state.models)
    };
}

/// The simulate peer a context over `state` holds for the feature whose slot
/// is `slot`: none for simulate itself, whose slot it is.
macro_rules! simulate_peer {
    (simulate, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&*$state.simulate)
    };
}

/// What a context over `app` lends the feature as what it runs: the
/// application's field `own`, or nothing for a feature that runs nothing.
macro_rules! own {
    (&mut $app:ident) => {
        &mut []
    };
    (&mut $app:ident, $own:ident) => {
        &mut $app.$own
    };
    (&$app:ident) => {
        &[]
    };
    (&$app:ident, $own:ident) => {
        &$app.$own
    };
}

/// Register each `slot: Feature` pair: the feature `Feature`, whose state is
/// the application state's field `slot`. `slot: Feature => own` also names
/// the application's field `own` that holds what the feature runs.
macro_rules! features {
    ($($slot:ident: $feature:ty $(=> $own:ident)?),+ $(,)?) => {
        $(
            impl Registered for $feature {
                fn registry() -> &'static FeatureRegistry<Self> {
                    static REGISTRY: LazyLock<FeatureRegistry<$feature>> =
                        LazyLock::new(FeatureRegistry::build);
                    &REGISTRY
                }

                fn reads(app: &RSpiceApp) -> Reads<'_, Self> {
                    let state = &app.state;
                    Reads::new(
                        &*state.$slot,
                        own!(&app $(, $own)?),
                        &state.model,
                        &*state.workbench,
                        &*state.platform,
                        &*state.runs,
                        &*state.results,
                        command_registry(),
                    )
                }

                fn cx(app: &mut RSpiceApp) -> Cx<'_, Self> {
                    let handles = LowerHandles {
                        export_io: app.export_workflow_io.as_ref(),
                        run_controller: &mut app.simulation_controller,
                        model_hub: &mut app.model_hub,
                    };
                    let state = &mut app.state;
                    Cx::new(
                        &mut *state.$slot,
                        own!(&mut app $(, $own)?),
                        &mut state.model,
                        &mut *state.workbench,
                        &mut *state.platform,
                        &mut *state.runs,
                        handles,
                        Peers::new(
                            &mut *state.design,
                            cloud_peer!($slot, state),
                            hardcopy_peer!($slot, state),
                            results_peer!($slot, state),
                            simulate_peer!($slot, state),
                            &*state.code,
                            models_peer!($slot, state),
                        ),
                        command_registry(),
                    )
                }
            }
        )+

        /// Whether the feature command `key` names is offered here at all, as
        /// its feature answers before the shell's own guards; `true` for a
        /// key no registered feature declares.
        pub(super) fn offered(app: &RSpiceApp, key: CommandKey) -> bool {
            $(
                if let Some(command) = <$feature as Registered>::registry().command(key) {
                    return command.offered(&<$feature as Registered>::reads(app));
                }
            )+
            true
        }

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
        pub(super) fn availability(app: &RSpiceApp, key: CommandKey) -> Option<CommandAvailability> {
            $(
                if let Some(command) = <$feature as Registered>::registry().command(key) {
                    return Some(command.availability(&<$feature as Registered>::reads(app)));
                }
            )+
            None
        }

        /// Whether the routed command `key` can run in the feature that takes
        /// it here; `None` when no registered feature takes it here.
        pub(super) fn routed_enabled(app: &RSpiceApp, key: CommandKey) -> Option<bool> {
            $(
                if let Some(routed) = <$feature as Registered>::registry().routed_command(key) {
                    let reads = <$feature as Registered>::reads(app);
                    if (routed.takes)(&reads) {
                        return Some((routed.enabled)(&reads));
                    }
                }
            )+
            None
        }

        /// Carry out the routed command `key` in the feature that takes it
        /// here, and say whether one did. What the feature raises waits in
        /// the outbox as what the shell's own branch raises does.
        pub(super) fn route(app: &mut RSpiceApp, key: CommandKey) -> bool {
            $(
                if let Some(routed) = <$feature as Registered>::registry().routed_command(key)
                    && (routed.takes)(&<$feature as Registered>::reads(app))
                {
                    (routed.execute)(&mut cx::<$feature>(app));
                    return true;
                }
            )+
            false
        }

        /// Whether the state the feature command `key` names switches is on;
        /// `None` for a command that acts once, and for a key no registered
        /// feature declares.
        pub(super) fn checked(app: &RSpiceApp, key: CommandKey) -> Option<bool> {
            $(
                if let Some(command) = <$feature as Registered>::registry().command(key) {
                    return command.checked(&<$feature as Registered>::reads(app));
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
                    (dialog.render)(&mut cx::<$feature>(app), ctx);
                }
            )else+
            app.after_contribution();
        }

        /// Carry out the feature command `key` names, once the shell's
        /// prologue has let it through.
        pub(super) fn execute(app: &mut RSpiceApp, key: CommandKey) {
            $(
                if let Some(command) = <$feature as Registered>::registry().command(key) {
                    command.execute(&mut cx::<$feature>(app));
                }
            )else+
        }
    };
}

features! {
    sheets: SheetsFeature,
    settings: SettingsFeature,
    hardcopy: HardcopyFeature,
    results: ResultsFeature,
    cloud: CloudFeature => cloud,
    models: ModelsFeature,
    project: ProjectFeature,
}

/// Draw `F`'s inspector section in the container's `ui`. The caller answers
/// what it raised straight after.
pub(super) fn show_inspector<F: Registered>(app: &mut RSpiceApp, ui: &mut egui::Ui) {
    if let Some(inspector) = F::registry().inspector_section() {
        (inspector.show)(&mut cx::<F>(app), ui);
    }
}

/// Draw `F`'s surface on the stage's `ui`. The caller answers what it raised
/// straight after.
pub(super) fn show_surface<F: Registered>(app: &mut RSpiceApp, ui: &mut egui::Ui) {
    if let Some(surface) = F::registry().surface_section() {
        (surface.show)(&mut cx::<F>(app), ui);
    }
}

/// Draw `F`'s navigator section in the navigator's `ui`. The caller answers
/// what it raised straight after.
pub(super) fn show_navigator<F: Registered>(app: &mut RSpiceApp, ui: &mut egui::Ui) {
    if let Some(navigator) = F::registry().navigator_section() {
        (navigator.show)(&mut cx::<F>(app), ui);
    }
}

/// Draw `F`'s split projection in the split's `ui`. The caller answers what
/// it raised straight after.
pub(super) fn show_split<F: Registered>(app: &mut RSpiceApp, ui: &mut egui::Ui) {
    if let Some(split) = F::registry().split_section() {
        (split.show)(&mut cx::<F>(app), ui);
    }
}

/// The tools `F` puts on the context toolbar, in order.
pub(super) fn toolbar_tools<F: Registered>() -> &'static [ToolbarTool] {
    F::registry().toolbar_tools()
}

/// Commit what `F`'s inspector section holds typed and not yet committed.
/// The caller answers what that raised straight after.
pub(super) fn commit_inspector<F: Registered>(app: &mut RSpiceApp, ctx: &egui::Context) {
    if let Some(inspector) = F::registry().inspector_section() {
        (inspector.commit_pending_edit)(&mut cx::<F>(app), ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::vocabulary::command_registry;

    /// The registry the shell orders commands by declares each feature
    /// command exactly as its feature does, and one feature declares it.
    #[test]
    fn every_feature_command_is_ordered_as_its_feature_declares_it() {
        let registry = command_registry();
        let sheets = <SheetsFeature as Registered>::registry();
        let settings = <SettingsFeature as Registered>::registry();
        let hardcopy = <HardcopyFeature as Registered>::registry();
        let results = <ResultsFeature as Registered>::registry();
        let cloud = <CloudFeature as Registered>::registry();
        let models = <ModelsFeature as Registered>::registry();
        let project = <ProjectFeature as Registered>::registry();
        for key in registry.order() {
            let Some(spec) = spec(*key) else { continue };
            assert_eq!(
                format!("{spec:?}"),
                format!("{:?}", registry.spec(*key).expect("ordered")),
            );
            let declaring = [
                sheets.command(*key).is_some(),
                settings.command(*key).is_some(),
                hardcopy.command(*key).is_some(),
                results.command(*key).is_some(),
                cloud.command(*key).is_some(),
                models.command(*key).is_some(),
                project.command(*key).is_some(),
            ];
            assert_eq!(
                declaring.iter().filter(|declares| **declares).count(),
                1,
                "{key:?}"
            );
        }
        assert_eq!(
            registry
                .order()
                .iter()
                .filter(|key| spec(**key).is_some())
                .count(),
            crate::workbench::sheets::commands::SheetsCommand::ALL.len()
                + crate::workbench::settings::commands::SettingsCommand::ALL.len()
                + crate::workbench::hardcopy::commands::HardcopyCommand::LISTED.len()
                + crate::workbench::results::commands::ResultsCommand::ALL.len()
                + crate::workbench::cloud::commands::CloudCommand::ALL.len()
                + crate::workbench::models::commands::ModelsCommand::ALL.len()
                + crate::workbench::project::commands::ProjectCommand::ALL.len()
        );
    }

    /// A command a feature leaves out of the registry is not ordered, and the
    /// key a container holds still reaches it.
    #[test]
    fn a_command_left_out_of_the_registry_is_reached_by_its_key() {
        use crate::workbench::hardcopy::commands::HardcopyCommand;

        let registry = command_registry();
        for command in HardcopyCommand::REPORT_TOOLBAR {
            let key = command.key();
            assert!(!registry.order().contains(&key), "{key:?}");
            assert!(registry.spec(key).is_none(), "{key:?}");
            assert_eq!(
                spec(key).map(|spec| spec.key),
                Some(key),
                "the container's key reaches {key:?}"
            );
        }
    }

    /// A routed command is the shell's: a feature takes a part in a command
    /// the registry orders and no feature declares.
    #[test]
    fn every_routed_command_is_one_the_shell_orders() {
        let registry = command_registry();
        let routed = <ResultsFeature as Registered>::registry().registered_routed();
        assert!(!routed.is_empty());
        for part in routed {
            assert!(registry.order().contains(&part.key), "{:?}", part.key);
            assert!(spec(part.key).is_none(), "{:?}", part.key);
        }
    }

    /// A feature that showed the sheet's properties from its own surface
    /// asks for the Sheet section. The sheets feature's section takes the
    /// request as it draws, and the panel spends it bringing its Sheet
    /// section into view, as when the sheets feature asks itself.
    #[test]
    fn the_sheet_section_takes_a_reveal_request_as_it_draws() {
        use crate::workbench::framework::requests::RevealSheetSection;

        let mut app = crate::workbench::RSpiceApp::test_instance();
        app.state.workbench.outbox.push(RevealSheetSection);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(312.0);
                show_inspector::<SheetsFeature>(&mut app, ui);
            });
        });
        assert!(
            app.state
                .workbench
                .outbox
                .peek::<RevealSheetSection>()
                .is_none(),
            "the section takes the request"
        );
        assert!(
            !app.state.sheets.reveal_sheet_section,
            "the panel brought its Sheet section into view, once"
        );
    }
}

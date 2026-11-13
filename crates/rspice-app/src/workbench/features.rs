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
use crate::workbench::code::CodeFeature;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::design::DesignFeature;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    Cx, Feature, FeatureRegistry, ForeignAvailability, LowerHandles, Peers, Reads, RoutedCommand,
    ToolbarId, ToolbarTool,
};
use crate::workbench::hardcopy::HardcopyFeature;
use crate::workbench::models::ModelsFeature;
use crate::workbench::project::ProjectFeature;
use crate::workbench::results::ResultsFeature;
use crate::workbench::settings::SettingsFeature;
use crate::workbench::sheets::SheetsFeature;
use crate::workbench::simulate::SimulateFeature;

/// A Verilog-A source, read-only, in the editor well the code feature draws:
/// how the Library page shows a file of a built-in model. The shell is what
/// may put one feature's well on another's page.
pub(in crate::workbench) fn read_only_veriloga_well(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    source: &mut String,
    revision: u64,
    messages: crate::workbench::MessageCatalog,
) {
    use crate::workbench::documents::code_workspace::{
        CodeEditorLanguage, EditorFindings, show_veriloga_document_versioned,
    };

    show_veriloga_document_versioned(
        ui,
        id,
        source,
        revision,
        CodeEditorLanguage::VerilogA,
        EditorFindings::Outdated,
        None,
        false,
        messages,
        &[],
    );
}

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

/// The context `F`'s call runs in, as [`cx`] lends it, holding the
/// dispatcher's answers for the commands `F` reads when `reads(app)` says
/// the call reads them. They are taken only after that check, so a step
/// that reads them only while it is open costs nothing more while it is
/// closed.
pub(crate) fn cx_with_foreign_answers<F: Registered>(
    app: &mut RSpiceApp,
    reads: impl FnOnce(&RSpiceApp) -> bool,
) -> Cx<'_, F> {
    if !reads(app) {
        return F::cx(app);
    }
    let answers =
        ForeignAvailability::take(F::FOREIGN_AVAILABILITY, |key| app.command_availability(key));
    F::cx(app).with_foreign_availability(answers)
}

/// The context a page of `F` with a menu draws in, as [`cx`] lends it,
/// holding the dispatcher's answers for the commands `F` reads and, when
/// `menu_open(app)` says one of `F`'s menus is open or opening this frame,
/// for those its menus read after them. No other lend takes the menus'
/// answers, so a frame without an open menu asks the dispatcher nothing
/// more.
pub(crate) fn cx_with_menu_answers<F: Registered>(
    app: &mut RSpiceApp,
    menu_open: impl FnOnce(&RSpiceApp) -> bool,
) -> Cx<'_, F> {
    let menus = if menu_open(app) {
        F::MENU_AVAILABILITY
    } else {
        &[]
    };
    let answers = ForeignAvailability::take_with_menus(F::FOREIGN_AVAILABILITY, menus, |key| {
        app.command_availability(key)
    });
    F::cx(app).with_foreign_availability(answers)
}

/// The context `F`'s command runs in, as [`cx`] lends it, holding whether
/// anything the dialog gate counts is open and whether anything holding the
/// application modal is, both taken as the run starts.
fn command_cx<F: Registered>(app: &mut RSpiceApp) -> Cx<'_, F> {
    let dialog_modal = app.state.dialog_modal_open();
    let application_modal = app.state.application_modal_open();
    F::cx(app)
        .with_dialog_modal(dialog_modal)
        .with_application_modal(application_modal)
}

/// Carry out `routed` where `F` takes it here, and say whether it did. Its
/// context holds whether an application-modal dialog is open, taken once
/// `F` has taken the command, as the shell's own arm asked it at that point.
fn route_part<F: Registered>(app: &mut RSpiceApp, routed: &RoutedCommand<F>) -> bool {
    if !(routed.takes)(&F::reads(app)) {
        return false;
    }
    let modal_open = app.state.application_modal_open();
    (routed.execute)(&mut F::cx(app).with_application_modal(modal_open));
    true
}

/// The results peer a context over `state` holds for the feature whose slot
/// is `slot`: none for the results feature itself, whose slot it is.
macro_rules! results_peer {
    (results, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&mut *$state.results)
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

/// `peers` with the cloud's sessions lent beside them for the feature whose
/// slot is `slot`: none for the cloud feature itself, whose own handles they
/// are.
macro_rules! cloud_session_peer {
    (cloud, $app:ident, $peers:expr) => {
        $peers
    };
    ($slot:ident, $app:ident, $peers:expr) => {
        $peers.with_cloud_session(&$app.cloud)
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

/// The design peer a context over `state` holds for the feature whose slot
/// is `slot`: none for design itself, whose slot it is.
macro_rules! design_peer {
    (design, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&mut *$state.design)
    };
}

/// The code peer a context over `state` holds for the feature whose slot is
/// `slot`: none for code itself, whose slot it is.
macro_rules! code_peer {
    (code, $state:ident) => {
        None
    };
    ($slot:ident, $state:ident) => {
        Some(&*$state.code)
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
                        cloud_session_peer!(
                            $slot,
                            app,
                            Peers::new(
                                design_peer!($slot, state),
                                cloud_peer!($slot, state),
                                hardcopy_peer!($slot, state),
                                results_peer!($slot, state),
                                simulate_peer!($slot, state),
                                code_peer!($slot, state),
                                models_peer!($slot, state),
                            )
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
                    && route_part::<$feature>(app, routed)
                {
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

        /// The commands each registered feature reads the dispatcher's answer
        /// for, with the slot of the feature that reads them.
        #[cfg(test)]
        fn foreign_availability() -> Vec<(&'static str, &'static [CommandKey])> {
            vec![$((stringify!($slot), <$feature as Feature>::FOREIGN_AVAILABILITY)),+]
        }

        /// The commands each registered feature's menus read the
        /// dispatcher's answer for, with the slot of the feature.
        #[cfg(test)]
        fn menu_availability() -> Vec<(&'static str, &'static [CommandKey])> {
            vec![$((stringify!($slot), <$feature as Feature>::MENU_AVAILABILITY)),+]
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
                    command.execute(&mut command_cx::<$feature>(app));
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
    simulate: SimulateFeature,
    project: ProjectFeature,
    design: DesignFeature,
    code: CodeFeature => code,
}

/// Draw `F`'s inspector section in the container's `ui`. The caller answers
/// what it raised straight after.
pub(super) fn show_inspector<F: Registered>(app: &mut RSpiceApp, ui: &mut egui::Ui) {
    if let Some(inspector) = F::registry().inspector_section() {
        (inspector.show)(&mut cx::<F>(app), ui);
    }
}

/// [`show_inspector`], in a context holding the dispatcher's answers for the
/// commands `F` reads when `reads(app)` says the section reads them, as
/// [`cx_with_foreign_answers`] takes them.
pub(super) fn show_inspector_reading<F: Registered>(
    app: &mut RSpiceApp,
    ui: &mut egui::Ui,
    reads: impl FnOnce(&RSpiceApp) -> bool,
) {
    if let Some(inspector) = F::registry().inspector_section() {
        (inspector.show)(&mut cx_with_foreign_answers::<F>(app, reads), ui);
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

/// Draw the toolbar `F` registered as `id` in the toolbar's lane: its
/// presses and conditions read over `F`'s reads, its controls draw in `F`'s
/// context. The caller answers what it raised straight after.
pub(super) fn show_toolbar<F: Registered>(
    app: &mut RSpiceApp,
    ui: &mut egui::Ui,
    id: ToolbarId,
    layout: crate::workbench::layout::LayoutSpec,
) {
    if let Some(entries) = F::registry().toolbar_entries(id) {
        crate::workbench::chrome::toolbar::feature_toolbar(
            ui,
            app,
            entries,
            layout,
            |app, ask| ask(&F::reads(app)),
            |app, ui, show, layout| show(&mut cx::<F>(app), ui, layout),
        );
    }
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
    use crate::workbench::framework::feature::CloudSessionPort;

    /// A feature's context lends the cloud's sessions as a peer, so what it
    /// signs carries the name the cloud gives.
    #[test]
    fn a_feature_context_reads_the_cloud_sessions_as_a_peer() {
        let mut app = RSpiceApp::test_instance();
        let expected = app.cloud.review_actor(&*app.state.cloud);
        let cx = cx::<DesignFeature>(&mut app);
        assert_eq!(
            cx.peers.cloud_session().review_actor(cx.peers.cloud()),
            expected
        );
    }

    /// The cloud feature holds its sessions as its own handles, never as a
    /// peer of itself.
    #[test]
    #[should_panic(expected = "the cloud feature holds its own sessions, never as a peer")]
    fn the_cloud_feature_holds_its_sessions_as_its_own() {
        let mut app = RSpiceApp::test_instance();
        let cx = cx::<CloudFeature>(&mut app);
        let _ = cx.peers.cloud_session();
    }

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
        let simulate = <SimulateFeature as Registered>::registry();
        let project = <ProjectFeature as Registered>::registry();
        let design = <DesignFeature as Registered>::registry();
        let code = <CodeFeature as Registered>::registry();
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
                simulate.command(*key).is_some(),
                project.command(*key).is_some(),
                design.command(*key).is_some(),
                code.command(*key).is_some(),
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
                + crate::workbench::simulate::commands::SimulateCommand::ALL.len()
                + crate::workbench::project::commands::ProjectCommand::ALL.len()
                + crate::workbench::design::commands::DesignCommand::ALL.len()
                + crate::workbench::code::commands::CodeCommand::ALL.len()
        );
    }

    /// Every command a feature or its menus read the dispatcher's answer
    /// for is one the dispatcher answers to, read once, and no feature reads
    /// more answers than a context holds.
    #[test]
    fn every_command_a_feature_reads_is_one_the_dispatcher_answers() {
        use crate::workbench::commands::vocabulary::legacy_command;

        for ((slot, foreign), (_, menus)) in
            foreign_availability().into_iter().zip(menu_availability())
        {
            let keys = [foreign, menus].concat();
            assert!(keys.len() <= ForeignAvailability::CAPACITY, "{slot}");
            for (index, key) in keys.iter().enumerate() {
                assert!(
                    spec(*key).is_some() || legacy_command(*key).is_some(),
                    "{slot} reads {key:?}, which no command answers to"
                );
                assert!(!keys[..index].contains(key), "{slot} reads {key:?} twice");
            }
        }
    }

    /// A lend for anything but an open menu, the deck page's while its menu
    /// is closed among them, asks the dispatcher for the code feature's three
    /// answers, the toolbar's every frame among them; the page's lend while
    /// its menu is open asks for its menu's eight after them, and holds each
    /// as the dispatcher gives it.
    #[test]
    fn a_plain_lend_takes_three_answers_and_a_menu_lend_eleven() {
        use crate::workbench::code::CodeFeature;

        let mut app = RSpiceApp::test_instance();
        let menu_answers: Vec<_> = CodeFeature::MENU_AVAILABILITY
            .iter()
            .map(|key| (*key, app.command_availability(*key)))
            .collect();
        assert_eq!(
            cx_with_foreign_answers::<CodeFeature>(&mut app, |_| true).foreign_answers_taken(),
            3
        );
        assert_eq!(
            cx_with_menu_answers::<CodeFeature>(&mut app, |_| false).foreign_answers_taken(),
            3
        );
        let cx = cx_with_menu_answers::<CodeFeature>(&mut app, |_| true);
        assert_eq!(cx.foreign_answers_taken(), 11);
        for (key, answer) in menu_answers {
            assert_eq!(cx.foreign_availability(key), answer, "{key:?}");
        }
    }

    /// A command a feature reads runs exactly when the dispatcher answers that
    /// it is available, so the answer a context holds, reason and all, is the
    /// one a raise of it meets. A feature's own command runs exactly when it
    /// is available; an application command does too unless its answer adds
    /// a refusal the run does not make, which only a command that changes
    /// the window session or runs only in a source editor does.
    #[test]
    fn a_command_a_feature_reads_runs_exactly_when_it_is_available() {
        use crate::workbench::code::commands::CodeCommand;
        use crate::workbench::commands::vocabulary::legacy_command;

        for (slot, keys) in foreign_availability() {
            for key in keys {
                if let Some(command) = legacy_command(*key) {
                    assert!(
                        !command.mutates_window_session(),
                        "{slot} reads {key:?}, whose answer refuses more than its run does"
                    );
                }
                assert!(
                    !CodeCommand::from_key(*key).is_some_and(CodeCommand::source_editor_only),
                    "{slot} reads {key:?}, whose answer refuses more than its run does"
                );
            }
        }
        for open in [false, true] {
            let mut app = RSpiceApp::test_instance();
            if open {
                app.state.open_project_session();
            }
            for (slot, keys) in foreign_availability() {
                for key in keys {
                    assert_eq!(
                        app.command_key_enabled(*key),
                        app.command_availability(*key).is_available(),
                        "{slot} reads {key:?} (project open: {open})"
                    );
                }
            }
        }
    }

    /// A command a feature leaves out of the registry is not ordered, and the
    /// key a container holds still reaches it.
    #[test]
    fn a_command_left_out_of_the_registry_is_reached_by_its_key() {
        use crate::workbench::hardcopy::commands::HardcopyCommand;
        use crate::workbench::simulate::commands::SimulateCommand;

        let registry = command_registry();
        let left_out = HardcopyCommand::REPORT_TOOLBAR
            .map(HardcopyCommand::key)
            .into_iter()
            .chain(SimulateCommand::UNLISTED.map(SimulateCommand::key));
        for key in left_out {
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
        let results = <ResultsFeature as Registered>::registry().registered_routed();
        let design = <DesignFeature as Registered>::registry().registered_routed();
        assert!(!results.is_empty() && !design.is_empty());
        let keys = results.iter().map(|part| part.key);
        for key in keys.chain(design.iter().map(|part| part.key)) {
            assert!(registry.order().contains(&key), "{key:?}");
            assert!(spec(key).is_none(), "{key:?}");
        }
    }

    /// A routed command's part reads whether an application-modal dialog is
    /// open as the shell lends its context: open while Preferences is, and
    /// closed once it is not.
    #[test]
    fn a_routed_part_reads_whether_an_application_modal_dialog_is_open() {
        use std::cell::Cell;

        thread_local! {
            static SEEN: Cell<Option<bool>> = const { Cell::new(None) };
        }
        const PROBE: RoutedCommand<DesignFeature> = RoutedCommand {
            key: CommandKey::new("probe"),
            takes: |_| true,
            enabled: |_| true,
            execute: |cx| SEEN.set(Some(cx.application_modal_open())),
        };
        let mut app = RSpiceApp::test_instance();
        for open in [true, false] {
            app.state.settings.preferences_open = open;
            assert_eq!(app.state.application_modal_open(), open);
            assert!(route_part(&mut app, &PROBE));
            assert_eq!(SEEN.get(), Some(open), "Preferences open: {open}");
        }
    }

    /// A feature command's context reads whether anything the dialog gate
    /// counts is open as the shell lends it for the run: open while About
    /// is, and closed once it is not.
    #[test]
    fn a_feature_command_reads_whether_a_dialog_is_open() {
        let mut app = RSpiceApp::test_instance();
        for open in [true, false] {
            app.state.shell.about = open;
            assert_eq!(app.state.dialog_modal_open(), open);
            let cx = command_cx::<DesignFeature>(&mut app);
            assert_eq!(cx.dialog_modal_open(), open, "About open: {open}");
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

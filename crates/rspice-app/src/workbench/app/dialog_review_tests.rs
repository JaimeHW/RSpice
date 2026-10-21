//! The census of what every menu command puts on screen, rendered for review.
//!
//! Each registered command runs on a fresh application seeded the way a
//! reader leaves one: a project open on the RC low-pass example, its
//! transient run finished, the cellview checked and R1 selected. A command
//! that is unavailable there runs where it is offered instead: the netlist
//! page, a Verilog-A cell view, a symbol view. The project lifecycle commands
//! also run with no project open, where they are the only way in.
//!
//! Whatever a command opened is rendered at desktop size in both themes and
//! at tablet size with a touch pointer, and `index.md` beside the images says
//! what every command did, including the ones that opened nothing. Nothing on
//! these pages is seeded by hand: every row is one the product made.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use super::veriloga_design_tests::{add_veriloga_cell, open_in_design};
use super::*;
use crate::state::{
    AnalysisType, Cell, CellViewRef, Library, Point, PortDirection, SymbolDocument, SymbolPin,
    SymbolShape, View, ViewType,
};
use crate::workbench::commands::vocabulary::{COMMAND_REGISTRY, Command};
use crate::workbench::documents::code_workspace::CodeWorkspacePage;
use crate::workbench::preferences::TogglePreference;
use crate::workbench::state::Workspace;

const DESKTOP: egui::Vec2 = egui::vec2(1440.0, 900.0);
const TABLET: egui::Vec2 = egui::vec2(1024.0, 768.0);

/// Fonts build on the first pass and sizing settles on the second, so the
/// third is the one a reader would see.
const SETTLE_FRAMES: usize = 3;

/// Where a command runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seed {
    /// The RC low-pass project on Design, run, checked, R1 selected.
    Project,
    /// The same project on the Netlist page.
    Netlist,
    /// The same project with a Verilog-A cell view in front.
    VerilogA,
    /// The same project with a symbol view in front.
    Symbol,
    /// The application as it starts with no project open.
    NoProject,
}

impl Seed {
    const OFFERED: [Self; 4] = [Self::Project, Self::Netlist, Self::VerilogA, Self::Symbol];

    const fn label(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Netlist => "netlist page",
            Self::VerilogA => "Verilog-A view",
            Self::Symbol => "symbol view",
            Self::NoProject => "no project",
        }
    }
}

/// Commands that hand the reader to the operating system's file picker from
/// the seeded state. A headless pass cannot answer one, so they are listed
/// rather than run.
const SYSTEM_FILE_PICKERS: &[Command] = &[
    Command::OpenProject,
    Command::Save,
    Command::SaveAs,
    Command::SaveAll,
    Command::OpenDocument,
    Command::OpenNetlist,
    Command::ImportNetlist,
    Command::ImportVerilogA,
    Command::ImportResultDataset,
    Command::ExportSchematicSvg,
    Command::ExportPublicationSnapshot,
];

/// The commands a reader meets with no project open.
const NO_PROJECT_COMMANDS: &[Command] = &[
    Command::ProjectLauncher,
    Command::RecentProjects,
    Command::NewProject,
];

/// Window commands the registry leaves out but the Window menu names.
const UNREGISTERED_MENU_COMMANDS: &[Command] =
    &[Command::PreviousWorkspace, Command::NextWorkspace];

/// A context with the application's fonts bound before its first pass, as
/// the native host binds them before the first frame.
fn context(app: &RSpiceApp) -> egui::Context {
    let ctx = egui::Context::default();
    // A keycap reads "Cmd" on macOS and "Ctrl" elsewhere; a review must come
    // out the same on every machine.
    ctx.set_os(egui::os::OperatingSystem::Nix);
    ctx.enable_accesskit();
    app.state.ui.theme.apply(&ctx);
    ctx
}

/// The application under review.
struct Reader {
    app: RSpiceApp,
}

impl Reader {
    /// One application frame in the order the native frame loop runs it,
    /// less the autosave and the preferences file, which would write this
    /// machine's application data.
    fn frame(&mut self, ctx: &egui::Context, size: egui::Vec2) -> egui::FullOutput {
        let app = &mut self.app;
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ui| {
                let ctx = ui.ctx().clone();
                let before = FrameSimulationInput::capture(&app.state);
                app.prepare_frame(&ctx);
                app.render_frame_chrome(ui);
                app.render_secondary_application_windows(&ctx);
                app.refresh_live_checks(&ctx);
                app.render_frame_dialogs(&ctx);
                app.end_frame(&before);
            },
        )
    }

    fn settle(&mut self, ctx: &egui::Context, size: egui::Vec2) -> egui::FullOutput {
        let mut output = self.frame(ctx, size);
        for _ in 1..SETTLE_FRAMES {
            output = self.frame(ctx, size);
        }
        output
    }

    /// Run the plan the project starts with, as the toolbar's Run does, and
    /// wait for the engine to finish it.
    fn run_transient(&mut self, ctx: &egui::Context) {
        self.frame(ctx, DESKTOP);
        assert!(
            Command::RunSimulation.is_enabled(&self.app),
            "the seeded project can run"
        );
        Command::RunSimulation.execute(&mut self.app);
        let deadline = Instant::now() + Duration::from_secs(180);
        loop {
            self.frame(ctx, DESKTOP);
            let state = &self.app.state;
            if let Some(report) = state.workbench.preflight.report.as_ref() {
                assert!(
                    report.blockers.is_empty(),
                    "the run is blocked: {:#?}",
                    report.blockers
                );
            }
            if !state.simulation.retained.runs.is_empty()
                && !state.simulation.execution.has_active_execution()
            {
                break;
            }
            assert!(Instant::now() < deadline, "timed out waiting for the run");
            // A frame of the whole workbench is the expensive part of a seed;
            // the engine runs on its own thread meanwhile.
            std::thread::sleep(Duration::from_millis(100));
        }
        let run = self
            .app
            .state
            .simulation
            .retained
            .runs
            .last()
            .expect("the run is retained");
        assert!(
            run.analyses.iter().any(|analysis| {
                analysis.analysis_type == AnalysisType::Transient && analysis.success
            }),
            "the default plan's transient analysis succeeded: {:?}",
            run.analyses
                .iter()
                .map(|analysis| (&analysis.label, &analysis.error_message))
                .collect::<Vec<_>>()
        );
    }

    /// The frames a fresh context draws, as a reader would first see them.
    fn look(&mut self, mode: crate::ui::Mode, size: egui::Vec2, coarse: bool) -> Shot {
        self.app.state.ui.theme.mode = mode;
        self.app.state.workbench.coarse_pointer = coarse;
        // The theme is applied to a context only when it changes, and this
        // context has never had one.
        self.app.applied_theme = None;
        let ctx = context(&self.app);
        let output = self.settle(&ctx, size);
        Shot { ctx, output }
    }
}

/// A symbol view of an amplifier cell: two pins on a body.
fn open_symbol_view(state: &mut AppState) {
    let mut library = Library::new("analog");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut view = View::new("symbol", ViewType::Symbol);
    SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
        ],
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 20,
        }],
        ..SymbolDocument::default()
    }
    .store_in_view(&mut view)
    .expect("the symbol stores");
    cell.add_view(view);
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("analog", "amp", "symbol"));
    state.workbench.activate(Workspace::Design);
}

/// A fresh application in `seed`'s state, and what its last frame showed.
fn seeded(seed: Seed) -> (Reader, Observation) {
    let mut reader = Reader {
        app: RSpiceApp::test_instance(),
    };
    let app = &mut reader.app;
    // A headless pass has no clock for an opening fade to run against, so a
    // dialog would be rasterized part-transparent over the workbench.
    app.state
        .ui
        .preferences
        .set_toggle(TogglePreference::ReducedMotion, true);
    app.state.ui.theme.mode = crate::ui::Mode::Light;
    app.symbol_library = crate::schematic::symbols::SymbolLibrary::load_embedded().ok();
    let ctx = context(app);
    if seed == Seed::NoProject {
        let output = reader.settle(&ctx, DESKTOP);
        let seen = observe(&ctx, &output, &reader.app);
        return (reader, seen);
    }
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    crate::workbench::examples::load_example("RC Lowpass Filter", &mut app.state.schematic);
    // The default plan steps a millisecond at 10 ns: a hundred thousand
    // points, which an unoptimized engine takes seconds over per seed. A
    // microsecond still resolves the filter's 159 us time constant.
    app.state.sim_setup.tran.step = "1u".to_owned();
    reader.run_transient(&ctx);
    let app = &mut reader.app;
    // The run leaves the reader on Simulate with its notice up; they return
    // to the design and close the notice.
    app.state.workbench.activate(Workspace::Design);
    let notices: Vec<u64> = app
        .state
        .ui
        .toasts
        .activity()
        .iter()
        .map(|record| record.id())
        .collect();
    for id in notices {
        app.state.ui.toasts.dismiss(id);
    }
    Command::CheckCellview.execute(app);
    let resistor = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "R1")
        .map(|component| component.id)
        .expect("the example draws R1");
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(resistor);
    match seed {
        Seed::Project | Seed::NoProject => {}
        Seed::Netlist => Command::OpenWorkspace(Workspace::Netlist).execute(app),
        Seed::VerilogA => {
            let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "gain.va", "gain");
            open_in_design(&mut app.state, &reference);
        }
        Seed::Symbol => open_symbol_view(&mut app.state),
    }
    let output = reader.settle(&ctx, DESKTOP);
    let seen = observe(&ctx, &output, &reader.app);
    (reader, seen)
}

/// What a frame put in front of the reader.
#[derive(Debug, Default)]
struct Observation {
    /// Every dialog and window the frame named.
    surfaces: BTreeSet<String>,
    /// Every layer above the workbench that was visible, with where it was
    /// when it did not cover the whole viewport (a modal's scrim does).
    layers: BTreeMap<String, Option<egui::Rect>>,
    workspace: String,
    surface: String,
    prompt: Option<String>,
}

fn observe(ctx: &egui::Context, output: &egui::FullOutput, app: &RSpiceApp) -> Observation {
    let surfaces = output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .filter(|(_, node)| {
            matches!(
                node.role(),
                egui::accesskit::Role::Dialog
                    | egui::accesskit::Role::AlertDialog
                    | egui::accesskit::Role::Window
            )
        })
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect();
    let screen = ctx.content_rect();
    let layers = ctx
        .memory(|memory| memory.areas().visible_layer_ids())
        .into_iter()
        .filter(|layer| layer.order != egui::Order::Background)
        .map(|layer| {
            let rect = egui::AreaState::load(ctx, layer.id)
                .map(|area| area.rect())
                .filter(|rect| !rect.contains_rect(screen));
            (format!("{layer:?}"), rect)
        })
        .collect();
    Observation {
        surfaces,
        layers,
        workspace: app.state.workbench.workspace.label().to_owned(),
        surface: format!("{:?}", app.state.workbench.current_route().surface_id()),
        prompt: app.state.prompt_text().map(str::to_owned),
    }
}

struct Shot {
    ctx: egui::Context,
    output: egui::FullOutput,
}

impl Shot {
    fn write(&self, directory: &Path, name: &str, size: egui::Vec2) {
        let canvas = crate::ui::raster::rasterize(&self.ctx, size, &self.output);
        std::fs::write(directory.join(name), canvas.png(canvas.height()))
            .expect("write the render");
    }
}

/// A file-name stem for a command identity.
fn stem(command: Command) -> String {
    command
        .stable_id()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

/// One row of the index.
struct Row {
    order: usize,
    text: String,
    renders: usize,
}

fn availability_text(command: Command, app: &RSpiceApp) -> Option<String> {
    let availability = command.availability(app);
    if availability.is_available() {
        return None;
    }
    // `Disabled("why")` or `Hidden`, in the vocabulary's own words.
    let text = format!("{availability:?}");
    Some(
        text.strip_prefix("Disabled(\"")
            .and_then(|rest| rest.strip_suffix("\")"))
            .map_or_else(
                || "not shown".to_owned(),
                |reason| format!("unavailable: {reason}"),
            ),
    )
}

/// Run `command` on a fresh `seed` application and render what it opened.
fn census(command: Command, seed: Seed, suffix: &str, directory: &Path) -> (String, Vec<String>) {
    let (mut reader, before) = seeded(seed);
    if matches!(command, Command::ExportNetlist(_)) {
        // Off the netlist page the export goes straight to the system's
        // save picker; on it, the page asks for the format first.
        let state = &reader.app.state;
        let on_page = state.workbench.workspace == Workspace::Netlist
            && state.ui.code_workspace.page == CodeWorkspacePage::Netlist;
        if !on_page {
            return (
                "not run: opens the system save picker".to_owned(),
                Vec::new(),
            );
        }
    }
    command.execute(&mut reader.app);
    let light = reader.look(crate::ui::Mode::Light, DESKTOP, false);
    let after = observe(&light.ctx, &light.output, &reader.app);

    let mut what = Vec::new();
    let opened: Vec<_> = after.surfaces.difference(&before.surfaces).collect();
    let new_layers: Vec<Option<egui::Rect>> = after
        .layers
        .iter()
        .filter(|(layer, _)| !before.layers.contains_key(*layer))
        .map(|(_, rect)| *rect)
        .collect();
    let placed: Vec<String> = new_layers
        .iter()
        .flatten()
        .map(|rect| {
            format!(
                "x {:.0}..{:.0}, y {:.0}..{:.0}",
                rect.left(),
                rect.right(),
                rect.top(),
                rect.bottom()
            )
        })
        .collect();
    let placed = if placed.is_empty() {
        String::new()
    } else {
        format!(" at {}", placed.join(" and "))
    };
    for title in &opened {
        what.push(format!("opened \u{201c}{title}\u{201d}{placed}"));
    }
    if opened.is_empty() && !new_layers.is_empty() {
        what.push(format!("opened an untitled layer{placed}"));
    }
    if after.workspace != before.workspace {
        what.push(format!("switched to {}", after.workspace));
    } else if after.surface != before.surface && opened.is_empty() {
        what.push(format!("moved to {}", after.surface));
    }
    if after.prompt != before.prompt
        && let Some(prompt) = &after.prompt
    {
        what.push(format!("said \u{201c}{prompt}\u{201d}"));
    }
    if what.is_empty() {
        what.push("nothing visible changed".to_owned());
    }

    let mut renders = Vec::new();
    if !opened.is_empty() || !new_layers.is_empty() {
        let stem = format!("{}{suffix}", stem(command));
        let name = format!("{stem}-light-1440x900.png");
        light.write(directory, &name, DESKTOP);
        renders.push(name);
        let dark = reader.look(crate::ui::Mode::Dark, DESKTOP, false);
        let name = format!("{stem}-dark-1440x900.png");
        dark.write(directory, &name, DESKTOP);
        renders.push(name);
        let touch = reader.look(crate::ui::Mode::Light, TABLET, true);
        let name = format!("{stem}-light-1024x768.png");
        touch.write(directory, &name, TABLET);
        renders.push(name);
    }
    (what.join("; "), renders)
}

/// Render everything the menus open, and index it.
#[test]
#[ignore = "writes PNGs and an index for a human to review; run with --ignored"]
fn render_every_menu_dialog_for_review() {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    // A comma-separated list of command identities narrows a review run.
    let only: Option<BTreeSet<String>> = std::env::var("RSPICE_CENSUS_ONLY")
        .ok()
        .map(|list| list.split(',').map(str::to_owned).collect());
    // Where each command would run, without running any.
    let plan_only = std::env::var_os("RSPICE_CENSUS_PLAN").is_some();
    let threads: usize = std::env::var("RSPICE_CENSUS_THREADS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8);

    let commands: Vec<(usize, Command)> = COMMAND_REGISTRY
        .iter()
        .chain(UNREGISTERED_MENU_COMMANDS)
        .copied()
        .enumerate()
        .filter(|(_, command)| {
            only.as_ref()
                .is_none_or(|only| only.contains(command.stable_id()))
        })
        .collect();
    // Where each command is offered, measured once on one application per
    // seed rather than once per command: `None` when it is, else why not.
    let refusals: Vec<(Seed, Vec<Option<String>>)> = std::thread::scope(|scope| {
        let measured: Vec<_> = Seed::OFFERED
            .into_iter()
            .chain([Seed::NoProject])
            .map(|seed| {
                let commands = &commands;
                let measure = scope.spawn(move || {
                    let (reader, _) = seeded(seed);
                    commands
                        .iter()
                        .map(|(_, command)| availability_text(*command, &reader.app))
                        .collect::<Vec<_>>()
                });
                (seed, measure)
            })
            .collect();
        measured
            .into_iter()
            .map(|(seed, measure)| (seed, measure.join().expect("the seed builds")))
            .collect()
    });

    let mut jobs: Vec<(usize, Command, Option<Seed>, &str, String)> = Vec::new();
    for (index, (order, command)) in commands.iter().copied().enumerate() {
        let offered = &refusals[..Seed::OFFERED.len()];
        // A netlist export asks for its format only on the netlist page;
        // anywhere else it goes straight to the system's save picker.
        let preferred = matches!(command, Command::ExportNetlist(_)).then_some(Seed::Netlist);
        let seed = offered
            .iter()
            .filter(|(seed, _)| preferred.is_none_or(|preferred| preferred == *seed))
            .find(|(_, refusals)| refusals[index].is_none())
            .map(|(seed, _)| *seed);
        let refusal = offered[0].1[index].clone().unwrap_or_default();
        jobs.push((order * 2, command, seed, "", refusal));
        if NO_PROJECT_COMMANDS.contains(&command) {
            let refusal = refusals[Seed::OFFERED.len()].1[index].clone();
            jobs.push((
                order * 2 + 1,
                command,
                refusal.is_none().then_some(Seed::NoProject),
                "-no-project",
                refusal.unwrap_or_default(),
            ));
        }
    }

    let next = AtomicUsize::new(0);
    let rows = Mutex::new(Vec::<Row>::new());
    std::thread::scope(|scope| {
        for _ in 0..threads.max(1) {
            scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some((order, command, seed, suffix, refusal)) = jobs.get(index) else {
                        break;
                    };
                    let spec = command.spec();
                    let (ran_in, what, renders) = match seed {
                        None => ("nowhere", refusal.clone(), Vec::new()),
                        Some(_) if SYSTEM_FILE_PICKERS.contains(command) => (
                            "not run",
                            "opens the system file picker".to_owned(),
                            Vec::new(),
                        ),
                        Some(seed) if plan_only => (seed.label(), String::new(), Vec::new()),
                        Some(seed) => {
                            let outcome =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    census(*command, *seed, suffix, &directory)
                                }));
                            match outcome {
                                Ok((what, renders)) => (seed.label(), what, renders),
                                Err(panic) => {
                                    let message = panic
                                        .downcast_ref::<String>()
                                        .cloned()
                                        .or_else(|| {
                                            panic.downcast_ref::<&str>().map(|s| (*s).to_owned())
                                        })
                                        .unwrap_or_default();
                                    (seed.label(), format!("PANICKED: {message}"), Vec::new())
                                }
                            }
                        }
                    };
                    let text = format!(
                        "| `{}{suffix}` | {} | {ran_in} | {} | {} |",
                        spec.id,
                        spec.label,
                        what.replace('|', "\\|"),
                        renders.join("<br>")
                    );
                    rows.lock().expect("census rows").push(Row {
                        order: *order,
                        text,
                        renders: renders.len(),
                    });
                }
            });
        }
    });

    let mut rows = rows.into_inner().expect("census rows");
    rows.sort_by_key(|row| row.order);
    let rendered = rows.iter().filter(|row| row.renders > 0).count();
    let images: usize = rows.iter().map(|row| row.renders).sum();
    let mut index = String::new();
    writeln!(index, "# Menu command census\n").ok();
    writeln!(
        index,
        "Every registered command, run on a fresh application: a project open on Design on the RC \
         low-pass example, its default transient run (1 ms at 1 us steps) finished and its notice \
         closed, the cellview checked and R1 selected. A command \
         unavailable there ran where it is offered (netlist page, Verilog-A view, symbol view); \
         the project lifecycle commands also ran with no project open. What a command opened is \
         rendered at 1440x900 in light and dark and at 1024x768 in light with a touch pointer.\n"
    )
    .ok();
    writeln!(
        index,
        "{} commands, {rendered} opened something, {images} images.\n",
        rows.len()
    )
    .ok();
    writeln!(
        index,
        "| Command | Label | Ran in | What it did | Renders |"
    )
    .ok();
    writeln!(index, "|---|---|---|---|---|").ok();
    for row in &rows {
        writeln!(index, "{}", row.text).ok();
    }
    std::fs::write(directory.join("index.md"), index).expect("write the index");
}

/// How a Help dialog is opened for review.
#[derive(Debug, Clone, Copy)]
enum Opens {
    Command(Command),
    /// The Examples dialog, which no menu row offers yet.
    Examples,
    /// The license dialog over a key it verified, before Activate.
    VerifiedKey,
}

/// The Help dialogs, by the name their renders are filed under.
const HELP_DIALOGS: [(&str, Seed, Opens); 9] = [
    ("license-verified", Seed::Project, Opens::VerifiedKey),
    ("about", Seed::Project, Opens::Command(Command::About)),
    (
        "help-center",
        Seed::Project,
        Opens::Command(Command::HelpCenter),
    ),
    (
        "legal",
        Seed::Project,
        Opens::Command(Command::LegalPrivacy),
    ),
    (
        "report-problem",
        Seed::Project,
        Opens::Command(Command::SystemDiagnostics),
    ),
    ("examples", Seed::Project, Opens::Examples),
    ("examples-no-project", Seed::NoProject, Opens::Examples),
    ("license", Seed::Project, Opens::Command(Command::License)),
    (
        "command-reference",
        Seed::Project,
        Opens::Command(Command::KeyboardShortcuts),
    ),
];

/// The Help dialogs, each rendered: light and dark at desktop size, light
/// at tablet size with a touch pointer.
#[test]
#[ignore = "writes PNGs for a human to review; run with --ignored"]
fn render_the_help_dialogs_for_review() {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    std::thread::scope(|scope| {
        for (name, seed, opens) in HELP_DIALOGS {
            let directory = &directory;
            scope.spawn(move || {
                let (mut reader, _) = seeded(seed);
                match opens {
                    Opens::Command(command) => command.execute(&mut reader.app),
                    Opens::VerifiedKey => {
                        use crate::services::license;
                        Command::License.execute(&mut reader.app);
                        let dialog = &mut reader.app.state.dialogs.license_dialog;
                        dialog.text = license::SAMPLE_KEY.to_owned();
                        dialog.phase =
                            crate::workbench::app::dialogs::license::LicensePhase::Verified(
                                license::parse_and_verify(license::SAMPLE_KEY)
                                    .expect("the sample key verifies in a test build"),
                            );
                    }
                    Opens::Examples => reader.app.state.workbench.open_examples(),
                }
                let light = reader.look(crate::ui::Mode::Light, DESKTOP, false);
                light.write(directory, &format!("{name}-light-1440x900.png"), DESKTOP);
                let dark = reader.look(crate::ui::Mode::Dark, DESKTOP, false);
                dark.write(directory, &format!("{name}-dark-1440x900.png"), DESKTOP);
                let touch = reader.look(crate::ui::Mode::Light, TABLET, true);
                touch.write(directory, &format!("{name}-light-1024x768.png"), TABLET);
            });
        }
    });
}

//! The Verilog-A view's navigator, driven the way a reader drives it.
//!
//! Every frame is a whole application frame, every row is found by the name
//! it announces and pressed where it was drawn, and what a press did is read
//! back from the editor and the schematic it was meant to move.

use super::veriloga_design_tests::{DESKTOP, open_in_design, open_project_app};
use super::*;
use crate::state::{
    Cell, CellViewRef, Library, LibraryCellInstance, Point, ProjectSourceDependency,
    ProjectSourceFile, ProjectSourceId, ProjectSourceLanguage, ProjectSourceOwner,
    ProjectSourceRole, ProjectSourceRoleBinding, View, ViewType,
};
use crate::workbench::{MessageCatalog, MessageId, UiTextLocale};

const LIBRARY: &str = "behavioral";
const ROOT: &str = "gain.va";
const PROFILE: &str = "gain.build.toml";
const SOURCE: &str = "`include \"disciplines.vams\"\n`include \"missing.vams\"\nmodule gain(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = 1m from (0:inf);\n  analog I(p, n) <+ g * V(p, n);\nendmodule\n";

/// A `behavioral/<cell>/veriloga` view binding module `cell` with ports `p`
/// and `n`, whose bundle is `root` holding `source`, a build profile beside
/// it, and `includes`; and an instance of it placed in the top schematic the
/// way the library shelf places one. Returns the view, its bundle and the
/// instance.
pub(super) fn placed_veriloga_cell(
    state: &mut AppState,
    cell: &str,
    root: &str,
    source: &str,
    includes: &[(&str, &str)],
) -> (CellViewRef, ProjectSourceId, u64) {
    let reference = CellViewRef::new(LIBRARY, cell, "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), cell.to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut library_cell = Cell::new(cell);
    library_cell.add_view(view);
    if state.library_manager.get_library(LIBRARY).is_none() {
        state.library_manager.add_library(Library::new(LIBRARY));
    }
    state
        .library_manager
        .get_library_mut(LIBRARY)
        .expect("the behavioral library")
        .add_cell(library_cell);
    let profile_path = format!("{}.build.toml", root.trim_end_matches(".va"));
    let profile =
        rspice_simulation::project_veriloga::build_profile::VerilogABuildProfile::starter(cell)
            .to_toml()
            .expect("the starter profile encodes");
    let files = std::iter::once((profile_path.as_str(), profile.as_str()))
        .chain(includes.iter().copied())
        .map(|(path, text)| ProjectSourceFile::try_new(path, text).unwrap())
        .collect::<Vec<_>>();
    let dependencies = files
        .iter()
        .map(|file| ProjectSourceDependency::try_new(root, file.logical_path()).unwrap())
        .collect::<Vec<_>>();
    let bundle =
        crate::state::ProjectSourceBundle::try_new_with_roles(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            root,
            source,
            files,
            dependencies,
            [ProjectSourceRoleBinding::try_new(
                &profile_path,
                ProjectSourceRole::VerilogABuildProfile,
            )
            .unwrap()],
        )
        .expect("a valid cell-view bundle");
    let bundle_id = bundle.id();
    state
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");

    let ports = rspice_design::library::metadata::metadata_ports(
        &state
            .library_manager
            .get_library(LIBRARY)
            .and_then(|library| library.get_cell(cell))
            .and_then(|cell| cell.get_view("veriloga"))
            .expect("the view")
            .metadata,
    )
    .expect("the view states its ports");
    state.open_workspace_view(CellViewRef::default_top());
    let mut binding = LibraryCellInstance::new(LIBRARY, cell, "veriloga");
    binding.bind_interface(&ports);
    let instance = state
        .schematic
        .add_library_cell_component(Point::new(20, 20), binding);
    (reference, bundle_id, instance)
}

/// Someone at the application with the Verilog-A view in front of them.
struct Reader {
    ctx: egui::Context,
    app: RSpiceApp,
    named: Vec<(String, egui::Rect)>,
}

impl Reader {
    fn new() -> (Self, CellViewRef, ProjectSourceId, u64) {
        let mut app = open_project_app();
        let (reference, bundle_id, instance) =
            placed_veriloga_cell(&mut app.state, "gain", ROOT, SOURCE, &[]);
        open_in_design(&mut app.state, &reference);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut reader = Self {
            ctx,
            app,
            named: Vec::new(),
        };
        reader.settle();
        (reader, reference, bundle_id, instance)
    }

    fn frame(&mut self, events: Vec<egui::Event>) {
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, DESKTOP)),
                events,
                ..Default::default()
            },
            |ui| {
                self.app.prepare_frame(ui.ctx());
                self.app.render_frame_chrome(ui);
                self.app.render_frame_dialogs(ui.ctx());
            },
        );
        self.named = output
            .platform_output
            .accesskit_update
            .into_iter()
            .flat_map(|update| update.nodes)
            .filter_map(|(_, node)| {
                let name = node.label().or_else(|| node.value())?.to_owned();
                let bounds = node.bounds()?;
                Some((
                    name,
                    egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                ))
            })
            .collect();
    }

    fn settle(&mut self) {
        self.frame(Vec::new());
        self.frame(Vec::new());
    }

    fn names(&self) -> Vec<&str> {
        self.named.iter().map(|(name, _)| name.as_str()).collect()
    }

    fn shows(&self, name: &str) -> bool {
        self.named.iter().any(|(candidate, _)| candidate == name)
    }

    /// Where the navigator drew `name`: the leftmost of the places it is
    /// named, since the navigator is the leftmost dock.
    fn navigator_rect(&self, name: &str) -> egui::Rect {
        self.named
            .iter()
            .filter(|(candidate, _)| candidate == name)
            .map(|(_, rect)| *rect)
            .min_by(|left, right| left.left().total_cmp(&right.left()))
            .unwrap_or_else(|| panic!("nothing is named {name:?}; named: {:?}", self.names()))
    }

    fn click(&mut self, name: &str) {
        let at = self
            .named
            .iter()
            .find(|(candidate, _)| candidate == name)
            .map(|(_, rect)| rect.center())
            .unwrap_or_else(|| panic!("nothing is named {name:?}; named: {:?}", self.names()));
        self.frame(vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]);
        self.frame(vec![egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        self.settle();
    }

    fn caret(&self, bundle_id: ProjectSourceId, path: &str) -> Option<usize> {
        self.app
            .state
            .ui
            .code_workspace
            .source_carets
            .get(&(bundle_id, path.to_owned()))
            .copied()
    }
}

fn text(id: MessageId) -> String {
    MessageCatalog::new(UiTextLocale::EnglishUnitedStates).text(id)
}

/// Character index of the start of one-based `line` of [`SOURCE`].
fn line_start(line: usize) -> usize {
    SOURCE
        .split_inclusive('\n')
        .take(line - 1)
        .map(|text| text.chars().count())
        .sum()
}

/// The panel used to open on `/ behavioral / gain / veriloga`, which the
/// document tab already says, and to label every file "source". It opens on
/// the files, and only the build profile states a role.
#[test]
fn the_panel_opens_on_its_files_and_states_only_a_role_that_is_not_source() {
    let (reader, ..) = Reader::new();

    assert!(
        !reader.shows("/ behavioral / gain / veriloga"),
        "the view's path is the tab's to state; named: {:?}",
        reader.names()
    );
    assert!(reader.shows(ROOT), "named: {:?}", reader.names());
    assert!(
        !reader
            .names()
            .iter()
            .any(|name| name.starts_with(&format!("{ROOT}, "))),
        "an ordinary source row states no role; named: {:?}",
        reader.names()
    );
    assert!(
        reader.shows(&format!(
            "{PROFILE}, {}",
            text(MessageId::CodeNavigatorRoleBuildProfile)
        )),
        "named: {:?}",
        reader.names()
    );
}

/// Used in finds the instance a real placement made, and pressing it opens
/// the schematic with that instance selected.
#[test]
fn used_in_lists_a_placed_instance_and_opens_it_selected() {
    let (mut reader, _, _, instance) = Reader::new();
    let top = CellViewRef::default_top();
    let name = reader
        .app
        .state
        .workspace
        .content
        .schematic_buffers
        .get(&top.key())
        .and_then(|schematic| {
            schematic
                .as_ref()
                .components
                .iter()
                .find(|component| component.id == instance)
                .map(|component| component.name.clone())
        })
        .expect("the placed instance is in the top schematic");
    let row = format!("{} / {name}", top.cell);
    assert!(
        reader.shows(&format!("{}, 1", text(MessageId::CodeNavigatorUsedIn))),
        "the header counts the one instance; named: {:?}",
        reader.names()
    );

    reader.click(&row);

    assert_eq!(reader.app.state.workspace.content.active_view, top);
    let selection = &reader.app.state.schematic.session.editor.selection;
    assert!(
        selection.components.contains(&instance) && selection.components.len() == 1,
        "the instance alone is selected: {:?}",
        selection.components
    );
}

/// A built-in header opens read-only in the editor well; choosing a file of
/// the bundle returns to editing it.
#[test]
fn a_built_in_include_is_read_in_place_and_a_file_returns_to_editing() {
    let (mut reader, ..) = Reader::new();
    let editor = text(MessageId::EditorVerilogASourceEditor);
    let viewer = text(MessageId::EditorVerilogASourceViewer);
    assert!(reader.shows(&editor));

    reader.click(&format!(
        "disciplines.vams, {}",
        text(MessageId::CodeNavigatorIncludeBuiltIn)
    ));

    assert!(
        reader.shows(&viewer) && !reader.shows(&editor),
        "the header is read, not edited; named: {:?}",
        reader.names()
    );

    reader.click(ROOT);

    assert!(reader.shows(&editor) && !reader.shows(&viewer));
}

/// An include nothing holds is marked as an error beside its neutral name,
/// and pressing it puts the caret on its directive.
#[test]
fn a_missing_include_takes_the_reader_to_its_directive() {
    let (mut reader, _, bundle_id, _) = Reader::new();

    reader.click("Error: missing.vams");

    assert_eq!(reader.caret(bundle_id, ROOT), Some(line_start(2)));
}

/// A group's caption takes one row of the outline and is centred on it, so
/// the rows keep one pitch down through every group rather than drifting at
/// each caption.
#[test]
fn outline_captions_keep_the_row_pitch() {
    let (reader, ..) = Reader::new();
    let rows = [
        text(MessageId::CodeNavigatorOutlineModule),
        "gain".to_owned(),
        text(MessageId::CodeNavigatorOutlinePorts),
        "p, inout electrical".to_owned(),
        "n, inout electrical".to_owned(),
        text(MessageId::CodeNavigatorOutlineParameters),
        "g, real = 1m from (0:inf)".to_owned(),
    ];
    let centres = rows
        .iter()
        .map(|row| reader.navigator_rect(row).center().y)
        .collect::<Vec<_>>();
    let pitch = centres[4] - centres[3];
    assert!(pitch > 0.0, "the two ports are stacked: {centres:?}");
    for (pair, centre) in rows.windows(2).zip(centres.windows(2)) {
        assert_eq!(
            centre[1] - centre[0],
            pitch,
            "{} to {}: {centres:?}",
            pair[0],
            pair[1]
        );
    }
}

/// An outline row puts the caret on the line that declares it — the line
/// itself, not the one after it.
#[test]
fn an_outline_row_takes_the_reader_to_its_declaration() {
    let (mut reader, _, bundle_id, _) = Reader::new();

    reader.click("g, real = 1m from (0:inf)");

    assert_eq!(reader.caret(bundle_id, ROOT), Some(line_start(6)));
}

//! The pin of every menu: each menu's rows exactly, read in every context
//! that changes what a menu shows, so a change to a menu is a change to
//! these lists.

use super::*;

/// Every row `entries` show for `app`, hidden ones included, as
/// `label | command id | key | state`, then `| checked` or `| unchecked` for
/// a row that switches a state; separators read `---` and a submenu row
/// `label >`. Keys are the Windows desktop ones.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn menu_pin(entries: &[MenuEntry], app: &RSpiceApp) -> Vec<String> {
    menu_pin_on(entries, app, egui::os::OperatingSystem::Windows)
}

/// `entries` as rows read on the desktop on `os`, a submenu's rows under
/// its row, indented.
pub(super) fn menu_pin_on(
    entries: &[MenuEntry],
    app: &RSpiceApp,
    os: egui::os::OperatingSystem,
) -> Vec<String> {
    menu_pin_for(
        entries,
        app,
        crate::state::command_catalog::CommandPlatform::Desktop,
        os,
    )
}

/// `entries` as rows read on `platform` and `os`.
pub(super) fn menu_pin_for(
    entries: &[MenuEntry],
    app: &RSpiceApp,
    platform: crate::state::command_catalog::CommandPlatform,
    os: egui::os::OperatingSystem,
) -> Vec<String> {
    fn pin(rows: &[menu_entries::MenuRow], indent: &str, out: &mut Vec<String>) {
        for row in rows {
            match row {
                menu_entries::MenuRow::Command(row) => {
                    let state = match row.availability {
                        CommandAvailability::Available => "on".to_owned(),
                        CommandAvailability::Disabled(reason) => format!("off: {reason}"),
                        CommandAvailability::Hidden => "hidden".to_owned(),
                    };
                    let mark = match (row.checked, row.radio) {
                        (Some(true), true) => " | selected",
                        (Some(false), true) => " | unselected",
                        (Some(true), false) => " | checked",
                        (Some(false), false) => " | unchecked",
                        (None, _) => "",
                    };
                    out.push(format!(
                        "{indent}{} | {} | {} | {state}{mark}",
                        row.label,
                        row.command.as_str(),
                        row.shortcut
                    ));
                }
                menu_entries::MenuRow::Separator => out.push(format!("{indent}---")),
                menu_entries::MenuRow::Submenu { label, rows } => {
                    out.push(format!("{indent}{label} >"));
                    pin(rows, &format!("{indent}  "), out);
                }
            }
        }
    }
    let mut out = Vec::new();
    pin(
        &menu_entries::resolve_menu_rows(entries, app, platform, os),
        "",
        &mut out,
    );
    out
}

/// The application with no project open: the start screen.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn no_project_app() -> RSpiceApp {
    let mut app = title_test_app();
    app.state.close_project_session();
    app
}

/// The application with the Design workspace showing a symbol view.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn symbol_view_app() -> RSpiceApp {
    use crate::state::{
        Cell, CellViewRef, Library, Point, SymbolDocument, SymbolShape, View, ViewType,
    };
    let mut app = title_test_app();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut view = View::new("symbol", ViewType::Symbol);
    SymbolDocument {
        body: vec![SymbolShape::Dot {
            center: Point::new(20, 0),
            radius: 2,
        }],
        ..SymbolDocument::default()
    }
    .store_in_view(&mut view)
    .expect("a symbol stores");
    cell.add_view(view);
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    app.state.workbench.activate(Workspace::Design);
    app
}

/// The application with the Design workspace showing a Verilog-A view.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn veriloga_view_app() -> RSpiceApp {
    use crate::state::{Cell, CellViewRef, Library, View, ViewType};
    let mut app = title_test_app();
    let mut library = Library::new("work");
    let mut cell = Cell::new("gain");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    cell.add_view(view);
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .open_workspace_view(CellViewRef::new("work", "gain", "veriloga"));
    app.state.workbench.activate(Workspace::Design);
    app
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn source_editor_app() -> RSpiceApp {
    let mut app = title_test_app();
    app.state.workbench.active_source_editor = Some(
        crate::workbench::documents::text_editor_commands::ActiveSourceEditor {
            id: Id::new("menu-pin-source"),
            editable: true,
            navigable: true,
        },
    );
    app
}

/// Each menu's rows exactly, for an open project with nothing selected and
/// for a source editor in front, so a change to what a menu shows is a
/// change to this list.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_menu_shows_exactly_its_pinned_rows() {
    let mut mismatches = Vec::new();
    let mut check = |name: &str, actual: Vec<String>, expected: &[&str]| {
        if actual != expected {
            mismatches.push(format!(
                "{name}:\n{}",
                actual
                    .iter()
                    .map(|line| format!("        {line:?},"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }
    };
    for (menu, expected) in PINNED_MENUS {
        check(
            menu.label(),
            menu_pin(application_menu_entries(menu), &title_test_app()),
            expected,
        );
    }
    for (menu, expected) in PINNED_SOURCE_EDITOR_MENUS {
        check(
            &format!("{} with a source editor", menu.label()),
            menu_pin(application_menu_entries(menu), &source_editor_app()),
            expected,
        );
    }
    for (menu, expected) in PINNED_SYMBOL_MENUS {
        check(
            &format!("{} with a symbol open", menu.label()),
            menu_pin(application_menu_entries(menu), &symbol_view_app()),
            expected,
        );
    }
    for (menu, expected) in PINNED_NO_PROJECT_MENUS {
        check(
            &format!("{} with no project open", menu.label()),
            menu_pin(application_menu_entries(menu), &no_project_app()),
            expected,
        );
    }
    check(
        "Compact Menu button",
        menu_pin(title_test_app().compact_menu(), &title_test_app())
            .into_iter()
            .filter(|row| !row.starts_with(' '))
            .collect(),
        &PINNED_COMPACT_MENU,
    );
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n\n"));
}

#[cfg(not(target_arch = "wasm32"))]
const PINNED_MENUS: [(ApplicationMenu, &[&str]); 10] = [
    (
        ApplicationMenu::File,
        &[
            "New project… | new-project | Ctrl+Shift+N | on",
            "New cell… | new-cell | Ctrl+N | on",
            "New cell view… | new-view |  | on",
            "New Verilog-A device… | new-veriloga-device |  | on",
            "---",
            "Open project… | open-project | Ctrl+O | on",
            "Open netlist as project… | open-netlist | Ctrl+Shift+L | on",
            "Open recent >",
            "  ---",
            "  All projects… | project-launcher | Ctrl+Shift+O | on",
            "---",
            "Close | close-document | Ctrl+W | off: no closable document is active",
            "Close others | close-other-documents |  | off: no other closable document is open",
            "Close all | close-all-documents |  | off: no closable document is open",
            "Close project… | close-project | Ctrl+Shift+W | on",
            "---",
            "Save | save-project | Ctrl+S | on",
            "Save as… | save-project-as | Ctrl+Alt+Shift+S | on",
            "Save all | save-all | Ctrl+Shift+S | on",
            "Revert to saved… | revert-document |  | off: active document has no changes to revert",
            "Revision history… | revision-history |  | on",
            "---",
            "Import >",
            "  SPICE deck… | import-netlist |  | on",
            "  Verilog-A… | import-veriloga |  | on",
            "  Result dataset… | import-dataset |  | on",
            "  Schematic into new cell view… | open-schematic |  | on",
            "Export >",
            "  Image or PDF… | export-active-view | Ctrl+Alt+Shift+E | off: command is unavailable in this context",
            "  SPICE netlist… | export-netlist-spice |  | on",
            "  Spectre netlist… | export-netlist-spectre |  | on",
            "  Dataset… | export-results |  | off: no result dataset is available",
            "---",
            "Publish to web… | publish-web |  | hidden",
            "Live session… | live-session |  | hidden",
            "---",
            "Print… | print-hardcopy | Ctrl+P | off: command is unavailable in this context",
            "---",
            "Exit | exit-rspice | Alt+F4 | on",
        ],
    ),
    (
        ApplicationMenu::Edit,
        &[
            "Undo | undo | Ctrl+Z | off: nothing to undo",
            "Redo | redo | Ctrl+Shift+Z | off: nothing to redo",
            "Repeat last command | repeat-last-command | . | off: no placement or edit has run yet",
            "---",
            "Cut | cut-selection | Ctrl+X | off: select an editable object",
            "Copy | copy-selection | Ctrl+C | off: select an editable object",
            "Paste | paste-selection | Ctrl+V | off: clipboard has no compatible content",
            "Duplicate | duplicate-selection | Ctrl+D | off: select an editable object",
            "Delete | delete-selection | Delete | off: select an editable object",
            "---",
            "Select all | select-all | Ctrl+A | on",
            "Deselect all | deselect-all |  | off: nothing is selected",
            "---",
            "Move | move-selection | M | off: select an editable object",
            "Stretch | stretch-selection | S | off: select an editable object",
            "Create array… | array-selection |  | off: select an editable object",
            "Replace instance… | replace-instance |  | off: select an editable object",
            "Bulk edit… | design-bulk-tools |  | on",
            "---",
            "Rename… | rename-selection | F2 | off: select one editable component, net label, or declared bus",
            "Properties… | object-properties | Q | off: select one inspectable object",
            "---",
            "Find… | find-design | Ctrl+F | on",
            "Find and replace… | find-replace | Ctrl+H | on",
            "---",
            "Preferences… | preferences | Ctrl+, | on",
        ],
    ),
    (
        ApplicationMenu::View,
        &[
            "Zoom in | zoom-in | + | on",
            "Zoom out | zoom-out | − | on",
            "Zoom 100% | zoom-one-to-one | Ctrl+0 | on",
            "Fit drawing sheet | canvas-fit-sheet | F | on",
            "Fit content | canvas-fit-content | Shift+F | on",
            "Zoom to selection | zoom-to-selection |  | off: nothing is selected",
            "Zoom box | zoom-box | Z | on",
            "---",
            "Grid >",
            "  Dots | grid-dots |  | on | selected",
            "  Lines | grid-lines |  | on | unselected",
            "  Off | grid-off |  | on | unselected",
            "  ---",
            "  Grid, snap and routing… | grid-snap-routing |  | on",
            "Display options… | visibility-options |  | on",
            "---",
            "Navigator | toggle-navigator | Ctrl+B | on | checked",
            "Inspector | toggle-inspector | Ctrl+I | on | checked",
            "Console | console | Ctrl+J | on | unchecked",
            "Split with results | toggle-split-view | Ctrl+\\ | off: no retained result dataset is available | unchecked",
            "Problems | open-problems |  | on",
            "---",
            "Focus mode | toggle-focus-mode | Ctrl+Shift+F | on | unchecked",
            "Full screen | full-screen | F11 | on | unchecked",
        ],
    ),
    (
        ApplicationMenu::Design,
        &[
            "Descend | descend-selected-instance | Shift+E | off: select one hierarchical or Verilog-A instance",
            "Descend with options… | descend-hierarchy | H | off: select one hierarchical instance",
            "Return to parent | ascend-hierarchy | Shift+H | off: already at top hierarchy",
            "Edit symbol in place | edit-symbol-in-place |  | off: select one instance",
            "Create hierarchy… | create-hierarchy |  | off: select an editable object",
            "Create symbol from schematic… | create-symbol-from-schematic |  | on",
            "---",
            "Place >",
            "  Instance… | place-instance | Shift+I | on",
            "  Wire | place-wire | W | on",
            "  Bus | place-bus | B | on",
            "  Bus tap | place-bus-tap | T | on",
            "  Net label | place-label | N | on",
            "  Off-sheet connector | place-off-sheet-connector |  | on",
            "  Pins… | place-pin | Shift+P | on",
            "  Junction | place-junction | J | on",
            "  Text or note… | place-text | Shift+T | on",
            "  Documentation shape… | place-shape |  | on",
            "  ---",
            "  Resistor | place-resistor | R | on",
            "  Capacitor | place-capacitor | C | on",
            "  Inductor | place-inductor | L | on",
            "  Diode | place-diode | D | on",
            "  Voltage source | place-voltage-source | V | on",
            "  Current source | place-current-source | I | on",
            "  Ground | place-ground | Shift+G | on",
            "Arrange >",
            "  Rotate clockwise | rotate-selection | Ctrl+R | off: select an editable object",
            "  Rotate counter-clockwise | rotate-selection-ccw | Ctrl+Shift+R | off: select an editable object",
            "  Mirror about vertical axis | mirror-selection-horizontal | Ctrl+M | off: select an editable object",
            "  Mirror about horizontal axis | mirror-selection-vertical | Ctrl+Shift+M | off: select an editable object",
            "  ---",
            "  Align left | align-selection-left |  | off: command is unavailable in this context",
            "  Align horizontal centers | align-selection-horizontal-centers |  | off: command is unavailable in this context",
            "  Align right | align-selection-right |  | off: command is unavailable in this context",
            "  Align top | align-selection-top |  | off: command is unavailable in this context",
            "  Align vertical centers | align-selection-vertical-centers |  | off: command is unavailable in this context",
            "  Align bottom | align-selection-bottom |  | off: command is unavailable in this context",
            "  ---",
            "  Distribute horizontally | distribute-selection-horizontal |  | off: command is unavailable in this context",
            "  Distribute vertically | distribute-selection-vertical |  | off: command is unavailable in this context",
            "  ---",
            "  Snap selection to grid | snap-selection-to-grid |  | off: select an editable object",
            "  Join collinear wires | join-collinear-wires |  | on",
            "Sheet >",
            "  Previous sheet | previous-sheet | PageUp | off: the active cell view has a single sheet",
            "  Next sheet | next-sheet | PageDown | off: the active cell view has a single sheet",
            "  New sheet | new-sheet |  | on",
            "  Delete… | delete-sheet |  | off: the active cell view has a single sheet",
            "  ---",
            "  Sheet properties | page-setup | Ctrl+Alt+Shift+P | on",
            "  Sheet formats… | sheet-format-manager |  | on",
            "  Sheet defaults… | sheet-defaults |  | on",
            "---",
            "Variants… | design-variants |  | on",
            "Annotate reference designators… | annotate-reference-designators |  | on",
            "Connectivity and buses… | design-connectivity-tools |  | on",
            "Review comments… | review-comments |  | on",
            "Object table… | engineering-table-manager | Ctrl+Shift+T | on",
            "---",
            "Check cellview | check-cellview | Ctrl+E | on",
            "Check design | check-design |  | on",
            "Check and save… | check-and-save | Ctrl+Shift+E | on",
            "Violations >",
            "  Next violation | next-violation | ] | off: the last check found no violations",
            "  Previous violation | previous-violation | [ | off: the last check found no violations",
            "  ---",
            "  Clear check results | clear-checks |  | off: no check has run",
            "Design rule setup… | design-rule-setup |  | on",
        ],
    ),
    (
        ApplicationMenu::Simulate,
        &[
            "Manage simulation plans… | manage-simulation-plans |  | on",
            "Add analysis… | add-analysis |  | on",
            "---",
            "Setup >",
            "  Analyses | simulation-analyses |  | on",
            "  Excitations | simulation-excitations |  | on",
            "  Design variables | simulation-variables |  | on",
            "  Outputs & expressions | simulation-outputs |  | on",
            "  Requirements & specifications | simulation-specifications |  | on",
            "  PVT, sweeps & variation | simulation-run-set |  | on",
            "  Models & sections | simulation-models |  | on",
            "  Simulator options | simulation-solver |  | on",
            "  Save, streaming & retention policy | simulation-save-policy |  | on",
            "  ---",
            "  Configuration sets… | configuration-sets |  | on",
            "  Stimulus library | stimulus | Alt+8 | on",
            "---",
            "Check and netlist | check |  | on",
            "Run | start-run | F5 | on",
            "Stop | stop-run | Shift+F5 | off: no simulation is running",
            "Open generated netlist | generated-netlist | Ctrl+L | on",
            "Run history… | jobs-manager |  | on",
            "---",
            "Probe | place-probe | P | on",
            "Annotate DC operating point | annotate-operating-point |  | on | checked",
            "Operating-point table | result-operating-point |  | off: Requires the active analysis to be a DC operating-point result carrying a node DC solution, a device operating-point report, or a retained operating-point payload",
        ],
    ),
    (
        ApplicationMenu::Results,
        &[
            "Results browser | dataset-browser |  | off: no retained result dataset is available",
            "Waveforms | waveforms |  | on",
            "View as >",
            "  DC sweep | result-dc-sweep |  | hidden",
            "  Bode | result-bode |  | hidden",
            "  Nyquist | result-nyquist |  | hidden",
            "  Smith chart | result-smith |  | hidden",
            "  Polar response | result-polar |  | hidden",
            "  Pole-zero | result-pole-zero |  | hidden",
            "  Transfer function | result-transfer-function |  | hidden",
            "  Network matrix | result-network-matrix |  | hidden",
            "  FFT | result-fft |  | hidden",
            "  Harmonic balance tones | result-harmonic-balance |  | hidden",
            "  Phase noise | result-phase-noise |  | hidden",
            "  Noise contribution | result-noise |  | hidden",
            "  Sensitivity contribution | result-sensitivity-contribution |  | hidden",
            "  Eye diagram | result-eye |  | hidden",
            "  Histogram | result-histogram |  | hidden",
            "  Correlation scatter | result-scatter |  | hidden",
            "  Box and violin | result-distribution |  | hidden",
            "  Event history | result-events |  | hidden",
            "  Safe operating area | result-safe-operating-area |  | hidden",
            "  Optimization | result-optimization |  | hidden",
            "  Specification results | result-specifications |  | hidden",
            "  Sample table | result-table |  | hidden",
            "  Dataset manifest | result-manifest |  | hidden",
            "---",
            "New result document… | create-result-document |  | off: no retained result dataset is available",
            "Add pane… | visualization-add-pane |  | off: open an active result document",
            "Compare datasets… | compare-datasets |  | off: open an active result document",
            "Add measurement… | measurement-library |  | off: open a result in Results",
            "Family slicing and pivot… | family-slicing |  | off: open a result in Results",
            "---",
            "Link A/B cursors | toggle-linked-cursors | Shift+C | off: open a result in Results | unchecked",
            "Plot properties… | plot-properties |  | off: open a result in Results",
            "Calculator… | calculator |  | on",
            "Expression diagnostics… | expression-diagnostics |  | off: no result dataset is available",
            "Report editor… | report-page-editor |  | on",
            "Clear result history | clear-results |  | off: no result dataset is available",
        ],
    ),
    (
        ApplicationMenu::Verify,
        &[
            "Monte Carlo yield | yield |  | on",
            "Process corners | corners |  | on",
            "Parameter tuning | tuning |  | on",
            "Optimization | optimization |  | on",
            "Safe operating area | device-safety |  | on",
            "Specification matrix… | specifications |  | on",
            "Regression | regression |  | on",
        ],
    ),
    (
        ApplicationMenu::Tools,
        &[
            "Library manager | models-catalog |  | on",
            "Model browser… | model-browser |  | on",
            "Edit model… | model-editor |  | off: select one model in the library manager",
            "Create editable project copy | model-create-project-copy |  | off: select one external or built-in model in the library manager",
            "Measurement correlation… | model-correlation |  | off: select one model in the library manager",
            "PDK and model paths… | pdk-settings |  | on",
            "Rescan model libraries | rescan-model-libraries |  | on",
            "---",
            "Netlist and script editor | netlist | Alt+7 | on",
            "Scripts | automation | Ctrl+` | on",
        ],
    ),
    (
        ApplicationMenu::Window,
        &[
            "New window… | new-application-window | Ctrl+Alt+Shift+N | on",
            "Move document to >",
            "  New window… | detach-document |  | on",
            "  Another window… | move-document-window |  | off: create another application window before moving this document",
            "Reattach document | reattach-document |  | off: the active document is already in the main window",
            "Merge all windows… | consolidate-windows |  | off: there are no detached application windows",
            "Recover off-screen windows… | monitor-recovery |  | off: there are no secondary application windows to recover",
            "---",
            "Workspace >",
            "  Project | project | Alt+1 | on",
            "  Design | design | Alt+2 | on",
            "  Simulate | simulate | Alt+3 | on",
            "  Results | results | Alt+4 | on",
            "  Verify | verify | Alt+5 | on",
            "  Models | models | Alt+6 | on",
            "  Netlist and script editor | netlist | Alt+7 | on",
            "  Stimulus library | stimulus | Alt+8 | on",
            "Previous document | previous-document | Ctrl+Shift+Tab | off: only one document is open",
            "Next document | next-document | Ctrl+Tab | off: only one document is open",
            "---",
            "Layout >",
            "  Saved layouts… | workspace-layouts |  | on",
            "  Reset layout | reset-workspace-layout |  | on",
            "Windows… | window-manager |  | on",
        ],
    ),
    (
        ApplicationMenu::Help,
        &[
            "RSpice Help | help-center | F1 | on",
            "Keyboard shortcuts | command-reference |  | on",
            "Examples… | examples |  | on",
            "Release notes | release-notes |  | on",
            "---",
            "Report a problem… | system-diagnostics |  | on",
            "License and activation… | license-activation |  | on",
            "---",
            "Third-party notices and privacy | legal-privacy-center |  | on",
            "About RSpice | about |  | on",
        ],
    ),
];

#[cfg(not(target_arch = "wasm32"))]
const PINNED_SOURCE_EDITOR_MENUS: [(ApplicationMenu, &[&str]); 2] = [
    (
        ApplicationMenu::Edit,
        &[
            "Undo | undo | Ctrl+Z | on",
            "Redo | redo | Ctrl+Shift+Z | on",
            "---",
            "Cut | cut-selection | Ctrl+X | on",
            "Copy | copy-selection | Ctrl+C | on",
            "Paste | paste-selection | Ctrl+V | on",
            "Duplicate line | duplicate-line | Ctrl+D | on",
            "Delete line | delete-line | Ctrl+Shift+K | on",
            "---",
            "Select all | select-all | Ctrl+A | on",
            "---",
            "Toggle line comment | toggle-line-comment | Ctrl+/ | on",
            "Format document | format-document |  | on",
            "Go to line… | go-to-line | Ctrl+G | on",
            "---",
            "Find in source… | find-code | Ctrl+F | off: command is unavailable in this context",
            "---",
            "Preferences… | preferences | Ctrl+, | on",
        ],
    ),
    (
        ApplicationMenu::View,
        &[
            "Zoom in | zoom-in | + | on",
            "Zoom out | zoom-out | − | on",
            "Zoom 100% | zoom-one-to-one | Ctrl+0 | on",
            "Fit drawing sheet | canvas-fit-sheet | F | on",
            "Fit content | canvas-fit-content | Shift+F | on",
            "Zoom to selection | zoom-to-selection |  | off: nothing is selected",
            "Zoom box | zoom-box | Z | on",
            "---",
            "Grid >",
            "  Dots | grid-dots |  | on | selected",
            "  Lines | grid-lines |  | on | unselected",
            "  Off | grid-off |  | on | unselected",
            "  ---",
            "  Grid, snap and routing… | grid-snap-routing |  | on",
            "Display options… | visibility-options |  | on",
            "---",
            "Navigator | toggle-navigator | Ctrl+B | on | checked",
            "Inspector | toggle-inspector | Ctrl+I | on | checked",
            "Console | console | Ctrl+J | on | unchecked",
            "Split with results | toggle-split-view | Ctrl+\\ | off: no retained result dataset is available | unchecked",
            "Problems | open-problems |  | on",
            "---",
            "Folding >",
            "  Fold selection | fold-selection | Ctrl+Shift+[ | on",
            "  Unfold at caret | unfold-at-caret | Ctrl+Shift+] | on",
            "  Fold all | fold-all |  | on",
            "  Unfold all | unfold-all |  | on",
            "Focus mode | toggle-focus-mode | Ctrl+Shift+F | on | unchecked",
            "Full screen | full-screen | F11 | on | unchecked",
        ],
    ),
];

#[cfg(not(target_arch = "wasm32"))]
const PINNED_NO_PROJECT_MENUS: [(ApplicationMenu, &[&str]); 1] = [(
    ApplicationMenu::File,
    &[
        "New project… | new-project | Ctrl+Shift+N | on",
        "New cell… | new-cell | Ctrl+N | off: no project is open",
        "New cell view… | new-view |  | off: no project is open",
        "New Verilog-A device… | new-veriloga-device |  | off: no project is open",
        "---",
        "Open schematic… | open-schematic |  | on",
        "Open project… | open-project | Ctrl+O | on",
        "Open netlist as project… | open-netlist | Ctrl+Shift+L | on",
        "Open recent >",
        "  ---",
        "  All projects… | project-launcher | Ctrl+Shift+O | on",
        "---",
        "Close | close-document | Ctrl+W | off: no project is open",
        "Close others | close-other-documents |  | off: no project is open",
        "Close all | close-all-documents |  | off: no project is open",
        "Close project… | close-project | Ctrl+Shift+W | off: no project is open",
        "---",
        "Save | save-project | Ctrl+S | off: no project is open",
        "Save as… | save-project-as | Ctrl+Alt+Shift+S | off: no project is open",
        "Save all | save-all | Ctrl+Shift+S | off: no project is open",
        "Revert to saved… | revert-document |  | off: no project is open",
        "Revision history… | revision-history |  | off: no project is open",
        "---",
        "Import >",
        "  SPICE deck… | import-netlist |  | off: no project is open",
        "  Verilog-A… | import-veriloga |  | off: no project is open",
        "  Result dataset… | import-dataset |  | off: no project is open",
        "Export >",
        "  Image or PDF… | export-active-view | Ctrl+Alt+Shift+E | off: no project is open",
        "  SPICE netlist… | export-netlist-spice |  | off: no project is open",
        "  Spectre netlist… | export-netlist-spectre |  | off: no project is open",
        "  Dataset… | export-results |  | off: no project is open",
        "---",
        "Publish to web… | publish-web |  | hidden",
        "Live session… | live-session |  | hidden",
        "---",
        "Print… | print-hardcopy | Ctrl+P | off: no project is open",
        "---",
        "Exit | exit-rspice | Alt+F4 | on",
    ],
)];

#[cfg(not(target_arch = "wasm32"))]
const PINNED_SYMBOL_MENUS: [(ApplicationMenu, &[&str]); 1] = [(
    ApplicationMenu::Design,
    &[
        "Descend | descend-selected-instance | Shift+E | off: open a schematic or testbench",
        "Descend with options… | descend-hierarchy | H | off: open a schematic or testbench",
        "Return to parent | ascend-hierarchy | Shift+H | off: open a schematic or testbench",
        "Return to parent schematic | exit-symbol-in-place | Enter | off: no symbol is being edited in place",
        "Create hierarchy… | create-hierarchy |  | off: open an editable schematic or testbench",
        "Create symbol from schematic… | create-symbol-from-schematic |  | off: open a schematic or testbench",
        "---",
        "Place >",
        "  Pin | symbol-pin-tool | P | on",
        "  Line | symbol-line-tool | L | on",
        "  Rectangle | symbol-rectangle-tool | R | on",
        "  Circle | symbol-circle-tool | C | on",
        "  Arc | symbol-arc-tool | A | on",
        "  Polygon | symbol-polygon-tool | G | on",
        "  Text | symbol-text-tool | T | on",
        "  ---",
        "  Rotate selected pin to next side | symbol-rotate-pin | Ctrl+R | off: command is unavailable in this context",
        "  Mirror selected pin across body | symbol-mirror-pin | Ctrl+M | off: command is unavailable in this context",
        "  Update pins from contract | update-pins-from-contract |  | off: command is unavailable in this context",
        "Arrange >",
        "  Rotate clockwise | rotate-selection | Ctrl+R | off: open an editable schematic or testbench",
        "  Rotate counter-clockwise | rotate-selection-ccw | Ctrl+Shift+R | off: open an editable schematic or testbench",
        "  Mirror about vertical axis | mirror-selection-horizontal | Ctrl+M | off: open an editable schematic or testbench",
        "  Mirror about horizontal axis | mirror-selection-vertical | Ctrl+Shift+M | off: open an editable schematic or testbench",
        "  ---",
        "  Align left | align-selection-left |  | off: command is unavailable in this context",
        "  Align horizontal centers | align-selection-horizontal-centers |  | off: command is unavailable in this context",
        "  Align right | align-selection-right |  | off: command is unavailable in this context",
        "  Align top | align-selection-top |  | off: command is unavailable in this context",
        "  Align vertical centers | align-selection-vertical-centers |  | off: command is unavailable in this context",
        "  Align bottom | align-selection-bottom |  | off: command is unavailable in this context",
        "  ---",
        "  Distribute horizontally | distribute-selection-horizontal |  | off: command is unavailable in this context",
        "  Distribute vertically | distribute-selection-vertical |  | off: command is unavailable in this context",
        "  ---",
        "  Snap selection to grid | snap-selection-to-grid |  | off: open an editable schematic or testbench",
        "  Join collinear wires | join-collinear-wires |  | off: open an editable schematic or testbench",
        "Sheet >",
        "  Previous sheet | previous-sheet | PageUp | off: open a schematic or testbench",
        "  Next sheet | next-sheet | PageDown | off: open a schematic or testbench",
        "  New sheet | new-sheet |  | off: open an editable schematic or testbench",
        "  Delete… | delete-sheet |  | off: open an editable schematic or testbench",
        "  ---",
        "  Sheet properties | page-setup | Ctrl+Alt+Shift+P | off: open an editable schematic or testbench drawing sheet",
        "  Sheet formats… | sheet-format-manager |  | off: open an editable schematic or testbench drawing sheet",
        "  Sheet defaults… | sheet-defaults |  | off: command is unavailable in this context",
        "---",
        "Variants… | design-variants |  | off: open an editable schematic",
        "Annotate reference designators… | annotate-reference-designators |  | off: open an editable schematic",
        "Connectivity and buses… | design-connectivity-tools |  | off: open a schematic or testbench",
        "Review comments… | review-comments |  | off: open a schematic project",
        "Object table… | engineering-table-manager | Ctrl+Shift+T | off: command is unavailable in this context",
        "---",
        "Check cellview | check-cellview | Ctrl+E | on",
        "Check design | check-design |  | on",
        "Validate and save symbol | save-symbol |  | on",
        "Violations >",
        "  Next violation | next-violation | ] | off: the last check found no violations",
        "  Previous violation | previous-violation | [ | off: the last check found no violations",
        "  ---",
        "  Clear check results | clear-checks |  | off: no check has run",
        "Design rule setup… | design-rule-setup |  | on",
    ],
)];

#[cfg(not(target_arch = "wasm32"))]
const PINNED_COMPACT_MENU: [&str; 10] = [
    "File >",
    "Edit >",
    "View >",
    "Design >",
    "Simulate >",
    "Results >",
    "Verify >",
    "Tools >",
    "Window >",
    "Help >",
];

//! The Configuration page's inspector: the selected row's binding in full,
//! or, with no row selected, the configuration itself. On Default it states
//! the built-in facts and edits nothing.

use egui::{Key, TextEdit, Ui};
use rspice_ui_kit::panels::section_header;

use crate::state::{
    ConfigurationPlatform, ConfigurationSetDefinition, DEFAULT_STOP_VIEW_TYPES,
    default_view_search_order,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{ordered_chips, select};
use crate::workbench::app_state::Model;
use crate::workbench::design_system::{property_row, property_row_wrapped};
use crate::workbench::project::configuration_state::{
    BindingChange, ConfigurationPageState, ConfigurationResolution, ConfigurationRow,
    ConfigurationSelection, FieldEdit, apply_binding_change, apply_cell_binding_change,
    cell_binding, cell_view_names, governing_binding, instance_binding, runs_on_label,
    used_for_runs,
};

const RUNS_ON: [&str; 3] = ["All", "Desktop", "Browser"];
const NO_STOP: &str = "None";

/// What the inspector asked for this frame.
enum InspectorAction {
    Rename(String),
    Binding {
        path: String,
        inherited: Vec<String>,
        change: BindingChange,
    },
    CellBinding {
        library: String,
        cell: String,
        inherited: Vec<String>,
        change: BindingChange,
    },
    UseForRuns(ConfigurationSelection),
    Deselect,
}

/// The Configuration page's inspector over the page and the model. The
/// edits it asks for land on the page as it draws; using a configuration for
/// runs commits a project transaction, so the target is handed back for the
/// shell to carry out.
pub(super) fn project_configuration(
    ui: &mut Ui,
    page: &mut ConfigurationPageState,
    model: &Model,
) -> Option<ConfigurationSelection> {
    let catalog = &model.workspace.content.configuration_sets;
    let shown = &*page;
    let selection = shown.selection(catalog);
    let resolution = shown.last_resolution();
    let mut buffer = shown.field_edit.clone();
    let definition = match selection {
        ConfigurationSelection::Configuration(id) => shown.definition(catalog, id),
        ConfigurationSelection::Default => None,
    };
    let editable = definition.is_some() && !model.safe_mode.project_read_only();
    let action = match shown.row.clone() {
        Some(ConfigurationRow::Instance(path)) => instance(
            ui,
            model,
            definition,
            resolution.as_deref(),
            &path,
            editable,
            &mut buffer,
        ),
        Some(ConfigurationRow::Cell { library, cell }) => cell_facts(
            ui,
            model,
            definition,
            resolution.as_deref(),
            (&library, &cell),
            editable,
        ),
        Some(ConfigurationRow::Pattern(pattern)) => definition.and_then(|definition| {
            pattern_binding(
                ui,
                definition,
                resolution.as_deref(),
                &pattern,
                editable,
                &mut buffer,
            )
        }),
        None => match definition {
            Some(definition) => {
                configuration(ui, model, selection, definition, editable, &mut buffer)
            }
            None => default_facts(ui, model),
        },
    };
    page.field_edit = buffer;
    match action? {
        InspectorAction::UseForRuns(target) => return Some(target),
        InspectorAction::Deselect => page.row = None,
        InspectorAction::CellBinding {
            library,
            cell,
            inherited,
            change,
        } => edit(page, model, selection, |definition| {
            apply_cell_binding_change(definition, &library, &cell, &inherited, change);
        }),
        InspectorAction::Rename(name) => edit(page, model, selection, |definition| {
            definition.name = name;
        }),
        InspectorAction::Binding {
            path,
            inherited,
            change,
        } => {
            let renamed = match &change {
                BindingChange::Path(pattern) => Some(pattern.trim().to_owned()),
                _ => None,
            };
            let removed = change == BindingChange::Remove;
            edit(page, model, selection, |definition| {
                apply_binding_change(definition, &path, &inherited, change);
            });
            if let Some(pattern) = renamed {
                page.row = Some(ConfigurationRow::Pattern(pattern));
            } else if removed && matches!(page.row, Some(ConfigurationRow::Pattern(_))) {
                page.row = None;
            }
        }
    }
    None
}

fn edit(
    page: &mut ConfigurationPageState,
    model: &Model,
    selection: ConfigurationSelection,
    change: impl FnOnce(&mut ConfigurationSetDefinition),
) {
    if let ConfigurationSelection::Configuration(id) = selection {
        page.edit(&model.workspace.content.configuration_sets, id, change);
    }
}

/// The built-in Default's facts.
fn default_facts(ui: &mut Ui, model: &Model) -> Option<InspectorAction> {
    let project = &model.workspace.content.project;
    let catalog = &model.workspace.content.configuration_sets;
    property_row(ui, "Name", "Default");
    property_row(
        ui,
        "Top cell",
        &format!("{}/{}/schematic", project.root_library, project.top_cell),
    );
    property_row(ui, "DUT instance", "\u{2014}");
    let order = default_view_search_order("schematic", false);
    let mut words = vec!["the instance's own view".to_owned()];
    words.extend(order.into_iter().skip(1));
    property_row_wrapped(ui, "View list", &words.join(", then "));
    property_row_wrapped(
        ui,
        "Stop list",
        &DEFAULT_STOP_VIEW_TYPES
            .iter()
            .map(|view_type| view_type.display_name())
            .collect::<Vec<_>>()
            .join(", "),
    );
    property_row(ui, "Unresolved cells", "Stop the netlist");
    used_for_runs_row(ui, model, ConfigurationSelection::Default, catalog)
}

/// "Used for runs" as a fact, or the action that makes it one.
fn used_for_runs_row(
    ui: &mut Ui,
    model: &Model,
    selection: ConfigurationSelection,
    catalog: &crate::state::ConfigurationSetCatalog,
) -> Option<InspectorAction> {
    if used_for_runs(catalog, selection) {
        property_row(ui, "Used for runs", "Yes");
        return None;
    }
    property_row(ui, "Used for runs", "No");
    action_button(ui, "Use for runs", !model.safe_mode.project_read_only())
        .then_some(InspectorAction::UseForRuns(selection))
}

fn action_button(ui: &mut Ui, label: &str, enabled: bool) -> bool {
    let width = ui.available_width().max(1.0);
    let clicked = ui
        .add_enabled_ui(enabled, |ui| {
            ui.add_sized([width, 28.0], egui::Button::new(label))
        })
        .inner
        .clicked();
    ui.add_space(4.0);
    clicked
}

/// The configuration itself, with no row selected.
fn configuration(
    ui: &mut Ui,
    model: &Model,
    selection: ConfigurationSelection,
    definition: &ConfigurationSetDefinition,
    editable: bool,
    buffer: &mut Option<FieldEdit>,
) -> Option<InspectorAction> {
    let catalog = &model.workspace.content.configuration_sets;
    let mut action = committed_text(
        ui,
        buffer,
        "configuration-name",
        "Name",
        &definition.name,
        editable,
    )
    .filter(|name| !name.trim().is_empty())
    .map(|name| InspectorAction::Rename(name.trim().to_owned()));
    property_row(ui, "Top cell", &definition.root.display_path());
    property_row(ui, "DUT instance", &definition.dut_path);
    property_row(
        ui,
        "Instance bindings",
        &definition.overrides.len().to_string(),
    );
    property_row(
        ui,
        "Cell bindings",
        &definition.cell_bindings.len().to_string(),
    );
    if let Some(use_for_runs) = used_for_runs_row(ui, model, selection, catalog) {
        action = Some(use_for_runs);
    }
    action
}

/// One instance: what it resolved to, and its own binding.
fn instance(
    ui: &mut Ui,
    model: &Model,
    definition: Option<&ConfigurationSetDefinition>,
    resolution: Option<&ConfigurationResolution>,
    path: &str,
    editable: bool,
    buffer: &mut Option<FieldEdit>,
) -> Option<InspectorAction> {
    let row = resolution.and_then(|resolution| {
        let row = resolution.instance(path)?;
        let hierarchy = resolution.hierarchy.as_ref().ok()?;
        Some(&hierarchy.bindings[row.binding])
    });
    section_header(ui, "Instance", None);
    property_row(ui, "Instance", path);
    let Some(resolved) = row else {
        property_row_wrapped(
            ui,
            "Status",
            "This instance is not in the design any more. Pick another row.",
        );
        return action_button(ui, "Show the configuration", true)
            .then_some(InspectorAction::Deselect);
    };
    let reference = &resolved.reference;
    property_row(
        ui,
        "Cell",
        &format!("{}/{}", reference.library, reference.cell),
    );
    property_row(
        ui,
        "View found",
        if resolved.status.is_resolved() {
            &reference.view
        } else {
            "\u{2014}"
        },
    );
    property_row_wrapped(ui, "Searched", &resolved.view_search_order.join(", then "));
    if let Some(diagnostic) = &resolved.diagnostic {
        property_row_wrapped(ui, "Problem", diagnostic);
    }
    for warning in &resolved.warnings {
        property_row_wrapped(ui, "Note", warning);
    }
    let definition = definition?;
    let exact = instance_binding(definition, path);
    let inherited = governing_binding(definition, path).map_or_else(
        || definition.executable_view_policy.clone(),
        |binding| binding.executable_views.clone(),
    );
    section_header(
        ui,
        "Binding",
        Some(if exact.is_some() {
            "this instance"
        } else {
            "inherited"
        }),
    );
    let views = exact.map_or(&inherited, |binding| &binding.executable_views);
    let stop = exact.and_then(|binding| binding.stop_view.clone());
    let section = exact
        .and_then(|binding| binding.model_section.clone())
        .unwrap_or_default();
    let platforms = exact.map_or(ConfigurationPlatform::ALL.to_vec(), |binding| {
        binding.eligible_platforms.clone()
    });
    let views_owned = views.clone();
    let addable = cell_view_names(&model.library_manager, &reference.library, &reference.cell);
    let change = binding_fields(
        ui,
        &format!("instance-{path}"),
        BindingFields {
            views: &views_owned,
            stop: stop.as_deref(),
            section: Some(&section),
            platforms: Some(&platforms),
            addable: &addable,
        },
        editable,
        buffer,
    )
    .or_else(|| {
        (editable && exact.is_some() && action_button(ui, "Inherit the view list", true))
            .then_some(BindingChange::Remove)
    });
    change.map(|change| InspectorAction::Binding {
        path: path.to_owned(),
        inherited,
        change,
    })
}

/// One library/cell: what its instances resolved to, and the cell binding
/// they all follow.
fn cell_facts(
    ui: &mut Ui,
    model: &Model,
    definition: Option<&ConfigurationSetDefinition>,
    resolution: Option<&ConfigurationResolution>,
    (library, cell): (&str, &str),
    editable: bool,
) -> Option<InspectorAction> {
    let row = resolution.and_then(|resolution| {
        resolution.cells.iter().find(|row| {
            row.library.eq_ignore_ascii_case(library) && row.cell.eq_ignore_ascii_case(cell)
        })
    });
    section_header(ui, "Cell", None);
    property_row(ui, "Cell", &format!("{library}/{cell}"));
    match row {
        Some(row) => {
            property_row(ui, "Instances", &row.instances.to_string());
            property_row(
                ui,
                "View found",
                &if row.views_found.is_empty() {
                    "\u{2014}".to_owned()
                } else {
                    row.views_found.join(", ")
                },
            );
        }
        None => {
            property_row_wrapped(
                ui,
                "Instances",
                "None: this design does not instantiate the cell, so its binding binds nothing.",
            );
        }
    }
    let definition = definition?;
    let binding = cell_binding(definition, library, cell);
    let inherited = definition.executable_view_policy.clone();
    section_header(
        ui,
        "Binding",
        Some(if binding.is_some() {
            "this cell"
        } else {
            "inherited"
        }),
    );
    let views = binding.map_or_else(
        || inherited.clone(),
        |binding| binding.executable_views.clone(),
    );
    let stop = binding.and_then(|binding| binding.stop_view.clone());
    let addable = cell_view_names(&model.library_manager, library, cell);
    let mut unused = None;
    let change = binding_fields(
        ui,
        &format!("cell-{library}-{cell}"),
        BindingFields {
            views: &views,
            stop: stop.as_deref(),
            section: None,
            platforms: None,
            addable: &addable,
        },
        editable,
        &mut unused,
    )
    .or_else(|| {
        (editable && binding.is_some() && action_button(ui, "Inherit the view list", true))
            .then_some(BindingChange::Remove)
    });
    change.map(|change| InspectorAction::CellBinding {
        library: library.to_owned(),
        cell: cell.to_owned(),
        inherited,
        change,
    })
}

/// One pattern binding.
fn pattern_binding(
    ui: &mut Ui,
    definition: &ConfigurationSetDefinition,
    resolution: Option<&ConfigurationResolution>,
    pattern: &str,
    editable: bool,
    buffer: &mut Option<FieldEdit>,
) -> Option<InspectorAction> {
    let binding = definition
        .overrides
        .iter()
        .find(|binding| binding.instance_path == pattern)?;
    section_header(ui, "Pattern binding", None);
    let mut change = committed_text(
        ui,
        buffer,
        &format!("pattern-{pattern}"),
        "Pattern",
        &binding.instance_path,
        editable,
    )
    .filter(|text| !text.trim().is_empty())
    .map(BindingChange::Path);
    let fields = binding_fields(
        ui,
        &format!("pattern-binding-{pattern}"),
        BindingFields {
            views: &binding.executable_views,
            stop: binding.stop_view.as_deref(),
            section: Some(binding.model_section.as_deref().unwrap_or_default()),
            platforms: Some(&binding.eligible_platforms),
            addable: resolution.map_or(&[][..], |resolution| &resolution.view_names),
        },
        editable,
        buffer,
    );
    change = change.or(fields);
    if editable && action_button(ui, "Remove pattern binding", true) {
        change = Some(BindingChange::Remove);
    }
    change.map(|change| InspectorAction::Binding {
        path: pattern.to_owned(),
        inherited: Vec::new(),
        change,
    })
}

/// A binding's editable properties, shared by instances and patterns.
struct BindingFields<'a> {
    views: &'a [String],
    stop: Option<&'a str>,
    /// Instance and pattern bindings only: a cell binding names no model
    /// section and runs wherever its instances do.
    section: Option<&'a str>,
    platforms: Option<&'a [ConfigurationPlatform]>,
    /// The views the view list can add: the bound cell's, or for a pattern,
    /// which binds no single cell, every view the project's libraries hold.
    addable: &'a [String],
}

/// The view list, stop view, model section and platforms of one binding.
fn binding_fields(
    ui: &mut Ui,
    key: &str,
    fields: BindingFields<'_>,
    editable: bool,
    buffer: &mut Option<FieldEdit>,
) -> Option<BindingChange> {
    let mut change = None;
    let t = Tokens::get(ui.ctx());
    field_label(ui, "View list", &t);
    let addable = fields
        .addable
        .iter()
        .filter(|name| {
            !fields
                .views
                .iter()
                .any(|view| view.eq_ignore_ascii_case(name))
        })
        .cloned()
        .collect::<Vec<_>>();
    ui.horizontal_wrapped(|ui| {
        if let Some(edit) = ordered_chips(
            ui,
            &format!("{key}-views"),
            "View list",
            "View",
            fields.views,
            &addable,
            editable,
        ) {
            change = Some(BindingChange::Views(edit));
        }
    });
    ui.add_space(4.0);

    let mut stops = vec![NO_STOP.to_owned()];
    stops.extend(fields.views.iter().cloned());
    let current_stop = fields.stop.unwrap_or(NO_STOP);
    if let Some(index) = choice_row(
        ui,
        &format!("{key}-stop"),
        "Stop view",
        current_stop,
        &stops,
        editable,
    ) && stops[index] != current_stop
    {
        change = Some(BindingChange::Stop(
            (index > 0).then(|| stops[index].clone()),
        ));
    }

    if let Some(current) = fields.section
        && let Some(section) = committed_text(
            ui,
            buffer,
            &format!("{key}-section"),
            "Model section",
            current,
            editable,
        )
    {
        change = Some(BindingChange::ModelSection(
            (!section.trim().is_empty()).then_some(section),
        ));
    }

    let Some(platforms) = fields.platforms else {
        return change;
    };
    let runs_on = runs_on_label(platforms);
    let options = RUNS_ON.map(str::to_owned);
    if let Some(index) = choice_row(
        ui,
        &format!("{key}-runs-on"),
        "Runs on",
        runs_on,
        &options,
        editable,
    ) && RUNS_ON[index] != runs_on
    {
        change = Some(BindingChange::RunsOn(match RUNS_ON[index] {
            "Desktop" => vec![ConfigurationPlatform::Desktop],
            "Browser" => vec![ConfigurationPlatform::Browser],
            _ => ConfigurationPlatform::ALL.to_vec(),
        }));
    }
    change
}

fn field_label(ui: &mut Ui, label: &str, t: &Tokens) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
}

/// A labelled dropdown, or the value alone when nothing may change.
fn choice_row(
    ui: &mut Ui,
    id: &str,
    label: &str,
    current: &str,
    options: &[String],
    editable: bool,
) -> Option<usize> {
    if !editable {
        property_row(ui, label, current);
        return None;
    }
    let t = Tokens::get(ui.ctx());
    field_label(ui, label, &t);
    let mut picked = None;
    ui.horizontal(|ui| {
        let width = ui.available_width().max(80.0);
        picked = select(ui, id, label, current, options, width);
    });
    ui.add_space(4.0);
    picked
}

/// A text field that commits on Enter or when it loses focus, and puts its
/// value back on Escape. What is typed in between stays in `buffer`, so no
/// keystroke edits the configuration or re-resolves the design.
fn committed_text(
    ui: &mut Ui,
    buffer: &mut Option<FieldEdit>,
    field: &str,
    label: &str,
    current: &str,
    editable: bool,
) -> Option<String> {
    if !editable {
        property_row(
            ui,
            label,
            if current.is_empty() {
                "\u{2014}"
            } else {
                current
            },
        );
        return None;
    }
    let t = Tokens::get(ui.ctx());
    field_label(ui, label, &t);
    let mut text = match buffer.as_ref() {
        Some(edit) if edit.field == field => edit.text.clone(),
        _ => current.to_owned(),
    };
    let mut committed = None;
    ui.horizontal(|ui| {
        let width = ui.available_width().max(80.0);
        let response = ui.add_sized(
            [width, 24.0],
            TextEdit::singleline(&mut text)
                .id(egui::Id::new(("configuration-field", field)))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
        );
        if response.changed() {
            *buffer = Some(FieldEdit {
                field: field.to_owned(),
                text: text.clone(),
            });
        }
        if response.lost_focus() {
            let escaped = ui.input(|input| input.key_pressed(Key::Escape));
            let typed = buffer
                .take_if(|edit| edit.field == field)
                .map(|edit| edit.text);
            if !escaped && let Some(typed) = typed {
                committed = (typed != current).then_some(typed);
            }
        }
    });
    ui.add_space(4.0);
    committed
}

#[cfg(test)]
mod tests;

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
use crate::ui::widgets::{Problem, ordered_chips, problem_line, select};
use crate::workbench::app_state::Model;
use crate::workbench::design_system::{property_row, property_row_wrapped};
use crate::workbench::project::configuration_state::{
    BindingChange, ConfigurationPageState, ConfigurationResolution, ConfigurationRow,
    ConfigurationSelection, FieldEdit, ONLY_ON, apply_binding_change, apply_cell_binding_change,
    cell_binding, cell_view_names, inherited_views, instance_binding, only_on_label, used_for_runs,
};

const NO_STOP: &str = "None";

/// The text fields' state for one frame: what is being typed, the text a
/// field committed this frame, by field, so a refused commit can be put back
/// into the field that made it, and the model sections the attached
/// technology offers the Model section field.
struct Fields {
    buffer: Option<FieldEdit>,
    committed: Option<(String, String)>,
    sections: Vec<String>,
}

/// What the inspector asked for this frame.
enum InspectorAction {
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
pub fn project_configuration(
    ui: &mut Ui,
    page: &mut ConfigurationPageState,
    model: &Model,
) -> Option<ConfigurationSelection> {
    let catalog = &model.workspace.content.configuration_sets;
    let shown = &*page;
    let selection = shown.selection(catalog);
    let resolution = shown.last_resolution();
    let mut fields = Fields {
        buffer: shown.field_edit.clone(),
        committed: None,
        sections: model
            .workspace
            .content
            .project
            .technology_binding()
            .map_or_else(Vec::new, |technology| {
                technology.process_sections().to_vec()
            }),
    };
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
            &mut fields,
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
                &mut fields,
            )
        }),
        None => {
            // While an Apply or a Use for runs is checked, neither is
            // offered again.
            let runs_allowed = !model.safe_mode.project_read_only() && !shown.is_checking();
            match definition {
                Some(definition) => configuration(ui, model, selection, definition, runs_allowed),
                None => default_facts(ui, model, runs_allowed),
            }
        }
    };
    page.field_edit = fields.buffer;
    let taken = match action? {
        InspectorAction::UseForRuns(target) => return Some(target),
        InspectorAction::Deselect => {
            page.row = None;
            true
        }
        InspectorAction::CellBinding {
            library,
            cell,
            inherited,
            change,
        } => edit(page, model, selection, |definition| {
            apply_cell_binding_change(definition, &library, &cell, &inherited, change);
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
            let taken = edit(page, model, selection, |definition| {
                apply_binding_change(definition, &path, &inherited, change);
            });
            if taken {
                if let Some(pattern) = renamed {
                    page.row = Some(ConfigurationRow::Pattern(pattern));
                } else if removed && matches!(page.row, Some(ConfigurationRow::Pattern(_))) {
                    page.row = None;
                }
            }
            taken
        }
    };
    // A typed value the configuration refused stays in its field, with the
    // reason under it, rather than vanishing into a refusal elsewhere.
    if !taken && let Some((field, text)) = fields.committed {
        page.field_edit = Some(FieldEdit {
            field,
            text,
            refusal: page.refusal.take(),
        });
    }
    None
}

fn edit(
    page: &mut ConfigurationPageState,
    model: &Model,
    selection: ConfigurationSelection,
    change: impl FnOnce(&mut ConfigurationSetDefinition),
) -> bool {
    match selection {
        ConfigurationSelection::Configuration(id) => {
            page.edit(&model.workspace.content.configuration_sets, id, change)
        }
        ConfigurationSelection::Default => false,
    }
}

/// The built-in Default's facts.
fn default_facts(ui: &mut Ui, model: &Model, runs_allowed: bool) -> Option<InspectorAction> {
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
    used_for_runs_row(ui, ConfigurationSelection::Default, catalog, runs_allowed)
}

/// "Used for runs" as a fact, or the action that makes it one.
fn used_for_runs_row(
    ui: &mut Ui,
    selection: ConfigurationSelection,
    catalog: &crate::state::ConfigurationSetCatalog,
    runs_allowed: bool,
) -> Option<InspectorAction> {
    if used_for_runs(catalog, selection) {
        property_row(ui, "Used for runs", "Yes");
        return None;
    }
    property_row(ui, "Used for runs", "No");
    action_button(ui, "Use for runs", runs_allowed)
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

/// The configuration itself, with no row selected. Its name is a fact here:
/// More > Rename… is the one way to rename it.
fn configuration(
    ui: &mut Ui,
    model: &Model,
    selection: ConfigurationSelection,
    definition: &ConfigurationSetDefinition,
    runs_allowed: bool,
) -> Option<InspectorAction> {
    let catalog = &model.workspace.content.configuration_sets;
    property_row(ui, "Name", &definition.name);
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
    used_for_runs_row(ui, selection, catalog, runs_allowed)
}

/// One instance: what it resolved to, and its own binding. While the shown
/// edits do not resolve, the binding is still edited from the definition, so
/// an edit that does not resolve can always be edited back.
fn instance(
    ui: &mut Ui,
    model: &Model,
    definition: Option<&ConfigurationSetDefinition>,
    resolution: Option<&ConfigurationResolution>,
    path: &str,
    editable: bool,
    fields: &mut Fields,
) -> Option<InspectorAction> {
    let resolved = resolution.and_then(|resolution| {
        let row = resolution.instance(path)?;
        let hierarchy = resolution.hierarchy.as_ref().ok()?;
        Some(&hierarchy.bindings[row.binding])
    });
    let unresolved = resolution.is_some_and(|resolution| resolution.hierarchy.is_err());
    section_header(ui, "Instance", None);
    property_row(ui, "Instance", path);
    let (library, cell) = match resolved {
        Some(resolved) => {
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
            (reference.library.as_str(), reference.cell.as_str())
        }
        None if unresolved => {
            property_row_wrapped(
                ui,
                "Status",
                "The edits do not resolve, so what this instance binds to is not known.",
            );
            ("", "")
        }
        None => {
            property_row_wrapped(
                ui,
                "Status",
                "This instance is not in the design any more. Pick another row.",
            );
            return action_button(ui, "Show the configuration", true)
                .then_some(InspectorAction::Deselect);
        }
    };
    let definition = definition?;
    let exact = instance_binding(definition, path);
    let own_views = exact.filter(|binding| !binding.executable_views.is_empty());
    let (inherited, inherited_stop) = inherited_views(definition, path, (library, cell));
    section_header(
        ui,
        "Binding",
        Some(if own_views.is_some() {
            "this instance"
        } else {
            "inherited"
        }),
    );
    // A binding that names views stops where it says, else where its cell's
    // binding does; one that names none follows the binding it inherits from.
    let (views, stop) = match own_views {
        Some(binding) => (
            binding.executable_views.clone(),
            binding.stop_view.clone().or_else(|| {
                cell_binding(definition, library, cell).and_then(|cell| cell.stop_view.clone())
            }),
        ),
        None => (inherited.clone(), inherited_stop),
    };
    let section = exact
        .and_then(|binding| binding.model_section.clone())
        .unwrap_or_default();
    let platforms = exact.map_or(ConfigurationPlatform::ALL.to_vec(), |binding| {
        binding.eligible_platforms.clone()
    });
    let addable = if resolved.is_some() {
        cell_view_names(&model.library_manager, library, cell)
    } else {
        resolution.map_or_else(Vec::new, |resolution| resolution.view_names.clone())
    };
    let change = binding_fields(
        ui,
        &format!("instance-{path}"),
        BindingFields {
            views: &views,
            stop: stop.as_deref(),
            section: Some(&section),
            platforms: Some(&platforms),
            addable: &addable,
        },
        editable,
        fields,
    )
    .or_else(|| {
        (editable && own_views.is_some() && action_button(ui, "Inherit the view list", true))
            .then_some(BindingChange::Inherit)
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
    let mut unused = Fields {
        buffer: None,
        committed: None,
        sections: Vec::new(),
    };
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
    fields: &mut Fields,
) -> Option<InspectorAction> {
    // Reverting can take away a pattern binding the edits added: the
    // inspector then goes back to the configuration rather than to nothing.
    let Some(binding) = definition
        .overrides
        .iter()
        .find(|binding| binding.instance_path == pattern)
    else {
        return Some(InspectorAction::Deselect);
    };
    section_header(ui, "Pattern binding", None);
    let mut change = committed_text(
        ui,
        fields,
        &format!("pattern-{pattern}"),
        "Pattern",
        &binding.instance_path,
        editable,
    )
    .map(BindingChange::Path);
    let edited = binding_fields(
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
        fields,
    );
    change = change.or(edited);
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
    /// section and binds on every platform its instances do.
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
    text: &mut Fields,
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

    if let Some(current) = fields.section {
        if let Some(section) = committed_text(
            ui,
            text,
            &format!("{key}-section"),
            "Model section",
            current,
            editable,
        ) {
            change = Some(BindingChange::ModelSection(
                (!section.trim().is_empty()).then_some(section),
            ));
        }
        // The attached technology's sections, offered beside the field: a
        // typo is the commonest wrong section, and the field still takes a
        // section the technology does not list.
        if editable && !text.sections.is_empty() {
            let offered = text.sections.clone();
            let shown = if offered.iter().any(|section| section == current) {
                current
            } else {
                "Pick a section"
            };
            if let Some(index) = choice_row(
                ui,
                &format!("{key}-sections"),
                "Technology sections",
                shown,
                &offered,
                true,
            ) && offered[index] != current
            {
                change = Some(BindingChange::ModelSection(Some(offered[index].clone())));
            }
        }
    }

    let Some(platforms) = fields.platforms else {
        return change;
    };
    // A restriction, not a placement: a binding set to one platform binds
    // nothing on the other, so the instance does not resolve there.
    let only_on = only_on_label(platforms);
    let options = ONLY_ON.map(str::to_owned);
    if let Some(index) = choice_row(
        ui,
        &format!("{key}-only-on"),
        "Only on",
        only_on,
        &options,
        editable,
    ) && ONLY_ON[index] != only_on
    {
        change = Some(BindingChange::OnlyOn(match index {
            1 => vec![ConfigurationPlatform::Desktop],
            2 => vec![ConfigurationPlatform::Browser],
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
    fields: &mut Fields,
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
    let held = fields.buffer.as_ref().filter(|edit| edit.field == field);
    let refusal = held.and_then(|edit| edit.refusal.clone());
    let mut text = held.map_or_else(|| current.to_owned(), |edit| edit.text.clone());
    let mut committed = None;
    ui.horizontal(|ui| {
        let width = ui.available_width().max(80.0);
        let response = ui.add_sized(
            [width, 24.0],
            TextEdit::singleline(&mut text)
                .id(egui::Id::new(("configuration-field", field)))
                .font(theme::field_mono()),
        );
        if response.changed() {
            fields.buffer = Some(FieldEdit {
                field: field.to_owned(),
                text: text.clone(),
                refusal: None,
            });
        }
        if response.lost_focus() {
            let escaped = ui.input(|input| input.key_pressed(Key::Escape));
            let typed = fields
                .buffer
                .take_if(|edit| edit.field == field)
                .map(|edit| edit.text);
            if !escaped && let Some(typed) = typed {
                committed = (typed != current).then_some(typed);
            }
        }
    });
    if let Some(refusal) = &refusal {
        problem_line(ui, Problem::Failure, refusal);
    }
    if let Some(typed) = &committed {
        fields.committed = Some((field.to_owned(), typed.clone()));
    }
    ui.add_space(4.0);
    committed
}

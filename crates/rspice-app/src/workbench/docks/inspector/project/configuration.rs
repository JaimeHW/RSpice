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
use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::project::configuration::{
    ConfigurationTransaction, commit_configuration_transaction,
};
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::{property_row, property_row_wrapped};
use crate::workbench::project::configuration_state::{
    BindingChange, ConfigurationResolution, ConfigurationRow, ConfigurationSelection, FieldEdit,
    apply_binding_change, cell_view_names, governing_binding, instance_binding, runs_on_label,
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
    UseForRuns(ConfigurationSelection),
    Deselect,
}

pub(super) fn project_configuration(ui: &mut Ui, app: &mut RSpiceApp) {
    let state = &app.state;
    let catalog = &state.model.workspace.content.configuration_sets;
    let page = &state.project.configuration;
    let selection = page.selection(catalog);
    let resolution = page.last_resolution();
    let mut buffer = page.field_edit.clone();
    let definition = match selection {
        ConfigurationSelection::Configuration(id) => page.definition(catalog, id),
        ConfigurationSelection::Default => None,
    };
    let editable = definition.is_some() && !state.model.safe_mode.project_read_only();
    let action = match page.row.clone() {
        Some(ConfigurationRow::Instance(path)) => instance(
            ui,
            state,
            definition,
            resolution.as_deref(),
            &path,
            editable,
            &mut buffer,
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
                configuration(ui, state, selection, definition, editable, &mut buffer)
            }
            None => default_facts(ui, state),
        },
    };
    app.state.project.configuration.field_edit = buffer;
    let Some(action) = action else {
        return;
    };
    match action {
        InspectorAction::UseForRuns(target) => {
            let refusal =
                commit_configuration_transaction(app, ConfigurationTransaction::UseForRuns(target))
                    .err();
            app.state.project.configuration.refusal = refusal;
        }
        InspectorAction::Deselect => app.state.project.configuration.row = None,
        InspectorAction::Rename(name) => edit(&mut app.state, selection, |definition| {
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
            edit(&mut app.state, selection, |definition| {
                apply_binding_change(definition, &path, &inherited, change);
            });
            let page = &mut app.state.project.configuration;
            if let Some(pattern) = renamed {
                page.row = Some(ConfigurationRow::Pattern(pattern));
            } else if removed && matches!(page.row, Some(ConfigurationRow::Pattern(_))) {
                page.row = None;
            }
        }
    }
}

fn edit(
    state: &mut AppState,
    selection: ConfigurationSelection,
    change: impl FnOnce(&mut ConfigurationSetDefinition),
) {
    if let ConfigurationSelection::Configuration(id) = selection {
        state.project.configuration.edit(
            &state.model.workspace.content.configuration_sets,
            id,
            change,
        );
    }
}

/// The built-in Default's facts.
fn default_facts(ui: &mut Ui, state: &AppState) -> Option<InspectorAction> {
    let project = &state.model.workspace.content.project;
    let catalog = &state.model.workspace.content.configuration_sets;
    section_header(ui, "Configuration", None);
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
    used_for_runs_row(ui, state, ConfigurationSelection::Default, catalog)
}

/// "Used for runs" as a fact, or the action that makes it one.
fn used_for_runs_row(
    ui: &mut Ui,
    state: &AppState,
    selection: ConfigurationSelection,
    catalog: &crate::state::ConfigurationSetCatalog,
) -> Option<InspectorAction> {
    if used_for_runs(catalog, selection) {
        property_row(ui, "Used for runs", "Yes");
        return None;
    }
    property_row(ui, "Used for runs", "No");
    action_button(
        ui,
        "Use for runs",
        !state.model.safe_mode.project_read_only(),
    )
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
    state: &AppState,
    selection: ConfigurationSelection,
    definition: &ConfigurationSetDefinition,
    editable: bool,
    buffer: &mut Option<FieldEdit>,
) -> Option<InspectorAction> {
    let catalog = &state.model.workspace.content.configuration_sets;
    section_header(ui, "Configuration", None);
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
    if let Some(use_for_runs) = used_for_runs_row(ui, state, selection, catalog) {
        action = Some(use_for_runs);
    }
    action
}

/// One instance: what it resolved to, and its own binding.
fn instance(
    ui: &mut Ui,
    state: &AppState,
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
    let Some(definition) = definition else {
        return None;
    };
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
    let addable = cell_view_names(
        &state.model.library_manager,
        &reference.library,
        &reference.cell,
    );
    let change = binding_fields(
        ui,
        &format!("instance-{path}"),
        BindingFields {
            views: &views_owned,
            stop: stop.as_deref(),
            section: &section,
            platforms: &platforms,
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

/// One pattern binding.
fn pattern_binding(
    ui: &mut Ui,
    definition: &ConfigurationSetDefinition,
    resolution: Option<&ConfigurationResolution>,
    pattern: &str,
    editable: bool,
    buffer: &mut Option<FieldEdit>,
) -> Option<InspectorAction> {
    let Some(binding) = definition
        .overrides
        .iter()
        .find(|binding| binding.instance_path == pattern)
    else {
        return None;
    };
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
            section: binding.model_section.as_deref().unwrap_or_default(),
            platforms: &binding.eligible_platforms,
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
    section: &'a str,
    platforms: &'a [ConfigurationPlatform],
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
        ui.add_space(10.0);
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

    if let Some(section) = committed_text(
        ui,
        buffer,
        &format!("{key}-section"),
        "Model section",
        fields.section,
        editable,
    ) {
        change = Some(BindingChange::ModelSection(
            (!section.trim().is_empty()).then_some(section),
        ));
    }

    let runs_on = runs_on_label(fields.platforms);
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
        ui.add_space(10.0);
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
        ui.add_space(10.0);
        let width = (ui.available_width() - 10.0).max(80.0);
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
        ui.add_space(10.0);
        let width = (ui.available_width() - 10.0).max(80.0);
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

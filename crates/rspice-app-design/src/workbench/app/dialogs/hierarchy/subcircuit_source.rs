//! Descend on an instance of a `.subckt`: its master is text a model library
//! holds, not a drawing, so descending reads that text, as Virtuoso descends
//! into a text view read-only.

use egui::{Context, RichText, ScrollArea};
use rspice_model_library::{ModelLibrary, ModelSubcircuitInterface};

use crate::state::Component;
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app_state::Model;

/// The subcircuit text open for reading.
#[derive(Debug, Clone, Default)]
pub struct SubcircuitSourceDialogState {
    pub open: bool,
    /// The subcircuit's name.
    pub(crate) title: String,
    /// Where its text is written.
    pub(crate) location: String,
    /// Its definition, `.subckt` through `.ends`.
    pub(crate) source: String,
}

/// The library and subcircuit an instance stands on, when its master is a
/// `.subckt` a model library holds.
fn instance_subcircuit<'a>(
    model: &'a Model,
    component: &Component,
) -> Option<(&'a ModelLibrary, &'a ModelSubcircuitInterface)> {
    let binding = component.library_cell.as_ref()?;
    let library = model.model_library_manager.get_library(&binding.library)?;
    let name = binding.module_name.as_deref().unwrap_or(&binding.cell);
    let interface = library.subcircuits.get(name).or_else(|| {
        library
            .subcircuits
            .values()
            .find(|interface| interface.name.eq_ignore_ascii_case(name))
    })?;
    Some((library, interface))
}

/// The one selected instance whose master is a `.subckt`, by id.
pub fn selected_instance_subcircuit(model: &Model) -> Option<u64> {
    let id = model
        .schematic
        .session
        .editor
        .selection
        .single_component()?;
    let component = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)?;
    instance_subcircuit(model, component).map(|_| id)
}

/// Open the `.subckt` behind instance `component_id` for reading: the text
/// the project retained for its file, or the file itself, cut to the
/// definition. Answers why when there is no text to show.
pub fn open_subcircuit_source(
    dialog: &mut SubcircuitSourceDialogState,
    model: &Model,
    component_id: u64,
) -> Result<(), String> {
    let component = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or("The instance is no longer on the sheet.")?;
    let (library, interface) = instance_subcircuit(model, component)
        .ok_or_else(|| format!("{} is not an instance of a subcircuit.", component.name))?;
    let path = interface
        .file_path
        .as_ref()
        .or(library.root_path.as_ref())
        .ok_or_else(|| format!("No file holds the text of subcircuit {}.", interface.name))?;
    let text = match library
        .source_contents
        .iter()
        .find(|content| content.path == *path)
    {
        Some(content) => String::from_utf8_lossy(&content.bytes).into_owned(),
        None => std::fs::read_to_string(path)
            .map_err(|error| format!("Could not read {}: {error}", path.display()))?,
    };
    let (line, source) = definition(&text, &interface.name, interface.source_line);
    *dialog = SubcircuitSourceDialogState {
        open: true,
        title: interface.name.clone(),
        location: match line {
            Some(line) => format!("{}:{line}", path.display()),
            None => path.display().to_string(),
        },
        source,
    };
    Ok(())
}

/// The definition of subcircuit `name` in `text`, `.subckt` through its
/// `.ends`, and the line it starts on; the whole text when it cannot be
/// found. The recorded line is where the search starts, so a file edited
/// since it was read still finds its definition.
fn definition(text: &str, name: &str, recorded_line: Option<usize>) -> (Option<usize>, String) {
    let lines = text.lines().collect::<Vec<_>>();
    let opens = |line: &str| {
        let mut words = line.split_whitespace();
        words
            .next()
            .is_some_and(|word| word.eq_ignore_ascii_case(".subckt"))
            && words
                .next()
                .is_some_and(|word| word.eq_ignore_ascii_case(name))
    };
    let from = recorded_line.unwrap_or(1).saturating_sub(1);
    let Some(start) = (from..lines.len())
        .chain(0..from.min(lines.len()))
        .find(|index| opens(lines[*index]))
    else {
        return (None, text.to_owned());
    };
    let end = (start + 1..lines.len())
        .find(|index| {
            lines[*index]
                .split_whitespace()
                .next()
                .is_some_and(|word| word.eq_ignore_ascii_case(".ends"))
        })
        .unwrap_or(lines.len() - 1);
    (Some(start + 1), lines[start..=end].join("\n"))
}

/// The subcircuit's text, read-only: selectable and copyable, never a
/// buffer an edit could be lost in.
pub fn render_subcircuit_source_dialog(dialog: &mut SubcircuitSourceDialogState, ctx: &Context) {
    if !dialog.open {
        return;
    }
    let choice = Dialog::new(dialog.title.clone(), "Close")
        .description(format!("The text of subcircuit {}.", dialog.title))
        .size(DialogSize::WideWorkflow)
        .hint(dialog.location.clone())
        .primary_on_enter(false)
        .note_only_footer()
        .show(ctx, |ui| {
            ui.label(
                RichText::new("Read-only: a subcircuit is edited where its library is kept.")
                    .small()
                    .color(Tokens::get(ui.ctx()).color.text_faint),
            );
            ui.add_space(tokens::SP_2);
            let mut body = dialog.source.as_str();
            ScrollArea::both()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut body)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(26),
                    );
                });
        });
    if !matches!(choice, DialogChoice::None) {
        *dialog = SubcircuitSourceDialogState::default();
    }
}

#[cfg(test)]
mod tests {
    use super::{definition, selected_instance_subcircuit};
    use crate::schematic::view::context_menu::{action_availability, execute_context_action};
    use crate::schematic::view::schematic_symbol_context;
    use crate::state::{LibraryCellInstance, Point};
    use crate::workbench::design::test_host::{DesignTestHost, canvas_facts, canvas_parts};
    use rspice_model_library::{ModelLibrary, ModelSourceContent, ModelSubcircuitInterface};
    use rspice_schematic_editor::view::context_menu::ContextAction;

    /// Descend on an instance of a `.subckt` opens the subcircuit's own text
    /// to read, where it used to refuse for want of a schematic master.
    #[test]
    fn descending_into_a_subcircuit_instance_opens_its_text() {
        const SOURCE: &str =
            "* opamps\n.subckt buf in out\nE1 out 0 in 0 1\n.ends buf\n.subckt other a\n.ends\n";
        let path = std::path::PathBuf::from("/lib/opamps.lib");
        let mut library = ModelLibrary {
            name: "opamps".to_owned(),
            ..ModelLibrary::default()
        };
        library.subcircuits.insert(
            "buf".to_owned(),
            ModelSubcircuitInterface {
                name: "buf".to_owned(),
                ports: vec!["in".to_owned(), "out".to_owned()],
                parameter_defaults: Default::default(),
                description: None,
                file_path: Some(path.clone()),
                source_line: Some(2),
                section: None,
            },
        );
        library.source_contents.push(ModelSourceContent {
            path,
            bytes: SOURCE.as_bytes().to_vec(),
        });
        let mut host = DesignTestHost::default();
        host.model.model_library_manager.add_library(library);
        let mut binding = LibraryCellInstance::new("opamps", "buf", "spice");
        binding.module_name = Some("buf".to_owned());
        let instance = host
            .model
            .schematic
            .add_library_cell_component(Point::origin(), binding);
        host.model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(instance);
        assert_eq!(selected_instance_subcircuit(&host.model), Some(instance));
        assert!(action_availability(ContextAction::DescendHierarchy, &canvas_facts(&host)).0);

        let symbol_context = schematic_symbol_context(&host);
        execute_context_action(
            ContextAction::DescendHierarchy,
            &mut canvas_parts(&mut host),
            Point::origin(),
            &symbol_context,
        );

        let dialog = &host.design.subcircuit_source;
        assert!(dialog.open);
        assert_eq!(
            dialog.source,
            ".subckt buf in out\nE1 out 0 in 0 1\n.ends buf"
        );
        assert_eq!(dialog.location, "/lib/opamps.lib:2");
    }

    /// The definition is cut from its `.subckt` to its `.ends`, found from
    /// the recorded line even when the file moved it.
    #[test]
    fn the_definition_runs_from_its_subckt_line_to_its_ends() {
        let text = "* lib\n.subckt other a b\nR1 a b 1k\n.ends\n.SUBCKT amp in out\nE1 out 0 in 0 1e5\n.ENDS amp\n";
        let (line, body) = definition(text, "amp", Some(2));
        assert_eq!(line, Some(5));
        assert_eq!(body, ".SUBCKT amp in out\nE1 out 0 in 0 1e5\n.ENDS amp");
        assert_eq!(definition(text, "missing", None).0, None);
    }
}

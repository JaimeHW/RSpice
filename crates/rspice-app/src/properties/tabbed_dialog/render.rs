//! Component property services and dialog lifecycle adapter.

use super::state::{ComponentPropertyContext, RetainedTableFile, TabbedPropertyDialogState};
use crate::properties::PropertyEditorSchema;
use crate::properties::model_browser::ModelBrowserState;
use crate::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use crate::simulation::stimulus_realize::PreviewTiming;
use crate::state::{Component, ComponentType, PropertyValue};
use egui::Ui;
use rspice_schematic_editor::component_properties::{
    ComponentPropertyDialogResult, ComponentPropertyDialogView, ComponentPropertyDraft,
    ComponentPropertyServices, PropertyBrowseRequest, render_component_property_dialog,
};

pub fn render_tabbed_property_dialog(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    context: &ComponentPropertyContext,
    registry: &PropertyEditorSchema,
    model_library_manager: &crate::state::ModelLibraryManager,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> ComponentPropertyDialogResult {
    if !state.open {
        return ComponentPropertyDialogResult::None;
    }
    let Some(kind) = state.draft.component_type else {
        state.close();
        return ComponentPropertyDialogResult::Cancelled;
    };
    let Some(sheet) = registry.get(kind) else {
        state.close();
        return ComponentPropertyDialogResult::Cancelled;
    };
    let view = ComponentPropertyDialogView {
        component_name: state.component_name.as_deref(),
        context: &context.editor,
        sheet,
        advisories: &state.source_advisories,
        session_error: state.session_error.clone(),
        model_browser_open: state.model_browser.open,
        show_source_preview: crate::simulation::stimulus_realize::is_independent_source(kind),
        quantity_policy,
        number_locale,
    };
    let mut services = ComponentDialogServices {
        reference: state.component_name.as_deref(),
        model_browser: &mut state.model_browser,
        session_error: &mut state.session_error,
        preview: ComponentPreview {
            baseline: state.component_baseline.as_ref(),
            component_id: state.component_id,
            kind,
            data_root: state.data_root.as_deref(),
            timing: state.preview_timing,
            retained_table: context.retained_table.as_ref(),
            registry,
        },
    };
    let result = render_component_property_dialog(ctx, &mut state.draft, view, &mut services);
    if result == ComponentPropertyDialogResult::Cancelled {
        state.close();
    }
    render_model_browser(ctx, state, model_library_manager);
    result
}

struct ComponentPreview<'a> {
    baseline: Option<&'a Component>,
    component_id: Option<u64>,
    kind: ComponentType,
    data_root: Option<&'a std::path::Path>,
    timing: PreviewTiming,
    retained_table: Option<&'a RetainedTableFile>,
    registry: &'a PropertyEditorSchema,
}

struct ComponentDialogServices<'a> {
    /// The instance's designator when the editor was opened.
    reference: Option<&'a str>,
    model_browser: &'a mut ModelBrowserState,
    session_error: &'a mut Option<String>,
    preview: ComponentPreview<'a>,
}

impl ComponentPropertyServices for ComponentDialogServices<'_> {
    fn browse(&mut self, request: PropertyBrowseRequest<'_>, draft: &mut ComponentPropertyDraft) {
        match request {
            PropertyBrowseRequest::DataFile(property) => {
                match attach_data_file(self.preview.data_root) {
                    Ok(Some(reference)) => {
                        draft.set_value(property, PropertyValue::String(reference));
                        *self.session_error = None;
                    }
                    Ok(None) => {}
                    Err(error) => *self.session_error = Some(error),
                }
            }
            PropertyBrowseRequest::Model => {
                let Some(kind) = draft.component_type else {
                    return;
                };
                let text = |name: &str| {
                    draft
                        .get_value(name)
                        .map(PropertyValue::display_string)
                        .map(|value| value.trim().to_owned())
                        .filter(|value| !value.is_empty())
                };
                // The designator as the draft holds it, so a rename typed
                // before Browse is the name the browser speaks to.
                let reference = text("name")
                    .or_else(|| self.reference.map(str::to_owned))
                    .unwrap_or_else(|| kind.display_name().to_owned());
                // The drawn size as the draft holds it, which selects the bin
                // of a binned family. An expression of design variables is
                // not evaluated here, and selects none.
                let dimension = |name: &str| {
                    let value = draft.get_value(name)?;
                    value.as_number().or_else(|| {
                        let text = value.display_string();
                        let text = text.trim();
                        let literal = text
                            .strip_prefix('{')
                            .and_then(|text| text.strip_suffix('}'))
                            .unwrap_or(text);
                        crate::quantity::parse_engineering_value(literal).ok()
                    })
                };
                self.model_browser.open_for_instance(
                    &reference,
                    kind,
                    text("model"),
                    text("model_library"),
                    crate::properties::model_browser::InstanceGeometry {
                        length: dimension("l"),
                        width: dimension("w"),
                    },
                );
            }
        }
    }
    fn preview_source(&mut self, ui: &mut Ui, draft: &ComponentPropertyDraft) {
        source_preview_card(ui, draft, &self.preview);
    }
}

/// The engine's own evaluation of this source, over the plan's transient.
///
/// Nothing here interprets a waveform. The draft becomes a component, the
/// component becomes a card, the card becomes a `SourceSpec` through the
/// engine's parser, and the curve is the transient evaluator stepped over the
/// stop time — so every substitution the engine makes for an omitted field is
/// visible here exactly as it will be in the run, which is what the dialog's
/// own sampler could not promise.
///
/// The curve alone: it is drawn inside the editor's Stimulus section, which
/// owns the band, the frame and the margin around it. It used to carry its own
/// band, titled `Transient stimulus preview` with the status `engine
/// evaluator`, above a plot whose caption already read `Engine-evaluated
/// waveform` — three statements of one fact, and a second banded block for
/// what is one subject with the library record under it.
///
/// The drawing itself belongs to [`crate::properties::source_preview`], which
/// the stimulus link dialog paints through as well: that dialog shows the card
/// a definition would leave on this instance, and a second painter is how two
/// surfaces come to disagree about the same curve.
fn source_preview_card(
    ui: &mut Ui,
    draft: &ComponentPropertyDraft,
    preview: &ComponentPreview<'_>,
) {
    let timing = preview.timing;
    let component = preview_component(draft, preview);
    let unit = component
        .as_ref()
        .map_or("V", crate::simulation::placed_sources::source_unit);
    let curve = component
        .ok_or_else(|| "This editor has no instance to evaluate.".to_owned())
        .and_then(|component| {
            let tables = preview_tables(preview, &component);
            crate::properties::source_preview::source_curve(&component, timing, tables)
        });
    crate::properties::source_preview::paint_preview_card(ui, &curve, timing, unit);
}

/// The instance as the draft currently describes it.
///
/// Built from the editor's live values rather than from the baseline, so the
/// curve follows the field being typed into; built through the property bridge
/// rather than field by field, so it is the same component a commit would
/// write.
fn preview_component(
    draft: &ComponentPropertyDraft,
    preview: &ComponentPreview<'_>,
) -> Option<Component> {
    let mut component = preview.baseline.cloned().or_else(|| {
        preview
            .component_id
            .map(|id| crate::state::Component::new(id, preview.kind, crate::state::Point::origin()))
    })?;
    component.kind = preview.kind;
    crate::properties::property_bridge::apply_properties_to_component(
        &mut component,
        &draft.values,
        preview.registry,
    )
    .ok()?;
    Some(component)
}

/// What the draft's data file can be found with: the project's folder, and the
/// adopted definition's retained copy while the draft still names the file
/// that copy stands in for.
fn preview_tables<'a>(
    preview: &ComponentPreview<'a>,
    draft: &Component,
) -> crate::simulation::table_route::TableSources<'a> {
    let reference = crate::simulation::stimulus_realize::data_file_reference(draft);
    crate::simulation::table_route::TableSources {
        data_root: preview.data_root,
        retained: preview
            .retained_table
            .filter(|table| reference.as_deref() == Some(table.reference.as_str()))
            .map(|table| table.path.as_path()),
    }
}

/// Folder inside a project that attached waveform data is copied into.
const PROJECT_DATA_DIR: &str = "data";

/// Ask for a waveform data file and return the reference to store for it.
///
/// A saved project takes a copy: the file is brought inside the project folder
/// and referenced relative to it, so the design keeps working when the folder is
/// moved, zipped, or handed to someone else — the same bargain every EDA tool
/// strikes, paying one duplicated file for a reference that cannot dangle. An
/// unsaved project has nowhere to copy to, so its reference stays absolute and
/// becomes relative the first time the project is saved somewhere.
///
/// `Ok(None)` means the picker was dismissed.
///
/// It takes the project's data root rather than an editor, because the
/// Stimulus Library imports the same kind of file for the same reason and
/// there must be exactly one route that decides where an attached waveform
/// lands.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn attach_data_file(
    data_root: Option<&std::path::Path>,
) -> Result<Option<String>, String> {
    let Some(source) = rfd::FileDialog::new()
        .add_filter("Waveform data", &["csv", "wav"])
        .add_filter("All files", &["*"])
        .pick_file()
    else {
        return Ok(None);
    };

    let Some(root) = data_root else {
        return Ok(Some(source.to_string_lossy().into_owned()));
    };
    // Already inside the project: reference it where it lies rather than
    // making a second copy of a file the project already owns.
    if let Ok(relative) = source.strip_prefix(root) {
        return Ok(Some(relative.to_string_lossy().replace('\\', "/")));
    }

    let name = source
        .file_name()
        .ok_or_else(|| "The selected path has no file name".to_owned())?;
    let directory = root.join(PROJECT_DATA_DIR);
    std::fs::create_dir_all(&directory)
        .map_err(|error| format!("Cannot create '{}': {error}", directory.display()))?;

    let destination = data_file_destination(&directory, &source, std::path::Path::new(name))?;
    if !destination.exists() {
        std::fs::copy(&source, &destination)
            .map_err(|error| format!("Cannot copy '{}': {error}", source.display()))?;
    }
    Ok(Some(format!(
        "{PROJECT_DATA_DIR}/{}",
        destination.file_name().unwrap_or(name).to_string_lossy()
    )))
}

/// Where a copy of `source` belongs in `directory`.
///
/// A slot already holding the identical file is returned as-is, so attaching
/// the same waveform to a second source does not carry in a second copy of it.
/// A slot holding a *different* file of the same name yields to the first free
/// numbered variant, so two unrelated `wave.csv` both survive.
#[cfg(not(target_arch = "wasm32"))]
fn data_file_destination(
    directory: &std::path::Path,
    source: &std::path::Path,
    name: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    let stem = name.file_stem().unwrap_or(name.as_os_str());
    let extension = name.extension();
    for attempt in 0..1000 {
        let mut candidate = if attempt == 0 {
            std::path::PathBuf::from(stem)
        } else {
            std::path::PathBuf::from(format!("{}-{}", stem.to_string_lossy(), attempt + 1))
        };
        if let Some(extension) = extension {
            candidate.set_extension(extension);
        }
        let candidate = directory.join(candidate);
        if !candidate.exists() || files_have_equal_contents(source, &candidate) {
            return Ok(candidate);
        }
    }
    Err(format!(
        "'{}' already holds too many files named like '{}'",
        directory.display(),
        name.display()
    ))
}

/// Whether two files hold the same bytes. An unreadable file is reported as
/// different, which costs a redundant copy rather than a silently wrong reuse.
#[cfg(not(target_arch = "wasm32"))]
fn files_have_equal_contents(left: &std::path::Path, right: &std::path::Path) -> bool {
    let (Ok(left_meta), Ok(right_meta)) = (std::fs::metadata(left), std::fs::metadata(right))
    else {
        return false;
    };
    if left_meta.len() != right_meta.len() {
        return false;
    }
    match (std::fs::read(left), std::fs::read(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn attach_data_file(
    _data_root: Option<&std::path::Path>,
) -> Result<Option<String>, String> {
    Err("Waveform data files are only available in the desktop application".to_owned())
}

fn render_model_browser(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    model_library_manager: &crate::state::ModelLibraryManager,
) {
    if state.model_browser.open {
        use crate::properties::model_browser::{ModelBrowserResult, render_model_browser};

        match render_model_browser(ctx, &mut state.model_browser, model_library_manager) {
            ModelBrowserResult::Selected { library, model } => {
                state.draft.set_value("model", PropertyValue::String(model));
                state
                    .draft
                    .set_value("model_library", PropertyValue::String(library));
                // Process corner is owned by the simulation plan. Clear any
                // legacy per-instance hint rather than presenting it as an
                // executable component property.
                state
                    .draft
                    .set_value("model_corner", PropertyValue::String(String::new()));
                state.model_browser.open = false;
            }
            ModelBrowserResult::Cancelled => {
                state.model_browser.open = false;
            }
            ModelBrowserResult::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::ComponentPropertySession;
    use crate::simulation::stimulus_realize;
    use crate::state::{Component, ComponentType, Point};

    fn editor(kind: ComponentType) -> (TabbedPropertyDialogState, PropertyEditorSchema) {
        let registry = PropertyEditorSchema::new();
        let mut component = Component::new(4, kind, Point::origin());
        component.name = format!("{}4", kind.spice_prefix());
        let values = crate::properties::property_bridge::collect_properties_from_component(
            &component, &registry,
        );
        let sheet = registry.get(kind).expect("sheet").clone();
        let mut state = TabbedPropertyDialogState::default();
        state.open_for_component(
            4,
            component.name.clone(),
            kind,
            &sheet,
            values,
            ComponentPropertySession::detached(component).with_preview_timing(
                stimulus_realize::PreviewTiming {
                    tstep: 1e-9,
                    tstop: 4e-6,
                    from_analysis: true,
                },
            ),
        );
        (state, registry)
    }

    fn preview<'a>(
        state: &'a TabbedPropertyDialogState,
        kind: ComponentType,
        registry: &'a PropertyEditorSchema,
    ) -> ComponentPreview<'a> {
        ComponentPreview {
            baseline: state.component_baseline.as_ref(),
            component_id: state.component_id,
            kind,
            data_root: state.data_root.as_deref(),
            timing: state.preview_timing,
            retained_table: None,
            registry,
        }
    }

    fn trace_of(
        state: &TabbedPropertyDialogState,
        kind: ComponentType,
        registry: &PropertyEditorSchema,
    ) -> stimulus_realize::WaveformTrace {
        let component =
            preview_component(&state.draft, &preview(state, kind, registry)).expect("a component");
        // Through the shared painter's own evaluation, so a test cannot agree
        // with a sampling the card does not use.
        crate::properties::source_preview::source_curve(
            &component,
            state.preview_timing,
            crate::simulation::table_route::TableSources::default(),
        )
        .expect("curve")
    }

    /// The draft is what the curve follows, and it reaches the engine through
    /// the same bridge a commit writes through.
    #[test]
    fn the_preview_follows_the_draft_rather_than_the_baseline() {
        let (mut state, registry) = editor(ComponentType::VoltageSourceSin);
        let before = trace_of(&state, ComponentType::VoltageSourceSin, &registry);
        state.draft.set_value("va", PropertyValue::number(9.0));
        let after = trace_of(&state, ComponentType::VoltageSourceSin, &registry);

        let peak =
            |trace: &stimulus_realize::WaveformTrace| trace.readouts().expect("readouts").maximum;
        // The sampling grid need not land exactly on a crest, so the amplitude
        // is read to within one sample of the sine's own curvature.
        assert!(peak(&after) > peak(&before) * 2.0, "{before:?} {after:?}");
        assert!((peak(&after) - 9.0).abs() < 1e-2, "{after:?}");
    }

    /// The card is read back through the netlist grammar, so a field the dialog
    /// holds as text is read the way the deck reads it. The old sampler parsed
    /// `1ms` itself and fell through to its own 2 µs default when it could not.
    #[test]
    fn the_preview_reads_a_period_authored_with_its_unit() {
        let (mut typed, registry) = editor(ComponentType::VoltageSourcePulse);
        typed
            .draft
            .set_value("per", PropertyValue::String("1ms".to_owned()));
        let (mut numeric, _) = editor(ComponentType::VoltageSourcePulse);
        numeric.draft.set_value("per", PropertyValue::number(1e-3));

        assert_eq!(
            trace_of(&typed, ComponentType::VoltageSourcePulse, &registry),
            trace_of(&numeric, ComponentType::VoltageSourcePulse, &registry)
        );
    }

    /// One render-review scene: the file stem it writes, the component the
    /// editor opens for, and the context the host would hand it.
    #[cfg(not(target_arch = "wasm32"))]
    type EditorScene = (
        &'static str,
        ComponentType,
        fn() -> ComponentPropertyContext,
    );

    /// A component that binds no model and adopts no waveform, so its editor
    /// is a parameter form and nothing else.
    #[cfg(not(target_arch = "wasm32"))]
    fn plain_context() -> ComponentPropertyContext {
        ComponentPropertyContext {
            editor: rspice_schematic_editor::component_properties::ComponentEditorContext {
                subtitle: "Resistor".to_owned(),
                library_cell: String::new(),
                stimulus: None,
            },
            retained_table: None,
        }
    }

    /// One editor context for a source that has adopted a definition the
    /// library has since moved past, so the Stimulus section offers every verb
    /// it has at once.
    #[cfg(not(target_arch = "wasm32"))]
    fn adopted_source_context() -> ComponentPropertyContext {
        use rspice_design::stimulus_library::provenance::ProvenanceState;
        use rspice_schematic_editor::component_properties::StimulusEditorContext;

        ComponentPropertyContext {
            editor: rspice_schematic_editor::component_properties::ComponentEditorContext {
                subtitle: "Independent voltage source".to_owned(),
                library_cell: String::new(),
                stimulus: Some(StimulusEditorContext {
                    state: ProvenanceState::Behind {
                        adopted: 1,
                        library: 2,
                    },
                    definition: Some("sensor_drive".to_owned()),
                    library_revision: Some(2),
                    library_is_empty: false,
                }),
            },
            retained_table: None,
        }
    }

    /// A device with a model to type into its form, instanced from a master.
    ///
    /// Where that model comes from is the design inspector's, not this
    /// dialog's: the editor is a parameter form, and the card that used to
    /// restate the binding beside it is gone.
    #[cfg(not(target_arch = "wasm32"))]
    fn bound_model_context() -> ComponentPropertyContext {
        ComponentPropertyContext {
            editor: rspice_schematic_editor::component_properties::ComponentEditorContext {
                subtitle: "nmos4 (symbol)".to_owned(),
                library_cell: "analogLib/nmos4/symbol".to_owned(),
                stimulus: None,
            },
            retained_table: None,
        }
    }

    /// Write the editor's three shapes to PNGs: a form on its own, a form with
    /// a model binding under it, and a form with the Stimulus section under
    /// it.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn print_component_editor_renders_for_review() {
        use std::io::Write as _;

        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        let stderr = std::io::stderr();
        let mut report = stderr.lock();
        let scenes: [EditorScene; 4] = [
            ("resistor", ComponentType::Resistor, plain_context),
            ("mosfet", ComponentType::Nmos, bound_model_context),
            (
                "source-pulse",
                ComponentType::VoltageSourcePulse,
                adopted_source_context,
            ),
            // The point table and its summary, which no other scene draws.
            (
                "source-pwl",
                ComponentType::VoltageSourcePwl,
                adopted_source_context,
            ),
        ];
        for (stem, kind, context) in scenes {
            let (mut state, registry) = editor(kind);
            let context = context();
            let library = crate::state::ModelLibraryManager::default();
            let canvas = crate::ui::raster::render_themed(
                crate::ui::Theme::default(),
                egui::vec2(1100.0, 760.0),
                |ui, background| {
                    let _ = background;
                    render_tabbed_property_dialog(
                        ui.ctx(),
                        &mut state,
                        &context,
                        &registry,
                        &library,
                        QuantityPresentationPolicy::default(),
                        UiNumberLocale::default(),
                    );
                },
            );
            let height = canvas.content_height().max(1);
            let path = directory.join(format!("component-editor-{stem}.png"));
            std::fs::write(&path, canvas.png(height)).expect("write dialog render");
            writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                .expect("write raster qualification report");
        }
    }

    /// The pin the hand-rolled sampler could not hold: a noise source is not a
    /// waveform the spec can be asked for, so the card says so instead of
    /// drawing a plausible-looking realization nobody will ever simulate.
    #[test]
    fn a_noise_source_states_its_defect_rather_than_drawing_a_realization() {
        let (state, registry) = editor(ComponentType::CurrentSourceNoise);
        let component = preview_component(
            &state.draft,
            &preview(&state, ComponentType::CurrentSourceNoise, &registry),
        )
        .expect("built");
        let spec = stimulus_realize::source_spec(&component).expect("spec");

        assert!(stimulus_realize::preview_defect(&spec).is_some());
    }
}

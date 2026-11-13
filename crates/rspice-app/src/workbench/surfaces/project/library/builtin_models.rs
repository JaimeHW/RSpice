//! The built-in Verilog-A models on the Library page.
//!
//! A read-only library, the way `ahdlLib` is in a Library Manager: the models
//! stand grouped by device class in the cell column, a model's source files in
//! the view column, and Copy cell… takes an editable copy into a writable
//! library. Nothing here is a project object: the library exists only on this
//! page, so a project that never opens it carries none of it.

use rspice_app_models::veriloga_library::{
    BUILTIN_VERILOGA_LIBRARY, BUILTIN_VERILOGA_MODELS, BuiltinVerilogAModel,
    builtin_veriloga_model, read_builtin_veriloga_model,
};

use super::*;
use crate::workbench::features::read_only_veriloga_well;
use crate::workbench::slots::BuiltinModelsBrowser;

/// How much of the page the columns keep while a source is open below them.
const COLUMNS_HEIGHT_WITH_SOURCE: f32 = 214.0;
const SOURCE_BAR_HEIGHT: f32 = 30.0;

/// The columns' height: all of `available` until a source is open below them.
pub(super) fn columns_height(browser: &BuiltinModelsBrowser, available: f32) -> f32 {
    if browser.open && browser.source.is_some() {
        available.min(COLUMNS_HEIGHT_WITH_SOURCE)
    } else {
        available
    }
}

fn selected_model(browser: &BuiltinModelsBrowser) -> Option<&'static BuiltinVerilogAModel> {
    builtin_veriloga_model(browser.model.as_deref()?)
}

/// `library/cell/file`, the path the page's context strip reads.
pub(super) fn selection_path(browser: &BuiltinModelsBrowser) -> String {
    match (selected_model(browser), browser.file.as_deref()) {
        (Some(model), Some(file)) => {
            format!("{BUILTIN_VERILOGA_LIBRARY}/{}/{file}", model.name)
        }
        (Some(model), None) => format!("{BUILTIN_VERILOGA_LIBRARY}/{}", model.name),
        (None, _) => BUILTIN_VERILOGA_LIBRARY.to_owned(),
    }
}

/// Make `registry_name` the model the page shows, its root file selected.
fn select_model(browser: &mut BuiltinModelsBrowser, registry_name: &str) {
    let Some(model) = builtin_veriloga_model(registry_name) else {
        return;
    };
    let source = read_builtin_veriloga_model(model);
    let root = source.root.clone();
    browser.model = Some(registry_name.to_owned());
    browser.files = std::iter::once(root.clone())
        .chain(
            source
                .closure
                .files
                .iter()
                .filter(|file| source.files.contains_key(&file.path))
                .map(|file| file.path.clone()),
        )
        .collect();
    browser.file = Some(root);
    browser.texts = source.files;
}

/// Answer one of the built-in library's intents.
pub(super) fn apply(cx: &mut Cx<'_, ProjectFeature>, intent: BuiltinModelsIntent) {
    if let BuiltinModelsIntent::CopyModel(registry_name) = intent {
        if let Err(error) = cx
            .state
            .open_copy_builtin_model_dialog(cx.model, registry_name)
        {
            cx.wb.prompt(error);
        }
    } else {
        browse(&mut cx.state.builtin_models, intent);
    }
}

/// Put `file` in the open source pane, when there is one: the pane reads what
/// the columns have selected.
fn follow_selection(browser: &mut BuiltinModelsBrowser) {
    if browser.source.is_some() {
        browser.source = browser
            .file
            .as_ref()
            .and_then(|file| browser.texts.get(file))
            .cloned();
        browser.source_revision = browser.source_revision.wrapping_add(1);
    }
}

fn browse(browser: &mut BuiltinModelsBrowser, intent: BuiltinModelsIntent) {
    match intent {
        BuiltinModelsIntent::ShowLibrary => {
            browser.open = true;
            if browser.model.is_none()
                && let Some(first) = BUILTIN_VERILOGA_MODELS.first()
            {
                select_model(browser, first.registry_name);
            }
        }
        BuiltinModelsIntent::SelectModel(registry_name) => {
            select_model(browser, registry_name);
            follow_selection(browser);
        }
        BuiltinModelsIntent::SelectFile(file) => {
            browser.file = Some(file);
            follow_selection(browser);
        }
        BuiltinModelsIntent::OpenFile(file) => {
            browser.file = Some(file.clone());
            browser.source = browser.texts.get(&file).cloned();
            browser.source_revision = browser.source_revision.wrapping_add(1);
        }
        BuiltinModelsIntent::CloseSource => browser.source = None,
        BuiltinModelsIntent::CopyModel(_) => {}
    }
}

/// What the page's built-in library can ask for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BuiltinModelsIntent {
    ShowLibrary,
    SelectModel(&'static str),
    SelectFile(String),
    OpenFile(String),
    CloseSource,
    CopyModel(&'static str),
}

/// The library's row in the Library column, under the project's libraries.
pub(super) fn library_row(ui: &mut Ui, browser: &BuiltinModelsBrowser) -> Option<LibraryIntent> {
    let count = BUILTIN_VERILOGA_MODELS.len().to_string();
    browser_row(
        ui,
        BrowserRowSpec {
            icon: WorkbenchIcon::Folder,
            label: BUILTIN_VERILOGA_LIBRARY,
            meta: &count,
            status: "RO",
            selected: browser.open,
            read_only: true,
        },
    )
    .clicked()
    .then_some(LibraryIntent::BuiltinModels(
        BuiltinModelsIntent::ShowLibrary,
    ))
}

fn matches_filter(model: &BuiltinVerilogAModel, query: &str) -> bool {
    query.is_empty()
        || [
            model.name,
            model.module,
            model.summary,
            model.class.label(),
            model.version,
        ]
        .iter()
        .any(|field| field.to_ascii_lowercase().contains(query))
}

/// The heading a device class's models stand under, on the rows' own pitch.
fn class_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), BROWSER_ROW_HEIGHT),
        Sense::hover(),
    );
    ui.painter().text(
        pos2(rect.left() + 10.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, label));
}

pub(super) fn cell_column(
    ui: &mut Ui,
    browser: &BuiltinModelsBrowser,
    filter: &str,
    width: f32,
    height: f32,
) -> Option<LibraryIntent> {
    let query = filter.trim().to_ascii_lowercase();
    let shown = BUILTIN_VERILOGA_MODELS
        .iter()
        .filter(|model| matches_filter(model, &query))
        .collect::<Vec<_>>();
    let selected = selected_model(browser).map(|model| model.registry_name);
    browser_column(ui, "Cell", shown.len(), width, height, |ui| {
        let mut intent = None;
        let mut class = None;
        for model in &shown {
            if class != Some(model.class) {
                class = Some(model.class);
                class_heading(ui, model.class.label());
            }
            let response = browser_row(
                ui,
                BrowserRowSpec {
                    icon: WorkbenchIcon::Project,
                    label: model.name,
                    meta: model.version,
                    status: "",
                    selected: selected == Some(model.registry_name),
                    read_only: false,
                },
            )
            .on_hover_text(format!("{} (module {})", model.summary, model.module));
            if response.clicked() {
                intent = Some(LibraryIntent::BuiltinModels(
                    BuiltinModelsIntent::SelectModel(model.registry_name),
                ));
            }
        }
        intent
    })
}

pub(super) fn view_column(
    ui: &mut Ui,
    browser: &BuiltinModelsBrowser,
    width: f32,
    height: f32,
) -> Option<LibraryIntent> {
    browser_column(ui, "View", browser.files.len(), width, height, |ui| {
        let mut intent = None;
        for (index, file) in browser.files.iter().enumerate() {
            let response = browser_row(
                ui,
                BrowserRowSpec {
                    icon: if index == 0 {
                        WorkbenchIcon::VerilogA
                    } else {
                        WorkbenchIcon::Code
                    },
                    label: file,
                    meta: if index == 0 { "Verilog-A" } else { "Include" },
                    status: "",
                    selected: browser.file.as_deref() == Some(file.as_str()),
                    read_only: false,
                },
            );
            if response.double_clicked() {
                intent = Some(LibraryIntent::BuiltinModels(BuiltinModelsIntent::OpenFile(
                    file.clone(),
                )));
            } else if response.clicked() {
                intent = Some(LibraryIntent::BuiltinModels(
                    BuiltinModelsIntent::SelectFile(file.clone()),
                ));
            }
        }
        intent
    })
}

/// The band under the columns: what the selected model is, and the two things
/// a read-only library offers, Copy cell… while `libraries` holds a writable
/// one. The strip above already names the selection.
pub(super) fn detail(
    ui: &mut Ui,
    browser: &BuiltinModelsBrowser,
    libraries: &crate::state::LibraryManager,
) -> Option<LibraryIntent> {
    let t = Tokens::get(ui.ctx());
    let model = selected_model(browser);
    let writable_library_available = libraries
        .libraries_sorted()
        .into_iter()
        .any(|library| !library.read_only);
    let mut intent = None;
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.set_min_height(DETAIL_HEIGHT - 10.0);
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if browser.source.is_none()
                        && let Some(file) = browser.file.as_ref()
                        && Button::new("Open read-only").accent().show(ui).clicked()
                    {
                        intent = Some(LibraryIntent::BuiltinModels(BuiltinModelsIntent::OpenFile(
                            file.clone(),
                        )));
                    }
                    if let Some(model) = model
                        && Button::new("Copy cell\u{2026}")
                            .enabled(writable_library_available)
                            .show(ui)
                            .on_disabled_hover_text("No writable destination library is available.")
                            .clicked()
                    {
                        intent = Some(LibraryIntent::BuiltinModels(
                            BuiltinModelsIntent::CopyModel(model.registry_name),
                        ));
                    }
                    if let Some(model) = model {
                        let font = egui::TextStyle::Body.resolve(ui.style());
                        let summary = elide_text(ui, model.summary, &font, ui.available_width());
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            let label = ui.label(&summary);
                            if summary != model.summary {
                                label.on_hover_text(model.summary);
                            }
                        });
                    }
                });
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    intent
}

/// The open file, read-only, between the columns and the band: the editor well
/// every other Verilog-A source reads in, so it highlights the same.
pub(super) fn source_pane(
    ui: &mut Ui,
    browser: &mut BuiltinModelsBrowser,
    messages: crate::workbench::MessageCatalog,
    height: f32,
) -> Option<LibraryIntent> {
    let mut source = browser.source.take()?;
    let t = Tokens::get(ui.ctx());
    let mut intent = None;
    let file = browser.file.clone().unwrap_or_default();
    let revision = browser.source_revision;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), height),
        Layout::top_down(Align::Min),
        |ui| {
            let (bar, bar_response) = ui.allocate_exact_size(
                vec2(ui.available_width(), SOURCE_BAR_HEIGHT),
                Sense::hover(),
            );
            ui.painter().rect_filled(bar, 0.0, t.color.bg_panel);
            ui.painter().hline(
                bar.x_range(),
                bar.top(),
                Stroke::new(1.0, t.color.border_strong),
            );
            let title = format!("{file}  \u{00b7}  read only");
            ui.painter().text(
                pos2(bar.left() + 10.0, bar.center().y),
                Align2::LEFT_CENTER,
                &title,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
            );
            bar_response
                .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &title));
            let close = egui::Rect::from_min_size(
                pos2(bar.right() - 70.0, bar.top() + 3.0),
                vec2(62.0, SOURCE_BAR_HEIGHT - 6.0),
            );
            if ui
                .put(close, egui::Button::new("Close").frame(false))
                .clicked()
            {
                intent = Some(LibraryIntent::BuiltinModels(
                    BuiltinModelsIntent::CloseSource,
                ));
            }
            read_only_veriloga_well(
                ui,
                ("workbench.project.library.builtin-source", revision),
                &mut source,
                revision,
                messages,
            );
        },
    );
    browser.source = Some(source);
    intent
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::AppState;

    fn opened_on_a_model_with_includes() -> (BuiltinModelsBrowser, &'static BuiltinVerilogAModel) {
        let model = BUILTIN_VERILOGA_MODELS
            .iter()
            .find(|model| read_builtin_veriloga_model(model).files.len() > 1)
            .expect("a model with includes");
        let mut browser = BuiltinModelsBrowser::default();
        browse(&mut browser, BuiltinModelsIntent::ShowLibrary);
        let file = browser.file.clone().expect("the first model's root");
        browse(&mut browser, BuiltinModelsIntent::OpenFile(file));
        (browser, model)
    }

    fn shown_is_selected(browser: &BuiltinModelsBrowser) {
        let file = browser.file.as_ref().expect("a selected file");
        assert_eq!(browser.source.as_ref(), browser.texts.get(file), "{file}");
    }

    #[test]
    fn an_open_source_reads_what_the_columns_select() {
        let (mut browser, model) = opened_on_a_model_with_includes();
        shown_is_selected(&browser);

        let opened = browser.source_revision;
        browse(
            &mut browser,
            BuiltinModelsIntent::SelectModel(model.registry_name),
        );
        assert_eq!(browser.file.as_deref(), Some(browser.files[0].as_str()));
        shown_is_selected(&browser);
        assert_ne!(browser.source_revision, opened, "the editor reloads");

        let include = browser.files[1].clone();
        browse(
            &mut browser,
            BuiltinModelsIntent::SelectFile(include.clone()),
        );
        assert_eq!(browser.file.as_ref(), Some(&include));
        shown_is_selected(&browser);
    }

    /// The band's text for `state` on a page `width` wide.
    fn band_text(state: &AppState, width: f32) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 120.0),
            )),
            ..Default::default()
        };
        let pass = |ctx: &mut Ui| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = detail(
                    ui,
                    &state.project.builtin_models,
                    &state.model.library_manager,
                );
            });
        };
        // Two passes: the first builds the fonts the band is measured in.
        let _ = ctx.run_ui(input(), pass);
        ctx.run_ui(input(), pass)
            .shapes
            .into_iter()
            .filter_map(|clipped| match clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn the_band_states_the_summary_and_clips_it_before_the_buttons() {
        let mut state = AppState::default();
        browse(
            &mut state.project.builtin_models,
            BuiltinModelsIntent::ShowLibrary,
        );
        let model = selected_model(&state.project.builtin_models).expect("a selected model");

        let wide = band_text(&state, 1440.0);
        assert!(wide.iter().any(|text| text == model.summary), "{wide:?}");
        assert!(wide.iter().any(|text| text == "Open read-only"), "{wide:?}");
        let version = format!("version {}", model.version);
        for gone in [model.class.label(), version.as_str(), "read only"] {
            assert!(
                !wide.iter().any(|text| text == gone),
                "{gone:?} in {wide:?}"
            );
        }

        let narrow = band_text(&state, 280.0);
        let clipped = narrow
            .iter()
            .filter(|text| *text != "Copy cell\u{2026}")
            .find_map(|text| text.strip_suffix('\u{2026}'))
            .unwrap_or_else(|| panic!("no clipped summary in {narrow:?}"));
        assert!(model.summary.starts_with(clipped), "{clipped:?}");
        assert!(
            narrow.iter().any(|text| text == "Open read-only"),
            "{narrow:?}"
        );
    }

    #[test]
    fn a_closed_source_stays_closed_while_the_selection_moves() {
        let (mut browser, model) = opened_on_a_model_with_includes();
        browse(&mut browser, BuiltinModelsIntent::CloseSource);
        browse(
            &mut browser,
            BuiltinModelsIntent::SelectModel(model.registry_name),
        );
        let include = browser.files[1].clone();
        browse(&mut browser, BuiltinModelsIntent::SelectFile(include));
        assert!(browser.source.is_none());
    }
}

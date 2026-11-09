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
use crate::workbench::shell_host::read_only_veriloga_well;
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
    browser.source = None;
}

/// Answer one of the built-in library's intents.
pub(super) fn apply(state: &mut AppState, intent: BuiltinModelsIntent) {
    let browser = &mut state.project.builtin_models;
    match intent {
        BuiltinModelsIntent::ShowLibrary => {
            browser.open = true;
            if browser.model.is_none()
                && let Some(first) = BUILTIN_VERILOGA_MODELS.first()
            {
                select_model(browser, first.registry_name);
            }
        }
        BuiltinModelsIntent::SelectModel(registry_name) => select_model(browser, registry_name),
        BuiltinModelsIntent::SelectFile(file) => browser.file = Some(file),
        BuiltinModelsIntent::OpenFile(file) => {
            browser.file = Some(file.clone());
            browser.source = browser.texts.get(&file).cloned();
            browser.source_revision = browser.source_revision.wrapping_add(1);
        }
        BuiltinModelsIntent::CloseSource => browser.source = None,
        BuiltinModelsIntent::CopyModel(registry_name) => {
            if let Err(error) = state
                .project
                .open_copy_builtin_model_dialog(&state.model, registry_name)
            {
                state.prompt(error);
            }
        }
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
    let (rect, _) = ui.allocate_exact_size(
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

/// The band under the columns: what is selected, and the two things a
/// read-only library offers.
pub(super) fn detail(ui: &mut Ui, state: &AppState) -> Option<LibraryIntent> {
    let t = Tokens::get(ui.ctx());
    let browser = &state.project.builtin_models;
    let model = selected_model(browser);
    let writable_library_available = state
        .model
        .library_manager
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
                ui.label(
                    egui::RichText::new(selection_path(browser))
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text),
                );
                if let Some(model) = model {
                    ui.separator();
                    ui.label(model.class.label());
                    ui.separator();
                    ui.label(format!("version {}", model.version));
                }
                ui.separator();
                ui.label("read only");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if let Some(file) = browser.file.as_ref()
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
            let (bar, _) = ui.allocate_exact_size(
                vec2(ui.available_width(), SOURCE_BAR_HEIGHT),
                Sense::hover(),
            );
            ui.painter().rect_filled(bar, 0.0, t.color.bg_panel);
            ui.painter().hline(
                bar.x_range(),
                bar.top(),
                Stroke::new(1.0, t.color.border_strong),
            );
            ui.painter().text(
                pos2(bar.left() + 10.0, bar.center().y),
                Align2::LEFT_CENTER,
                format!("{file}  \u{00b7}  read only"),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
            );
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

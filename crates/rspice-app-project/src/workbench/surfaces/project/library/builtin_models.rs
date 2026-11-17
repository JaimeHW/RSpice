//! The built-in Verilog-A models on the Library page.
//!
//! A read-only library, the way `ahdlLib` is in a Library Manager: the models
//! stand grouped by device class in the cell column, a model's source files in
//! the view column, and Copy cell… takes an editable copy into a writable
//! library. Nothing here is a project object: the library exists only on this
//! page, so a project that never opens it carries none of it.

use rspice_app_state::veriloga_library::{
    BUILTIN_VERILOGA_LIBRARY, BUILTIN_VERILOGA_MODELS, BuiltinVerilogAModel,
    builtin_veriloga_model, read_builtin_veriloga_model,
};

use super::*;
use crate::workbench::framework::feature::CodePort;
use crate::workbench::slots::BuiltinModelsBrowser;

/// The list rows the columns keep in view while a source is open below them.
const ROWS_KEPT_WITH_SOURCE: f32 = 7.0;

/// The columns' height: all of `available` until a source is open below them,
/// then their heading and a few rows, on the live metrics.
pub(super) fn columns_height(ui: &Ui, browser: &BuiltinModelsBrowser, available: f32) -> f32 {
    if browser.open && browser.source.is_some() {
        let t = Tokens::get(ui.ctx());
        let kept = column_head_height(ui)
            + ROWS_KEPT_WITH_SOURCE * crate::ui::widgets::table::LineHeights::list(&t).row;
        available.min(kept)
    } else {
        available
    }
}

/// The models a filter leaves, in the catalogue's order: grouped by class.
pub(super) fn shown_models(filter: &str) -> Vec<&'static BuiltinVerilogAModel> {
    let query = filter.trim().to_ascii_lowercase();
    BUILTIN_VERILOGA_MODELS
        .iter()
        .filter(|model| matches_filter(model, &query))
        .collect()
}

pub fn selected_model(browser: &BuiltinModelsBrowser) -> Option<&'static BuiltinVerilogAModel> {
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

pub fn browse(browser: &mut BuiltinModelsBrowser, intent: BuiltinModelsIntent) {
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
pub enum BuiltinModelsIntent {
    ShowLibrary,
    SelectModel(&'static str),
    SelectFile(String),
    OpenFile(String),
    CloseSource,
    CopyModel(&'static str),
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

/// The menu a model or file row opens: the band's own two actions.
pub(super) fn row_items(
    ui: &mut Ui,
    browser: &BuiltinModelsBrowser,
    writable_library_available: bool,
) -> Option<LibraryIntent> {
    let mut intent = None;
    if let Some(file) = browser.file.as_ref()
        && menu_item(ui, "Open read-only", None)
    {
        intent = Some(LibraryIntent::BuiltinModels(BuiltinModelsIntent::OpenFile(
            file.clone(),
        )));
    }
    if let Some(model) = selected_model(browser)
        && menu_item(
            ui,
            "Copy cell\u{2026}",
            (!writable_library_available)
                .then_some("Every library is read-only; none can take a copy."),
        )
    {
        intent = Some(LibraryIntent::BuiltinModels(
            BuiltinModelsIntent::CopyModel(model.registry_name),
        ));
    }
    intent
}

/// The band under the columns: what the selected model is, and the two things
/// a read-only library offers, Copy cell… while `libraries` holds a writable
/// one. `creation` draws the page's Library actions at its right end; the strip
/// above already names the selection.
pub fn detail(
    ui: &mut Ui,
    browser: &BuiltinModelsBrowser,
    libraries: &crate::state::LibraryManager,
    creation: impl FnOnce(&mut Ui) -> Option<LibraryIntent>,
) -> Option<LibraryIntent> {
    let t = Tokens::get(ui.ctx());
    let model = selected_model(browser);
    let writable_library_available = libraries
        .libraries_by_key()
        .any(|(_, library)| !library.read_only);
    let mut intent = None;
    let shown = bar_frame(ui, t.color.bg_panel).show(ui, |ui| {
        ui.set_min_height(t.metrics.ctl_h);
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
                        .on_disabled_hover_text("Every library is read-only; none can take a copy.")
                        .clicked()
                {
                    intent = Some(LibraryIntent::BuiltinModels(
                        BuiltinModelsIntent::CopyModel(model.registry_name),
                    ));
                }
                if let Some(asked) = creation(ui) {
                    intent = Some(asked);
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
    code: &dyn CodePort,
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
            let (bar, bar_response) =
                ui.allocate_exact_size(vec2(ui.available_width(), bar_height(ui)), Sense::hover());
            ui.painter().rect_filled(bar, 0.0, t.color.bg_panel);
            ui.painter().hline(
                bar.x_range(),
                bar.top(),
                Stroke::new(1.0, t.color.border_strong),
            );
            let title = format!("{file}  \u{00b7}  read-only");
            ui.painter().text(
                pos2(bar.left() + 10.0, bar.center().y),
                Align2::LEFT_CENTER,
                &title,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
            );
            bar_response
                .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &title));
            let close = egui::Rect::from_center_size(
                pos2(bar.right() - 10.0 - 31.0, bar.center().y),
                vec2(62.0, t.metrics.ctl_h),
            );
            if ui
                .put(close, egui::Button::new("Close").frame(false))
                .clicked()
            {
                intent = Some(LibraryIntent::BuiltinModels(
                    BuiltinModelsIntent::CloseSource,
                ));
            }
            code.read_only_veriloga_well(
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

//! Which schematic instances name which model, and which name nothing.
//!
//! One pass over the placed components answers both questions, so one pass is
//! what the workspace makes: the catalog's "used by" column, the detail pane's
//! consumer list and the unresolved-binding findings are three readings of the
//! same index rather than three walks of the design.
//!
//! It lives beside the manager rather than inside it because it is a
//! derivation about the *design*, not about the catalog page that happens to
//! read it first — and because the manager is a capped file with no room for
//! anything that has somewhere better to be.

use super::*;

/// Which schematic instances name which model.
///
/// One pass over the placed components per frame instead of one per model.
/// Deriving it per model made the catalog cost the product of the corpus and
/// the schematic, and the facet chips paid it again for every facet.
pub(super) struct ConsumerIndex {
    by_provider: BTreeMap<(String, String, String), Vec<String>>,
    pub(super) diagnostics: Vec<BindingDiagnostic>,
}

/// One placed instance whose model reference does not resolve to a provider.
///
/// The page used to compress these to a single sentence — the first finding
/// and a count of the rest — and discard the remainder, which is the one shape
/// that cannot be acted on: a reader with eleven unresolved instances was told
/// about one of them and left to find the other ten by opening the schematic.
/// Every field here is what a reader needs to fix one row: which instance,
/// which sheet it sits on, what it asked for, and why nothing answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BindingDiagnostic {
    pub component_id: u64,
    pub instance: String,
    pub sheet: String,
    /// The model name the instance names.
    pub reference: String,
    pub reason: String,
}

#[cfg(test)]
thread_local! {
    /// How many consumer indexes have been built on this thread.
    ///
    /// The doc above says one pass per frame, and for a while the page built
    /// two: the second was hidden inside the detail pane, which resolved the
    /// selected model's consumers by building a whole index and reading one
    /// entry out of it. Nothing in the painted frame said so — both indexes
    /// agree, because they are the same derivation — which is why this is
    /// counted rather than asserted about the result.
    pub(crate) static CONSUMER_INDEX_BUILDS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

impl ConsumerIndex {
    pub(super) fn build(app: &ManagerRenderContext<'_>) -> Self {
        #[cfg(test)]
        CONSUMER_INDEX_BUILDS.with(|count| count.set(count.get() + 1));

        let mut by_provider = BTreeMap::<(String, String, String), Vec<String>>::new();
        let mut diagnostics = Vec::new();
        let Some(schematic) = app.state.workspace.active_schematic() else {
            return Self {
                by_provider,
                diagnostics,
            };
        };
        let sheet = app.state.workspace.active_view.display_path();
        for component in &schematic.components {
            let Some(model) = explicit_component_model(component) else {
                continue;
            };
            let unresolved = |reason: String| BindingDiagnostic {
                component_id: component.id,
                instance: component.name.clone(),
                sheet: sheet.clone(),
                reference: model.clone(),
                reason,
            };
            let label = format!(
                "{} · {} · ({}, {})",
                component.name,
                component.kind.display_name(),
                component.pos.x,
                component.pos.y
            );
            let declared_provider = crate::state::parse_params_string(&component.params)
                .get("model_library")
                .map(|provider| provider.trim().to_owned())
                .filter(|provider| !provider.is_empty());
            match app
                .state
                .model_library_manager
                .effective_definition_provider(ModelConsumerScope::PrimitiveModel, &model)
            {
                Ok(Some(provider))
                    if declared_provider.as_deref().is_none_or(|declared| {
                        declared.eq_ignore_ascii_case(&provider.library)
                    }) =>
                {
                    by_provider
                        .entry((
                            provider.library.to_ascii_lowercase(),
                            provider.definition.to_ascii_lowercase(),
                            provider.source_digest.to_string(),
                        ))
                        .or_default()
                        .push(label);
                }
                Ok(Some(provider)) => diagnostics.push(unresolved(format!(
                    "declares provider '{}', but the executable provider is '{}'",
                    declared_provider.as_deref().unwrap_or_default(),
                    provider.library
                ))),
                Ok(None) => diagnostics.push(unresolved(
                    "no library in this project provides an executable definition".to_owned(),
                )),
                Err(error) => diagnostics.push(unresolved(error.to_string())),
            }
        }
        for consumers in by_provider.values_mut() {
            consumers.sort();
        }
        diagnostics.sort_by(|left, right| {
            left.instance
                .cmp(&right.instance)
                .then_with(|| left.reference.cmp(&right.reference))
        });
        Self {
            by_provider,
            diagnostics,
        }
    }

    pub(super) fn of(&self, library: &ModelLibrary, model_name: &str) -> &[String] {
        self.by_provider
            .get(&(
                library.name.to_ascii_lowercase(),
                model_name.to_ascii_lowercase(),
                model_library_source_digest(library).to_string(),
            ))
            .map_or(&[], Vec::as_slice)
    }
}

/// Rows the card lists before the scroll area takes over the arithmetic.
///
/// The count can scale with the schematic — a sheet where the model library
/// failed to load has one finding per instance on it — so the list is
/// virtualized and the header states the total.
const UNRESOLVED_ROW_H: f32 = 26.0;

/// Every instance whose model reference does not resolve, and the two ways out.
///
/// Exception-only: a design where every instance resolves renders nothing at
/// all. When there is something to say, it is said in full — one row per
/// instance rather than one sentence and a count of what was thrown away.
pub(super) fn unresolved_card(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    diagnostics: &[BindingDiagnostic],
) {
    if diagnostics.is_empty() {
        return;
    }
    let selected_model = app
        .state
        .model_library_manager
        .selected_library
        .clone()
        .zip(app.state.workbench.selected_model.clone());
    let t = Tokens::get(ui.ctx());
    card(ui, |ui| {
        card_title(
            ui,
            "UNRESOLVED INSTANCE BINDINGS",
            Some(&format!(
                "{} instance{}",
                diagnostics.len(),
                if diagnostics.len() == 1 { "" } else { "s" }
            )),
        );
        ui.label(
            RichText::new(
                "Each of these names a model no library in this project provides under the \
                 provider it declares. A run refuses the whole design until every one resolves.",
            )
            .small()
            .color(t.color.text_dim),
        );
        let height = (diagnostics.len() as f32 * UNRESOLVED_ROW_H).min(190.0);
        ScrollArea::vertical()
            .id_salt("models-unresolved-bindings")
            .max_height(height.max(UNRESOLVED_ROW_H))
            .show_rows(ui, UNRESOLVED_ROW_H, diagnostics.len(), |ui, range| {
                for diagnostic in &diagnostics[range] {
                    unresolved_row(ui, app, diagnostic, selected_model.as_ref());
                }
            });
    });
}

fn unresolved_row(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    diagnostic: &BindingDiagnostic,
    selected_model: Option<&(String, String)>,
) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        ui.label(
            RichText::new(&diagnostic.instance)
                .monospace()
                .small()
                .color(t.color.text),
        );
        ui.label(
            RichText::new(&diagnostic.sheet)
                .small()
                .color(t.color.text_faint),
        );
        ui.label(
            RichText::new(&diagnostic.reference)
                .monospace()
                .small()
                .color(t.color.warn),
        );
        ui.label(
            RichText::new(&diagnostic.reason)
                .small()
                .color(t.color.text_dim),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if Button::new("Show instance")
                .show(ui)
                .on_hover_text("Selects this instance and opens the schematic it sits on.")
                .clicked()
            {
                app.state
                    .schematic
                    .selection
                    .select_only_component(diagnostic.component_id);
                navigate_specialist(app, crate::workbench::SurfaceId::Design);
            }
            // The catalog's selected model is the thing this instance would be
            // bound to, so the refusal names it rather than saying "select a
            // model" beside a page that already has one selected.
            let blocked = match selected_model {
                None => {
                    Some("Select a model in the catalog above to bind this instance to.".to_owned())
                }
                Some((library, model)) => {
                    crate::workbench::docks::validate_component_model_catalog_binding(
                        app.state,
                        diagnostic.component_id,
                        library,
                        model,
                    )
                    .err()
                }
            };
            let bind = Button::new("Bind to selection…")
                .enabled(blocked.is_none())
                .show(ui);
            let bind = match blocked.as_deref() {
                Some(reason) => bind.on_disabled_hover_text(reason),
                None => bind,
            };
            if bind.clicked()
                && let Some((library, model)) = selected_model
            {
                app.queue_model_binding(diagnostic.component_id, library, model);
            }
        });
    });
}

fn model_consumers_for_provider(
    app: &ManagerRenderContext<'_>,
    library: &ModelLibrary,
    model_name: &str,
) -> Vec<String> {
    ConsumerIndex::build(app).of(library, model_name).to_vec()
}

pub(super) fn effective_model_consumers(
    app: &ManagerRenderContext<'_>,
    model_name: &str,
) -> Vec<String> {
    app.state
        .model_library_manager
        .effective_definition_provider(ModelConsumerScope::PrimitiveModel, model_name)
        .ok()
        .flatten()
        .and_then(|provider| {
            app.state
                .model_library_manager
                .get_library(&provider.library)
                .map(|library| model_consumers_for_provider(app, library, model_name))
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;

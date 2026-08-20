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
    pub(super) diagnostics: Vec<String>,
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
        for component in &schematic.components {
            let Some(model) = explicit_component_model(component) else {
                continue;
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
                Ok(Some(provider)) => diagnostics.push(format!(
                    "{} declares provider '{}' for model '{}', but the executable provider is '{}'",
                    component.name,
                    declared_provider.as_deref().unwrap_or_default(),
                    model,
                    provider.library
                )),
                Ok(None) => diagnostics.push(format!(
                    "{} references model '{}', which has no executable provider",
                    component.name, model
                )),
                Err(error) => diagnostics.push(format!("{}: {error}", component.name)),
            }
        }
        for consumers in by_provider.values_mut() {
            consumers.sort();
        }
        diagnostics.sort();
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

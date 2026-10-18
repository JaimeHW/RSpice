//! Derivation of the project catalog's rows from the loaded corpus.
//!
//! Pure functions over the model libraries and the consumer index: one scan
//! per frame builds the rows, the facet counts, and the review tally in a
//! single pass, and the sort reorders only what that pass produced. Nothing
//! here paints or reads the screen, which is what lets the catalog's chips,
//! table, and footer all state the same pass over the design.

use super::*;

pub(super) fn project_catalog_scan(
    app: &ManagerRenderContext<'_>,
    consumers: &ConsumerIndex,
) -> ProjectCatalogScan {
    let query = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    let facet = app.state.workbench.models_view.project_facet;
    let mut rows = Vec::new();
    let mut total = 0usize;
    let mut facets = [0usize; ProjectModelFacet::ALL.len()];
    let mut review_shown = 0usize;
    for library in app.state.model_library_manager.libraries_sorted() {
        let pinned = model_is_pinned(library);
        let built_in = matches!(library.source_authority, ModelSourceAuthority::BuiltIn);
        let drifted = !drift::findings_for(app.state, &library.name).is_empty();
        let library_source = library.root_path.as_deref().map(path_label);
        // A binned family is one row, named as an instance binds it; its bins
        // are its detail, not rows beside it.
        for entry in library.bindable_models() {
            let cards = entry.cards();
            let model = cards[0];
            let (usages, bins) = match &entry {
                BindableModel::Card(card) => (Cow::Borrowed(consumers.of(library, &card.name)), 0),
                BindableModel::Family(family) => (
                    Cow::Owned(consumers.of_family(library, family)),
                    family.bins.len(),
                ),
            };
            let review = cards.iter().any(|card| model_needs_review(library, card));
            total += 1;
            let matches = |facet: ProjectModelFacet| match facet {
                ProjectModelFacet::All => true,
                ProjectModelFacet::Bound => !usages.is_empty(),
                ProjectModelFacet::Pinned => pinned,
                ProjectModelFacet::Review => review,
                ProjectModelFacet::BuiltIn => built_in,
            };
            for (index, candidate) in ProjectModelFacet::ALL.into_iter().enumerate() {
                facets[index] += usize::from(matches(candidate));
            }
            if !matches(facet) {
                continue;
            }
            // A family is found by its own name or by anything one of its bins
            // reports.
            if !query.is_empty()
                && !contains_query(entry.name(), &query)
                && !cards
                    .iter()
                    .any(|card| model_matches_query(library, card, &query))
            {
                continue;
            }
            if review {
                review_shown += 1;
            }
            rows.push(ProjectModelRow {
                library: library.name.clone(),
                model: entry.name().to_owned(),
                bins,
                family: model.model_type.display_name(),
                source: model
                    .file_path
                    .as_deref()
                    .map(path_label)
                    .or_else(|| library_source.clone())
                    .unwrap_or_else(|| match library.source_authority {
                        ModelSourceAuthority::BuiltIn => "RSpice built-in".to_owned(),
                        ModelSourceAuthority::External => "external source".to_owned(),
                        ModelSourceAuthority::RetainedImport { .. } => "retained import".to_owned(),
                        ModelSourceAuthority::ProjectOwned { .. } => "project source".to_owned(),
                    }),
                pinned,
                review,
                drifted,
                usage: usages
                    .first()
                    .map(|consumer| bindings::consumer_designator(consumer).to_owned()),
                usage_count: usages.len(),
                // Qualification is kept per card, so a family's vectors are
                // its bins' together.
                vectors: cards
                    .iter()
                    .filter_map(|card| library.model_qualification.get(&card.name))
                    .flat_map(|qualification| &qualification.suites)
                    .map(|suite| suite.vectors.len())
                    .sum(),
            });
        }
    }
    sort_catalog_rows(&mut rows, app.state.workbench.models_view.catalog_sort);
    ProjectCatalogScan {
        rows,
        total,
        facets,
        review: review_shown,
        consumer_diagnostics: consumers.diagnostics.clone(),
    }
}

/// Put the derived rows in the order the reader asked for.
///
/// This reorders a vector the page has already built — the rows that survived
/// the facet and the query — so a header click costs a sort of what is on the
/// page and never another pass over the corpus. The model identity is always
/// the final tie-break, which is what keeps the order total: two rows equal on
/// the chosen column still land in the same place on every frame.
pub(super) fn sort_catalog_rows(rows: &mut [ProjectModelRow], sort: ModelsTableSort) {
    rows.sort_by(|left, right| {
        let primary = match sort.key {
            // The identity tie-break below *is* the model order.
            ModelsCatalogSortKey::Model => std::cmp::Ordering::Equal,
            ModelsCatalogSortKey::Family => case_folded_cmp(left.family, right.family),
            ModelsCatalogSortKey::Source => case_folded_cmp(&left.source, &right.source),
            ModelsCatalogSortKey::UsedBy => case_folded_cmp(
                left.usage.as_deref().unwrap_or(""),
                right.usage.as_deref().unwrap_or(""),
            ),
            ModelsCatalogSortKey::Vectors => left.vectors.cmp(&right.vectors),
            ModelsCatalogSortKey::Status => catalog_status(left)
                .label()
                .cmp(catalog_status(right).label()),
        };
        let identity = case_folded_cmp(&left.model, &right.model)
            .then_with(|| left.library.cmp(&right.library));
        let ordering = primary.then(identity);
        if sort.descending {
            ordering.reverse()
        } else {
            ordering
        }
    });
}

/// Order two names exactly as comparing their `to_ascii_lowercase` would,
/// without allocating either one.
///
/// The catalog lower-cased both sides inside the comparator, which allocates
/// two strings per comparison — well over a hundred thousand allocations to
/// order one frame of a corpus-sized catalog. Folding ASCII per byte, rather
/// than reaching for `char::to_lowercase`, keeps the ordering identical to the
/// one this replaces; full Unicode folding would be both slower and a
/// different order.
pub(super) fn case_folded_cmp(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

/// Whether a search term appears anywhere a catalog row reports.
///
/// Matched field by field rather than by joining the card into one haystack:
/// the join allocated a string per model per frame, including the whole
/// parameter map, and was thrown away immediately.
pub(super) fn model_matches_query(
    library: &ModelLibrary,
    model: &DeviceModel,
    query: &str,
) -> bool {
    let contains = |field: &str| contains_query(field, query);
    contains(&model.name)
        || contains(&model.description)
        || contains(model.model_type.display_name())
        || contains(&library.name)
        || model.parameters.keys().any(|parameter| contains(parameter))
}

/// Whether a lower-cased search term appears in one field, ignoring ASCII case.
fn contains_query(field: &str, query: &str) -> bool {
    field.len() >= query.len()
        && field
            .as_bytes()
            .windows(query.len())
            .any(|window| window.eq_ignore_ascii_case(query.as_bytes()))
}

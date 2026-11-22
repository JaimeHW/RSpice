//! The inspected part in the variant chosen in the document bar.
//!
//! The section comes first, above Identity, and only while a variant is
//! chosen. Its rows change what that variant does to the part: fit it, leave
//! it off, put another library cell in its place, or give it another value.
//! Every row below it edits the base design, which every variant shares. Each
//! change is one undo step. An override inside a cell reaches every copy of
//! that cell, and the section says so when there is more than one.

use std::collections::BTreeMap;
use std::sync::Arc;

use super::*;
use crate::state::{
    AssemblyVariant, AssemblyVariantCatalog, ComponentSubstitution, ModelEquivalencePolicy,
    SchematicObjectKey, VariantInheritance, VariantObjectOverride, VariantQualificationState,
};
use crate::workbench::app::dialogs::replace_instance::{
    NO_SUBSTITUTE, VariantSubstitute, opening_variant_substitute, variant_substitutes,
};
use crate::workbench::design_system::property_row_combo_showing;

/// What the section's hover says the rows beneath it edit.
const BELOW_EDITS_THE_BASE: &str =
    "Rows below this section edit the base design, which every variant shares.";

/// A part's state in a variant, as the State row names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartState {
    Fitted,
    DoNotPopulate,
    Substituted,
}

impl PartState {
    const ALL: [Self; 3] = [Self::Fitted, Self::DoNotPopulate, Self::Substituted];

    fn key(self) -> &'static str {
        match self {
            Self::Fitted => "fitted",
            Self::DoNotPopulate => "dnp",
            Self::Substituted => "substituted",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Fitted => "Fitted",
            Self::DoNotPopulate => "Do not populate",
            Self::Substituted => "Substituted",
        }
    }

    fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|state| state.key() == key)
    }

    /// The state an effective override puts a part in.
    #[must_use]
    pub fn of(effective: Option<&VariantObjectOverride>) -> Self {
        match effective {
            Some(VariantObjectOverride::DoNotPopulate { .. }) => Self::DoNotPopulate,
            Some(VariantObjectOverride::Substitute { .. }) => Self::Substituted,
            Some(VariantObjectOverride::Fitted { .. }) | None => Self::Fitted,
        }
    }
}

/// One part as the chosen variant has it: the variant's own override, and
/// what its ancestors do to the part, named by the nearest one that does it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartInVariant {
    pub own: Option<VariantObjectOverride>,
    pub inherited: Option<(String, VariantObjectOverride)>,
}

impl PartInVariant {
    /// Read part `key` in `variant` of `catalog`.
    #[must_use]
    pub fn read(
        catalog: &AssemblyVariantCatalog,
        variant: &AssemblyVariant,
        key: &SchematicObjectKey,
    ) -> Self {
        Self {
            own: variant.definition().overrides.get(key).cloned(),
            inherited: inherited(catalog, variant, key),
        }
    }

    /// What the variant does to the part once its own override and its
    /// ancestors' are layered; `None` is the part as the base design has it.
    #[must_use]
    pub fn effective(&self) -> Option<&VariantObjectOverride> {
        match &self.own {
            Some(own) if own.is_plain_fitted() => None,
            Some(own) => Some(own),
            None => self.inherited.as_ref().map(|(_, change)| change),
        }
    }

    /// The variant's own override that gives the part `effective`: none where
    /// the ancestors already give it that, a plain Fitted where the part is
    /// as the base design has it but an ancestor would change it.
    #[must_use]
    pub fn own_for(
        &self,
        effective: Option<VariantObjectOverride>,
    ) -> Option<VariantObjectOverride> {
        let inherited = self.inherited.as_ref().map(|(_, change)| change);
        match effective {
            None if inherited.is_some() => Some(VariantObjectOverride::Fitted {
                value_override: None,
            }),
            None => None,
            Some(change) if Some(&change) == inherited => None,
            Some(change) => Some(change),
        }
    }

    /// The effective override after the State row picks `state`; a
    /// substitute is `substitute`, which the caller supplies for that state
    /// alone. A value the part has in the variant is kept, except by Do not
    /// populate, which leaves nothing to carry a value.
    #[must_use]
    pub fn with_state(
        &self,
        state: PartState,
        substitute: Option<ComponentSubstitution>,
    ) -> Option<VariantObjectOverride> {
        let value = self
            .effective()
            .and_then(VariantObjectOverride::value_override)
            .map(str::to_owned);
        match state {
            PartState::Fitted => value.map(|value| VariantObjectOverride::Fitted {
                value_override: Some(value),
            }),
            PartState::DoNotPopulate => Some(VariantObjectOverride::DoNotPopulate {
                approval_reference: String::new(),
            }),
            PartState::Substituted => substitute.map(|mut replacement| {
                replacement.value_override = value;
                VariantObjectOverride::Substitute { replacement }
            }),
        }
    }

    /// The effective override after the Value row is set to `value`, or
    /// cleared with `None`. The part keeps its state.
    #[must_use]
    pub fn with_value(&self, value: Option<String>) -> Option<VariantObjectOverride> {
        match self.effective() {
            Some(VariantObjectOverride::Substitute { replacement }) => {
                let mut replacement = replacement.clone();
                replacement.value_override = value;
                Some(VariantObjectOverride::Substitute { replacement })
            }
            Some(VariantObjectOverride::DoNotPopulate { approval_reference }) => {
                Some(VariantObjectOverride::DoNotPopulate {
                    approval_reference: approval_reference.clone(),
                })
            }
            Some(VariantObjectOverride::Fitted { .. }) | None => {
                value.map(|value| VariantObjectOverride::Fitted {
                    value_override: Some(value),
                })
            }
        }
    }
}

/// What the ancestors of `variant` do to part `key`, with the name of the
/// nearest ancestor that does it. A variant copied independently of its
/// parent inherits nothing.
fn inherited(
    catalog: &AssemblyVariantCatalog,
    variant: &AssemblyVariant,
    key: &SchematicObjectKey,
) -> Option<(String, VariantObjectOverride)> {
    if variant.definition().inheritance == VariantInheritance::IndependentReviewedCopy {
        return None;
    }
    let parent = variant.definition().parent.as_ref()?;
    let change = catalog
        .resolve(parent.id)
        .ok()?
        .overrides
        .get(key)
        .cloned()?;
    let mut cursor = catalog.find(parent.id);
    while let Some(ancestor) = cursor {
        if ancestor.definition().overrides.contains_key(key) {
            return Some((ancestor.name().to_owned(), change));
        }
        cursor = ancestor
            .definition()
            .parent
            .as_ref()
            .and_then(|parent| catalog.find(parent.id));
    }
    None
}

/// The substitution a variant records for `substitute`.
fn substitution(substitute: &VariantSubstitute) -> ComponentSubstitution {
    ComponentSubstitution {
        library: substitute.library.clone(),
        cell: substitute.cell.clone(),
        view: substitute.view.clone(),
        value_override: None,
        model_section: None,
        port_equivalence_digest: ComponentSubstitution::port_equivalence_of(
            &substitute.terminal_mapping,
        )
        .ok(),
        qualification: if substitute.carries_everything {
            VariantQualificationState::Current
        } else {
            VariantQualificationState::ReviewRequired
        },
    }
}

/// How a substitution is named in a row and an undo step.
fn substitution_name(replacement: &ComponentSubstitution) -> String {
    format!("{}/{}", replacement.library, replacement.cell)
}

/// Whether a part of this kind can vary at all: a ground or an interface
/// port is part of the circuit's frame, not a fitted part.
fn varies(component: &Component) -> bool {
    !matches!(component.kind, ComponentType::Ground | ComponentType::Port)
}

/// The key a variant stores an override of a part of the open cell view by.
fn part_key(model: &Model, id: u64) -> Option<SchematicObjectKey> {
    SchematicObjectKey::new(
        &model.workspace.content.active_schematic_reference().key(),
        id,
    )
    .ok()
}

/// The variant section for one part.
pub fn variant_section(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, component: &Component) {
    if !varies(component) {
        return;
    }
    let Some(key) = part_key(cx.model, component.id) else {
        return;
    };
    let catalog = cx.model.workspace.content.design_management.variants();
    let Some(variant) = catalog.active() else {
        return;
    };
    let variant_name = variant.name().to_owned();
    let part = PartInVariant::read(catalog, variant, &key);
    let editable = !cx.model.schematic_edit_read_only();
    let state = PartState::of(part.effective());

    section_header_explained(ui, "Variant", &variant_name, BELOW_EDITS_THE_BASE);
    let mut picked = state.key().to_owned();
    let options = PartState::ALL
        .into_iter()
        .map(|state| (state.key().to_owned(), state.label().to_owned()))
        .collect::<Vec<_>>();
    if property_row_combo(
        ui,
        "State",
        ("variant-state", component.id),
        &mut picked,
        &options,
        editable,
    ) && let Some(next) = PartState::from_key(&picked)
        && next != state
    {
        set_state(ui.ctx(), cx, component, &key, &part, &variant_name, next);
        return;
    }
    let copies = crate::workbench::design::surface::master_occurrence_paths(cx.model).len();
    if copies > 1 {
        property_row(
            ui,
            "Applies to",
            &format!(
                "all {copies} copies of {}",
                cx.model.workspace.content.active_view.cell
            ),
        );
    }

    match part.effective().cloned() {
        Some(VariantObjectOverride::Substitute { replacement }) => {
            substitute_row(ui, cx, component, &key, &part, &variant_name, &replacement);
            value_row(ui, cx, component, &key, &part, &variant_name, editable);
            if replacement.qualification != VariantQualificationState::Current {
                property_row_problem(
                    ui,
                    "Qualification",
                    "Review required",
                    Some(Problem::Warning),
                );
            }
        }
        Some(VariantObjectOverride::DoNotPopulate { approval_reference }) => {
            reason_row(
                ui,
                cx,
                component,
                &key,
                &variant_name,
                &approval_reference,
                editable,
            );
            muted_inspector_copy(ui, &format!("Not in {variant_name}\u{2019}s netlist."));
        }
        Some(VariantObjectOverride::Fitted { .. }) | None => {
            value_row(ui, cx, component, &key, &part, &variant_name, editable);
        }
    }
    if part.own.is_none()
        && let Some((ancestor, _)) = &part.inherited
    {
        property_row(ui, "Inherited", &format!("from {ancestor}"));
    }
}

/// The State row for a selected set: Fitted or Do not populate for every
/// part in it at once. A set whose parts differ reads "—".
pub fn variant_set_rows(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, ids: &[u64]) {
    let catalog = cx.model.workspace.content.design_management.variants();
    let Some(variant) = catalog.active() else {
        return;
    };
    let variant_name = variant.name().to_owned();
    let parts = cx
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| ids.contains(&component.id) && varies(component))
        .filter_map(|component| {
            let key = part_key(cx.model, component.id)?;
            let part = PartInVariant::read(catalog, variant, &key);
            Some((key, part))
        })
        .collect::<Vec<_>>();
    if parts.is_empty() {
        return;
    }
    let states = parts
        .iter()
        .map(|(_, part)| PartState::of(part.effective()))
        .collect::<Vec<_>>();
    let shared = states
        .first()
        .copied()
        .filter(|first| states.iter().all(|state| state == first));
    section_header_explained(ui, "Variant", &variant_name, BELOW_EDITS_THE_BASE);
    let options = [PartState::Fitted, PartState::DoNotPopulate]
        .into_iter()
        .map(|state| (state.key().to_owned(), state.label().to_owned()))
        .collect::<Vec<_>>();
    let mut picked = shared.map_or_else(String::new, |state| state.key().to_owned());
    let shown = shared.map_or("\u{2014}", PartState::label);
    let editable = !cx.model.schematic_edit_read_only();
    if !property_row_combo_showing(
        ui,
        "State",
        ("variant-set-state", ids.len()),
        &mut picked,
        &options,
        shown,
        editable,
    ) {
        return;
    }
    let Some(next) = PartState::from_key(&picked) else {
        return;
    };
    let edits = parts
        .iter()
        .filter(|(_, part)| PartState::of(part.effective()) != next)
        .map(|(key, part)| (key.clone(), part.own_for(part.with_state(next, None))))
        .collect::<BTreeMap<_, _>>();
    if edits.is_empty() {
        return;
    }
    let count = edits.len();
    let description = match next {
        PartState::DoNotPopulate => {
            format!("Mark {count} parts do not populate in {variant_name}")
        }
        _ => format!("Fit {count} parts in {variant_name}"),
    };
    commit(cx, &description, edits);
}

/// Put the part in `next`, as the State row asked.
fn set_state(
    ctx: &egui::Context,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    key: &SchematicObjectKey,
    part: &PartInVariant,
    variant_name: &str,
    next: PartState,
) {
    let name = &component.name;
    let (effective, description) = match next {
        PartState::Fitted => (
            part.with_state(next, None),
            format!("Fit {name} in {variant_name}"),
        ),
        PartState::DoNotPopulate => (
            part.with_state(next, None),
            format!("Mark {name} do not populate in {variant_name}"),
        ),
        PartState::Substituted => {
            let substitutes = substitutes(ctx, cx, component);
            let Some(pick) =
                opening_variant_substitute(&cx.state.property_registry, cx.model, &substitutes)
            else {
                cx.wb.prompt(NO_SUBSTITUTE);
                return;
            };
            let replacement = substitution(&substitutes[pick]);
            let description = format!(
                "Substitute {} for {name} in {variant_name}",
                substitution_name(&replacement)
            );
            (part.with_state(next, Some(replacement)), description)
        }
    };
    commit_one(cx, key, part, effective, &description);
}

/// The Substitute row: the master standing in for the part, picked from
/// the masters Replace instance would offer.
fn substitute_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    key: &SchematicObjectKey,
    part: &PartInVariant,
    variant_name: &str,
    current: &ComponentSubstitution,
) {
    let editable = !cx.model.schematic_edit_read_only();
    let substitutes = if editable {
        substitutes(ui.ctx(), cx, component)
    } else {
        Arc::new(Vec::new())
    };
    let current_key = substitute_key(&current.library, &current.cell, &current.view);
    let mut options = substitutes
        .iter()
        .map(|substitute| {
            (
                substitute_key(&substitute.library, &substitute.cell, &substitute.view),
                substitute.display.clone(),
            )
        })
        .collect::<Vec<_>>();
    if !options.iter().any(|(key, _)| *key == current_key) {
        options.insert(0, (current_key.clone(), substitution_name(current)));
    }
    let mut picked = current_key.clone();
    if !property_row_combo(
        ui,
        "Substitute",
        ("variant-substitute", component.id),
        &mut picked,
        &options,
        editable,
    ) || picked == current_key
    {
        return;
    }
    let Some(substitute) = substitutes.iter().find(|substitute| {
        substitute_key(&substitute.library, &substitute.cell, &substitute.view) == picked
    }) else {
        return;
    };
    let mut replacement = substitution(substitute);
    replacement
        .value_override
        .clone_from(&current.value_override);
    let description = format!(
        "Substitute {} for {} in {variant_name}",
        substitution_name(&replacement),
        component.name
    );
    commit_one(
        cx,
        key,
        part,
        Some(VariantObjectOverride::Substitute { replacement }),
        &description,
    );
}

fn substitute_key(library: &str, cell: &str, view: &str) -> String {
    format!("{library}/{cell}/{view}")
}

/// The masters the part can be substituted with, ranked, kept while the
/// drawing and the libraries stand still. Under a policy that demands
/// qualified replacements, only masters every pin and parameter carries to.
fn substitutes(
    ctx: &egui::Context,
    cx: &Cx<'_, DesignFeature>,
    component: &Component,
) -> Arc<Vec<VariantSubstitute>> {
    let model = &*cx.model;
    let stamp = (
        component.id,
        model.schematic.content_version(),
        crate::workbench::app::symbol_context_revision(model),
        model.design_execution_epoch,
    );
    let id = egui::Id::new(("variant-substitutes", component.id));
    if let Some((kept, list)) =
        ctx.data(|data| data.get_temp::<((u64, u64, u64, u64), Arc<Vec<VariantSubstitute>>)>(id))
        && kept == stamp
    {
        return list;
    }
    let qualified_only = model
        .workspace
        .content
        .design_management
        .variants()
        .settings()
        .model_equivalence
        == ModelEquivalencePolicy::RequireQualifiedReplacement;
    let list = Arc::new(
        variant_substitutes(&cx.state.property_registry, model)
            .unwrap_or_default()
            .into_iter()
            .filter(|substitute| !qualified_only || substitute.carries_everything)
            .collect::<Vec<_>>(),
    );
    ctx.data_mut(|data| data.insert_temp(id, (stamp, Arc::clone(&list))));
    list
}

/// The Value row: the part's value in this variant, which leaves its state
/// alone. Empty is the base design's value, which the hint states.
fn value_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    key: &SchematicObjectKey,
    part: &PartInVariant,
    variant_name: &str,
    editable: bool,
) {
    let current = part
        .effective()
        .and_then(VariantObjectOverride::value_override)
        .unwrap_or_default()
        .to_owned();
    let hint = if component.value.is_empty() {
        "as base".to_owned()
    } else {
        format!("as base: {}", component.value)
    };
    let Some(text) = edited_text(
        ui,
        ("variant-value", component.id),
        "Value",
        &current,
        &hint,
        editable,
    ) else {
        return;
    };
    let value = (!text.is_empty()).then_some(text);
    let description = match &value {
        Some(value) => format!("Set {} value to {value} in {variant_name}", component.name),
        None => format!("Clear {} value in {variant_name}", component.name),
    };
    commit_one(cx, key, part, part.with_value(value), &description);
}

/// The Reason row of a part left off: why, if anyone said. It may stay
/// empty.
fn reason_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    key: &SchematicObjectKey,
    variant_name: &str,
    current: &str,
    editable: bool,
) {
    let Some(text) = edited_text(
        ui,
        ("variant-reason", component.id),
        "Reason",
        current,
        "optional",
        editable,
    ) else {
        return;
    };
    let description = format!(
        "Set why {} is not populated in {variant_name}",
        component.name
    );
    let change = VariantObjectOverride::DoNotPopulate {
        approval_reference: text,
    };
    let edits = BTreeMap::from([(key.clone(), Some(change))]);
    commit(cx, &description, edits);
}

/// A text row that commits when it lets go of the keyboard with Enter or a
/// click elsewhere, and answers its trimmed text when that differs from
/// `current`. Escape puts `current` back.
fn edited_text(
    ui: &mut Ui,
    salt: (&str, u64),
    label: &str,
    current: &str,
    hint: &str,
    editable: bool,
) -> Option<String> {
    let id = ui.id().with(salt);
    let mut text = ui
        .data(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| current.to_owned());
    let response = ui
        .add_enabled_ui(editable, |ui| {
            property_row_input_with_hint(ui, label, &mut text, hint, false)
        })
        .inner;
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return None;
    }
    ui.data_mut(|data| data.remove::<String>(id));
    let escaped = ui.input(|input| input.key_pressed(egui::Key::Escape));
    let text = text.trim().to_owned();
    (response.lost_focus() && !escaped && text != current).then_some(text)
}

/// Give the part `effective` in the chosen variant, as one undo step.
fn commit_one(
    cx: &mut Cx<'_, DesignFeature>,
    key: &SchematicObjectKey,
    part: &PartInVariant,
    effective: Option<VariantObjectOverride>,
    description: &str,
) {
    let own = part.own_for(effective);
    if own == part.own {
        return;
    }
    commit(cx, description, BTreeMap::from([(key.clone(), own)]));
}

/// Publish `edits` to the chosen variant, as one undo step; a run prepared
/// before it is not reused.
fn commit(
    cx: &mut Cx<'_, DesignFeature>,
    description: &str,
    edits: BTreeMap<SchematicObjectKey, Option<VariantObjectOverride>>,
) {
    match cx.model.set_variant_overrides(description, edits) {
        Ok(()) => rspice_app_runs::workbench::preflight::invalidate(
            &mut cx.runs.preflight,
            &mut *cx.handles.run_controller,
        ),
        Err(error) => cx.wb.prompt(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_not_populate() -> VariantObjectOverride {
        VariantObjectOverride::DoNotPopulate {
            approval_reference: String::new(),
        }
    }

    fn substitute(value: Option<&str>) -> VariantObjectOverride {
        VariantObjectOverride::Substitute {
            replacement: ComponentSubstitution {
                library: "qualified".to_owned(),
                cell: "resistor_aecq".to_owned(),
                view: "symbol".to_owned(),
                value_override: value.map(str::to_owned),
                model_section: None,
                port_equivalence_digest: None,
                qualification: VariantQualificationState::Current,
            },
        }
    }

    fn fitted(value: Option<&str>) -> VariantObjectOverride {
        VariantObjectOverride::Fitted {
            value_override: value.map(str::to_owned),
        }
    }

    fn part(
        own: Option<VariantObjectOverride>,
        inherited: Option<VariantObjectOverride>,
    ) -> PartInVariant {
        PartInVariant {
            own,
            inherited: inherited.map(|change| ("Industrial".to_owned(), change)),
        }
    }

    #[test]
    fn fitting_a_part_the_parent_leaves_off_stores_an_explicit_fitted() {
        let part = part(None, Some(do_not_populate()));
        assert_eq!(PartState::of(part.effective()), PartState::DoNotPopulate);
        let own = part.own_for(part.with_state(PartState::Fitted, None));
        assert_eq!(own, Some(fitted(None)));
    }

    #[test]
    fn fitting_a_part_nothing_above_changes_removes_the_override() {
        let part = part(Some(do_not_populate()), None);
        assert_eq!(part.own_for(part.with_state(PartState::Fitted, None)), None);
    }

    #[test]
    fn a_value_alone_keeps_a_primitive_fitted() {
        let part = part(None, None);
        let own = part.own_for(part.with_value(Some("2k".to_owned())));
        assert_eq!(own, Some(fitted(Some("2k"))));
        assert_eq!(PartState::of(own.as_ref()), PartState::Fitted);
    }

    #[test]
    fn a_value_on_an_inherited_substitute_keeps_the_substitute() {
        let part = part(None, Some(substitute(None)));
        let own = part.own_for(part.with_value(Some("2k".to_owned())));
        assert_eq!(own, Some(substitute(Some("2k"))));
    }

    #[test]
    fn clearing_the_value_of_a_restored_part_keeps_it_restored() {
        let part = part(Some(fitted(Some("2k"))), Some(do_not_populate()));
        assert_eq!(part.own_for(part.with_value(None)), Some(fitted(None)));
    }

    #[test]
    fn substituting_keeps_the_value_and_leaving_off_drops_it() {
        let part = part(Some(fitted(Some("2k"))), None);
        let replacement = match substitute(None) {
            VariantObjectOverride::Substitute { replacement } => replacement,
            _ => unreachable!(),
        };
        assert_eq!(
            part.with_state(PartState::Substituted, Some(replacement)),
            Some(substitute(Some("2k")))
        );
        assert_eq!(
            part.with_state(PartState::DoNotPopulate, None),
            Some(do_not_populate())
        );
    }
}

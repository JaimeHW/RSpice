//! Choosing the device model a schematic instance runs with.
//!
//! The left pane lists every model a loaded library offers that the instance
//! can take, under the library it comes from, with the card's level and key
//! figures on each row so two candidates can be told apart without opening
//! either. A model the instance cannot take is not listed at all: the binding
//! gate would refuse it, and a picker that offers what it will refuse spends
//! the reader's attention on rows nobody can choose. A model the instance
//! could take but cannot be bound from *this* library — another library gives
//! the same name to the project's one flat namespace — stays listed and inert,
//! with the reason on its hover, because that is a decision the reader can go
//! and make rather than a fact about the device.
//!
//! The right pane is the picked model's sheet: what it is, where it comes
//! from, and every parameter its card sets.
//!
//! A binned family — `nch.1` … `nch.48`, each card valid over its own L×W
//! window — is one row named `nch`, and binding it binds that name, as a
//! Spectre or HSPICE deck binds a foundry device: the simulator chooses the
//! bin from the instance's L and W. A row per bin invited pinning the instance
//! to one card, which then runs at every size. The family's sheet lays out
//! its coverage, bin by bin, and the parameters of the bin the instance's
//! geometry selects.
//!
//! Opened without an instance, the same surface browses every loaded model.

use std::cmp::Ordering;
use std::collections::HashMap;

use egui::{Align, Context, Key, Layout, Modifiers, Rect, Sense, Ui, vec2};

use crate::quantity::{EngineeringPrecision, engineering};
use crate::state::ComponentType;
use crate::state::model_library::{
    BinFamily, BindableModel, DeviceModel, ModelConsumerScope, ModelLevel, ModelLibrary,
    ModelLibraryManager, ModelType, validate_component_model_compatibility,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize};

// =============================================================================
// State
// =============================================================================

/// One session of the model browser: what it is choosing for, the filter, and
/// the pick.
#[derive(Debug, Clone, Default)]
pub struct ModelBrowserState {
    /// Whether the dialog is open.
    pub open: bool,
    /// The instance a model is being chosen for. `None` browses every loaded
    /// model and binds nothing.
    target: Option<ModelTarget>,
    /// What the list is narrowed to.
    filter: String,
    /// The picked model.
    pick: Option<ModelKey>,
    /// Whether the pick has been taken from the instance's current model yet.
    seeded: bool,
    /// Bring the pick into view on the next frame: on opening, so the model in
    /// use is in sight, and after the arrow keys move it.
    reveal_pick: bool,
}

/// One model, named by the library that provides it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ModelKey {
    library: String,
    model: String,
}

/// The instance a model is being chosen for.
#[derive(Debug, Clone)]
struct ModelTarget {
    /// `D1`.
    reference: String,
    kind: ComponentType,
    /// The model the instance names now, with the library it was bound from
    /// when it was bound from one.
    current: Option<(String, Option<String>)>,
    geometry: InstanceGeometry,
}

/// The instance's drawn size, in metres: what selects the bin of a binned
/// family. A dimension the dialog cannot evaluate — an expression of design
/// variables — is `None`, and selects no bin.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct InstanceGeometry {
    pub length: Option<f64>,
    pub width: Option<f64>,
}

impl InstanceGeometry {
    fn known(self) -> Option<(f64, f64)> {
        self.length.zip(self.width)
    }
}

impl ModelBrowserState {
    /// Open the browser to choose the model one instance runs with.
    pub fn open_for_instance(
        &mut self,
        reference: &str,
        kind: ComponentType,
        model: Option<String>,
        library: Option<String>,
        geometry: InstanceGeometry,
    ) {
        *self = Self {
            open: true,
            target: Some(ModelTarget {
                reference: reference.to_owned(),
                kind,
                current: model.map(|model| (model, library)),
                geometry,
            }),
            ..Self::default()
        };
    }

    /// Close the dialog.
    pub fn close(&mut self) {
        self.open = false;
    }
}

/// What the browser decided this frame.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ModelBrowserResult {
    /// Still open, or closed without a decision.
    #[default]
    None,
    /// Dismissed without binding anything.
    Cancelled,
    /// Bind this model, provided by this library.
    Selected {
        /// Library name.
        library: String,
        /// Model name.
        model: String,
    },
}

// =============================================================================
// Catalog
// =============================================================================

/// What a row binds: one card, or a binned family whose bin the simulator
/// chooses from the instance's geometry.
#[derive(Debug, Clone)]
enum Entry<'a> {
    Card(&'a DeviceModel),
    Family(BinFamily<'a>),
}

impl<'a> Entry<'a> {
    /// The name binding the row gives the instance.
    fn name(&self) -> &'a str {
        match *self {
            Self::Card(model) => &model.name,
            Self::Family(ref family) => family.name,
        }
    }

    /// The card that speaks for the row: the card itself, or the bin this
    /// geometry selects — the first bin when it selects none.
    fn card(&self, geometry: InstanceGeometry) -> &'a DeviceModel {
        match *self {
            Self::Card(model) => model,
            Self::Family(ref family) => family
                .selected(geometry.length, geometry.width)
                // A family holds at least one bin.
                .unwrap_or(family.bins[0]),
        }
    }

    /// Every card the row can run.
    fn cards(&self) -> &[&'a DeviceModel] {
        match self {
            Self::Card(model) => std::slice::from_ref(model),
            Self::Family(family) => &family.bins,
        }
    }
}

/// What `name` binds in `library`: its card, or the family it names.
fn entry<'a>(library: &'a ModelLibrary, name: &str) -> Option<Entry<'a>> {
    match library.get_model(name) {
        Some(model) => Some(Entry::Card(model)),
        None => library.bin_family(name).map(Entry::Family),
    }
}

/// One listed model.
struct Row<'a> {
    library: &'a ModelLibrary,
    entry: Entry<'a>,
    /// The dim line under the name, as whole facts in the order a narrow row
    /// gives them up.
    meta: Vec<String>,
    /// Why this copy cannot be bound to the instance.
    refusal: Option<Refusal>,
    /// Whether the instance runs this copy now.
    current: bool,
}

impl Row<'_> {
    fn key(&self) -> ModelKey {
        ModelKey {
            library: self.library.name.clone(),
            model: self.entry.name().to_owned(),
        }
    }

    fn is(&self, key: &ModelKey) -> bool {
        self.library.name == key.library && self.entry.name() == key.model
    }
}

/// Why a listed copy cannot be bound: the row's one-word tag and the sentence
/// on its hover.
struct Refusal {
    tag: &'static str,
    reason: String,
}

/// One library's run of rows.
struct Group<'a> {
    library: &'a str,
    rows: Vec<Row<'a>>,
}

/// Every model the browser lists, grouped by library and narrowed by the
/// filter.
///
/// An empty group is absent rather than headed by nothing.
fn catalog<'a>(
    manager: &'a ModelLibraryManager,
    target: Option<&ModelTarget>,
    filter: &str,
) -> Vec<Group<'a>> {
    let terms: Vec<String> = filter.split_whitespace().map(str::to_lowercase).collect();
    let libraries = manager.libraries_sorted();
    let entries: Vec<(&ModelLibrary, Vec<Entry<'a>>)> = libraries
        .into_iter()
        .map(|library| (library, entries_of(library)))
        .collect();
    // How many libraries define each name, as a card or as a family. Only a
    // name defined twice can be shadowed, so only those rows pay for the
    // project's provider decision.
    let mut definitions: HashMap<String, usize> = HashMap::new();
    for (library, library_entries) in &entries {
        let names = library.models.values().map(|model| model.name.as_str());
        let families = library_entries.iter().filter_map(|entry| match entry {
            Entry::Family(family) => Some(family.name),
            Entry::Card(_) => None,
        });
        for name in names.chain(families) {
            *definitions.entry(name.to_ascii_lowercase()).or_default() += 1;
        }
    }
    entries
        .into_iter()
        .filter_map(|(library, library_entries)| {
            let mut rows: Vec<Row<'a>> = library_entries
                .into_iter()
                .filter(|entry| {
                    target.is_none_or(|target| {
                        entry.cards().iter().all(|model| {
                            validate_component_model_compatibility(target.kind, model).is_ok()
                        })
                    })
                })
                .filter(|entry| matches(library, entry, &terms))
                .map(|entry| {
                    let name = entry.name();
                    let shared = definitions
                        .get(&name.to_ascii_lowercase())
                        .is_some_and(|count| *count > 1);
                    let refusal = (target.is_some() && shared)
                        .then(|| provider_refusal(manager, library, name))
                        .flatten();
                    let current = target
                        .is_some_and(|target| runs_now(target, library, name, refusal.is_none()));
                    Row {
                        library,
                        meta: meta_line(&entry, target.is_none()),
                        entry,
                        refusal,
                        current,
                    }
                })
                .collect();
            rows.sort_by(|left, right| natural_cmp(left.entry.name(), right.entry.name()));
            (!rows.is_empty()).then_some(Group {
                library: &library.name,
                rows,
            })
        })
        .collect()
}

/// What a library offers to bind: each family it provides, and each card
/// that is not a bin of one.
fn entries_of(library: &ModelLibrary) -> Vec<Entry<'_>> {
    library
        .bindable_models()
        .into_iter()
        .map(|model| match model {
            BindableModel::Card(card) => Entry::Card(card),
            BindableModel::Family(family) => Entry::Family(family),
        })
        .collect()
}

/// Whether every filter term appears in something the row says. A family is
/// found by any of its bins' names as well as by its own.
fn matches(library: &ModelLibrary, entry: &Entry<'_>, terms: &[String]) -> bool {
    if terms.is_empty() {
        return true;
    }
    let model = entry.card(InstanceGeometry::default());
    let mut haystack = format!(
        "{} {} {} {} {} {}",
        entry.name(),
        model.model_type.display_name(),
        level_label(model).unwrap_or_default(),
        model.spice_type.as_deref().unwrap_or_default(),
        library.name,
        model.description,
    );
    if let Entry::Family(family) = entry {
        for bin in &family.bins {
            haystack.push(' ');
            haystack.push_str(&bin.name);
        }
    }
    let haystack = haystack.to_lowercase();
    terms.iter().all(|term| haystack.contains(term.as_str()))
}

/// Why this library's copy of a name defined in several cannot be bound.
///
/// The project's provider decision is asked rather than restated: Properties,
/// catalog binding and the sealed run plan all read the same answer, and a
/// picker that guessed differently would offer a row the commit then refuses.
fn provider_refusal(
    manager: &ModelLibraryManager,
    library: &ModelLibrary,
    name: &str,
) -> Option<Refusal> {
    match manager.effective_definition_provider(ModelConsumerScope::PrimitiveModel, name) {
        Ok(Some(provider)) if provider.library.eq_ignore_ascii_case(&library.name) => None,
        Ok(Some(provider)) => Some(Refusal {
            tag: "shadowed",
            reason: format!(
                "This project runs {name} from {}. Choose that library as its provider in Models to bind this copy.",
                provider.library
            ),
        }),
        Ok(None) => None,
        Err(_) => {
            let libraries: Vec<String> = manager
                .definition_providers(ModelConsumerScope::PrimitiveModel, name)
                .into_iter()
                .map(|provider| provider.library)
                .collect();
            Some(Refusal {
                tag: "unresolved",
                reason: format!(
                    "{name} is defined in {}, and the project has not chosen which one runs. Choose its provider in Models.",
                    libraries.join(" and ")
                ),
            })
        }
    }
}

/// Whether the instance runs this copy now.
///
/// An instance bound from a library names it; one whose model was typed
/// names only the model, and runs whichever copy the project provides.
fn runs_now(target: &ModelTarget, library: &ModelLibrary, name: &str, provided: bool) -> bool {
    let Some((current, bound_from)) = &target.current else {
        return false;
    };
    current.eq_ignore_ascii_case(name)
        && bound_from.as_deref().map_or(provided, |bound_from| {
            bound_from.eq_ignore_ascii_case(&library.name)
        })
}

/// Whether the instance is pinned to one bin of this row's family: the row a
/// rebinding replaces that bin with, and where the browser opens.
fn pins_a_bin_of(target: &ModelTarget, row: &Row<'_>) -> bool {
    let (Some((current, bound_from)), Entry::Family(family)) = (&target.current, &row.entry) else {
        return false;
    };
    family
        .bins
        .iter()
        .any(|bin| bin.name.eq_ignore_ascii_case(current))
        && bound_from
            .as_deref()
            .is_none_or(|bound_from| bound_from.eq_ignore_ascii_case(&row.library.name))
}

/// Name order that reads bins and part numbers the way a designer counts:
/// `nch.2` before `nch.10`.
fn natural_cmp(left: &str, right: &str) -> Ordering {
    let mut a = left.chars().peekable();
    let mut b = right.chars().peekable();
    loop {
        match (a.peek().copied(), b.peek().copied()) {
            (None, None) => return left.cmp(right),
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
                let run = |chars: &mut std::iter::Peekable<std::str::Chars<'_>>| {
                    let mut digits = String::new();
                    while let Some(digit) = chars.next_if(char::is_ascii_digit) {
                        digits.push(digit);
                    }
                    digits
                };
                let (x, y) = (run(&mut a), run(&mut b));
                let (x, y) = (x.trim_start_matches('0'), y.trim_start_matches('0'));
                let order = x.len().cmp(&y.len()).then_with(|| x.cmp(y));
                if order != Ordering::Equal {
                    return order;
                }
            }
            (Some(x), Some(y)) => {
                let order = x.to_ascii_lowercase().cmp(&y.to_ascii_lowercase());
                if order != Ordering::Equal {
                    return order;
                }
                a.next();
                b.next();
            }
        }
    }
}

// =============================================================================
// What a card says
// =============================================================================

/// The card's model level as the catalog names it, with its version.
///
/// Absent when the card declares neither a family nor a level: SPICE would
/// run its default level, but the card does not say so and neither does this.
fn level_label(model: &DeviceModel) -> Option<String> {
    let level = match model.level {
        ModelLevel::Unknown => format!("Level {}", model.spice_level?),
        level => level.display_name().to_owned(),
    };
    Some(match model.model_version {
        Some(version) => format!("{level} {version}"),
        None => level,
    })
}

/// The parameters that tell two cards of one family apart, in the order a
/// designer reads them. Card keys are lowercase; the sheet spells them upper.
fn key_names(model_type: ModelType) -> &'static [&'static str] {
    match model_type {
        ModelType::Diode | ModelType::Esd => &["is", "n", "bv", "rs", "cjo", "cj0", "tt"],
        ModelType::Varactor => &["cjo", "cj0", "vj", "m", "is"],
        ModelType::Npn | ModelType::Pnp => &["bf", "is", "vaf", "br", "tf", "cje", "cjc"],
        ModelType::Njfet | ModelType::Pjfet => &["vto", "beta", "lambda", "is", "cgs", "cgd"],
        ModelType::Nmesfet | ModelType::Pmesfet => &["vto", "beta", "alpha", "b", "lambda"],
        ModelType::Nmos | ModelType::Pmos => &["vth0", "vto", "kp", "u0", "toxe", "tox", "lambda"],
        ModelType::NVdmos | ModelType::PVdmos => &["vto", "kp", "rd", "rs", "rg"],
        ModelType::Resistor => &["rsh", "tc1", "tc2"],
        ModelType::Capacitor => &["cj", "cjsw", "cox"],
        ModelType::Inductor | ModelType::Rf | ModelType::Other => &[],
    }
}

/// Card keys the sheet's identity line already states.
const STATED_ELSEWHERE: [&str; 2] = ["level", "version"];

/// One parameter as the sheet shows it.
struct Parameter {
    /// `IS`.
    name: String,
    /// `10f`.
    value: String,
    /// The value exactly as the card holds it, for the hover.
    exact: String,
}

/// A numeric card value in engineering notation, or as an exponent past the
/// ladder's top rung: a doping of `1.7e17` is not `170000T`.
fn spelled(value: f64) -> String {
    if value.abs() >= 1e15 {
        return format!("{value:e}");
    }
    engineering::format_engineering_display_with(value, EngineeringPrecision::Adaptive)
}

/// Every parameter the card sets: the family's key figures first, then the
/// rest in name order, then the non-numeric ones.
fn parameters(model: &DeviceModel) -> Vec<Parameter> {
    let keys = key_names(model.model_type);
    let mut numeric: Vec<(&String, f64)> = model
        .parameters
        .iter()
        .filter(|(name, _)| !STATED_ELSEWHERE.contains(&name.as_str()))
        .map(|(name, value)| (name, *value))
        .collect();
    numeric.sort_by(|(left, _), (right, _)| {
        let rank = |name: &str| {
            keys.iter()
                .position(|key| *key == name)
                .unwrap_or(keys.len())
        };
        rank(left.as_str())
            .cmp(&rank(right.as_str()))
            .then_with(|| natural_cmp(left, right))
    });
    let mut text: Vec<(&String, &String)> = model
        .string_parameters
        .iter()
        .filter(|(name, _)| !STATED_ELSEWHERE.contains(&name.as_str()))
        .collect();
    text.sort_by(|(left, _), (right, _)| natural_cmp(left, right));
    numeric
        .into_iter()
        .map(|(name, value)| Parameter {
            name: name.to_ascii_uppercase(),
            value: spelled(value),
            exact: format!("{value:e}"),
        })
        .chain(text.into_iter().map(|(name, value)| Parameter {
            name: name.to_ascii_uppercase(),
            value: value.clone(),
            exact: value.clone(),
        }))
        .collect()
}

/// A row's meta line: the device family when the list mixes them, the level,
/// and up to three key figures — or, for a binned family, how many bins it
/// holds and the L and W they cover together.
fn meta_line(entry: &Entry<'_>, with_family: bool) -> Vec<String> {
    let model = entry.card(InstanceGeometry::default());
    let mut parts = Vec::new();
    if with_family {
        parts.push(model.model_type.display_name().to_owned());
    }
    parts.extend(level_label(model));
    if let Entry::Family(family) = entry {
        parts.push(counted(family.bins.len(), "bin", "bins"));
        for (axis, (min, max)) in ["L", "W"].into_iter().zip(coverage(family)) {
            if min.is_some() || max.is_some() {
                parts.push(format!("{axis} {}", window_label(min, max)));
            }
        }
        return parts;
    }
    parts.extend(
        key_names(model.model_type)
            .iter()
            .filter_map(|key| {
                model
                    .parameters
                    .get(*key)
                    .map(|value| format!("{} {}", key.to_ascii_uppercase(), spelled(*value)))
            })
            .take(3),
    );
    parts
}

/// Format a device length in SI prefixes.
///
/// Not the shared engineering formatter. This one is length-only: its ladder
/// stops at nanometres and each rung carries the unit — `mm`, `µm`, `nm` —
/// because a process dimension is read against its unit or not at all.
/// [`crate::quantity::format_engineering_value`] emits a bare prefix over
/// every decade, which is what a unitless quantity wants and what a geometry
/// column must not print.
fn format_si_value(value: f64) -> String {
    let abs_value = value.abs();

    if abs_value == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs_value >= 1e-3 {
        (value * 1e3, "mm")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "µm")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "nm")
    } else {
        (value * 1e12, "pm")
    };

    if (scaled.round() - scaled).abs() < 1e-9 {
        format!("{:.0}{}", scaled.round(), suffix)
    } else {
        format!("{:.2}{}", scaled, suffix)
    }
}

/// A bin's window on one axis, short enough for a table cell: the unit once
/// when both ends share it, `100–500nm`, and a dash for an axis it leaves
/// open.
fn window_label(min: Option<f64>, max: Option<f64>) -> String {
    // Where the unit `format_si_value` writes after the digits starts.
    let unit = |text: &str| text.find(char::is_alphabetic).unwrap_or(text.len());
    match (min, max) {
        (Some(min), Some(max)) => {
            let (low, high) = (format_si_value(min), format_si_value(max));
            let (low_value, low_unit) = low.split_at(unit(&low));
            let (_, high_unit) = high.split_at(unit(&high));
            if !low_unit.is_empty() && low_unit == high_unit {
                format!("{low_value}\u{2013}{high}")
            } else {
                format!("{low}\u{2013}{high}")
            }
        }
        (Some(min), None) => format!("min {}", format_si_value(min)),
        (None, Some(max)) => format!("max {}", format_si_value(max)),
        (None, None) => "\u{2014}".to_owned(),
    }
}

/// The L and W windows a family's bins cover together, from the smallest
/// bound to the largest. An end some bin leaves open is open for the family.
fn coverage(family: &BinFamily<'_>) -> [(Option<f64>, Option<f64>); 2] {
    let bound = |side: fn(&DeviceModel) -> Option<f64>, widest: fn(f64, f64) -> f64| {
        family
            .bins
            .iter()
            .map(|bin| side(bin))
            .collect::<Option<Vec<f64>>>()
            .and_then(|bounds| bounds.into_iter().reduce(widest))
    };
    [
        (
            bound(|bin| bin.l_min, f64::min),
            bound(|bin| bin.l_max, f64::max),
        ),
        (
            bound(|bin| bin.w_min, f64::min),
            bound(|bin| bin.w_max, f64::max),
        ),
    ]
}

/// A geometry window as one fact, open at either end when the card bounds only
/// one side.
fn range_label(min: Option<f64>, max: Option<f64>) -> Option<String> {
    match (min, max) {
        (Some(min), Some(max)) => Some(format!(
            "{} \u{2013} {}",
            format_si_value(min),
            format_si_value(max)
        )),
        // Words rather than inequality signs: the bundled Plex faces carry
        // neither, and a tofu box is the one thing a bound must not read as.
        (Some(min), None) => Some(format!("min {}", format_si_value(min))),
        (None, Some(max)) => Some(format!("max {}", format_si_value(max))),
        (None, None) => None,
    }
}

// =============================================================================
// Rendering
// =============================================================================

/// What the modal is for, for a reader who reaches it through the
/// accessibility tree rather than through the schematic.
const PURPOSE_BIND: &str =
    "Choose the device model this instance runs with from the loaded model libraries";
const PURPOSE_BROWSE: &str = "Browse the loaded model libraries and read each model's parameters";

/// The list pane, and the width it falls back to when the dialog has been
/// narrowed to the viewport.
const LIST_WIDTH: f32 = 300.0;
const NARROW_LIST_WIDTH: f32 = 260.0;
const NARROW_BODY: f32 = 840.0;
/// Below this body width the sheet gives up its pane and the list takes the
/// body: a sheet narrower than two parameter columns says less than the row's
/// own key figures.
const SPLIT_MIN: f32 = 600.0;
/// The body's bounds. Below the minimum a two-pane dialog reads as a strip of
/// controls; above the maximum the catalog has outgrown a modal and scrolls.
const MIN_BODY: f32 = 300.0;
const MAX_BODY: f32 = 460.0;

/// The inset each pane keeps from the dialog edge and from the hairline: the
/// kit's own body inset, so the filter, the title and the footer's sentence
/// share one left edge.
const PANE_INSET: f32 = 12.0;
/// The air above the filter, and between it and the first row.
const FILTER_TOP: f32 = 14.0;
const FILTER_GAP: f32 = 10.0;
/// One model row, the label above a library's run of them, and the gap
/// between two libraries' runs.
const ROW_HEIGHT: f32 = 36.0;
const GROUP_LABEL_HEIGHT: f32 = 24.0;
const GROUP_GAP: f32 = 6.0;
/// One line of state.
const NOTE_HEIGHT: f32 = 18.0;
/// The sheet's kind line under the model's name, and the air between its
/// bands.
const KIND_HEIGHT: f32 = 16.0;
const DESCRIPTION_GAP: f32 = 4.0;
const FACTS_GAP: f32 = 4.0;
const PARAMETERS_GAP: f32 = 10.0;
const CAPTION_HEIGHT: f32 = 16.0;
const RULE_GAP: f32 = 4.0;
/// The sheet's fact rows, on the parameter grid's pitch, and the air between
/// the widest label and the column every value starts in.
const FACT_PITCH: f32 = PARAMETER_PITCH;
const FACT_LABEL_GAP: f32 = 12.0;
/// Every label [`sheet_facts`] can state. The value column is measured over
/// all of them, so it holds still while the reader steps from card to card.
const FACT_LABELS: [&str; 7] = [
    "Library", "Source", "Section", "Corners", "Length", "Width", "Supply",
];
/// The parameter grid: row pitch, the narrowest column, and the gap between
/// two columns.
const PARAMETER_PITCH: f32 = 20.0;
const PARAMETER_CELL_MIN: f32 = 150.0;
const PARAMETER_GUTTER: f32 = 24.0;

/// Render the model browser.
///
/// Returns [`ModelBrowserResult::Selected`] when the reader binds a model.
pub fn render_model_browser(
    ctx: &Context,
    state: &mut ModelBrowserState,
    manager: &ModelLibraryManager,
) -> ModelBrowserResult {
    if !state.open {
        return ModelBrowserResult::None;
    }
    let target = state.target.clone();
    let groups = catalog(manager, target.as_ref(), &state.filter);
    if !state.seeded {
        state.seeded = true;
        state.reveal_pick = true;
        if state.pick.is_none() {
            // The model in use, or — for an instance pinned to one bin — the
            // family that bin belongs to, which Select then binds instead.
            let rows = || {
                groups
                    .iter()
                    .flat_map(|group| &group.rows)
                    .filter(|row| row.refusal.is_none())
            };
            state.pick = rows()
                .find(|row| row.current)
                .or_else(|| {
                    let target = target.as_ref()?;
                    rows().find(|row| pins_a_bin_of(target, row))
                })
                .map(Row::key);
        }
    }

    // The pick is looked up whole rather than through the list, so a filter
    // that hides it does not also take its sheet away.
    let picked = state.pick.as_ref().and_then(|key| {
        let library = manager.get_library(&key.library)?;
        Some((library, entry(library, &key.model)?))
    });
    let geometry = target
        .as_ref()
        .map_or_else(InstanceGeometry::default, |target| target.geometry);
    let status = footer(
        manager,
        target.as_ref(),
        picked.as_ref().map(|(library, entry)| (*library, entry)),
        &groups,
        &state.filter,
    );
    let binding = target.is_some();
    let title = target.as_ref().map_or_else(
        || "Model browser".to_owned(),
        |target| format!("Select a model for {}", target.reference),
    );

    let mut dialog = Dialog::new(title, if binding { "Select" } else { "Close" })
        .description(if binding {
            PURPOSE_BIND
        } else {
            PURPOSE_BROWSE
        })
        .size(DialogSize::StimulusLink)
        .flush_body()
        .manual_body_scroll()
        .hint(status.hint)
        .hint_tone(status.tone)
        .initial_focus(DialogInitialFocus::BodyControl);
    if binding {
        dialog = dialog.ghost("Cancel").primary_enabled(status.can_select);
    }

    let empty = empty_note(manager, target.as_ref(), &state.filter);
    // The body is sized from the whole catalog rather than from what the
    // filter leaves, so typing does not shrink the dialog under the reader.
    let whole = (!state.filter.trim().is_empty()).then(|| catalog(manager, target.as_ref(), ""));
    let whole = whole.as_deref().unwrap_or(&groups);
    let mut filter = std::mem::take(&mut state.filter);
    let mut pick = state.pick.take();
    let mut reveal = state.reveal_pick;
    let mut activated = false;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        let width = ui.available_width();
        let split = width >= SPLIT_MIN;
        let list_width = if !split {
            width
        } else if width < NARROW_BODY {
            NARROW_LIST_WIDTH
        } else {
            LIST_WIDTH
        };
        let body = body_height(
            ui,
            whole,
            split.then_some(width - list_width - 1.0),
            geometry,
        );
        let mut list = |ui: &mut Ui| {
            list_pane(
                ui,
                &groups,
                &mut filter,
                &mut pick,
                &mut reveal,
                &mut activated,
                &empty,
            )
        };
        let focus = if split {
            panes(ui, list_width, body, list, |ui| {
                sheet_pane(ui, picked.as_ref(), geometry);
            })
        } else {
            ui.allocate_ui_with_layout(vec2(width, body), Layout::top_down(Align::Min), |ui| {
                ui.set_min_size(vec2(width, body));
                list(ui)
            })
            .inner
        };
        Some(focus)
    });
    state.filter = filter;
    state.pick = pick;
    state.reveal_pick = reveal;

    // A double-click is the primary for the row it lands on. On the model the
    // instance already runs it is a decision to keep it.
    let commit = choice == DialogChoice::Primary || (binding && activated);
    match choice {
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.close();
            ModelBrowserResult::Cancelled
        }
        _ if commit && binding => {
            let chosen = state.pick.clone().filter(|key| {
                let Some(target) = target.as_ref() else {
                    return false;
                };
                manager
                    .get_library(&key.library)
                    .filter(|library| entry(library, &key.model).is_some())
                    .is_some_and(|library| {
                        !runs_now(
                            target,
                            library,
                            &key.model,
                            provider_refusal(manager, library, &key.model).is_none(),
                        )
                    })
            });
            state.close();
            match chosen {
                Some(key) => ModelBrowserResult::Selected {
                    library: key.library,
                    model: key.model,
                },
                None => ModelBrowserResult::Cancelled,
            }
        }
        DialogChoice::Primary => {
            state.close();
            ModelBrowserResult::Cancelled
        }
        _ => ModelBrowserResult::None,
    }
}

/// What the footer says, how it marks it, and whether the primary is live.
struct Footer {
    hint: String,
    tone: DialogHintTone,
    can_select: bool,
}

/// The footer's sentence: what Select would do, or why there is nothing to
/// select.
///
/// For a binned family it also says which bin the instance's L and W run —
/// or, when no bin holds them, that a run refuses the instance: circuit
/// construction rejects a geometry its family does not cover. Select stays
/// live, since the binding is right and the geometry is what has to change.
fn footer(
    manager: &ModelLibraryManager,
    target: Option<&ModelTarget>,
    picked: Option<(&ModelLibrary, &Entry<'_>)>,
    groups: &[Group<'_>],
    filter: &str,
) -> Footer {
    let neutral = |hint: String| Footer {
        hint,
        tone: DialogHintTone::Neutral,
        can_select: false,
    };
    let Some(target) = target else {
        let libraries = manager.library_count();
        return neutral(format!(
            "{} in {}",
            counted(manager.total_model_count(), "model", "models"),
            counted(libraries, "library", "libraries"),
        ));
    };
    let reference = &target.reference;
    if let Some((library, entry)) = picked {
        let name = entry.name();
        let provided = provider_refusal(manager, library, name).is_none();
        let unchanged = runs_now(target, library, name, provided);
        let action = match &target.current {
            _ if unchanged => format!("{reference} already uses {name}"),
            Some((current, _)) => {
                format!(
                    "Replaces {current} on {reference} with {name} from {}",
                    library.name
                )
            }
            None => format!("Binds {name} from {} to {reference}", library.name),
        };
        let (hint, tone) = match (entry, target.geometry.known()) {
            (Entry::Family(family), Some((length, width))) => {
                match family.selected(Some(length), Some(width)) {
                    Some(bin) => (
                        format!("{action}; its L and W run {}", bin.name),
                        DialogHintTone::Neutral,
                    ),
                    None => (
                        format!(
                            "{action}; no {name} bin holds L {}, W {}, so a run refuses {reference}",
                            format_si_value(length),
                            format_si_value(width)
                        ),
                        DialogHintTone::Warn,
                    ),
                }
            }
            _ => (action, DialogHintTone::Neutral),
        };
        return Footer {
            hint,
            tone,
            can_select: !unchanged,
        };
    }
    if let Some((current, _)) = &target.current
        && manager
            .definition_providers(ModelConsumerScope::PrimitiveModel, current)
            .is_empty()
    {
        return Footer {
            hint: format!("{reference} names {current}, which no loaded library provides"),
            tone: DialogHintTone::Warn,
            can_select: false,
        };
    }
    if groups.is_empty() && filter.trim().is_empty() {
        return neutral(format!(
            "Load a model library in Models to bind {reference}"
        ));
    }
    neutral(format!("Pick a model for {reference}"))
}

/// The one line the list shows when it has no rows.
fn empty_note(manager: &ModelLibraryManager, target: Option<&ModelTarget>, filter: &str) -> String {
    if manager.library_count() == 0 {
        "No model library is loaded".to_owned()
    } else if !filter.trim().is_empty() {
        "No model matches this filter".to_owned()
    } else if let Some(target) = target {
        format!("No loaded library has a model {} can run", target.reference)
    } else {
        "The loaded libraries hold no models".to_owned()
    }
}

/// `1 library`, `3 libraries`.
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

/// The body's two panes: a fixed-width pane, one hairline, and a pane that
/// fills what is left.
fn panes<R>(
    ui: &mut Ui,
    left_width: f32,
    height: f32,
    left: impl FnOnce(&mut Ui) -> R,
    right: impl FnOnce(&mut Ui),
) -> R {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let inner = ui
            .allocate_ui_with_layout(
                vec2(left_width, height),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(left_width, height));
                    left(ui)
                },
            )
            .inner;
        let (divider, _) = ui.allocate_exact_size(vec2(1.0, height), Sense::hover());
        ui.painter().rect_filled(divider, 0.0, t.color.border);
        let remaining = ui.available_width();
        ui.allocate_ui_with_layout(
            vec2(remaining, height),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_min_size(vec2(remaining, height));
                right(ui);
            },
        );
        inner
    })
    .inner
}

/// The list pane: the filter, then each library's models. Returns the filter
/// field's id, which is where the dialog opens its focus.
fn list_pane(
    ui: &mut Ui,
    groups: &[Group<'_>],
    filter: &mut String,
    pick: &mut Option<ModelKey>,
    reveal: &mut bool,
    activated: &mut bool,
    empty_note: &str,
) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let filter_id = ui.id().with("model-browser-filter");
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: PANE_INSET as i8,
            right: PANE_INSET as i8,
            top: FILTER_TOP as i8,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.set_width(width - 2.0 * PANE_INSET);
            ui.spacing_mut().item_spacing.y = 0.0;
            let response = ui.add_sized(
                [ui.available_width(), t.metrics.ctl_h],
                egui::TextEdit::singleline(filter)
                    .id(filter_id)
                    .hint_text("Filter models")
                    .font(egui::TextStyle::Monospace)
                    .margin(egui::Margin::symmetric(8, 4))
                    .vertical_align(Align::Center),
            );
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_label("Filter models");
                node.set_description(
                    "Matches a model's name, family, level, library or description; each word narrows further",
                );
            });
            if step_selection(ui, groups, pick) {
                *reveal = true;
            }
            focus_filter_on_typing(ui, filter_id);
            ui.add_space(FILTER_GAP);
            egui::ScrollArea::vertical()
                .id_salt("model-browser-models")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for (index, group) in groups.iter().enumerate() {
                        if index > 0 {
                            ui.add_space(GROUP_GAP);
                        }
                        group_label(ui, group.library, group.rows.len());
                        for row in &group.rows {
                            let selected = pick.as_ref().is_some_and(|key| row.is(key));
                            let response = model_row(ui, row, selected);
                            if selected && *reveal {
                                ui.scroll_to_rect(response.rect, Some(Align::Center));
                            }
                            if response.clicked() || response.double_clicked() {
                                *pick = Some(row.key());
                                // The keys stay with the filter, where the
                                // arrows move the pick and typing narrows.
                                ui.memory_mut(|memory| memory.request_focus(filter_id));
                            }
                            if response.double_clicked() {
                                *activated = true;
                            }
                        }
                    }
                    if groups.is_empty() {
                        ui.add_space(4.0);
                        note(ui, empty_note);
                    }
                });
            *reveal = false;
        });
    filter_id
}

/// Move the pick with the arrow keys, across library boundaries and over the
/// rows that cannot be bound. Reports whether it moved.
///
/// A refused row is skipped rather than picked: the primary reads its
/// enablement from the pick, so landing on one would leave a reader pressing a
/// button that does nothing.
fn step_selection(ui: &Ui, groups: &[Group<'_>], pick: &mut Option<ModelKey>) -> bool {
    let step = ui.input_mut(|input| {
        i32::from(input.consume_key(Modifiers::NONE, Key::ArrowDown))
            - i32::from(input.consume_key(Modifiers::NONE, Key::ArrowUp))
    });
    if step == 0 {
        return false;
    }
    match stepped(groups, pick.as_ref(), step) {
        Some(next) if pick.as_ref() != Some(&next) => {
            *pick = Some(next);
            true
        }
        _ => false,
    }
}

/// Where `step` rows from `pick` lands, over the rows a reader can bind.
fn stepped(groups: &[Group<'_>], pick: Option<&ModelKey>, step: i32) -> Option<ModelKey> {
    let reachable: Vec<&Row<'_>> = groups
        .iter()
        .flat_map(|group| &group.rows)
        .filter(|row| row.refusal.is_none())
        .collect();
    let last = reachable.len().checked_sub(1)?;
    let index = match pick.and_then(|key| reachable.iter().position(|row| row.is(key))) {
        Some(index) => index.saturating_add_signed(step as isize).min(last),
        None => 0,
    };
    Some(reachable[index].key())
}

/// Send a keystroke to the filter wherever the reader's focus happens to be.
///
/// Typing into a list means one thing: narrow it. The character is not lost,
/// because focus is granted before the field is added on this same frame.
fn focus_filter_on_typing(ui: &Ui, filter_id: egui::Id) {
    let typed = ui.input(|input| {
        input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Text(_)))
    });
    if typed && !ui.memory(|memory| memory.has_focus(filter_id)) {
        ui.memory_mut(|memory| memory.request_focus(filter_id));
    }
}

/// One library's quiet label, with how many of its models are listed.
fn group_label(ui: &mut Ui, library: &str, count: usize) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), GROUP_LABEL_HEIGHT),
        Sense::hover(),
    );
    let announcement = format!("{library}, {}", counted(count, "model", "models"));
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &announcement));
    let baseline = Rect::from_min_max(
        egui::pos2(rect.left(), rect.bottom() - 16.0),
        egui::pos2(rect.right(), rect.bottom() - 2.0),
    );
    let count = count.to_string();
    let mono = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let count_width = measured_width(ui, &count, mono.clone());
    run(ui, baseline, true, &count, mono, t.color.text_faint);
    run(
        ui,
        Rect::from_min_max(
            baseline.min,
            egui::pos2(baseline.right() - count_width - 12.0, baseline.bottom()),
        ),
        false,
        library,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
}

/// One model row: its name over its level and key figures, and on the right
/// whether the instance runs it now or why it cannot.
///
/// Every run is painted rather than added as a label: a label inside a click
/// rect swallows the press that was meant for the row. The row takes clicks
/// but not focus, which stays in the filter, where the arrow keys move the
/// pick.
fn model_row(ui: &mut Ui, row: &Row<'_>, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let enabled = row.refusal.is_none();
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), ROW_HEIGHT),
        if enabled {
            Sense::CLICK
        } else {
            Sense::hover()
        },
    );
    let meta = row.meta.join(" \u{00b7} ");
    let tag = if row.current {
        Some("current")
    } else {
        row.refusal.as_ref().map(|refusal| refusal.tag)
    };
    let name = row.entry.name();
    let announcement = match (&row.refusal, row.current) {
        (Some(refusal), _) => format!("{name} {meta}. {}", refusal.reason),
        (None, true) => format!("{name} {meta}, current"),
        (None, false) => format!("{name} {meta}"),
    };
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            enabled,
            selected,
            &announcement,
        )
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    } else if enabled && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let mut text_right = rect.right() - 10.0;
    if let Some(tag) = tag {
        let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
        let width = measured_width(ui, tag, font.clone()) + 12.0;
        let pill = Rect::from_min_size(
            egui::pos2(text_right - width, rect.center().y - 8.0),
            vec2(width, 16.0),
        );
        ui.painter().rect_filled(pill, 3.0, t.color.bg_inset);
        ui.painter().text(
            pill.center(),
            egui::Align2::CENTER_CENTER,
            tag,
            font,
            if row.current {
                t.color.text_dim
            } else {
                t.color.text_faint
            },
        );
        text_right = pill.left() - 8.0;
    }

    let text_left = rect.left() + 12.0;
    let text_width = (text_right - text_left).max(20.0);
    run(
        ui,
        Rect::from_min_size(
            egui::pos2(text_left, rect.top() + 4.0),
            vec2(text_width, 15.0),
        ),
        false,
        name,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    let meta_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let fitted = fitted_meta(ui, &row.meta, text_width, meta_font.clone());
    run(
        ui,
        Rect::from_min_size(
            egui::pos2(text_left, rect.bottom() - 18.0),
            vec2(text_width, 14.0),
        ),
        false,
        &fitted,
        meta_font,
        t.color.text_faint,
    );
    match &row.refusal {
        Some(refusal) => response.on_hover_text(refusal.reason.as_str()),
        None => response,
    }
}

/// As many of a row's meta facts as fit its track, longest prefix first.
///
/// A fact is whole — a level, one figure — so a row too narrow for all of
/// them gives up the last one entirely rather than printing half a number.
fn fitted_meta(ui: &mut Ui, parts: &[String], width: f32, font: egui::FontId) -> String {
    let mut kept = parts.len();
    while kept > 1 {
        let line = parts[..kept].join(" \u{00b7} ");
        if measured_width(ui, &line, font.clone()) <= width {
            return line;
        }
        kept -= 1;
    }
    parts.first().cloned().unwrap_or_default()
}

/// The sheet pane: the picked model's identity, where it comes from, and every
/// parameter its card sets — for a binned family, its bins' windows and the
/// parameters of the bin the instance's geometry selects.
fn sheet_pane(
    ui: &mut Ui,
    picked: Option<&(&ModelLibrary, Entry<'_>)>,
    geometry: InstanceGeometry,
) {
    let width = ui.available_width();
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: PANE_INSET as i8,
            right: PANE_INSET as i8,
            top: FILTER_TOP as i8,
            bottom: PANE_INSET as i8,
        })
        .show(ui, |ui| {
            ui.set_width(width - 2.0 * PANE_INSET);
            ui.spacing_mut().item_spacing.y = 0.0;
            let t = Tokens::get(ui.ctx());
            let Some((library, entry)) = picked else {
                // On the filter's own line, as the model's name would be.
                ui.add_space((t.metrics.ctl_h - NOTE_HEIGHT) * 0.5);
                note(ui, "Pick a model to read its parameters");
                return;
            };
            let model = entry.card(geometry);
            let line = |ui: &mut Ui, height: f32, text: &str, font: egui::FontId, color| {
                let (rect, response) =
                    ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
                response
                    .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
                if run(ui, rect, false, text, font, color) {
                    response.on_hover_text(text);
                }
            };
            // The name sits on the filter's centreline, so across the hairline
            // the two first lines read as one, and keeps none of the control
            // height below its own line.
            let name = entry.name();
            let name_font = theme::mono(tokens::FS_3, FontWeight::Medium);
            let height = name_height(ui, &name_font);
            let (rect, response) =
                ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
            response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, name));
            let centred = Rect::from_min_size(rect.min, vec2(rect.width(), t.metrics.ctl_h));
            if run(ui, centred, false, name, name_font, t.color.text) {
                response.on_hover_text(name);
            }
            let mut kind = vec![model.model_type.display_name().to_owned()];
            kind.extend(level_label(model));
            if let Entry::Family(family) = entry {
                kind.push(counted(family.bins.len(), "bin", "bins"));
            }
            line(
                ui,
                KIND_HEIGHT,
                kind.join(" \u{00b7} ").as_str(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
            );
            if !model.description.trim().is_empty() {
                ui.add_space(DESCRIPTION_GAP);
                description(ui, model.description.trim());
            }

            ui.add_space(FACTS_GAP);
            let column = fact_column(ui);
            for (label, value, hover) in &sheet_facts(library, entry, model) {
                fact(ui, column, label, value, hover.as_deref());
            }

            let parameters = parameters(model);
            let family = match entry {
                Entry::Family(family) => Some(family),
                Entry::Card(_) => None,
            };
            let selected =
                family.and_then(|family| family.selected(geometry.length, geometry.width));
            let label = match (family, selected) {
                (None, _) => "Parameters".to_owned(),
                (Some(_), Some(bin)) => format!("Parameters of {}", bin.name),
                (Some(_), None) => format!("Parameters of {}, the first bin", model.name),
            };
            let parameter_head = |ui: &mut Ui| {
                caption(ui, &label, &parameters.len().to_string());
                rule(ui);
            };
            ui.add_space(PARAMETERS_GAP);
            let scroll = egui::ScrollArea::vertical()
                .id_salt("model-browser-parameters")
                .auto_shrink([false, false]);
            let Some(family) = family else {
                // A card's head stays put while its grid scrolls under it.
                parameter_head(ui);
                if parameters.is_empty() {
                    note(ui, NO_PARAMETERS);
                    return;
                }
                scroll.show(ui, |ui| parameter_grid(ui, &parameters));
                return;
            };
            // A family's bins and its parameters scroll as one: 48 bins are
            // taller than the pane, and two scroll regions in one sheet would
            // leave the reader working out which one the wheel moves.
            scroll.show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                caption(ui, "Bins", &family.bins.len().to_string());
                rule(ui);
                bin_table(ui, family, selected);
                ui.add_space(PARAMETERS_GAP);
                parameter_head(ui);
                if parameters.is_empty() {
                    note(ui, NO_PARAMETERS);
                } else {
                    parameter_grid(ui, &parameters);
                }
            });
        });
}

/// What the sheet says of a card that sets no parameters.
const NO_PARAMETERS: &str = "The card sets no parameters; every one takes its default";

/// A hairline under a caption, with the air the sheet keeps on each side of it.
fn rule(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(RULE_GAP);
    let (rule, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(rule, 0.0, t.color.border);
    ui.add_space(RULE_GAP);
}

/// The bin table's inset inside a cell, so a marked cell's fill frames its
/// text, and the air between two of a group's columns.
const BIN_CELL_INSET: f32 = 4.0;
const BIN_CELL_GAP: f32 = 16.0;

/// The bin table's three columns, each as wide as the widest bin name, L
/// window and W window it lays out: a kit's names and windows vary too much
/// for fixed columns, which either elide `nch_lvt.48` or leave room for one
/// column group where two fit.
fn bin_columns(ui: &mut Ui, family: &BinFamily<'_>) -> [f32; 3] {
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut widths = [0.0_f32; 3];
    for bin in &family.bins {
        let texts = [
            bin.name.clone(),
            window_label(bin.l_min, bin.l_max),
            window_label(bin.w_min, bin.w_max),
        ];
        for (width, text) in widths.iter_mut().zip(texts) {
            *width = width.max(measured_width(ui, &text, mono.clone()));
        }
    }
    widths
}

/// How many column groups of the bin table a pane `width` across holds.
fn bin_groups(columns: [f32; 3], width: f32, bins: usize) -> usize {
    let group = 2.0 * BIN_CELL_INSET + columns.iter().sum::<f32>() + 2.0 * BIN_CELL_GAP;
    (((width + PARAMETER_GUTTER) / (group + PARAMETER_GUTTER)).floor() as usize)
        .clamp(1, bins.max(1))
}

/// Rows the bin table takes at `width`, its head included.
fn bin_table_rows(ui: &mut Ui, family: &BinFamily<'_>, width: f32) -> usize {
    let groups = bin_groups(bin_columns(ui, family), width, family.bins.len());
    1 + family.bins.len().div_ceil(groups)
}

/// Every bin with the L and W window it holds, read down each column group
/// and then across, with the bin the instance's geometry selects marked.
fn bin_table(ui: &mut Ui, family: &BinFamily<'_>, selected: Option<&DeviceModel>) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let widths = bin_columns(ui, family);
    let groups = bin_groups(widths, width, family.bins.len());
    let rows = family.bins.len().div_ceil(groups);
    let group_width = (width - PARAMETER_GUTTER * (groups - 1) as f32) / groups as f32;
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    // Each column's left edge and width in a group whose cell starts at `left`.
    let columns = |left: f32| {
        let name = left + BIN_CELL_INSET;
        let length = name + widths[0] + BIN_CELL_GAP;
        let width = length + widths[1] + BIN_CELL_GAP;
        [(name, widths[0]), (length, widths[1]), (width, widths[2])]
    };
    let column = |(x, width): (f32, f32), top: f32| {
        Rect::from_min_size(egui::pos2(x, top), vec2(width, PARAMETER_PITCH))
    };
    let (head, _) = ui.allocate_exact_size(vec2(width, PARAMETER_PITCH), Sense::hover());
    for group in 0..groups {
        let left = head.left() + group as f32 * (group_width + PARAMETER_GUTTER);
        for (place, label) in columns(left).into_iter().zip(["Bin", "L", "W"]) {
            run(
                ui,
                column(place, head.top()),
                false,
                label,
                theme::sans(tokens::FS_MICRO, FontWeight::Medium),
                t.color.text_faint,
            );
        }
    }
    for row_index in 0..rows {
        let (row, _) = ui.allocate_exact_size(vec2(width, PARAMETER_PITCH), Sense::hover());
        if !ui.is_rect_visible(row) {
            continue;
        }
        for group in 0..groups {
            let Some(bin) = family.bins.get(group * rows + row_index) else {
                continue;
            };
            let left = row.left() + group as f32 * (group_width + PARAMETER_GUTTER);
            let cell = Rect::from_min_size(
                egui::pos2(left, row.top()),
                vec2(group_width, PARAMETER_PITCH),
            );
            let runs = selected.is_some_and(|selected| std::ptr::eq(selected, *bin));
            if runs {
                ui.painter().rect_filled(cell, 0.0, t.color.accent_dim);
            }
            let length_window = window_label(bin.l_min, bin.l_max);
            let width_window = window_label(bin.w_min, bin.w_max);
            let name_color = if runs { t.color.text } else { t.color.text_dim };
            for (place, (text, color)) in columns(left).into_iter().zip([
                (bin.name.as_str(), name_color),
                (length_window.as_str(), t.color.text),
                (width_window.as_str(), t.color.text),
            ]) {
                run(
                    ui,
                    column(place, cell.top()),
                    false,
                    text,
                    mono.clone(),
                    color,
                );
            }
            let id = ui.id().with(("model-browser-bin", row_index, group));
            let response = ui.interact(cell, id, Sense::hover());
            let announcement = format!(
                "{} L {length_window} W {width_window}{}",
                bin.name,
                if runs { ", runs at this geometry" } else { "" }
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &announcement)
            });
        }
    }
}

/// What the sheet states about where a card comes from and what it covers,
/// each with the whole value for the hover where the row shows less.
///
/// A family's source is the card that speaks for it, and its length and
/// width are what its bins cover together; each bin's own window is in the
/// table below.
fn sheet_facts(
    library: &ModelLibrary,
    entry: &Entry<'_>,
    model: &DeviceModel,
) -> Vec<(&'static str, String, Option<String>)> {
    let source = match &model.file_path {
        Some(path) => {
            let file = path
                .file_name()
                .and_then(|name| name.to_str())
                .map_or_else(|| path.display().to_string(), str::to_owned);
            let shown = model
                .source_line
                .map_or_else(|| file.clone(), |line| format!("{file}:{line}"));
            (shown, Some(path.display().to_string()))
        }
        None => ("Built in".to_owned(), None),
    };
    let mut facts = vec![
        ("Library", library.name.clone(), None),
        ("Source", source.0, source.1),
    ];
    if let Some(section) = &model.section {
        facts.push(("Section", section.clone(), None));
    }
    let mut corners: Vec<&String> = library.corners.keys().collect();
    corners.sort_by(|left, right| natural_cmp(left, right));
    if !corners.is_empty() {
        let corners = corners
            .iter()
            .map(|corner| corner.as_str())
            .collect::<Vec<_>>()
            .join(" \u{00b7} ");
        facts.push(("Corners", corners, None));
    }
    let [length, width] = match entry {
        Entry::Card(_) => [(model.l_min, model.l_max), (model.w_min, model.w_max)],
        Entry::Family(family) => coverage(family),
    };
    if let Some(length) = range_label(length.0, length.1) {
        facts.push(("Length", length, None));
    }
    if let Some(width) = range_label(width.0, width.1) {
        facts.push(("Width", width, None));
    }
    if let Some(vdd) = model.vdd {
        facts.push(("Supply", format!("{} V", spelled(vdd)), None));
    }
    facts
}

/// How many parameters the sheet's grid holds for a card.
fn parameter_count(model: &DeviceModel) -> usize {
    let shown = |name: &String| !STATED_ELSEWHERE.contains(&name.as_str());
    model.parameters.keys().filter(|name| shown(name)).count()
        + model
            .string_parameters
            .keys()
            .filter(|name| shown(name))
            .count()
}

/// How many parameter columns a grid `width` across holds.
fn grid_columns(width: f32) -> usize {
    (((width + PARAMETER_GUTTER) / (PARAMETER_CELL_MIN + PARAMETER_GUTTER)).floor() as usize).max(1)
}

/// How tall the body is: what the list needs, or the tallest sheet of any
/// model it lists, held between the two bounds a modal stays readable in.
///
/// Measured over the catalog rather than over the pick, so the dialog keeps
/// its frame while the reader moves through it. Written as the same sums the
/// panes paint, in the same order, so a band added to one is visibly missing
/// from the other.
fn body_height(
    ui: &mut Ui,
    groups: &[Group<'_>],
    sheet_width: Option<f32>,
    geometry: InstanceGeometry,
) -> f32 {
    let control = Tokens::get(ui.ctx()).metrics.ctl_h;
    let mut height = FILTER_TOP + control + FILTER_GAP + PANE_INSET;
    if groups.is_empty() {
        height += 4.0 + NOTE_HEIGHT;
    }
    for (index, group) in groups.iter().enumerate() {
        if index > 0 {
            height += GROUP_GAP;
        }
        height += GROUP_LABEL_HEIGHT + group.rows.len() as f32 * ROW_HEIGHT;
    }
    if let Some(width) = sheet_width {
        let inner = width - 2.0 * PANE_INSET;
        for row in groups.iter().flat_map(|group| &group.rows) {
            if height >= MAX_BODY {
                break;
            }
            height = height.max(sheet_height(ui, row.library, &row.entry, geometry, inner));
        }
    }
    height.clamp(MIN_BODY, MAX_BODY)
}

/// How tall the sheet's name line is: down to the foot of the name's own line
/// when it is centred on the filter beside it.
fn name_height(ui: &mut Ui, font: &egui::FontId) -> f32 {
    let row = ui.fonts_mut(|fonts| fonts.row_height(font));
    (Tokens::get(ui.ctx()).metrics.ctl_h + row) * 0.5
}

/// The height one model's sheet paints at `width` inside its insets.
fn sheet_height(
    ui: &mut Ui,
    library: &ModelLibrary,
    entry: &Entry<'_>,
    geometry: InstanceGeometry,
    width: f32,
) -> f32 {
    let model = entry.card(geometry);
    let name_font = theme::mono(tokens::FS_3, FontWeight::Medium);
    let mut height = FILTER_TOP + name_height(ui, &name_font) + KIND_HEIGHT;
    let description = model.description.trim();
    if !description.is_empty() {
        let job = description_job(description, width, egui::Color32::WHITE);
        height += DESCRIPTION_GAP + ui.fonts_mut(|fonts| fonts.layout_job(job)).size().y;
    }
    height += FACTS_GAP + sheet_facts(library, entry, model).len() as f32 * FACT_PITCH;
    let head = CAPTION_HEIGHT + RULE_GAP + 1.0 + RULE_GAP;
    height += PARAMETERS_GAP + head;
    if let Entry::Family(family) = entry {
        height +=
            bin_table_rows(ui, family, width) as f32 * PARAMETER_PITCH + PARAMETERS_GAP + head;
    }
    let count = parameter_count(model);
    height += if count == 0 {
        NOTE_HEIGHT
    } else {
        count.div_ceil(grid_columns(width)) as f32 * PARAMETER_PITCH
    };
    height + PANE_INSET
}

/// A card's description laid out to two lines at most.
fn description_job(text: &str, width: f32, color: egui::Color32) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::single_section(
        text.to_owned(),
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            color,
            ..Default::default()
        },
    );
    job.wrap = egui::text::TextWrapping {
        max_width: width,
        max_rows: 2,
        overflow_character: Some('\u{2026}'),
        ..Default::default()
    };
    job
}

/// The card's description, wrapped to two lines at most and whole on hover.
fn description(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let galley =
        ui.fonts_mut(|fonts| fonts.layout_job(description_job(text, width, t.color.text_dim)));
    let elided = galley.elided;
    let (rect, response) = ui.allocate_exact_size(vec2(width, galley.size().y), Sense::hover());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    ui.painter().galley(rect.min, galley, t.color.text_dim);
    if elided {
        response.on_hover_text(text);
    }
}

/// Where the facts' values start: past the widest label any sheet states.
fn fact_column(ui: &mut Ui) -> f32 {
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    FACT_LABELS
        .iter()
        .map(|label| measured_width(ui, label, font.clone()))
        .fold(0.0, f32::max)
        + FACT_LABEL_GAP
}

/// One label/value fact, the value starting `column` in.
fn fact(ui: &mut Ui, column: f32, label: &str, value: &str, hover: Option<&str>) {
    debug_assert!(
        FACT_LABELS.contains(&label),
        "{label} is not in FACT_LABELS"
    );
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), FACT_PITCH), Sense::hover());
    let announcement = format!("{label} {value}");
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &announcement));
    run(
        ui,
        Rect::from_min_max(
            rect.min,
            egui::pos2(rect.left() + column - FACT_LABEL_GAP, rect.bottom()),
        ),
        false,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let elided = run(
        ui,
        Rect::from_min_max(egui::pos2(rect.left() + column, rect.top()), rect.max),
        false,
        value,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    match hover {
        Some(hover) => {
            response.on_hover_text(hover);
        }
        None if elided => {
            response.on_hover_text(value);
        }
        None => {}
    }
}

/// A section caption: its name at the left, a count at the right.
fn caption(ui: &mut Ui, label: &str, count: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), CAPTION_HEIGHT), Sense::hover());
    let announcement = format!("{label}, {count}");
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &announcement));
    run(
        ui,
        rect,
        false,
        label,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    run(
        ui,
        rect,
        true,
        count,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
    );
}

/// Every parameter on one pitch, as many columns across as the pane holds,
/// read across: each cell its name at the left and its value at the right, so
/// the values of a column line up on their last digit.
fn parameter_grid(ui: &mut Ui, parameters: &[Parameter]) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let columns = grid_columns(width);
    let cell = (width - PARAMETER_GUTTER * (columns - 1) as f32) / columns as f32;
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    for (row_index, chunk) in parameters.chunks(columns).enumerate() {
        let (row, _) = ui.allocate_exact_size(vec2(width, PARAMETER_PITCH), Sense::hover());
        if !ui.is_rect_visible(row) {
            continue;
        }
        for (column, parameter) in chunk.iter().enumerate() {
            let left = row.left() + column as f32 * (cell + PARAMETER_GUTTER);
            let cell_rect =
                Rect::from_min_size(egui::pos2(left, row.top()), vec2(cell, PARAMETER_PITCH));
            let name_width = measured_width(ui, &parameter.name, mono.clone()).min(cell * 0.6);
            run(
                ui,
                Rect::from_min_size(cell_rect.min, vec2(name_width, PARAMETER_PITCH)),
                false,
                &parameter.name,
                mono.clone(),
                t.color.text_dim,
            );
            run(
                ui,
                Rect::from_min_max(
                    egui::pos2(cell_rect.left() + name_width + 10.0, cell_rect.top()),
                    cell_rect.max,
                ),
                true,
                &parameter.value,
                mono.clone(),
                t.color.text,
            );
            let id = ui.id().with(("model-browser-parameter", row_index, column));
            let response = ui.interact(cell_rect, id, Sense::hover());
            let announcement = format!("{} {}", parameter.name, parameter.value);
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &announcement)
            });
            response.on_hover_text(format!("{} = {}", parameter.name, parameter.exact));
        }
    }
}

/// One faint line of state.
fn note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), NOTE_HEIGHT), Sense::hover());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    run(
        ui,
        rect,
        false,
        text,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

/// How wide one run lays out, for placing what follows it.
fn measured_width(ui: &mut Ui, text: &str, font: egui::FontId) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
            .size()
            .x
    })
}

/// One run of text, vertically centred in `rect` and elided into its width.
/// Reports whether it was elided, so the caller can state it whole on hover.
fn run(
    ui: &mut Ui,
    rect: Rect,
    right_aligned: bool,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
) -> bool {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, color);
    job.wrap.max_width = rect.width().max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.overflow_character = Some('\u{2026}');
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let elided = galley.elided;
    let x = if right_aligned {
        rect.right() - galley.size().x
    } else {
        rect.left()
    };
    ui.painter().galley(
        egui::pos2(x, rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    elided
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests;

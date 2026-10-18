//! Geometry-bin families: the cards one binned device name runs as.
//!
//! A foundry kit ships a MOS device as a family of bin cards — `nch.1` …
//! `nch.48`, each valid over its own L×W window — and a design binds the
//! family by its name, `nch`, leaving the simulator to choose the card from the
//! instance's geometry. Pinning an instance to one bin is wrong practice: it
//! then runs that card at every size, inside its window or not.
//!
//! The engine's resolver is the authority. An instance naming a card exactly
//! runs that card; otherwise it runs the first card of the family, in
//! declaration order, whose window holds its geometry, and is refused when
//! none does. These are its rules read over a library's catalogued cards, so
//! provider resolution, the sealed namespace and the Properties picker agree
//! with it about which cards a family name reaches.

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::{DeviceModel, ModelLibrary};

/// The family a bin card belongs to: `nch` for `nch.12`.
///
/// A bin is named `<family>.<n>`, `n` its number. Any other name is no bin,
/// and neither is one whose family part would be empty.
#[must_use]
pub fn model_bin_family(name: &str) -> Option<&str> {
    let (family, bin) = name.rsplit_once('.')?;
    (!family.is_empty() && !bin.is_empty() && bin.bytes().all(|byte| byte.is_ascii_digit()))
        .then_some(family)
}

/// One library's bins of one family.
#[derive(Debug, Clone)]
pub struct BinFamily<'a> {
    /// The name an instance binds, as the family's first bin spells it.
    pub name: &'a str,
    /// Every bin, in declaration order.
    pub bins: Vec<&'a DeviceModel>,
}

impl<'a> BinFamily<'a> {
    /// The bin an instance of this geometry runs: the first, in declaration
    /// order, whose window holds it — the test circuit construction applies,
    /// at its tolerance. `None` when no bin does, which the engine refuses.
    ///
    /// The instance's fin count is not known here, so a bin that bounds NFIN
    /// is never selected, as the engine does not select it for an instance
    /// that sets none.
    #[must_use]
    pub fn selected(&self, length: Option<f64>, width: Option<f64>) -> Option<&'a DeviceModel> {
        self.bins
            .iter()
            .copied()
            .find(|bin| bin_geometry(bin).contains(length, width, None))
    }
}

/// What an instance binds from one library: a card, or a binned family whose
/// bin the simulator chooses from the instance's geometry.
#[derive(Debug, Clone)]
pub enum BindableModel<'a> {
    Card(&'a DeviceModel),
    Family(BinFamily<'a>),
}

impl<'a> BindableModel<'a> {
    /// The name an instance binds it by.
    #[must_use]
    pub fn name(&self) -> &'a str {
        match *self {
            Self::Card(model) => &model.name,
            Self::Family(ref family) => family.name,
        }
    }

    /// Every card an instance binding it can run.
    #[must_use]
    pub fn cards(&self) -> &[&'a DeviceModel] {
        match self {
            Self::Card(model) => std::slice::from_ref(model),
            Self::Family(family) => &family.bins,
        }
    }
}

/// A bin card's window as the engine reads it.
fn bin_geometry(model: &DeviceModel) -> rspice_core::engine::ModelBinCardGeometry {
    use rspice_core::engine::{ModelBinAxisRange, ModelBinCardGeometry};
    let parameter = |name: &str| model.parameters.get(name).copied();
    ModelBinCardGeometry {
        length: ModelBinAxisRange {
            min: model.l_min,
            max: model.l_max,
        },
        width: ModelBinAxisRange {
            min: model.w_min,
            max: model.w_max,
        },
        nfin: ModelBinAxisRange {
            min: parameter("nfinmin"),
            max: parameter("nfinmax"),
        },
    }
}

/// Declaration order as the catalogue records it: source file, then line.
/// Cards with no recorded line — built in, or made in memory — follow by bin
/// number.
fn declaration_order(left: &DeviceModel, right: &DeviceModel) -> Ordering {
    let file = |model: &DeviceModel| model.file_path.as_deref().map(Path::to_path_buf);
    let number = |model: &DeviceModel| {
        model_bin_family(&model.name)
            .and_then(|family| model.name[family.len() + 1..].parse::<u64>().ok())
            .unwrap_or(u64::MAX)
    };
    file(left)
        .cmp(&file(right))
        .then_with(|| {
            left.source_line
                .unwrap_or(usize::MAX)
                .cmp(&right.source_line.unwrap_or(usize::MAX))
        })
        .then_with(|| number(left).cmp(&number(right)))
        .then_with(|| left.name.cmp(&right.name))
}

impl ModelLibrary {
    /// The family `name` names, when this library provides it as one: it holds
    /// a bin of that family and no card named `name` itself, which an instance
    /// naming it would run instead.
    #[must_use]
    pub fn bin_family(&self, name: &str) -> Option<BinFamily<'_>> {
        let name = name.trim();
        if self
            .models
            .values()
            .any(|model| model.name.eq_ignore_ascii_case(name))
        {
            return None;
        }
        let mut bins: Vec<&DeviceModel> = self
            .models
            .values()
            .filter(|model| {
                model_bin_family(&model.name)
                    .is_some_and(|family| family.eq_ignore_ascii_case(name))
            })
            .collect();
        bins.sort_by(|left, right| declaration_order(left, right));
        let first = bins.first()?;
        Some(BinFamily {
            name: model_bin_family(&first.name).expect("a bin names its family"),
            bins,
        })
    }

    /// Every family this library provides, by name.
    ///
    /// One pass over the cards rather than [`Self::bin_family`] per family: the
    /// Properties picker asks this of every library on every frame, and a kit
    /// holds thousands of cards in dozens of families.
    #[must_use]
    pub fn bin_families(&self) -> Vec<BinFamily<'_>> {
        let names: HashSet<String> = self
            .models
            .values()
            .map(|model| model.name.to_ascii_lowercase())
            .collect();
        let mut families: BTreeMap<String, Vec<&DeviceModel>> = BTreeMap::new();
        for model in self.models.values() {
            let Some(family) = model_bin_family(&model.name).map(str::to_ascii_lowercase) else {
                continue;
            };
            if !names.contains(&family) {
                families.entry(family).or_default().push(model);
            }
        }
        families
            .into_values()
            .map(|mut bins| {
                bins.sort_by(|left, right| declaration_order(left, right));
                BinFamily {
                    name: model_bin_family(&bins[0].name).expect("a bin names its family"),
                    bins,
                }
            })
            .collect()
    }

    /// The cards an instance naming `name` runs from this library: the card of
    /// that name, or every bin of the family it names. Empty when it runs none.
    #[must_use]
    pub fn executable_cards(&self, name: &str) -> Vec<&DeviceModel> {
        self.bindable_model(name)
            .map(|model| model.cards().to_vec())
            .unwrap_or_default()
    }

    /// What an instance naming `name` binds in this library: the card of that
    /// name, or the family it names.
    #[must_use]
    pub fn bindable_model(&self, name: &str) -> Option<BindableModel<'_>> {
        let name = name.trim();
        match self
            .models
            .values()
            .find(|model| model.name.eq_ignore_ascii_case(name))
        {
            Some(card) => Some(BindableModel::Card(card)),
            None => self.bin_family(name).map(BindableModel::Family),
        }
    }

    /// Everything an instance can bind from this library: each family it
    /// provides, and each card that is not a bin of one. A bin is reached
    /// through its family, as the simulator reaches it.
    #[must_use]
    pub fn bindable_models(&self) -> Vec<BindableModel<'_>> {
        let families = self.bin_families();
        let bins: HashSet<&str> = families
            .iter()
            .flat_map(|family| family.bins.iter().map(|bin| bin.name.as_str()))
            .collect();
        self.models
            .values()
            .filter(|model| !bins.contains(model.name.as_str()))
            .map(BindableModel::Card)
            .chain(families.into_iter().map(BindableModel::Family))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModelType;

    fn bin(name: &str, l: (f64, f64), w: (f64, f64), line: usize) -> DeviceModel {
        let mut card = DeviceModel::new(name, ModelType::Nmos).with_geometry(l.0, l.1, w.0, w.1);
        card.source_line = Some(line);
        card
    }

    fn kit() -> ModelLibrary {
        let mut library = ModelLibrary::new("kit");
        library.add_model(bin("nch.2", (1e-6, 10e-6), (1e-6, 10e-6), 20));
        library.add_model(bin("nch.1", (0.1e-6, 1e-6), (1e-6, 10e-6), 10));
        library.add_model(bin("nch.10", (0.1e-6, 1e-6), (0.1e-6, 1e-6), 30));
        library.add_model(DeviceModel::new("nch_25", ModelType::Nmos));
        library.add_model(DeviceModel::new("rpoly", ModelType::Resistor));
        library
    }

    #[test]
    fn a_bin_is_a_numbered_member_of_a_named_family() {
        assert_eq!(model_bin_family("nch.12"), Some("nch"));
        assert_eq!(model_bin_family("NCH.1"), Some("NCH"));
        assert_eq!(model_bin_family("nch.lvt.3"), Some("nch.lvt"));
        for name in ["nch", "nch.", ".1", "nch.1a", "nch_25", "nch.tt"] {
            assert_eq!(model_bin_family(name), None, "{name}");
        }
    }

    #[test]
    fn a_family_holds_its_bins_in_declaration_order() {
        let library = kit();
        let family = library.bin_family("NCH").expect("nch family");
        assert_eq!(family.name, "nch");
        let names: Vec<&str> = family.bins.iter().map(|bin| bin.name.as_str()).collect();
        assert_eq!(names, ["nch.1", "nch.2", "nch.10"]);
        assert_eq!(
            library
                .bin_families()
                .iter()
                .map(|family| family.name)
                .collect::<Vec<_>>(),
            ["nch"]
        );
    }

    /// An instance naming a card exactly runs that card, so a library holding
    /// a card of the family's own name does not provide the family.
    #[test]
    fn a_card_of_the_familys_own_name_is_what_the_name_runs() {
        let mut library = kit();
        assert_eq!(library.executable_cards("nch").len(), 3);
        assert_eq!(library.executable_cards("nch.2").len(), 1);
        assert!(library.executable_cards("pch").is_empty());
        library.add_model(DeviceModel::new("nch", ModelType::Nmos));
        assert!(library.bin_family("nch").is_none());
        assert!(library.bin_families().is_empty());
        let cards = library.executable_cards("nch");
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].name, "nch");
    }

    /// A family is offered once, by its name, and its bins are not offered
    /// beside it; a card of the family's own name keeps its bins as cards.
    #[test]
    fn a_library_offers_each_family_once_and_no_bin_of_one() {
        let mut library = kit();
        let names = |library: &ModelLibrary| {
            let mut names: Vec<String> = library
                .bindable_models()
                .iter()
                .map(|model| model.name().to_owned())
                .collect();
            names.sort();
            names
        };
        assert_eq!(names(&library), ["nch", "nch_25", "rpoly"]);
        let family = library.bindable_model("NCH").expect("the family binds");
        assert!(matches!(family, BindableModel::Family(_)));
        assert_eq!((family.name(), family.cards().len()), ("nch", 3));
        assert!(matches!(
            library.bindable_model("nch.2"),
            Some(BindableModel::Card(card)) if card.name == "nch.2"
        ));
        assert!(library.bindable_model("pch").is_none());

        library.add_model(DeviceModel::new("nch", ModelType::Nmos));
        assert_eq!(
            names(&library),
            ["nch", "nch.1", "nch.10", "nch.2", "nch_25", "rpoly"]
        );
    }

    /// Inclusive at both ends within the engine's 1 nm, first declared wins on
    /// a shared edge, and a geometry no bin holds selects none.
    #[test]
    fn the_selected_bin_is_the_one_the_engine_picks() {
        let library = kit();
        let family = library.bin_family("nch").expect("nch family");
        let selected = |l: f64, w: f64| {
            family
                .selected(Some(l), Some(w))
                .map(|bin| bin.name.as_str())
        };
        assert_eq!(selected(0.5e-6, 2e-6), Some("nch.1"));
        assert_eq!(selected(5e-6, 2e-6), Some("nch.2"));
        assert_eq!(
            selected(1e-6, 2e-6),
            Some("nch.1"),
            "the shared edge goes to the first card"
        );
        assert_eq!(selected(10e-6 + 0.5e-9, 2e-6), Some("nch.2"));
        assert_eq!(selected(0.5e-6, 0.5e-6), Some("nch.10"));
        assert_eq!(selected(25e-6, 2e-6), None);
        assert_eq!(family.selected(None, Some(2e-6)), None);
    }
}

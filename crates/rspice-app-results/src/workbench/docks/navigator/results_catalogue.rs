//! The data browser's catalogue of each analysis's signals, and the listing
//! the query and the facets make of it.
//!
//! A run is immutable once written, so what the browser reads of an
//! analysis's signals (each one's name, the unit the run gave it and the
//! samples it holds) is copied into a catalogue once, a slice of the analysis
//! a frame, and no frame walks a whole large run. The listing (the signals the
//! query and the facets let through, in the order they list, with the
//! instance tree over them) is made from the catalogue away from the frame
//! when the analysis is large, and a frame takes it in when it is ready:
//! until then the browser keeps the listing it had. Rows are made only for
//! what is on screen.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use rspice_results::result_presentation::SourceWaveformPresentationKey;

use super::results::{ResultSignal, ResultsBrowserKind, ResultsBrowserLayout, ResultsBrowserOrder};
use super::results_rows::result_quantity_kind_label;
use super::results_tree::{
    BrowserQuery, InstanceNode, SignalKind, TreeItem, build_tree, is_generated_node,
    split_hierarchy,
};
use crate::state::{AnalysisResult, RetainedTrace};
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, analysis_default_unit, browser_signal_unit,
};

/// Signals a frame copies into a catalogue: a couple of milliseconds of
/// copying, so a large analysis is catalogued over a few frames and none of
/// them is slow.
pub(super) const CATALOGUE_SLICE: usize = 65_536;

/// The most signals a listing is made of on the frame. Matching, sorting and
/// building the tree over more takes longer than a frame has.
const LISTED_ON_FRAME: usize = 16_384;

/// What the browser reads of one analysis's signals, in the order the run
/// wrote them.
#[derive(Debug, Default)]
pub(super) struct SignalCatalogue {
    names: String,
    /// Where each signal's name ends in `names`.
    ends: Vec<usize>,
    /// Each signal's retained unit, by the first signal of each run of
    /// signals that share it.
    units: Vec<(usize, Option<String>)>,
    /// Each signal's sample count, likewise.
    samples: Vec<(usize, usize)>,
    /// The unit the kind facet files a signal with none of its own under.
    kind_default: &'static str,
    /// The unit such a signal's row reads in.
    row_default: &'static str,
    /// The signals the analysis holds; the catalogue is whole once it has
    /// them all.
    expected: usize,
    /// Its internal nodes, which the browser hides unless asked.
    internal: usize,
}

impl SignalCatalogue {
    fn of(analysis: &AnalysisResult) -> Self {
        Self {
            ends: Vec::with_capacity(analysis.waveforms.len()),
            kind_default: analysis_default_unit(analysis.analysis_type),
            row_default: analysis.analysis_type.axis_info().3,
            expected: analysis.waveforms.len(),
            ..Self::default()
        }
    }

    /// Copy up to `count` more of `analysis`'s signals; whether the
    /// catalogue now holds them all.
    fn take(&mut self, analysis: &AnalysisResult, count: usize) -> bool {
        let start = self.len().min(analysis.waveforms.len());
        let end = start.saturating_add(count).min(analysis.waveforms.len());
        for waveform in &analysis.waveforms[start..end] {
            let index = self.ends.len();
            self.internal += usize::from(is_generated_node(&waveform.name));
            self.names.push_str(&waveform.name);
            self.ends.push(self.names.len());
            if self
                .units
                .last()
                .is_none_or(|(_, unit)| *unit != waveform.unit)
            {
                self.units.push((index, waveform.unit.clone()));
            }
            let samples = waveform.y.len();
            if self.samples.last().is_none_or(|(_, held)| *held != samples) {
                self.samples.push((index, samples));
            }
        }
        self.is_whole()
    }

    fn is_whole(&self) -> bool {
        self.len() == self.expected
    }

    pub(super) fn len(&self) -> usize {
        self.ends.len()
    }

    pub(super) fn name(&self, index: usize) -> &str {
        let start = index
            .checked_sub(1)
            .map_or(0, |previous| self.ends[previous]);
        &self.names[start..self.ends[index]]
    }

    fn retained_unit(&self, index: usize) -> Option<&str> {
        run_at(&self.units, index).as_deref()
    }

    pub(super) fn samples(&self, index: usize) -> usize {
        *run_at(&self.samples, index)
    }

    /// The unit signal `index` reads in on its row.
    pub(super) fn unit(&self, index: usize) -> &str {
        browser_signal_unit(
            self.name(index),
            self.retained_unit(index),
            self.row_default,
        )
    }

    /// The unit the kind facet files signal `index` under.
    fn kind_unit(&self, index: usize) -> &str {
        browser_signal_unit(
            self.name(index),
            self.retained_unit(index),
            self.kind_default,
        )
    }

    /// Whether this catalogue was taken of `analysis` as it stands.
    fn is_of(&self, analysis: &AnalysisResult) -> bool {
        self.expected == analysis.waveforms.len()
    }
}

/// The value at `index` of a list kept as runs, each by its first index.
fn run_at<T>(runs: &[(usize, T)], index: usize) -> &T {
    let position = runs.partition_point(|(first, _)| *first <= index);
    &runs[position.saturating_sub(1)].1
}

/// What the query and the facets ask of one analysis's listing.
#[derive(Clone, PartialEq)]
pub(super) struct ListingAsk {
    pub(super) query: BrowserQuery,
    /// Whether the query names the analysis itself, which then lists every
    /// signal the facets let through.
    pub(super) analysis_matches: bool,
    pub(super) kind: ResultsBrowserKind,
    pub(super) show_internal: bool,
    pub(super) order: ResultsBrowserOrder,
    pub(super) layout: ResultsBrowserLayout,
    /// Under the Recent scope, the analysis's recent signals by name, the
    /// most recent first: the scope lists only these, in that order.
    pub(super) recent: Option<Vec<String>>,
    /// The analysis's other results, as its tree lists them beside the
    /// signals, in the order the browser holds them.
    pub(super) artifacts: Vec<ListedArtifact>,
}

/// An artifact as an analysis's tree reads it.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct ListedArtifact {
    /// Its name in the design's hierarchy.
    pub(super) path: String,
    pub(super) unit: String,
    /// Where its kind lists among its siblings.
    pub(super) kind_rank: u8,
}

/// The signals of one analysis the query and the facets let through, in the
/// order they list, and the tree over them.
#[derive(Debug)]
pub(super) struct SignalListing {
    catalogue: Arc<SignalCatalogue>,
    /// Indices into the catalogue.
    signals: Vec<usize>,
    tree: Option<InstanceNode>,
}

impl SignalListing {
    fn of(catalogue: Arc<SignalCatalogue>, ask: &ListingAsk) -> Self {
        let query = &ask.query;
        // Every signal is a vector, so a query for the word lists them all.
        let names_all = ask.analysis_matches || query.matches_word("vector");
        let recent = ask.recent.as_ref().map(|names| {
            let mut ranks = HashMap::with_capacity(names.len());
            for (rank, name) in names.iter().enumerate() {
                ranks.entry(name.as_str()).or_insert(rank);
            }
            ranks
        });
        let mut signals = Vec::new();
        for index in 0..catalogue.len() {
            let name = catalogue.name(index);
            if !ask.show_internal && catalogue.internal > 0 && is_generated_node(name) {
                continue;
            }
            // The unit the signal actually reads in decides its kind.
            // Treating "not a current" as a voltage filed noise densities
            // and decibel magnitudes under Voltage.
            if !ask.kind.admits(catalogue.kind_unit(index)) {
                continue;
            }
            if !names_all {
                let unit = catalogue.unit(index);
                if !query.matches_name(name)
                    && !query.matches_word(unit)
                    && !query.matches_word(result_quantity_kind_label(name, unit))
                {
                    continue;
                }
            }
            if recent
                .as_ref()
                .is_some_and(|recent| !recent.contains_key(name))
            {
                continue;
            }
            signals.push(index);
        }
        if let Some(recent) = &recent {
            // Recent means recency order; the sort facet yields.
            signals.sort_by_key(|index| recent.get(catalogue.name(*index)).copied());
        } else if ask.layout == ResultsBrowserLayout::Flat
            && ask.order != ResultsBrowserOrder::RunOrder
        {
            signals.sort_by(|left, right| {
                ask.order
                    .compare(quantity(&catalogue, *left), quantity(&catalogue, *right))
            });
        }
        // The tree lists what the filters let through, so every branch it
        // holds leads to a match.
        let tree = (ask.layout == ResultsBrowserLayout::Tree).then(|| {
            let mut tree = build_tree(
                signals
                    .iter()
                    .enumerate()
                    .map(|(position, index)| (TreeItem::Signal(position), catalogue.name(*index)))
                    .chain(ask.artifacts.iter().enumerate().map(|(index, artifact)| {
                        (TreeItem::Artifact(index), artifact.path.as_str())
                    })),
            );
            // Recent keeps recency order and run order the order the run
            // wrote them in; otherwise each level reads by the order's group
            // and then by name: by kind, as a hierarchy browser's do, by
            // unit, or by sample count.
            if recent.is_none() && ask.order != ResultsBrowserOrder::RunOrder {
                let group = ask.order.groups(
                    signals
                        .iter()
                        .map(|index| (catalogue.unit(*index), catalogue.samples(*index))),
                    ask.artifacts.iter().map(|artifact| artifact.unit.as_str()),
                );
                tree.sort(&|item| match item {
                    TreeItem::Signal(position) => {
                        let index = signals[position];
                        let name = catalogue.name(index);
                        let unit = catalogue.unit(index);
                        let kind =
                            SignalKind::of_waveform(name, result_quantity_kind_label(name, unit));
                        (
                            group.of_signal(unit, catalogue.samples(index), kind),
                            split_hierarchy(name).leaf,
                        )
                    }
                    TreeItem::Artifact(index) => {
                        let artifact = &ask.artifacts[index];
                        (
                            group.of_artifact(&artifact.unit, artifact.kind_rank),
                            split_hierarchy(&artifact.path).leaf,
                        )
                    }
                });
            }
            tree
        });
        Self {
            catalogue,
            signals,
            tree,
        }
    }
}

/// What the browser's orders read of catalogue signal `index`.
fn quantity(catalogue: &SignalCatalogue, index: usize) -> Quantity<'_> {
    Quantity {
        name: catalogue.name(index),
        unit: catalogue.unit(index),
        samples: catalogue.samples(index),
    }
}

/// What the browser's orders read of a signal.
#[derive(Debug, Clone, Copy)]
pub(super) struct Quantity<'a> {
    pub(super) name: &'a str,
    pub(super) unit: &'a str,
    pub(super) samples: usize,
}

/// One analysis's listed signals as the browser's rows read them: what the
/// listing holds, or nothing yet while the first listing is made.
#[derive(Debug, Clone, Default)]
pub(super) struct ListedSignals {
    listing: Option<Arc<SignalListing>>,
    /// Whether a listing for what the browser now asks is still being made:
    /// the one held, if any, answers an earlier ask.
    pending: bool,
}

impl ListedSignals {
    pub(super) fn len(&self) -> usize {
        self.listing
            .as_ref()
            .map_or(0, |listing| listing.signals.len())
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether the listing the browser asked for is still being made.
    pub(super) fn pending(&self) -> bool {
        self.pending
    }

    /// The instance tree over the listed signals, in the tree layout.
    pub(super) fn tree(&self) -> Option<&InstanceNode> {
        self.listing.as_ref()?.tree.as_ref()
    }

    fn index(&self, position: usize) -> (&SignalCatalogue, usize) {
        let listing = self
            .listing
            .as_ref()
            .expect("a listed signal belongs to a listing");
        (&listing.catalogue, listing.signals[position])
    }

    /// The name of the listed signal at `position`.
    pub(super) fn name(&self, position: usize) -> &str {
        let (catalogue, index) = self.index(position);
        catalogue.name(index)
    }

    /// The index among its analysis's waveforms of the listed signal at
    /// `position`.
    pub(super) fn waveform_index(&self, position: usize) -> usize {
        self.index(position).1
    }

    /// The identity of the listed signal at `position` of `analysis`.
    pub(super) fn identity(
        &self,
        position: usize,
        analysis: AnalysisPresentationKey,
    ) -> SourceWaveformPresentationKey {
        SourceWaveformPresentationKey::new(analysis, self.name(position))
    }

    /// The row of the listed signal at `position` of `analysis`, with its
    /// colour and value read from `waveform`, the signal as the run holds it
    /// now, when the browser still finds it there.
    pub(super) fn signal(
        &self,
        position: usize,
        analysis: AnalysisPresentationKey,
        waveform: Option<&RetainedTrace>,
    ) -> ResultSignal {
        #[cfg(test)]
        ROWS_MADE.with(|made| made.set(made.get() + 1));
        let (catalogue, index) = self.index(position);
        let name = catalogue.name(index);
        let unit = catalogue.unit(index);
        let samples = catalogue.samples(index);
        let waveform = waveform.filter(|waveform| waveform.name == name);
        ResultSignal {
            kind: SignalKind::of_waveform(name, result_quantity_kind_label(name, unit)),
            waveform_index: index,
            identity: SourceWaveformPresentationKey::new(analysis, name),
            name: name.to_owned(),
            color: waveform
                .map(|waveform| waveform.color.clone())
                .unwrap_or_default(),
            // Nothing: every row of a waveform list is a vector, the unit
            // rides the value, and the sample count is not something a
            // reader acts on from here.
            meta: String::new(),
            // A swept quantity has no one value: the last sample is the end
            // of the sweep, and read against the row's name it passes for
            // "the value of V(out)". A solved scalar does have one, and
            // states it.
            value: (samples == 1)
                .then(|| {
                    waveform.and_then(|waveform| waveform.y.samples(..).find(|v| v.is_finite()))
                })
                .flatten()
                .map(|value| crate::ui::plot::fmt_si_significant(value, unit, 4)),
        }
    }
}

#[cfg(test)]
thread_local! {
    /// Signal rows made, each for a row on screen.
    pub(super) static ROWS_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Signals copied into a catalogue.
    pub(super) static SIGNALS_CATALOGUED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// A catalogue, whole or still being copied.
#[derive(Debug)]
enum Catalogue {
    Copying(SignalCatalogue),
    Whole(Arc<SignalCatalogue>),
}

/// The listing an analysis shows, and the one being made away from the
/// frame for what the browser asks now.
#[derive(Default)]
struct Listings {
    shown: Option<(ListingAsk, Arc<SignalListing>)>,
    making: Option<Making>,
}

/// A listing being made away from the frame: what it answers, and where the
/// thread making it leaves it.
struct Making {
    ask: ListingAsk,
    catalogue: Arc<SignalCatalogue>,
    made: Arc<Mutex<Option<SignalListing>>>,
}

/// Every analysis's catalogue and listings, kept between frames.
#[derive(Default)]
pub(super) struct BrowserCatalogues {
    catalogues: HashMap<AnalysisPresentationKey, Catalogue>,
    listings: HashMap<AnalysisPresentationKey, Listings>,
    /// Moves whenever a catalogue is whole or a listing made away from the
    /// frame arrives, so the browser builds its rows again.
    generation: u64,
}

type Shared = Arc<Mutex<BrowserCatalogues>>;

fn shared(ctx: &egui::Context) -> Shared {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<Shared>(egui::Id::new("rspice.results.browser.catalogues"))
            .clone()
    })
}

/// Copy the next slice of each catalogue being copied, take in each listing
/// made away from the frame, and let go of what belongs to an analysis no
/// longer retained. Says the generation of what the browser lists, which
/// moves whenever any of it changed.
pub(super) fn advance<'a>(
    ctx: &egui::Context,
    analyses: impl IntoIterator<Item = (AnalysisPresentationKey, &'a AnalysisResult)>,
) -> u64 {
    let shared = shared(ctx);
    let Ok(mut held) = shared.lock() else {
        return 0;
    };
    let held = &mut *held;
    let mut retained = HashSet::new();
    let mut moved = false;
    let mut unfinished = false;
    for (key, analysis) in analyses {
        retained.insert(key);
        if let Some(Catalogue::Copying(catalogue)) = held.catalogues.get_mut(&key) {
            if !catalogue.is_of(analysis) {
                *catalogue = SignalCatalogue::of(analysis);
            }
            #[cfg(test)]
            let before = catalogue.len();
            let whole = catalogue.take(analysis, CATALOGUE_SLICE);
            #[cfg(test)]
            SIGNALS_CATALOGUED.with(|copied| copied.set(copied.get() + catalogue.len() - before));
            if whole {
                let whole = std::mem::take(catalogue);
                held.catalogues
                    .insert(key, Catalogue::Whole(Arc::new(whole)));
                moved = true;
            } else {
                unfinished = true;
            }
        }
    }
    for listings in held.listings.values_mut() {
        let Some(making) = &listings.making else {
            continue;
        };
        let made = making.made.lock().ok().and_then(|mut made| made.take());
        match made {
            Some(listing) => {
                let making = listings.making.take().expect("a listing being made");
                release(listings.shown.replace((making.ask, Arc::new(listing))));
                moved = true;
            }
            None => unfinished = true,
        }
    }
    let gone = held
        .catalogues
        .keys()
        .filter(|key| !retained.contains(*key))
        .copied()
        .collect::<Vec<_>>();
    for key in gone {
        held.catalogues.remove(&key);
        if let Some(listings) = held.listings.remove(&key) {
            release(listings.shown);
        }
    }
    if moved {
        held.generation = held.generation.wrapping_add(1);
    }
    if unfinished {
        ctx.request_repaint();
    }
    held.generation
}

/// The listing of `analysis` for `ask`: the one made for it, or while that
/// is still being made, the one the analysis showed before. A small analysis
/// is catalogued and listed on the frame; a large one is catalogued a slice a
/// frame by [`advance`] and listed away from the frame.
pub(super) fn listed(
    ctx: &egui::Context,
    key: AnalysisPresentationKey,
    analysis: &AnalysisResult,
    ask: ListingAsk,
) -> ListedSignals {
    let shared = shared(ctx);
    let Ok(mut held) = shared.lock() else {
        return ListedSignals::default();
    };
    let held = &mut *held;
    let catalogue = catalogue(held, ctx, key, analysis);
    let listings = held.listings.entry(key).or_default();
    let shown = |listings: &Listings, pending| ListedSignals {
        listing: listings
            .shown
            .as_ref()
            .map(|(_, listing)| Arc::clone(listing)),
        pending,
    };
    let Some(catalogue) = catalogue else {
        return shown(listings, true);
    };
    if let Some((shown_ask, listing)) = &listings.shown
        && *shown_ask == ask
        && Arc::ptr_eq(&listing.catalogue, &catalogue)
    {
        return shown(listings, false);
    }
    if listings
        .making
        .as_ref()
        .is_some_and(|making| making.ask == ask && Arc::ptr_eq(&making.catalogue, &catalogue))
    {
        return shown(listings, true);
    }
    if catalogue.len() <= LISTED_ON_FRAME {
        let listing = Arc::new(SignalListing::of(catalogue, &ask));
        listings.making = None;
        release(listings.shown.replace((ask, listing)));
        return shown(listings, false);
    }
    let made = Arc::new(Mutex::new(None));
    listings.making = Some(Making {
        ask: ask.clone(),
        catalogue: Arc::clone(&catalogue),
        made: Arc::clone(&made),
    });
    let repaint = ctx.clone();
    rspice_app_services::workbench::platform::spawn_or_inline(move || {
        let listing = SignalListing::of(catalogue, &ask);
        if let Ok(mut made) = made.lock() {
            *made = Some(listing);
        }
        repaint.request_repaint();
    });
    shown(listings, true)
}

/// The signals of `analysis` the browser lists before its filters: all of
/// them, less its internal nodes unless they are shown, once its catalogue
/// has counted them.
pub(super) fn loaded(
    ctx: &egui::Context,
    key: AnalysisPresentationKey,
    analysis: &AnalysisResult,
    show_internal: bool,
) -> usize {
    if show_internal {
        return analysis.waveforms.len();
    }
    let shared = shared(ctx);
    let internal = shared.lock().ok().and_then(|mut held| {
        catalogue(&mut held, ctx, key, analysis).map(|catalogue| catalogue.internal)
    });
    analysis.waveforms.len() - internal.unwrap_or(0)
}

/// The whole catalogue of `analysis`, once there is one. A small analysis is
/// catalogued on the frame that first asks; a large one is put to
/// [`advance`], which copies it a slice a frame.
fn catalogue(
    held: &mut BrowserCatalogues,
    ctx: &egui::Context,
    key: AnalysisPresentationKey,
    analysis: &AnalysisResult,
) -> Option<Arc<SignalCatalogue>> {
    match held.catalogues.get(&key) {
        Some(Catalogue::Whole(catalogue)) if catalogue.is_of(analysis) => {
            return Some(Arc::clone(catalogue));
        }
        Some(Catalogue::Copying(_)) => return None,
        _ => {}
    }
    let mut catalogue = SignalCatalogue::of(analysis);
    if analysis.waveforms.len() > CATALOGUE_SLICE {
        held.catalogues.insert(key, Catalogue::Copying(catalogue));
        ctx.request_repaint();
        return None;
    }
    #[cfg(test)]
    SIGNALS_CATALOGUED.with(|copied| copied.set(copied.get() + analysis.waveforms.len()));
    catalogue.take(analysis, CATALOGUE_SLICE);
    let catalogue = Arc::new(catalogue);
    held.catalogues
        .insert(key, Catalogue::Whole(Arc::clone(&catalogue)));
    Some(catalogue)
}

/// Let go of a listing no longer shown, a large one where its size costs no
/// frame: a large analysis's tree is a great many allocations to free.
fn release(shown: Option<(ListingAsk, Arc<SignalListing>)>) {
    if shown
        .as_ref()
        .is_some_and(|(_, listing)| listing.signals.len() > LISTED_ON_FRAME)
    {
        rspice_app_services::workbench::platform::spawn_or_inline(move || drop(shown));
    }
}

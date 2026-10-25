//! Cross-probe mapping.
//!
//! Ties a waveform back to the schematic object that produced it, so
//! selecting a trace highlights its net and probing a net finds its trace.
//!
//! A net name is only half an address. `n1` inside `/X1` and `n1` inside `/X2`
//! are two different nodes, and the engine knows neither of them by that name:
//! it solved `x1.n1` and `x2.n1`. Every mapping therefore carries the
//! occurrence its signals are read in, and the engine spelling of each one, so
//! a probe placed on a descended sheet asks for the node its author pointed at
//! rather than for whichever node of that name the flattened deck resolves
//! first.

use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock};

use super::*;
use crate::state::{CellViewRef, InstancePath, SheetId};
use rspice_simulation::netlist_gen::{SheetNetGeometries, SheetNetGeometry};

/// What a sheet the run captured nothing for reads: no nets at all.
static NO_GEOMETRY: LazyLock<SheetNetGeometry> = LazyLock::new(SheetNetGeometry::default);

/// Bidirectional mapping between schematic grid points and SPICE net names.
///
/// This enables professional-grade cross-probing between schematic and waveform viewer:
/// - Click on schematic wire → find corresponding waveform
/// - Click on waveform → highlight corresponding wire on schematic
///
/// The mapping is populated during netlist generation and persists until the next
/// simulation run or schematic modification.
#[derive(Debug, Clone, Default)]
pub struct CrossProbeMapping {
    /// Exact source document that owns the retained point coordinates.
    pub source_reference: Option<CellViewRef>,

    /// The occurrence whose scope the signals on this map are read in.
    ///
    /// `None` is the design root, where a signal is named by its leaf alone.
    pub occurrence: Option<InstancePath>,

    /// The run's net geometry, sheet by sheet, each in the coordinates its
    /// own sheet draws it in; `None` keys a cell view with no sheet catalog.
    ///
    /// Read one sheet at a time through [`Self::on_sheet`]. Two sheets can
    /// draw different nets at the same coordinates, so a point means nothing
    /// without the sheet it was read on. Shared, because every occurrence of
    /// the captured master reads the same geometry under its own names.
    sheets: Arc<SheetNetGeometries>,

    /// Engine spelling of every signal on this map, keyed by the folded local
    /// name.
    ///
    /// Derived, never authored: the engine flattens the hierarchy, and this is
    /// the one place that flattening is written down for a probe to read back.
    /// A signal the engine grammar cannot name is absent rather than guessed.
    pub engine_names: HashMap<String, String>,

    /// Version counter - incremented when mapping is updated
    /// Used to detect when probe cache needs refresh
    pub version: u64,

    /// Exact schematic topology revision from which this map was generated.
    /// Consumers must reject the map after any structural edit.
    pub source_topology_version: Option<u64>,
}

/// Does `p` lie exactly on segment `(a, b)` (inclusive)?
///
/// Cross-probing shares the schematic wire primitive's canonical integer
/// geometry so diagonal and extreme-coordinate conductors resolve exactly.
fn on_segment(p: Point, a: Point, b: Point) -> bool {
    crate::state::WireSegment::new(a, b).contains_point(p)
}

/// The engine's name for every net that has geometry on any sheet, read at
/// `occurrence`.
///
/// A net with no retained point is skipped for the same reason a probe cannot
/// land on one: there is nothing on the drawing to name.
fn engine_names_at(
    occurrence: &InstancePath,
    sheets: &SheetNetGeometries,
) -> HashMap<String, String> {
    sheets
        .values()
        .flat_map(|sheet| &sheet.net_to_points)
        .filter(|(_, points)| !points.is_empty())
        .filter_map(|(name, _)| {
            let engine = occurrence.engine_name_for(name).ok()?;
            Some((name.to_ascii_lowercase(), engine))
        })
        .collect()
}

impl CrossProbeMapping {
    /// Create a new empty mapping
    #[cfg(test)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Install the geometry one run captured.
    ///
    /// Generation runs at the design root, so the map it produces is the
    /// root's: its signals are named by their leaves and its engine spellings
    /// are those leaves. [`CrossProbeIndex`] is what re-reads the same geometry
    /// at an occurrence below the root.
    pub fn install(
        &mut self,
        source_reference: CellViewRef,
        sheets: SheetNetGeometries,
        source_topology_version: u64,
    ) {
        self.source_reference = Some(source_reference);
        self.occurrence = None;
        self.engine_names = engine_names_at(&InstancePath::root(), &sheets);
        self.sheets = Arc::new(sheets);
        self.source_topology_version = Some(source_topology_version);
        self.version += 1;
    }

    /// [`Self::install`] for a cell view with no sheet catalog, whose one
    /// drawing is the whole geometry.
    #[cfg(test)]
    pub fn update(
        &mut self,
        source_reference: CellViewRef,
        point_to_net: HashMap<Point, String>,
        net_to_points: HashMap<String, Vec<Point>>,
        net_segments: HashMap<String, Vec<(Point, Point)>>,
        source_topology_version: u64,
    ) {
        self.install(
            source_reference,
            HashMap::from([(
                None,
                SheetNetGeometry {
                    point_to_net,
                    net_to_points,
                    net_segments,
                },
            )]),
            source_topology_version,
        );
    }

    /// The geometry one sheet draws, in that sheet's coordinates; `None` is
    /// a cell view with no sheet catalog. A sheet the run captured nothing
    /// for draws no nets.
    pub fn on_sheet(&self, sheet: Option<SheetId>) -> &SheetNetGeometry {
        self.sheets.get(&sheet).unwrap_or(&NO_GEOMETRY)
    }

    /// The net under a drawn point of `sheet`.
    ///
    /// Node points answer from the hash; anything else scans the net
    /// segment lists (probe clicks are rare — this is not a frame path).
    /// Returns None if the point is not on a net (e.g., empty space).
    pub fn net_at(&self, sheet: Option<SheetId>, point: Point) -> Option<&String> {
        let geometry = self.on_sheet(sheet);
        if let Some(name) = geometry.point_to_net.get(&point) {
            return Some(name);
        }
        geometry.net_segments.iter().find_map(|(name, segments)| {
            segments
                .iter()
                .any(|&(a, b)| on_segment(point, a, b))
                .then_some(name)
        })
    }

    /// Resolve a net only when both the owning cell/view and topology receipt
    /// match the currently displayed schematic.
    pub fn net_at_in(
        &self,
        reference: &CellViewRef,
        topology_version: u64,
        sheet: Option<SheetId>,
        point: Point,
    ) -> Option<&String> {
        self.is_current_for(reference, topology_version)
            .then(|| self.net_at(sheet, point))
            .flatten()
    }

    pub fn is_current_for(&self, reference: &CellViewRef, topology_version: u64) -> bool {
        self.is_captured_from(reference) && self.source_topology_version == Some(topology_version)
    }

    /// Whether this map's geometry was captured from `reference`.
    pub fn is_captured_from(&self, reference: &CellViewRef) -> bool {
        self.source_reference
            .as_ref()
            .is_some_and(|source| source.key().eq_ignore_ascii_case(&reference.key()))
    }

    /// Check if mapping is populated
    pub fn is_populated(&self) -> bool {
        self.sheets
            .values()
            .any(|sheet| !sheet.point_to_net.is_empty())
    }

    /// The engine's name for one local signal on this map.
    pub fn engine_name(&self, leaf: &str) -> Option<&str> {
        self.engine_names
            .get(&leaf.to_ascii_lowercase())
            .map(String::as_str)
    }

    /// The same geometry read at `occurrence`.
    ///
    /// Two instances of one master share every point on the drawing and not
    /// one signal name, so the points are shared and the engine spellings are
    /// derived afresh from the occurrence.
    fn at_occurrence(&self, occurrence: &InstancePath) -> Self {
        Self {
            occurrence: (!occurrence.is_root()).then(|| occurrence.clone()),
            engine_names: engine_names_at(occurrence, &self.sheets),
            ..self.clone()
        }
    }
}

/// Every occurrence one run emitted, paired with the cross-probe geometry that
/// answers for it.
///
/// The run receipt names each occurrence and the master emitted for it; the
/// retained map holds one master's geometry. Joining the two gives every
/// occurrence its own reading of the drawing — the same points, its own engine
/// names. An occurrence of some other master is still listed, because a
/// surface asking "what is visible here" needs an answer for every occurrence
/// the run emitted, but it carries no geometry: none was captured for it, and
/// borrowing another master's points would place probes on the wrong drawing.
#[derive(Debug, Clone, Default)]
pub struct CrossProbeIndex {
    by_occurrence: BTreeMap<String, Vec<CrossProbeMapping>>,
}

impl CrossProbeIndex {
    /// Join the receipt's occurrence map onto one captured drawing.
    ///
    /// A row whose occurrence has no engine spelling is skipped rather than
    /// guessed at — the receipt records the empty prefix exactly so that the
    /// map can never be read backwards through a name the deck never used.
    pub fn from_receipt(receipt: &PreparedRunReceipt, captured: &CrossProbeMapping) -> Self {
        let root = InstancePath::root();
        let mut by_occurrence =
            BTreeMap::from([(root.fold_key(), vec![captured.at_occurrence(&root)])]);
        for row in receipt.hierarchy_map() {
            if row.engine_prefix().is_empty() {
                continue;
            }
            let Ok(occurrence) = InstancePath::parse(row.occurrence()) else {
                continue;
            };
            let mapping = if captured.is_captured_from(row.master_reference()) {
                captured.at_occurrence(&occurrence)
            } else {
                CrossProbeMapping {
                    source_reference: Some(row.master_reference().clone()),
                    occurrence: Some(occurrence.clone()),
                    ..CrossProbeMapping::default()
                }
            };
            by_occurrence
                .entry(occurrence.fold_key())
                .or_default()
                .push(mapping);
        }
        Self { by_occurrence }
    }

    /// The mappings visible for one occurrence, and no other occurrence's.
    pub fn for_occurrence(&self, occurrence: &InstancePath) -> &[CrossProbeMapping] {
        self.by_occurrence
            .get(&occurrence.fold_key())
            .map_or(&[], Vec::as_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn amp_reference() -> CellViewRef {
        CellViewRef::new("user", "amp", "schematic")
    }

    /// One sealed receipt whose hierarchy map names `rows`.
    fn receipt_with(rows: Vec<HierarchyMapRow>) -> PreparedRunReceipt {
        PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(0x31),
            source_content_digest: digest(0x32),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    digest(0x34),
                )
                .expect("valid task receipt"),
            ],
        })
        .expect("valid plan receipt")
        .with_hierarchy_map(rows)
        .expect("distinct occurrences seal")
    }

    /// The `amp` drawing with one conductor named `n1`.
    fn amp_map() -> CrossProbeMapping {
        let point = Point::new(10, 20);
        let mut mapping = CrossProbeMapping::new();
        mapping.update(
            amp_reference(),
            HashMap::from([(point, "n1".to_owned())]),
            HashMap::from([("n1".to_owned(), vec![point])]),
            HashMap::new(),
            5,
        );
        mapping
    }

    #[test]
    fn lookup_requires_exact_source_document_and_topology_receipt() {
        let source = CellViewRef::new("user", "top", "schematic");
        let other = CellViewRef::new("user", "child", "schematic");
        let point = Point::new(10, 20);
        let mut mapping = CrossProbeMapping::new();
        mapping.update(
            source.clone(),
            HashMap::from([(point, "OUT".to_owned())]),
            HashMap::from([("OUT".to_owned(), vec![point])]),
            HashMap::new(),
            7,
        );

        assert_eq!(
            mapping
                .net_at_in(&source, 7, None, point)
                .map(String::as_str),
            Some("OUT")
        );
        assert!(mapping.net_at_in(&other, 7, None, point).is_none());
        assert!(mapping.net_at_in(&source, 8, None, point).is_none());
    }

    /// A point names a net only on the sheet it was read on: two sheets may
    /// draw different nets at the same coordinates.
    #[test]
    fn a_drawn_point_resolves_only_on_its_own_sheet() {
        use crate::state::{SheetCatalog, SheetDefinition, SheetPortPolicy, SheetTemplate};

        let mut catalog = SheetCatalog::default();
        let mut sheets = Vec::new();
        for page in [1, 2] {
            sheets.push(
                catalog
                    .create_sheet(
                        SheetDefinition {
                            name: format!("Sheet {page}"),
                            template: SheetTemplate::AnalogSchematic,
                            port_policy: SheetPortPolicy::TypedOffSheetPorts,
                            explicit_page_number: Some(page),
                        },
                        sheets.last().copied(),
                    )
                    .expect("sheet"),
            );
        }
        let source = CellViewRef::new("user", "top", "schematic");
        let drawn = Point::new(100, 0);
        let projected = Point::new(1_000_100, 0);
        let mut mapping = CrossProbeMapping::new();
        mapping.install(
            source.clone(),
            rspice_simulation::netlist_gen::net_geometry_by_sheet(
                Some(&catalog),
                HashMap::from([(drawn, "VOUT".to_owned()), (projected, "BIAS".to_owned())]),
                HashMap::from([
                    ("VOUT".to_owned(), vec![drawn]),
                    ("BIAS".to_owned(), vec![projected]),
                ]),
                HashMap::new(),
            ),
            3,
        );

        let net = |sheet| mapping.net_at_in(&source, 3, Some(sheet), drawn).cloned();
        assert_eq!(net(sheets[0]).as_deref(), Some("VOUT"));
        assert_eq!(net(sheets[1]).as_deref(), Some("BIAS"));
        assert!(mapping.on_sheet(None).net_to_points.is_empty());
        assert_eq!(mapping.engine_name("bias"), Some("bias"));
    }

    #[test]
    fn diagonal_segment_interior_resolves_with_canonical_wire_geometry() {
        let source = CellViewRef::new("user", "top", "schematic");
        let start = Point::new(-30, -20);
        let end = Point::new(30, 40);
        let interior = Point::new(0, 10);
        let mut mapping = CrossProbeMapping::new();
        mapping.update(
            source.clone(),
            HashMap::new(),
            HashMap::from([("DIAGONAL".to_owned(), vec![start, end])]),
            HashMap::from([("DIAGONAL".to_owned(), vec![(start, end)])]),
            11,
        );

        assert_eq!(
            mapping
                .net_at_in(&source, 11, None, interior)
                .map(String::as_str),
            Some("DIAGONAL")
        );
        assert!(
            mapping
                .net_at_in(&source, 11, None, Point::new(0, 11))
                .is_none(),
            "a nearby off-segment point must not alias the diagonal conductor"
        );
    }

    #[test]
    fn a_probe_at_the_root_keeps_todays_spelling() {
        let spelling = OccurrenceProbeSpelling::for_leaf(&InstancePath::root(), 'V', "OUT")
            .expect("the root always spells");

        assert_eq!(spelling.display(), "V(OUT)");
        assert_eq!(spelling.engine(), "V(OUT)");

        let map = amp_map();
        assert!(map.occurrence.is_none(), "generation runs at the root");
        assert_eq!(map.engine_name("N1"), Some("n1"));
    }

    #[test]
    fn the_index_partitions_one_master_by_occurrence() {
        let receipt = receipt_with(vec![
            HierarchyMapRow::new("/X1", "amp_1", "X1", amp_reference()).expect("first instance"),
            HierarchyMapRow::new("/X2", "amp_1", "X2", amp_reference()).expect("second instance"),
        ]);
        let index = CrossProbeIndex::from_receipt(&receipt, &amp_map());
        let first = InstancePath::parse("/X1").expect("first path");
        let second = InstancePath::parse("/X2").expect("second path");

        assert_eq!(index.by_occurrence.len(), 3, "two instances and the root");
        assert_eq!(index.for_occurrence(&first).len(), 1);
        assert_eq!(index.for_occurrence(&second).len(), 1);
        assert_eq!(
            index.for_occurrence(&first)[0].engine_name("n1"),
            Some("x1.n1")
        );
        assert_eq!(
            index.for_occurrence(&second)[0].engine_name("n1"),
            Some("x2.n1")
        );
        assert_eq!(
            index.for_occurrence(&first)[0].occurrence.as_ref(),
            Some(&first),
            "a mapping states the occurrence it was read at"
        );
        assert!(
            index
                .for_occurrence(&InstancePath::parse("/X3").expect("absent path"))
                .is_empty()
        );
    }

    #[test]
    fn an_occurrence_engine_name_round_trips_through_the_hierarchy_map() {
        let receipt = receipt_with(vec![
            HierarchyMapRow::new("/X1/X2", "amp_1", "X1.X2", amp_reference()).expect("nested"),
        ]);
        let index = CrossProbeIndex::from_receipt(&receipt, &amp_map());
        let nested = InstancePath::parse("/X1/X2").expect("nested path");
        let row = &receipt.hierarchy_map()[0];

        let engine = index.for_occurrence(&nested)[0]
            .engine_name("n1")
            .expect("the captured master answers for this occurrence");
        assert_eq!(engine, "x1.x2.n1");
        assert_eq!(
            engine,
            format!("{}.n1", row.engine_prefix().to_ascii_lowercase()),
            "the mapping's engine name must be the receipt's prefix and the leaf"
        );
    }

    #[test]
    fn an_occurrence_of_another_master_carries_no_borrowed_geometry() {
        let other = CellViewRef::new("user", "bias", "schematic");
        let receipt = receipt_with(vec![
            HierarchyMapRow::new("/X9", "bias_1", "X9", other.clone()).expect("other master"),
        ]);
        let index = CrossProbeIndex::from_receipt(&receipt, &amp_map());
        let elsewhere = InstancePath::parse("/X9").expect("other path");
        let mapping = &index.for_occurrence(&elsewhere)[0];

        assert_eq!(mapping.source_reference.as_ref(), Some(&other));
        assert!(!mapping.is_populated());
        assert!(mapping.engine_name("n1").is_none());
    }

    #[test]
    fn the_root_mapping_is_the_captured_drawing() {
        let index = CrossProbeIndex::from_receipt(&receipt_with(Vec::new()), &amp_map());
        let root = index.for_occurrence(&InstancePath::root());

        assert_eq!(root.len(), 1);
        assert!(root[0].occurrence.is_none());
        assert_eq!(
            root[0].net_at(None, Point::new(10, 20)).map(String::as_str),
            Some("n1")
        );
        assert_eq!(root[0].engine_name("n1"), Some("n1"));
    }
}

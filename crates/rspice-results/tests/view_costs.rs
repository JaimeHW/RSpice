//! What a view costs at size: the memory its undo steps hold, and the time
//! one large transaction takes. Allocations are counted per thread, so each
//! test reads only its own.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::time::Instant;

use rspice_results::analysis_type::AnalysisType;
use rspice_results::visualization_document::{
    AxisOrientation, AxisScale, ColumnRole, DocumentEdit, EntityRef, NewAxis, NewTrace, PaneKind,
    ResultDocumentTracking, RunSourceAnalysis, SourceColumn, SourceDataset, SourceRow, TraceColor,
    TypedValue, ValueType, VisualizationDocument, WorkingPane, WorkingViewSource,
};
use rspice_vocabulary::product::{
    AnalysisInstanceId, ContentDigest, DatasetBinding, DatasetId, SimulationPlanId,
};

thread_local! {
    static LIVE: Cell<isize> = const { Cell::new(0) };
}

struct Counting;

fn count(bytes: isize) {
    let _ = LIVE.try_with(|live| live.set(live.get() + bytes));
}

fn signed(bytes: usize) -> isize {
    isize::try_from(bytes).unwrap_or(isize::MAX)
}

// SAFETY: every operation is forwarded unchanged to System; the count is a
// thread-local Cell, which never allocates.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count(signed(layout.size()));
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count(signed(layout.size()));
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        count(-signed(layout.size()));
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count(signed(size) - signed(layout.size()));
        unsafe { System.realloc(ptr, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// The bytes this thread holds now.
fn live() -> isize {
    LIVE.with(Cell::get)
}

/// A run's view plotting `signals` signals of its one analysis.
fn view(signals: usize) -> VisualizationDocument {
    let analysis = AnalysisInstanceId::new();
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([1; 32]));
    let source = SourceDataset::run_reference(
        binding,
        "Run",
        vec![RunSourceAnalysis::new(analysis, AnalysisType::Transient)],
    )
    .expect("a run reference");
    let mut document = VisualizationDocument::new_working_view(
        WorkingViewSource::Plan {
            plan_id: SimulationPlanId::new(),
        },
        source,
        ResultDocumentTracking::pinned(),
        vec![WorkingPane {
            title: "Transient".to_owned(),
            kind: PaneKind::Cartesian,
            viewer_id: "viewer-waveform".to_owned(),
            analysis_id: analysis,
            plotted: Vec::new(),
        }],
    )
    .expect("a view");
    let pane_id = document.pane_of_analysis(analysis).expect("its pane").id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: (0..signals).map(|index| format!("V(n{index})")).collect(),
                plotted: true,
            }],
        )
        .expect("every signal plots");
    document
}

/// A small edit of the trace at `index`: its colour, or whether it shows.
fn small_edit(document: &VisualizationDocument, index: usize) -> DocumentEdit {
    let trace_id = document.traces()[index % document.traces().len()].id;
    if index % 2 == 0 {
        DocumentEdit::SetTraceColor {
            trace_id,
            color: Some(TraceColor::from_rgb(1, 2, 3)),
        }
    } else {
        DocumentEdit::SetTraceVisibility {
            trace_id,
            visible: false,
        }
    }
}

/// Two hundred small edits of a view of ten thousand traces keep undo steps
/// that hold a small part of one view, not two hundred views, and undoing
/// them all takes every edit back.
#[test]
fn undo_of_small_edits_to_a_large_view_holds_what_they_changed() {
    const TRACES: usize = 10_000;
    const EDITS: usize = 200;
    let mut document = view(TRACES);
    let before = live();
    let copy = document.clone();
    let one_view = live() - before;
    drop(copy);
    // A commit leaves the view's lists sized to what they hold; take that
    // one-off shrink with an edit that changes nothing before counting.
    let shown = DocumentEdit::SetTraceVisibility {
        trace_id: document.traces()[0].id,
        visible: true,
    };
    document
        .transact(document.revision(), vec![shown])
        .expect("an edit commits");

    let before = live();
    let mut steps = Vec::with_capacity(EDITS);
    for edit in 0..EDITS {
        let edit = small_edit(&document, edit * 499);
        let (_, step) = document
            .transact_with_undo(document.revision(), vec![edit])
            .expect("a small edit commits");
        steps.push(step);
    }
    let held = live() - before;
    println!("one view of {TRACES} traces: {one_view} bytes; undo of {EDITS} edits: {held} bytes");
    assert!(
        held < one_view,
        "{EDITS} steps hold {held} bytes, a view {one_view}"
    );

    let mut redo = Vec::with_capacity(EDITS);
    while let Some(step) = steps.pop() {
        let (_, reverse) = document
            .take_step(document.revision(), step)
            .expect("each step undoes");
        redo.push(reverse);
    }
    assert!(
        document
            .traces()
            .iter()
            .all(|trace| trace.visible && trace.color.is_none()),
        "undo takes every edit back"
    );
}

/// One transaction of a hundred thousand edits, each to another trace of a
/// view of a hundred thousand, commits every edit.
#[test]
fn one_transaction_of_a_hundred_thousand_edits_commits_them_all() {
    const TRACES: usize = 100_000;
    let mut document = view(TRACES);
    let edits: Vec<_> = (0..TRACES)
        .map(|index| small_edit(&document, index))
        .collect();
    let started = Instant::now();
    let receipt = document
        .transact(document.revision(), edits)
        .expect("the transaction commits");
    println!(
        "one transaction of {} edits: {:?}",
        receipt.edit_count,
        started.elapsed()
    );
    let traces = document.traces();
    assert!(traces.iter().step_by(2).all(|trace| trace.color.is_some()));
    assert!(traces.iter().skip(1).step_by(2).all(|trace| !trace.visible));
}

/// Ten thousand markers on a table of a hundred thousand rows are each
/// checked against the table's rows in one transaction, and one between two
/// rows is still refused.
#[test]
fn many_markers_on_a_large_table_are_checked_in_one_transaction() {
    const ROWS: usize = 100_000;
    const MARKERS: usize = 10_000;
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([7; 32]));
    let column = |key: &str, role, unit: &str| {
        SourceColumn::new(key, key, ValueType::Real, role, Some(unit.to_owned())).expect("a column")
    };
    let table = SourceDataset::new(
        binding,
        vec![
            column("time", ColumnRole::Coordinate, "s"),
            column("v(out)", ColumnRole::Signal, "V"),
        ],
        (0..ROWS)
            .map(|row| SourceRow::new(vec![TypedValue::Real(row as f64), TypedValue::Real(1.0)]))
            .collect(),
    )
    .expect("a table");
    let mut document = VisualizationDocument::new("Markers", vec![table]).expect("a document");
    let pane_id = document.panes()[0].id;
    let axis = |label: &str, orientation, unit: &str| {
        DocumentEdit::AddAxis(NewAxis {
            pane_id,
            label: label.to_owned(),
            orientation,
            scale: AxisScale::Linear,
            unit: Some(unit.to_owned()),
            range: None,
        })
    };
    let receipt = document
        .transact(
            document.revision(),
            vec![
                axis("Time", AxisOrientation::Horizontal, "s"),
                axis("V", AxisOrientation::VerticalLeft, "V"),
            ],
        )
        .expect("two axes");
    let axes: Vec<_> = receipt
        .created
        .iter()
        .filter_map(|entity| match entity {
            EntityRef::Axis(id) => Some(*id),
            _ => None,
        })
        .collect();
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(NewTrace {
                pane_id,
                binding,
                signal_key: "v(out)".to_owned(),
                coordinate_key: "time".to_owned(),
                row_predicates: Vec::new(),
                x_axis_id: axes[0],
                y_axis_id: axes[1],
                label: "V(out)".to_owned(),
            })],
        )
        .expect("a trace");
    let trace_id = receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Trace(id) => Some(*id),
            _ => None,
        })
        .expect("the trace");
    let marker = |x: f64| DocumentEdit::AddMarker {
        pane_id,
        trace_id,
        coordinate: TypedValue::Real(x),
        label: format!("M{x}"),
    };
    let edits: Vec<_> = (0..MARKERS)
        .map(|index| marker((index * (ROWS / MARKERS)) as f64))
        .collect();
    let started = Instant::now();
    document
        .transact(document.revision(), edits)
        .expect("every marker is on a row");
    println!(
        "{MARKERS} markers on a {ROWS}-row table in one transaction: {:?}",
        started.elapsed()
    );
    assert_eq!(document.markers().len(), MARKERS);
    assert!(
        document
            .transact(document.revision(), vec![marker(0.5)])
            .is_err(),
        "a marker between two rows is refused"
    );
}

//! What a view costs at size: the memory its undo steps hold, and the time
//! one large transaction takes. Allocations are counted per thread, so each
//! test reads only its own.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::time::Instant;

use rspice_results::analysis_type::AnalysisType;
use rspice_results::visualization_document::{
    AxisOrientation, AxisScale, ColumnRole, CursorId, DocumentEdit, EntityRef, NewAxis, NewTrace,
    PaneKind, ResultDocumentTracking, RunSourceAnalysis, SourceColumn, SourceDataset, SourceRow,
    TraceColor, TypedValue, ValueType, VisualizationDocument, WorkingPane, WorkingViewSource,
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
    if index.is_multiple_of(2) {
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

/// One gesture that plots a hundred thousand more signals on a view of a
/// hundred thousand is one step: undo takes every one of them off and redo
/// puts them back in their order. Taking them all off again hides each.
#[test]
fn a_hundred_thousand_signals_plotted_at_once_are_one_step() {
    const TRACES: usize = 100_000;
    let mut document = view(TRACES);
    let pane_id = document.panes()[0].id;
    let names = |from: usize, to: usize| -> Vec<String> {
        (from..to).map(|index| format!("V(n{index})")).collect()
    };
    // The traces are made away from the document, as a reader's gesture
    // makes them off the frame, and taken in as one edit.
    let preparation = document
        .plot_preparation(pane_id)
        .expect("the pane reads a run");
    let signals = names(TRACES, 2 * TRACES);
    let started = Instant::now();
    let prepared = preparation.prepare(signals).expect("every signal is named");
    println!(
        "making {TRACES} traces away from the view: {:?}",
        started.elapsed()
    );
    let started = Instant::now();
    let (_, step) = document
        .transact_with_undo(
            document.revision(),
            vec![DocumentEdit::PlotPrepared(prepared)],
        )
        .expect("every signal plots");
    println!(
        "taking {TRACES} new traces into a view of {TRACES} traces: {:?}",
        started.elapsed()
    );
    assert_eq!(document.traces().len(), 2 * TRACES);
    let started = Instant::now();
    let (_, redo) = document
        .take_step(document.revision(), step)
        .expect("the gesture undoes");
    println!("its undo: {:?}", started.elapsed());
    assert_eq!(document.traces().len(), TRACES, "undo takes them all off");
    let started = Instant::now();
    document
        .take_step(document.revision(), redo)
        .expect("and redoes");
    println!("its redo: {:?}", started.elapsed());
    let labels: Vec<String> = document
        .traces()
        .iter()
        .map(|trace| trace.label.clone())
        .collect();
    assert_eq!(labels, names(0, 2 * TRACES), "redo puts them back in order");
    let started = Instant::now();
    document
        .transact_with_undo(
            document.revision(),
            vec![DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: names(0, 2 * TRACES),
                plotted: false,
            }],
        )
        .expect("every signal comes off");
    println!("taking {} signals off: {:?}", 2 * TRACES, started.elapsed());
    assert!(document.traces().iter().all(|trace| !trace.visible));
}

/// A view of a hundred thousand traces following its run as the run grows
/// reads the grown run with every trace in one transaction, and draws no
/// signal otherwise: each trace reads its signal as it did.
#[test]
fn a_large_view_following_its_grown_run_draws_no_signal_otherwise() {
    const TRACES: usize = 100_000;
    let mut document = view(TRACES);
    let held = document.datasets()[0].binding();
    let analysis = document.panes()[0]
        .binding
        .expect("the pane reads the run")
        .analysis_id;
    let grown = DatasetBinding::new(held.dataset_id, ContentDigest::from_bytes([2; 32]));
    let next = SourceDataset::run_reference(
        grown,
        "Run",
        vec![RunSourceAnalysis::new(analysis, AnalysisType::Transient)],
    )
    .expect("a run reference");
    let generation = document.generation();
    let started = Instant::now();
    document
        .transact(document.revision(), vec![DocumentEdit::FollowRun { next }])
        .expect("the view follows its run");
    println!(
        "following the grown run with {TRACES} traces: {:?}",
        started.elapsed()
    );
    assert!(document.traces().iter().all(|trace| trace.binding == grown));
    assert_eq!(
        document.signals_changed_since(generation),
        Some(Vec::new()),
        "no signal is drawn otherwise"
    );
}

/// A view of a hundred thousand traces with a cursor on its pane, and the
/// cursor.
fn view_with_cursor(signals: usize) -> (VisualizationDocument, CursorId) {
    let mut document = view(signals);
    let pane_id = document.panes()[0].id;
    let axis_id = document
        .axes()
        .iter()
        .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal)
        .expect("the pane's time axis")
        .id;
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddCursor {
                pane_id,
                axis_id,
                position: TypedValue::Real(0.0),
                label: "C1".to_owned(),
            }],
        )
        .expect("a cursor");
    let cursor_id = receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Cursor(id) => Some(*id),
            _ => None,
        })
        .expect("the cursor");
    (document, cursor_id)
}

/// One small edit of a view of a hundred thousand traces, with the step
/// that undoes it, costs what it changes: a cursor move, a tick of a new
/// signal, an untick, a colour and a removal each leave the rest of the
/// view as it was.
#[test]
fn a_small_edit_of_a_large_view_costs_what_it_changes() {
    const TRACES: usize = 100_000;
    let (mut document, cursor_id) = view_with_cursor(TRACES);
    let pane_id = document.panes()[0].id;
    let seventh = document.traces()[7].id;
    let last = document.traces()[TRACES - 1].id;
    let first = document.traces()[0].id;
    let edits = [
        (
            "a cursor move",
            DocumentEdit::MoveCursor {
                cursor_id,
                position: TypedValue::Real(1e-9),
            },
        ),
        (
            "a tick of a new signal",
            DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: vec!["V(added)".to_owned()],
                plotted: true,
            },
        ),
        (
            "an untick",
            DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: vec!["V(n7)".to_owned()],
                plotted: false,
            },
        ),
        (
            "a colour",
            DocumentEdit::SetTraceColor {
                trace_id: last,
                color: Some(TraceColor::from_rgb(9, 9, 9)),
            },
        ),
        ("a removal", DocumentEdit::Remove(EntityRef::Trace(first))),
        (
            "a dataset attached",
            DocumentEdit::AttachDataset(
                SourceDataset::run_reference(
                    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([2; 32])),
                    "Baseline",
                    vec![RunSourceAnalysis::new(
                        AnalysisInstanceId::new(),
                        AnalysisType::Transient,
                    )],
                )
                .expect("a run reference"),
            ),
        ),
    ];
    for (what, edit) in edits {
        let started = Instant::now();
        document
            .transact_with_undo(document.revision(), vec![edit])
            .expect("a small edit commits");
        println!(
            "{what} on a view of {TRACES} traces: {:?}",
            started.elapsed()
        );
    }
    assert_eq!(
        document.cursors()[0].position,
        TypedValue::Real(1e-9),
        "the cursor moved"
    );
    let traces = document.traces();
    assert_eq!(traces.len(), TRACES, "one trace added, one removed");
    assert_eq!(traces[TRACES - 1].label, "V(added)");
    assert!(
        traces
            .iter()
            .all(|trace| trace.visible == (trace.id != seventh)),
        "only the unticked signal is hidden"
    );
    assert!(traces.iter().all(|trace| trace.id != first));
    assert_eq!(document.datasets().len(), 2, "the dataset is attached");
}

/// Removing half the traces of a view of a hundred thousand in one
/// transaction keeps the other half in their order, and tombstones each
/// removed trace in the order it was removed.
#[test]
fn removing_half_of_a_large_view_in_one_transaction_keeps_the_rest_in_order() {
    const TRACES: usize = 100_000;
    let mut document = view(TRACES);
    let removed: Vec<_> = document
        .traces()
        .iter()
        .step_by(2)
        .map(|trace| trace.id)
        .collect();
    let kept: Vec<_> = document
        .traces()
        .iter()
        .skip(1)
        .step_by(2)
        .map(|trace| trace.id)
        .collect();
    let edits = removed
        .iter()
        .map(|id| DocumentEdit::Remove(EntityRef::Trace(*id)))
        .collect();
    let started = Instant::now();
    let receipt = document
        .transact(document.revision(), edits)
        .expect("the removals commit");
    println!(
        "removing {} of {TRACES} traces in one transaction: {:?}",
        removed.len(),
        started.elapsed()
    );
    let left: Vec<_> = document.traces().iter().map(|trace| trace.id).collect();
    assert_eq!(left, kept, "what is left keeps its order");
    let tombstoned: Vec<_> = receipt
        .tombstoned
        .iter()
        .map(|entity| match entity {
            EntityRef::Trace(id) => *id,
            other => panic!("only traces were removed, not {other:?}"),
        })
        .collect();
    assert_eq!(tombstoned, removed, "each removal is tombstoned in order");
}

/// Taking panes out of a large view in one transaction passes each list once
/// for them all, keeps the order of what is left, and tombstones each pane's
/// entities before the pane, pane by pane, as taking them out one at a time
/// did.
#[test]
fn removing_panes_of_a_large_view_keeps_the_rest_in_order() {
    const PANES: usize = 4;
    const ROUNDS: usize = 10;
    const EACH: usize = 2_500;
    let analyses: Vec<AnalysisInstanceId> = (0..PANES).map(|_| AnalysisInstanceId::new()).collect();
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([1; 32]));
    let source = SourceDataset::run_reference(
        binding,
        "Run",
        analyses
            .iter()
            .map(|analysis| RunSourceAnalysis::new(*analysis, AnalysisType::Transient))
            .collect(),
    )
    .expect("a run reference");
    let mut document = VisualizationDocument::new_working_view(
        WorkingViewSource::Plan {
            plan_id: SimulationPlanId::new(),
        },
        source,
        ResultDocumentTracking::pinned(),
        analyses
            .iter()
            .enumerate()
            .map(|(index, analysis)| WorkingPane {
                title: format!("Transient {index}"),
                kind: PaneKind::Cartesian,
                viewer_id: "viewer-waveform".to_owned(),
                analysis_id: *analysis,
                plotted: Vec::new(),
            })
            .collect(),
    )
    .expect("a view");
    let panes: Vec<_> = analyses
        .iter()
        .map(|analysis| document.pane_of_analysis(*analysis).expect("its pane").id)
        .collect();
    // The panes' traces made a block at a time, each pane's blocks between
    // the others'.
    for round in 0..ROUNDS {
        for &pane_id in &panes {
            document
                .transact(
                    document.revision(),
                    vec![DocumentEdit::SetSignalsPlotted {
                        pane_id,
                        signals: (round * EACH..(round + 1) * EACH)
                            .map(|index| format!("V(n{index})"))
                            .collect(),
                        plotted: true,
                    }],
                )
                .expect("the signals plot");
        }
    }
    let (gone, stays) = ([panes[1], panes[3]], [panes[0], panes[2]]);
    let kept: Vec<_> = document
        .traces()
        .iter()
        .filter(|trace| stays.contains(&trace.pane_id))
        .map(|trace| trace.id)
        .collect();
    let owners: std::collections::HashMap<EntityRef, _> = document
        .traces()
        .iter()
        .map(|trace| (EntityRef::Trace(trace.id), trace.pane_id))
        .chain(
            document
                .axes()
                .iter()
                .map(|axis| (EntityRef::Axis(axis.id), axis.pane_id)),
        )
        .collect();
    let started = Instant::now();
    let receipt = document
        .transact(
            document.revision(),
            gone.iter()
                .map(|pane_id| DocumentEdit::Remove(EntityRef::Pane(*pane_id)))
                .collect(),
        )
        .expect("the panes go");
    println!(
        "taking {} of {PANES} panes out of a view of {} traces: {:?}",
        gone.len(),
        PANES * ROUNDS * EACH,
        started.elapsed()
    );
    let left: Vec<_> = document.traces().iter().map(|trace| trace.id).collect();
    assert_eq!(left, kept, "what is left keeps its order");
    let stood: Vec<_> = document.panes().iter().map(|pane| pane.id).collect();
    assert_eq!(stood, stays, "the panes left keep their order");
    // Pane by pane: its entities, then the pane.
    let mut tombstoned = receipt.tombstoned.iter();
    for pane_id in gone {
        let mut held = 0;
        for entity in tombstoned.by_ref() {
            if *entity == EntityRef::Pane(pane_id) {
                break;
            }
            assert_eq!(
                owners.get(entity),
                Some(&pane_id),
                "{entity:?} is tombstoned with its pane"
            );
            held += 1;
        }
        assert!(
            held >= ROUNDS * EACH,
            "every trace of the pane is tombstoned"
        );
    }
    assert_eq!(tombstoned.next(), None, "nothing else is taken out");
}

/// A transaction refused at its last edit, after editing every trace of a
/// view of a hundred thousand, leaves the view byte-identical.
#[test]
fn a_refused_transaction_leaves_a_large_view_as_it_was() {
    const TRACES: usize = 100_000;
    let (mut document, cursor_id) = view_with_cursor(TRACES);
    let saved = serde_json::to_vec(&document).expect("the view serializes");
    let revision = document.revision();
    let gone = document.traces()[3].id;
    let mut edits = vec![DocumentEdit::MoveCursor {
        cursor_id,
        position: TypedValue::Real(2e-9),
    }];
    edits.extend((0..TRACES).map(|index| small_edit(&document, index)));
    edits.push(DocumentEdit::Remove(EntityRef::Trace(gone)));
    edits.push(DocumentEdit::SetTraceVisibility {
        trace_id: gone,
        visible: true,
    });
    let count = edits.len();
    let started = Instant::now();
    let refused = document.transact(revision, edits);
    println!(
        "a transaction of {count} edits refused at its last: {:?}",
        started.elapsed()
    );
    assert!(refused.is_err(), "an edit of a removed trace is refused");
    assert_eq!(document.revision(), revision);
    assert_eq!(
        serde_json::to_vec(&document).expect("the view serializes"),
        saved,
        "a refused transaction changes nothing"
    );
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

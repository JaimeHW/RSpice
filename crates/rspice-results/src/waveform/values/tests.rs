use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;

/// A stored column in memory that counts its reads and can be made to fail.
struct Column {
    name: &'static str,
    values: Vec<f64>,
    reads: AtomicUsize,
    fail: bool,
}

impl Column {
    fn new(name: &'static str, values: Vec<f64>) -> Arc<Self> {
        Arc::new(Self {
            name,
            values,
            reads: AtomicUsize::new(0),
            fail: false,
        })
    }
}

impl ColumnSource for Column {
    fn name(&self) -> &str {
        self.name
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err("the chunk's checksum does not match".to_owned());
        }
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }
}

fn window(first: u64, rows: u64, lead: WindowLead) -> StoredWindow {
    StoredWindow { first, rows, lead }
}

#[test]
fn a_stored_window_is_read_as_each_reader_asks_and_kept_by_none() {
    let column = Column::new("v(out)", vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    let values = WaveformValues::stored(column.clone(), window(1, 3, WindowLead::None)).unwrap();
    let clone = values.clone();
    assert_eq!((values.len(), clone.is_empty()), (3, false));
    assert_eq!(column.reads.load(Ordering::SeqCst), 0, "len reads nothing");
    assert_eq!(values.in_memory(), None, "a stored window has no slice");
    assert_eq!(clone.samples(..).collect::<Vec<_>>(), vec![1.0, 2.0, 3.0]);
    assert_eq!(values.sample(2), Some(3.0));
    assert_eq!(values.sample(3), None);
    assert_eq!(values.gather(&[2, 0, 1]), vec![3.0, 1.0, 2.0]);
    let reads = column.reads.load(Ordering::SeqCst);
    assert_eq!(clone.samples(..).collect::<Vec<_>>(), vec![1.0, 2.0, 3.0]);
    assert!(
        column.reads.load(Ordering::SeqCst) > reads,
        "a second pass reads again: nothing was kept"
    );
    assert!(WaveformValues::ptr_eq(&values, &clone));
}

#[test]
fn a_pass_over_a_long_window_holds_one_block_at_a_time() {
    let rows = 2 * BLOCK_SAMPLES + 5;
    let column = Column::new("v(long)", (0..rows).map(|row| row as f64).collect());
    let values =
        WaveformValues::stored(column.clone(), window(0, rows as u64, WindowLead::None)).unwrap();
    let blocks = values
        .blocks(3..rows)
        .map(|block| block.unwrap().len())
        .collect::<Vec<_>>();
    assert_eq!(blocks, vec![BLOCK_SAMPLES, BLOCK_SAMPLES, 2]);
    assert!(
        values
            .samples(..)
            .enumerate()
            .all(|(row, value)| value == row as f64)
    );
    assert_eq!(values.partition_point(|value| value < 70_000.5), 70_001);
    assert!(values.ascends_strictly());
    let memory = WaveformValues::from(vec![1.0, 2.0, 3.0]);
    assert_eq!(memory.blocks(..).count(), 1, "memory is one block");
}

#[test]
fn a_whole_read_larger_than_the_budget_is_refused_not_attempted() {
    let column = Column::new("v(budget)", vec![1.0; 64]);
    let values = WaveformValues::stored(column.clone(), window(0, 64, WindowLead::None)).unwrap();
    set_whole_read_budget(8 * 63);
    let refused = values.read_whole().unwrap_err();
    set_whole_read_budget(1 << 30);
    assert!(refused.cause.contains("read it by window"), "{refused}");
    assert_eq!(column.reads.load(Ordering::SeqCst), 0, "nothing was read");
    assert_eq!(values.read_whole().unwrap().len(), 64);
    // Memory is never refused: it is already held.
    let memory = WaveformValues::from(vec![1.0; 64]);
    set_whole_read_budget(0);
    let held = memory.read_whole().map(|samples| samples.len());
    set_whole_read_budget(1 << 30);
    assert_eq!(held, Ok(64));
}

#[test]
fn a_window_starting_between_rows_holds_its_start_and_the_value_there() {
    let time = Column::new("time", vec![0.0, 1.0, 2.0, 3.0]);
    let wave = Column::new("v(out)", vec![10.0, 20.0, 40.0, 80.0]);
    let (start, fraction) = (1.25, 0.25);
    let axis = WaveformValues::stored(time, window(2, 2, WindowLead::Value(start))).unwrap();
    let values =
        WaveformValues::stored(wave, window(2, 2, WindowLead::Interpolated(fraction))).unwrap();
    assert_eq!(axis, vec![1.25, 2.0, 3.0]);
    // The in-memory window's arithmetic: lower + fraction * (upper - lower).
    assert_eq!(values, vec![20.0 + fraction * (40.0 - 20.0), 40.0, 80.0]);
    assert_eq!(values.sample(0), Some(20.0 + fraction * (40.0 - 20.0)));
    assert_eq!(values.len(), 3);
}

#[test]
fn a_range_of_a_window_reads_only_its_rows_keeps_nothing_and_matches_the_whole_read() {
    let time = Column::new("time", vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    let wave = Column::new("v(out)", vec![10.0, 20.0, 40.0, 80.0, 160.0, 320.0]);
    for (column, lead) in [
        (&time, WindowLead::None),
        (&time, WindowLead::Value(1.5)),
        (&wave, WindowLead::Interpolated(0.25)),
    ] {
        let whole = WaveformValues::stored(column.clone(), window(2, 4, lead)).unwrap();
        let expected = whole.samples(..).collect::<Vec<_>>();
        let ranged = WaveformValues::stored(column.clone(), window(2, 4, lead)).unwrap();
        for start in 0..=expected.len() {
            for end in start..=expected.len() {
                let read = ranged.read_range(start..end).unwrap();
                assert_eq!(
                    read.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
                    expected[start..end]
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    "{lead:?} {start}..{end}"
                );
            }
        }
        // Each range read the source again: nothing was kept.
        let before = column.reads.load(Ordering::SeqCst);
        ranged.read_range(1..3).unwrap();
        assert_eq!(column.reads.load(Ordering::SeqCst), before + 1);
        // A window of another length is unequal without being read.
        assert_ne!(ranged, WaveformValues::from(vec![0.0; 3]));
        assert_eq!(column.reads.load(Ordering::SeqCst), before + 1);
    }
    let unreadable = Arc::new(Column {
        name: "v(unreadable_range_test)",
        values: vec![1.0; 3],
        reads: AtomicUsize::new(0),
        fail: true,
    });
    let values = WaveformValues::stored(unreadable, window(0, 3, WindowLead::None)).unwrap();
    let failure = values.read_range(0..2).unwrap_err();
    assert_eq!(failure.column, "v(unreadable_range_test)");
}

#[test]
fn a_window_past_its_column_or_interpolating_before_its_first_row_is_refused() {
    let column = Column::new("v(a)", vec![0.0; 4]);
    let past = WaveformValues::stored(column.clone(), window(2, 3, WindowLead::None));
    assert!(
        past.unwrap_err()
            .contains("reaches past the 4 rows of 'v(a)'")
    );
    let before = WaveformValues::stored(column, window(0, 2, WindowLead::Interpolated(0.5)));
    assert!(before.unwrap_err().contains("before its first row"));
}

#[test]
fn an_unreadable_window_shows_a_gap_and_its_failure_is_kept_for_diagnostics() {
    let column = Arc::new(Column {
        name: "v(unreadable_window_test)",
        values: vec![1.0; 3],
        reads: AtomicUsize::new(0),
        fail: true,
    });
    let values = WaveformValues::stored(column, window(0, 3, WindowLead::None)).unwrap();
    let failure = values.read_range(0..3).unwrap_err();
    assert_eq!(failure.column, "v(unreadable_window_test)");
    assert_eq!(failure.store, "run.rsr");
    assert!(values.samples(..).all(f64::is_nan));
    assert_eq!(values.sample(1).map(f64::is_nan), Some(true));
    assert_eq!(values.len(), 3);
    assert_eq!(values.blocks(..).next().unwrap().unwrap_err(), failure);
    let reported = take_column_read_failures()
        .into_iter()
        .filter(|reported| reported.column == failure.column)
        .collect::<Vec<_>>();
    assert_eq!(reported, vec![failure.clone()]);
    assert!(
        failure
            .to_string()
            .contains("'v(unreadable_window_test)' could not be read from run.rsr: the chunk's")
    );
}

#[test]
fn values_in_memory_are_shared_changed_and_compared_as_vectors_are() {
    let mut values = WaveformValues::from(vec![1.0, f64::NAN]);
    let shared = values.clone();
    assert!(WaveformValues::ptr_eq(&values, &shared));
    // A NaN sample is unequal to itself, as `Vec<f64>` compares it.
    assert_ne!(values, WaveformValues::from(vec![1.0, f64::NAN]));
    values.make_mut()[1] = 2.0;
    // The change copies the shared values first.
    let shared = shared.in_memory().unwrap();
    assert!(shared[0] == 1.0 && shared[1].is_nan());
    assert_eq!(values, vec![1.0, 2.0]);
    let column = Column::new("v(b)", vec![5.0, 6.0]);
    let mut stored = WaveformValues::stored(column, window(0, 2, WindowLead::None)).unwrap();
    stored.make_mut()[0] = 7.0;
    assert!(!stored.is_stored());
    assert_eq!(stored, vec![7.0, 6.0]);
    let json = serde_json::to_string(&stored).unwrap();
    assert_eq!(json, "[7.0,6.0]");
    let back: WaveformValues = serde_json::from_str(&json).unwrap();
    assert_eq!(back, stored);
}

/// Two series share a key exactly when `ptr_eq` finds them the same samples: clones and handles
/// of one window do; another window, or another allocation of equal samples, does not.
#[test]
fn series_share_a_key_as_ptr_eq_finds_them_the_same() {
    let column = Column::new("time", vec![0.0, 1.0, 2.0]);
    let stored = |rows| WaveformValues::stored(column.clone(), window(0, rows, WindowLead::None));
    let one = stored(3).unwrap();
    let another_handle = stored(3).unwrap();
    let shorter = stored(2).unwrap();
    let memory = WaveformValues::from(vec![0.0, 1.0, 2.0]);
    let copy = WaveformValues::from(vec![0.0, 1.0, 2.0]);
    let (one_again, memory_again) = (one.clone(), memory.clone());
    let all = [
        &one,
        &one_again,
        &another_handle,
        &shorter,
        &memory,
        &memory_again,
        &copy,
    ];
    for left in all {
        for right in all {
            assert_eq!(
                left.sharing_key() == right.sharing_key(),
                WaveformValues::ptr_eq(left, right)
            );
        }
    }
    assert_eq!(one.sharing_key(), another_handle.sharing_key());
    assert_ne!(memory.sharing_key(), copy.sharing_key());
}

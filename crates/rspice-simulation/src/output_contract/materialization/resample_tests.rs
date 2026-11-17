use super::*;
use rspice_results::waveform::{ColumnSource, StoredWindow, WindowLead};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A stored column in memory that keeps the most rows any one read asked for.
struct Column {
    values: Vec<f64>,
    ascending: bool,
    widest_read: AtomicUsize,
}

impl Column {
    fn stored(values: &[f64], ascending: bool) -> (Arc<Self>, WaveformValues) {
        let column = Arc::new(Self {
            values: values.to_vec(),
            ascending,
            widest_read: AtomicUsize::new(0),
        });
        let window = StoredWindow {
            first: 0,
            rows: values.len() as u64,
            lead: WindowLead::None,
        };
        let stored = WaveformValues::stored(column.clone(), window).unwrap();
        (column, stored)
    }
}

impl ColumnSource for Column {
    fn name(&self) -> &str {
        "v(out)"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.widest_read.fetch_max(out.len(), Ordering::SeqCst);
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }

    fn ascending(&self) -> bool {
        self.ascending
    }
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn a_stored_run_resamples_block_by_block_to_the_bits_of_the_whole_columns() {
    let rows = 3 * RESAMPLE_BLOCK_ROWS + 17;
    let x = (0..rows)
        .map(|row| (row as f64 + (row * 7_919 % 13) as f64 / 26.0) * 1e-9)
        .collect::<Vec<_>>();
    let mut y = x.iter().map(|x| (x * 3.1e7).sin()).collect::<Vec<_>>();
    y[2 * RESAMPLE_BLOCK_ROWS + 5] = f64::NAN;
    let real = y.iter().map(|y| y * 0.5).collect::<Vec<_>>();
    let imag = y.iter().map(|y| -y).collect::<Vec<_>>();
    let last = x[rows - 1];
    // A grid that lands on block boundaries' own rows, runs past the axis, and ends between
    // rows.
    for grid in [
        TransientSelectionGrid {
            start: 0.0,
            step: x[RESAMPLE_BLOCK_ROWS],
            stop: 2.0 * last,
        },
        TransientSelectionGrid {
            start: x[3],
            step: last / 4_099.0,
            stop: 0.75 * last,
        },
    ] {
        for ascending in [true, false] {
            let memory = RetainedWaveform::new("v(out)", x.clone(), y.clone())
                .with_complex_components("V(out)", real.clone(), imag.clone());
            let columns = [&x, &y, &real, &imag].map(|values| Column::stored(values, ascending));
            let stored =
                RetainedWaveform::new("v(out)", columns[0].1.clone(), columns[1].1.clone())
                    .with_complex_components("V(out)", columns[2].1.clone(), columns[3].1.clone());

            let expected = resample_selected_and_final(&memory, grid).unwrap();
            let resampled = resample_selected_and_final(&stored, grid).unwrap();
            assert!(expected.x.len() > 2);
            assert_eq!(bits(&resampled.x), bits(&expected.x));
            assert_eq!(bits(&resampled.y), bits(&expected.y));
            let (want, got) = (expected.complex.unwrap(), resampled.complex.unwrap());
            assert_eq!(bits(&got.real), bits(&want.real));
            assert_eq!(bits(&got.imag), bits(&want.imag));
            // No column was read whole: each read asked for at most a block.
            for (column, _) in &columns {
                let widest = column.widest_read.load(Ordering::SeqCst);
                assert!(
                    widest <= RESAMPLE_BLOCK_ROWS + 1,
                    "read {widest} rows at once"
                );
            }
        }
    }
}

#[test]
fn a_stored_axis_that_does_not_ascend_or_a_misaligned_column_is_refused_as_in_memory() {
    let x = vec![0.0, 1.0, 1.0, 2.0];
    let y = vec![0.0; 4];
    let grid = TransientSelectionGrid {
        start: 0.0,
        step: 0.5,
        stop: 2.0,
    };
    let stored = RetainedWaveform::new(
        "v(out)",
        Column::stored(&x, false).1,
        Column::stored(&y, false).1,
    );
    assert_eq!(
        resample_selected_and_final(&stored, grid).unwrap_err(),
        "source waveform axis is not strictly increasing"
    );
    let x = vec![0.0, 1.0, 2.0, 3.0];
    let misaligned = RetainedWaveform::new("v(out)", Column::stored(&x, true).1, y)
        .with_complex_components("V(out)", vec![0.0; 4], vec![0.0; 3]);
    let memory = RetainedWaveform::new("v(out)", x.clone(), vec![0.0; 4]).with_complex_components(
        "V(out)",
        vec![0.0; 4],
        vec![0.0; 3],
    );
    assert_eq!(
        resample_selected_and_final(&misaligned, grid).unwrap_err(),
        resample_selected_and_final(&memory, grid).unwrap_err()
    );
}

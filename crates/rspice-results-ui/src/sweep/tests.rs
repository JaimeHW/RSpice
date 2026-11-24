use super::*;
use rspice_results::sweep_table::{SweepOutput, SweptAxis, SweptValue};

fn table() -> SweepTable {
    let numbers = |values: &[f64]| {
        values
            .iter()
            .map(|value| SweptValue::Number(*value))
            .collect()
    };
    SweepTable {
        rows: vec!["corner=tt, r=1k".into(), "corner=ff, r=2k".into()],
        analyses: vec![0, 1],
        axes: vec![
            SweptAxis {
                name: "corner".into(),
                unit: String::new(),
                values: vec![SweptValue::Text("tt".into()), SweptValue::Text("ff".into())],
            },
            SweptAxis {
                name: "r".into(),
                unit: String::new(),
                values: numbers(&[1e3, 2e3]),
            },
        ],
        outputs: vec![
            SweepOutput {
                name: "V(out)".into(),
                unit: "V".into(),
                kind: OutputKind::Signal,
                values: Vec::new(),
            },
            SweepOutput {
                name: "gain".into(),
                unit: String::new(),
                kind: OutputKind::Measurement,
                values: vec![Some(0.5), Some(1.0 / 3.0)],
            },
        ],
        signal_axis: Some(("time".into(), "s".into())),
    }
}

/// With nothing chosen, the sheet draws the first `.MEAS` result against
/// the first quantity that is a number; a corner's name is no X axis.
#[test]
fn the_sheet_opens_on_a_measurement_against_a_numeric_quantity() {
    let table = table();
    assert_eq!(
        resolve(&table, &SweepSheetState::default()),
        (Some(1), Some(1))
    );
    let chosen = SweepSheetState {
        output: Some("V(out)".into()),
        x: Some("corner".into()),
        at: None,
    };
    assert_eq!(resolve(&table, &chosen), (Some(0), Some(1)));
    // A choice the table no longer holds falls back rather than vanishing.
    let gone = SweepSheetState {
        output: Some("bandwidth".into()),
        x: Some("c".into()),
        at: None,
    };
    assert_eq!(resolve(&table, &gone), (Some(1), Some(1)));
}

/// A signal's column says where each point read it.
#[test]
fn a_signal_column_names_where_it_was_read() {
    let table = table();
    assert_eq!(output_head(&table, 0, None), "V(out) at end");
    assert_eq!(output_head(&table, 0, Some(5e-3)), "V(out) at 5 ms");
    assert_eq!(output_head(&table, 1, Some(5e-3)), "gain");
}

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

/// A swept number right-aligns in its column with its scale suffix in a
/// slot as wide as the widest, so in mono the digits of `500`, `2k` and
/// `1Meg` end in one column; a resistor's ohms follow the slot, and a unit
/// that takes no prefix, a temperature, has none.
#[test]
fn a_swept_numbers_digits_end_in_one_column() {
    let axis = |unit: &str, values: &[f64]| SweptAxis {
        name: "r".into(),
        unit: unit.into(),
        values: values
            .iter()
            .map(|value| SweptValue::Number(*value))
            .collect(),
    };
    // What follows the last digit: right-aligned in mono, rows whose
    // digits are followed by one width end their digits in one column.
    let after_digits = |cell: &String| {
        cell.chars()
            .rev()
            .take_while(|c| !c.is_ascii_digit())
            .count()
    };
    let r = slotted_spellings(&axis("", &[500.0, 2e3, 1e6]));
    assert_eq!(r, ["500   ", "2k  ", "1Meg"]);
    assert!(r.iter().all(|cell| after_digits(cell) == 3), "{r:?}");
    assert_eq!(
        slotted_spellings(&axis("Ω", &[500.0, 2e3])),
        ["500 Ω", "2kΩ"]
    );
    let temperatures = axis("°C", &[-40.0, 125.0]);
    assert_eq!(
        slotted_spellings(&temperatures),
        [temperatures.spelled(0), temperatures.spelled(1)]
    );
}

//! The conversions a trace's or a lane's menu offers: an analog trace read
//! as logic with `digital()`, a bus or logic lane drawn as a staircase with
//! `analog()`. Each is added as an ordinary expression of the strip, so it
//! is saved, edited and removed as one.

use egui::Ui;

/// How the expression spells a trace: a name that is already an accessor as
/// it is, a bare node name as its voltage.
pub fn signal_reference(name: &str) -> String {
    let accessor = name
        .get(..2)
        .is_some_and(|head| head.eq_ignore_ascii_case("V(") || head.eq_ignore_ascii_case("I("));
    if accessor && name.ends_with(')') {
        name.to_owned()
    } else {
        format!("V({name})")
    }
}

/// A threshold as an expression writes it: four significant digits, in
/// plain decimal where that is short and in exponent form where it is not.
pub fn threshold_text(value: f64) -> String {
    let rounded: f64 = format!("{value:.3e}").parse().unwrap_or(value);
    let magnitude = rounded.abs();
    if rounded == 0.0 || (1e-3..1e4).contains(&magnitude) {
        format!("{rounded}")
    } else {
        format!("{rounded:e}")
    }
}

/// The middle of a trace's finite values: where `digital()` puts its one
/// threshold until the reader moves it.
pub fn mid_range(values: &[f64]) -> Option<f64> {
    let (low, high) = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold(None, |range: Option<(f64, f64)>, value| {
            Some(range.map_or((value, value), |(low, high)| {
                (low.min(value), high.max(value))
            }))
        })?;
    Some(0.5 * (low + high))
}

/// What the reader typed into the threshold form, kept between frames.
#[derive(Debug, Clone, Default)]
struct Draft {
    threshold: String,
    low: String,
    time_to_x: String,
}

/// The `digital()` form of a trace's menu: a threshold, starting at the
/// middle of the trace's range; a low threshold, which gives the band its
/// hysteresis; and a time to X, past which a signal held in the band is X.
/// Returns the expression once the reader adds it.
pub fn digital_form(ui: &mut Ui, signal: &str, values: &[f64]) -> Option<String> {
    digital_form_of(ui, &signal_reference(signal), values)
}

/// The same form for a value the strip already spells as an expression,
/// which `digital()` reads as it is typed.
pub fn digital_form_of(ui: &mut Ui, reference: &str, values: &[f64]) -> Option<String> {
    let id = ui.id().with(("digital-form", reference));
    let mut draft = ui
        .data(|data| data.get_temp::<Draft>(id))
        .unwrap_or_else(|| Draft {
            threshold: mid_range(values).map(threshold_text).unwrap_or_default(),
            ..Draft::default()
        });
    let parse = |text: &str| rspice_results::spice_value::parse_spice_value_checked(text.trim());
    egui::Grid::new(id.with("grid"))
        .num_columns(2)
        .spacing([8.0, 3.0])
        .show(ui, |ui| {
            for (label, text, hint) in [
                ("Threshold", &mut draft.threshold, ""),
                ("Low threshold", &mut draft.low, "none"),
                ("Time to X", &mut draft.time_to_x, "unlimited"),
            ] {
                ui.label(label);
                ui.add(
                    egui::TextEdit::singleline(text)
                        .desired_width(80.0)
                        .hint_text(hint),
                );
                ui.end_row();
            }
        });
    let threshold = parse(&draft.threshold);
    let low = (!draft.low.trim().is_empty()).then(|| parse(&draft.low));
    let time_to_x = (!draft.time_to_x.trim().is_empty()).then(|| parse(&draft.time_to_x));
    let problem = match (&threshold, &low, &time_to_x) {
        (Err(_), ..) => Some("The threshold is not a number".to_owned()),
        (_, Some(Err(_)), _) => Some("The low threshold is not a number".to_owned()),
        (_, _, Some(Err(_))) => Some("The time to X is not a number".to_owned()),
        (Ok(high), Some(Ok(low)), _) if low >= high => {
            Some("The low threshold must be below the threshold".to_owned())
        }
        (_, None, Some(Ok(_))) => Some("A time to X needs a low threshold".to_owned()),
        (_, _, Some(Ok(time))) if *time < 0.0 => {
            Some("The time to X cannot be negative".to_owned())
        }
        _ => None,
    };
    if let Some(problem) = &problem {
        ui.label(problem);
    }
    let add = ui
        .add_enabled(problem.is_none(), egui::Button::new("Add digital lane"))
        .clicked();
    if add {
        ui.data_mut(|data| data.remove::<Draft>(id));
        let mut args = vec![reference.to_owned(), draft.threshold.trim().to_owned()];
        if !draft.low.trim().is_empty() {
            args.push(draft.low.trim().to_owned());
        }
        if !draft.time_to_x.trim().is_empty() {
            args.push(draft.time_to_x.trim().to_owned());
        }
        ui.close();
        return Some(format!("digital({})", args.join(", ")));
    }
    ui.data_mut(|data| data.insert_temp(id, draft));
    None
}

/// The name a `digital()` lane goes by in the gutter and the cursor table:
/// its signal and thresholds, `V(CLK) ≥ 0.9`, or `V(CLK) 0.6…1.2` with a
/// band, with its time to X when it has one. Anything else goes by its text.
pub fn digital_label(text: &str) -> String {
    let trimmed = text.trim();
    let inner = trimmed
        .get(..8)
        .filter(|head| head.eq_ignore_ascii_case("digital("))
        .and_then(|_| trimmed[8..].strip_suffix(')'));
    let Some(inner) = inner else {
        return text.to_owned();
    };
    let mut args = Vec::new();
    let (mut depth, mut start) = (0_i32, 0);
    for (index, c) in inner.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                args.push(inner[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    args.push(inner[start..].trim());
    match args.as_slice() {
        [signal, high] => format!("{signal} ≥ {high}"),
        [signal, high, low] => format!("{signal} {low}…{high}"),
        [signal, high, low, time] => format!("{signal} {low}…{high}, X after {time}"),
        _ => text.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_threshold_reads_back_to_its_four_digits() {
        assert_eq!(threshold_text(0.9), "0.9");
        assert_eq!(threshold_text(0.899_999_999), "0.9");
        assert_eq!(threshold_text(1.234_56e-6), "1.235e-6");
        assert_eq!(threshold_text(-2.5), "-2.5");
        assert_eq!(threshold_text(0.0), "0");
        assert_eq!(mid_range(&[0.0, f64::NAN, 1.8, 0.4]), Some(0.9));
        assert_eq!(mid_range(&[f64::NAN]), None);
    }

    #[test]
    fn a_digital_lane_is_named_by_its_signal_and_thresholds() {
        assert_eq!(digital_label("digital(V(CLK), 0.9)"), "V(CLK) ≥ 0.9");
        assert_eq!(
            digital_label("DIGITAL(V(a)+V(b),1.2,0.6)"),
            "V(a)+V(b) 0.6…1.2"
        );
        assert_eq!(
            digital_label("digital(V(CLK), 1.2, 0.6, 0.2n)"),
            "V(CLK) 0.6…1.2, X after 0.2n"
        );
        assert_eq!(digital_label("digital(V(x))"), "digital(V(x))");
    }

    #[test]
    fn a_trace_is_named_by_its_accessor() {
        assert_eq!(signal_reference("TOP"), "V(TOP)");
        assert_eq!(signal_reference("V(out)"), "V(out)");
        assert_eq!(signal_reference("i(R1)"), "i(R1)");
        assert_eq!(signal_reference("x1.out"), "V(x1.out)");
    }
}

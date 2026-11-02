//! `wrdata file vectors`: the vectors as columns of numbers, as ngspice's
//! com_write_simple (frontend/com_gnuplot.c:44) hands them through plotit
//! (frontend/plotting/plotit.c:275) to ft_writesimple
//! (frontend/plotting/gnuplot.c:684).
use super::*;
use crate::engine::control::shell::{ShellValue, percent_e};

/// One written vector and the scale it is written against, each named as
/// plotit names them.
struct Column {
    scale_name: String,
    scale: Vec<Value>,
    name: String,
    values: Vec<ComplexValue>,
    complex: bool,
}

impl ControlCircuit {
    /// The file `wrdata` names and the text it writes there, or `None` when
    /// it names no vector, which ngspice writes nothing for.
    pub(in crate::engine) fn write_data(
        &self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<Option<(String, String)>, ControlExecutionError> {
        let line = command.line;
        let mut input = command.arguments.trim();
        let path = word(&mut input, line)?.to_owned();
        // plotit reads the plot options and drops them; ft_writesimple uses
        // none of them.
        let mut options = ControlPlotOptions::default();
        let mut columns: Vec<Column> = Vec::new();
        // `a b vs c d vs e`: a `vs` scale is every vector's since the last
        // `vs` (plotit.c:805).
        let mut unscaled = 0;
        while !input.is_empty() {
            check_abort(abort, line)?;
            if parse_option(&mut input, &mut options, variables, line, abort)? {
                continue;
            }
            if first_word(input).eq_ignore_ascii_case("vs") {
                word(&mut input, line)?;
                if unscaled == columns.len() {
                    return Err(command_error(line, "misplaced vs arg").into());
                }
                if input.is_empty() {
                    return Err(command_error(line, "missing vs arg").into());
                }
                let (x, label) = expression(&mut input, line, abort)?;
                let x = self.evaluate_vector(&x, variables, line, abort)?;
                for column in &mut columns[unscaled..] {
                    if x.samples.len() != column.values.len() {
                        return Err(command_error(
                            line,
                            format!(
                                "'{label}' has {} values and '{}' has {}",
                                x.samples.len(),
                                column.name,
                                column.values.len()
                            ),
                        )
                        .into());
                    }
                    column.scale_name = vector_name(&label);
                    column.scale = x.samples.iter().map(|sample| sample.re).collect();
                }
                unscaled = columns.len();
                continue;
            }
            let (y, label) = expression(&mut input, line, abort)?;
            let mut value = self.evaluate_vector(&y, variables, line, abort)?;
            // plotit.c:1035 xtend: a single value runs the length of the
            // current plot's scale.
            if let ([sample], None, Some(plot)) = (
                value.samples.as_slice(),
                &value.scale,
                self.current_dataset(),
            ) {
                value.samples = vec![*sample; plot.length()];
                value.scale = Some(plot.name.clone());
            }
            let dataset = self.vector_dataset(&value);
            let scale = self.vector_scale(&value, &dataset, line, abort)?;
            columns.push(Column {
                scale_name: self.scale_name(&scale),
                scale: scale.samples.iter().map(|sample| sample.re).collect(),
                name: vector_name(&label),
                values: value.samples,
                complex: value.complex,
            });
        }
        if columns.is_empty() {
            return Ok(None);
        }
        let single_scale = self.shell.contains_key("wr_singlescale");
        if single_scale
            && let Some(other) = columns
                .iter()
                .find(|column| column.scale.len() != columns[0].values.len())
        {
            return Err(command_error(
                line,
                format!(
                    "wr_singlescale is not possible: vectors {} and {} have different lengths",
                    columns[0].name, other.name
                ),
            )
            .into());
        }
        Ok(Some((path, self.write_columns(&columns, single_scale))))
    }

    /// ft_writesimple's text: a row per scale point, each vector after its
    /// scale (the first vector's only, under `wr_singlescale`), a complex one
    /// as its real and imaginary parts, numbers in `% .*e` to `numdgt`
    /// digits, eight unless it is set.
    fn write_columns(&self, columns: &[Column], single_scale: bool) -> String {
        let digits = match self.shell.get("numdgt") {
            Some(ShellValue::Word(text)) => text
                .parse::<Value>()
                .ok()
                .map(|digits| (digits + 0.5).floor())
                .filter(|digits| *digits > 0.0 && *digits < 100.0)
                .map_or(8, |digits| digits as usize),
            _ => 8,
        };
        let width = digits + 7;
        let mut text = String::new();
        if self.shell.contains_key("wr_vecnames") {
            let one_space = self.shell.contains_key("wr_onespace");
            for (index, column) in columns.iter().enumerate() {
                let names = if column.complex { 2 } else { 1 };
                let scale = (index == 0 || !single_scale).then_some(&column.scale_name);
                for name in scale
                    .into_iter()
                    .chain(std::iter::repeat_n(&column.name, names))
                {
                    if one_space {
                        text.push_str(name);
                        text.push(' ');
                    } else {
                        text.push_str(&format!(" {name:<width$}"));
                    }
                }
            }
            text.push('\n');
        }
        let rows = if single_scale {
            columns[0].values.len()
        } else {
            columns
                .iter()
                .map(|column| column.scale.len())
                .max()
                .unwrap_or(0)
        };
        let number = |text: &mut String, value: Value| {
            let formatted = percent_e(value, digits);
            if !formatted.starts_with('-') {
                text.push(' ');
            }
            text.push_str(&formatted);
            text.push(' ');
        };
        for row in 0..rows {
            for (index, column) in columns.iter().enumerate() {
                let with_scale = index == 0 || !single_scale;
                match (column.scale.get(row), column.values.get(row)) {
                    (Some(&scale), Some(value)) => {
                        if with_scale {
                            number(&mut text, scale);
                        }
                        number(&mut text, value.re);
                        if column.complex {
                            number(&mut text, value.im);
                        }
                    }
                    _ => {
                        let fields = usize::from(with_scale) + if column.complex { 2 } else { 1 };
                        text.push_str(&" ".repeat(fields * (digits + 8)));
                    }
                }
            }
            text.push('\n');
        }
        text
    }
}

impl ControlCircuit {
    /// The name ngspice gives a vector's scale: a DC sweep's is named for
    /// what it sweeps (spicelib/analysis/dctrcurv.c:181), not for the source.
    fn scale_name(&self, scale: &ControlVector) -> String {
        match self.dataset(&scale.dataset) {
            Some(dataset)
                if matches!(dataset.result, ControlAnalysisResult::DcSweep(_))
                    && dataset.scale_name() == scale.expression =>
            {
                let source = scale.expression.as_str();
                if source.eq_ignore_ascii_case("temp") || source.eq_ignore_ascii_case("temper") {
                    return "temp-sweep".into();
                }
                match source
                    .chars()
                    .next()
                    .map(|first| first.to_ascii_lowercase())
                {
                    Some('v') => "v-sweep".into(),
                    Some('i') => "i-sweep".into(),
                    Some('r') => "res-sweep".into(),
                    _ => scale.expression.clone(),
                }
            }
            _ => scale.expression.clone(),
        }
    }
}

/// plotit's name for a vector expression: its words joined by one space,
/// with the `V(` and `I(` of node and branch references in lower case.
fn vector_name(label: &str) -> String {
    let joined = label.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut name = String::with_capacity(joined.len());
    let mut previous: Option<char> = None;
    let mut chars = joined.chars().peekable();
    while let Some(character) = chars.next() {
        let starts_word = previous.is_none_or(|p| !(p.is_ascii_alphanumeric() || p == '_'));
        if starts_word && matches!(character, 'V' | 'I') && chars.peek() == Some(&'(') {
            name.push(character.to_ascii_lowercase());
        } else {
            name.push(character);
        }
        previous = Some(character);
    }
    name
}

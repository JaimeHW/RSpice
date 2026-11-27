//! An expression whose value at each sample is read from its operands at that sample alone
//! (arithmetic, comparisons, and the functions that work sample by sample) evaluated a block
//! of rows at a time, so a stored operand of any length is read one block at a time and never
//! whole. A measurement inside one, `V(out) - avg(V(out))`, is a number over the whole trace,
//! read where it is stored as a measurement is, and stands in the expression as that number.

use std::cell::Cell;
use std::ops::Range;

use super::ast::{CalculatorConstant, CalculatorExpr, Cursor, MarkerReading};
use super::evaluator::{CalcValue, EvaluationContext, EvaluationError, RealValue, evaluate};
use crate::waveform::WaveformValues;

/// The functions that map a series sample by sample, their other arguments being numbers.
const SAMPLEWISE: [&str; 25] = [
    "abs",
    "mag",
    "magnitude",
    "sqrt",
    "exp",
    "log",
    "ln",
    "log10",
    "db",
    "db20",
    "db10",
    "limit",
    "if",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "sinh",
    "cosh",
    "tanh",
    "asinh",
    "acosh",
    "atanh",
];

/// The names an expression reads its X axis by.
fn names_axis(signal: &str) -> bool {
    matches!(
        signal.to_ascii_uppercase().as_str(),
        "TIME" | "T" | "FREQ" | "FREQUENCY"
    )
}

/// Whether `name` (lower case) measures a trace to a number, as a stored trace is measured.
fn measures(name: &str) -> bool {
    name == "delay"
        || super::measures::reduction(name).is_some()
        || super::time_measures::lookup(name).is_some()
        || super::levels::lookup(name).is_some()
}

/// `expr` evaluated over the rows of `axis`, `rows` of them at a time, the axis being the X
/// axis every signal of `ctx` it names runs over: its value at each row, as
/// [`evaluate`] reads it from every sample at once. `progress` hears the rows done of the rows
/// there are after each block and stops the evaluation when it answers false.
///
/// `None` when `expr` is not one to read in blocks: it names a signal held in memory, of
/// another run or on another axis, or calls a function that reads more than one sample at a
/// time, or reads nothing stored. It is then evaluated whole.
pub fn evaluate_in_blocks(
    expr: &CalculatorExpr,
    ctx: &impl EvaluationContext,
    axis: &WaveformValues,
    rows: usize,
    mut progress: impl FnMut(usize, usize) -> bool,
) -> Option<Result<Vec<f64>, EvaluationError>> {
    let mut plan = Plan {
        ctx,
        axis,
        reads_store: false,
    };
    let rewritten = match plan.rewrite(expr) {
        Ok(Some(rewritten)) => rewritten,
        Ok(None) => return None,
        Err(error) => return Some(Err(error)),
    };
    if !plan.reads_store || axis.is_empty() {
        return None;
    }
    let total = axis.len();
    let mut values = Vec::new();
    if values.try_reserve_exact(total).is_err() {
        return Some(Err(EvaluationError::MathError(format!(
            "the expression's {total} samples do not fit in this process's memory"
        ))));
    }
    let refused = Cell::new(false);
    let mut start = 0;
    while start < total {
        let range = start..(start + rows.max(1)).min(total);
        let x = match axis.read_range(range.clone()) {
            Ok(x) => x.into_owned(),
            Err(failure) => {
                return Some(Err(EvaluationError::WaveformMismatch(failure.to_string())));
            }
        };
        let block = Block {
            whole: ctx,
            rows: range.clone(),
            x,
            refused: &refused,
        };
        match evaluate(&rewritten, &block) {
            Ok(CalcValue::Real(RealValue::Waveform(_, y))) if y.len() == range.len() => {
                values.extend_from_slice(&y);
            }
            // A number, or a complex value, is not one value for each row.
            Ok(_) => return None,
            Err(_) if refused.get() => return None,
            Err(error) => return Some(Err(error)),
        }
        if !progress(range.end, total) {
            return Some(Err(EvaluationError::MathError(
                "the evaluation was stopped".to_owned(),
            )));
        }
        start = range.end;
    }
    Some(Ok(values))
}

/// What an expression reads, found before any block is.
struct Plan<'c, C> {
    ctx: &'c C,
    axis: &'c WaveformValues,
    /// Whether any operand is stored: one held in memory is cheaper read whole.
    reads_store: bool,
}

impl<C: EvaluationContext> Plan<'_, C> {
    /// `expr` with each measurement in it replaced by its number, or `None` when it is not
    /// one to read in blocks.
    fn rewrite(
        &mut self,
        expr: &CalculatorExpr,
    ) -> Result<Option<CalculatorExpr>, EvaluationError> {
        Ok(Some(match expr {
            CalculatorExpr::Number(_)
            | CalculatorExpr::Constant(
                CalculatorConstant::Cursor(_) | CalculatorConstant::Marker(_),
            ) => expr.clone(),
            CalculatorExpr::Constant(CalculatorConstant::Time | CalculatorConstant::Frequency) => {
                self.reads_store |= self.axis.is_stored();
                expr.clone()
            }
            CalculatorExpr::WaveformRef {
                signal,
                dataset: None,
            } => {
                if names_axis(signal) {
                    self.reads_store |= self.axis.is_stored();
                } else {
                    match self.ctx.stored_columns(signal, None) {
                        Some((x, _)) if WaveformValues::ptr_eq(x, self.axis) => {
                            self.reads_store = true;
                        }
                        _ => return Ok(None),
                    }
                }
                expr.clone()
            }
            CalculatorExpr::WaveformRef { .. } | CalculatorExpr::Keyword(_) => return Ok(None),
            CalculatorExpr::UnaryOp { op, operand } => match self.rewrite(operand)? {
                Some(operand) => CalculatorExpr::UnaryOp {
                    op: *op,
                    operand: Box::new(operand),
                },
                None => return Ok(None),
            },
            CalculatorExpr::BinaryOp { op, left, right } => {
                let (Some(left), Some(right)) = (self.rewrite(left)?, self.rewrite(right)?) else {
                    return Ok(None);
                };
                CalculatorExpr::BinaryOp {
                    op: *op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            CalculatorExpr::FunctionCall { name, args } => {
                let lower = name.to_ascii_lowercase();
                if SAMPLEWISE.contains(&lower.as_str()) {
                    let mut rewritten = Vec::with_capacity(args.len());
                    for arg in args {
                        match self.rewrite(arg)? {
                            Some(arg) => rewritten.push(arg),
                            None => return Ok(None),
                        }
                    }
                    CalculatorExpr::FunctionCall {
                        name: name.clone(),
                        args: rewritten,
                    }
                } else if measures(&lower) {
                    match evaluate(expr, self.ctx)? {
                        CalcValue::Real(RealValue::Scalar(value)) => CalculatorExpr::Number(value),
                        _ => return Ok(None),
                    }
                } else {
                    return Ok(None);
                }
            }
        }))
    }
}

/// The rows `rows` of every signal an expression names, read for one block.
struct Block<'a, C> {
    whole: &'a C,
    rows: Range<usize>,
    /// The axis over these rows.
    x: Vec<f64>,
    /// Set when the expression named something [`Plan`] did not find, so it is evaluated
    /// whole instead.
    refused: &'a Cell<bool>,
}

impl<C: EvaluationContext> EvaluationContext for Block<'_, C> {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        let refuse = || {
            self.refused.set(true);
            EvaluationError::IdentifierNotFound(signal.to_owned())
        };
        if dataset.is_some() {
            return Err(refuse());
        }
        if names_axis(signal) {
            return Ok(CalcValue::create_waveform(self.x.clone(), self.x.clone()));
        }
        let (_, y) = self.whole.stored_columns(signal, None).ok_or_else(refuse)?;
        let y = y
            .read_range(self.rows.clone())
            .map_err(|failure| EvaluationError::WaveformMismatch(failure.to_string()))?;
        Ok(CalcValue::create_waveform(self.x.clone(), y.into_owned()))
    }

    fn cursor(&self, cursor: Cursor) -> Option<f64> {
        self.whole.cursor(cursor)
    }

    fn marker(&self, reading: MarkerReading) -> Option<f64> {
        self.whole.marker(reading)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::calculator::parser::try_parse;
    use crate::calculator::retained::WaveformsContext;
    use crate::waveform::{ColumnSource, RetainedWaveform, StoredWindow, WindowLead};

    /// A stored column that records the longest read it is asked for.
    struct Column {
        values: Vec<f64>,
        longest: AtomicUsize,
    }

    impl ColumnSource for Column {
        fn name(&self) -> &str {
            "column"
        }

        fn store(&self) -> String {
            "run.rsr".to_owned()
        }

        fn rows(&self) -> u64 {
            self.values.len() as u64
        }

        fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
            self.longest.fetch_max(out.len(), Ordering::SeqCst);
            let first = first as usize;
            out.copy_from_slice(&self.values[first..first + out.len()]);
            Ok(())
        }
    }

    fn stored(values: &[f64]) -> (Arc<Column>, WaveformValues) {
        let column = Arc::new(Column {
            values: values.to_vec(),
            longest: AtomicUsize::new(0),
        });
        let window = StoredWindow {
            first: 0,
            rows: values.len() as u64,
            lead: WindowLead::None,
        };
        let values = WaveformValues::stored(column.clone(), window).expect("the window fits");
        (column, values)
    }

    fn parse(text: &str) -> CalculatorExpr {
        try_parse(text, |number| {
            number.parse().map_err(|error| format!("{error}"))
        })
        .expect("the expression parses")
    }

    /// Two signals on one uneven axis, stored, and the same samples in memory.
    fn signals() -> (
        Vec<Arc<Column>>,
        Vec<RetainedWaveform>,
        Vec<RetainedWaveform>,
    ) {
        let x: Vec<f64> = (0..101)
            .map(|i| f64::from(i) * 1.0e-6 + 2.0e-7 * f64::from(i % 3))
            .collect();
        let a: Vec<f64> = x.iter().map(|t| (t * 3.0e5).sin()).collect();
        // A sample that is not a number stays a hole, block or whole.
        let mut b: Vec<f64> = x.iter().map(|t| 0.5 + (t * 1.1e5).cos()).collect();
        b[40] = f64::NAN;
        let (x_column, x_stored) = stored(&x);
        let (a_column, a_stored) = stored(&a);
        let (b_column, b_stored) = stored(&b);
        let on_disk = vec![
            RetainedWaveform::new("V(a)", x_stored.clone(), a_stored),
            RetainedWaveform::new("V(b)", x_stored, b_stored),
        ];
        let in_memory = vec![
            RetainedWaveform::new("V(a)", x.clone(), a),
            RetainedWaveform::new("V(b)", x, b),
        ];
        (vec![x_column, a_column, b_column], on_disk, in_memory)
    }

    /// Each expression read in blocks of a few rows gives, bit for bit, what the same samples
    /// read whole give, and no read of its samples takes more rows than a block: a measure in
    /// it reads by the store's own blocks, once.
    #[test]
    fn an_expression_read_in_blocks_matches_the_expression_read_whole() {
        let (columns, on_disk, in_memory) = signals();
        let (disk, memory) = (
            WaveformsContext::new(&on_disk),
            WaveformsContext::new(&in_memory),
        );
        for text in [
            "V(a) * 2 + V(b)",
            "-V(a) / V(b)",
            "dB(V(a)) - db10(V(b))",
            "V(a) - avg(V(a))",
            "if(V(a) > 0, V(a), -V(b))",
            "sqrt(abs(V(a))) * TIME",
            "limit(V(a), 0.5, -0.5) ^ 2",
            "sin(V(b)) + atanh(V(a) / 2)",
            "V(a) * pp(V(a))",
        ] {
            for column in &columns {
                column.longest.store(0, Ordering::SeqCst);
            }
            let expr = parse(text);
            let whole = match evaluate(&expr, &memory).expect(text) {
                CalcValue::Real(RealValue::Waveform(_, y)) => y,
                other => panic!("{text} read whole is not a series: {other:?}"),
            };
            let mut blocks_done = Vec::new();
            let blocks = evaluate_in_blocks(&expr, &disk, &on_disk[0].x, 7, |done, total| {
                blocks_done.push((done, total));
                true
            })
            .unwrap_or_else(|| panic!("{text} reads in blocks"))
            .expect(text);
            assert_eq!(blocks.len(), whole.len(), "{text}");
            for (row, (block, whole)) in blocks.iter().zip(&whole).enumerate() {
                assert!(
                    block.to_bits() == whole.to_bits() || (block.is_nan() && whole.is_nan()),
                    "{text} row {row}: in blocks {block}, whole {whole}"
                );
            }
            assert_eq!(blocks_done.len(), 101_usize.div_ceil(7), "{text}");
            assert_eq!(blocks_done.last(), Some(&(101, 101)), "{text}");
            if !text.contains("avg(") && !text.contains("pp(") {
                for column in &columns {
                    let longest = column.longest.load(Ordering::SeqCst);
                    assert!(longest <= 7, "{text} read {longest} rows at once");
                }
            }
        }
    }

    /// What reads more than one sample at a time, another run, or nothing stored is left to be
    /// read whole.
    #[test]
    fn an_expression_that_is_not_read_sample_by_sample_is_left_whole() {
        let (_, on_disk, in_memory) = signals();
        let disk = WaveformsContext::new(&on_disk);
        for text in [
            "deriv(V(a))",
            "clip(V(a), 1e-5, 2e-5)",
            "V(a)@baseline",
            "V(a) + V(missing)",
            "V(a) * period(V(a), \"plot\")",
            "2 * 3",
        ] {
            assert!(
                evaluate_in_blocks(&parse(text), &disk, &on_disk[0].x, 7, |_, _| true).is_none(),
                "{text} reads in blocks"
            );
        }
        let memory = WaveformsContext::new(&in_memory);
        assert!(
            evaluate_in_blocks(&parse("V(a) * 2"), &memory, &in_memory[0].x, 7, |_, _| true)
                .is_none(),
            "a series held in memory is read whole"
        );
        let stopped = evaluate_in_blocks(&parse("V(a) * 2"), &disk, &on_disk[0].x, 7, |done, _| {
            done < 30
        });
        assert!(matches!(stopped, Some(Err(_))), "a stopped evaluation ends");
    }
}

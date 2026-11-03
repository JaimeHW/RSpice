//! Plots made from a transient plot (ngspice frontend/linear.c): `cutout`
//! copies a window of its rows, and `linearize` interpolates its vectors onto
//! an even time grid.

use super::*;

impl ControlCircuit {
    /// ngspice `cutout [vector ...]` (frontend/linear.c:173 com_cutout): a new
    /// transient plot holds the current one's rows from the first time past
    /// `cut-tstart` up to, and without, the first time past `cut-tstop`, with
    /// the plot's own `cut-tstart` and `cut-tstop` vectors as the bounds, or
    /// its ends when it lacks them. It copies the vectors named, or each
    /// vector of the plot long enough for the window.
    pub(in crate::engine) fn cutout(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        let line = command.line;
        let (plot, scale, times) = self.transient_plot(command)?;
        let last = times.len() - 1;
        // linear.c:205: the first row past a bound, or the last row.
        let past = |bound: Value| (0..last).find(|&row| times[row] > bound).unwrap_or(last);
        let start = self.plot_bound(&plot, "cut-tstart");
        let stop = self.plot_bound(&plot, "cut-tstop");
        let (tstart, first) = start.map_or((times[0], 0), |bound| (bound, past(bound)));
        let (tstop, end) = stop.map_or((times[last], last), |bound| (bound, past(bound)));
        if tstop - tstart <= 0.0 || end <= first {
            return Err(command_error(
                line,
                format!("cutout: bad parameters -- start = {tstart}, stop = {tstop}"),
            )
            .into());
        }
        let vectors = self.transient_vectors(command, &plot, end, variables, abort)?;
        let kind = if start.is_none() && stop.is_none() {
            "Transient Analysis (copy)"
        } else {
            "Transient Analysis (cut out)"
        };
        self.new_transient_plot(kind, &scale, &times[first..end], line)?;
        for (name, value) in vectors {
            let samples = value.samples[first..end].to_vec();
            self.push_vector(
                &name,
                VectorValue {
                    samples,
                    scale: None,
                    ..value
                },
                false,
            );
        }
        Ok(())
    }

    /// ngspice `linearize [np=n] [vector ...]` (frontend/linear.c:24
    /// com_linearize): a new transient plot holds the current one's vectors
    /// interpolated linearly (interp.c:18 lincopy) onto the times from tstart
    /// in steps of tstep up to tstop: the transient analysis's own, or what
    /// the plot's `lin-tstart`, `lin-tstop` and `lin-tstep` vectors set,
    /// which it reports. `np=n` sets the number of points instead, and
    /// `np=auto2n` the power of two nearest the span over the step. Returns
    /// the report.
    pub(in crate::engine) fn linearize(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<String, ControlExecutionError> {
        let line = command.line;
        let (plot, scale, times) = self.transient_plot(command)?;
        let (mut tstart, mut tstop, mut tstep) = match self.dataset(&plot).map(|d| &d.command) {
            Some(AnalysisCommand::Tran {
                step, stop, start, ..
            }) => (start.unwrap_or(0.0), *stop, *step),
            // linear.c:57: the scale's ends and its mean step.
            _ => {
                let (first, last) = (times[0], times[times.len() - 1]);
                (first, last, (last - first) / times.len() as Value)
            }
        };
        let mut report = String::new();
        for (name, value) in [
            ("tstart", &mut tstart),
            ("tstop", &mut tstop),
            ("tstep", &mut tstep),
        ] {
            if let Some(bound) = self.plot_bound(&plot, &format!("lin-{name}")) {
                report.push_str(&format!(
                    "linearize {name} is set to: {}\n",
                    crate::engine::control::shell::percent_e(bound, 6)
                ));
                *value = bound;
            }
        }
        if (tstop - tstart) * tstep <= 0.0 || tstop - tstart < tstep {
            return Err(command_error(
                line,
                format!(
                    "linearize: bad parameters -- start = {tstart}, stop = {tstop}, step = {tstep}"
                ),
            )
            .into());
        }
        let points = crate::engine::control::shell::words(&command.arguments)
            .into_iter()
            .find_map(|word| {
                word.text
                    .get(..3)
                    .filter(|prefix| prefix.eq_ignore_ascii_case("np="))
                    .map(|_| word.text[3..].to_owned())
            });
        let length = match points {
            Some(points) if points.eq_ignore_ascii_case("auto2n") => {
                let exponent = ((tstop - tstart) / tstep).log2().round();
                if !(0.0..63.0).contains(&exponent) {
                    return Err(command_error(line, "linearize: np=auto2n is out of range").into());
                }
                1usize << exponent as u32
            }
            Some(points) => points
                .parse::<usize>()
                .ok()
                .filter(|points| *points > 0)
                .ok_or_else(|| command_error(line, "linearize: np= requires a number of points"))?,
            None => ((tstop - tstart) / tstep + 1.5) as usize,
        };
        let vectors = self.transient_vectors(command, &plot, times.len(), variables, abort)?;
        if length.saturating_mul(vectors.len() + 1) > self.value_limit {
            return Err(command_error(
                line,
                format!("linearize: {length} points exceed the result value limit"),
            )
            .into());
        }
        // linear.c:137 steps the time by adding the step, as here.
        let mut grid = Vec::with_capacity(length);
        let mut time = tstart;
        for _ in 0..length {
            grid.push(time);
            time += tstep;
        }
        self.new_transient_plot("Transient Analysis (linearized)", &scale, &grid, line)?;
        for (name, value) in vectors {
            check_abort(abort, line)?;
            let samples = grid
                .iter()
                .map(|&time| interpolate(&times, &value.samples, time).into())
                .collect();
            self.push_vector(
                &name,
                VectorValue {
                    samples,
                    scale: None,
                    ..value
                },
                false,
            );
        }
        Ok(report)
    }

    /// The current plot when it is a transient dataset: its name, its scale's
    /// name and its times.
    fn transient_plot(
        &self,
        command: &ControlCommand,
    ) -> Result<(String, String, Vec<Value>), ControlError> {
        let line = command.line;
        let dataset = self
            .current_dataset()
            .filter(|dataset| {
                self.vectors.current_plot().is_none()
                    && matches!(dataset.result, ControlAnalysisResult::Transient(_))
            })
            .ok_or_else(|| {
                command_error(
                    line,
                    format!("{}: plot must be a transient analysis", command.name),
                )
            })?;
        let times = (0..dataset.length())
            .map(|row| dataset.scale_value(row).filter(|time| time.is_finite()))
            .collect::<Option<Vec<_>>>()
            .filter(|times| !times.is_empty())
            .ok_or_else(|| command_error(line, format!("{}: no data in vector", command.name)))?;
        Ok((dataset.name.clone(), dataset.scale_name().to_owned(), times))
    }

    /// The first value of the vector `let` bound in the plot under `name`.
    fn plot_bound(&self, plot: &str, name: &str) -> Option<Value> {
        self.vectors
            .vectors_in(plot)
            .find(|(vector, _)| vector.eq_ignore_ascii_case(name))
            .and_then(|(_, value)| value.samples.first())
            .map(|value| value.re)
    }

    /// The vectors `cutout` and `linearize` copy from a transient plot: those
    /// the command names, or the plot's vectors that `display` lists without
    /// its scale. A vector shorter than `length` is left out (linear.c:285,
    /// interp.c:34). A copy of `v(node)` or `i(source)` takes the name the
    /// plot gives that node or branch, so the same reference finds it.
    fn transient_vectors(
        &self,
        command: &ControlCommand,
        plot: &str,
        length: usize,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(String, VectorValue)>, ControlExecutionError> {
        let line = command.line;
        let mut requested = Vec::new();
        for word in crate::engine::control::shell::words(&command.arguments) {
            if command.name == "linearize"
                && word
                    .text
                    .get(..3)
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("np="))
            {
                continue;
            }
            // Each word names a vector of the plot (vectors.c vec_fromplot):
            // `2` is node 2, and `v(node)` and `i(source)` the vectors
            // `node` and `source#branch`.
            let mut text = word.text.as_str();
            let (expression, _) = expression(&mut text, line, abort)?;
            let name = match &expression {
                Expr::FnCall { name, args } => match (name.as_str(), args.as_slice()) {
                    ("V", [Expr::Param(node)]) => node.to_ascii_lowercase(),
                    ("I", [Expr::Param(source)]) => {
                        format!("{}#branch", source.to_ascii_lowercase())
                    }
                    _ => word.text.to_ascii_lowercase(),
                },
                _ => word.text.to_ascii_lowercase(),
            };
            requested.push((name.clone(), Expr::Param(name)));
        }
        if requested.is_empty() {
            let dataset = self
                .dataset(plot)
                .ok_or_else(|| command_error(line, "the transient plot is gone"))?;
            let policy = self.netlist.ground_policy();
            let scale = dataset.scale_vector();
            let mut names: Vec<String> = dataset
                .vectors(|name| policy.is_ground(name) || dataset.is_event_node(name))
                .into_iter()
                .map(|(name, _)| name)
                .filter(|name| Some(name) != scale.as_ref())
                .collect();
            for (vector, _) in self.vectors.vectors_in(plot) {
                let vector = vector.to_ascii_lowercase();
                if !names.contains(&vector) {
                    names.push(vector);
                }
            }
            requested = names
                .into_iter()
                .map(|name| (name.clone(), Expr::Param(name)))
                .collect();
        }
        let mut vectors = Vec::with_capacity(requested.len());
        for (name, expression) in requested {
            let value = self.evaluate_vector(&expression, variables, line, abort)?;
            if value.samples.len() >= length && !value.complex {
                vectors.push((name, value));
            }
        }
        Ok(vectors)
    }

    /// Make a transient plot of `kind` current, its scale `times`.
    fn new_transient_plot(
        &mut self,
        kind: &'static str,
        scale: &str,
        times: &[Value],
        line: usize,
    ) -> Result<(), ControlError> {
        let name = self.new_plot_name("tran", line)?;
        self.vectors
            .new_analysis_plot(name, self.datasets.len(), kind);
        let value = VectorValue {
            samples: times.iter().map(|&time| time.into()).collect(),
            complex: false,
            scale: None,
            unit: SignalUnit::Second,
        };
        self.push_vector(scale, value, false);
        Ok(())
    }
}

/// ngspice's degree-one ft_interpolate (maths/poly/interpolate.c): the value
/// on the line through the two rows around `time`, or through the end rows
/// past either end.
fn interpolate(times: &[Value], samples: &[ComplexValue], time: Value) -> Value {
    let rows = times.len().min(samples.len());
    if rows < 2 {
        return samples.first().map_or(0.0, |value| value.re);
    }
    let upper = times[..rows]
        .partition_point(|&row| row <= time)
        .clamp(1, rows - 1);
    let (t0, t1) = (times[upper - 1], times[upper]);
    let (v0, v1) = (samples[upper - 1].re, samples[upper].re);
    if t1 == t0 {
        v0
    } else {
        v0 + (v1 - v0) * (time - t0) / (t1 - t0)
    }
}

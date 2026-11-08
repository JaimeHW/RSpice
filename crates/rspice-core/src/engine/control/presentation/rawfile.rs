//! `write [file [vectors]]`: the vectors as an ngspice raw file, one section
//! for each plot they belong to (frontend/postcoms.c:440 com_write,
//! frontend/rawfile.c:42 raw_write).
use super::batch::type_name;
use super::wrdata::plot_scale_name;
use super::*;
use crate::engine::control::shell::{ShellValue, percent_e};

/// ngspice's `ft_rawfile`, where a `write` naming no file goes.
const DEFAULT_FILE: &str = "rawspice.raw";
/// raw_write's `DEFPREC`.
const DIGITS: usize = 15;

/// One written vector, named and typed as raw_write lists it.
struct RawVector {
    /// The node or vector name, without `v(...)`, which finds the plot's scale
    /// among the vectors written (vec_eq).
    bare: String,
    name: String,
    kind: &'static str,
    complex: bool,
    samples: Vec<ComplexValue>,
}

/// One section of the file: the vectors of one plot, in the order named.
struct RawPlot {
    plot: String,
    scale: Option<RawVector>,
    vectors: Vec<RawVector>,
}

fn raw_vector(
    label: &str,
    kind: &'static str,
    complex: bool,
    samples: Vec<ComplexValue>,
) -> RawVector {
    let label = label.to_ascii_lowercase();
    // raw_write writes `i(name)` for `name#branch`, and `v(name)` for a node,
    // whatever the vector's name is otherwise.
    let name = match kind {
        "current" => {
            let name = label.split("#branch").next().unwrap_or(&label);
            if name.starts_with("i(") {
                name.to_owned()
            } else {
                format!("i({name})")
            }
        }
        "voltage" if !label.starts_with("v(") => format!("v({label})"),
        _ => label.clone(),
    };
    let bare = label
        .strip_prefix("v(")
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap_or(&label)
        .to_owned();
    RawVector {
        bare,
        name,
        kind,
        complex,
        samples,
    }
}

/// Whether a `#branch` vector is a lead current a transient here derives for
/// a device ngspice gives no matrix branch, which its raw file does not list.
fn is_derived_current(name: &str) -> bool {
    name.strip_suffix("#branch")
        .and_then(|name| name.rsplit('.').next())
        .and_then(|device| device.chars().next())
        .is_some_and(|letter| {
            matches!(
                letter.to_ascii_lowercase(),
                'c' | 'd' | 'f' | 'g' | 'i' | 'j' | 'm' | 'q' | 'r' | 's' | 'w' | 'z'
            )
        })
}

impl ControlCircuit {
    /// The file `write` names and the bytes it writes there.
    pub(in crate::engine) fn write_raw(
        &self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(String, Vec<u8>), ControlExecutionError> {
        let line = command.line;
        let mut input = command.arguments.trim();
        let path = if input.is_empty() {
            DEFAULT_FILE.to_owned()
        } else {
            word(&mut input, line)?.to_owned()
        };
        if input.is_empty() {
            input = "all";
        }
        let mut plots = Vec::new();
        while !input.is_empty() {
            check_abort(abort, line)?;
            let (parsed, label) = expression(&mut input, line, abort)?;
            match self.plot_group(&parsed, line)? {
                Some(names) => {
                    for name in names.iter().filter(|name| !is_derived_current(name)) {
                        let vector = Expr::Param(name.clone());
                        self.add_raw_vector(&mut plots, &vector, name, variables, line, abort)?;
                    }
                }
                None => {
                    self.add_raw_vector(&mut plots, &parsed, &label, variables, line, abort)?;
                }
            }
        }
        // ngspice's own default is binary; `conf.c` sets AsciiRawFile to 0.
        let binary = !matches!(
            self.shell.get("filetype"),
            Some(ShellValue::Word(kind)) if kind == "ascii"
        );
        let mut bytes = Vec::new();
        for plot in plots {
            self.raw_section(&mut bytes, plot, binary);
        }
        Ok((path, bytes))
    }

    fn add_raw_vector(
        &self,
        plots: &mut Vec<RawPlot>,
        expression: &Expr,
        label: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlError> {
        let value = self.evaluate_vector(expression, variables, line, abort)?;
        let plot = self.vector_dataset(&value);
        let index = match plots.iter().position(|known: &RawPlot| known.plot == plot) {
            Some(index) => index,
            None => {
                let scale = self.raw_scale(&plot, variables, line, abort)?;
                plots.push(RawPlot {
                    plot,
                    scale,
                    vectors: Vec::new(),
                });
                plots.len() - 1
            }
        };
        let kind = if self.keeps_vector_type(expression) {
            type_name(&value.unit)
        } else {
            "notype"
        };
        plots[index]
            .vectors
            .push(raw_vector(label, kind, value.complex, value.samples));
        Ok(())
    }

    /// Whether a vector expression keeps a type: ngspice's arithmetic gives
    /// its result none, and a function keeps its argument's.
    fn keeps_vector_type(&self, expression: &Expr) -> bool {
        match expression {
            Expr::Param(name) => !self.is_untyped_vector(name),
            Expr::FnCall { args, .. } => args.len() == 1,
            _ => false,
        }
    }

    /// A plot's scale (`pl_scale`), which every section starts with.
    fn raw_scale(
        &self,
        plot: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<RawVector>, ControlError> {
        let first_vector = match self.dataset(plot) {
            Some(dataset) if dataset.scale_vector().is_some() => {
                let samples = self.dataset_scale(dataset, line, abort)?;
                return Ok(Some(raw_vector(
                    &plot_scale_name(dataset),
                    type_name(&dataset.scale_unit()),
                    dataset.is_complex(),
                    samples,
                )));
            }
            // An operating point sweeps nothing: its first vector is its scale.
            Some(dataset) => {
                let policy = self.netlist.ground_policy();
                dataset
                    .vectors(|name| policy.is_ground(name) || dataset.is_event_node(name))
                    .into_iter()
                    .next()
                    .map(|(name, _)| name)
            }
            None => self.vectors.plot_scale(plot).map(str::to_owned),
        };
        let Some(name) = first_vector else {
            return Ok(None);
        };
        let scale = Expr::Param(format!("{plot}.{name}"));
        let value = self.evaluate_vector(&scale, variables, line, abort)?;
        Ok(Some(raw_vector(
            &name,
            type_name(&value.unit),
            value.complex,
            value.samples,
        )))
    }

    /// raw_write: the header, the scale first among the variables, then a
    /// row of values per point, each vector padded to the longest with zeros.
    fn raw_section(&self, out: &mut Vec<u8>, plot: RawPlot, binary: bool) {
        let RawPlot {
            plot,
            scale,
            mut vectors,
        } = plot;
        if let Some(scale) = scale {
            match vectors.iter().position(|vector| vector.bare == scale.bare) {
                Some(index) => vectors[..=index].rotate_right(1),
                None => vectors.insert(0, scale),
            }
        }
        let dataset = self.dataset(&plot);
        let length = vectors
            .iter()
            .map(|vector| vector.samples.len())
            .max()
            .unwrap_or(0);
        let complex = vectors.iter().any(|vector| vector.complex);
        let mut header = format!(
            "Title: {}\nCommand: rspice-{}\nPlotname: {}\nFlags: {}\nNo. Variables: {}\n\
             No. Points: {length}\nVariables:\n",
            dataset.map_or_else(|| "Anonymous".to_owned(), |_| self.title()),
            env!("CARGO_PKG_VERSION"),
            dataset.map_or("unknown", ControlNamedDataset::plot_title),
            if complex { "complex" } else { "real" },
            vectors.len(),
        );
        for (index, vector) in vectors.iter().enumerate() {
            header.push_str(&format!("\t{index}\t{}\t{}", vector.name, vector.kind));
            if vector.samples.len() != length {
                header.push_str(&format!(" dims={}", vector.samples.len()));
            }
            header.push('\n');
        }
        header.push_str(if binary { "Binary:\n" } else { "Values:\n" });
        out.extend_from_slice(header.as_bytes());
        for row in 0..length {
            if binary {
                for vector in &vectors {
                    let sample = vector.samples.get(row).copied();
                    let re = sample.map_or(0.0, |value| value.re);
                    out.extend_from_slice(&re.to_le_bytes());
                    if complex {
                        let im = sample.map_or(0.0, |value| value.im);
                        out.extend_from_slice(&im.to_le_bytes());
                    }
                }
            } else {
                let mut text = format!(" {row}");
                let number = |value: Value| percent_e(value, DIGITS);
                for vector in &vectors {
                    let sample = vector.samples.get(row);
                    let re = number(sample.map_or(0.0, |value| value.re));
                    text.push_str(&match (complex, sample) {
                        (false, _) => format!("\t{re}\n"),
                        (true, Some(_)) if !vector.complex => format!("\t{re},0.0\n"),
                        (true, Some(value)) => format!("\t{re},{}\n", number(value.im)),
                        (true, None) => format!("\t{re},{}\n", number(0.0)),
                    });
                }
                text.push('\n');
                out.extend_from_slice(text.as_bytes());
            }
        }
    }
}

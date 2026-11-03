//! Batch-mode text for commands that change no circuit and no dataset:
//! `echo`, `display` and the plot list of `setplot` (ngspice frontend
//! com_echo.c, com_display.c and plotting/pvec.c, postcoms.c `com_splot`).

use super::*;

impl ControlNamedDataset {
    fn node_names(&self) -> &[String] {
        match &self.result {
            ControlAnalysisResult::TransferFunction(_)
            | ControlAnalysisResult::PoleZero(_)
            | ControlAnalysisResult::Sensitivity(_)
            | ControlAnalysisResult::Stability(_)
            | ControlAnalysisResult::SParameters(_)
            | ControlAnalysisResult::DcMatch(_) => &[],
            ControlAnalysisResult::DcSweep(result) => result
                .points
                .first()
                .map_or(&[], |point| &point.result.node_names),
            ControlAnalysisResult::Noise(points)
            | ControlAnalysisResult::NoiseTable(FrequencyDataResult { points, .. }) => {
                points.first().map_or(&[], |point| &point.node_names)
            }
            ControlAnalysisResult::Distortion(result) => result
                .points
                .first()
                .map_or(&[], |point| &point.fundamental_f1.node_names),
            ControlAnalysisResult::OperatingPoint(result) => &result.node_names,
            ControlAnalysisResult::Ac(points)
            | ControlAnalysisResult::AcTable(FrequencyDataResult { points, .. }) => {
                points.first().map_or(&[], |point| &point.node_names)
            }
            ControlAnalysisResult::Transient(result) => &result.node_names,
        }
    }

    /// The plot name ngspice gives the analysis (`pl_name`).
    fn plot_title(&self) -> &'static str {
        match &self.result {
            ControlAnalysisResult::OperatingPoint(_) => "Operating Point",
            ControlAnalysisResult::Ac(_) | ControlAnalysisResult::AcTable(_) => "AC Analysis",
            ControlAnalysisResult::Distortion(_) => "Distortion Analysis",
            ControlAnalysisResult::Stability(_) => "Stability Analysis",
            ControlAnalysisResult::SParameters(_) => "SP Analysis",
            ControlAnalysisResult::Noise(_) | ControlAnalysisResult::NoiseTable(_) => {
                "Noise Spectral Density Curves"
            }
            ControlAnalysisResult::DcSweep(_) => "DC transfer characteristic",
            ControlAnalysisResult::TransferFunction(_) => "Transfer Function",
            ControlAnalysisResult::PoleZero(_) => "Pole-Zero Analysis",
            ControlAnalysisResult::Sensitivity(_) => "Sensitivity Analysis",
            ControlAnalysisResult::DcMatch(_) => "DC Match Analysis",
            ControlAnalysisResult::Transient(_) => "Transient Analysis",
        }
    }

    pub(super) fn is_complex(&self) -> bool {
        matches!(
            self.result,
            ControlAnalysisResult::Ac(_)
                | ControlAnalysisResult::AcTable(_)
                | ControlAnalysisResult::Distortion(_)
                | ControlAnalysisResult::Stability(_)
                | ControlAnalysisResult::SParameters(_)
        )
    }

    /// Whether a node is an XSPICE event node. ngspice keeps those out of the
    /// analog plot (frontend/outitf.c names only analog nodes and branches);
    /// they answer `eprint` and `edisplay`.
    fn is_event_node(&self, name: &str) -> bool {
        let ControlAnalysisResult::Transient(result) = &self.result else {
            return false;
        };
        result
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .chain(
                result
                    .real_traces
                    .iter()
                    .map(|trace| trace.node_name.as_str()),
            )
            .any(|node| node.eq_ignore_ascii_case(name))
    }

    /// Whether the analysis swept its frequency by decade or octave, which
    /// ngspice records as the scale's GRID_XLOG (spicelib/analysis/acan.c:177,
    /// noisean.c:284, distoan.c:521, cktsens.c:296, span.c:567).
    fn log_scale(&self) -> bool {
        let variation = match &self.command {
            AnalysisCommand::Ac { variation, .. }
            | AnalysisCommand::Sp { variation, .. }
            | AnalysisCommand::Stb { variation, .. }
            | AnalysisCommand::Disto { variation, .. }
            | AnalysisCommand::Noise { variation, .. } => variation,
            AnalysisCommand::Sensitivity {
                ac_sweep: Some(sweep),
                ..
            } => &sweep.variation,
            _ => return false,
        };
        !matches!(variation, crate::netlist::FreqVariation::Lin)
    }

    /// The scale vector's name; a one-point result has none.
    fn scale_vector(&self) -> Option<String> {
        (!matches!(
            self.result,
            ControlAnalysisResult::OperatingPoint(_)
                | ControlAnalysisResult::TransferFunction(_)
                | ControlAnalysisResult::PoleZero(_)
        ))
        .then(|| self.scale_name().to_ascii_lowercase())
    }

    /// `(name, type)` for every retained raw vector; ground is no vector.
    fn vectors(&self, is_ground: impl Fn(&str) -> bool) -> Vec<(String, &'static str)> {
        let mut vectors: Vec<_> = self
            .scale_vector()
            .map(|name| (name, type_name(&self.scale_unit())))
            .into_iter()
            .collect();
        vectors.extend(
            self.node_names()
                .iter()
                .map(|name| name.to_ascii_lowercase())
                .filter(|name| !is_ground(name))
                .map(|name| (name, "voltage")),
        );
        vectors.extend(
            self.branch_names()
                .iter()
                .map(|name| (format!("{}#branch", name.to_ascii_lowercase()), "current")),
        );
        if let ControlAnalysisResult::SParameters(run) = &self.result {
            vectors.extend(super::sparam::names(run));
        }
        vectors
    }
}

/// ngspice `ft_typenames` (frontend/typesdef.c:38) for the units a vector
/// can carry, as `settype` spells them in this host.
fn type_name(unit: &SignalUnit) -> &'static str {
    match unit {
        SignalUnit::Volt => "voltage",
        SignalUnit::Ampere => "current",
        SignalUnit::Second => "time",
        SignalUnit::Hertz => "frequency",
        SignalUnit::Ohm => "impedance",
        SignalUnit::Siemens => "admittance",
        SignalUnit::Watt => "power",
        SignalUnit::Radian => "phase",
        SignalUnit::Custom(unit) => match unit.as_str() {
            "dB" => "decibel",
            "degC" => "temperature",
            "F" => "capacitance",
            "C" => "charge",
            "V/sqrt(Hz)" => "voltage-density",
            "A/sqrt(Hz)" => "current-density",
            "V^2/Hz" => "voltage^2-density",
            "A^2/Hz" => "current^2-density",
            "V^2" => "voltage^2",
            "A^2" => "current^2",
            _ => "notype",
        },
        _ => "notype",
    }
}

/// A `let` vector's `display` line (frontend/plotting/pvec.c).
fn vector_line(name: &str, value: &VectorValue, scale: bool) -> String {
    let numeric = if value.complex { "complex" } else { "real" };
    let mark = if scale { " [default scale]" } else { "" };
    format!(
        "    {name:<20}: {}, {numeric}, {} long{mark}\n",
        type_name(&value.unit),
        value.samples.len()
    )
}

/// `display` of a plot's `(name, line)` pairs: every line under the header,
/// sorted by name as frontend/com_display.c sorts them unless `nosort` is
/// set, or only the requested vectors' lines.
fn listed(
    command: &ControlCommand,
    header: String,
    mut lines: Vec<(String, String)>,
    requested: Vec<crate::engine::control::shell::Word>,
) -> Result<String, ControlError> {
    if requested.is_empty() {
        lines.sort();
        return Ok(lines.into_iter().fold(header, |mut text, (_, line)| {
            text.push_str(&line);
            text
        }));
    }
    // frontend/vectors.c:183 findvec: a name matches without regard to case,
    // then as `v(name)`.
    let find = |name: &str| {
        lines
            .iter()
            .find(|(vector, _)| vector.eq_ignore_ascii_case(name))
    };
    let mut text = String::new();
    for word in requested {
        let name = word.text.as_str();
        let (_, line) = find(name)
            .or_else(|| find(&format!("v({name})")))
            .or_else(|| node_form(name).and_then(|node| find(&node)))
            .ok_or_else(|| command_error(command.line, format!("no such vector as {name}")))?;
        text.push_str(line);
    }
    Ok(text)
}

/// ngspice vec_fromplot (frontend/vectors.c:528): `x(node)` names the node,
/// and `i(node)` its branch current `node#branch`.
fn node_form(word: &str) -> Option<String> {
    let mut chars = word.chars();
    let kind = chars.next().filter(|kind| *kind != '(')?;
    let node = chars.as_str().strip_prefix('(')?.strip_suffix(')')?;
    if node.is_empty() {
        return None;
    }
    Some(if kind.eq_ignore_ascii_case(&'i') {
        format!("{node}#branch")
    } else {
        node.to_owned()
    })
}

impl ControlCircuit {
    /// ngspice `pl_title`: the deck's title line, which ngspice reads in
    /// lower case as it reads every line of the deck (frontend/inpcom.c:3421
    /// inp_casefix).
    pub(in crate::engine::control) fn title(&self) -> String {
        self.netlist.title.to_ascii_lowercase()
    }

    /// `echo [-n] words...`: the words after substitution, unquoted and
    /// joined by single spaces.
    pub(in crate::engine) fn echo(command: &ControlCommand) -> String {
        let mut words: Vec<_> = crate::engine::control::shell::words(&command.arguments)
            .into_iter()
            .map(|word| word.text)
            .collect();
        let newline = if words.first().is_some_and(|word| word == "-n") {
            words.remove(0);
            ""
        } else {
            "\n"
        };
        format!("{}{newline}", words.join(" "))
    }

    /// `display [vectors...]` on the current plot: a dataset's vectors with
    /// the `let` vectors bound in it, or a plot `setplot new` made.
    pub(in crate::engine) fn display(
        &self,
        command: &ControlCommand,
    ) -> Result<String, ControlError> {
        let requested = crate::engine::control::shell::words(&command.arguments);
        if let Some(plot) = self.vectors.current_plot() {
            let scale = self.vectors.plot_scale(plot);
            let lines = self
                .vectors
                .vectors_in(plot)
                .map(|(name, value)| {
                    let name = name.to_ascii_lowercase();
                    let scale = scale.is_some_and(|scale| scale.eq_ignore_ascii_case(&name));
                    let line = vector_line(&name, value, scale);
                    (name, line)
                })
                .collect();
            let (title, kind) = self
                .vectors
                .plot_kind(plot)
                .map_or(("Anonymous".to_owned(), "unknown"), |kind| {
                    (self.title(), kind)
                });
            let header = format!(
                "Here are the vectors currently active:\n\nTitle: {title}\nName: {plot} ({kind})\n\n"
            );
            return listed(command, header, lines, requested);
        }
        let Some(dataset) = self.current_dataset() else {
            return Ok("There are no vectors currently active.\n".into());
        };
        let length = dataset.length();
        let numeric = if dataset.is_complex() {
            "complex"
        } else {
            "real"
        };
        let policy = self.netlist.ground_policy();
        let vectors = dataset.vectors(|name| policy.is_ground(name) || dataset.is_event_node(name));
        // An operating point sweeps nothing, so its first vector is its scale
        // (frontend/vectors.c vec_new): the circuit's first node.
        let scale = dataset.scale_vector().or_else(|| {
            matches!(dataset.result, ControlAnalysisResult::OperatingPoint(_))
                .then(|| vectors.first().map(|(name, _)| name.clone()))
                .flatten()
        });
        let grid = if dataset.log_scale() {
            ", grid = xlog"
        } else {
            ""
        };
        let line = |name: &str, kind: &str| {
            let (grid, mark) = if Some(name) == scale.as_deref() {
                (grid, " [default scale]")
            } else {
                ("", "")
            };
            format!("    {name:<20}: {kind}, {numeric}, {length} long{grid}{mark}\n")
        };
        // A `let` of a dataset vector's name replaces it in the plot.
        let mut lines: Vec<_> = self
            .vectors
            .vectors_in(&dataset.name)
            .map(|(name, value)| {
                let name = name.to_ascii_lowercase();
                let line = vector_line(&name, value, false);
                (name, line)
            })
            .collect();
        for (name, kind) in vectors {
            if !lines.iter().any(|(bound, _)| *bound == name) {
                let line = line(&name, kind);
                lines.push((name, line));
            }
        }
        let header = format!(
            "Here are the vectors currently active:\n\nTitle: {}\nName: {} ({})\n\n",
            self.title(),
            dataset.name,
            dataset.plot_title()
        );
        listed(command, header, lines, requested)
    }

    /// ngspice `destroy [all | plot ...]` (frontend/postcoms.c:891): the
    /// plots leave the plot list with their vectors, and `destroy` alone
    /// takes the current plot. The constants plot stays. A destroyed
    /// dataset's results stay published; when it was current, the newest
    /// remaining dataset is.
    pub(in crate::engine) fn destroy(
        &mut self,
        command: &ControlCommand,
    ) -> Result<(), ControlError> {
        let line = command.line;
        let words = crate::engine::control::shell::words(&command.arguments);
        let names: Vec<String> = match words.as_slice() {
            [] => vec![self.plot_name()],
            [all] if all.text.eq_ignore_ascii_case("all") => {
                // postcoms.c:907: plot names number from 1 again.
                self.plot_number = 1;
                (0..self.datasets.len())
                    .filter(|index| !self.destroyed.contains(index))
                    .map(|index| self.datasets[index].name.clone())
                    .chain(self.vectors.plot_names())
                    .collect()
            }
            words => words.iter().map(|word| word.text.clone()).collect(),
        };
        for name in names {
            if name.eq_ignore_ascii_case("const") {
                return Err(command_error(line, "destroy: the constants plot stays"));
            }
            if let Some(index) = self.dataset_index(&name) {
                self.destroyed.insert(index);
                self.vectors.remove_vectors_of(&name);
                if self.current == Some(index) {
                    self.current = (0..self.datasets.len())
                        .rev()
                        .find(|index| !self.destroyed.contains(index));
                }
            } else if !self.vectors.remove_plot(&name) {
                return Err(command_error(
                    line,
                    format!("destroy: no plot named '{name}'"),
                ));
            }
        }
        Ok(())
    }

    /// `setplot` with no plot name: ngspice lists the plots newest first,
    /// the plots `setplot new` made among them, and the constants plot last
    /// (frontend/vectors.c:902 `plot_add` puts each new plot at the head).
    pub(in crate::engine) fn plot_list(&self) -> String {
        let current = self.plot_name();
        let made = self.vectors.plots_made();
        let mut made = made.iter().peekable();
        let title = self.title();
        let mut plots = Vec::new();
        let made_line = |kind: Option<&str>| {
            kind.map_or("Anonymous (unknown)".to_string(), |kind| {
                format!("{title} ({kind})")
            })
        };
        for (index, dataset) in self.datasets.iter().enumerate() {
            while let Some((name, _, kind)) = made.next_if(|(_, after, _)| *after <= index) {
                plots.push((name.to_string(), made_line(*kind)));
            }
            if !self.destroyed.contains(&index) {
                plots.push((
                    dataset.name.clone(),
                    format!("{title} ({})", dataset.plot_title()),
                ));
            }
        }
        for (name, _, kind) in made {
            plots.push((name.to_string(), made_line(*kind)));
        }
        let mut text = String::from("List of plots available:\n\n");
        let constants = (
            "const".to_string(),
            "Constant values (constants)".to_string(),
        );
        for (name, description) in plots.iter().rev().chain([&constants]) {
            let prefix = if name.eq_ignore_ascii_case(&current) {
                "Current "
            } else {
                "\t"
            };
            text.push_str(&format!("{prefix}{name}\t{description}\n"));
        }
        text
    }
}

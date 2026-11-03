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

/// ngspice `ft_typenames` for the units a raw vector can carry.
fn type_name(unit: &SignalUnit) -> &'static str {
    match unit {
        SignalUnit::Volt => "voltage",
        SignalUnit::Ampere => "current",
        SignalUnit::Second => "time",
        SignalUnit::Hertz => "frequency",
        SignalUnit::Ohm => "impedance",
        SignalUnit::Siemens => "admittance",
        _ => "notype",
    }
}

impl ControlCircuit {
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

    /// `display [vectors...]` on the current dataset.
    pub(in crate::engine) fn display(
        &self,
        command: &ControlCommand,
    ) -> Result<String, ControlError> {
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
        let mut vectors = dataset.vectors(|name| policy.is_ground(name));
        // An operating point sweeps nothing, so its first vector is its scale
        // (frontend/vectors.c vec_new): the circuit's first node.
        let scale = dataset.scale_vector().or_else(|| {
            matches!(dataset.result, ControlAnalysisResult::OperatingPoint(_))
                .then(|| vectors.first().map(|(name, _)| name.clone()))
                .flatten()
        });
        let line = |name: &str, kind: &str| {
            let mark = if Some(name) == scale.as_deref() {
                " [default scale]"
            } else {
                ""
            };
            format!("    {name:<20}: {kind}, {numeric}, {length} long{mark}\n")
        };
        let requested = crate::engine::control::shell::words(&command.arguments);
        if !requested.is_empty() {
            let mut text = String::new();
            for word in requested {
                let name = word.text.to_ascii_lowercase();
                let (_, kind) = vectors
                    .iter()
                    .find(|(vector, _)| *vector == name)
                    .ok_or_else(|| {
                        command_error(command.line, format!("no such vector as {name}"))
                    })?;
                text.push_str(&line(&name, kind));
            }
            return Ok(text);
        }
        // com_display.c sorts by name unless `nosort` is set.
        vectors.sort();
        let mut text = format!(
            "Here are the vectors currently active:\n\nTitle: {}\nName: {} ({})\n\n",
            self.netlist.title,
            dataset.name,
            dataset.plot_title()
        );
        for (name, kind) in &vectors {
            text.push_str(&line(name, kind));
        }
        Ok(text)
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
        let mut plots = Vec::new();
        for (index, dataset) in self.datasets.iter().enumerate() {
            while let Some((name, _)) = made.next_if(|(_, after)| *after <= index) {
                plots.push((name.to_string(), "Anonymous (unknown)".to_string()));
            }
            if !self.destroyed.contains(&index) {
                plots.push((
                    dataset.name.clone(),
                    format!("{} ({})", self.netlist.title, dataset.plot_title()),
                ));
            }
        }
        for (name, _) in made {
            plots.push((name.to_string(), "Anonymous (unknown)".to_string()));
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

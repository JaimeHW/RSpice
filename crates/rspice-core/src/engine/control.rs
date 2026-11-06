//! Electrical host for the shared resumable control-command machine.

use super::{
    Engine, FrequencyDataResult, SimulationConfigError, SimulationError, TransientStartupMode,
};
use crate::analysis::AcResult;
use crate::analysis::transient::TransientResult;
use crate::control_protocol::{
    ControlCommand, ControlError, ControlErrorKind, ControlScalarEvaluator,
};
use crate::netlist::expr::{
    Expr, ParamContext, eval_expression_complex_with_abort,
    evaluate_complex_with_functions_and_abort, is_real, parse_control_expression_with_abort,
};
use crate::netlist::{AnalysisCommand, Netlist};
use crate::resource::{ResourceKind, ResourceLimitError};
use crate::{AbortSignal, ComplexValue, Value};
use std::collections::{BTreeMap, BTreeSet};

mod analysis;
mod options;
mod presentation;
/// Shared with the session, which sits above this host.
pub(crate) mod shell;
mod transient;
pub use options::ControlSettings;
pub(super) use options::apply_runtime_options;
use presentation::VectorValue;
pub use presentation::{
    ControlCurrentSource, ControlPlotOptions, ControlPresentation, ControlPresentationKind,
    ControlScalar, ControlTrace, ControlVector, ControlVectorId, ControlVoltageSource,
};

#[derive(Debug, thiserror::Error)]
pub enum ControlExecutionError {
    #[error(transparent)]
    Command(#[from] ControlError),
    #[error("control line {line}: {source}")]
    Simulation {
        line: usize,
        #[source]
        source: SimulationError,
    },
    #[error("control line {line}: {source}")]
    Configuration {
        line: usize,
        #[source]
        source: SimulationConfigError,
    },
}

#[derive(Debug)]
pub enum ControlAnalysisResult {
    OperatingPoint(Box<crate::solver::SimulationResult>),
    Ac(Vec<AcResult>),
    AcTable(FrequencyDataResult<AcResult>),
    Distortion(Box<crate::analysis::DistortionAnalysisResult>),
    Stability(Box<super::StbAnalysisResult>),
    /// An `sp` sweep's scattering parameters.
    SParameters(Box<super::SParameterRun>),
    Noise(Vec<crate::analysis::NoiseResult>),
    NoiseTable(FrequencyDataResult<crate::analysis::NoiseResult>),
    DcSweep(Box<super::DcSweepResult>),
    TransferFunction(Box<crate::analysis::TransferFunctionResult>),
    PoleZero(Box<crate::analysis::PoleZeroResult>),
    Sensitivity(Box<super::SensitivityCardResult>),
    DcMatch(Box<crate::analysis::dcmatch::DcMatchResult>),
    Transient(Box<TransientResult>),
}

#[derive(Debug)]
pub struct ControlNamedDataset {
    pub name: String,
    pub analysis_id: crate::identity::AnalysisInstanceId,
    pub device_op_report: Option<Box<crate::circuit::DeviceOpReport>>,
    pub command: AnalysisCommand,
    pub result: ControlAnalysisResult,
}

#[derive(Debug)]
pub enum ControlCommandEffect {
    CircuitChanged,
    /// Names of newly retained, independently owned result datasets.
    Analyses(Vec<String>),
    /// The presentation host must handle this request before advancing the
    /// command session. Returning it is not a claim that rendering occurred.
    Presentation(ControlPresentation),
}

enum PreparedCommand {
    Complete(ControlCommandEffect),
    Analyses(Vec<(AnalysisCommand, Option<usize>)>),
}

enum CommandKind {
    Options,
    Set,
    Unset,
    Save,
    Destroy,
    /// `unlet vectors`.
    Unlet,
    /// `reset`: reload the deck.
    Reset,
    Alter,
    /// `altermod`: set a parameter of a model card.
    AlterModel,
    /// `alterparam`: change a `.param` of the deck `reset` loads.
    AlterParam,
    Analysis,
    Run,
    Presentation,
    Echo,
    Display,
    /// `cd directory`.
    ChangeDirectory,
    /// XSPICE `edisplay`.
    EventDisplay,
    /// XSPICE `eprint`.
    EventPrint,
    /// XSPICE `eprvcd`.
    EventVcd,
    Listing,
    ResourceUsage,
    SetPlot,
    Compose,
    /// `wrs2p [file]`.
    WriteSParameters,
    /// `wrdata file vectors`.
    WriteData,
    /// `cutout`: a transient plot of a window of the current one.
    Cutout,
    /// `linearize`: a transient plot of the current one on an even grid.
    Linearize,
    /// `fft`: a spectrum plot of the current transient plot's vectors.
    Fft,
    /// `meas` and `measure`.
    Measure,
    /// `setscale [vector]`: the current plot's scale.
    SetScale,
    /// A command ngspice -b accepts with no batch effect.
    Ignored(Ignored),
}

#[derive(Clone, Copy)]
enum Ignored {
    /// Plotting and hardcopy devices: a batch run has no window or printer.
    Device,
    /// Interactive editors, help and history.
    Interactive,
    /// Commands that would start another program on the host.
    HostProgram,
}

impl CommandKind {
    /// The commands whose standard output a redirection can send to a file.
    fn prints_text(&self) -> bool {
        matches!(
            self,
            Self::Echo
                | Self::Display
                | Self::EventDisplay
                | Self::EventPrint
                | Self::EventVcd
                | Self::Measure
        )
    }

    fn parse(command: &ControlCommand) -> Result<Self, ControlError> {
        Ok(match command.name.as_str() {
            "option" | "options" => Self::Options,
            "set" | "setcs" => Self::Set,
            "unset" => Self::Unset,
            "save" => Self::Save,
            "destroy" => Self::Destroy,
            "unlet" => Self::Unlet,
            "reset" => Self::Reset,
            "alter" => Self::Alter,
            "altermod" => Self::AlterModel,
            "alterparam" => Self::AlterParam,
            "op" | "dc" | "ac" | "noise" | "tran" | "tf" | "pz" | "sens" | "disto" | "stb"
            | "dcmatch" | "sp" => Self::Analysis,
            "run" => Self::Run,
            "plot" | "print" | "settype" => Self::Presentation,
            "echo" => Self::Echo,
            "display" => Self::Display,
            "edisplay" => Self::EventDisplay,
            "cd" => Self::ChangeDirectory,
            "eprint" => Self::EventPrint,
            "eprvcd" => Self::EventVcd,
            "listing" => Self::Listing,
            "rusage" => Self::ResourceUsage,
            "setplot" => Self::SetPlot,
            "compose" => Self::Compose,
            "wrs2p" => Self::WriteSParameters,
            "wrdata" => Self::WriteData,
            "cutout" => Self::Cutout,
            "linearize" => Self::Linearize,
            "fft" => Self::Fft,
            "meas" | "measure" => Self::Measure,
            "setscale" => Self::SetScale,
            "gnuplot" | "hardcopy" | "asciiplot" | "iplot" | "xgraph" | "bltplot" => {
                Self::Ignored(Ignored::Device)
            }
            "edit" | "help" | "history" => Self::Ignored(Ignored::Interactive),
            "shell" => Self::Ignored(Ignored::HostProgram),
            _ => {
                return Err(command_error(
                    command.line,
                    format!(
                        "control command '{}' has no electrical or presentation handler",
                        command.name
                    ),
                ));
            }
        })
    }
}

/// Mutable circuit state and immutable completed datasets for one script.
///
/// Construct the netlist from `ControlProgram::declarative_source`, using the
/// caller's normal path/sealed-source parser. Drive this host with that
/// program's `ControlSession`. This preserves one execution path on all targets.
pub struct ControlCircuit {
    netlist: Netlist,
    /// The deck as the script found it, which `reset` reloads.
    authored: Netlist,
    datasets: Vec<ControlNamedDataset>,
    /// The dataset unqualified vector names resolve against: the newest,
    /// unless `setplot` selected another (ngspice `plot_cur`).
    current: Option<usize>,
    /// Datasets `destroy` took out of the plot list. Their results stay
    /// retained for publication; no name reads them.
    destroyed: BTreeSet<usize>,
    /// Datasets run per analysis kind, which number their identities.
    ordinals: BTreeMap<&'static str, usize>,
    /// ngspice's `plot_num` (frontend/plotting/plotting.c:18), the number a
    /// new plot's name takes unless a live plot already has that name.
    plot_number: usize,
    retained_values: usize,
    vector_units: BTreeMap<ControlVectorId, crate::signal_unit::SignalUnit>,
    settings: ControlSettings,
    runtime_options: crate::netlist::SimulationOptions,
    /// Vectors bound by `let`, per plot.
    vectors: presentation::VectorStore,
    /// The engine's result-value ceiling, which `let` vectors share with the
    /// retained datasets. Refreshed from the engine on every command.
    value_limit: usize,
    /// ngspice `inputdir`: the directory the deck was read from, as the
    /// presentation host names it.
    input_directory: Option<String>,
    /// The directory `cd` moved to, which a relative redirection's file is
    /// in; `None` is the presentation host's own, the deck's directory.
    directory: Option<String>,
    /// The shell variables `set` gave, which steer what `wrdata` writes
    /// (`wr_singlescale`, `wr_vecnames`, `wr_onespace`, `appendwrite`,
    /// `numdgt`).
    shell: BTreeMap<String, crate::engine::control::shell::ShellValue>,
}

impl ControlCircuit {
    pub fn new(netlist: Netlist) -> Result<Self, ControlExecutionError> {
        if !netlist.control_dispositions.is_empty() {
            return Err(command_error(0, "construct the control circuit from the program's declarative source; promoted script commands would duplicate execution").into());
        }
        Ok(Self {
            authored: netlist.clone(),
            netlist,
            datasets: Vec::new(),
            current: None,
            destroyed: BTreeSet::new(),
            ordinals: BTreeMap::new(),
            plot_number: 1,
            retained_values: 0,
            vector_units: BTreeMap::new(),
            settings: ControlSettings::default(),
            runtime_options: crate::netlist::SimulationOptions::default(),
            vectors: presentation::VectorStore::default(),
            value_limit: crate::resource::ResourceLimits::default().max_result_values,
            input_directory: None,
            directory: None,
            shell: BTreeMap::new(),
        })
    }

    /// Name the directory the deck was read from, which a script reads as
    /// `$inputdir` (ngspice frontend/inp.c:589, the path's directory part,
    /// or `.`). Without it `$inputdir` is undefined.
    pub fn set_input_directory(&mut self, directory: String) {
        self.input_directory = Some(directory);
    }

    pub fn netlist(&self) -> &Netlist {
        &self.netlist
    }

    pub fn settings(&self) -> &ControlSettings {
        &self.settings
    }

    pub fn datasets(&self) -> &[ControlNamedDataset] {
        &self.datasets
    }

    fn current_dataset(&self) -> Option<&ControlNamedDataset> {
        if self.vectors.current_plot().is_some() {
            return None;
        }
        self.current.and_then(|index| self.datasets.get(index))
    }

    /// ngspice `plot_setcur` (frontend/vectors.c:1160): `new` makes an empty
    /// plot current, any other name the plot of that name.
    fn set_plot(&mut self, name: &str, line: usize) -> Result<(), ControlError> {
        if name.eq_ignore_ascii_case("new") {
            // frontend/vectors.c:1164 plot_alloc("unknown").
            let name = self.new_plot_name("unknown", line)?;
            self.vectors.new_plot(name, self.datasets.len());
        } else if let Some(index) = self.dataset_index(name) {
            self.current = Some(index);
            self.vectors.leave_plot();
        } else if !self.vectors.select_plot(name) {
            return Err(command_error(
                line,
                format!("setplot: no plot named '{name}'"),
            ));
        }
        Ok(())
    }

    /// ngspice `plot_alloc` (frontend/vectors.c:897): the plot's type and
    /// `plot_num`, which only a live plot of that name advances. A
    /// destroyed plot's name is free again, so `op` after `destroy op1`
    /// makes `op1`, and `op`, `op`, `tran` make `op1`, `op2` and `tran2`.
    fn new_plot_name(&mut self, kind: &str, line: usize) -> Result<String, ControlError> {
        loop {
            let name = format!("{kind}{}", self.plot_number);
            if self.dataset_index(&name).is_none() && !self.vectors.has_plot(&name) {
                return Ok(name);
            }
            self.plot_number = self
                .plot_number
                .checked_add(1)
                .ok_or_else(|| command_error(line, "plot number overflow"))?;
        }
    }

    /// Consume the completed session for publication without cloning results.
    pub fn into_datasets(self) -> Vec<ControlNamedDataset> {
        self.datasets
    }

    fn prepare_command(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedCommand, ControlExecutionError> {
        let line = command.line;
        if abort.is_aborted() {
            return Err(simulation_error(line, SimulationError::Aborted));
        }
        ResourceLimitError::ensure(
            ResourceKind::NetlistBytes,
            command.name.len().saturating_add(command.arguments.len()),
            engine.config().resource_limits.max_netlist_bytes,
        )
        .map_err(|error| simulation_error(line, error.into()))?;
        self.value_limit = engine.config().resource_limits.max_result_values;
        if command.arguments.contains(['\r', '\n']) {
            return Err(command_error(
                line,
                "a control command cannot contain another physical line",
            )
            .into());
        }
        let kind = CommandKind::parse(command)?;
        // ngspice cp_redirect (frontend/streams.c:47) sends the output of
        // any command to the file a `>` names.
        if kind.prints_text()
            && let (arguments, Some(redirect)) = shell::split_redirect(&command.arguments, line)?
        {
            let command = ControlCommand {
                arguments,
                ..command.clone()
            };
            if let CommandKind::Measure = kind {
                // A failed measurement's line goes to the terminal, not the
                // file (measure.c:131 prints it on stdout).
                let (text, measured) = self.measure(&command, variables, abort)?;
                let kind = if measured {
                    ControlPresentationKind::File {
                        path: self.script_file(redirect.path),
                        text,
                        append: redirect.append,
                    }
                } else {
                    ControlPresentationKind::Text(text)
                };
                return Ok(Self::batch_output(&command, kind));
            }
            let mut prepared = self.prepare_kind(kind, engine, &command, variables, abort)?;
            if let PreparedCommand::Complete(ControlCommandEffect::Presentation(request)) =
                &mut prepared
                && let ControlPresentationKind::Text(text) = &mut request.kind
            {
                request.kind = ControlPresentationKind::File {
                    path: self.script_file(redirect.path),
                    text: std::mem::take(text),
                    append: redirect.append,
                };
            }
            return Ok(prepared);
        }
        self.prepare_kind(kind, engine, command, variables, abort)
    }

    fn prepare_kind(
        &mut self,
        kind: CommandKind,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedCommand, ControlExecutionError> {
        let line = command.line;
        match kind {
            CommandKind::Options => {
                self.apply_options(engine, command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Set => {
                self.apply_set(engine, command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Unset => {
                self.apply_unset(command)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            // ngspice `save` (frontend/breakp2.c:34 com_save) narrows the
            // vectors a later analysis keeps. Every analysis here keeps all
            // of its vectors, so each one the list names stays readable.
            CommandKind::Save => Ok(PreparedCommand::Complete(
                ControlCommandEffect::CircuitChanged,
            )),
            CommandKind::Destroy => {
                self.destroy(command)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Cutout => {
                self.cutout(command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Fft => {
                let report = self.fft(command, variables, abort)?;
                if report.is_empty() {
                    Ok(PreparedCommand::Complete(
                        ControlCommandEffect::CircuitChanged,
                    ))
                } else {
                    Ok(Self::batch_output(
                        command,
                        ControlPresentationKind::Text(report),
                    ))
                }
            }
            CommandKind::Linearize => {
                let report = self.linearize(command, variables, abort)?;
                if report.is_empty() {
                    Ok(PreparedCommand::Complete(
                        ControlCommandEffect::CircuitChanged,
                    ))
                } else {
                    Ok(Self::batch_output(
                        command,
                        ControlPresentationKind::Text(report),
                    ))
                }
            }
            CommandKind::Reset => {
                // ngspice `reset` (frontend/runcoms2.c:175) removes the circuit
                // and loads its deck again: alterations and `option` values
                // go, the plots and the script's variables stay
                // (frontend/inp.c:549 names the deck it reloads).
                self.netlist = self.authored.clone();
                self.runtime_options = crate::netlist::SimulationOptions::default();
                Ok(Self::batch_output(
                    command,
                    ControlPresentationKind::Text(format!(
                        "Reset re-loads circuit {}\n",
                        self.title()
                    )),
                ))
            }
            CommandKind::Unlet => {
                self.unlet(command)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Alter => {
                self.alter(engine, command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::AlterModel => {
                self.alter_model(command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::AlterParam => {
                self.alter_parameter(engine, command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Analysis => {
                let analysis =
                    Self::parse_analysis_command(command, engine.config().resource_limits, abort)?;
                Ok(PreparedCommand::Analyses(vec![(analysis, None)]))
            }
            CommandKind::Run => {
                if !command.arguments.is_empty() {
                    return Err(command_error(line, "run does not accept arguments").into());
                }
                // `run` solves each small-signal card at the operating point,
                // so a card that names a transient state would be answered
                // around the wrong state.
                if let Some((card, state)) =
                    self.netlist
                        .analyses
                        .iter()
                        .enumerate()
                        .find_map(|(index, card)| {
                            self.netlist
                                .small_signal_state(index)
                                .map(|state| (card, state))
                        })
                {
                    let card = analysis::identity(card, line).map_or_else(
                        |_| "a small-signal".to_owned(),
                        |(name, _)| format!(".{name}"),
                    );
                    let keyword = state.keyword();
                    return Err(command_error(
                        line,
                        format!(
                            "run cannot execute the {card} card: it carries {keyword}=, which \
                             linearizes it around a transient state, and a control script cannot \
                             run that yet. Run the deck without its .control block, or remove \
                             {keyword}= to linearize at the operating point."
                        ),
                    )
                    .into());
                }
                let analyses = self
                    .netlist
                    .analyses
                    .iter()
                    .enumerate()
                    .filter(|(_, command)| {
                        !matches!(
                            command,
                            AnalysisCommand::Four { .. }
                                | AnalysisCommand::Step(_)
                                | AnalysisCommand::Temp { .. }
                        )
                    })
                    .map(|(index, command)| (command.clone(), Some(index)))
                    .collect::<Vec<_>>();
                if analyses.is_empty() {
                    // ngspice's `run` executes the deck's own analysis cards
                    // (frontend/spiceif.c:347 ci_defTask), so after the script
                    // ran its own it does nothing. With nothing run at all, the
                    // script produces no result, which is a mistake to report.
                    if self.datasets.is_empty() {
                        return Err(command_error(
                            line,
                            "run has no declarative analysis to execute",
                        )
                        .into());
                    }
                    return Ok(Self::batch_output(
                        command,
                        ControlPresentationKind::Ignored(
                            "run: the deck has no analysis card, and the script's own analyses \
                             already ran"
                                .into(),
                        ),
                    ));
                }
                Ok(PreparedCommand::Analyses(analyses))
            }
            CommandKind::Presentation => match self.present(engine, command, variables, abort) {
                Ok(request) => Ok(PreparedCommand::Complete(
                    ControlCommandEffect::Presentation(request),
                )),
                // ngspice -b never draws, so a plot it cannot resolve does not
                // stop its run; an rspice plot file is a convenience on top.
                Err(ControlExecutionError::Command(error))
                    if command.name == "plot"
                        && !matches!(
                            error.kind,
                            ControlErrorKind::Aborted | ControlErrorKind::ResourceLimit
                        ) =>
                {
                    Ok(Self::batch_output(
                        command,
                        ControlPresentationKind::Ignored(format!(
                            "plot not drawn: {}",
                            error.message
                        )),
                    ))
                }
                Err(error) => Err(error),
            },
            CommandKind::Echo => Ok(Self::batch_output(
                command,
                ControlPresentationKind::Text(Self::echo(command)),
            )),
            CommandKind::SetScale => {
                let report = self.set_scale(command)?;
                if report.is_empty() {
                    Ok(PreparedCommand::Complete(
                        ControlCommandEffect::CircuitChanged,
                    ))
                } else {
                    Ok(Self::batch_output(
                        command,
                        ControlPresentationKind::Text(report),
                    ))
                }
            }
            CommandKind::Display => Ok(Self::batch_output(
                command,
                ControlPresentationKind::Text(self.display(command)?),
            )),
            CommandKind::ChangeDirectory => {
                // frontend/com_chdir.c: the first word names the directory.
                // ngspice's bare `cd` goes to the home directory, which a
                // batch run's files may not be written in.
                let words = shell::words(&command.arguments);
                let Some(directory) = words.first() else {
                    return Err(command_error(
                        line,
                        "cd needs a directory; a control script writes files only in the deck's \
                         directory",
                    )
                    .into());
                };
                let base = std::path::Path::new(self.directory.as_deref().unwrap_or(""));
                self.directory = Some(base.join(&directory.text).to_string_lossy().into_owned());
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::EventDisplay => Ok(Self::batch_output(
                command,
                ControlPresentationKind::Text(self.event_display(command)?),
            )),
            CommandKind::EventPrint => Ok(Self::batch_output(
                command,
                ControlPresentationKind::Text(self.event_print(command)?),
            )),
            CommandKind::EventVcd => Ok(Self::batch_output(
                command,
                ControlPresentationKind::Text(self.event_vcd(command)?),
            )),
            CommandKind::Listing => Ok(Self::batch_output(
                command,
                ControlPresentationKind::Listing,
            )),
            CommandKind::ResourceUsage => Ok(Self::batch_output(
                command,
                ControlPresentationKind::ResourceUsage,
            )),
            CommandKind::SetPlot => {
                let words = shell::words(&command.arguments);
                let [name] = words.as_slice() else {
                    if words.is_empty() {
                        return Ok(Self::batch_output(
                            command,
                            ControlPresentationKind::Text(self.plot_list()),
                        ));
                    }
                    return Err(command_error(line, "setplot takes one plot name").into());
                };
                self.set_plot(&name.text, line)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Measure => Ok(Self::batch_output(
                command,
                ControlPresentationKind::Text(self.measure(command, variables, abort)?.0),
            )),
            CommandKind::WriteSParameters => {
                let (path, text) = self.touchstone(command, variables, abort)?;
                Ok(Self::batch_output(
                    command,
                    ControlPresentationKind::File {
                        path: self.script_file(path),
                        text,
                        append: false,
                    },
                ))
            }
            CommandKind::WriteData => match self.write_data(command, variables, abort)? {
                Some((path, text)) => Ok(Self::batch_output(
                    command,
                    ControlPresentationKind::File {
                        path: self.script_file(path),
                        text,
                        append: self.shell.contains_key("appendwrite"),
                    },
                )),
                None => Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                )),
            },
            CommandKind::Compose => {
                self.compose(command, variables, abort)?;
                Ok(PreparedCommand::Complete(
                    ControlCommandEffect::CircuitChanged,
                ))
            }
            CommandKind::Ignored(kind) => {
                let reason = match kind {
                    Ignored::Device | Ignored::Interactive => format!(
                        "command '{}' is not available during batch simulation, ignored",
                        command.name
                    ),
                    Ignored::HostProgram => format!(
                        "command '{}' would start a host program; a control script cannot, ignored",
                        command.name
                    ),
                };
                Ok(Self::batch_output(
                    command,
                    ControlPresentationKind::Ignored(reason),
                ))
            }
        }
    }

    /// A relative file is in the directory `cd` moved to.
    fn script_file(&self, path: String) -> String {
        match &self.directory {
            Some(directory) => std::path::Path::new(directory)
                .join(&path)
                .to_string_lossy()
                .into_owned(),
            None => path,
        }
    }

    fn batch_output(command: &ControlCommand, kind: ControlPresentationKind) -> PreparedCommand {
        PreparedCommand::Complete(ControlCommandEffect::Presentation(ControlPresentation {
            command: command.clone(),
            kind,
            scalars: Vec::new(),
        }))
    }

    fn run_analysis<E, F>(
        &mut self,
        engine: &Engine,
        analysis: AnalysisCommand,
        authored_index: Option<usize>,
        line: usize,
        abort: &dyn AbortSignal,
        runner: &mut F,
    ) -> Result<String, E>
    where
        E: From<ControlExecutionError> + From<ControlError>,
        F: FnMut(
            &Engine,
            &Netlist,
            &AnalysisCommand,
            crate::identity::AnalysisInstanceId,
            &dyn AbortSignal,
        ) -> Result<TransientResult, E>,
    {
        engine
            .ensure_batch_runs(self.datasets.len().saturating_add(1))
            .map_err(|error| simulation_error(line, error))?;
        // Resolve authored options before marking this bounded engine resolved.
        // Then apply only settings changed by executed commands: a caller's
        // resolved policy must not be overwritten by unrelated authored options.
        let resolved = engine.resolved_for_netlist(&self.netlist);
        let mut configured = crate::resolve_simulation_config(
            resolved.config(),
            Some(&self.runtime_options),
            &crate::SimulationConfigOverrides::default(),
        );
        if let Some(workers) = self.settings.maximum_parallel_workers {
            configured.resource_limits.max_parallel_workers =
                configured.resource_limits.max_parallel_workers.min(workers);
        }
        configured.resource_limits.max_result_values = configured
            .resource_limits
            .max_result_values
            .saturating_sub(self.retained_values);
        let bounded = if matches!(
            analysis,
            AnalysisCommand::AcData { .. } | AnalysisCommand::NoiseData { .. }
        ) {
            engine.control_frequency_table_engine(&self.runtime_options, configured.resource_limits)
        } else {
            engine.try_resolved_with_config(configured)
        }
        .map_err(|source| ControlExecutionError::Configuration { line, source })?;
        let (kind, identity_kind) = analysis::identity(&analysis, line)?;
        let ordinal = self
            .ordinals
            .get(kind)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| command_error(line, "dataset ordinal overflow"))?;
        let identity_ordinal = u32::try_from(ordinal - 1)
            .map_err(|_| command_error(line, "analysis identity ordinal overflow"))?;
        let analysis_id = crate::identity::AnalysisInstanceId::new(identity_kind, identity_ordinal);
        let name = self.new_plot_name(kind, line)?;
        let netlist =
            transient::analysis_netlist(&self.netlist, &analysis, authored_index, line, abort)?;
        let mut device_op_report = None;
        let (kind, result, count) = match &analysis {
            AnalysisCommand::Op => {
                let (result, report) = bounded
                    .run_dc_op_with_report_and_abort(&netlist, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = report.entries.iter().fold(
                    Engine::simulation_result_value_count(&result),
                    |count, entry| count.saturating_add(entry.params.len()),
                );
                device_op_report = Some(Box::new(report));
                (
                    "op",
                    ControlAnalysisResult::OperatingPoint(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode,
                sweep2,
                restart,
            } => {
                let spec = crate::netlist::DcSweepSpec {
                    start: *start,
                    stop: *stop,
                    step: *step,
                    mode: mode.clone(),
                    restart: *restart,
                };
                let result = bounded
                    .run_dc_analysis_with_abort(&netlist, source, &spec, sweep2.as_ref(), abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result.value_count();
                (
                    "dc",
                    ControlAnalysisResult::DcSweep(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Ac { .. }
            | AnalysisCommand::Noise { .. }
            | AnalysisCommand::Disto { .. } => {
                let (result, count) =
                    analysis::frequency(&bounded, &netlist, &analysis, line, abort)?;
                (kind, result, count)
            }
            AnalysisCommand::AcData { .. } | AnalysisCommand::NoiseData { .. } => {
                let (result, count) =
                    analysis::frequency_table(&bounded, &netlist, &analysis, line, abort)?;
                (kind, result, count)
            }
            AnalysisCommand::Tran { .. } => {
                let result = runner(&bounded, &netlist, &analysis, analysis_id, abort)?;
                let count = Engine::transient_result_value_count(&result);
                (
                    "tran",
                    ControlAnalysisResult::Transient(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Stb { .. } => {
                let config = crate::analysis::StbConfig::try_from(&analysis)
                    .map_err(|error| command_error(line, error.to_string()))?;
                let result = bounded
                    .run_stb_with_abort(&netlist, config, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result.retained_value_count();
                (
                    kind,
                    ControlAnalysisResult::Stability(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Sp { .. } => {
                let result = bounded
                    .run_sp_with_abort(&netlist, &analysis, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let ports = result.scattering.num_ports;
                let count = result
                    .scattering
                    .data
                    .len()
                    .saturating_mul(ports.saturating_mul(ports).saturating_add(1));
                (
                    kind,
                    ControlAnalysisResult::SParameters(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::PoleZero { .. } => {
                let result = bounded
                    .run_pz_from_card_with_abort(&netlist, &analysis, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result.retained_value_count();
                (
                    kind,
                    ControlAnalysisResult::PoleZero(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Sensitivity { .. } => {
                let result = bounded
                    .run_sensitivity_from_card_with_abort(&netlist, &analysis, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result.retained_value_count();
                (
                    kind,
                    ControlAnalysisResult::Sensitivity(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::DcMatch(card) => {
                let result = bounded
                    .run_dc_match_with_abort(&netlist, card, abort)
                    .map_err(|error| simulation_error(line, error))?;
                let count = result.retained_value_count();
                (
                    kind,
                    ControlAnalysisResult::DcMatch(Box::new(result)),
                    count,
                )
            }
            AnalysisCommand::Tf {
                output_node,
                reference_node,
                output_is_current,
                input_source,
            } => {
                // The fixed scalar result can be admitted before solving, with
                // cumulative diagnostics against the caller's full allowance.
                engine
                    .ensure_result_values(self.retained_values.saturating_add(3))
                    .map_err(|error| simulation_error(line, error))?;
                let result = bounded
                    .run_transfer_function_with_abort(
                        &netlist,
                        output_node,
                        reference_node.as_deref(),
                        *output_is_current,
                        input_source,
                        abort,
                    )
                    .map_err(|error| simulation_error(line, error))?;
                (
                    kind,
                    ControlAnalysisResult::TransferFunction(Box::new(result)),
                    3,
                )
            }
            _ => {
                return Err(command_error(
                    line,
                    "this analysis has no control-host execution handler",
                )
                .into());
            }
        };
        if abort.is_aborted() {
            return Err(simulation_error(line, SimulationError::Aborted).into());
        }
        let retained_values = self.retained_values.saturating_add(count);
        engine
            .ensure_result_values(retained_values)
            .map_err(|error| simulation_error(line, error))?;
        // A destroyed plot's name may return; its `settype` units do not.
        self.vector_units
            .retain(|id, _| !id.dataset.eq_ignore_ascii_case(&name));
        self.datasets.push(ControlNamedDataset {
            name: name.clone(),
            analysis_id,
            device_op_report,
            command: analysis,
            result,
        });
        self.current = Some(self.datasets.len() - 1);
        self.vectors.leave_plot();
        self.ordinals.insert(kind, ordinal);
        self.retained_values = retained_values;
        self.integrated_noise_plot(line)?;
        Ok(name)
    }

    fn alter(
        &mut self,
        _engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        let line = command.line;
        let text = command.arguments.trim();
        let split = text
            .find(|character: char| character.is_whitespace() || character == '=')
            .unwrap_or(text.len());
        let target = text.get(..split).unwrap_or_default();
        let value = text.get(split..).unwrap_or_default().trim();
        let value = value.strip_prefix('=').unwrap_or(value).trim();
        if target.is_empty() || value.is_empty() {
            return Err(command_error(line, "alter requires a target and value").into());
        }
        let (name, parameter) = if let Some(target) = target.strip_prefix('@') {
            let (name, parameter) = target
                .split_once('[')
                .ok_or_else(|| command_error(line, "@alter target requires [parameter]"))?;
            let parameter = parameter
                .strip_suffix(']')
                .ok_or_else(|| command_error(line, "unterminated alter parameter"))?;
            (name, Some(parameter))
        } else {
            (target, None)
        };
        let element = self
            .netlist
            .elements
            .iter_mut()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| command_error(line, format!("alter target '{name}' was not found")))?;
        let mut candidate = element.kind.clone();
        let mut changes = Vec::new();
        if parameter.is_some_and(|parameter| parameter.eq_ignore_ascii_case("sin")) {
            let values = value
                .strip_prefix('[')
                .and_then(|body| body.strip_suffix(']'))
                .ok_or_else(|| {
                    command_error(line, "SIN alteration requires a bracketed value vector")
                })?;
            let values = values
                .split_whitespace()
                .map(|value| scalar(value, variables, line, abort))
                .collect::<Result<Vec<_>, _>>()?;
            if !(3..=6).contains(&values.len()) {
                return Err(command_error(line, "SIN alteration requires offset, amplitude, frequency and up to three optional values").into());
            }
            for (index, parameter) in ["VO", "VA", "FREQ", "TD", "THETA", "PHASE"]
                .iter()
                .enumerate()
            {
                let value = values.get(index).copied().unwrap_or(0.0);
                Engine::apply_device_step_value(&mut candidate, Some(parameter), value)
                    .map_err(|error| simulation_error(line, error))?;
                let canonical = Engine::canonical_device_parameter(&candidate, Some(parameter));
                changes.push((canonical, value));
            }
        } else {
            let value = scalar(value, variables, line, abort)?;
            let canonical = Engine::canonical_device_parameter(&candidate, parameter);
            Engine::apply_device_step_value(&mut candidate, Some(&canonical), value)
                .map_err(|error| simulation_error(line, error))?;
            changes.push((canonical, value));
        }
        // Publish only after every field has validated and applied to a private
        // candidate. The overlay keeps changes through later source replay.
        element.kind = candidate;
        let element_name = element.name.to_ascii_uppercase();
        for (parameter, value) in changes {
            self.netlist
                .retain_device_parameter((element_name.clone(), parameter), value);
        }
        Ok(())
    }

    /// `altermod model parameter = value`, `altermod @model[parameter] = value`
    /// and `altermod instance parameter = value` (frontend/device.c:1194
    /// com_altermod, through com_alter_common with do_model). A name is looked
    /// up as an instance first and as a model after (spiceif.c:1240 finddev);
    /// an instance names the model it uses (spiceif.c:967), so every instance
    /// of that model sees the new value. Each analysis builds the circuit
    /// again, which re-runs the model's setup and temperature update, as
    /// ngspice's CKTtemp after the change does (spiceif.c:979).
    fn alter_model(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        let line = command.line;
        let text = command.arguments.trim();
        if text.split_whitespace().any(|word| {
            word.get(..4)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("file"))
        }) {
            return Err(command_error(line, "altermod from a model file is not supported").into());
        }
        let (target, value) = match text.split_once('=') {
            Some((target, value)) => (target.trim(), value.trim()),
            // The pre-3f4 form `altermod model parameter value`
            // (device.c:1290).
            None => {
                let split = text.rfind(char::is_whitespace).unwrap_or(0);
                (text[..split].trim(), text[split..].trim())
            }
        };
        let (name, parameter) = if let Some(target) = target
            .strip_prefix('@')
            .or_else(|| target.strip_prefix('#'))
        {
            let (name, parameter) = target
                .split_once('[')
                .ok_or_else(|| command_error(line, "@altermod target requires [parameter]"))?;
            let parameter = parameter
                .strip_suffix(']')
                .ok_or_else(|| command_error(line, "unterminated altermod parameter"))?;
            (name.trim(), parameter.trim())
        } else {
            let mut words = target.split_whitespace();
            let name = words.next().unwrap_or_default();
            let parameter = words.next().unwrap_or_default();
            if words.next().is_some() {
                return Err(command_error(line, "altermod takes one model parameter").into());
            }
            (name, parameter)
        };
        if name.is_empty() || parameter.is_empty() || value.is_empty() {
            return Err(
                command_error(line, "altermod requires a model, a parameter and a value").into(),
            );
        }
        let model_name = match self
            .netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
        {
            Some(element) => super::builder::element_model_name(&element.kind)
                .ok_or_else(|| {
                    command_error(line, format!("altermod instance '{name}' has no model"))
                })?
                .to_owned(),
            None => name.to_owned(),
        };
        let value = scalar(value, variables, line, abort)?;
        let model = self
            .netlist
            .models
            .iter_mut()
            .find(|model| model.name.eq_ignore_ascii_case(&model_name))
            .ok_or_else(|| {
                command_error(
                    line,
                    format!("altermod model or instance '{name}' was not found"),
                )
            })?;
        Engine::apply_model_step_value(model, parameter, value);
        // The overlay keeps the change through later source replay.
        let key = (
            model.name.to_ascii_uppercase(),
            parameter.to_ascii_uppercase(),
        );
        self.netlist.retain_model_parameter(key, value);
        Ok(())
    }

    /// `alterparam name = value` (frontend/inp.c:1742 com_alterparam) changes
    /// a `.param` of the deck that `reset` loads, not the circuit: the change
    /// takes effect at the next `reset`, with every parameter and device
    /// value that depends on it, and stays for each `reset` after.
    fn alter_parameter(
        &mut self,
        engine: &Engine,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        let line = command.line;
        let (target, value) = command
            .arguments
            .split_once('=')
            .ok_or_else(|| command_error(line, "alterparam requires name = value"))?;
        let name = match target.split_whitespace().collect::<Vec<_>>().as_slice() {
            [name] => name.to_ascii_uppercase(),
            [_, _] => {
                return Err(command_error(
                    line,
                    "alterparam of a subcircuit parameter is not supported",
                )
                .into());
            }
            _ => return Err(command_error(line, "alterparam requires name = value").into()),
        };
        let value = scalar(value.trim(), variables, line, abort)?;
        if !self.authored.params.has_any_parameter_binding(&name) {
            return Err(command_error(
                line,
                format!(
                    "alterparam: parameter '{}' not found",
                    name.to_ascii_lowercase()
                ),
            )
            .into());
        }
        let (authored, _) = Engine::replay_parameter_overrides(
            &self.authored,
            &[(name, value)],
            None,
            engine.config().resource_limits,
            abort,
        )
        .map_err(|error| simulation_error(line, error))?;
        self.authored = authored;
        Ok(())
    }
}

impl ControlCircuit {
    /// Scalar evaluation as it was before `let` kept vectors: parameters,
    /// completed analyses' scalars and root functions.
    fn scalar_value(
        &self,
        expression: &Expr,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ControlError> {
        evaluate_complex_with_functions_and_abort(
            expression,
            variables,
            &mut |name| presentation::resolve_scalar(self, name),
            &mut |name, args| presentation::resolve_root_function(self, name, args),
            abort,
        )
        .map_err(|error| ControlError::evaluation(line, error))
    }

    /// An expression that names a `let` vector is a vector expression. Any
    /// other is evaluated as a scalar first, exactly as before; only when
    /// that fails to evaluate is it retried as a vector expression, and if
    /// the retry fails too the scalar diagnostic stands.
    fn vector_or_scalar(
        &self,
        expression: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Evaluated, ControlError> {
        let parsed = parse_control_expression_with_abort(expression, abort)
            .map_err(|error| ControlError::evaluation(line, error.into()))?;
        let scalar = if self.names_user_vector(&parsed) {
            None
        } else {
            match self.scalar_value(&parsed, variables, line, abort) {
                Ok(value) => return Ok(Evaluated::Scalar(value)),
                Err(error) if error.kind == ControlErrorKind::Expression => Some(error),
                Err(error) => return Err(error),
            }
        };
        match self.evaluate_vector(&parsed, variables, line, abort) {
            Ok(value) => Ok(Evaluated::Vector { value, scalar }),
            Err(error)
                if matches!(
                    error.kind,
                    ControlErrorKind::Expression | ControlErrorKind::Host
                ) =>
            {
                Err(scalar.unwrap_or(error))
            }
            Err(error) => Err(error),
        }
    }
}

enum Evaluated {
    Scalar(ComplexValue),
    /// A vector value, with the scalar evaluation's diagnostic when the
    /// expression names no `let` vector and was tried as a scalar first.
    Vector {
        value: VectorValue,
        scalar: Option<ControlError>,
    },
}

impl ControlScalarEvaluator for ControlCircuit {
    fn evaluate_scalar(
        &mut self,
        expression: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ControlError> {
        match self.vector_or_scalar(expression, variables, line, abort)? {
            Evaluated::Scalar(value) => Ok(value),
            Evaluated::Vector { value, scalar } => match value.samples.as_slice() {
                [value] => Ok(*value),
                samples => Err(scalar.unwrap_or_else(|| {
                    ControlError::new(
                        line,
                        ControlErrorKind::Expression,
                        format!(
                            "'{expression}' is a vector of {} values where a scalar is required",
                            samples.len()
                        ),
                    )
                })),
            },
        }
    }

    /// ngspice `com_let` binds a vector; a value that evaluates as a scalar
    /// without reading any vector stays a session parameter.
    fn evaluate_let(
        &mut self,
        name: &str,
        expression: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<ComplexValue>, ControlError> {
        match self.vector_or_scalar(expression, variables, line, abort)? {
            Evaluated::Scalar(value) => {
                self.bind_scalar(name, value, line)?;
                Ok(Some(value))
            }
            Evaluated::Vector { value, .. } => {
                self.bind_vector(name, value, line)?;
                Ok(None)
            }
        }
    }

    fn evaluate_indexed_let(
        &mut self,
        name: &str,
        index: &str,
        expression: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<ComplexValue>, ControlError> {
        self.assign_elements(name, index, expression, variables, line, abort)
    }

    fn vector_words(
        &mut self,
        name: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<String>>, ControlError> {
        self.expand_vector(name, variables, line, abort)
    }

    /// ngspice defines `curplot` from its plots (frontend/options.c:72),
    /// `batchmode` under `-b` (main.c:1032), as which this host runs every
    /// script, and `oscompiled` for the system it was built for
    /// (frontend/init.c:40).
    fn host_variable(&mut self, name: &str) -> Option<Vec<String>> {
        match name.to_ascii_lowercase().as_str() {
            "curplot" => Some(vec![self.plot_name()]),
            "batchmode" => Some(Vec::new()),
            "oscompiled" => Some(vec![os_compiled().to_string()]),
            "inputdir" => self
                .input_directory
                .clone()
                .map(|directory| vec![directory]),
            _ => None,
        }
    }
}

fn scalar(
    expression: &str,
    variables: &ParamContext,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<Value, ControlError> {
    let value = eval_expression_complex_with_abort(expression, variables, abort)
        .map_err(|error| ControlError::evaluation(line, error))?;
    if !value.re.is_finite() || !value.im.is_finite() {
        Err(ControlError::new(
            line,
            ControlErrorKind::Expression,
            "command value is nonfinite",
        ))
    } else if !is_real(value) {
        Err(ControlError::new(
            line,
            ControlErrorKind::Expression,
            "command requires a real value; use real(), imag() or mag() explicitly",
        ))
    } else {
        Ok(value.re)
    }
}

/// ngspice's code for the system it was built for (frontend/init.c:40):
/// 8 for a Visual Studio build on Windows, 1 for MinGW, 3 FreeBSD,
/// 4 OpenBSD, 5 Solaris, 6 Linux, 7 macOS, and 0 for any other.
fn os_compiled() -> u8 {
    if cfg!(all(windows, target_env = "msvc")) {
        8
    } else if cfg!(windows) {
        1
    } else if cfg!(target_os = "freebsd") {
        3
    } else if cfg!(target_os = "openbsd") {
        4
    } else if cfg!(any(target_os = "solaris", target_os = "illumos")) {
        5
    } else if cfg!(target_os = "linux") {
        6
    } else if cfg!(target_os = "macos") {
        7
    } else {
        0
    }
}

fn command_error(line: usize, message: impl Into<String>) -> ControlError {
    ControlError::new(line, ControlErrorKind::Host, message)
}

fn simulation_error(line: usize, source: SimulationError) -> ControlExecutionError {
    ControlExecutionError::Simulation { line, source }
}

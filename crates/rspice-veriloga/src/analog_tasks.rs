//! Ordered analog task calls shared by frontend and executable lowerings.

pub use rspice_veriloga_runtime::AnalogTaskKind;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Hidden scalar results read by simulator-control task consumers.
pub(crate) const SIMULATOR_CONTROL_TASK_VARIABLES: [&str; 2] = ["$bound_step", "$discontinuity"];

/// Where one task call was written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalogTaskOrigin {
    pub site: u32,
    pub file: SmolStr,
    /// One-based.
    pub line: u32,
}

/// System tasks and functions the standards define: Verilog-AMS (LRM 2.4 and
/// 2023, chapter 9) and the IEEE 1364-2005 ones Verilog-AMS inherits.
///
/// An analog position that does not accept one of these refuses it as
/// unsupported; any other `$name` is reported as unknown, which is what a
/// misspelling looks like.
const STANDARD_SYSTEM_NAMES: &[&str] = &[
    // Display and file I/O
    "$display",
    "$strobe",
    "$write",
    "$monitor",
    "$monitoron",
    "$monitoroff",
    "$debug",
    "$fdisplay",
    "$fstrobe",
    "$fwrite",
    "$fmonitor",
    "$fdebug",
    "$fopen",
    "$fclose",
    "$fflush",
    "$fgetc",
    "$fgets",
    "$fscanf",
    "$sscanf",
    "$swrite",
    "$sformat",
    "$feof",
    "$ferror",
    "$fread",
    "$fseek",
    "$ftell",
    "$rewind",
    "$ungetc",
    "$readmemb",
    "$readmemh",
    // Simulation control and announcements
    "$finish",
    "$stop",
    "$fatal",
    "$error",
    "$warning",
    "$info",
    // Time
    "$abstime",
    "$realtime",
    "$time",
    "$stime",
    "$printtimescale",
    "$timeformat",
    // Random numbers and distributions
    "$random",
    "$arandom",
    "$urandom",
    "$urandom_range",
    "$rdist_uniform",
    "$rdist_normal",
    "$rdist_exponential",
    "$rdist_poisson",
    "$rdist_chi_square",
    "$rdist_t",
    "$rdist_erlang",
    "$dist_uniform",
    "$dist_normal",
    "$dist_exponential",
    "$dist_poisson",
    "$dist_chi_square",
    "$dist_t",
    "$dist_erlang",
    // Simulator environment and hierarchy queries
    "$temperature",
    "$vt",
    "$simparam",
    "$simparam$str",
    "$simprobe",
    "$mfactor",
    "$xposition",
    "$yposition",
    "$angle",
    "$hflip",
    "$vflip",
    "$param_given",
    "$port_connected",
    "$analog_node_alias",
    "$analog_port_alias",
    "$test$plusargs",
    "$value$plusargs",
    // Analog kernel control and models
    "$limit",
    "$discontinuity",
    "$bound_step",
    "$table_model",
    // Conversions
    "$bitstoreal",
    "$realtobits",
    "$itor",
    "$rtoi",
    "$signed",
    "$unsigned",
    "$clog2",
    // IEEE 1364-2005 section 17.11 math functions
    "$ln",
    "$log10",
    "$exp",
    "$sqrt",
    "$pow",
    "$floor",
    "$ceil",
    "$sin",
    "$cos",
    "$tan",
    "$asin",
    "$acos",
    "$atan",
    "$atan2",
    "$hypot",
    "$sinh",
    "$cosh",
    "$tanh",
    "$asinh",
    "$acosh",
    "$atanh",
];

/// Whether `name` is a constant system function of IEEE 1364-2005 section
/// 17.11: `$clog2` (17.11.1) or a math function (17.11.2), legal in the
/// constant expressions of digital code.
pub(crate) fn is_constant_system_function(name: &str) -> bool {
    const CONSTANT_SYSTEM_FUNCTIONS: &[&str] = &[
        "$clog2", "$ln", "$log10", "$exp", "$sqrt", "$pow", "$floor", "$ceil", "$sin", "$cos",
        "$tan", "$asin", "$acos", "$atan", "$atan2", "$hypot", "$sinh", "$cosh", "$tanh", "$asinh",
        "$acosh", "$atanh",
    ];
    CONSTANT_SYSTEM_FUNCTIONS.contains(&name)
}

/// Why an analog position refuses the system task or function `name`
/// (`role` is "task" or "function"): a standard one is unsupported here, any
/// other is unknown.
pub(crate) fn system_name_refusal(name: &str, role: &str) -> String {
    if STANDARD_SYSTEM_NAMES.contains(&name) {
        format!("system {role} '{name}' is not supported in analog code")
    } else {
        format!("unknown system {role} '{name}'")
    }
}

/// Whether `name` is a system task or function the standards define.
pub(crate) fn is_standard_system_name(name: &str) -> bool {
    STANDARD_SYSTEM_NAMES.contains(&name)
}

/// An argument's language type must survive numerical backend lowering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalogTaskOperand<E> {
    Real(E),
    Integer(E),
    String(SmolStr),
}

/// A call at one source site. The enclosing guard belongs to the flattened
/// representation; structured regions carry the guard in their control flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalogTaskCall<E, S> {
    pub kind: AnalogTaskKind,
    pub site: u32,
    pub guard: Option<E>,
    pub arguments: Vec<AnalogTaskOperand<E>>,
    pub span: S,
    pub initialization: bool,
}

impl<E, S> AnalogTaskCall<E, S> {
    pub(crate) fn finish_operand(&self) -> Option<&E> {
        match (self.kind, self.arguments.as_slice()) {
            (AnalogTaskKind::Finish, [AnalogTaskOperand::Integer(value)]) => Some(value),
            _ => None,
        }
    }

    /// Every backend executes a `$finish` with its level and the output tasks.
    pub(crate) fn is_executable(&self) -> bool {
        self.finish_operand().is_some() || self.kind.is_output()
    }

    /// An output task's numeric operands in source order; the string literals
    /// stay in the task table for the host's formatter.
    pub(crate) fn output_operands(&self) -> Option<Vec<&E>> {
        self.kind.is_output().then(|| {
            self.arguments
                .iter()
                .filter_map(|argument| match argument {
                    AnalogTaskOperand::Real(value) | AnalogTaskOperand::Integer(value) => {
                        Some(value)
                    }
                    AnalogTaskOperand::String(_) => None,
                })
                .collect()
        })
    }

    /// The one program a native plan evaluates for this call: the `$finish`
    /// level, or the program that evaluates an output task's operands and
    /// journals them itself (a plan merges them into one, see
    /// `NativeProgram::analog_output`).
    #[cfg(any(feature = "native", feature = "wasm-jit"))]
    pub(crate) fn plan_operand(&self) -> Option<&E> {
        self.finish_operand().or_else(
            || match (self.kind.is_output(), self.arguments.as_slice()) {
                (true, [AnalogTaskOperand::Real(value)]) => Some(value),
                _ => None,
            },
        )
    }

    pub fn map<T, U>(&self, span: U, mut lower: impl FnMut(&E) -> T) -> AnalogTaskCall<T, U> {
        self.try_map(span, |expression| {
            Ok::<_, std::convert::Infallible>(lower(expression))
        })
        .expect("infallible expression lowering")
    }

    /// Guard first, followed by numeric arguments in source order.
    pub fn expressions(&self) -> impl Iterator<Item = &E> {
        self.guard
            .iter()
            .chain(self.arguments.iter().filter_map(|argument| match argument {
                AnalogTaskOperand::Real(value) | AnalogTaskOperand::Integer(value) => Some(value),
                AnalogTaskOperand::String(_) => None,
            }))
    }

    pub fn expressions_mut(&mut self) -> impl Iterator<Item = &mut E> {
        self.guard
            .iter_mut()
            .chain(
                self.arguments
                    .iter_mut()
                    .filter_map(|argument| match argument {
                        AnalogTaskOperand::Real(value) | AnalogTaskOperand::Integer(value) => {
                            Some(value)
                        }
                        AnalogTaskOperand::String(_) => None,
                    }),
            )
    }

    /// Change the expression representation without losing argument types or
    /// call identity. Every lowering uses the same traversal order.
    pub fn try_map<T, U, Error>(
        &self,
        span: U,
        mut lower: impl FnMut(&E) -> Result<T, Error>,
    ) -> Result<AnalogTaskCall<T, U>, Error> {
        let guard = self.guard.as_ref().map(&mut lower).transpose()?;
        let arguments = self
            .arguments
            .iter()
            .map(|argument| {
                Ok(match argument {
                    AnalogTaskOperand::Real(value) => AnalogTaskOperand::Real(lower(value)?),
                    AnalogTaskOperand::Integer(value) => AnalogTaskOperand::Integer(lower(value)?),
                    AnalogTaskOperand::String(value) => AnalogTaskOperand::String(value.clone()),
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(AnalogTaskCall {
            kind: self.kind,
            site: self.site,
            guard,
            arguments,
            span,
            initialization: self.initialization,
        })
    }
}

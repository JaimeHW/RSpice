//! The Verilog-A language as the editor offers and explains it.
//!
//! Every name here comes from the compiler: its reserved words, the functions
//! and analog operators, `$` functions, `$` tasks and constants it reads
//! without a declaration, the natures, disciplines and access functions of
//! its built-in `disciplines.vams`, the macros of its built-in
//! `constants.vams` and the names of its built-in headers. What the editor
//! adds is how each callable is written and what it does, one line each; a
//! test fails when the compiler gains a built-in this table does not explain,
//! or loses one it does.

use std::sync::OnceLock;

use rspice_veriloga::language_catalog::BuiltinKind;

/// What a catalog name is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CatalogKind {
    Keyword,
    Function,
    AnalogOperator,
    SystemFunction,
    SystemTask,
    Constant,
    /// An access function of a nature: `V`, `I`.
    Access,
    Nature,
    Discipline,
    Macro,
    /// A preprocessor directive, written with its backtick.
    Directive,
    Header,
}

impl CatalogKind {
    /// The short word the completion list and hover show for the kind.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Keyword => "keyword",
            Self::Function => "function",
            Self::AnalogOperator => "operator",
            Self::SystemFunction => "function",
            Self::SystemTask => "task",
            Self::Constant => "constant",
            Self::Access => "access",
            Self::Nature => "nature",
            Self::Discipline => "discipline",
            Self::Macro => "macro",
            Self::Directive => "directive",
            Self::Header => "header",
        }
    }
}

/// One argument of a callable, as the signature writes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CatalogParameter {
    pub name: String,
    pub optional: bool,
    /// Any number of further arguments of this kind may follow.
    pub repeated: bool,
}

impl CatalogParameter {
    /// The text before the argument's name, the name and the text after it,
    /// at position `index` of a signature: `[, abstol]` is `" [, "`,
    /// `"abstol"`, `"]"`.
    fn pieces(&self, index: usize) -> (&'static str, String, &'static str) {
        let name = if self.repeated {
            format!("{}…", self.name)
        } else {
            self.name.clone()
        };
        match (index, self.optional) {
            (0, false) => ("", name, ""),
            (0, true) => ("[", name, "]"),
            (_, false) => (", ", name, ""),
            (_, true) => (" [, ", name, "]"),
        }
    }
}

/// One name of the language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CatalogEntry {
    pub name: String,
    pub kind: CatalogKind,
    /// The arguments of a callable; `None` for a name that takes none and
    /// is written without parentheses (`$temperature`, `M_PI`, a keyword).
    pub parameters: Option<Vec<CatalogParameter>>,
    /// One line on what it is.
    pub description: String,
}

impl CatalogEntry {
    /// How the name is written: `ddt(expr [, abstol])`.
    pub(crate) fn signature(&self) -> String {
        match &self.parameters {
            None => self.name.clone(),
            Some(parameters) => {
                let mut text = format!("{}(", self.name);
                for (index, parameter) in parameters.iter().enumerate() {
                    let (before, name, after) = parameter.pieces(index);
                    text.push_str(before);
                    text.push_str(&name);
                    text.push_str(after);
                }
                text.push(')');
                text
            }
        }
    }

    /// The signature split around argument `active`, for signature help to
    /// mark it: (before, active, after). `None` when the name takes no
    /// arguments or `active` is past a fixed list.
    pub(crate) fn signature_parts(&self, active: usize) -> Option<(String, String, String)> {
        let parameters = self.parameters.as_ref()?;
        let marked = if active < parameters.len() {
            active
        } else if parameters.last().is_some_and(|last| last.repeated) {
            parameters.len() - 1
        } else {
            return None;
        };
        let mut before = format!("{}(", self.name);
        let mut current = String::new();
        let mut after = String::new();
        for (index, parameter) in parameters.iter().enumerate() {
            let (opening, name, closing) = parameter.pieces(index);
            match index.cmp(&marked) {
                std::cmp::Ordering::Less => {
                    before.push_str(opening);
                    before.push_str(&name);
                    before.push_str(closing);
                }
                std::cmp::Ordering::Equal => {
                    before.push_str(opening);
                    current = name;
                    after.push_str(closing);
                }
                std::cmp::Ordering::Greater => {
                    after.push_str(opening);
                    after.push_str(&name);
                    after.push_str(closing);
                }
            }
        }
        after.push(')');
        Some((before, current, after))
    }
}

/// The whole catalog, read once.
pub(crate) struct VerilogACatalog {
    entries: Vec<CatalogEntry>,
}

impl VerilogACatalog {
    pub(crate) fn get() -> &'static Self {
        static CATALOG: OnceLock<VerilogACatalog> = OnceLock::new();
        CATALOG.get_or_init(Self::read)
    }

    pub(crate) fn entries(&self) -> &[CatalogEntry] {
        &self.entries
    }

    /// The entry named `name`. A name that is both a keyword and a function
    /// (`cross`) answers as the function.
    pub(crate) fn entry(&self, name: &str) -> Option<&CatalogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.name == name)
            .min_by_key(|entry| entry.kind == CatalogKind::Keyword)
    }

    /// The entry for a word as the source writes it: `` `M_PI `` is the
    /// macro and `` `define `` the directive, `M_PI` the bare constant.
    pub(crate) fn written(&self, word: &str) -> Option<&CatalogEntry> {
        match word.strip_prefix('`') {
            Some(name) => self.entries.iter().find(|entry| {
                (entry.kind == CatalogKind::Directive && entry.name == word)
                    || (entry.kind == CatalogKind::Macro && entry.name == name)
            }),
            None => self.entry(word),
        }
    }

    /// Whether a declaration may not take `name`: a reserved word, or a name
    /// the compiler already reads as a function, task or constant.
    pub(crate) fn reserves(&self, name: &str) -> bool {
        rspice_veriloga::language_catalog::is_reserved_word(name)
            || rspice_veriloga::language_catalog::builtin_kind(name).is_some()
    }

    fn read() -> Self {
        let mut entries = Vec::new();
        for (name, kind) in rspice_veriloga::language_catalog::builtins() {
            let Some((arguments, description)) = documentation(name) else {
                continue;
            };
            let kind = match kind {
                BuiltinKind::Function if ANALOG_OPERATORS.contains(&name) => {
                    CatalogKind::AnalogOperator
                }
                BuiltinKind::Function => CatalogKind::Function,
                BuiltinKind::SystemFunction => CatalogKind::SystemFunction,
                BuiltinKind::SystemTask => CatalogKind::SystemTask,
                BuiltinKind::Constant => CatalogKind::Constant,
            };
            entries.push(CatalogEntry {
                name: name.to_owned(),
                kind,
                parameters: arguments.map(parse_arguments),
                description: description.to_owned(),
            });
        }
        for word in rspice_veriloga::language_catalog::reserved_words() {
            entries.push(CatalogEntry {
                name: word.to_owned(),
                kind: CatalogKind::Keyword,
                parameters: None,
                description: "Reserved word.".to_owned(),
            });
        }
        for directive in rspice_veriloga::language_catalog::directives() {
            entries.push(CatalogEntry {
                name: format!("`{directive}"),
                kind: CatalogKind::Directive,
                parameters: None,
                description: directive_documentation(directive)
                    .unwrap_or_default()
                    .to_owned(),
            });
        }
        for (header, source) in rspice_veriloga::language_catalog::builtin_headers() {
            entries.push(CatalogEntry {
                name: header.to_owned(),
                kind: CatalogKind::Header,
                parameters: None,
                description: "Standard header built into RSpice.".to_owned(),
            });
            entries.extend(header_declarations(source));
        }
        Self { entries }
    }
}

/// The natures, disciplines, access functions and macros a built-in header
/// declares, read through the compiler's parser.
fn header_declarations(source: &str) -> Vec<CatalogEntry> {
    let mut entries = Vec::new();
    for line in source.lines() {
        let Some(rest) = line.trim_start().strip_prefix("`define") else {
            continue;
        };
        let mut words = rest.split_whitespace();
        let (Some(name), value) = (words.next(), words.collect::<Vec<_>>().join(" ")) else {
            continue;
        };
        if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && !value.is_empty() {
            entries.push(CatalogEntry {
                name: name.to_owned(),
                kind: CatalogKind::Macro,
                parameters: None,
                description: format!("Constant from constants.vams: {value}."),
            });
        }
    }
    let readable = super::source_navigation::without_preprocessor_syntax(source);
    let source_id = rspice_veriloga::SourceMap::new().add_source("header", String::new());
    let Ok(tokens) = rspice_veriloga::Lexer::new(&readable, source_id).collect_tokens() else {
        return entries;
    };
    let Ok(file) = rspice_veriloga::Parser::new(&tokens).parse() else {
        return entries;
    };
    for item in &file.items {
        match item {
            rspice_veriloga::ast::Item::Nature(nature) => {
                let units = nature
                    .units
                    .as_deref()
                    .map(|units| format!(" in {units}"))
                    .unwrap_or_default();
                entries.push(CatalogEntry {
                    name: nature.name.to_string(),
                    kind: CatalogKind::Nature,
                    parameters: None,
                    description: format!("Nature{units} from disciplines.vams."),
                });
                if let Some(access) = &nature.access
                    && !entries.iter().any(|entry| entry.name == access.as_str())
                {
                    entries.push(CatalogEntry {
                        name: access.to_string(),
                        kind: CatalogKind::Access,
                        parameters: Some(parse_arguments("node, [node]")),
                        description: format!(
                            "The {} of a node, or between two nodes or across a branch.",
                            nature.name.to_lowercase()
                        ),
                    });
                }
            }
            rspice_veriloga::ast::Item::Discipline(discipline) => {
                let natures = [
                    discipline
                        .potential
                        .as_deref()
                        .map(|nature| format!("potential {nature}")),
                    discipline
                        .flow
                        .as_deref()
                        .map(|nature| format!("flow {nature}")),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
                entries.push(CatalogEntry {
                    name: discipline.name.to_string(),
                    kind: CatalogKind::Discipline,
                    parameters: None,
                    description: if natures.is_empty() {
                        "Discipline from disciplines.vams.".to_owned()
                    } else {
                        format!("Discipline from disciplines.vams: {}.", natures.join(", "))
                    },
                });
            }
            _ => {}
        }
    }
    entries
}

/// `"expr, [abstol], arg…"` as parameters.
fn parse_arguments(text: &str) -> Vec<CatalogParameter> {
    text.split(", ")
        .filter(|word| !word.is_empty())
        .map(|word| {
            let optional = word.starts_with('[');
            let word = word.trim_start_matches('[').trim_end_matches(']');
            CatalogParameter {
                name: word.trim_end_matches('…').to_owned(),
                optional,
                repeated: word.ends_with('…'),
            }
        })
        .collect()
}

/// The functions the compiler lowers as analog operators rather than plain
/// arithmetic: each keeps state across time or answers the analysis.
const ANALOG_OPERATORS: &[&str] = &[
    "ddt",
    "idt",
    "idtmod",
    "ddx",
    "absdelay",
    "transition",
    "slew",
    "cross",
    "above",
    "timer",
    "last_crossing",
    "analysis",
    "white_noise",
    "flicker_noise",
    "noise_table",
    "noise_table_log",
    "laplace_nd",
    "laplace_zp",
    "laplace_zd",
    "laplace_np",
    "zi_nd",
    "zi_zp",
    "zi_zd",
    "zi_np",
];

fn documentation(name: &str) -> Option<(Option<&'static str>, &'static str)> {
    DOCUMENTATION
        .iter()
        .find(|(documented, _, _)| *documented == name)
        .map(|(_, arguments, description)| (*arguments, *description))
}

fn directive_documentation(name: &str) -> Option<&'static str> {
    DIRECTIVE_DOCUMENTATION
        .iter()
        .find(|(documented, _)| *documented == name)
        .map(|(_, description)| *description)
}

/// What each preprocessor directive does.
const DIRECTIVE_DOCUMENTATION: &[(&str, &str)] = &[
    ("include", "Reads another source file in at this place."),
    ("define", "Defines a text macro."),
    ("undef", "Removes the definition of a text macro."),
    (
        "ifdef",
        "Keeps the text that follows when a macro is defined.",
    ),
    (
        "ifndef",
        "Keeps the text that follows when a macro is not defined.",
    ),
    ("else", "The alternative of an `ifdef or `ifndef."),
    (
        "elsif",
        "A further alternative of an `ifdef or `ifndef, on another macro.",
    ),
    ("endif", "Closes an `ifdef or `ifndef."),
    (
        "timescale",
        "Sets the time unit and precision of the modules that follow.",
    ),
    ("resetall", "Sets every directive back to its default."),
    (
        "default_nettype",
        "Sets the type of nets declared implicitly.",
    ),
    (
        "default_transition",
        "Sets the transition time digital events take by default.",
    ),
    (
        "default_discipline",
        "Sets the discipline of nets that declare none.",
    ),
    (
        "begin_keywords",
        "Reads the text that follows with the reserved words of a named standard.",
    ),
    ("end_keywords", "Closes a `begin_keywords."),
];

/// How each built-in is written and what it does (Verilog-AMS LRM 2.4,
/// chapters 4 and 9). `None` arguments: written without parentheses.
const DOCUMENTATION: &[(&str, Option<&str>, &str)] = &[
    ("abs", Some("x"), "Absolute value."),
    ("sqrt", Some("x"), "Square root."),
    ("exp", Some("x"), "Exponential, e to the power x."),
    ("ln", Some("x"), "Natural logarithm."),
    ("log", Some("x"), "Base-10 logarithm."),
    ("log10", Some("x"), "Base-10 logarithm."),
    ("sin", Some("x"), "Sine of x radians."),
    ("cos", Some("x"), "Cosine of x radians."),
    ("tan", Some("x"), "Tangent of x radians."),
    ("sinh", Some("x"), "Hyperbolic sine."),
    ("cosh", Some("x"), "Hyperbolic cosine."),
    ("tanh", Some("x"), "Hyperbolic tangent."),
    ("asin", Some("x"), "Arc sine, in radians."),
    ("acos", Some("x"), "Arc cosine, in radians."),
    ("atan", Some("x"), "Arc tangent, in radians."),
    ("asinh", Some("x"), "Inverse hyperbolic sine."),
    ("acosh", Some("x"), "Inverse hyperbolic cosine."),
    ("atanh", Some("x"), "Inverse hyperbolic tangent."),
    (
        "atan2",
        Some("y, x"),
        "Arc tangent of y/x in the quadrant of (x, y).",
    ),
    ("hypot", Some("x, y"), "Square root of x² + y²."),
    ("floor", Some("x"), "Largest integer not above x."),
    ("ceil", Some("x"), "Smallest integer not below x."),
    ("min", Some("x, y"), "The smaller of x and y."),
    ("max", Some("x, y"), "The larger of x and y."),
    ("pow", Some("x, y"), "x to the power y."),
    (
        "limexp",
        Some("x"),
        "Exponential whose growth the simulator limits between Newton iterations.",
    ),
    ("ddt", Some("expr, [abstol]"), "Time derivative of expr."),
    (
        "idt",
        Some("expr, [ic], [assert], [abstol]"),
        "Time integral of expr from ic, reset to ic while assert is nonzero.",
    ),
    (
        "idtmod",
        Some("expr, [ic], [modulus], [offset], [abstol]"),
        "Time integral of expr, wrapped into [offset, offset + modulus).",
    ),
    (
        "ddx",
        Some("expr, unknown"),
        "Partial derivative of expr with respect to a node potential or branch flow.",
    ),
    (
        "absdelay",
        Some("expr, td, [maxdelay]"),
        "expr delayed by td seconds.",
    ),
    (
        "transition",
        Some("expr, [td], [rise_time], [fall_time], [time_tol]"),
        "A piecewise-constant expr turned into ramps of the given delay and times.",
    ),
    (
        "slew",
        Some("expr, [max_pos_slew_rate], [max_neg_slew_rate]"),
        "expr with its rate of change bounded.",
    ),
    (
        "cross",
        Some("expr, [direction], [time_tol], [expr_tol]"),
        "Event at each zero crossing of expr in the given direction.",
    ),
    (
        "above",
        Some("expr, [time_tol], [expr_tol]"),
        "Event whenever expr rises through zero, and at the start of a run where it is above.",
    ),
    (
        "timer",
        Some("start_time, [period], [time_tol], [enable]"),
        "Event at start_time and every period after it.",
    ),
    (
        "last_crossing",
        Some("expr, [direction]"),
        "Time of the last zero crossing of expr in the given direction.",
    ),
    (
        "analysis",
        Some("name, [name…]"),
        "Whether the running analysis is one of the names: \"dc\", \"ac\", \"tran\", \"noise\", \"static\", \"ic\".",
    ),
    (
        "white_noise",
        Some("pwr, [name]"),
        "White noise source of power spectral density pwr.",
    ),
    (
        "flicker_noise",
        Some("pwr, exp, [name]"),
        "Noise source of power spectral density pwr / f^exp.",
    ),
    (
        "noise_table",
        Some("vector, [name]"),
        "Noise source whose density is interpolated from (frequency, density) pairs.",
    ),
    (
        "noise_table_log",
        Some("vector, [name]"),
        "Noise source whose density is interpolated log-log from (frequency, density) pairs.",
    ),
    (
        "laplace_nd",
        Some("expr, numerator, denominator, [abstol]"),
        "Laplace transfer function from numerator and denominator coefficients.",
    ),
    (
        "laplace_zp",
        Some("expr, zeros, poles, [abstol]"),
        "Laplace transfer function from its zeros and poles.",
    ),
    (
        "laplace_zd",
        Some("expr, zeros, denominator, [abstol]"),
        "Laplace transfer function from its zeros and denominator coefficients.",
    ),
    (
        "laplace_np",
        Some("expr, numerator, poles, [abstol]"),
        "Laplace transfer function from numerator coefficients and its poles.",
    ),
    (
        "zi_nd",
        Some("expr, numerator, denominator, T, [tau], [t0]"),
        "Z-domain filter from numerator and denominator coefficients, sampled every T.",
    ),
    (
        "zi_zp",
        Some("expr, zeros, poles, T, [tau], [t0]"),
        "Z-domain filter from its zeros and poles, sampled every T.",
    ),
    (
        "zi_zd",
        Some("expr, zeros, denominator, T, [tau], [t0]"),
        "Z-domain filter from its zeros and denominator coefficients, sampled every T.",
    ),
    (
        "zi_np",
        Some("expr, numerator, poles, T, [tau], [t0]"),
        "Z-domain filter from numerator coefficients and its poles, sampled every T.",
    ),
    (
        "$vt",
        Some("[temperature]"),
        "Thermal voltage kT/q at the circuit temperature, or at temperature in kelvin.",
    ),
    (
        "$thermal_vt",
        Some("[temperature]"),
        "Thermal voltage kT/q, as $vt.",
    ),
    ("$temperature", None, "Circuit temperature in kelvin."),
    ("$abstime", None, "Simulation time in seconds."),
    ("$mfactor", None, "Multiplicity of the instance."),
    (
        "$simparam",
        Some("name, [default]"),
        "Value of the simulator parameter name, or default when the simulator has none.",
    ),
    (
        "$param_given",
        Some("parameter"),
        "Whether the instance or its model sets the parameter.",
    ),
    (
        "$port_connected",
        Some("port"),
        "Whether the port is connected.",
    ),
    (
        "$limit",
        Some("access, [function], [arg…]"),
        "The access value, limited between Newton iterations by function (\"pnjlim\", \"fetlim\").",
    ),
    (
        "$table_model",
        Some("input…, data, [control]"),
        "Value interpolated from a table of samples.",
    ),
    (
        "$white_noise",
        Some("pwr, [name]"),
        "White noise source of power spectral density pwr, as white_noise.",
    ),
    (
        "$flicker_noise",
        Some("pwr, exp, [name]"),
        "Noise source of density pwr / f^exp, as flicker_noise.",
    ),
    (
        "$noise_table",
        Some("vector, [name]"),
        "Tabulated noise source, as noise_table.",
    ),
    (
        "$noise_table_log",
        Some("vector, [name]"),
        "Tabulated noise source interpolated log-log, as noise_table_log.",
    ),
    (
        "$strobe",
        Some("format, [arg…]"),
        "Prints formatted text once the time step is accepted.",
    ),
    (
        "$display",
        Some("format, [arg…]"),
        "Prints formatted text and a line break.",
    ),
    (
        "$write",
        Some("format, [arg…]"),
        "Prints formatted text without a line break.",
    ),
    (
        "$monitor",
        Some("format, [arg…]"),
        "Prints formatted text whenever one of its arguments changes.",
    ),
    ("$info", Some("format, [arg…]"), "Reports information."),
    ("$warning", Some("format, [arg…]"), "Reports a warning."),
    ("$error", Some("format, [arg…]"), "Reports an error."),
    (
        "$finish",
        Some("[level]"),
        "Ends the simulation once the current time step is done.",
    ),
    (
        "$bound_step",
        Some("max_step"),
        "Limits the next time step to max_step seconds.",
    ),
    (
        "$discontinuity",
        Some("[degree]"),
        "Announces a discontinuity in the given derivative of the equations.",
    ),
    (
        "$analog_node_alias",
        Some("node, path"),
        "Binds the local node to the node the path names in the design.",
    ),
    (
        "$analog_port_alias",
        Some("node, path"),
        "Binds the local node to the port the path names in the design.",
    ),
    ("M_PI", None, "π."),
    ("P_PI", None, "π."),
    ("M_E", None, "e, the base of natural logarithms."),
    ("P_E", None, "e, the base of natural logarithms."),
    ("M_LN2", None, "Natural logarithm of 2."),
    ("M_LN10", None, "Natural logarithm of 10."),
    ("M_LOG2E", None, "Base-2 logarithm of e."),
    ("M_LOG10E", None, "Base-10 logarithm of e."),
    ("M_SQRT2", None, "Square root of 2."),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// The catalog explains exactly the compiler's built-ins: one it gains
    /// fails here until it is written down, and so does one it loses.
    #[test]
    fn every_builtin_the_compiler_reads_is_explained_and_nothing_else() {
        let compiler = rspice_veriloga::language_catalog::builtins()
            .map(|(name, _)| name)
            .collect::<BTreeSet<_>>();
        let documented = DOCUMENTATION
            .iter()
            .map(|(name, _, _)| *name)
            .collect::<BTreeSet<_>>();
        assert_eq!(
            compiler.difference(&documented).collect::<Vec<_>>(),
            Vec::<&&str>::new(),
            "built-ins the catalog does not explain"
        );
        assert_eq!(
            documented.difference(&compiler).collect::<Vec<_>>(),
            Vec::<&&str>::new(),
            "explained names the compiler does not read"
        );
        for operator in ANALOG_OPERATORS {
            assert!(compiler.contains(operator), "{operator} is a built-in");
        }
        let catalog = VerilogACatalog::get();
        for name in compiler {
            let entry = catalog.entry(name).expect("every built-in is an entry");
            assert!(!entry.description.is_empty());
        }
    }

    /// The directive table explains exactly what the preprocessor acts on.
    #[test]
    fn every_directive_the_preprocessor_acts_on_is_explained_and_nothing_else() {
        let compiler = rspice_veriloga::language_catalog::directives().collect::<BTreeSet<_>>();
        let documented = DIRECTIVE_DOCUMENTATION
            .iter()
            .map(|(name, _)| *name)
            .collect::<BTreeSet<_>>();
        assert_eq!(compiler, documented);
        let catalog = VerilogACatalog::get();
        let define = catalog.written("`define").expect("`define");
        assert_eq!(define.kind, CatalogKind::Directive);
        assert_eq!(
            catalog.written("`M_TWO_PI").map(|entry| entry.kind),
            Some(CatalogKind::Macro)
        );
        assert_eq!(
            catalog.written("M_PI").map(|entry| entry.kind),
            Some(CatalogKind::Constant)
        );
    }

    #[test]
    fn the_headers_give_natures_disciplines_access_functions_and_macros() {
        let catalog = VerilogACatalog::get();
        let kind = |name: &str| catalog.entry(name).map(|entry| entry.kind);
        assert_eq!(kind("Voltage"), Some(CatalogKind::Nature));
        assert_eq!(kind("Current"), Some(CatalogKind::Nature));
        assert_eq!(kind("magnetic"), Some(CatalogKind::Discipline));
        assert_eq!(kind("V"), Some(CatalogKind::Access));
        assert_eq!(kind("I"), Some(CatalogKind::Access));
        assert_eq!(kind("M_TWO_PI"), Some(CatalogKind::Macro));
        assert_eq!(kind("P_K"), Some(CatalogKind::Macro));
        assert_eq!(kind("disciplines.vams"), Some(CatalogKind::Header));
        // `electrical` is both: the lexer reserves it, the header declares it.
        assert!(
            catalog
                .entries()
                .iter()
                .any(|entry| entry.name == "electrical" && entry.kind == CatalogKind::Discipline)
        );
        assert_eq!(kind("begin"), Some(CatalogKind::Keyword));
        assert_eq!(kind("cross"), Some(CatalogKind::AnalogOperator));
    }

    #[test]
    fn a_signature_marks_the_argument_being_written() {
        let catalog = VerilogACatalog::get();
        let ddt = catalog.entry("ddt").expect("ddt");
        assert_eq!(ddt.signature(), "ddt(expr [, abstol])");
        assert_eq!(
            ddt.signature_parts(1),
            Some((
                "ddt(expr [, ".to_owned(),
                "abstol".to_owned(),
                "])".to_owned()
            ))
        );
        assert_eq!(ddt.signature_parts(2), None);
        let strobe = catalog.entry("$strobe").expect("$strobe");
        assert_eq!(strobe.signature(), "$strobe(format [, arg…])");
        assert_eq!(
            strobe.signature_parts(4).map(|(_, active, _)| active),
            Some("arg…".to_owned())
        );
        let temperature = catalog.entry("$temperature").expect("$temperature");
        assert_eq!(temperature.signature(), "$temperature");
        assert!(catalog.reserves("ddt") && catalog.reserves("begin"));
        assert!(!catalog.reserves("gain"));
    }
}

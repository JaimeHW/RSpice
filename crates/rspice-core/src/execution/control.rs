//! Resumable, target-neutral execution of ngspice control flow.
//!
//! A session evaluates assignments and loops, yielding each external command
//! in source order. The caller executes that command before requesting another.
//! Analysis, plotting and file commands are never silently discarded here.

use crate::abort_signal::AbortSignal;
pub use crate::control_protocol::{
    ControlCommand, ControlError, ControlErrorKind, ControlScalarEvaluator,
};
use crate::netlist::expr::{ParamContext, eval_expression_complex_with_abort, is_real};
use crate::{ComplexValue, Value};
use std::collections::{BTreeMap, BTreeSet};

use crate::engine::control::shell::{self, ShellValue};

#[derive(Debug, Clone, Copy)]
pub struct ControlLimits {
    pub max_source_bytes: usize,
    pub max_source_lines: usize,
    pub max_nesting: usize,
    pub max_loop_values: usize,
    /// Counts executed instructions, including loop tests and empty bodies.
    pub max_steps: usize,
}

impl Default for ControlLimits {
    fn default() -> Self {
        let source = crate::resource::ResourceLimits::default();
        Self {
            max_source_bytes: source.max_netlist_bytes,
            max_source_lines: source.max_netlist_lines,
            max_nesting: 128,
            max_loop_values: source.max_batch_runs,
            max_steps: 1_000_000,
        }
    }
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct ParameterScalarEvaluator;

#[cfg(test)]
impl ControlScalarEvaluator for ParameterScalarEvaluator {
    fn evaluate_scalar(
        &mut self,
        expression: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ControlError> {
        eval_expression_complex_with_abort(expression, variables, abort)
            .map_err(|error| ControlError::evaluation(line, error))
    }
}

#[derive(Debug, Clone)]
enum LoopKind {
    Foreach { variable: String, words: String },
    Repeat(String),
    While(String),
    Dowhile(String),
}

#[derive(Debug, Clone)]
enum Op {
    Loop {
        kind: LoopKind,
        end: usize,
    },
    If {
        condition: String,
        alternative: usize,
        end: usize,
    },
    Else {
        end: usize,
    },
    End {
        opening: usize,
    },
    Let {
        name: String,
        /// The text after the destination's first `[`, as com_let.c:73
        /// splits it; the host finds the indices when it runs.
        index: Option<String>,
        expression: String,
    },
    Break,
    Continue,
    /// ngspice `quit [status]`; the argument is substituted when it runs.
    Quit {
        argument: String,
    },
    /// ngspice `define name(arguments) body`, substituted when it runs.
    Define {
        text: String,
    },
    /// ngspice `strcmp variable first second`, substituted when it runs.
    Strcmp {
        arguments: String,
    },
    Command {
        name: String,
        arguments: String,
    },
}

#[derive(Debug, Clone)]
struct Instruction {
    line: usize,
    op: Op,
}

#[derive(Debug, Clone)]
pub struct ControlProgram {
    declarative_source: String,
    instructions: Vec<Instruction>,
    limits: ControlLimits,
}

impl ControlProgram {
    /// Extract control regions while preserving physical line numbers in the
    /// declarative source. Includes must be resolved by the caller's source
    /// provider first when they can contain additional control regions.
    pub fn parse_deck_with_abort(
        source: &str,
        limits: ControlLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ControlError> {
        check_limit(source.len(), limits.max_source_bytes, 0, "source bytes")?;
        let mut program = Self {
            declarative_source: String::with_capacity(source.len()),
            instructions: Vec::new(),
            limits,
        };
        let mut in_control = false;
        let mut blocks: Vec<(usize, Option<usize>)> = Vec::new();
        for (index, text) in source.lines().enumerate() {
            let line = index + 1;
            check_abort(abort, line)?;
            check_limit(line, limits.max_source_lines, line, "source lines")?;
            if index == 0 {
                program.declarative_source.push_str(text);
                program.declarative_source.push('\n');
                continue;
            }
            let body = control_body(text);
            let (head, rest) = split_head(body);
            if head.eq_ignore_ascii_case(".control") {
                require_empty(rest, line)?;
                if in_control {
                    return Err(syntax(line, "nested .control region"));
                }
                in_control = true;
                program.declarative_source.push('\n');
                continue;
            }
            if head.eq_ignore_ascii_case(".endc") {
                require_empty(rest, line)?;
                if !in_control {
                    return Err(syntax(line, ".endc without .control"));
                }
                if !blocks.is_empty() {
                    return Err(syntax(line, "control block is missing end"));
                }
                in_control = false;
                program.declarative_source.push('\n');
                continue;
            }
            if !in_control {
                program.declarative_source.push_str(text);
                program.declarative_source.push('\n');
                continue;
            }
            program.declarative_source.push('\n');
            if head.is_empty() {
                continue;
            }
            let pc = program.instructions.len();
            let op = match head.to_ascii_lowercase().as_str() {
                "foreach" => {
                    let (variable, words) = split_head(rest);
                    require_identifier(variable, line)?;
                    Op::Loop {
                        kind: LoopKind::Foreach {
                            variable: variable.to_owned(),
                            words: words.to_owned(),
                        },
                        end: 0,
                    }
                }
                "repeat" | "while" | "dowhile" => {
                    if rest.is_empty() {
                        return Err(syntax(line, "loop requires an expression"));
                    }
                    let expression = rest.to_owned();
                    let kind = match head.to_ascii_lowercase().as_str() {
                        "repeat" => LoopKind::Repeat(expression),
                        "while" => LoopKind::While(expression),
                        _ => LoopKind::Dowhile(expression),
                    };
                    Op::Loop { kind, end: 0 }
                }
                "if" => {
                    if rest.is_empty() {
                        return Err(syntax(line, "if requires an expression"));
                    }
                    Op::If {
                        condition: rest.to_owned(),
                        alternative: 0,
                        end: 0,
                    }
                }
                "else" => {
                    require_empty(rest, line)?;
                    let Some((opening, alternative)) = blocks.last_mut() else {
                        return Err(syntax(line, "else without if"));
                    };
                    if alternative.is_some()
                        || !matches!(
                            program
                                .instructions
                                .get(*opening)
                                .map(|instruction| &instruction.op),
                            Some(Op::If { .. })
                        )
                    {
                        return Err(syntax(line, "else requires an unmatched if"));
                    }
                    *alternative = Some(pc);
                    Op::Else { end: 0 }
                }
                "end" => {
                    require_empty(rest, line)?;
                    let (opening, alternative_pc) = blocks
                        .pop()
                        .ok_or_else(|| syntax(line, "end without a block"))?;
                    match program
                        .instructions
                        .get_mut(opening)
                        .map(|instruction| &mut instruction.op)
                    {
                        Some(Op::Loop { end, .. }) => *end = pc,
                        Some(Op::If {
                            alternative, end, ..
                        }) => {
                            *alternative = alternative_pc.unwrap_or(pc);
                            *end = pc;
                        }
                        _ => return Err(syntax(line, "invalid block opening")),
                    }
                    if let Some(alternative) = alternative_pc
                        && let Some(Instruction {
                            op: Op::Else { end },
                            ..
                        }) = program.instructions.get_mut(alternative)
                    {
                        *end = pc;
                    }
                    Op::End { opening }
                }
                "let" => {
                    let (target, expression) = rest
                        .split_once('=')
                        .ok_or_else(|| syntax(line, "let requires name = expression"))?;
                    let (name, index) = match target.split_once('[') {
                        Some((name, index)) => (name.trim(), Some(index.trim().to_owned())),
                        None => (target.trim(), None),
                    };
                    require_identifier(name, line)?;
                    if expression.trim().is_empty() {
                        return Err(syntax(line, "let requires an expression"));
                    }
                    Op::Let {
                        name: name.to_owned(),
                        index,
                        expression: expression.trim().to_owned(),
                    }
                }
                "break" | "continue" => {
                    require_empty(rest, line)?;
                    if !blocks.iter().any(|(opening, _)| {
                        matches!(
                            program
                                .instructions
                                .get(*opening)
                                .map(|instruction| &instruction.op),
                            Some(Op::Loop { .. })
                        )
                    }) {
                        return Err(syntax(line, "loop control outside a loop"));
                    }
                    if head.eq_ignore_ascii_case("break") {
                        Op::Break
                    } else {
                        Op::Continue
                    }
                }
                "quit" => Op::Quit {
                    argument: rest.to_owned(),
                },
                "define" => Op::Define {
                    text: rest.to_owned(),
                },
                "strcmp" => Op::Strcmp {
                    arguments: rest.to_owned(),
                },
                _ => Op::Command {
                    name: head.to_ascii_lowercase(),
                    arguments: rest.to_owned(),
                },
            };
            if matches!(op, Op::Loop { .. } | Op::If { .. }) {
                check_limit(
                    blocks.len().saturating_add(1),
                    limits.max_nesting,
                    line,
                    "block nesting",
                )?;
                blocks.push((pc, None));
            }
            program.instructions.push(Instruction { line, op });
        }
        if in_control {
            return Err(syntax(source.lines().count(), "missing .endc"));
        }
        Ok(program)
    }

    pub fn declarative_source(&self) -> &str {
        &self.declarative_source
    }

    /// Visit authored host commands in source order without evaluating control
    /// flow or substituting variables. Includes commands inside every branch;
    /// intended for static validation, never as an execution sequence.
    pub fn authored_commands(&self) -> impl Iterator<Item = ControlCommand> + '_ {
        self.instructions.iter().filter_map(|instruction| {
            if let Op::Command { name, arguments } = &instruction.op {
                Some(ControlCommand {
                    line: instruction.line,
                    name: name.clone(),
                    arguments: arguments.clone(),
                })
            } else {
                None
            }
        })
    }

    pub fn start(&self, variables: ParamContext) -> ControlSession<'_> {
        ControlSession {
            program: self,
            variables,
            shell: BTreeMap::new(),
            vectors: BTreeSet::new(),
            loops: Vec::new(),
            pc: 0,
            steps: 0,
            failed: false,
            quit_status: None,
        }
    }
}

#[derive(Debug)]
enum LoopState {
    Foreach {
        variable: String,
        values: Vec<String>,
        next: usize,
    },
    Repeat {
        remaining: usize,
    },
    Conditional,
}

#[derive(Debug)]
struct LoopFrame {
    opening: usize,
    end: usize,
    state: LoopState,
}

pub struct ControlSession<'a> {
    program: &'a ControlProgram,
    variables: ParamContext,
    /// Shell variables (`set`, `foreach`, `strcmp`) by lowercase name, as
    /// ngspice keeps them apart from vectors (frontend/variable.c).
    shell: BTreeMap<String, ShellValue>,
    /// Lowercase names the host bound as vectors (`let` of a vector value).
    vectors: BTreeSet<String>,
    loops: Vec<LoopFrame>,
    pc: usize,
    steps: usize,
    failed: bool,
    quit_status: Option<i32>,
}

impl ControlSession<'_> {
    pub fn variables(&self) -> &ParamContext {
        &self.variables
    }

    /// The status an executed `quit` requested, or `None` when the script
    /// ended without one. ngspice exits a batch run with this status.
    pub fn quit_status(&self) -> Option<i32> {
        self.quit_status
    }

    /// Yield the next host command. A host failure must stop the session; it
    /// must not call this again as though that command had succeeded.
    pub fn next_command(
        &mut self,
        evaluator: &mut dyn ControlScalarEvaluator,
        abort: &dyn AbortSignal,
    ) -> Result<Option<ControlCommand>, ControlError> {
        if self.failed {
            return Err(syntax(0, "session already failed"));
        }
        let result = self.advance(evaluator, abort);
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    fn scalar(
        &self,
        expression: &str,
        line: usize,
        evaluator: &mut dyn ControlScalarEvaluator,
        abort: &dyn AbortSignal,
    ) -> Result<ComplexValue, ControlError> {
        let expression = self.substitute(expression, line, evaluator, abort)?;
        let value = evaluator.evaluate_scalar(&expression, &self.variables, line, abort)?;
        finite(value, line)
    }

    fn advance(
        &mut self,
        evaluator: &mut dyn ControlScalarEvaluator,
        abort: &dyn AbortSignal,
    ) -> Result<Option<ControlCommand>, ControlError> {
        while let Some(instruction) = self.program.instructions.get(self.pc) {
            let line = instruction.line;
            check_abort(abort, line)?;
            self.steps = self
                .steps
                .checked_add(1)
                .ok_or_else(|| syntax(line, "execution step count overflow"))?;
            check_limit(
                self.steps,
                self.program.limits.max_steps,
                line,
                "execution steps",
            )?;
            match &instruction.op {
                Op::Let {
                    name,
                    index: Some(index),
                    expression,
                } => {
                    let index = self.substitute(index, line, evaluator, abort)?;
                    let expression = self.substitute(expression, line, evaluator, abort)?;
                    // A scalar `let` keeps its parameter in step with its
                    // one element.
                    if let Some(value) = evaluator.evaluate_indexed_let(
                        name,
                        &index,
                        &expression,
                        &self.variables,
                        line,
                        abort,
                    )? {
                        self.variables.set_complex(name, finite(value, line)?);
                    }
                    self.pc += 1;
                }
                Op::Let {
                    name,
                    index: None,
                    expression,
                } => {
                    let expression = self.substitute(expression, line, evaluator, abort)?;
                    let key = name.to_ascii_lowercase();
                    match evaluator.evaluate_let(name, &expression, &self.variables, line, abort)? {
                        Some(value) => {
                            finite(value, line)?;
                            self.vectors.remove(&key);
                            self.variables.set_complex(name, value);
                        }
                        // The host bound a vector; a scalar of the same name
                        // left in the parameters is stale from here on.
                        None => {
                            self.vectors.insert(key);
                        }
                    }
                    self.pc += 1;
                }
                Op::If {
                    condition,
                    alternative,
                    ..
                } => {
                    self.pc =
                        if self.scalar(condition, line, evaluator, abort)? != ComplexValue::ZERO {
                            self.pc + 1
                        } else {
                            alternative + 1
                        };
                }
                Op::Else { end } => self.pc = end + 1,
                Op::Loop { kind, end } => {
                    match kind {
                        LoopKind::While(condition) => {
                            if self.scalar(condition, line, evaluator, abort)? == ComplexValue::ZERO
                            {
                                if self
                                    .loops
                                    .last()
                                    .is_some_and(|frame| frame.opening == self.pc)
                                {
                                    self.loops.pop();
                                }
                                self.pc = end + 1;
                                continue;
                            }
                            if self
                                .loops
                                .last()
                                .is_none_or(|frame| frame.opening != self.pc)
                            {
                                self.loops.push(LoopFrame {
                                    opening: self.pc,
                                    end: *end,
                                    state: LoopState::Conditional,
                                });
                            }
                        }
                        LoopKind::Dowhile(_) => self.loops.push(LoopFrame {
                            opening: self.pc,
                            end: *end,
                            state: LoopState::Conditional,
                        }),
                        LoopKind::Repeat(expression) => {
                            let count = self.scalar(expression, line, evaluator, abort)?;
                            if !is_real(count)
                                || count.re < 0.0
                                || count.re.fract() != 0.0
                                || count.re >= usize::MAX as Value
                            {
                                return Err(syntax(
                                    line,
                                    "repeat count must be a real, representable nonnegative integer",
                                ));
                            }
                            if count.re == 0.0 {
                                self.pc = end + 1;
                                continue;
                            }
                            self.loops.push(LoopFrame {
                                opening: self.pc,
                                end: *end,
                                state: LoopState::Repeat {
                                    remaining: count.re as usize,
                                },
                            });
                        }
                        LoopKind::Foreach { variable, words } => {
                            let values = split_words(
                                &self.substitute(words, line, evaluator, abort)?,
                                line,
                                self.program.limits.max_loop_values,
                            )?;
                            let Some(first) = values.first() else {
                                self.pc = end + 1;
                                continue;
                            };
                            self.shell.insert(
                                variable.to_ascii_lowercase(),
                                ShellValue::Word(first.clone()),
                            );
                            self.loops.push(LoopFrame {
                                opening: self.pc,
                                end: *end,
                                state: LoopState::Foreach {
                                    variable: variable.clone(),
                                    values,
                                    next: 1,
                                },
                            });
                        }
                    }
                    self.pc += 1;
                }
                Op::End { opening } => {
                    let Some(start) = self.program.instructions.get(*opening) else {
                        return Err(syntax(line, "invalid block target"));
                    };
                    if let Op::Loop { kind, .. } = &start.op {
                        if let LoopKind::While(_) = kind {
                            self.pc = *opening;
                            continue;
                        }
                        if let LoopKind::Dowhile(condition) = kind {
                            if self.scalar(condition, start.line, evaluator, abort)?
                                != ComplexValue::ZERO
                            {
                                self.pc = opening + 1;
                                continue;
                            }
                        } else if let Some(frame) = self.loops.last_mut() {
                            match &mut frame.state {
                                LoopState::Foreach {
                                    variable,
                                    values,
                                    next,
                                } => {
                                    if let Some(value) = values.get(*next) {
                                        self.shell.insert(
                                            variable.to_ascii_lowercase(),
                                            ShellValue::Word(value.clone()),
                                        );
                                        *next += 1;
                                        self.pc = opening + 1;
                                        continue;
                                    }
                                }
                                LoopState::Repeat { remaining } => {
                                    *remaining -= 1;
                                    if *remaining > 0 {
                                        self.pc = opening + 1;
                                        continue;
                                    }
                                }
                                LoopState::Conditional => {}
                            }
                        }
                        self.loops.pop();
                    }
                    self.pc += 1;
                }
                Op::Break => {
                    let frame = self
                        .loops
                        .pop()
                        .ok_or_else(|| syntax(line, "break without an active loop"))?;
                    self.pc = frame.end + 1;
                }
                Op::Continue => {
                    let frame = self
                        .loops
                        .last()
                        .ok_or_else(|| syntax(line, "continue without an active loop"))?;
                    self.pc = frame.end;
                }
                Op::Quit { argument } => {
                    let argument = self.substitute(argument, line, evaluator, abort)?;
                    self.quit_status = Some(quit_status(&argument));
                    self.pc = self.program.instructions.len();
                    return Ok(None);
                }
                Op::Define { text } => {
                    let text = self.substitute(text, line, evaluator, abort)?;
                    let (name, arguments, body) = definition(&text, line)?;
                    // Expressions call it through the parameters, as they
                    // call a deck's .func; a later definition replaces it.
                    self.variables.define_function(name, arguments, body);
                    self.pc += 1;
                }
                Op::Strcmp { arguments } => {
                    let arguments = self.substitute(arguments, line, evaluator, abort)?;
                    let words = shell::words(&arguments);
                    let [variable, first, second] = words.as_slice() else {
                        return Err(syntax(line, "strcmp requires a variable and two strings"));
                    };
                    // frontend/com_strcmp.c:12 sets the variable to the
                    // comparison of the unquoted strings.
                    let order = first.text.cmp(&second.text) as i32;
                    self.shell.insert(
                        variable.text.to_ascii_lowercase(),
                        ShellValue::Word(order.to_string()),
                    );
                    self.pc += 1;
                }
                Op::Command { name, arguments } => {
                    let arguments = self.substitute(arguments, line, evaluator, abort)?;
                    // A shell variable is the session's (variable.c:86
                    // cp_vset); the host still sees `set` and `unset`, for
                    // the variables that steer it.
                    match name.as_str() {
                        "set" | "setcs" => {
                            for assignment in shell::assignments(&arguments, line)? {
                                self.shell.insert(
                                    assignment.name.to_ascii_lowercase(),
                                    ShellValue::assigned(&assignment.value),
                                );
                            }
                        }
                        "unset" => {
                            for word in shell::words(&arguments) {
                                self.shell.remove(&word.text.to_ascii_lowercase());
                            }
                        }
                        _ => {}
                    }
                    self.pc += 1;
                    return Ok(Some(ControlCommand {
                        line,
                        name: name.clone(),
                        arguments,
                    }));
                }
            }
        }
        Ok(None)
    }

    fn substitute(
        &self,
        text: &str,
        line: usize,
        evaluator: &mut dyn ControlScalarEvaluator,
        abort: &dyn AbortSignal,
    ) -> Result<String, ControlError> {
        let mut output = String::new();
        let mut chars = text.chars().peekable();
        let mut braces = 0usize;
        while let Some(character) = chars.next() {
            // ngspice expands `{$name}` to the variable's words (brace
            // expansion after substitution, frontend/parser/glob.c:136), so
            // `"{$gold}.$n"` reads `unknown1.i1`. Other braces keep their
            // expression meaning here, as in `option reltol={expr}`.
            if character == '{' && chars.peek() == Some(&'$') {
                braces += 1;
                continue;
            }
            if character == '}' && braces > 0 {
                braces -= 1;
                continue;
            }
            if character != '$' {
                output.push(character);
                continue;
            }
            let scalar = chars.next_if_eq(&'&').is_some();
            if scalar {
                let name = vector_reference(&mut chars);
                let words = self.vector_words(&name, line, evaluator, abort)?;
                check_limit(
                    output.len().saturating_add(words.len()),
                    self.program.limits.max_source_bytes,
                    line,
                    "expanded command bytes",
                )?;
                output.push_str(&words);
                continue;
            }
            let existence = chars.next_if_eq(&'?').is_some();
            let count = !existence && chars.next_if_eq(&'#').is_some();
            let mut name = String::new();
            while chars
                .peek()
                .is_some_and(|character| character.is_ascii_alphanumeric() || *character == '_')
            {
                if let Some(character) = chars.next() {
                    name.push(character);
                }
            }
            require_identifier(&name, line)?;
            // ngspice reads a shell variable, then one its host defines
            // (frontend/variable.c:908 vareval); a scalar parameter answers
            // after both.
            let key = name.to_ascii_lowercase();
            let value = self.shell.get(&key).cloned().or_else(|| {
                evaluator
                    .host_variable(&name)
                    .map(|words| match <[String; 1]>::try_from(words) {
                        Ok([word]) => ShellValue::Word(word),
                        Err(words) if words.is_empty() => ShellValue::Flag,
                        Err(words) => ShellValue::List(words),
                    })
            });
            if existence {
                output.push(
                    if value.is_some() || self.variables.has_any_parameter_binding(&name) {
                        '1'
                    } else {
                        '0'
                    },
                );
            } else if let Some(value) = value {
                let text = if count {
                    value.count().to_string()
                } else {
                    value.text()
                };
                check_limit(
                    output.len().saturating_add(text.len()),
                    self.program.limits.max_source_bytes,
                    line,
                    "expanded command bytes",
                )?;
                output.push_str(&text);
            } else if self.vectors.contains(&key) {
                return Err(ControlError::new(
                    line,
                    ControlErrorKind::Expression,
                    format!("'${name}' names a vector; a control variable cannot expand it"),
                ));
            } else if count {
                if !self.variables.has_any_parameter_binding(&name) {
                    return Err(ControlError::new(
                        line,
                        ControlErrorKind::Expression,
                        format!("'$#{name}': no such variable"),
                    ));
                }
                output.push('1');
            } else {
                output.push_str(&self.parameter_text(&name, line, abort)?);
            }
            check_limit(
                output.len(),
                self.program.limits.max_source_bytes,
                line,
                "expanded command bytes",
            )?;
        }
        check_limit(
            output.len(),
            self.program.limits.max_source_bytes,
            line,
            "expanded command bytes",
        )?;
        Ok(output)
    }

    /// A scalar parameter as `$name` expands it.
    fn parameter_text(
        &self,
        name: &str,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<String, ControlError> {
        let value = eval_expression_complex_with_abort(name, &self.variables, abort)
            .map_err(|error| ControlError::evaluation(line, error))?;
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(ControlError::new(
                line,
                ControlErrorKind::Expression,
                format!("variable '{name}' is nonfinite"),
            ));
        }
        Ok(if is_real(value) {
            format!("{:.17e}", value.re)
        } else {
            // Group the entire value so surrounding multiplication,
            // negation and powers keep the authored precedence.
            format!("({:.17e}{:+.17e}j)", value.re, value.im)
        })
    }

    /// `$&name` reads a vector (frontend/options.c:132 `cp_enqvec_as_var`):
    /// one `%G` word per element's real part, separated by spaces, as
    /// variable.c `cp_varwl` renders the list. A scalar `let` stays a
    /// session parameter and expands as `$name` does; the host answers for
    /// every vector it keeps.
    fn vector_words(
        &self,
        name: &str,
        line: usize,
        evaluator: &mut dyn ControlScalarEvaluator,
        abort: &dyn AbortSignal,
    ) -> Result<String, ControlError> {
        if name.is_empty() {
            return Err(syntax(line, "invalid variable name ''"));
        }
        let plain = require_identifier(name, line).is_ok();
        let vector = !plain || self.vectors.contains(&name.to_ascii_lowercase());
        let scalar = if vector {
            None
        } else {
            match self.parameter_text(name, line, abort) {
                Ok(text) => return Ok(text),
                Err(error) if error.kind == ControlErrorKind::Expression => Some(error),
                Err(error) => return Err(error),
            }
        };
        if let Some(words) = evaluator.vector_words(name, &self.variables, line, abort)? {
            return Ok(words.join(" "));
        }
        Err(scalar.unwrap_or_else(|| {
            ControlError::new(
                line,
                ControlErrorKind::Expression,
                format!("'$&{name}' names no vector in the current plot"),
            )
        }))
    }
}

/// The text a `$&` reference spans, as ngspice `span_var_expr`
/// (frontend/variable.c:802) measures it: name characters, ending after the
/// parenthesis or bracket that closes the first group.
fn vector_reference(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut name = String::new();
    let (mut parentheses, mut brackets) = (0usize, 0usize);
    while let Some(&character) = chars.peek() {
        if !character.is_ascii_alphanumeric() && !"-_<#?@.()[]&".contains(character) {
            break;
        }
        let closes = match character {
            '[' => {
                brackets += 1;
                false
            }
            '(' => {
                parentheses += 1;
                false
            }
            ']' | ')' => {
                let depth = if character == ']' {
                    &mut brackets
                } else {
                    &mut parentheses
                };
                if *depth == 0 {
                    break;
                }
                *depth -= 1;
                *depth == 0
            }
            _ => false,
        };
        name.push(character);
        chars.next();
        if closes {
            break;
        }
    }
    name
}

fn control_body(text: &str) -> &str {
    let text = text.trim();
    if text.starts_with('*') || text.starts_with('$') {
        return "";
    }
    // ngspice inp_stripcomments_line (frontend/inpcom.c:3546): outside
    // quotes, `;`, `$ ` and `//` start a comment in a control section.
    let mut quote = None;
    let mut characters = text.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        let next = characters.peek().map(|(_, next)| *next);
        if quote == Some(character) {
            quote = None;
        } else if quote.is_none() && matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if quote.is_none()
            && (character == ';'
                || (character == '$' && next == Some(' '))
                || (character == '/' && next == Some('/')))
        {
            return text.get(..index).unwrap_or_default().trim();
        }
    }
    text
}

/// ngspice `com_quit` (frontend/misccoms.c) reads the status with `%d`
/// from the first word; any other word, `noask` included, exits normally.
fn quit_status(argument: &str) -> i32 {
    let word = argument.split_whitespace().next().unwrap_or_default();
    let (sign, digits) = match word.strip_prefix('-') {
        Some(rest) => (-1i64, rest),
        None => (1, word.strip_prefix('+').unwrap_or(word)),
    };
    let digits: String = digits.chars().take_while(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return 0;
    }
    digits
        .parse::<i64>()
        .ok()
        .and_then(|value| i32::try_from(sign * value).ok())
        .unwrap_or(i32::MAX)
}

/// ngspice `com_define` (frontend/define.c:62): the head runs to the first
/// `)` with its spaces removed, and the rest of the line is the body.
fn definition(text: &str, line: usize) -> Result<(&str, Vec<String>, &str), ControlError> {
    let malformed = || syntax(line, "define requires name(arguments) and a body");
    let close = text.find(')').ok_or_else(malformed)?;
    let (name, arguments) = text[..close].split_once('(').ok_or_else(malformed)?;
    let body = text[close + 1..].trim();
    if body.is_empty() {
        return Err(malformed());
    }
    let name = name.trim();
    require_identifier(name, line)?;
    let arguments: Vec<String> = if arguments.trim().is_empty() {
        Vec::new()
    } else {
        arguments
            .split(',')
            .map(|argument| argument.trim().to_owned())
            .collect()
    };
    for argument in &arguments {
        require_identifier(argument, line)?;
    }
    // define.c:98 refuses the name of a built-in vector function.
    if PREDEFINED_FUNCTIONS
        .split_whitespace()
        .any(|function| function.eq_ignore_ascii_case(name))
    {
        return Err(syntax(line, format!("{name} is a predefined function")));
    }
    Ok((name, arguments, body))
}

/// ngspice `ft_funcs` (frontend/parse.c:324).
const PREDEFINED_FUNCTIONS: &str = "mag magnitude cph cphase unwrap ph phase j real re imag \
    im conj db log log10 ln exp abs sqrt sin cos tan sinh cosh tanh atan atanh sortorder norm \
    rnd sunif poisson exponential sgauss pos nint floor ceil mean stddev avg m3avg group_delay \
    vector cvector unitvec length vecmin minimum vecmax maximum vecd interpolate deriv integ fft \
    ifft mtimeavg v minus not";

fn split_head(text: &str) -> (&str, &str) {
    text.split_once(char::is_whitespace)
        .map_or((text, ""), |(head, rest)| (head, rest.trim()))
}

fn split_words(text: &str, line: usize, max_words: usize) -> Result<Vec<String>, ControlError> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quote = None;
    let mut started = false;
    for character in text.chars() {
        if quote == Some(character) {
            quote = None;
        } else if quote.is_none() && matches!(character, '\'' | '"') {
            quote = Some(character);
            started = true;
        } else if quote.is_none() && character.is_whitespace() {
            if started {
                check_limit(
                    words.len().saturating_add(1),
                    max_words,
                    line,
                    "foreach values",
                )?;
                words.push(std::mem::take(&mut word));
                started = false;
            }
        } else {
            word.push(character);
            started = true;
        }
    }
    if quote.is_some() {
        return Err(syntax(line, "unterminated quoted word"));
    }
    if started {
        check_limit(
            words.len().saturating_add(1),
            max_words,
            line,
            "foreach values",
        )?;
        words.push(word);
    }
    Ok(words)
}

fn require_identifier(name: &str, line: usize) -> Result<(), ControlError> {
    let mut chars = name.chars();
    if !chars
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        || !chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(syntax(line, format!("invalid variable name '{name}'")));
    }
    Ok(())
}

fn require_empty(text: &str, line: usize) -> Result<(), ControlError> {
    if text.is_empty() {
        Ok(())
    } else {
        Err(syntax(line, "unexpected command arguments"))
    }
}

fn check_abort(abort: &dyn AbortSignal, line: usize) -> Result<(), ControlError> {
    if abort.is_aborted() {
        Err(ControlError::new(
            line,
            ControlErrorKind::Aborted,
            "execution aborted",
        ))
    } else {
        Ok(())
    }
}

fn check_limit(actual: usize, limit: usize, line: usize, name: &str) -> Result<(), ControlError> {
    if actual > limit {
        Err(ControlError::new(
            line,
            ControlErrorKind::ResourceLimit,
            format!("{name} limit {limit} exceeded"),
        ))
    } else {
        Ok(())
    }
}

fn finite(value: ComplexValue, line: usize) -> Result<ComplexValue, ControlError> {
    if value.re.is_finite() && value.im.is_finite() {
        Ok(value)
    } else {
        Err(ControlError::new(
            line,
            ControlErrorKind::Expression,
            "scalar expression is nonfinite",
        ))
    }
}

fn syntax(line: usize, message: impl Into<String>) -> ControlError {
    ControlError::new(line, ControlErrorKind::Syntax, message)
}

#[cfg(test)]
mod tests;

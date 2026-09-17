//! Resumable, target-neutral execution of ngspice control flow.
//!
//! A session evaluates assignments and loops, yielding each external command
//! in source order. The caller executes that command before requesting another.
//! Analysis, plotting and file commands are never silently discarded here.

use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::netlist::expr::{ParamContext, eval_expression};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlErrorKind {
    Syntax,
    Expression,
    ResourceLimit,
    Aborted,
    Host,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlError {
    pub line: usize,
    pub kind: ControlErrorKind,
    pub message: String,
}

impl ControlError {
    pub fn new(line: usize, kind: ControlErrorKind, message: impl Into<String>) -> Self {
        Self {
            line,
            kind,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ControlError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "control line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ControlError {}

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

/// One command for the analysis/output host, after variable substitution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlCommand {
    pub line: usize,
    pub name: String,
    pub arguments: String,
}

/// Hosts may extend scalar evaluation with values from completed datasets.
pub trait ControlScalarEvaluator {
    fn evaluate_scalar(
        &mut self,
        expression: &str,
        variables: &ParamContext,
        line: usize,
    ) -> Result<Value, ControlError>;
}

#[derive(Default)]
pub struct ParameterScalarEvaluator;

impl ControlScalarEvaluator for ParameterScalarEvaluator {
    fn evaluate_scalar(
        &mut self,
        expression: &str,
        variables: &ParamContext,
        line: usize,
    ) -> Result<Value, ControlError> {
        eval_expression(expression, variables).map_err(|error| {
            ControlError::new(line, ControlErrorKind::Expression, error.to_string())
        })
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
        expression: String,
    },
    Break,
    Continue,
    Quit,
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
                    let (name, expression) = rest
                        .split_once('=')
                        .ok_or_else(|| syntax(line, "let requires name = expression"))?;
                    require_identifier(name.trim(), line)?;
                    if expression.trim().is_empty() {
                        return Err(syntax(line, "let requires an expression"));
                    }
                    Op::Let {
                        name: name.trim().to_owned(),
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
                "quit" => {
                    require_empty(rest, line)?;
                    Op::Quit
                }
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

    pub fn start(&self, variables: ParamContext) -> ControlSession<'_> {
        ControlSession {
            program: self,
            variables,
            words: BTreeMap::new(),
            loops: Vec::new(),
            pc: 0,
            steps: 0,
            failed: false,
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
    words: BTreeMap<String, String>,
    loops: Vec<LoopFrame>,
    pc: usize,
    steps: usize,
    failed: bool,
}

impl ControlSession<'_> {
    pub fn variables(&self) -> &ParamContext {
        &self.variables
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
    ) -> Result<Value, ControlError> {
        let expression = self.substitute(expression, line)?;
        let value = evaluator.evaluate_scalar(&expression, &self.variables, line)?;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(ControlError::new(
                line,
                ControlErrorKind::Expression,
                "scalar expression is nonfinite",
            ))
        }
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
                Op::Let { name, expression } => {
                    let value = self.scalar(expression, line, evaluator)?;
                    self.variables.set(name, value);
                    self.pc += 1;
                }
                Op::If {
                    condition,
                    alternative,
                    ..
                } => {
                    self.pc = if self.scalar(condition, line, evaluator)? != 0.0 {
                        self.pc + 1
                    } else {
                        alternative + 1
                    };
                }
                Op::Else { end } => self.pc = end + 1,
                Op::Loop { kind, end } => {
                    match kind {
                        LoopKind::While(condition) => {
                            if self.scalar(condition, line, evaluator)? == 0.0 {
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
                            let count = self.scalar(expression, line, evaluator)?;
                            if count < 0.0 || count.fract() != 0.0 || count >= usize::MAX as f64 {
                                return Err(syntax(
                                    line,
                                    "repeat count must be a representable nonnegative integer",
                                ));
                            }
                            if count == 0.0 {
                                self.pc = end + 1;
                                continue;
                            }
                            self.loops.push(LoopFrame {
                                opening: self.pc,
                                end: *end,
                                state: LoopState::Repeat {
                                    remaining: count as usize,
                                },
                            });
                        }
                        LoopKind::Foreach { variable, words } => {
                            let values = split_words(
                                &self.substitute(words, line)?,
                                line,
                                self.program.limits.max_loop_values,
                            )?;
                            let Some(first) = values.first() else {
                                self.pc = end + 1;
                                continue;
                            };
                            self.words.insert(variable.clone(), first.clone());
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
                            if self.scalar(condition, start.line, evaluator)? != 0.0 {
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
                                        self.words.insert(variable.clone(), value.clone());
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
                Op::Quit => {
                    self.pc = self.program.instructions.len();
                    return Ok(None);
                }
                Op::Command { name, arguments } => {
                    let command = ControlCommand {
                        line,
                        name: name.clone(),
                        arguments: self.substitute(arguments, line)?,
                    };
                    self.pc += 1;
                    return Ok(Some(command));
                }
            }
        }
        Ok(None)
    }

    fn substitute(&self, text: &str, line: usize) -> Result<String, ControlError> {
        let mut output = String::new();
        let mut chars = text.chars().peekable();
        while let Some(character) = chars.next() {
            if character != '$' {
                output.push(character);
                continue;
            }
            let scalar = chars.next_if_eq(&'&').is_some();
            let existence = !scalar && chars.next_if_eq(&'?').is_some();
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
            if existence {
                output.push(
                    if self.words.contains_key(&name)
                        || self.variables.has_any_parameter_binding(&name)
                    {
                        '1'
                    } else {
                        '0'
                    },
                );
            } else if !scalar && let Some(value) = self.words.get(&name) {
                check_limit(
                    output.len().saturating_add(value.len()),
                    self.program.limits.max_source_bytes,
                    line,
                    "expanded command bytes",
                )?;
                output.push_str(value);
            } else {
                let value = eval_expression(&name, &self.variables).map_err(|error| {
                    ControlError::new(line, ControlErrorKind::Expression, error.to_string())
                })?;
                if !value.is_finite() {
                    return Err(ControlError::new(
                        line,
                        ControlErrorKind::Expression,
                        format!("variable '{name}' is nonfinite"),
                    ));
                }
                output.push_str(&format!("{value:.17e}"));
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
}

fn control_body(text: &str) -> &str {
    let text = text.trim();
    if text.starts_with('*') || text.starts_with('$') {
        return "";
    }
    let mut quote = None;
    for (index, character) in text.char_indices() {
        if quote == Some(character) {
            quote = None;
        } else if quote.is_none() && matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if quote.is_none() && character == ';' {
            return text.get(..index).unwrap_or_default().trim();
        }
    }
    text
}

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

fn syntax(line: usize, message: impl Into<String>) -> ControlError {
    ControlError::new(line, ControlErrorKind::Syntax, message)
}

#[cfg(test)]
mod tests;

//! The programs a deck's `d_process` instances start.
//!
//! The CLI starts a `d_process` program by the path its deck names, read from
//! the root deck's directory when that path is relative. A prepared run is
//! solved without a source path, so preparation writes each relative program
//! path into the executable deck as that directory resolves it, and the solve
//! starts the program the CLI starts. A program named bare, with no
//! directory separator, is resolved there only when a file of that name
//! stands beside the deck; otherwise the solve looks it up on `PATH`, on
//! both routes. A host that cannot start a program, as the browser's worker
//! cannot, refuses each instance by name before solving.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{normalize_model_string_path_value, strip_spice_inline_comment};

const PROCESS_FILE: &str = "process_file";

/// Whether this build's solve can start a program: everywhere but the
/// browser, whose worker has no processes.
pub(crate) const STARTS_PROGRAMS: bool = cfg!(not(target_arch = "wasm32"));

/// One token of a code-model record.
#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    /// A word, with the byte range it spans in its physical line.
    Word(&'a str, std::ops::Range<usize>),
    /// A quoted string: its quote, its text, and the byte range of the text.
    Quoted(char, &'a str, std::ops::Range<usize>),
    Equals,
    Punctuation,
}

/// The tokens of one physical line's executable portion.
fn tokens(line: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut characters = line.char_indices().peekable();
    while let Some((start, character)) = characters.next() {
        match character {
            character if character.is_whitespace() => {}
            '=' => tokens.push(Token::Equals),
            '(' | ')' | ',' | '[' | ']' => tokens.push(Token::Punctuation),
            '"' | '\'' => {
                let text_start = start + 1;
                let mut end = line.len();
                for (index, next) in characters.by_ref() {
                    if next == character {
                        end = index;
                        break;
                    }
                }
                tokens.push(Token::Quoted(
                    character,
                    &line[text_start..end],
                    text_start..end,
                ));
            }
            _ => {
                let mut end = line.len();
                while let Some(&(index, next)) = characters.peek() {
                    if next.is_whitespace() || "=(),[]\"'".contains(next) {
                        end = index;
                        break;
                    }
                    characters.next();
                }
                tokens.push(Token::Word(&line[start..end], start..end));
            }
        }
    }
    tokens
}

/// Where a `process_file = value` assignment stands as a record's tokens
/// arrive, which may be across continuation lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Assignment {
    Idle,
    Named,
    Assigned,
}

impl Assignment {
    /// The next state after `token`, and whether `token` is the value of a
    /// `process_file` assignment.
    fn step(self, token: &Token<'_>) -> (Self, bool) {
        match (self, token) {
            (_, Token::Word(word, _)) if word.eq_ignore_ascii_case(PROCESS_FILE) => {
                (Self::Named, false)
            }
            (Self::Named, Token::Equals) => (Self::Assigned, false),
            (Self::Assigned, Token::Word(..) | Token::Quoted(..)) => (Self::Idle, true),
            _ => (Self::Idle, false),
        }
    }
}

/// The executable portion of a physical line: what precedes its inline
/// comment, with the byte offset at which it starts.
fn executable_portion(line: &str) -> (usize, &str) {
    let code = strip_spice_inline_comment(line, ExpressionDialect::Ngspice);
    let start = (code.as_ptr() as usize)
        .checked_sub(line.as_ptr() as usize)
        .filter(|start| start + code.len() <= line.len())
        .unwrap_or(0);
    (start, line.get(start..start + code.len()).unwrap_or(line))
}

/// Whether a record whose first word is `first` declares or instantiates a
/// code model, where `process_file` names a program.
fn is_code_model_record(first: &str) -> bool {
    let first = first.to_ascii_lowercase();
    first == ".model" || first.starts_with('a')
}

/// `source` with every relative `process_file` program of a code-model
/// record resolved from `deck_dir`, as the CLI resolves it from the root
/// deck's directory. Without a directory, or on a host that cannot start a
/// program, the source is returned as it is.
pub(crate) fn resolve_process_programs(source: String, deck_dir: Option<&Path>) -> String {
    let Some(deck_dir) = deck_dir.filter(|_| STARTS_PROGRAMS) else {
        return source;
    };
    if !source.to_ascii_lowercase().contains(PROCESS_FILE) {
        return source;
    }
    let mut resolved = String::with_capacity(source.len());
    let mut code_model = false;
    let mut state = Assignment::Idle;
    for physical in source.split_inclusive('\n') {
        let (offset, code) = executable_portion(physical);
        let trimmed = code.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            resolved.push_str(physical);
            continue;
        }
        let body_start = offset + (code.len() - trimmed.len());
        let (body_start, body) = match trimmed.strip_prefix('+') {
            Some(rest) => (body_start + 1, rest),
            None => {
                code_model = trimmed
                    .split_whitespace()
                    .next()
                    .is_some_and(is_code_model_record);
                state = Assignment::Idle;
                (body_start, trimmed)
            }
        };
        if !code_model {
            resolved.push_str(physical);
            continue;
        }
        let mut written = 0;
        for token in tokens(body) {
            let (next, is_value) = state.step(&token);
            state = next;
            if !is_value {
                continue;
            }
            let (quote, value, range) = match token {
                Token::Word(value, range) => (None, value, range),
                Token::Quoted(quote, value, range) => (Some(quote), value, range),
                Token::Equals | Token::Punctuation => continue,
            };
            if value.starts_with('{') {
                continue;
            }
            let program = normalize_model_string_path_value(PROCESS_FILE, value, Some(deck_dir));
            if program == value {
                continue;
            }
            let range = body_start + range.start..body_start + range.end;
            resolved.push_str(&physical[written..range.start]);
            let needs_quotes = quote.is_none()
                && program.contains(|c: char| c.is_whitespace() || "()=,".contains(c));
            if needs_quotes {
                resolved.push('"');
            }
            resolved.push_str(&program);
            if needs_quotes {
                resolved.push('"');
            }
            written = range.end;
        }
        resolved.push_str(&physical[written..]);
    }
    resolved
}

/// The programs a code-model record's `process_file` assignments name.
pub(crate) fn process_programs(record: &str) -> Vec<&str> {
    let mut state = Assignment::Idle;
    let mut programs = Vec::new();
    for token in tokens(record) {
        let (next, is_value) = state.step(&token);
        state = next;
        if is_value {
            match token {
                Token::Word(value, _) | Token::Quoted(_, value, _) => programs.push(value),
                Token::Equals | Token::Punctuation => {}
            }
        }
    }
    programs
}

/// Whether a solve with no source path starts `program` where the CLI
/// starts it: an absolute path, or an empty one the code model refuses
/// itself when the circuit is built.
pub(crate) fn names_its_program_absolutely(program: &str) -> bool {
    let base = program
        .strip_suffix("||")
        .or_else(|| program.strip_suffix('|'))
        .unwrap_or(program);
    base.trim().is_empty() || Path::new(base).is_absolute()
}

/// Whether `program` is a bare program name, with no directory and no FIFO
/// suffix: the one form a solve looks up on `PATH` when no file beside the
/// deck answers to it.
pub(crate) fn is_bare_program(program: &str) -> bool {
    !program.trim().is_empty() && !program.ends_with('|') && !program.contains(['/', '\\'])
}

/// The file a bare `program` starts from when it is looked up on `PATH`:
/// the first directory there holding it, as `execvp(3)` searches. Windows,
/// as `std::process::Command` does, also tries the name with `.exe`.
pub(crate) fn program_on_path(program: &str, search_path: Option<&OsStr>) -> Option<PathBuf> {
    if !is_bare_program(program) {
        return None;
    }
    std::env::split_paths(search_path?).find_map(|directory| {
        let candidate = directory.join(program);
        if candidate.is_file() {
            return Some(candidate);
        }
        let executable = candidate.with_extension("exe");
        (cfg!(windows) && Path::new(program).extension().is_none() && executable.is_file())
            .then_some(executable)
    })
}

/// The model a code-model instance record names: its last word that is
/// neither a parameter's name nor its value.
fn instance_model(record: &str) -> Option<&str> {
    let tokens = tokens(record);
    let mut model = None;
    let mut index = 0;
    while index < tokens.len() {
        if matches!(tokens.get(index + 1), Some(Token::Equals)) {
            index += 3;
            continue;
        }
        if let Token::Word(word, _) = tokens[index] {
            model = Some(word);
        }
        index += 1;
    }
    model
}

/// The `d_process` instances among `records` that `record` starts a program
/// for: the instance itself, or every instance of the model it declares.
pub(crate) fn process_instances<'a>(
    records: &'a [(usize, String)],
    record: &'a str,
) -> Vec<&'a str> {
    let mut words = record.split_whitespace();
    let first = words.next().unwrap_or_default();
    if !first.eq_ignore_ascii_case(".model") {
        return vec![first];
    }
    let Some(model) = words.next() else {
        return Vec::new();
    };
    records
        .iter()
        .filter_map(|(_, candidate)| {
            let name = candidate.split_whitespace().next()?;
            let instance = is_code_model_record(name) && !name.eq_ignore_ascii_case(".model");
            (instance && instance_model(candidate)?.eq_ignore_ascii_case(model)).then_some(name)
        })
        .collect()
}

#[cfg(test)]
mod tests;

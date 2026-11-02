//! ngspice command-line words, `set` assignments and shell variables
//! (frontend/parser/lexical.c, frontend/variable.c `cp_setparse`).

use crate::Value;
use crate::control_protocol::{ControlError, ControlErrorKind};

fn syntax(line: usize, message: impl Into<String>) -> ControlError {
    ControlError::new(line, ControlErrorKind::Syntax, message)
}

/// One shell word. Quotes group whitespace and are removed, as `cp_unquote`
/// removes them before a command sees its words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Word {
    pub text: String,
    /// Whether any part of the word was quoted: a quoted `set` value is a
    /// string even when it reads as a number.
    pub quoted: bool,
    /// Byte offset in `text` of the first `=` outside quotes.
    equals: Option<usize>,
}

impl Word {
    pub(crate) fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            quoted: false,
            equals: None,
        }
    }
}

pub(crate) fn words(text: &str) -> Vec<Word> {
    let mut words = Vec::new();
    let mut current: Option<Word> = None;
    let mut quote = None;
    for character in text.chars() {
        if let Some(open) = quote {
            if character == open {
                quote = None;
            } else if let Some(word) = current.as_mut() {
                word.text.push(character);
            }
        } else if character.is_whitespace() {
            words.extend(current.take());
        } else {
            let word = current.get_or_insert_with(|| Word::plain(String::new()));
            if matches!(character, '"' | '\'') {
                quote = Some(character);
                word.quoted = true;
            } else {
                if character == '=' && word.equals.is_none() {
                    word.equals = Some(word.text.len());
                }
                word.text.push(character);
            }
        }
    }
    words.extend(current);
    words
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SetValue {
    /// `set name`: a Boolean that is now true.
    Flag,
    Word(Word),
    /// `set name = ( a b c )`.
    List(Vec<Word>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Assignment {
    pub name: String,
    pub value: SetValue,
}

/// The part of `word` after its first unquoted `=`, if that part is not empty.
fn after_equals(word: &Word, split: usize) -> Option<Word> {
    let value = &word.text[split + 1..];
    (!value.is_empty()).then(|| Word {
        text: value.to_owned(),
        quoted: word.quoted,
        equals: None,
    })
}

/// Parse `set` arguments in every form `cp_setparse` accepts: `x`, `x=3`,
/// `x= 3`, `x =3`, `x = 3` and `x = ( 1 2 )`, several per command.
pub(crate) fn assignments(text: &str, line: usize) -> Result<Vec<Assignment>, ControlError> {
    let words = words(text);
    let mut assignments = Vec::new();
    let mut index = 0;
    while let Some(word) = words.get(index) {
        index += 1;
        let (name, value) = if let Some(split) = word.equals {
            (word.text[..split].to_owned(), after_equals(word, split))
        } else if let Some(next) = words.get(index).filter(|next| next.equals == Some(0)) {
            index += 1;
            (word.text.clone(), after_equals(next, 0))
        } else {
            assignments.push(Assignment {
                name: word.text.clone(),
                value: SetValue::Flag,
            });
            continue;
        };
        if name.is_empty() {
            return Err(syntax(line, "set requires a variable name before '='"));
        }
        let value = match value {
            Some(value) => value,
            None => {
                let value = words
                    .get(index)
                    .cloned()
                    .ok_or_else(|| syntax(line, format!("set {name} equals what?")))?;
                index += 1;
                value
            }
        };
        let value = if !value.quoted && value.text == "(" {
            let mut items = Vec::new();
            let mut balance = 1usize;
            loop {
                let item = words
                    .get(index)
                    .ok_or_else(|| syntax(line, format!("set {name}: unterminated list")))?;
                index += 1;
                if !item.quoted && item.text == "(" {
                    balance += 1;
                } else if !item.quoted && item.text == ")" {
                    balance -= 1;
                    if balance == 0 {
                        break;
                    }
                }
                items.push(item.clone());
            }
            SetValue::List(items)
        } else {
            SetValue::Word(value)
        };
        assignments.push(Assignment { name, value });
    }
    Ok(assignments)
}

/// An ngspice shell variable (frontend/variable.c `struct variable`), with
/// its numbers already rendered as `cp_varwl` renders them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShellValue {
    /// A Boolean: `set name`, or `batchmode` under `-b`.
    Flag,
    Word(String),
    List(Vec<String>),
}

impl ShellValue {
    /// The value `cp_set` stores (variable.c:387 `cp_setparse`): a quoted
    /// word is a string; otherwise a word that starts with a number is that
    /// number (an integer when it is one), and any other word a string.
    pub(crate) fn assigned(value: &SetValue) -> Self {
        let word = |word: &Word| {
            if word.quoted {
                return word.text.clone();
            }
            match number_prefix(&word.text) {
                Some(number) if number == f64::from(number as i32) => {
                    format!("{}", number as i32)
                }
                Some(number) => percent_g(number),
                None => word.text.clone(),
            }
        };
        match value {
            SetValue::Flag => Self::Flag,
            SetValue::Word(value) => Self::Word(word(value)),
            SetValue::List(items) => Self::List(items.iter().map(word).collect()),
        }
    }

    /// `$name` (variable.c:36 `cp_varwl`): a Boolean reads TRUE, and a list
    /// its words separated by spaces.
    pub(crate) fn text(&self) -> String {
        match self {
            Self::Flag => "TRUE".into(),
            Self::Word(word) => word.clone(),
            Self::List(items) => items.join(" "),
        }
    }

    /// `$#name` (variable.c:963): a list's length, 0 for a Boolean, else 1.
    pub(crate) fn count(&self) -> usize {
        match self {
            Self::Flag => 0,
            Self::Word(_) => 1,
            Self::List(items) => items.len(),
        }
    }
}

/// ngspice `ft_numparse` of a word's start, not requiring the whole word
/// (frontend/parser/numparse.c:40): a decimal number, then an exponent or
/// one scale factor; the letters after it are ignored. `None` when the word
/// does not start with a number.
pub(crate) fn number_prefix(text: &str) -> Option<Value> {
    let bytes = text.as_bytes();
    let digits = |mut index: usize| {
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        index
    };
    // numparse.c:181 get_decimal_number: a sign, then a digit or a point
    // followed by a digit.
    let decimal = |start: usize| -> Option<(Value, usize)> {
        let sign = usize::from(matches!(bytes.get(start), Some(b'+' | b'-')));
        let first = start + sign;
        let whole = digits(first);
        if whole == first && !(bytes.get(first) == Some(&b'.') && whole + 1 < digits(first + 1)) {
            return None;
        }
        let end = if bytes.get(whole) == Some(&b'.') {
            digits(whole + 1)
        } else {
            whole
        };
        let mut number = text[first..end].to_owned();
        if number.ends_with('.') {
            number.push('0');
        }
        if number.starts_with('.') {
            number.insert(0, '0');
        }
        let value: Value = number.parse().ok()?;
        Some((
            if bytes.get(start) == Some(&b'-') {
                -value
            } else {
                value
            },
            end,
        ))
    };
    let (mut mantissa, end) = decimal(0)?;
    let suffix = text[end..].to_ascii_lowercase();
    let exponent = match suffix.as_bytes().first() {
        Some(b'e') => decimal(end + 1).map_or(0.0, |(exponent, _)| exponent),
        Some(b't') => 12.0,
        Some(b'g') => 9.0,
        Some(b'k') => 3.0,
        Some(b'u') => -6.0,
        Some(b'n') => -9.0,
        Some(b'p') => -12.0,
        Some(b'f') => -15.0,
        Some(b'a') => -18.0,
        Some(b'm') if suffix.starts_with("meg") => 6.0,
        Some(b'm') if suffix.starts_with("mil") => {
            mantissa *= 25.4;
            -6.0
        }
        Some(b'm') => -3.0,
        _ => 0.0,
    };
    Some(if exponent == 0.0 {
        mantissa
    } else {
        mantissa * Value::powf(10.0, exponent)
    })
}

/// An output redirection: the file a command's output goes to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Redirect {
    pub path: String,
    /// `>>`: the output is appended to the file.
    pub append: bool,
}

/// Split ngspice's output redirection off a command's arguments
/// (frontend/streams.c:47 `cp_redirect`): an unquoted `>` starts it, a second
/// `>` appends, an `&` sends errors there too, and the next word names the
/// file. The words after the file stay the command's.
pub(crate) fn split_redirect(
    text: &str,
    line: usize,
) -> Result<(String, Option<Redirect>), ControlError> {
    let mut quote = None;
    let mut start = None;
    for (index, character) in text.char_indices() {
        if quote == Some(character) {
            quote = None;
        } else if quote.is_none() && matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if quote.is_none() && character == '>' {
            start = Some(index);
            break;
        }
    }
    let Some(start) = start else {
        return Ok((text.to_owned(), None));
    };
    let mut rest = text[start + 1..].trim_start();
    let append = rest.starts_with('>');
    if append {
        rest = rest[1..].trim_start();
    }
    if let Some(after) = rest.strip_prefix('&') {
        rest = after.trim_start();
    }
    let mut quote = None;
    let end = rest
        .char_indices()
        .find(|&(_, character)| {
            if quote == Some(character) {
                quote = None;
            } else if quote.is_none() && matches!(character, '\'' | '"') {
                quote = Some(character);
            }
            quote.is_none() && (character.is_whitespace() || character == '>')
        })
        .map_or(rest.len(), |(index, _)| index);
    let path = words(&rest[..end])
        .into_iter()
        .next()
        .map(|word| word.text)
        .filter(|path| !path.is_empty())
        .ok_or_else(|| syntax(line, "missing name for output"))?;
    let remainder = &rest[end..];
    if split_redirect(remainder, line)?.1.is_some() {
        return Err(syntax(line, "ambiguous output redirect"));
    }
    let arguments = format!("{}{remainder}", &text[..start]);
    Ok((arguments.trim().to_owned(), Some(Redirect { path, append })))
}

/// C's `%.*e`: `precision` decimals and an exponent of at least two digits.
pub(crate) fn percent_e(value: Value, precision: usize) -> String {
    let text = format!("{value:.precision$e}");
    let Some((mantissa, exponent)) = text.split_once('e') else {
        return text;
    };
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{mantissa}e{sign}{:02}", exponent.unsigned_abs())
}

/// C's `%G` (frontend/variable.c:53 `cp_varwl`): six significant digits,
/// in exponent form below 1e-4 or from 1e6 on, without trailing zeros.
pub(crate) fn percent_g(value: Value) -> String {
    percent_g_with(value, 6, true)
}

/// C's `%.*g` (`%.*G` when `upper`): `precision` significant digits, in
/// exponent form below 1e-4 or from 10^precision on, without trailing zeros.
pub(crate) fn percent_g_with(value: Value, precision: usize, upper: bool) -> String {
    if value.is_nan() {
        return "NAN".into();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-INF" } else { "INF" }.into();
    }
    let trim = |text: &str| -> String {
        if text.contains('.') {
            text.trim_end_matches('0').trim_end_matches('.').to_owned()
        } else {
            text.to_owned()
        }
    };
    let precision = precision.max(1);
    let digits = precision - 1;
    let text = format!("{value:.digits$e}");
    let (mantissa, exponent) = text.split_once('e').unwrap_or((text.as_str(), "0"));
    let exponent: i32 = exponent.parse().unwrap_or(0);
    if (-4..precision as i32).contains(&exponent) {
        let decimals = (digits as i32 - exponent) as usize;
        trim(&format!("{value:.decimals$}"))
    } else {
        let sign = if exponent < 0 { '-' } else { '+' };
        let e = if upper { 'E' } else { 'e' };
        format!("{}{e}{sign}{:02}", trim(mantissa), exponent.unsigned_abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quoted(text: &str) -> Word {
        Word {
            text: text.into(),
            quoted: true,
            equals: None,
        }
    }

    #[test]
    fn words_group_quotes_and_collapse_whitespace() {
        let words = words("hello   \"a  b\" 'c d'e");
        let texts: Vec<_> = words.iter().map(|word| word.text.as_str()).collect();
        assert_eq!(texts, ["hello", "a  b", "c de"]);
        assert!(!words[0].quoted && words[1].quoted && words[2].quoted);
    }

    /// variable.c:387 `cp_setparse` stores numbers through `ft_numparse`
    /// (frontend/parser/numparse.c:40), and `cp_varwl` prints them `%d`
    /// or `%G`; ngspice-46 echoes these words for the same `set` values.
    #[test]
    fn shell_values_render_numbers_as_ngspice_does() {
        let value = |text: &str| {
            ShellValue::assigned(&assignments(&format!("x = {text}"), 1).unwrap()[0].value).text()
        };
        for (text, expected) in [
            ("3", "3"),
            ("-4", "-4"),
            ("1k", "1000"),
            ("2.5", "2.5"),
            ("1.5meg", "1500000"),
            ("1e-20", "1E-20"),
            (".5", "0.5"),
            ("7.", "7"),
            ("3abc", "3E-18"),
            ("2mil", "5.08E-05"),
            ("abc", "abc"),
            ("\"3\"", "3"),
            ("( 1 2.50 x )", "1 2.5 x"),
        ] {
            assert_eq!(value(text), expected, "{text}");
        }
        assert_eq!(ShellValue::Flag.text(), "TRUE");
        assert_eq!(ShellValue::Flag.count(), 0);
        assert_eq!(ShellValue::List(vec!["a".into(), "b".into()]).count(), 2);
        for (value, expected) in [
            (0.0, "0"),
            (-0.0, "-0"),
            (1234567.0, "1.23457E+06"),
            (999999.6, "1E+06"),
            (0.0001, "0.0001"),
            (0.00001234567, "1.23457E-05"),
            (100000.0, "100000"),
        ] {
            assert_eq!(percent_g(value), expected);
        }
    }

    #[test]
    fn assignments_accept_every_cp_setparse_form() {
        let parsed = assignments(
            "noaskquit color0=white x= 3 y =4 z = 5 list = ( 1 \"two\" ) s=\"7\" color2=\"#ee82ee\"",
            1,
        )
        .unwrap();
        let named: Vec<_> = parsed
            .iter()
            .map(|assignment| (assignment.name.as_str(), assignment.value.clone()))
            .collect();
        let word = |text: &str| SetValue::Word(Word::plain(text));
        assert_eq!(
            named,
            [
                ("noaskquit", SetValue::Flag),
                ("color0", word("white")),
                ("x", word("3")),
                ("y", word("4")),
                ("z", word("5")),
                (
                    "list",
                    SetValue::List(vec![Word::plain("1"), quoted("two")])
                ),
                ("s", SetValue::Word(quoted("7"))),
                ("color2", SetValue::Word(quoted("#ee82ee"))),
            ]
        );
        assert!(assignments("x =", 1).is_err());
        assert!(assignments("x = ( 1 2", 1).is_err());
    }
}

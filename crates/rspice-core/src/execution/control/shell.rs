//! ngspice command-line words and `set` assignments (frontend/parser/lexical.c,
//! frontend/variable.c `cp_setparse`).

use super::{ControlError, syntax};

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

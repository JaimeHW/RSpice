//! Preserve executable control text at the normal source-provider boundary.

use super::include::{ExpandedSource, ExpandedSourceItem};
use super::*;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Control regions, with expanded line numbers mapped back to their authors.
/// Declarative lines are blank so this source can be passed directly to the
/// shared control interpreter without duplicating the circuit's source text.
#[derive(Debug, Clone)]
pub struct ControlScriptSource {
    text: String,
    origins: BTreeMap<usize, NetlistSourceLocation>,
}

impl ControlScriptSource {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn origin(&self, expanded_line: usize) -> Option<&NetlistSourceLocation> {
        self.origins.get(&expanded_line)
    }
}

struct Extractor {
    script: ControlScriptSource,
    opened: Option<NetlistSourceLocation>,
    found: bool,
    line: usize,
}

impl Extractor {
    fn new() -> Self {
        Self {
            script: ControlScriptSource {
                text: String::new(),
                origins: BTreeMap::new(),
            },
            opened: None,
            found: false,
            line: 0,
        }
    }

    fn line(
        &mut self,
        text: &mut String,
        origin: &NetlistSourceLocation,
        title: bool,
    ) -> Result<(), ParseWithAbortError> {
        self.line += 1;
        if title {
            self.script.text.push_str("control script\n");
            return Ok(());
        }
        let head = text.split_whitespace().next().unwrap_or_default();
        let boundary = head.eq_ignore_ascii_case(".control") || head.eq_ignore_ascii_case(".endc");
        if head.eq_ignore_ascii_case(".control") {
            if self.opened.is_some() {
                return Err(ParseError::Syntax {
                    line: origin.line,
                    message: "nested .CONTROL region".into(),
                }
                .into());
            }
            self.opened = Some(origin.clone());
            self.found = true;
        } else if head.eq_ignore_ascii_case(".endc") && self.opened.take().is_none() {
            return Err(ParseError::Syntax {
                line: origin.line,
                message: ".ENDC without .CONTROL".into(),
            }
            .into());
        }
        if boundary || self.opened.is_some() {
            self.script.text.push_str(text);
            self.script.origins.insert(self.line, origin.clone());
            // Preserve mapped physical lines and include-frame events. The
            // declarative parser must never promote anything from this block.
            text.clear();
        }
        self.script.text.push('\n');
        Ok(())
    }

    fn finish(self) -> Result<Option<Arc<ControlScriptSource>>, ParseWithAbortError> {
        if let Some(origin) = self.opened {
            return Err(ParseError::Syntax {
                line: origin.line,
                message: ".CONTROL without .ENDC".into(),
            }
            .into());
        }
        Ok(self.found.then(|| Arc::new(self.script)))
    }
}

pub(super) fn prepare_text<'a>(
    input: &'a str,
    retain: bool,
    abort: &dyn AbortSignal,
) -> Result<(std::borrow::Cow<'a, str>, Option<Arc<ControlScriptSource>>), ParseWithAbortError> {
    if !retain {
        return Ok((std::borrow::Cow::Borrowed(input), None));
    }
    let mut extractor = Extractor::new();
    let mut output = String::with_capacity(input.len());
    for (index, line) in input.lines().enumerate() {
        poll_parse_abort(abort, index)?;
        poll_parse_text(abort, line)?;
        let mut text = line.to_string();
        extractor.line(
            &mut text,
            &NetlistSourceLocation::in_memory(index + 1),
            index == 0,
        )?;
        output.push_str(&text);
        output.push('\n');
    }
    Ok((std::borrow::Cow::Owned(output), extractor.finish()?))
}

pub(super) fn prepare_expanded(
    mut expanded: ExpandedSource,
    retain: bool,
    abort: &dyn AbortSignal,
) -> Result<(ExpandedSource, Option<Arc<ControlScriptSource>>), ParseWithAbortError> {
    if !retain {
        return Ok((expanded, None));
    }
    let mut extractor = Extractor::new();
    let mut title_pending = expanded.implicit_title().is_none();
    if !title_pending {
        // The expanded parser owns an implicit title; the control interpreter
        // always expects an explicit first line, so supply only its placeholder.
        extractor.script.text.push_str("control script\n");
        extractor.line = 1;
    }
    for (index, item) in expanded.items.iter_mut().enumerate() {
        poll_parse_abort(abort, index)?;
        if let ExpandedSourceItem::Line { text, origin } = item {
            poll_parse_text(abort, text)?;
            extractor.line(text, origin, std::mem::take(&mut title_pending))?;
        }
    }
    Ok((expanded, extractor.finish()?))
}

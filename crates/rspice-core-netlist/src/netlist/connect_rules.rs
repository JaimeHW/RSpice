//! `.CONNECTRULES`: a Verilog-AMS `connectrules` block written in the deck.
//!
//! ```text
//! .CONNECTRULES run_rules
//! + connect a2d #(.vsup(1.8), .vthi(1.2), .vtlo(0.6));
//! + connect d2a #(.vsup(vdd), .trise(100p), .tfall(100p));
//! ```
//!
//! The body is the block's `connect` statements as LRM 2.4 section 7.7 writes
//! them, so it copies into a `.vams` `connectrules` block and back unchanged.
//! The engine compiles it with the same connect compiler that reads a design's
//! own blocks and the shipped rule sets, and `.OPTIONS CONNECTRULES` selects
//! it by name like any other block. It is how a run states its connect rules
//! without a Verilog-AMS file beside the deck (the Spectre AMS control file's
//! `ie` cards serve the same end).
//!
//! The body is Verilog-AMS text, not SPICE fields: a `;` ends a statement
//! rather than starting a comment, and `//` and `/* */` are its comments.
//! Each statement keeps the deck line it starts on, which is where the
//! converter report and every refusal about it point.

use super::NetlistSourceLocation;

/// One `.CONNECTRULES` block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckConnectRules {
    /// The block name, case-sensitive as Verilog-AMS names are.
    pub name: String,
    /// Its `connect` statements in the order written, each without its `;`.
    pub statements: Vec<DeckConnectStatement>,
    /// The `.CONNECTRULES` line.
    pub origin: NetlistSourceLocation,
}

/// One `connect` statement of a [`DeckConnectRules`] block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckConnectStatement {
    /// The statement as written, from `connect` to before its `;`, with its
    /// comments removed and its lines joined by single spaces.
    pub text: String,
    /// The line the statement starts on.
    pub origin: NetlistSourceLocation,
}

impl DeckConnectRules {
    /// The block as Verilog-AMS source, one statement per line: what the
    /// engine compiles. Each statement's byte range in that text is returned
    /// beside it, so a span the compiler reports maps back to its deck line.
    pub fn source(&self) -> (String, Vec<std::ops::Range<usize>>) {
        let mut source = format!("connectrules {};\n", self.name);
        let mut ranges = Vec::with_capacity(self.statements.len());
        for statement in &self.statements {
            source.push_str("    ");
            let start = source.len();
            source.push_str(&statement.text);
            source.push(';');
            ranges.push(start..source.len());
            source.push('\n');
        }
        source.push_str("endconnectrules\n");
        (source, ranges)
    }
}

/// A `.CONNECTRULES` card while its lines are read.
pub(crate) struct ConnectRulesCard {
    name: String,
    origin: NetlistSourceLocation,
    line: usize,
    statements: Vec<DeckConnectStatement>,
    /// The statement being read and the line it started on.
    pending: Option<(String, NetlistSourceLocation)>,
    in_block_comment: bool,
}

impl ConnectRulesCard {
    /// Open a card from its first line, `.CONNECTRULES name [statements]`.
    pub(crate) fn open(
        raw: &str,
        line: usize,
        origin: NetlistSourceLocation,
    ) -> Result<Self, String> {
        let after_head = raw
            .trim_start()
            .split_once(char::is_whitespace)
            .map_or("", |(_, rest)| rest)
            .trim_start();
        let name_end = after_head
            .find(|c: char| c.is_whitespace() || c == ';' || c == '/')
            .unwrap_or(after_head.len());
        let name = &after_head[..name_end];
        if name.is_empty() {
            return Err(
                ".CONNECTRULES needs a block name: .CONNECTRULES name, then one \
                        '+ connect ...;' line per statement"
                    .to_owned(),
            );
        }
        if !is_identifier(name) {
            return Err(format!(
                ".CONNECTRULES block name '{name}' is not a Verilog-AMS identifier (a letter or \
                 '_', then letters, digits, '_' or '$')"
            ));
        }
        let mut card = Self {
            name: name.to_owned(),
            origin: origin.clone(),
            line,
            statements: Vec::new(),
            pending: None,
            in_block_comment: false,
        };
        let rest = after_head[name_end..].trim_start();
        // The `;` a `.vams` block puts after its name.
        let rest = rest.strip_prefix(';').unwrap_or(rest);
        card.read(rest, origin)?;
        Ok(card)
    }

    /// Read one continuation line, its `+` removed.
    pub(crate) fn continue_with(
        &mut self,
        text: &str,
        origin: NetlistSourceLocation,
    ) -> Result<(), String> {
        self.read(text, origin)
    }

    fn read(&mut self, text: &str, origin: NetlistSourceLocation) -> Result<(), String> {
        let mut rest = text;
        while !rest.is_empty() {
            if self.in_block_comment {
                match rest.find("*/") {
                    Some(end) => {
                        self.in_block_comment = false;
                        rest = &rest[end + 2..];
                        continue;
                    }
                    None => return Ok(()),
                }
            }
            let next = rest.find([';', '/']);
            let Some(index) = next else {
                self.append(rest, &origin);
                return Ok(());
            };
            self.append(&rest[..index], &origin);
            let marker = &rest[index..];
            if marker.starts_with("//") {
                return Ok(());
            }
            if let Some(after) = marker.strip_prefix("/*") {
                self.in_block_comment = true;
                rest = after;
                continue;
            }
            if let Some(after) = marker.strip_prefix(';') {
                self.finish_statement(&origin)?;
                rest = after;
                continue;
            }
            // A lone `/` belongs to the statement (`.vsup(3.3/2)`).
            self.append("/", &origin);
            rest = &marker[1..];
        }
        Ok(())
    }

    fn append(&mut self, text: &str, origin: &NetlistSourceLocation) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }
        match &mut self.pending {
            Some((statement, _)) => {
                if !statement.ends_with('/') && !text.starts_with('/') {
                    statement.push(' ');
                }
                statement.push_str(text);
            }
            None => self.pending = Some((text.to_owned(), origin.clone())),
        }
    }

    fn finish_statement(&mut self, at: &NetlistSourceLocation) -> Result<(), String> {
        let Some((text, origin)) = self.pending.take() else {
            return Err(format!(
                ".CONNECTRULES {}: an empty statement (a ';' with nothing before it) at {at}",
                self.name
            ));
        };
        let keyword = text.split_whitespace().next().unwrap_or_default();
        if keyword != "connect" {
            return Err(format!(
                ".CONNECTRULES {} holds only connect statements; '{text}' at {origin} is not one",
                self.name
            ));
        }
        self.statements.push(DeckConnectStatement { text, origin });
        Ok(())
    }

    /// The `.CONNECTRULES` line, for a refusal about the card as a whole.
    pub(crate) fn line(&self) -> usize {
        self.line
    }

    /// Close the card: every statement ended with its `;`, and there is at
    /// least one.
    pub(crate) fn finish(self) -> Result<DeckConnectRules, String> {
        if let Some((text, origin)) = &self.pending {
            return Err(format!(
                ".CONNECTRULES {}: the statement '{text}' at {origin} does not end with ';'",
                self.name
            ));
        }
        if self.in_block_comment {
            return Err(format!(
                ".CONNECTRULES {}: a '/*' comment is not closed before the card ends",
                self.name
            ));
        }
        if self.statements.is_empty() {
            return Err(format!(
                ".CONNECTRULES {} has no connect statements; write one '+ connect ...;' line per \
                 statement",
                self.name
            ));
        }
        Ok(DeckConnectRules {
            name: self.name,
            statements: self.statements,
            origin: self.origin,
        })
    }
}

/// A Verilog-AMS simple identifier (IEEE 1364-2005 section 3.7).
fn is_identifier(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(line: usize) -> NetlistSourceLocation {
        NetlistSourceLocation::in_memory(line)
    }

    fn card(lines: &[&str]) -> Result<DeckConnectRules, String> {
        let mut card = ConnectRulesCard::open(lines[0], 1, at(1))?;
        for (index, line) in lines.iter().enumerate().skip(1) {
            card.continue_with(line, at(index + 1))?;
        }
        card.finish()
    }

    #[test]
    fn statements_keep_their_text_and_the_line_they_start_on() {
        let block = card(&[
            ".CONNECTRULES run_rules",
            " connect a2d #(.vsup(1.8), .vthi(1.2)); // the receivers",
            " connect d2a #(.vsup(vdd),",
            "   .trise(100p)); /* the drivers */ connect bidir;",
        ])
        .expect("the block reads");
        assert_eq!(block.name, "run_rules");
        let read: Vec<(&str, usize)> = block
            .statements
            .iter()
            .map(|statement| (statement.text.as_str(), statement.origin.line))
            .collect();
        assert_eq!(
            read,
            [
                ("connect a2d #(.vsup(1.8), .vthi(1.2))", 2),
                ("connect d2a #(.vsup(vdd), .trise(100p))", 3),
                ("connect bidir", 4),
            ]
        );
        let (source, ranges) = block.source();
        assert_eq!(
            source,
            "connectrules run_rules;\n    connect a2d #(.vsup(1.8), .vthi(1.2));\n    connect d2a \
             #(.vsup(vdd), .trise(100p));\n    connect bidir;\nendconnectrules\n"
        );
        assert_eq!(&source[ranges[2].clone()], "connect bidir;");
    }

    #[test]
    fn a_division_inside_an_override_is_part_of_the_statement() {
        let block = card(&[".CONNECTRULES half", " connect a2d #(.vthi(3.3/2));"])
            .expect("the block reads");
        assert_eq!(block.statements[0].text, "connect a2d #(.vthi(3.3/2))");
    }

    /// In a deck the body is Verilog-AMS: its `;` ends a statement instead of
    /// starting a SPICE comment, and the card ends at the next card.
    #[test]
    fn a_deck_card_reads_its_statements_and_ends_at_the_next_card() {
        let netlist = crate::netlist::Netlist::parse(
            "* deck\n.connectrules run_rules\n+ connect a2d #(.vthi(1.2)); connect d2a;\n* note\n\
             + connect bidir #(.trise(1n));\nr1 a 0 1k\n.if (0)\n.connectrules off\n+ connect \
             a2d;\n.endif\n.end\n",
        )
        .expect("the deck parses");
        let [block] = netlist.connect_rules.as_slice() else {
            panic!("one active block: {:?}", netlist.connect_rules);
        };
        assert_eq!(block.name, "run_rules");
        assert_eq!(block.origin.line, 2);
        let read: Vec<(&str, usize)> = block
            .statements
            .iter()
            .map(|statement| (statement.text.as_str(), statement.origin.line))
            .collect();
        assert_eq!(
            read,
            [
                ("connect a2d #(.vthi(1.2))", 3),
                ("connect d2a", 3),
                ("connect bidir #(.trise(1n))", 5),
            ]
        );
        assert_eq!(netlist.elements.len(), 1, "r1 is a card of its own");
    }

    #[test]
    fn a_deck_names_a_block_once_and_only_at_deck_level() {
        let twice = crate::netlist::Netlist::parse(
            "* deck\n.connectrules r\n+ connect a2d;\n.connectrules r\n+ connect d2a;\n.end\n",
        )
        .expect_err("a second block of one name is refused");
        let twice = twice.to_string();
        assert!(
            twice.contains("line 4") && twice.contains("already written at line 2"),
            "{twice}"
        );
        let nested = crate::netlist::Netlist::parse(
            "* deck\n.subckt cell a\n.connectrules r\n+ connect a2d;\n.ends\n.end\n",
        )
        .expect_err("a block inside a subcircuit is refused");
        assert!(nested.to_string().contains("deck-level card"), "{nested}");
    }

    #[test]
    fn a_malformed_card_is_refused_naming_what_is_wrong() {
        for (lines, expected) in [
            (&[".CONNECTRULES"][..], "needs a block name"),
            (
                &[".CONNECTRULES 9rules", " connect a2d;"][..],
                "not a Verilog-AMS identifier",
            ),
            (&[".CONNECTRULES r"][..], "has no connect statements"),
            (
                &[".CONNECTRULES r", " connect a2d"][..],
                "does not end with ';'",
            ),
            (
                &[".CONNECTRULES r", " endconnectrules;"][..],
                "only connect statements",
            ),
            (
                &[".CONNECTRULES r", " connect a2d;;"][..],
                "an empty statement",
            ),
            (
                &[".CONNECTRULES r", " connect a2d; /* open"][..],
                "not closed",
            ),
        ] {
            let error = card(lines).expect_err(expected);
            assert!(error.contains(expected), "{lines:?}: {error}");
        }
    }
}

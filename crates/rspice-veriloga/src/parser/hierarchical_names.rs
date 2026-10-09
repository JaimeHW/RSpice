//! Retain hierarchy as structured paths instead of treating dots as identifier text.
use super::*;

impl Parser<'_> {
    pub(super) fn is_root_reference(&self) -> bool {
        self.check(TokenKind::SystemIdentifier) && self.current().text.as_deref() == Some("$root")
    }

    pub(super) fn parse_reference_endpoint(
        &mut self,
        context: &str,
        allow_branch: bool,
    ) -> Result<SmolStr, ParseError> {
        if self.check(TokenKind::IntegerLiteral) {
            return Ok(self.expect_branch_endpoint(context)?.into());
        }
        self.parse_reference_path(context, allow_branch)
    }

    /// Consume only scope selectors. A selector on the final name belongs to the
    /// expression/lvalue grammar and is left for its caller.
    pub(super) fn parse_reference_name(&mut self, context: &str) -> Result<SmolStr, ParseError> {
        self.parse_reference_path(context, true)
    }

    fn parse_reference_path(
        &mut self,
        context: &str,
        allow_branch: bool,
    ) -> Result<SmolStr, ParseError> {
        let start = self.current_span();
        let absolute = self.is_root_reference();
        if absolute {
            self.advance();
            self.expect(TokenKind::Dot)?;
        }
        let mut segments = Vec::new();
        let mut branch = None;
        loop {
            let span = self.current_span();
            if !segments.is_empty()
                && self.check(TokenKind::Identifier)
                && self.current().text.as_deref() == Some("branch")
                && self.peek_is(TokenKind::LParen)
            {
                if !allow_branch {
                    return Err(self.error(ParseErrorKind::InvalidExpression));
                }
                self.advance();
                self.expect(TokenKind::LParen)?;
                let is_port = self.match_token(TokenKind::Lt);
                let pos = self.parse_hierarchical_branch_terminal()?;
                let neg = if is_port {
                    self.expect(TokenKind::Gt)?;
                    None
                } else if self.match_token(TokenKind::Comma) {
                    Some(self.parse_hierarchical_branch_terminal()?)
                } else {
                    None
                };
                self.expect(TokenKind::RParen)?;
                branch = Some(Box::new(HierarchicalBranch { pos, neg, is_port }));
                segments.push(HierarchicalSegment {
                    name: "branch".into(),
                    index: None,
                    span,
                });
                break;
            }
            let name = self.expect_identifier(context)?.into();
            let index = if self.scope_selector_follows() {
                self.expect(TokenKind::LBracket)?;
                let value = self.parse_expression()?;
                self.expect(TokenKind::RBracket)?;
                Some(value)
            } else {
                None
            };
            segments.push(HierarchicalSegment { name, index, span });
            if !self.match_token(TokenKind::Dot) {
                break;
            }
        }
        if !absolute && segments.len() == 1 {
            return Ok(segments.pop().unwrap().name);
        }
        let span = start.extend(self.previous_span());
        // Branch-access lookahead can rewind the parser. Reuse the source record
        // rather than retaining an orphan reference from the abandoned parse.
        if let Some(symbol) = self.reference_symbols.get(&span) {
            return Ok(symbol.clone());
        }
        let symbol = loop {
            let candidate: SmolStr = format!("$rspice_reference_{}", self.next_reference_id).into();
            self.next_reference_id += 1;
            if !self.authored_identifiers.contains(&candidate)
                && self.reserved_names.insert(candidate.clone())
            {
                break candidate;
            }
        };
        self.reference_symbols.insert(span, symbol.clone());
        self.hierarchical_names.insert(
            symbol.clone(),
            HierarchicalName {
                absolute,
                segments,
                branch,
                span,
            },
        );
        Ok(symbol)
    }

    fn parse_hierarchical_branch_terminal(
        &mut self,
    ) -> Result<HierarchicalBranchTerminal, ParseError> {
        let span = self.current_span();
        let (name, prefix, select) = self.parse_branch_terminal()?;
        let name = self
            .hierarchical_names
            .get(&name)
            .cloned()
            .unwrap_or_else(|| HierarchicalName {
                absolute: false,
                segments: vec![HierarchicalSegment {
                    name,
                    index: None,
                    span,
                }],
                branch: None,
                span,
            });
        if name.branch.is_some() {
            return Err(self.error(ParseErrorKind::InvalidExpression));
        }
        Ok(HierarchicalBranchTerminal {
            name,
            prefix,
            select,
        })
    }

    fn scope_selector_follows(&self) -> bool {
        if !self.check(TokenKind::LBracket) {
            return false;
        }
        let mut depth = 0usize;
        for (offset, token) in self.tokens[self.pos..].iter().enumerate() {
            match token.kind {
                TokenKind::LBracket => depth += 1,
                TokenKind::RBracket => {
                    depth -= 1;
                    if depth == 0 {
                        return self
                            .tokens
                            .get(self.pos + offset + 1)
                            .is_some_and(|token| token.kind == TokenKind::Dot);
                    }
                }
                TokenKind::Eof => break,
                _ => {}
            }
        }
        false
    }
}

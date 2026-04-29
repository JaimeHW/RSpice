use std::collections::HashMap;
use std::path::PathBuf;

use super::ParsedLibrary;
use super::lexer::{Lexer, Token};
use super::types::{
    IncludeDirective, IncludeType, LibrarySection, ModelDef, ParamValue, SubcircuitDef,
};

// =============================================================================
// Parser
// =============================================================================

/// Parser for SPICE library files
pub struct LibraryParser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    peeked: Option<Token>,
}

impl<'a> LibraryParser<'a> {
    /// Create a new parser
    pub fn new(input: &'a str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self {
            lexer,
            current,
            peeked: None,
        })
    }

    /// Advance to next token
    fn advance(&mut self) -> Result<Token, String> {
        let old = std::mem::replace(
            &mut self.current,
            if let Some(t) = self.peeked.take() {
                t
            } else {
                self.lexer.next_token()?
            },
        );
        Ok(old)
    }

    /// Skip newlines
    fn skip_newlines(&mut self) -> Result<(), String> {
        while self.current == Token::Newline {
            self.advance()?;
        }
        Ok(())
    }

    /// Parse the entire library
    pub fn parse(&mut self) -> Result<ParsedLibrary, String> {
        let mut library = ParsedLibrary::default();
        self.skip_newlines()?;

        while self.current != Token::Eof {
            match &self.current {
                Token::Keyword(kw) if kw == "library" => {
                    self.advance()?;
                    // Skip library name
                    if let Token::Identifier(name) = &self.current {
                        library.name = name.clone();
                        self.advance()?;
                    }
                }
                Token::Keyword(kw) if kw == "section" => {
                    let section = self.parse_section()?;
                    library.sections.insert(section.name.clone(), section);
                }
                Token::Keyword(kw) if kw == "model" => {
                    let model = self.parse_model()?;
                    library.global_models.insert(model.name.clone(), model);
                }
                Token::Keyword(kw) if kw == "subckt" => {
                    let subckt = self.parse_subcircuit()?;
                    library
                        .global_subcircuits
                        .insert(subckt.name.clone(), subckt);
                }
                Token::Keyword(kw) if kw == "param" || kw == "parameters" => {
                    let params = self.parse_parameters()?;
                    library.global_parameters.extend(params);
                }
                Token::Keyword(kw) if kw == "include" || kw == "lib" => {
                    let include = self.parse_include()?;
                    library.includes.push(include);
                }
                Token::Keyword(kw) if kw == "endlibrary" => {
                    self.advance()?;
                }
                Token::Newline => {
                    self.advance()?;
                }
                _ => {
                    // Skip unknown content
                    self.advance()?;
                }
            }
        }

        Ok(library)
    }

    /// Parse a section
    fn parse_section(&mut self) -> Result<LibrarySection, String> {
        self.advance()?; // Skip 'section'
        self.skip_newlines()?;

        let mut section = LibrarySection::default();

        // Get section name
        if let Token::Identifier(name) = &self.current {
            section.name = name.clone();
            self.advance()?;
        } else if let Token::Keyword(name) = &self.current {
            section.name = name.clone();
            self.advance()?;
        }

        self.skip_newlines()?;

        // Parse section content
        while self.current != Token::Eof {
            match &self.current {
                Token::Keyword(kw) if kw == "endsection" => {
                    self.advance()?;
                    break;
                }
                Token::Keyword(kw) if kw == "model" => {
                    let model = self.parse_model()?;
                    section.models.insert(model.name.clone(), model);
                }
                Token::Keyword(kw) if kw == "subckt" => {
                    let subckt = self.parse_subcircuit()?;
                    section.subcircuits.insert(subckt.name.clone(), subckt);
                }
                Token::Keyword(kw) if kw == "param" || kw == "parameters" => {
                    let params = self.parse_parameters()?;
                    section.parameters.extend(params);
                }
                Token::Keyword(kw) if kw == "include" || kw == "lib" => {
                    let include = self.parse_include()?;
                    section.includes.push(include);
                }
                Token::Newline => {
                    self.advance()?;
                }
                _ => {
                    self.advance()?;
                }
            }
        }

        Ok(section)
    }

    /// Parse a model definition
    fn parse_model(&mut self) -> Result<ModelDef, String> {
        let line = self.lexer.position().0;
        self.advance()?; // Skip 'model'

        let mut model = ModelDef {
            line,
            ..Default::default()
        };

        // Model name
        if let Token::Identifier(name) = &self.current {
            model.name = name.clone();
            self.advance()?;
        }

        // Model type
        if let Token::Identifier(mtype) = &self.current {
            model.model_type = mtype.clone();
            self.advance()?;
        }

        // Parameters (continue until newline or paren close)
        self.parse_model_params(&mut model)?;

        Ok(model)
    }

    /// Parse model parameters
    fn parse_model_params(&mut self, model: &mut ModelDef) -> Result<(), String> {
        while self.current != Token::Newline && self.current != Token::Eof {
            match &self.current {
                Token::Identifier(name) => {
                    let param_name = name.clone();
                    self.advance()?;

                    if self.current == Token::Operator('=') {
                        self.advance()?;

                        // Handle special parameters
                        if param_name.to_lowercase() == "level" {
                            if let Token::Number(n) = &self.current {
                                model.level = Some(*n as i32);
                            }
                        } else if param_name.to_lowercase() == "version" {
                            if let Token::Number(n) = &self.current {
                                model.version = Some(n.to_string());
                            } else if let Token::String(s) = &self.current {
                                model.version = Some(s.clone());
                            }
                        }

                        let value = self.parse_param_value()?;
                        model.parameters.insert(param_name, value);
                    }
                }
                Token::LParen => {
                    self.advance()?;
                }
                Token::RParen => {
                    self.advance()?;
                }
                _ => {
                    self.advance()?;
                }
            }
        }

        Ok(())
    }

    /// Parse subcircuit definition
    fn parse_subcircuit(&mut self) -> Result<SubcircuitDef, String> {
        let line = self.lexer.position().0;
        self.advance()?; // Skip 'subckt'

        let mut subckt = SubcircuitDef {
            line,
            ..Default::default()
        };

        // Subcircuit name
        if let Token::Identifier(name) = &self.current {
            subckt.name = name.clone();
            self.advance()?;
        }

        // Ports (until = or newline)
        while self.current != Token::Newline && self.current != Token::Eof {
            match &self.current {
                Token::Identifier(port) => {
                    if self.peeked.is_none() {
                        // Peek next
                        let next = self.lexer.next_token()?;
                        if next == Token::Operator('=') {
                            // This is a parameter, not a port
                            self.peeked = Some(next);
                            break;
                        }
                        self.peeked = Some(next);
                    }
                    subckt.ports.push(port.clone());
                    self.advance()?;
                }
                Token::LParen => {
                    self.advance()?;
                }
                Token::RParen => {
                    self.advance()?;
                }
                _ => {
                    break;
                }
            }
        }

        // Skip to .ends
        let mut depth = 1;
        while self.current != Token::Eof && depth > 0 {
            match &self.current {
                Token::Keyword(kw) if kw == "subckt" => {
                    depth += 1;
                    self.advance()?;
                }
                Token::Keyword(kw) if kw == "ends" => {
                    depth -= 1;
                    self.advance()?;
                }
                _ => {
                    self.advance()?;
                }
            }
        }

        Ok(subckt)
    }

    /// Parse parameter definitions
    fn parse_parameters(&mut self) -> Result<HashMap<String, ParamValue>, String> {
        self.advance()?; // Skip 'param' or 'parameters'
        let mut params = HashMap::new();

        while self.current != Token::Newline && self.current != Token::Eof {
            if let Token::Identifier(name) = &self.current {
                let param_name = name.clone();
                self.advance()?;

                if self.current == Token::Operator('=') {
                    self.advance()?;
                    let value = self.parse_param_value()?;
                    params.insert(param_name, value);
                }
            } else {
                self.advance()?;
            }
        }

        Ok(params)
    }

    /// Parse a parameter value
    fn parse_param_value(&mut self) -> Result<ParamValue, String> {
        match &self.current {
            Token::Number(n) => {
                let val = *n;
                self.advance()?;
                Ok(ParamValue::Number(val))
            }
            Token::Identifier(s) | Token::String(s) => {
                let val = s.clone();
                self.advance()?;
                Ok(ParamValue::Expression(val))
            }
            _ => {
                self.advance()?;
                Ok(ParamValue::Expression(String::new()))
            }
        }
    }

    /// Parse include directive
    fn parse_include(&mut self) -> Result<IncludeDirective, String> {
        let line = self.lexer.position().0;
        let directive_type = if let Token::Keyword(kw) = &self.current {
            if kw == "lib" {
                IncludeType::Lib
            } else {
                IncludeType::Include
            }
        } else {
            IncludeType::Include
        };

        self.advance()?;

        let mut path = PathBuf::new();
        let mut section = None;

        // Get path
        match &self.current {
            Token::String(s) => {
                path = PathBuf::from(s);
                self.advance()?;
            }
            Token::Identifier(s) => {
                path = PathBuf::from(s);
                self.advance()?;
            }
            _ => {}
        }

        // Get section for .lib
        if directive_type == IncludeType::Lib
            && let Token::Identifier(s) = &self.current
        {
            section = Some(s.clone());
            self.advance()?;
        }

        Ok(IncludeDirective {
            directive_type,
            path,
            section,
            line,
        })
    }
}

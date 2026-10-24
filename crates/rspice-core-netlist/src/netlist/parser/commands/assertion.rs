//! `.ASSERT <name> KEY=value ...`: RSpice's assertion card, with Spectre's
//! parameter names (see [`crate::netlist::AssertStatement`]).
//!
//! Every keyword binds once. A card checks exactly one thing: `EXPR=`,
//! `COND=`, or one device target with `PARAM=`. Three spellings are refused,
//! each in its own sentence: `LEVEL=FATAL` (an assertion is evaluated after
//! the run, so it cannot stop one), `MODELPARAM=` (model-card range checks),
//! and a `PARAM=` no device reports.

use super::super::analysis_card_scan::{at_card_end, at_keyword, take_keyword};
use super::{expect_measure_value, skip_commas};
use crate::Value;
use crate::netlist::lexer::TokenKind;
use crate::netlist::{
    AssertAnalysis, AssertCheck, AssertLevel, AssertStatement, AssertTarget, NetlistSourceLocation,
    ParamContext, ParseError, VectorFileReference,
};

use super::super::stream::TokenStream;

fn syntax(line: usize, message: impl Into<String>) -> ParseError {
    ParseError::Syntax {
        line,
        message: message.into(),
    }
}

/// The text of a quoted, braced or bare value: `EXPR='V(out)'`,
/// `MESSAGE="rail low"`, `MESSAGE=low`.
fn text_value(stream: &mut TokenStream, line: usize, keyword: &str) -> Result<String, ParseError> {
    let text = match &stream.peek().kind {
        TokenKind::Expression(text) | TokenKind::StringLit(text) => text.clone(),
        TokenKind::Ident(_) | TokenKind::Number(_) => stream.peek().lexeme.clone(),
        _ => {
            return Err(syntax(
                line,
                format!(".ASSERT {keyword}= expects a quoted value"),
            ));
        }
    };
    stream.advance();
    Ok(text)
}

/// A word value (`LEVEL=ERROR`, `PARAM=vds`, `DEV=x1.mp`), as written: the
/// source-contiguous tokens up to the next blank, comma or `=`.
fn word_value(stream: &mut TokenStream, line: usize, keyword: &str) -> Result<String, ParseError> {
    match &stream.peek().kind {
        TokenKind::Ident(_) | TokenKind::Number(_) => {
            let first = stream.peek().clone();
            let mut word = first.lexeme;
            let mut end = first.span.end;
            stream.advance();
            loop {
                let next = stream.peek();
                if next.span.line != first.span.line
                    || next.span.start != end
                    || matches!(
                        next.kind,
                        TokenKind::Newline | TokenKind::Eof | TokenKind::Comma | TokenKind::Equals
                    )
                {
                    break;
                }
                word.push_str(&next.lexeme);
                end = next.span.end;
                stream.advance();
            }
            Ok(word)
        }
        TokenKind::Expression(text) | TokenKind::StringLit(text) => {
            let text = text.trim().to_string();
            stream.advance();
            Ok(text)
        }
        _ => Err(syntax(line, format!(".ASSERT {keyword}= expects a name"))),
    }
}

/// A finite number (`MIN=-0.6`, `FROM=10n`, `MAX={vdd/2}`).
fn number_value(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    keyword: &str,
) -> Result<Value, ParseError> {
    let value = expect_measure_value(stream, line, params)
        .map_err(|error| syntax(line, format!(".ASSERT {keyword}= {error}")))?;
    if !value.is_finite() {
        return Err(syntax(
            line,
            format!(".ASSERT {keyword} must be finite, found {value}"),
        ));
    }
    Ok(value)
}

fn bind<T>(slot: &mut Option<T>, value: T, line: usize, keyword: &str) -> Result<(), ParseError> {
    if slot.is_some() {
        return Err(syntax(
            line,
            format!(".ASSERT binds {keyword}= more than once"),
        ));
    }
    *slot = Some(value);
    Ok(())
}

/// `ANALYSES=op,dc,tran`: one or more analysis names, comma-separated. A
/// comma before the next `KEY=` separates keywords, as everywhere in SPICE.
fn analyses_value(
    stream: &mut TokenStream,
    line: usize,
) -> Result<Vec<AssertAnalysis>, ParseError> {
    let mut analyses = Vec::new();
    loop {
        if !matches!(stream.peek().kind, TokenKind::Ident(_)) {
            return Err(syntax(
                line,
                ".ASSERT ANALYSES= expects op, dc or tran, comma-separated",
            ));
        }
        let word = stream.peek().lexeme.clone();
        let analysis = match word.to_ascii_lowercase().as_str() {
            "op" => AssertAnalysis::Op,
            "dc" => AssertAnalysis::Dc,
            "tran" => AssertAnalysis::Tran,
            _ => {
                return Err(syntax(
                    line,
                    format!(
                        ".ASSERT ANALYSES= names '{word}'; an assertion is evaluated after op, dc \
                         or tran"
                    ),
                ));
            }
        };
        stream.advance();
        if !analyses.contains(&analysis) {
            analyses.push(analysis);
        }
        if !matches!(stream.peek().kind, TokenKind::Comma)
            || !matches!(stream.peek_n(1).kind, TokenKind::Ident(_))
            || matches!(stream.peek_n(2).kind, TokenKind::Equals)
        {
            return Ok(analyses);
        }
        stream.advance();
    }
}

/// Whether some device reports an operating-point quantity of this name: the
/// fixed vocabulary every device family reports in.
fn is_op_quantity(parameter: &str) -> bool {
    crate::op_label::declared_op_label(parameter).is_some()
}

#[derive(Default)]
struct Keywords {
    expression: Option<String>,
    condition: Option<String>,
    target: Option<(&'static str, AssertTarget)>,
    parameter: Option<String>,
    min: Option<Value>,
    max: Option<Value>,
    from: Option<Value>,
    to: Option<Value>,
    duration: Option<Value>,
    level: Option<AssertLevel>,
    message: Option<String>,
    analyses: Option<Vec<AssertAnalysis>>,
    ignore_unknown: Option<bool>,
}

/// Parse the rest of an `.ASSERT` card after the command word.
pub(in crate::netlist::parser) fn parse_assert_command(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    origin: &NetlistSourceLocation,
) -> Result<AssertStatement, ParseError> {
    skip_commas(stream);
    if at_card_end(stream) || at_keyword(stream) {
        return Err(syntax(
            line,
            ".ASSERT requires a name before its parameters",
        ));
    }
    let name = word_value(stream, line, "name")
        .map_err(|_| syntax(line, ".ASSERT requires a name before its parameters"))?;
    let mut keywords = Keywords::default();
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            return Err(syntax(
                line,
                format!(
                    ".ASSERT {name}: expected KEY=value, found '{}'",
                    stream.peek().lexeme
                ),
            ));
        };
        match keyword.as_str() {
            "EXPR" => {
                let text = text_value(stream, line, "EXPR")?;
                bind(&mut keywords.expression, text, line, "EXPR")?;
            }
            "COND" => {
                let text = text_value(stream, line, "COND")?;
                bind(&mut keywords.condition, text, line, "COND")?;
            }
            "DEV" | "MOD" | "SUB" | "PRIMITIVE" => {
                let value = word_value(stream, line, &keyword)?;
                let (key, target) = match keyword.as_str() {
                    "DEV" => ("DEV", AssertTarget::Instance(value)),
                    "MOD" => ("MOD", AssertTarget::Model(value)),
                    "SUB" => ("SUB", AssertTarget::Subcircuit(value)),
                    _ => ("PRIMITIVE", AssertTarget::Primitive(value)),
                };
                if let Some((bound, _)) = &keywords.target {
                    return Err(syntax(
                        line,
                        format!(
                            ".ASSERT {name} names devices twice ({bound}= and {key}=); a card \
                             checks one target"
                        ),
                    ));
                }
                keywords.target = Some((key, target));
            }
            "PARAM" => {
                let parameter = word_value(stream, line, "PARAM")?.to_ascii_lowercase();
                if !is_op_quantity(&parameter) {
                    return Err(syntax(
                        line,
                        format!(
                            ".ASSERT {name}: PARAM={parameter} is not an operating-point \
                             quantity any device reports (the names @inst[param] reads, such as \
                             vgs, vds, id or gm)"
                        ),
                    ));
                }
                bind(&mut keywords.parameter, parameter, line, "PARAM")?;
            }
            "MODELPARAM" => {
                return Err(ParseError::UnsupportedCapability {
                    origin: origin.clone(),
                    capability: "netlist.assert.modelparam",
                    detail: format!(
                        ".ASSERT {name}: MODELPARAM= range checks on model-card parameters are \
                         not supported; check an operating-point quantity with PARAM= instead"
                    ),
                });
            }
            "MIN" => {
                let value = number_value(stream, line, params, "MIN")?;
                bind(&mut keywords.min, value, line, "MIN")?;
            }
            "MAX" => {
                let value = number_value(stream, line, params, "MAX")?;
                bind(&mut keywords.max, value, line, "MAX")?;
            }
            "FROM" => {
                let value = number_value(stream, line, params, "FROM")?;
                bind(&mut keywords.from, value, line, "FROM")?;
            }
            "TO" => {
                let value = number_value(stream, line, params, "TO")?;
                bind(&mut keywords.to, value, line, "TO")?;
            }
            "DURATION" => {
                let value = number_value(stream, line, params, "DURATION")?;
                if value < 0.0 {
                    return Err(syntax(
                        line,
                        format!(".ASSERT {name}: DURATION must not be negative, found {value}"),
                    ));
                }
                bind(&mut keywords.duration, value, line, "DURATION")?;
            }
            "LEVEL" => {
                let level = level_value(stream, line, &format!(".ASSERT {name}"), origin)?;
                bind(&mut keywords.level, level, line, "LEVEL")?;
            }
            "MESSAGE" => {
                let text = text_value(stream, line, "MESSAGE")?;
                bind(&mut keywords.message, text, line, "MESSAGE")?;
            }
            "ANALYSES" => {
                let analyses = analyses_value(stream, line)?;
                bind(&mut keywords.analyses, analyses, line, "ANALYSES")?;
            }
            "XZ" => {
                let word = word_value(stream, line, "XZ")?;
                let ignore = match word.to_ascii_uppercase().as_str() {
                    "FAIL" => false,
                    "IGNORE" => true,
                    _ => {
                        return Err(syntax(
                            line,
                            format!(".ASSERT {name}: XZ={word} is not a mode; use FAIL or IGNORE"),
                        ));
                    }
                };
                bind(&mut keywords.ignore_unknown, ignore, line, "XZ")?;
            }
            other => {
                return Err(syntax(
                    line,
                    format!(
                        ".ASSERT {name}: {other}= is not an assertion parameter (EXPR, COND, DEV, \
                         MOD, SUB, PRIMITIVE, PARAM, MIN, MAX, FROM, TO, DURATION, LEVEL, \
                         MESSAGE, ANALYSES, XZ)"
                    ),
                ));
            }
        }
    }
    finish(name, keywords, line, origin)
}

/// `LEVEL=NOTICE|WARNING|ERROR`; `FATAL` is refused in its own sentence.
fn level_value(
    stream: &mut TokenStream,
    line: usize,
    card: &str,
    origin: &NetlistSourceLocation,
) -> Result<AssertLevel, ParseError> {
    let word = word_value(stream, line, "LEVEL")?;
    match word.to_ascii_uppercase().as_str() {
        "NOTICE" => Ok(AssertLevel::Notice),
        "WARNING" => Ok(AssertLevel::Warning),
        "ERROR" => Ok(AssertLevel::Error),
        "FATAL" => Err(ParseError::UnsupportedCapability {
            origin: origin.clone(),
            capability: "netlist.assert.level_fatal",
            detail: format!(
                "{card}: LEVEL=FATAL would stop the run at the violation, but an assertion is \
                 evaluated after the run; use LEVEL=ERROR, which fails the run's verdict and \
                 keeps its results"
            ),
        }),
        _ => Err(syntax(
            line,
            format!("{card}: LEVEL={word} is not a level; use NOTICE, WARNING or ERROR"),
        )),
    }
}

/// `.VEC '<file>' [LEVEL=NOTICE|WARNING|ERROR]`: HSPICE's digital vector
/// file card. HSPICE's card takes the file alone; `LEVEL=` is RSpice's, the
/// level the file's output mismatches are reported at, ERROR by default.
pub(in crate::netlist::parser) fn parse_vec_command(
    stream: &mut TokenStream,
    line: usize,
    origin: &NetlistSourceLocation,
) -> Result<VectorFileReference, ParseError> {
    skip_commas(stream);
    if at_card_end(stream) || at_keyword(stream) {
        return Err(syntax(line, ".VEC requires the vector file's name"));
    }
    let path = match super::super::command_parsers::quoted_path_lexeme(stream) {
        Some(path) => path,
        None => word_value(stream, line, "file")
            .map_err(|_| syntax(line, ".VEC requires the vector file's name"))?,
    };
    if path.trim().is_empty() {
        return Err(syntax(line, ".VEC requires the vector file's name"));
    }
    let mut level = None;
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        match take_keyword(stream).as_deref() {
            Some("LEVEL") => {
                let value = level_value(stream, line, &format!(".VEC '{path}'"), origin)?;
                bind(&mut level, value, line, "LEVEL")?;
            }
            Some(other) => {
                return Err(syntax(
                    line,
                    format!(".VEC '{path}': {other}= is not a .VEC parameter (LEVEL)"),
                ));
            }
            None => {
                return Err(syntax(
                    line,
                    format!(
                        ".VEC '{path}': expected LEVEL=value after the file, found '{}'",
                        stream.peek().lexeme
                    ),
                ));
            }
        }
    }
    Ok(VectorFileReference {
        path,
        level: level.unwrap_or(AssertLevel::Error),
        origin: origin.clone(),
    })
}

fn finish(
    name: String,
    keywords: Keywords,
    line: usize,
    origin: &NetlistSourceLocation,
) -> Result<AssertStatement, ParseError> {
    let Keywords {
        expression,
        condition,
        target,
        parameter,
        min,
        max,
        from,
        to,
        duration,
        level,
        message,
        analyses,
        ignore_unknown,
    } = keywords;
    if parameter.is_some() && target.is_none() {
        return Err(syntax(
            line,
            format!(
                ".ASSERT {name}: PARAM= needs a device target (DEV=, MOD=, SUB= or PRIMITIVE=)"
            ),
        ));
    }
    let forms = usize::from(expression.is_some())
        + usize::from(condition.is_some())
        + usize::from(target.is_some());
    if forms != 1 {
        return Err(syntax(
            line,
            format!(
                ".ASSERT {name} must check exactly one thing: EXPR=, COND=, or a device target \
                 (DEV=, MOD=, SUB= or PRIMITIVE=) with PARAM="
            ),
        ));
    }
    let bounded = min.is_some() || max.is_some();
    if let (Some(min), Some(max)) = (min, max)
        && min > max
    {
        return Err(syntax(
            line,
            format!(".ASSERT {name}: MIN={min} is above MAX={max}"),
        ));
    }
    let check = if let Some(expression) = expression {
        check_expression(&name, "EXPR", &expression, line)?;
        if !bounded {
            return Err(syntax(
                line,
                format!(".ASSERT {name}: EXPR= needs MIN=, MAX= or both"),
            ));
        }
        AssertCheck::Expression {
            expression,
            min,
            max,
        }
    } else if let Some(condition) = condition {
        check_expression(&name, "COND", &condition, line)?;
        if bounded {
            return Err(syntax(
                line,
                format!(
                    ".ASSERT {name}: COND= is a condition; MIN= and MAX= bound EXPR= or PARAM="
                ),
            ));
        }
        AssertCheck::Condition { condition }
    } else {
        let Some((key, target)) = target else {
            unreachable!("exactly one form is bound");
        };
        let Some(parameter) = parameter else {
            return Err(syntax(
                line,
                format!(
                    ".ASSERT {name}: {key}= needs PARAM=, the operating-point quantity to check"
                ),
            ));
        };
        if !bounded {
            return Err(syntax(
                line,
                format!(".ASSERT {name}: PARAM={parameter} needs MIN=, MAX= or both"),
            ));
        }
        AssertCheck::Device {
            target,
            parameter,
            min,
            max,
        }
    };
    if let (Some(from), Some(to)) = (from, to)
        && from > to
    {
        return Err(syntax(
            line,
            format!(".ASSERT {name}: FROM={from} is after TO={to}"),
        ));
    }
    Ok(AssertStatement {
        name,
        check,
        from,
        to,
        duration,
        level: level.unwrap_or_default(),
        message,
        analyses: analyses.unwrap_or_default(),
        ignore_unknown: ignore_unknown.unwrap_or(false),
        origin: origin.clone(),
    })
}

/// Refuse an expression the evaluator could never read.
fn check_expression(name: &str, keyword: &str, text: &str, line: usize) -> Result<(), ParseError> {
    if text.trim().is_empty() {
        return Err(syntax(line, format!(".ASSERT {name}: {keyword}= is empty")));
    }
    crate::netlist::expr::parse_expression(text).map_err(|error| {
        syntax(
            line,
            format!(".ASSERT {name}: {keyword}='{text}' is not an expression: {error}"),
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::netlist::{
        AssertAnalysis, AssertCheck, AssertLevel, AssertTarget, Netlist, ParseError,
    };

    fn deck(card: &str) -> String {
        format!("assert\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n{card}\n.tran 1n 1u\n.end\n")
    }

    fn parse(card: &str) -> Netlist {
        Netlist::parse(&deck(card)).unwrap_or_else(|error| panic!("{card}: {error}"))
    }

    fn refusal(card: &str) -> ParseError {
        match Netlist::parse(&deck(card)) {
            Ok(_) => panic!("{card} parsed"),
            Err(error) => error,
        }
    }

    #[test]
    fn every_form_and_option_is_read() {
        let netlist = parse(
            ".ASSERT rail EXPR='V(out)' MAX=0.5 MIN={-1} FROM=10n TO=900n DURATION=5n \
             LEVEL=error MESSAGE='out too high' ANALYSES=op,tran XZ=IGNORE",
        );
        let [statement] = netlist.assertions.as_slice() else {
            panic!("one assertion");
        };
        assert_eq!(statement.name, "rail");
        assert_eq!(
            statement.check,
            AssertCheck::Expression {
                expression: "V(out)".into(),
                min: Some(-1.0),
                max: Some(0.5),
            }
        );
        assert_eq!(statement.from, Some(10e-9));
        assert_eq!(statement.to, Some(900e-9));
        assert_eq!(statement.duration, Some(5e-9));
        assert_eq!(statement.level, AssertLevel::Error);
        assert_eq!(statement.message.as_deref(), Some("out too high"));
        assert_eq!(
            statement.analyses,
            [AssertAnalysis::Op, AssertAnalysis::Tran]
        );
        assert!(statement.ignore_unknown);
        assert_eq!(statement.origin.line, 5);

        let netlist = parse(".ASSERT ok COND='D(p)==0 || V(q)<0.5'");
        assert_eq!(
            netlist.assertions[0].check,
            AssertCheck::Condition {
                condition: "D(p)==0 || V(q)<0.5".into()
            }
        );
        assert_eq!(netlist.assertions[0].level, AssertLevel::Warning);
        assert!(netlist.assertions[0].analyses.is_empty());
        assert!(!netlist.assertions[0].ignore_unknown);

        for (card, target) in [
            ("DEV=mp", AssertTarget::Instance("mp".into())),
            ("MOD=pch", AssertTarget::Model("pch".into())),
            ("SUB=amp", AssertTarget::Subcircuit("amp".into())),
            ("PRIMITIVE=mosfet", AssertTarget::Primitive("mosfet".into())),
        ] {
            let netlist = parse(&format!(".ASSERT sat {card} PARAM=VDS MIN=-0.6"));
            assert_eq!(
                netlist.assertions[0].check,
                AssertCheck::Device {
                    target,
                    parameter: "vds".into(),
                    min: Some(-0.6),
                    max: None,
                },
                "{card}"
            );
        }
    }

    #[test]
    fn level_fatal_is_refused_because_evaluation_is_after_the_run() {
        let ParseError::UnsupportedCapability {
            capability, detail, ..
        } = refusal(".ASSERT a EXPR='V(out)' MAX=1 LEVEL=FATAL")
        else {
            panic!("LEVEL=FATAL must be a capability refusal");
        };
        assert_eq!(capability, "netlist.assert.level_fatal");
        assert!(detail.contains("evaluated after the run"), "{detail}");
    }

    #[test]
    fn modelparam_is_refused_naming_param_instead() {
        let ParseError::UnsupportedCapability {
            capability, detail, ..
        } = refusal(".ASSERT a MODELPARAM=vth0 MIN=0.2")
        else {
            panic!("MODELPARAM= must be a capability refusal");
        };
        assert_eq!(capability, "netlist.assert.modelparam");
        assert!(detail.contains("PARAM="), "{detail}");
    }

    #[test]
    fn an_unknown_param_is_refused_naming_it() {
        let error = refusal(".ASSERT a DEV=mp PARAM=nosuch MIN=0").to_string();
        assert!(
            error.contains("PARAM=nosuch is not an operating-point quantity"),
            "{error}"
        );
    }

    #[test]
    fn a_malformed_card_is_refused_in_its_own_sentence() {
        for (card, needle) in [
            (".ASSERT", "requires a name"),
            (".ASSERT a MAX=1", "exactly one thing"),
            (".ASSERT a EXPR='V(out)' COND='1'", "exactly one thing"),
            (".ASSERT a EXPR='V(out)'", "needs MIN=, MAX= or both"),
            (".ASSERT a COND='V(out)>0' MAX=1", "COND= is a condition"),
            (".ASSERT a DEV=mp MIN=0", "needs PARAM="),
            (".ASSERT a PARAM=vds MIN=0", "needs a device target"),
            (
                ".ASSERT a DEV=mp MOD=p PARAM=vds MIN=0",
                "names devices twice",
            ),
            (".ASSERT a EXPR='V(out)' MAX=1 MAX=2", "more than once"),
            (".ASSERT a EXPR='V(out)' MIN=2 MAX=1", "is above MAX"),
            (".ASSERT a EXPR='V(out)' MAX=1 FROM=2 TO=1", "is after TO"),
            (".ASSERT a EXPR='V(out)' MAX=1 LEVEL=loud", "is not a level"),
            (".ASSERT a EXPR='V(out)' MAX=1 XZ=maybe", "is not a mode"),
            (".ASSERT a EXPR='V(out)' MAX=1 ANALYSES=ac", "names 'ac'"),
            (
                ".ASSERT a EXPR='V(out)' MAX=1 DURATION=-1n",
                "must not be negative",
            ),
            (
                ".ASSERT a EXPR='V(out)' MAX=1 WHEN=1",
                "is not an assertion parameter",
            ),
            (".ASSERT a EXPR='V(out' MAX=1", "is not an expression"),
        ] {
            let error = refusal(card).to_string();
            assert!(error.contains(needle), "{card}: {error}");
        }
    }

    #[test]
    fn an_assertion_inside_a_subcircuit_is_refused() {
        let error = Netlist::parse(
            "assert\n.subckt cell a\nR1 a 0 1k\n.ASSERT a EXPR='V(a)' MAX=1\n.ends\n\
             X1 n cell\nV1 n 0 1\n.op\n.end\n",
        )
        .expect_err("a subcircuit body holds no assertion")
        .to_string();
        assert!(
            error.contains("cannot be written inside a .SUBCKT"),
            "{error}"
        );
    }

    #[test]
    fn a_vec_card_names_its_file_and_reports_at_error_unless_it_says_otherwise() {
        // HSPICE Reference Manual, .VEC: `.VEC 'digital_vector_file'`.
        let netlist = parse(".vec 'digstim.vec'");
        let [reference] = netlist.vector_files.as_slice() else {
            panic!("{:?}", netlist.vector_files)
        };
        assert_eq!(reference.path, "digstim.vec");
        assert_eq!(reference.level, AssertLevel::Error);
        assert_eq!(reference.origin.line, 5);
        let netlist = parse(".VEC \"../net/c04.vec\" LEVEL=warning");
        assert_eq!(netlist.vector_files[0].path, "../net/c04.vec");
        assert_eq!(netlist.vector_files[0].level, AssertLevel::Warning);
        let netlist = parse(".vec stim/plain.vec");
        assert_eq!(netlist.vector_files[0].path, "stim/plain.vec");
        for (card, needle) in [
            (".VEC", "requires the vector file's name"),
            (".VEC 'a.vec' LEVEL=loud", "is not a level"),
            (".VEC 'a.vec' MODE=1", "MODE= is not a .VEC parameter"),
            (".VEC 'a.vec' LEVEL=error LEVEL=notice", "more than once"),
        ] {
            let error = refusal(card).to_string();
            assert!(error.contains(needle), "{card}: {error}");
        }
        let ParseError::UnsupportedCapability { capability, .. } =
            refusal(".VEC 'a.vec' LEVEL=FATAL")
        else {
            panic!("LEVEL=FATAL must be a capability refusal");
        };
        assert_eq!(capability, "netlist.assert.level_fatal");
        let error = Netlist::parse(
            "vec\n.subckt cell a\nR1 a 0 1k\n.vec 'a.vec'\n.ends\nX1 n cell\n.op\n.end\n",
        )
        .expect_err("a subcircuit body applies no vector file")
        .to_string();
        assert!(
            error.contains("cannot be written inside a .SUBCKT"),
            "{error}"
        );
    }
}

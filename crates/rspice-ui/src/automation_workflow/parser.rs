use std::{collections::BTreeSet, error::Error, fmt};

use sha2::{Digest, Sha256};

/// Maximum accepted workflow source size. The language has five statements;
/// anything larger is either accidental input or an abuse attempt.
pub const MAX_SOURCE_BYTES: usize = 64 * 1024;
const MAX_STRING_BYTES: usize = 4 * 1024;
const MAX_TOKENS: usize = 2_048;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

impl SourceSpan {
    const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn cover(self, other: Self) -> Self {
        Self::new(self.start.min(other.start), self.end.max(other.end))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    /// One-based line number.
    pub line: usize,
    /// One-based Unicode-scalar column.
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
    SourceTooLarge,
    TooManyTokens,
    InvalidCharacter,
    UnterminatedString,
    InvalidEscape,
    StringTooLarge,
    UnexpectedToken,
    UnknownStage,
    DuplicateStage,
    MissingStage,
    OutOfOrderStage,
    UnknownArgument,
    DuplicateArgument,
    MissingArgument,
    UnsupportedValue,
    DuplicateArtifact,
    EmptyArtifactSet,
}

impl DiagnosticCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceTooLarge => "AUT001",
            Self::TooManyTokens => "AUT002",
            Self::InvalidCharacter => "AUT003",
            Self::UnterminatedString => "AUT004",
            Self::InvalidEscape => "AUT005",
            Self::StringTooLarge => "AUT006",
            Self::UnexpectedToken => "AUT007",
            Self::UnknownStage => "AUT008",
            Self::DuplicateStage => "AUT009",
            Self::MissingStage => "AUT010",
            Self::OutOfOrderStage => "AUT011",
            Self::UnknownArgument => "AUT012",
            Self::DuplicateArgument => "AUT013",
            Self::MissingArgument => "AUT014",
            Self::UnsupportedValue => "AUT015",
            Self::DuplicateArtifact => "AUT016",
            Self::EmptyArtifactSet => "AUT017",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub message: String,
    pub span: SourceSpan,
    pub start: SourceLocation,
    pub end: SourceLocation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticSet {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticSet {
    fn from_parts(source: &str, parts: Vec<DiagnosticPart>) -> Self {
        let diagnostics = parts
            .into_iter()
            .map(|part| Diagnostic {
                code: part.code,
                message: part.message,
                span: part.span,
                start: source_location(source, part.span.start),
                end: source_location(source, part.span.end),
            })
            .collect();
        Self { diagnostics }
    }

    pub fn as_slice(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl fmt::Display for DiagnosticSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(first) = self.diagnostics.first() {
            write!(
                formatter,
                "{} at {}:{}: {}",
                first.code.as_str(),
                first.start.line,
                first.start.column,
                first.message
            )?;
            if self.diagnostics.len() > 1 {
                write!(formatter, " (and {} more)", self.diagnostics.len() - 1)?;
            }
        } else {
            formatter.write_str("workflow validation failed")?;
        }
        Ok(())
    }
}

impl Error for DiagnosticSet {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpannedString {
    pub value: String,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanStageAst {
    pub name: SpannedString,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecuteStageAst {
    pub corners: SpannedString,
    pub target: SpannedString,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequireStageAst {
    pub specs: SpannedString,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompareStageAst {
    pub baseline: SpannedString,
    pub waveforms: bool,
    pub waveforms_span: SourceSpan,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportStageAst {
    pub artifacts: Vec<SpannedString>,
    pub span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkflowAst {
    pub plan: PlanStageAst,
    pub execute: ExecuteStageAst,
    pub require: RequireStageAst,
    pub compare: CompareStageAst,
    pub export: ExportStageAst,
    pub span: SourceSpan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArtifactKind {
    JunitXml,
    SummaryJson,
    VerificationPdf,
}

impl ArtifactKind {
    pub const fn request_name(self) -> &'static str {
        match self {
            Self::JunitXml => "junit",
            Self::SummaryJson => "summary.json",
            Self::VerificationPdf => "report.pdf",
        }
    }

    pub const fn file_name(self) -> &'static str {
        match self {
            Self::JunitXml => "junit.xml",
            Self::SummaryJson => "summary.json",
            Self::VerificationPdf => "verification.pdf",
        }
    }

    pub const fn media_type(self) -> &'static str {
        match self {
            Self::JunitXml => "application/xml",
            Self::SummaryJson => "application/json",
            Self::VerificationPdf => "application/pdf",
        }
    }

    fn from_request_name(value: &str) -> Option<Self> {
        match value {
            "junit" => Some(Self::JunitXml),
            "summary.json" => Some(Self::SummaryJson),
            "report.pdf" => Some(Self::VerificationPdf),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceDigest([u8; 32]);

impl SourceDigest {
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(self) -> String {
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            use fmt::Write as _;
            let _ = write!(output, "{byte:02x}");
        }
        output
    }
}

impl fmt::Display for SourceDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

/// A validated workflow plan. All fields are private so execution sees the
/// exact, immutable result of compilation rather than mutable UI state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutomationPlan {
    source_digest: SourceDigest,
    project_name: String,
    artifacts: BTreeSet<ArtifactKind>,
}

impl AutomationPlan {
    pub const fn source_digest(&self) -> SourceDigest {
        self.source_digest
    }

    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    pub fn artifacts(&self) -> impl ExactSizeIterator<Item = ArtifactKind> + '_ {
        self.artifacts.iter().copied()
    }

    pub const fn corners(&self) -> &'static str {
        "all"
    }

    pub const fn target(&self) -> &'static str {
        "local"
    }

    pub const fn required_specs(&self) -> &'static str {
        "release"
    }

    pub const fn baseline(&self) -> &'static str {
        "main"
    }

    pub const fn compare_waveforms(&self) -> bool {
        true
    }
}

/// Parse and structurally validate the five-stage Automation workflow.
pub fn parse_workflow(source: &str) -> Result<WorkflowAst, DiagnosticSet> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(DiagnosticSet::from_parts(
            source,
            vec![DiagnosticPart::new(
                DiagnosticCode::SourceTooLarge,
                format!(
                    "workflow source is {} bytes; the maximum is {MAX_SOURCE_BYTES}",
                    source.len()
                ),
                SourceSpan::new(0, source.len()),
            )],
        ));
    }

    let tokens = lex(source).map_err(|parts| DiagnosticSet::from_parts(source, parts))?;
    Parser::new(source, tokens)
        .parse()
        .map_err(|parts| DiagnosticSet::from_parts(source, parts))
}

/// Parse, validate all supported values, and compile exact source bytes into
/// an immutable execution plan.
pub fn compile_workflow(source: &str) -> Result<AutomationPlan, DiagnosticSet> {
    let ast = parse_workflow(source)?;
    let mut diagnostics = Vec::new();

    validate_exact_value(
        &ast.plan.name,
        |value| {
            !value.trim().is_empty()
                && value.chars().count() <= 256
                && value.chars().all(|character| !character.is_control())
        },
        "project plan name must contain 1 to 256 non-control characters",
        &mut diagnostics,
    );
    validate_exact_value(
        &ast.execute.corners,
        |value| value == "all",
        "with_corners only supports \"all\"",
        &mut diagnostics,
    );
    validate_exact_value(
        &ast.execute.target,
        |value| value == "local",
        "execute target must be \"local\"; remote targets require a separate authorized service",
        &mut diagnostics,
    );
    validate_exact_value(
        &ast.require.specs,
        |value| value == "release",
        "require specs must be \"release\"",
        &mut diagnostics,
    );
    validate_exact_value(
        &ast.compare.baseline,
        |value| value == "main",
        "comparison baseline must be \"main\"",
        &mut diagnostics,
    );
    if !ast.compare.waveforms {
        diagnostics.push(DiagnosticPart::new(
            DiagnosticCode::UnsupportedValue,
            "waveforms must be True for the release comparison",
            ast.compare.waveforms_span,
        ));
    }

    let mut artifacts = BTreeSet::new();
    for artifact in &ast.export.artifacts {
        let Some(kind) = ArtifactKind::from_request_name(&artifact.value) else {
            diagnostics.push(DiagnosticPart::new(
                DiagnosticCode::UnsupportedValue,
                format!("unsupported export artifact {:?}", artifact.value),
                artifact.span,
            ));
            continue;
        };
        if !artifacts.insert(kind) {
            diagnostics.push(DiagnosticPart::new(
                DiagnosticCode::DuplicateArtifact,
                format!("artifact {:?} is requested more than once", artifact.value),
                artifact.span,
            ));
        }
    }
    if ast.export.artifacts.is_empty() {
        diagnostics.push(DiagnosticPart::new(
            DiagnosticCode::EmptyArtifactSet,
            "export must request at least one artifact",
            ast.export.span,
        ));
    }

    if !diagnostics.is_empty() {
        return Err(DiagnosticSet::from_parts(source, diagnostics));
    }

    let digest: [u8; 32] = Sha256::digest(source.as_bytes()).into();
    Ok(AutomationPlan {
        source_digest: SourceDigest(digest),
        project_name: ast.plan.name.value,
        artifacts,
    })
}

fn validate_exact_value(
    value: &SpannedString,
    predicate: impl FnOnce(&str) -> bool,
    message: &'static str,
    diagnostics: &mut Vec<DiagnosticPart>,
) {
    if !predicate(&value.value) {
        diagnostics.push(DiagnosticPart::new(
            DiagnosticCode::UnsupportedValue,
            message,
            value.span,
        ));
    }
}

#[derive(Clone, Debug)]
struct DiagnosticPart {
    code: DiagnosticCode,
    message: String,
    span: SourceSpan,
}

impl DiagnosticPart {
    fn new(code: DiagnosticCode, message: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            code,
            message: message.into(),
            span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TokenKind {
    Identifier(String),
    String(String),
    Equals,
    Dot,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Newline,
    Eof,
}

#[derive(Clone, Debug)]
struct Token {
    kind: TokenKind,
    span: SourceSpan,
}

fn lex(source: &str) -> Result<Vec<Token>, Vec<DiagnosticPart>> {
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();
    let mut cursor = 0;

    while cursor < source.len() {
        let character = source[cursor..].chars().next().expect("cursor is in range");
        let width = character.len_utf8();
        match character {
            ' ' | '\t' | '\u{000C}' => cursor += width,
            '\n' => {
                tokens.push(Token {
                    kind: TokenKind::Newline,
                    span: SourceSpan::new(cursor, cursor + 1),
                });
                cursor += 1;
            }
            '\r' => {
                let end = if source.as_bytes().get(cursor + 1) == Some(&b'\n') {
                    cursor + 2
                } else {
                    cursor + 1
                };
                tokens.push(Token {
                    kind: TokenKind::Newline,
                    span: SourceSpan::new(cursor, end),
                });
                cursor = end;
            }
            '#' => {
                cursor += 1;
                while cursor < source.len() {
                    let next = source[cursor..].chars().next().expect("cursor is in range");
                    if matches!(next, '\r' | '\n') {
                        break;
                    }
                    cursor += next.len_utf8();
                }
            }
            '"' => lex_string(source, &mut cursor, &mut tokens, &mut diagnostics),
            '=' => push_punctuation(&mut tokens, TokenKind::Equals, cursor, width, &mut cursor),
            '.' => push_punctuation(&mut tokens, TokenKind::Dot, cursor, width, &mut cursor),
            '(' => push_punctuation(
                &mut tokens,
                TokenKind::LeftParen,
                cursor,
                width,
                &mut cursor,
            ),
            ')' => push_punctuation(
                &mut tokens,
                TokenKind::RightParen,
                cursor,
                width,
                &mut cursor,
            ),
            '[' => push_punctuation(
                &mut tokens,
                TokenKind::LeftBracket,
                cursor,
                width,
                &mut cursor,
            ),
            ']' => push_punctuation(
                &mut tokens,
                TokenKind::RightBracket,
                cursor,
                width,
                &mut cursor,
            ),
            ',' => push_punctuation(&mut tokens, TokenKind::Comma, cursor, width, &mut cursor),
            _ if character == '_' || character.is_alphabetic() => {
                let start = cursor;
                cursor += width;
                while cursor < source.len() {
                    let next = source[cursor..].chars().next().expect("cursor is in range");
                    if next == '_' || next.is_alphanumeric() {
                        cursor += next.len_utf8();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    kind: TokenKind::Identifier(source[start..cursor].to_owned()),
                    span: SourceSpan::new(start, cursor),
                });
            }
            _ => {
                diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::InvalidCharacter,
                    format!("character {character:?} is not permitted in Automation workflows"),
                    SourceSpan::new(cursor, cursor + width),
                ));
                cursor += width;
            }
        }

        if tokens.len() > MAX_TOKENS {
            diagnostics.push(DiagnosticPart::new(
                DiagnosticCode::TooManyTokens,
                format!("workflow exceeds the {MAX_TOKENS}-token limit"),
                SourceSpan::new(cursor.saturating_sub(width), cursor),
            ));
            break;
        }
    }

    if diagnostics.is_empty() {
        tokens.push(Token {
            kind: TokenKind::Eof,
            span: SourceSpan::new(source.len(), source.len()),
        });
        Ok(tokens)
    } else {
        Err(diagnostics)
    }
}

fn push_punctuation(
    tokens: &mut Vec<Token>,
    kind: TokenKind,
    start: usize,
    width: usize,
    cursor: &mut usize,
) {
    tokens.push(Token {
        kind,
        span: SourceSpan::new(start, start + width),
    });
    *cursor += width;
}

fn lex_string(
    source: &str,
    cursor: &mut usize,
    tokens: &mut Vec<Token>,
    diagnostics: &mut Vec<DiagnosticPart>,
) {
    let start = *cursor;
    *cursor += 1;
    let mut value = String::new();
    let mut terminated = false;

    while *cursor < source.len() {
        let character = source[*cursor..]
            .chars()
            .next()
            .expect("cursor is in range");
        match character {
            '"' => {
                *cursor += 1;
                terminated = true;
                break;
            }
            '\r' | '\n' => break,
            '\\' => {
                let escape_start = *cursor;
                *cursor += 1;
                if *cursor >= source.len() {
                    break;
                }
                let escaped = source[*cursor..]
                    .chars()
                    .next()
                    .expect("cursor is in range");
                *cursor += escaped.len_utf8();
                match escaped {
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    _ => diagnostics.push(DiagnosticPart::new(
                        DiagnosticCode::InvalidEscape,
                        format!("unsupported string escape \\{escaped}"),
                        SourceSpan::new(escape_start, *cursor),
                    )),
                }
            }
            _ => {
                value.push(character);
                *cursor += character.len_utf8();
            }
        }

        if value.len() > MAX_STRING_BYTES {
            diagnostics.push(DiagnosticPart::new(
                DiagnosticCode::StringTooLarge,
                format!("string literal exceeds the {MAX_STRING_BYTES}-byte limit"),
                SourceSpan::new(start, *cursor),
            ));
            while *cursor < source.len() {
                let next = source[*cursor..]
                    .chars()
                    .next()
                    .expect("cursor is in range");
                if matches!(next, '"' | '\r' | '\n') {
                    if next == '"' {
                        *cursor += 1;
                        terminated = true;
                    }
                    break;
                }
                *cursor += next.len_utf8();
            }
            break;
        }
    }

    if terminated {
        tokens.push(Token {
            kind: TokenKind::String(value),
            span: SourceSpan::new(start, *cursor),
        });
    } else {
        diagnostics.push(DiagnosticPart::new(
            DiagnosticCode::UnterminatedString,
            "string literal is not terminated on this line",
            SourceSpan::new(start, *cursor),
        ));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StageKind {
    Plan,
    Execute,
    Require,
    Compare,
    Export,
}

impl StageKind {
    const ALL: [Self; 5] = [
        Self::Plan,
        Self::Execute,
        Self::Require,
        Self::Compare,
        Self::Export,
    ];

    const fn index(self) -> usize {
        match self {
            Self::Plan => 0,
            Self::Execute => 1,
            Self::Require => 2,
            Self::Compare => 3,
            Self::Export => 4,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Plan => "project.plan",
            Self::Execute => "with_corners/execute",
            Self::Require => "require",
            Self::Compare => "compare",
            Self::Export => "export",
        }
    }
}

struct Parser<'source> {
    source: &'source str,
    tokens: Vec<Token>,
    cursor: usize,
    diagnostics: Vec<DiagnosticPart>,
    seen_order: Vec<(StageKind, SourceSpan)>,
    plan: Option<PlanStageAst>,
    execute: Option<ExecuteStageAst>,
    require: Option<RequireStageAst>,
    compare: Option<CompareStageAst>,
    export: Option<ExportStageAst>,
}

impl<'source> Parser<'source> {
    fn new(source: &'source str, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            cursor: 0,
            diagnostics: Vec::new(),
            seen_order: Vec::new(),
            plan: None,
            execute: None,
            require: None,
            compare: None,
            export: None,
        }
    }

    fn parse(mut self) -> Result<WorkflowAst, Vec<DiagnosticPart>> {
        self.skip_newlines();
        while !self.at_eof() {
            let statement_start = self.current().span;
            let Some(kind) = self.classify_stage() else {
                self.diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::UnknownStage,
                    "expected one of the five Automation workflow stages",
                    statement_start,
                ));
                self.synchronize_line();
                self.skip_newlines();
                continue;
            };

            self.seen_order.push((kind, statement_start));
            let parsed = match kind {
                StageKind::Plan => self.parse_plan().map(ParsedStage::Plan),
                StageKind::Execute => self.parse_execute().map(ParsedStage::Execute),
                StageKind::Require => self.parse_require().map(ParsedStage::Require),
                StageKind::Compare => self.parse_compare().map(ParsedStage::Compare),
                StageKind::Export => self.parse_export().map(ParsedStage::Export),
            };

            if let Some(stage) = parsed {
                self.store_stage(kind, stage);
            } else {
                self.synchronize_line();
            }
            self.skip_newlines();
        }

        self.validate_stage_set();
        if !self.diagnostics.is_empty() {
            return Err(self.diagnostics);
        }

        let plan = self.plan.expect("validated plan stage");
        let execute = self.execute.expect("validated execute stage");
        let require = self.require.expect("validated require stage");
        let compare = self.compare.expect("validated compare stage");
        let export = self.export.expect("validated export stage");
        let span = plan.span.cover(export.span);
        Ok(WorkflowAst {
            plan,
            execute,
            require,
            compare,
            export,
            span,
        })
    }

    fn classify_stage(&self) -> Option<StageKind> {
        match self.kind_at(0) {
            TokenKind::Identifier(name) if name == "plan" => Some(StageKind::Plan),
            TokenKind::Identifier(name) if name == "run" => match self.kind_at(1) {
                TokenKind::Equals => Some(StageKind::Execute),
                TokenKind::Dot => match self.kind_at(2) {
                    TokenKind::Identifier(method) if method == "require" => {
                        Some(StageKind::Require)
                    }
                    TokenKind::Identifier(method) if method == "compare" => {
                        Some(StageKind::Compare)
                    }
                    TokenKind::Identifier(method) if method == "export" => Some(StageKind::Export),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_plan(&mut self) -> Option<PlanStageAst> {
        let start = self.expect_identifier("plan")?;
        self.expect_punctuation(TokenKind::Equals, "=")?;
        self.expect_identifier("project")?;
        self.expect_punctuation(TokenKind::Dot, ".")?;
        self.expect_identifier("plan")?;
        self.expect_punctuation(TokenKind::LeftParen, "(")?;
        let name = self.expect_string()?;
        let end = self.expect_punctuation(TokenKind::RightParen, ")")?;
        self.expect_line_end()?;
        Some(PlanStageAst {
            name,
            span: start.cover(end),
        })
    }

    fn parse_execute(&mut self) -> Option<ExecuteStageAst> {
        let start = self.expect_identifier("run")?;
        self.expect_punctuation(TokenKind::Equals, "=")?;
        self.expect_identifier("plan")?;
        self.expect_punctuation(TokenKind::Dot, ".")?;
        self.expect_identifier("with_corners")?;
        self.expect_punctuation(TokenKind::LeftParen, "(")?;
        let corners = self.expect_string()?;
        self.expect_punctuation(TokenKind::RightParen, ")")?;
        self.expect_punctuation(TokenKind::Dot, ".")?;
        self.expect_identifier("execute")?;
        self.expect_punctuation(TokenKind::LeftParen, "(")?;
        let arguments = self.parse_named_arguments(&["target"])?;
        let end = self.expect_punctuation(TokenKind::RightParen, ")")?;
        self.expect_line_end()?;
        let target = arguments.string("target", end, &mut self.diagnostics)?;
        Some(ExecuteStageAst {
            corners,
            target,
            span: start.cover(end),
        })
    }

    fn parse_require(&mut self) -> Option<RequireStageAst> {
        let start = self.expect_identifier("run")?;
        self.expect_punctuation(TokenKind::Dot, ".")?;
        self.expect_identifier("require")?;
        self.expect_punctuation(TokenKind::LeftParen, "(")?;
        let arguments = self.parse_named_arguments(&["specs"])?;
        let end = self.expect_punctuation(TokenKind::RightParen, ")")?;
        self.expect_line_end()?;
        let specs = arguments.string("specs", end, &mut self.diagnostics)?;
        Some(RequireStageAst {
            specs,
            span: start.cover(end),
        })
    }

    fn parse_compare(&mut self) -> Option<CompareStageAst> {
        let start = self.expect_identifier("run")?;
        self.expect_punctuation(TokenKind::Dot, ".")?;
        self.expect_identifier("compare")?;
        self.expect_punctuation(TokenKind::LeftParen, "(")?;
        let arguments = self.parse_named_arguments(&["baseline", "waveforms"])?;
        let end = self.expect_punctuation(TokenKind::RightParen, ")")?;
        self.expect_line_end()?;
        let baseline = arguments.string("baseline", end, &mut self.diagnostics)?;
        let (waveforms, waveforms_span) =
            arguments.boolean("waveforms", end, &mut self.diagnostics)?;
        Some(CompareStageAst {
            baseline,
            waveforms,
            waveforms_span,
            span: start.cover(end),
        })
    }

    fn parse_export(&mut self) -> Option<ExportStageAst> {
        let start = self.expect_identifier("run")?;
        self.expect_punctuation(TokenKind::Dot, ".")?;
        self.expect_identifier("export")?;
        self.expect_punctuation(TokenKind::LeftParen, "(")?;
        self.expect_punctuation(TokenKind::LeftBracket, "[")?;
        let mut artifacts = Vec::new();
        if !self.check_punctuation(&TokenKind::RightBracket) {
            loop {
                artifacts.push(self.expect_string()?);
                if !self.consume_punctuation(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect_punctuation(TokenKind::RightBracket, "]")?;
        let end = self.expect_punctuation(TokenKind::RightParen, ")")?;
        self.expect_line_end()?;
        Some(ExportStageAst {
            artifacts,
            span: start.cover(end),
        })
    }

    fn parse_named_arguments(&mut self, allowed: &[&str]) -> Option<NamedArguments> {
        let mut arguments = NamedArguments::default();
        if self.check_punctuation(&TokenKind::RightParen) {
            return Some(arguments);
        }

        loop {
            let (name, name_span) = self.expect_any_identifier()?;
            self.expect_punctuation(TokenKind::Equals, "=")?;
            let value = match &self.current().kind {
                TokenKind::String(value) => {
                    let value = NamedValue::String(SpannedString {
                        value: value.clone(),
                        span: self.current().span,
                    });
                    self.advance();
                    value
                }
                TokenKind::Identifier(value) if value == "True" || value == "False" => {
                    let value = NamedValue::Boolean(value == "True", self.current().span);
                    self.advance();
                    value
                }
                _ => {
                    self.unexpected("a string literal or True/False");
                    return None;
                }
            };

            if !allowed.contains(&name.as_str()) {
                self.diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::UnknownArgument,
                    format!("unknown argument {name:?}"),
                    name_span,
                ));
            } else if arguments
                .values
                .iter()
                .any(|argument| argument.name == name)
            {
                self.diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::DuplicateArgument,
                    format!("argument {name:?} is specified more than once"),
                    name_span,
                ));
            } else {
                arguments.values.push(NamedArgument { name, value });
            }

            if !self.consume_punctuation(&TokenKind::Comma) {
                break;
            }
        }
        Some(arguments)
    }

    fn store_stage(&mut self, kind: StageKind, stage: ParsedStage) {
        let occupied = match kind {
            StageKind::Plan => self.plan.is_some(),
            StageKind::Execute => self.execute.is_some(),
            StageKind::Require => self.require.is_some(),
            StageKind::Compare => self.compare.is_some(),
            StageKind::Export => self.export.is_some(),
        };
        if occupied {
            self.diagnostics.push(DiagnosticPart::new(
                DiagnosticCode::DuplicateStage,
                format!("{} stage is declared more than once", kind.label()),
                stage.span(),
            ));
            return;
        }
        match stage {
            ParsedStage::Plan(value) => self.plan = Some(value),
            ParsedStage::Execute(value) => self.execute = Some(value),
            ParsedStage::Require(value) => self.require = Some(value),
            ParsedStage::Compare(value) => self.compare = Some(value),
            ParsedStage::Export(value) => self.export = Some(value),
        }
    }

    fn validate_stage_set(&mut self) {
        for kind in StageKind::ALL {
            let present = match kind {
                StageKind::Plan => self.plan.is_some(),
                StageKind::Execute => self.execute.is_some(),
                StageKind::Require => self.require.is_some(),
                StageKind::Compare => self.compare.is_some(),
                StageKind::Export => self.export.is_some(),
            };
            if !present {
                self.diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::MissingStage,
                    format!("missing required {} stage", kind.label()),
                    SourceSpan::new(self.source.len(), self.source.len()),
                ));
            }
        }

        for (position, (kind, span)) in self.seen_order.iter().enumerate() {
            if kind.index() != position.min(StageKind::ALL.len()) {
                self.diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::OutOfOrderStage,
                    format!(
                        "{} is out of order; the workflow order is plan, execute, require, compare, export",
                        kind.label()
                    ),
                    *span,
                ));
            }
        }
    }

    fn expect_identifier(&mut self, expected: &'static str) -> Option<SourceSpan> {
        match &self.current().kind {
            TokenKind::Identifier(actual) if actual == expected => {
                let span = self.current().span;
                self.advance();
                Some(span)
            }
            _ => {
                self.unexpected(&format!("{expected:?}"));
                None
            }
        }
    }

    fn expect_any_identifier(&mut self) -> Option<(String, SourceSpan)> {
        match &self.current().kind {
            TokenKind::Identifier(value) => {
                let result = (value.clone(), self.current().span);
                self.advance();
                Some(result)
            }
            _ => {
                self.unexpected("an argument name");
                None
            }
        }
    }

    fn expect_string(&mut self) -> Option<SpannedString> {
        match &self.current().kind {
            TokenKind::String(value) => {
                let result = SpannedString {
                    value: value.clone(),
                    span: self.current().span,
                };
                self.advance();
                Some(result)
            }
            _ => {
                self.unexpected("a string literal");
                None
            }
        }
    }

    fn expect_punctuation(
        &mut self,
        expected: TokenKind,
        label: &'static str,
    ) -> Option<SourceSpan> {
        if self.current().kind == expected {
            let span = self.current().span;
            self.advance();
            Some(span)
        } else {
            self.unexpected(label);
            None
        }
    }

    fn expect_line_end(&mut self) -> Option<()> {
        if matches!(&self.current().kind, TokenKind::Newline | TokenKind::Eof) {
            Some(())
        } else {
            self.unexpected("the end of the statement");
            None
        }
    }

    fn unexpected(&mut self, expected: &str) {
        self.diagnostics.push(DiagnosticPart::new(
            DiagnosticCode::UnexpectedToken,
            format!("expected {expected}, found {}", self.current_label()),
            self.current().span,
        ));
    }

    fn current_label(&self) -> String {
        match &self.current().kind {
            TokenKind::Identifier(value) => format!("identifier {value:?}"),
            TokenKind::String(_) => "a string literal".to_owned(),
            TokenKind::Equals => "'='".to_owned(),
            TokenKind::Dot => "'.'".to_owned(),
            TokenKind::LeftParen => "'('".to_owned(),
            TokenKind::RightParen => "')'".to_owned(),
            TokenKind::LeftBracket => "'['".to_owned(),
            TokenKind::RightBracket => "']'".to_owned(),
            TokenKind::Comma => "','".to_owned(),
            TokenKind::Newline => "the end of the line".to_owned(),
            TokenKind::Eof => "the end of the source".to_owned(),
        }
    }

    fn kind_at(&self, offset: usize) -> &TokenKind {
        &self.tokens[(self.cursor + offset).min(self.tokens.len() - 1)].kind
    }

    fn current(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn advance(&mut self) {
        if !self.at_eof() {
            self.cursor += 1;
        }
    }

    fn at_eof(&self) -> bool {
        matches!(&self.current().kind, TokenKind::Eof)
    }

    fn check_punctuation(&self, expected: &TokenKind) -> bool {
        &self.current().kind == expected
    }

    fn consume_punctuation(&mut self, expected: &TokenKind) -> bool {
        if self.check_punctuation(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn synchronize_line(&mut self) {
        while !matches!(&self.current().kind, TokenKind::Newline | TokenKind::Eof) {
            self.advance();
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(&self.current().kind, TokenKind::Newline) {
            self.advance();
        }
    }
}

enum ParsedStage {
    Plan(PlanStageAst),
    Execute(ExecuteStageAst),
    Require(RequireStageAst),
    Compare(CompareStageAst),
    Export(ExportStageAst),
}

impl ParsedStage {
    const fn span(&self) -> SourceSpan {
        match self {
            Self::Plan(value) => value.span,
            Self::Execute(value) => value.span,
            Self::Require(value) => value.span,
            Self::Compare(value) => value.span,
            Self::Export(value) => value.span,
        }
    }
}

#[derive(Default)]
struct NamedArguments {
    values: Vec<NamedArgument>,
}

impl NamedArguments {
    fn value(&self, name: &str) -> Option<&NamedValue> {
        self.values
            .iter()
            .find(|argument| argument.name == name)
            .map(|argument| &argument.value)
    }

    fn string(
        &self,
        name: &'static str,
        fallback_span: SourceSpan,
        diagnostics: &mut Vec<DiagnosticPart>,
    ) -> Option<SpannedString> {
        match self.value(name) {
            Some(NamedValue::String(value)) => Some(value.clone()),
            Some(NamedValue::Boolean(_, span)) => {
                diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::UnexpectedToken,
                    format!("argument {name:?} must be a string literal"),
                    *span,
                ));
                None
            }
            None => {
                diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::MissingArgument,
                    format!("missing required argument {name:?}"),
                    fallback_span,
                ));
                None
            }
        }
    }

    fn boolean(
        &self,
        name: &'static str,
        fallback_span: SourceSpan,
        diagnostics: &mut Vec<DiagnosticPart>,
    ) -> Option<(bool, SourceSpan)> {
        match self.value(name) {
            Some(NamedValue::Boolean(value, span)) => Some((*value, *span)),
            Some(NamedValue::String(value)) => {
                diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::UnexpectedToken,
                    format!("argument {name:?} must be True or False"),
                    value.span,
                ));
                None
            }
            None => {
                diagnostics.push(DiagnosticPart::new(
                    DiagnosticCode::MissingArgument,
                    format!("missing required argument {name:?}"),
                    fallback_span,
                ));
                None
            }
        }
    }
}

struct NamedArgument {
    name: String,
    value: NamedValue,
}

enum NamedValue {
    String(SpannedString),
    Boolean(bool, SourceSpan),
}

fn source_location(source: &str, byte_offset: usize) -> SourceLocation {
    let limit = byte_offset.min(source.len());
    let mut line = 1;
    let mut column = 1;
    let mut cursor = 0;
    while cursor < limit {
        let character = source[cursor..].chars().next().expect("cursor is in range");
        match character {
            '\r' => {
                line += 1;
                column = 1;
                cursor += 1;
                if cursor < limit && source.as_bytes().get(cursor) == Some(&b'\n') {
                    cursor += 1;
                }
            }
            '\n' => {
                line += 1;
                column = 1;
                cursor += 1;
            }
            _ => {
                column += 1;
                cursor += character.len_utf8();
            }
        }
    }
    SourceLocation { line, column }
}

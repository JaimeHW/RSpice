//! Fail-closed Spectre model-library adapter.
//!
//! RSpice executes canonical SPICE cards. This module parses the Spectre
//! statements used by foundry model libraries and macromodels, builds a symbol
//! table for model and subcircuit masters, then lowers the typed statements to
//! a line-preserving canonical projection. Unsupported semantics remain
//! explicit errors instead of being discarded or guessed.
//!
//! It sits in the parsing layer rather than in `library` because it is source
//! text in and source text out — a dialect front-end, run by the include
//! expander on every source it reads before a single card is parsed.
//! `library` re-exports it, which is where a model-library consumer looks.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpectreModelAdapterError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for SpectreModelAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for SpectreModelAdapterError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    Spectre,
    Spice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpectreStatement {
    line: usize,
    consumed_lines: usize,
    kind: SpectreStatementKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SpectreStatementKind {
    Lowered(String),
    Model {
        name: String,
        canonical_type: String,
        source_family: String,
        lowered: String,
    },
    Subcircuit {
        name: String,
        lowered: String,
    },
    Statistics(SpectreStatisticsBlock),
    Instance(SpectreInstance),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpectreInstance {
    name: String,
    nodes: Vec<String>,
    master: String,
    parameters: Vec<SpectreModelAssignment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpectreVariationScope {
    Process,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpectreVariation {
    line: usize,
    scope: SpectreVariationScope,
    parameter: String,
    distribution: String,
    attributes: Vec<SpectreModelAssignment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpectreCorrelation {
    line: usize,
    parameters: Vec<String>,
    coefficient: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpectreStatisticsBlock {
    variations: Vec<SpectreVariation>,
    correlations: Vec<SpectreCorrelation>,
}

#[derive(Debug, Default)]
struct SpectreSymbols {
    models: HashMap<String, String>,
    bsimsoi_models: HashSet<String>,
    subcircuits: HashSet<String>,
    /// Lowercase Spectre instance name of every independent source, mapped to
    /// the canonical card name it lowers to. A `dc dev=` or a `noise iprobe=`
    /// names the Spectre instance, while the analysis card must name the
    /// lowered one.
    sources: HashMap<String, String>,
}

/// Adapt a supported Spectre model-library source without changing its line
/// count. Plain SPICE sources are returned by reference.
pub(crate) fn is_native_spectre_source(path: &Path, source: &str) -> bool {
    let is_scs = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("scs"));
    if is_scs {
        return true;
    }

    // A language boundary inside a line or block comment is inert. Keep this
    // lightweight recognition in lockstep with the adapter without requiring
    // a successful full Spectre comment pass merely to select the dialect.
    let mut in_block_comment = false;
    source.lines().enumerate().any(|(line_index, line)| {
        strip_spectre_comments_from_line(line, &mut in_block_comment, line_index + 1, None)
            .trim()
            .to_ascii_lowercase()
            .starts_with("simulator lang=")
    })
}

pub fn adapt_spectre_model_library<'a>(
    path: &Path,
    source: &'a str,
) -> Result<Cow<'a, str>, SpectreModelAdapterError> {
    if !is_native_spectre_source(path, source) {
        return Ok(Cow::Borrowed(source));
    }

    let lexed_lines = preprocess_spectre_comments(path, source)?;
    let lines = lexed_lines.iter().map(String::as_str).collect::<Vec<_>>();
    let statements = parse_spectre_statements(path, &lines)?;
    let symbols = SpectreSymbols::from_statements(&statements)?;
    let mut output = Vec::<String>::with_capacity(lines.len());
    for statement in statements {
        let continuation_label = match &statement.kind {
            SpectreStatementKind::Model { .. } => "model",
            SpectreStatementKind::Subcircuit { .. } => "subcircuit",
            SpectreStatementKind::Statistics(_) => "statistics",
            SpectreStatementKind::Instance(_) => "instance",
            SpectreStatementKind::Lowered(_) => "statement",
        };
        let lowered = match statement.kind {
            SpectreStatementKind::Lowered(lowered)
            | SpectreStatementKind::Model { lowered, .. }
            | SpectreStatementKind::Subcircuit { lowered, .. } => lowered,
            SpectreStatementKind::Statistics(statistics) => lower_statistics(&statistics)?,
            SpectreStatementKind::Instance(instance) => {
                lower_spectre_instance(&instance, &symbols, statement.line)?
            }
        };
        output.push(lowered);
        for continuation in 1..statement.consumed_lines {
            output.push(format!(
                "* RSpice spectre-{continuation_label}/2 continuation line {}",
                statement.line + continuation
            ));
        }
    }

    let mut adapted = output.join("\n");
    if source.ends_with('\n') {
        adapted.push('\n');
    }
    debug_assert_eq!(adapted.lines().count(), source.lines().count());
    Ok(Cow::Owned(adapted))
}

fn parse_spectre_statements(
    path: &Path,
    lines: &[&str],
) -> Result<Vec<SpectreStatement>, SpectreModelAdapterError> {
    let is_scs = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("scs"));
    let mut language = if is_scs {
        Language::Spectre
    } else {
        Language::Spice
    };
    let mut statements = Vec::with_capacity(lines.len());
    let mut index = 0usize;
    while index < lines.len() {
        let line_number = index + 1;
        let raw = lines[index];
        let trimmed = raw.trim();
        let lower = trimmed.to_ascii_lowercase();
        if lower == "simulator lang=spectre" {
            language = Language::Spectre;
            statements.push(lowered_statement(line_number, adapter_receipt(trimmed)));
            index += 1;
            continue;
        }
        if lower == "simulator lang=spice" {
            language = Language::Spice;
            statements.push(lowered_statement(
                line_number,
                spice_interop_receipt(trimmed),
            ));
            index += 1;
            continue;
        }
        if lower.starts_with("simulator lang=") {
            return Err(error(
                line_number,
                format!("unsupported Spectre language boundary '{trimmed}'"),
            ));
        }
        if language == Language::Spice {
            statements.push(lowered_statement(line_number, raw.to_owned()));
            index += 1;
            continue;
        }
        if trimmed.is_empty() {
            statements.push(lowered_statement(line_number, String::new()));
            index += 1;
            continue;
        }
        if let Some(comment) = trimmed.strip_prefix("//") {
            statements.push(lowered_statement(line_number, format!("*{}", comment)));
            index += 1;
            continue;
        }
        if trimmed.starts_with('*') {
            statements.push(lowered_statement(line_number, raw.to_owned()));
            index += 1;
            continue;
        }
        if trimmed.starts_with('.') {
            return Err(error(
                line_number,
                format!(
                    "SPICE directive '{}' in a Spectre source requires an explicit simulator lang=spice boundary",
                    trimmed.split_whitespace().next().unwrap_or(trimmed)
                ),
            ));
        }

        let (head, rest) = split_head(trimmed);
        let head_lower = head.to_ascii_lowercase();
        match head_lower.as_str() {
            "library" | "endlibrary" => {
                statements.push(lowered_statement(line_number, adapter_receipt(trimmed)))
            }
            "section" => {
                let name = one_identifier(rest, line_number, "section")?;
                statements.push(lowered_statement(line_number, format!(".lib {name}")));
            }
            "endsection" => {
                let name = rest.split_whitespace().next().unwrap_or_default();
                statements.push(lowered_statement(
                    line_number,
                    if name.is_empty() {
                        ".endl".to_owned()
                    } else {
                        format!(".endl {name}")
                    },
                ));
            }
            "include" => statements.push(lowered_statement(
                line_number,
                adapt_include(rest, line_number)?,
            )),
            "model" => {
                let (adapted, source_family, consumed) = adapt_model(lines, index)?;
                let (name, canonical_type) = adapted_model_identity(&adapted, line_number)?;
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: consumed,
                    kind: SpectreStatementKind::Model {
                        name,
                        canonical_type,
                        source_family,
                        lowered: adapted,
                    },
                });
                index += consumed;
                continue;
            }
            "parameters" => {
                let (logical, consumed) = collect_continued_statement(lines, index, true);
                let (_, assignments) = split_head(&logical);
                let assignments = parse_spectre_model_assignments(assignments, line_number)?;
                if assignments.is_empty() {
                    return Err(error(
                        line_number,
                        "Spectre parameters statement has no assignments",
                    ));
                }
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: consumed,
                    kind: SpectreStatementKind::Lowered(format!(
                        ".param {}",
                        render_assignments(&assignments)
                    )),
                });
                index += consumed;
                continue;
            }
            "subckt" => {
                let (name, lowered) = adapt_subcircuit(rest, line_number)?;
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: 1,
                    kind: SpectreStatementKind::Subcircuit { name, lowered },
                });
            }
            "inline" => {
                let (nested_head, nested_rest) = split_head(rest);
                if !nested_head.eq_ignore_ascii_case("subckt") {
                    return Err(error(
                        line_number,
                        "Spectre inline statement must declare a subckt",
                    ));
                }
                let (name, lowered) = adapt_subcircuit(nested_rest, line_number)?;
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: 1,
                    kind: SpectreStatementKind::Subcircuit { name, lowered },
                });
            }
            "ends" => {
                let name = rest.split_whitespace().next().unwrap_or_default();
                statements.push(lowered_statement(
                    line_number,
                    if name.is_empty() {
                        ".ends".to_owned()
                    } else {
                        format!(".ends {name}")
                    },
                ));
            }
            "global" => {
                if rest.is_empty() {
                    return Err(error(line_number, "Spectre global statement has no nodes"));
                }
                statements.push(lowered_statement(line_number, format!(".global {rest}")));
            }
            "ahdl_include" => {
                statements.push(lowered_statement(
                    line_number,
                    adapt_ahdl_include(rest, line_number)?,
                ));
            }
            "statistics" => {
                let (statistics, consumed) = parse_statistics_block(lines, index)?;
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: consumed,
                    kind: SpectreStatementKind::Statistics(statistics),
                });
                index += consumed;
                continue;
            }
            "save" => {
                let (logical, consumed) = collect_continued_statement(lines, index, false);
                let (_, entries) = split_head(&logical);
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: consumed,
                    kind: SpectreStatementKind::Lowered(adapt_save(entries, line_number)?),
                });
                index += consumed;
                continue;
            }
            _ if matches!(
                SpectreConstruct::classify(SpectreNamespace::Statement, &head_lower)
                    .map(SpectreConstruct::support),
                Some(SpectreSupport::Unsupported(_))
            ) =>
            {
                return Err(unsupported_construct(
                    SpectreNamespace::Statement,
                    head,
                    line_number,
                ));
            }
            _ => {
                let (logical, consumed) = collect_continued_statement(lines, index, false);
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: consumed,
                    kind: SpectreStatementKind::Instance(parse_spectre_instance(
                        &logical,
                        line_number,
                    )?),
                });
                index += consumed;
                continue;
            }
        }
        index += 1;
    }
    Ok(statements)
}

fn parse_statistics_block(
    lines: &[&str],
    start: usize,
) -> Result<(SpectreStatisticsBlock, usize), SpectreModelAdapterError> {
    let start_line = start + 1;
    let (_, opening) = split_head(lines[start].trim());
    if opening.trim() != "{" {
        return Err(error(
            start_line,
            "Spectre statistics declaration must open a standalone '{' block",
        ));
    }

    let mut variations = Vec::new();
    let mut correlations = Vec::new();
    let mut scope = None;
    let mut consumed = 1usize;
    let mut closed = false;
    while let Some(raw) = lines.get(start + consumed) {
        let line = start + consumed + 1;
        let trimmed = raw.trim();
        consumed += 1;
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "}" {
            if scope.take().is_none() {
                closed = true;
                break;
            }
            continue;
        }

        let (head, rest) = split_head(trimmed);
        match head.to_ascii_lowercase().as_str() {
            "process" | "mismatch" => {
                if scope.is_some() {
                    return Err(error(line, "Spectre statistics scopes cannot be nested"));
                }
                if rest.trim() != "{" {
                    return Err(error(
                        line,
                        format!("Spectre statistics scope '{head}' must open with '{{'"),
                    ));
                }
                scope = Some(if head.eq_ignore_ascii_case("process") {
                    SpectreVariationScope::Process
                } else {
                    SpectreVariationScope::Mismatch
                });
            }
            "vary" => {
                let active_scope = scope.ok_or_else(|| {
                    error(
                        line,
                        "Spectre vary declaration must be inside process or mismatch",
                    )
                })?;
                let (parameter, attributes) = take_token(rest)
                    .ok_or_else(|| error(line, "Spectre vary declaration has no parameter name"))?;
                let mut attributes = parse_spectre_model_assignments(attributes, line)?;
                let distribution = take_assignment(&mut attributes, "dist").ok_or_else(|| {
                    error(
                        line,
                        format!("Spectre vary declaration for '{parameter}' has no dist="),
                    )
                })?;
                let distribution_name = distribution
                    .value
                    .trim_matches(['\'', '"'])
                    .to_ascii_lowercase();
                if !matches!(
                    distribution_name.as_str(),
                    "gauss" | "gaussian" | "normal" | "lnorm" | "lognormal" | "unif" | "uniform"
                ) {
                    return Err(error(
                        line,
                        format!(
                            "Spectre vary declaration for '{parameter}' uses unsupported distribution '{}'",
                            distribution.value
                        ),
                    ));
                }
                if !attributes.iter().any(|attribute| {
                    ["std", "sigma", "percent", "N", "mean"]
                        .iter()
                        .any(|name| attribute.name.eq_ignore_ascii_case(name))
                }) {
                    return Err(error(
                        line,
                        format!(
                            "Spectre vary declaration for '{parameter}' has no spread attribute"
                        ),
                    ));
                }
                variations.push(SpectreVariation {
                    line,
                    scope: active_scope,
                    parameter: parameter.to_owned(),
                    distribution: distribution_name,
                    attributes,
                });
            }
            "correlate" => {
                if scope.is_some() {
                    return Err(error(
                        line,
                        "Spectre correlate declaration must be outside process/mismatch blocks",
                    ));
                }
                let mut attributes = parse_spectre_model_assignments(rest, line)?;
                if let Some(selection) = take_assignment(&mut attributes, "dev") {
                    return Err(error(
                        line,
                        format!(
                            "Spectre correlate dev={} device-selection semantics are not represented; use param=[...] for parameter correlation",
                            selection.value
                        ),
                    ));
                }
                let parameters = take_assignment(&mut attributes, "param").ok_or_else(|| {
                    error(line, "Spectre correlate declaration has no param= list")
                })?;
                let coefficient = take_assignment(&mut attributes, "cc").ok_or_else(|| {
                    error(line, "Spectre correlate declaration has no cc= coefficient")
                })?;
                if !attributes.is_empty() {
                    return Err(error(
                        line,
                        format!(
                            "Spectre correlate declaration has unsupported attributes: {}",
                            attributes
                                .iter()
                                .map(|attribute| attribute.name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    ));
                }
                let parameters = parameters
                    .value
                    .trim()
                    .strip_prefix('[')
                    .and_then(|value| value.strip_suffix(']'))
                    .ok_or_else(|| {
                        error(
                            line,
                            "Spectre correlate param= value must be a bracketed list",
                        )
                    })?
                    .split(|character: char| character.is_whitespace() || character == ',')
                    .filter(|parameter| !parameter.is_empty())
                    .map(str::to_owned)
                    .collect::<Vec<_>>();
                if parameters.len() < 2 {
                    return Err(error(
                        line,
                        "Spectre correlate param= list requires at least two variables",
                    ));
                }
                correlations.push(SpectreCorrelation {
                    line,
                    parameters,
                    coefficient: coefficient.value,
                });
            }
            _ => {
                return Err(error(
                    line,
                    format!("unsupported Spectre statistics statement '{head}'"),
                ));
            }
        }
    }
    if !closed {
        return Err(error(
            start_line,
            "Spectre statistics block is not closed by '}'",
        ));
    }
    if scope.is_some() {
        return Err(error(
            start_line,
            "Spectre statistics process/mismatch scope is not closed",
        ));
    }
    if variations.is_empty() {
        return Err(error(
            start_line,
            "Spectre statistics block declares no variations",
        ));
    }
    let declared = variations
        .iter()
        .filter(|variation| variation.scope == SpectreVariationScope::Process)
        .map(|variation| variation.parameter.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    for correlation in &correlations {
        for parameter in &correlation.parameters {
            if !declared.contains(&parameter.to_ascii_lowercase()) {
                return Err(error(
                    correlation.line,
                    format!(
                        "Spectre correlation references undeclared process variation '{parameter}'"
                    ),
                ));
            }
        }
    }
    Ok((
        SpectreStatisticsBlock {
            variations,
            correlations,
        },
        consumed,
    ))
}

fn preprocess_spectre_comments(
    path: &Path,
    source: &str,
) -> Result<Vec<String>, SpectreModelAdapterError> {
    let is_scs = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("scs"));
    let mut language = if is_scs {
        Language::Spectre
    } else {
        Language::Spice
    };
    let mut in_block_comment = false;
    let mut block_start_line = 0usize;
    let mut output = Vec::with_capacity(source.lines().count());
    for (line_index, raw) in source.lines().enumerate() {
        let line_number = line_index + 1;
        let mut boundary_block = false;
        let boundary_projection =
            strip_spectre_comments_from_line(raw, &mut boundary_block, line_number, None);
        let is_boundary = boundary_projection
            .trim()
            .to_ascii_lowercase()
            .starts_with("simulator lang=");
        let line = if language == Language::Spectre || in_block_comment || is_boundary {
            strip_spectre_comments_from_line(
                raw,
                &mut in_block_comment,
                line_number,
                Some(&mut block_start_line),
            )
        } else {
            raw.to_owned()
        };
        match line.trim().to_ascii_lowercase().as_str() {
            "simulator lang=spectre" => language = Language::Spectre,
            "simulator lang=spice" => language = Language::Spice,
            _ => {}
        }
        output.push(line);
    }
    if in_block_comment {
        return Err(error(
            block_start_line.max(1),
            "Spectre block comment has no closing '*/'",
        ));
    }
    Ok(output)
}

fn strip_spectre_comments_from_line(
    line: &str,
    in_block_comment: &mut bool,
    line_number: usize,
    mut block_start_line: Option<&mut usize>,
) -> String {
    let mut output = String::with_capacity(line.len());
    let mut characters = line.char_indices().peekable();
    let mut quote = None;
    let mut escaped = false;
    while let Some((_, character)) = characters.next() {
        let next = characters.peek().map(|(_, character)| *character);
        if *in_block_comment {
            if character == '*' && next == Some('/') {
                characters.next();
                *in_block_comment = false;
            }
            continue;
        }
        if let Some(active_quote) = quote {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == active_quote {
                quote = None;
            }
            continue;
        }
        if matches!(character, '\'' | '"') {
            quote = Some(character);
            output.push(character);
        } else if character == '/' && next == Some('/') {
            break;
        } else if character == '/' && next == Some('*') {
            characters.next();
            *in_block_comment = true;
            if let Some(start) = block_start_line.as_deref_mut() {
                *start = line_number;
            }
            if output
                .chars()
                .last()
                .is_some_and(|last| !last.is_whitespace())
            {
                output.push(' ');
            }
        } else {
            output.push(character);
        }
    }
    output
}

fn lower_statistics(
    statistics: &SpectreStatisticsBlock,
) -> Result<String, SpectreModelAdapterError> {
    let mut plan = super::SpectreStatisticsPlan::default();
    let mut names = HashSet::with_capacity(statistics.variations.len());
    for variation in &statistics.variations {
        if !valid_spice_identifier(&variation.parameter) {
            return Err(error(
                variation.line,
                format!(
                    "Spectre variation name '{}' is not a portable SPICE parameter identifier",
                    variation.parameter
                ),
            ));
        }
        if !names.insert((variation.scope, variation.parameter.to_ascii_lowercase())) {
            return Err(error(
                variation.line,
                format!(
                    "Spectre variation '{}' is declared more than once",
                    variation.parameter
                ),
            ));
        }

        let mut attributes = variation.attributes.clone();
        if let Some(mean) = take_assignment(&mut attributes, "mean") {
            return Err(error(
                variation.line,
                format!(
                    "Spectre variation '{}' uses nonstandard mean={}; the executable Spectre contract takes the current parameter value as its nominal mean",
                    variation.parameter, mean.value
                ),
            ));
        }
        let std = take_assignment(&mut attributes, "std")
            .or_else(|| take_assignment(&mut attributes, "sigma"));
        let half_range = take_assignment(&mut attributes, "N");
        let has_std = std.is_some();
        let has_half_range = half_range.is_some();
        let percent = take_assignment(&mut attributes, "percent")
            .map(|assignment| {
                let value = assignment
                    .value
                    .trim_matches(['\'', '"'])
                    .to_ascii_lowercase();
                match value.as_str() {
                    "yes" | "true" | "1" => Ok(true),
                    "no" | "false" | "0" => Ok(false),
                    _ => Err(error(
                        variation.line,
                        format!(
                            "Spectre variation '{}' percent= must be yes or no, found {}",
                            variation.parameter, assignment.value
                        ),
                    )),
                }
            })
            .transpose()?
            .unwrap_or(false);
        if !attributes.is_empty() {
            return Err(error(
                variation.line,
                format!(
                    "Spectre variation '{}' has unsupported executable attributes: {}",
                    variation.parameter,
                    attributes
                        .iter()
                        .map(|attribute| attribute.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ));
        }

        let (distribution, spread) = match variation.distribution.as_str() {
            "gauss" | "gaussian" | "normal" => (
                super::SpectreDistribution::Gaussian,
                super::SpectreSpread::StandardDeviation(
                    std.ok_or_else(|| {
                        error(
                            variation.line,
                            format!(
                                "Spectre Gaussian variation '{}' requires std=",
                                variation.parameter
                            ),
                        )
                    })?
                    .value,
                ),
            ),
            "unif" | "uniform" => (
                super::SpectreDistribution::Uniform,
                super::SpectreSpread::HalfRange(
                    half_range
                        .ok_or_else(|| {
                            error(
                                variation.line,
                                format!(
                                    "Spectre uniform variation '{}' requires N= (half range)",
                                    variation.parameter
                                ),
                            )
                        })?
                        .value,
                ),
            ),
            "lnorm" | "lognormal" => (
                super::SpectreDistribution::Lognormal,
                super::SpectreSpread::StandardDeviation(
                    std.ok_or_else(|| {
                        error(
                            variation.line,
                            format!(
                                "Spectre lognormal variation '{}' requires std=",
                                variation.parameter
                            ),
                        )
                    })?
                    .value,
                ),
            ),
            distribution => {
                return Err(error(
                    variation.line,
                    format!(
                        "Spectre variation '{}' uses unsupported executable distribution '{distribution}'",
                        variation.parameter,
                    ),
                ));
            }
        };
        if has_std && has_half_range {
            return Err(error(
                variation.line,
                format!(
                    "Spectre variation '{}' cannot declare both std= and N=",
                    variation.parameter
                ),
            ));
        }
        plan.variations.push(super::SpectreVariation {
            line: variation.line,
            scope: match variation.scope {
                SpectreVariationScope::Process => super::SpectreVariationScope::Process,
                SpectreVariationScope::Mismatch => super::SpectreVariationScope::Mismatch,
            },
            parameter: variation.parameter.clone(),
            distribution,
            spread,
            percent,
        });
    }
    for correlation in &statistics.correlations {
        plan.correlations.push(super::SpectreCorrelation {
            line: correlation.line,
            scope: super::SpectreVariationScope::Process,
            parameters: correlation.parameters.clone(),
            coefficient: correlation.coefficient.clone(),
        });
    }
    plan.validate_structure()
        .map_err(|failure| error(0, failure.to_string()))?;
    Ok(format!(
        "{} {}",
        super::spectre_statistics::SPECTRE_STATISTICS_DIRECTIVE,
        plan.encode_internal()
    ))
}

fn valid_spice_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

//=============================================================================
// Construct inventory
//=============================================================================

/// The namespace a Spectre name is resolved in.
///
/// The same word means different things in each: `d` is a diode model family
/// and also a diode instance master, `resistor` is both a master and a model
/// family, and `save` is only ever a statement keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpectreNamespace {
    /// First token of a top-level statement.
    Statement,
    /// Master of a named statement: an instance primitive or an analysis.
    Master,
    /// Device family named by a `model` statement.
    ModelFamily,
}

impl SpectreNamespace {
    const fn description(self) -> &'static str {
        match self {
            Self::Statement => "model-library statement",
            Self::Master => "instance master",
            Self::ModelFamily => "model family",
        }
    }
}

/// What this adapter does with a recognized Spectre construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpectreSupport {
    /// Lowered to a canonical SPICE card.
    Lowered,
    /// Recognized and deliberately refused. The text says why, and becomes
    /// the refusal a deck sees.
    Unsupported(&'static str),
    /// Preserved verbatim for a different effort, which the text names.
    OwnedElsewhere(&'static str),
}

/// Declare the construct inventory once.
///
/// The macro derives the enum and its `ALL` list from the same table, so the
/// list cannot fall behind the variants and every construct necessarily
/// declares a namespace, a support decision, and its spellings.
macro_rules! spectre_constructs {
    ($($variant:ident => ($namespace:expr, $support:expr, [$($name:literal),+ $(,)?] $(,)?)),+ $(,)?) => {
        /// Every Spectre statement keyword, instance master, and model family
        /// this adapter knows.
        ///
        /// This is the inventory the three catch-all refusals answer from: a
        /// name that is here is refused with the reason recorded beside it,
        /// and a name that is not here is refused as unknown. Nothing is
        /// silently discarded either way.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum SpectreConstruct {
            $($variant,)+
        }

        impl SpectreConstruct {
            const ALL: &'static [Self] = &[$(Self::$variant,)+];

            const fn namespace(self) -> SpectreNamespace {
                match self { $(Self::$variant => $namespace,)+ }
            }

            const fn support(self) -> SpectreSupport {
                match self { $(Self::$variant => $support,)+ }
            }

            /// Lowercase spellings Spectre accepts. The first is canonical.
            const fn names(self) -> &'static [&'static str] {
                match self { $(Self::$variant => &[$($name,)+],)+ }
            }
        }
    };
}

spectre_constructs! {
    // -- statements ---------------------------------------------------------
    StatementSimulator => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["simulator"]),
    StatementLibrary => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["library"]),
    StatementEndLibrary => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["endlibrary"]),
    StatementSection => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["section"]),
    StatementEndSection => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["endsection"]),
    StatementInclude => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["include"]),
    StatementModel => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["model"]),
    StatementParameters => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["parameters"]),
    StatementSubckt => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["subckt"]),
    StatementInline => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["inline"]),
    StatementEnds => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["ends"]),
    StatementGlobal => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["global"]),
    StatementStatistics => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["statistics"]),
    StatementSave => (SpectreNamespace::Statement, SpectreSupport::Lowered, ["save"]),
    StatementAhdlInclude => (
        SpectreNamespace::Statement,
        SpectreSupport::OwnedElsewhere("the separate Verilog-A/AHDL effort"),
        ["ahdl_include"],
    ),
    StatementSaveOptions => (
        SpectreNamespace::Statement,
        SpectreSupport::Unsupported(
            "Spectre saveOptions selects output scope with simulator-specific policies that have \
             no canonical .SAVE equivalent; name the signals with a save statement instead",
        ),
        ["saveoptions"],
    ),
    StatementSimulatorOptions => (
        SpectreNamespace::Statement,
        SpectreSupport::Unsupported(
            "Spectre simulatorOptions carries solver tolerances whose meanings are not RSpice \
             .OPTIONS meanings; accepting it would change numerical results without saying so",
        ),
        ["simulatoroptions"],
    ),
    StatementAlterGroup => (
        SpectreNamespace::Statement,
        SpectreSupport::Unsupported(
            "Spectre altergroup re-elaborates a deck variant; RSpice has no ALTER variant axis in \
             its deck plan yet",
        ),
        ["altergroup"],
    ),

    // -- instance masters ---------------------------------------------------
    MasterResistor => (SpectreNamespace::Master, SpectreSupport::Lowered, ["resistor", "res"]),
    MasterCapacitor => (SpectreNamespace::Master, SpectreSupport::Lowered, ["capacitor", "cap"]),
    MasterInductor => (SpectreNamespace::Master, SpectreSupport::Lowered, ["inductor", "ind"]),
    MasterVSource => (SpectreNamespace::Master, SpectreSupport::Lowered, ["vsource"]),
    MasterISource => (SpectreNamespace::Master, SpectreSupport::Lowered, ["isource"]),
    MasterVcvs => (SpectreNamespace::Master, SpectreSupport::Lowered, ["vcvs"]),
    MasterVccs => (SpectreNamespace::Master, SpectreSupport::Lowered, ["vccs"]),
    MasterBSource => (SpectreNamespace::Master, SpectreSupport::Lowered, ["bsource"]),
    MasterDiode => (SpectreNamespace::Master, SpectreSupport::Lowered, ["diode", "d"]),
    MasterMosfet => (SpectreNamespace::Master, SpectreSupport::Lowered, ["nmos", "pmos"]),
    MasterBjt => (SpectreNamespace::Master, SpectreSupport::Lowered, ["npn", "pnp"]),
    MasterJfet => (
        SpectreNamespace::Master,
        SpectreSupport::Lowered,
        ["njf", "pjf", "njfet", "pjfet"],
    ),
    MasterTran => (SpectreNamespace::Master, SpectreSupport::Lowered, ["tran"]),
    MasterDc => (SpectreNamespace::Master, SpectreSupport::Lowered, ["dc"]),
    MasterAc => (SpectreNamespace::Master, SpectreSupport::Lowered, ["ac"]),
    MasterNoise => (SpectreNamespace::Master, SpectreSupport::Lowered, ["noise"]),
    MasterSweep => (SpectreNamespace::Master, SpectreSupport::Lowered, ["sweep"]),
    MasterCcvs => (
        SpectreNamespace::Master,
        SpectreSupport::Unsupported(
            "a Spectre ccvs senses the branch formed by its own control terminals, while the \
             canonical H card senses a named voltage source; the adapter preserves one source \
             line per input line and may not synthesize the extra zero-volt probe",
        ),
        ["ccvs"],
    ),
    MasterCccs => (
        SpectreNamespace::Master,
        SpectreSupport::Unsupported(
            "a Spectre cccs senses the branch formed by its own control terminals, while the \
             canonical F card senses a named voltage source; the adapter preserves one source \
             line per input line and may not synthesize the extra zero-volt probe",
        ),
        ["cccs"],
    ),
    MasterSp => (
        SpectreNamespace::Master,
        SpectreSupport::Unsupported(
            "a Spectre sp analysis names its ports on the analysis statement, while .SP takes its \
             ports from P elements; no port mapping is defined",
        ),
        ["sp"],
    ),
    MasterPss => (
        SpectreNamespace::Master,
        SpectreSupport::OwnedElsewhere("the separate periodic/RF analysis-card work package"),
        ["pss"],
    ),
    MasterPac => (
        SpectreNamespace::Master,
        SpectreSupport::OwnedElsewhere("the separate periodic/RF analysis-card work package"),
        ["pac"],
    ),
    MasterPnoise => (
        SpectreNamespace::Master,
        SpectreSupport::OwnedElsewhere("the separate periodic/RF analysis-card work package"),
        ["pnoise"],
    ),
    MasterOptions => (
        SpectreNamespace::Master,
        SpectreSupport::Unsupported(
            "Spectre options statements carry solver tolerances whose meanings are not RSpice \
             .OPTIONS meanings; accepting them would change numerical results without saying so",
        ),
        ["options"],
    ),
    MasterInfo => (
        SpectreNamespace::Master,
        SpectreSupport::Unsupported(
            "a Spectre info statement requests a simulator-specific report artifact, not circuit \
             or analysis semantics",
        ),
        ["info"],
    ),
    MasterMonteCarlo => (
        SpectreNamespace::Master,
        SpectreSupport::Unsupported(
            "Spectre montecarlo scopes a nested analysis block; RSpice drives Monte Carlo from its \
             runner rather than from a netlist card",
        ),
        ["montecarlo"],
    ),

    // -- model families -----------------------------------------------------
    ModelDiode => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["d", "diode"]),
    ModelNpn => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["npn"]),
    ModelPnp => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["pnp"]),
    ModelNmos => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["nmos"]),
    ModelPmos => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["pmos"]),
    ModelNjf => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["njf", "njfet"]),
    ModelPjf => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["pjf", "pjfet"]),
    ModelResistor => (
        SpectreNamespace::ModelFamily,
        SpectreSupport::Lowered,
        ["r", "res", "resistor"],
    ),
    ModelCapacitor => (
        SpectreNamespace::ModelFamily,
        SpectreSupport::Lowered,
        ["c", "capacitor"],
    ),
    ModelMos1 => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["mos1"]),
    ModelMos2 => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["mos2"]),
    ModelMos3 => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["mos3"]),
    ModelMos6 => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["mos6"]),
    ModelBsim1 => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["bsim1"]),
    ModelBsim2 => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["bsim2"]),
    ModelBsim3 => (
        SpectreNamespace::ModelFamily,
        SpectreSupport::Lowered,
        ["bsim3", "bsim3v3"],
    ),
    ModelBsim4 => (
        SpectreNamespace::ModelFamily,
        SpectreSupport::Lowered,
        ["bsim4", "bsim4v8"],
    ),
    ModelBsimSoi => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["bsimsoi"]),
    ModelEkv => (
        SpectreNamespace::ModelFamily,
        SpectreSupport::Lowered,
        ["ekv", "ekv26"],
    ),
    ModelEkv3 => (SpectreNamespace::ModelFamily, SpectreSupport::Lowered, ["ekv3"]),
}

impl SpectreConstruct {
    /// Resolve one lowercase name inside a namespace.
    fn classify(namespace: SpectreNamespace, name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|construct| {
            construct.namespace() == namespace
                && construct
                    .names()
                    .iter()
                    .any(|candidate| candidate.eq_ignore_ascii_case(name))
        })
    }
}

/// The refusal for a name this adapter does not lower, answered from the
/// construct inventory so that a recognized-but-unsupported construct says
/// why and an unknown one says it is unknown.
fn unsupported_construct(
    namespace: SpectreNamespace,
    name: &str,
    line: usize,
) -> SpectreModelAdapterError {
    let description = namespace.description();
    match SpectreConstruct::classify(namespace, &name.to_ascii_lowercase())
        .map(SpectreConstruct::support)
    {
        Some(SpectreSupport::Unsupported(reason)) => error(
            line,
            format!("unsupported native Spectre {description} '{name}': {reason}"),
        ),
        Some(SpectreSupport::OwnedElsewhere(owner)) => error(
            line,
            format!(
                "unsupported native Spectre {description} '{name}': it is owned by {owner}; no statement was discarded"
            ),
        ),
        Some(SpectreSupport::Lowered) => error(
            line,
            format!(
                "native Spectre {description} '{name}' is lowered, but reached the refusal path; \
                 this is an adapter defect, not a deck error"
            ),
        ),
        None => error(
            line,
            format!("unknown native Spectre {description} '{name}'; no statement was discarded"),
        ),
    }
}

impl SpectreSymbols {
    fn from_statements(statements: &[SpectreStatement]) -> Result<Self, SpectreModelAdapterError> {
        let mut symbols = Self::default();
        for statement in statements {
            match &statement.kind {
                SpectreStatementKind::Model {
                    name,
                    canonical_type,
                    source_family,
                    ..
                } => {
                    let key = name.to_ascii_lowercase();
                    if source_family.eq_ignore_ascii_case("bsimsoi") {
                        symbols.bsimsoi_models.insert(key.clone());
                    }
                    if let Some(existing) = symbols.models.insert(key, canonical_type.clone())
                        && !existing.eq_ignore_ascii_case(canonical_type)
                    {
                        return Err(error(
                            statement.line,
                            format!(
                                "Spectre model '{name}' changes device family from '{existing}' to '{canonical_type}' across sections"
                            ),
                        ));
                    }
                }
                SpectreStatementKind::Subcircuit { name, .. } => {
                    symbols.subcircuits.insert(name.to_ascii_lowercase());
                }
                SpectreStatementKind::Instance(instance) => {
                    let prefix = match instance.master.to_ascii_lowercase().as_str() {
                        "vsource" => "V",
                        "isource" => "I",
                        _ => continue,
                    };
                    symbols.sources.insert(
                        instance.name.to_ascii_lowercase(),
                        format!("{prefix}{}", instance.name),
                    );
                }
                SpectreStatementKind::Lowered(_) | SpectreStatementKind::Statistics(_) => {}
            }
        }
        Ok(symbols)
    }
}

fn lowered_statement(line: usize, lowered: String) -> SpectreStatement {
    SpectreStatement {
        line,
        consumed_lines: 1,
        kind: SpectreStatementKind::Lowered(lowered),
    }
}

fn adapted_model_identity(
    adapted: &str,
    line: usize,
) -> Result<(String, String), SpectreModelAdapterError> {
    let mut fields = adapted.split_whitespace();
    if fields.next() != Some(".model") {
        return Err(error(
            line,
            "internal Spectre model lowering lost its directive",
        ));
    }
    let name = fields
        .next()
        .ok_or_else(|| error(line, "internal Spectre model lowering lost its name"))?;
    let canonical_type = fields
        .next()
        .ok_or_else(|| error(line, "internal Spectre model lowering lost its type"))?;
    Ok((name.to_owned(), canonical_type.to_owned()))
}

fn collect_continued_statement(lines: &[&str], start: usize, plus_lines: bool) -> (String, usize) {
    let mut logical = String::new();
    let mut consumed = 0usize;
    let mut backslash_continuation = true;
    while let Some(raw) = lines.get(start + consumed) {
        let trimmed = raw.trim();
        if consumed > 0 && !backslash_continuation && !(plus_lines && trimmed.starts_with('+')) {
            break;
        }
        let without_plus = if consumed > 0 && plus_lines {
            trimmed.strip_prefix('+').unwrap_or(trimmed).trim_start()
        } else {
            trimmed
        };
        backslash_continuation = without_plus.ends_with('\\');
        let fragment = without_plus.trim_end_matches('\\').trim_end();
        if !logical.is_empty() && !fragment.is_empty() {
            logical.push(' ');
        }
        logical.push_str(fragment);
        consumed += 1;
        if !backslash_continuation {
            let next_is_plus = plus_lines
                && lines
                    .get(start + consumed)
                    .is_some_and(|line| line.trim_start().starts_with('+'));
            if !next_is_plus {
                break;
            }
        }
    }
    (logical, consumed.max(1))
}

fn adapt_subcircuit(rest: &str, line: usize) -> Result<(String, String), SpectreModelAdapterError> {
    let (name, remainder) =
        take_token(rest).ok_or_else(|| error(line, "Spectre subckt declaration has no name"))?;
    let (ports, parameters) = if remainder.trim_start().starts_with('(') {
        let (ports, tail) = consume_parenthesized(remainder, line, "subckt port list")?;
        (split_node_list(&ports, line)?, tail)
    } else {
        let assignments_start = find_assignment_start(remainder);
        let (ports, tail) = remainder.split_at(assignments_start.unwrap_or(remainder.len()));
        (split_node_list(ports, line)?, tail)
    };
    if ports.is_empty() {
        return Err(error(
            line,
            format!("Spectre subckt '{name}' has no formal ports"),
        ));
    }
    let defaults = parse_spectre_model_assignments(parameters.trim(), line)?;
    let mut lowered = format!(".subckt {name} {}", ports.join(" "));
    if !defaults.is_empty() {
        lowered.push_str(" PARAMS: ");
        lowered.push_str(&render_assignments(&defaults));
    }
    Ok((name.to_owned(), lowered))
}

fn parse_spectre_instance(
    logical: &str,
    line: usize,
) -> Result<SpectreInstance, SpectreModelAdapterError> {
    let (name, remainder) = take_token(logical)
        .ok_or_else(|| error(line, "Spectre instance declaration has no name"))?;
    // An analysis statement is spelled like an instance without terminals
    // (`tran1 tran stop=1m`), so an absent node list is structural rather than
    // an error. Each master enforces its own arity below.
    let (nodes, remainder) = if remainder.trim_start().starts_with('(') {
        let (nodes, remainder) = consume_parenthesized(remainder, line, "instance node list")?;
        (split_node_list(&nodes, line)?, remainder)
    } else {
        (Vec::new(), remainder)
    };
    let (master, parameters) = take_token(remainder)
        .ok_or_else(|| error(line, format!("Spectre instance '{name}' has no master")))?;
    Ok(SpectreInstance {
        name: name.to_owned(),
        nodes,
        master: master.to_owned(),
        parameters: parse_spectre_model_assignments(parameters, line)?,
    })
}

fn consume_parenthesized<'a>(
    input: &'a str,
    line: usize,
    description: &str,
) -> Result<(String, &'a str), SpectreModelAdapterError> {
    let input = input.trim_start();
    if !input.starts_with('(') {
        return Err(error(
            line,
            format!("Spectre {description} must be parenthesized"),
        ));
    }
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    for (offset, character) in input.char_indices() {
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == active_quote {
                quote = None;
            }
            continue;
        }
        if matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if character == '(' {
            depth += 1;
        } else if character == ')' {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Ok((input[1..offset].to_owned(), input[offset + 1..].trim()));
            }
        }
    }
    Err(error(
        line,
        format!("Spectre {description} has no closing ')'"),
    ))
}

fn split_node_list(value: &str, line: usize) -> Result<Vec<String>, SpectreModelAdapterError> {
    let nodes = value
        .split(|character: char| character.is_whitespace() || character == ',')
        .filter(|node| !node.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if nodes.iter().any(|node| node.contains('=')) {
        return Err(error(
            line,
            "Spectre node list contains a parameter assignment",
        ));
    }
    Ok(nodes)
}

fn find_assignment_start(value: &str) -> Option<usize> {
    value
        .split_whitespace()
        .find(|token| token.contains('='))
        .and_then(|token| value.find(token))
}

fn lower_spectre_instance(
    instance: &SpectreInstance,
    symbols: &SpectreSymbols,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    let master = instance.master.to_ascii_lowercase();
    match master.as_str() {
        "resistor" | "res" => lower_two_terminal_primitive(instance, "R", "r", line),
        "capacitor" | "cap" => lower_two_terminal_primitive(instance, "C", "c", line),
        "inductor" | "ind" => lower_two_terminal_primitive(instance, "L", "l", line),
        "vsource" => lower_independent_source(instance, "V", line),
        "isource" => lower_independent_source(instance, "I", line),
        "vcvs" => lower_voltage_controlled_source(instance, "E", "gain", line),
        "vccs" => lower_voltage_controlled_source(instance, "G", "gm", line),
        "bsource" => lower_behavioral_source(instance, line),
        "tran" | "dc" | "ac" | "noise" | "sweep" => lower_spectre_analysis(instance, symbols, line),
        _ if symbols.subcircuits.contains(&master) => {
            Ok(render_model_or_subcircuit_instance(instance, "X"))
        }
        _ if symbols.models.contains_key(&master) => {
            let canonical = &symbols.models[&master];
            if symbols.bsimsoi_models.contains(&master) && !matches!(instance.nodes.len(), 4 | 5) {
                return Err(error(
                    line,
                    format!(
                        "Spectre BSIMSOI instance '{}' has {} nodes; the native BSIM3-SOI route supports exactly four terminals or five terminals with a body contact, while six/seven-terminal forms are not yet represented",
                        instance.name,
                        instance.nodes.len()
                    ),
                ));
            }
            let prefix = canonical_instance_prefix(canonical).ok_or_else(|| {
                error(
                    line,
                    format!(
                        "Spectre model '{}' lowers to unsupported instance family '{canonical}'",
                        instance.master
                    ),
                )
            })?;
            Ok(render_model_or_subcircuit_instance(instance, prefix))
        }
        "diode" | "d" => Ok(render_model_or_subcircuit_instance(instance, "D")),
        "nmos" | "pmos" => Ok(render_model_or_subcircuit_instance(instance, "M")),
        "npn" | "pnp" => Ok(render_model_or_subcircuit_instance(instance, "Q")),
        "njf" | "pjf" | "njfet" | "pjfet" => Ok(render_model_or_subcircuit_instance(instance, "J")),
        _ => Err(unsupported_construct(
            SpectreNamespace::Master,
            &instance.master,
            line,
        )),
    }
}

/// Lower `vcvs`/`vccs`, whose four terminals are the two output nodes
/// followed by the two control nodes — the same order the canonical `E` and
/// `G` cards use.
fn lower_voltage_controlled_source(
    instance: &SpectreInstance,
    prefix: &str,
    value_name: &str,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    if instance.nodes.len() != 4 {
        return Err(error(
            line,
            format!(
                "Spectre {} instance '{}' requires exactly four nodes (out+ out- control+ control-), got {}",
                instance.master,
                instance.name,
                instance.nodes.len()
            ),
        ));
    }
    let mut parameters = instance.parameters.clone();
    let value = take_assignment(&mut parameters, value_name).ok_or_else(|| {
        error(
            line,
            format!(
                "Spectre {} instance '{}' requires {value_name}=",
                instance.master, instance.name
            ),
        )
    })?;
    reject_remaining_parameters(&parameters, instance, line)?;
    Ok(format!(
        "{prefix}{} {} {}",
        instance.name,
        instance.nodes.join(" "),
        value.value
    ))
}

/// Lower `bsource`, whose behavioral equation is the canonical `B` card's.
fn lower_behavioral_source(
    instance: &SpectreInstance,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    if instance.nodes.len() != 2 {
        return Err(error(
            line,
            format!(
                "Spectre bsource instance '{}' requires exactly two nodes, got {}",
                instance.name,
                instance.nodes.len()
            ),
        ));
    }
    let mut parameters = instance.parameters.clone();
    let voltage = take_assignment(&mut parameters, "v");
    let current = take_assignment(&mut parameters, "i");
    let (kind, equation) = match (voltage, current) {
        (Some(voltage), None) => ("V", voltage.value),
        (None, Some(current)) => ("I", current.value),
        (Some(_), Some(_)) => {
            return Err(error(
                line,
                format!(
                    "Spectre bsource instance '{}' declares both v= and i=; a behavioral source is one or the other",
                    instance.name
                ),
            ));
        }
        (None, None) => {
            return Err(error(
                line,
                format!(
                    "Spectre bsource instance '{}' requires v= or i=",
                    instance.name
                ),
            ));
        }
    };
    reject_remaining_parameters(&parameters, instance, line)?;
    Ok(format!(
        "B{} {} {kind}={{{}}}",
        instance.name,
        instance.nodes.join(" "),
        equation.trim_matches(['\'', '"'])
    ))
}

/// Lower a Spectre analysis statement to its canonical analysis card.
///
/// Spectre analyses carry simulator-control parameters beside the ones that
/// define the analysis. Only the output-artifact and progress controls listed
/// in [`ANALYSIS_TOOL_CONTROLS`] are dropped; anything else — a tolerance
/// preset, an iteration limit, an integration method — would change results
/// without saying so, and is refused by name.
fn lower_spectre_analysis(
    instance: &SpectreInstance,
    symbols: &SpectreSymbols,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    let master = instance.master.to_ascii_lowercase();
    let mut parameters = instance.parameters.clone();
    for control in ANALYSIS_TOOL_CONTROLS {
        let _ = take_assignment(&mut parameters, control);
    }
    let lowered = match master.as_str() {
        "tran" => lower_spectre_transient(instance, &mut parameters, line)?,
        "dc" => lower_spectre_dc(instance, symbols, &mut parameters, line)?,
        "ac" => lower_spectre_ac(instance, &mut parameters, line)?,
        "noise" => lower_spectre_noise(instance, symbols, &mut parameters, line)?,
        "sweep" => lower_spectre_sweep(instance, &mut parameters, line)?,
        _ => {
            return Err(unsupported_construct(
                SpectreNamespace::Master,
                &instance.master,
                line,
            ));
        }
    };
    reject_remaining_parameters(&parameters, instance, line)?;
    Ok(lowered)
}

/// Spectre analysis parameters that select output artifacts or progress
/// reporting. They name files and console annotation, never circuit or
/// analysis semantics, so dropping them cannot change a result.
const ANALYSIS_TOOL_CONTROLS: &[&str] = &["write", "writefinal", "annotate", "title"];

fn lower_spectre_transient(
    instance: &SpectreInstance,
    parameters: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    require_no_nodes(instance, line)?;
    let stop = required_assignment(parameters, "stop", line, instance)?;
    let step = optional_assignment(parameters, "step", "0");
    let start = take_assignment(parameters, "start").map(|assignment| assignment.value);
    let maxstep = take_assignment(parameters, "maxstep").map(|assignment| assignment.value);
    let mut lowered = format!(".TRAN {step} {stop}");
    match (start, maxstep) {
        (None, None) => {}
        (Some(start), None) => lowered.push_str(&format!(" {start}")),
        // The canonical card positions the maximum step fourth, so an
        // explicit start must be written even when the deck left it default.
        (start, Some(maxstep)) => {
            let start = start.unwrap_or_else(|| "0".to_owned());
            lowered.push_str(&format!(" {start} {maxstep}"));
        }
    }
    Ok(lowered)
}

fn lower_spectre_dc(
    instance: &SpectreInstance,
    symbols: &SpectreSymbols,
    parameters: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    require_no_nodes(instance, line)?;
    if let Some(parameter) = take_assignment(parameters, "param") {
        return Err(error(
            line,
            format!(
                "Spectre dc analysis '{}' sweeps parameter '{}'; the canonical .DC card sweeps a source, so author .STEP for a parameter sweep",
                instance.name, parameter.value
            ),
        ));
    }
    let device = take_assignment(parameters, "dev");
    let start = take_assignment(parameters, "start").map(|assignment| assignment.value);
    let stop = take_assignment(parameters, "stop").map(|assignment| assignment.value);
    let step = take_assignment(parameters, "step").map(|assignment| assignment.value);
    let Some(device) = device else {
        if start.is_some() || stop.is_some() || step.is_some() {
            return Err(error(
                line,
                format!(
                    "Spectre dc analysis '{}' has a sweep range but names no dev= to sweep",
                    instance.name
                ),
            ));
        }
        // A Spectre dc analysis with no sweep is an operating point.
        return Ok(".OP".to_owned());
    };
    let (Some(start), Some(stop), Some(step)) = (start, stop, step) else {
        return Err(error(
            line,
            format!(
                "Spectre dc analysis '{}' sweeps '{}' but does not give start=, stop= and step=",
                instance.name, device.value
            ),
        ));
    };
    let swept = symbols
        .sources
        .get(&device.value.to_ascii_lowercase())
        .ok_or_else(|| {
            error(
                line,
                format!(
                    "Spectre dc analysis '{}' sweeps dev='{}', which is not an independent source declared in this source",
                    instance.name, device.value
                ),
            )
        })?;
    Ok(format!(".DC LIN {swept} {start} {stop} {step}"))
}

fn lower_spectre_ac(
    instance: &SpectreInstance,
    parameters: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    require_no_nodes(instance, line)?;
    let start = required_assignment(parameters, "start", line, instance)?;
    let stop = required_assignment(parameters, "stop", line, instance)?;
    let (variation, points) = spectre_frequency_variation(instance, parameters, line)?;
    Ok(format!(".AC {variation} {points} {start} {stop}"))
}

fn lower_spectre_noise(
    instance: &SpectreInstance,
    symbols: &SpectreSymbols,
    parameters: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    if instance.nodes.len() != 2 {
        return Err(error(
            line,
            format!(
                "Spectre noise analysis '{}' must name its output node pair as `({}  p n)`; the canonical .NOISE card measures a node pair, and probe-instance output has no defined mapping",
                instance.name, instance.name
            ),
        ));
    }
    let start = required_assignment(parameters, "start", line, instance)?;
    let stop = required_assignment(parameters, "stop", line, instance)?;
    let (variation, points) = spectre_frequency_variation(instance, parameters, line)?;
    let input = take_assignment(parameters, "iprobe").ok_or_else(|| {
        error(
            line,
            format!(
                "Spectre noise analysis '{}' requires iprobe= to name its input source",
                instance.name
            ),
        )
    })?;
    let source = symbols
        .sources
        .get(&input.value.to_ascii_lowercase())
        .ok_or_else(|| {
            error(
                line,
                format!(
                    "Spectre noise analysis '{}' names iprobe='{}', which is not an independent source declared in this source",
                    instance.name, input.value
                ),
            )
        })?;
    Ok(format!(
        ".NOISE V({},{}) {source} {variation} {points} {start} {stop}",
        instance.nodes[0], instance.nodes[1]
    ))
}

fn lower_spectre_sweep(
    instance: &SpectreInstance,
    parameters: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    require_no_nodes(instance, line)?;
    let parameter = take_assignment(parameters, "param").ok_or_else(|| {
        error(
            line,
            format!(
                "Spectre sweep '{}' requires param=; a dev= sweep is a dc analysis",
                instance.name
            ),
        )
    })?;
    let start = required_assignment(parameters, "start", line, instance)?;
    let stop = required_assignment(parameters, "stop", line, instance)?;
    let step = required_assignment(parameters, "step", line, instance)?;
    Ok(format!(
        ".STEP PARAM {} {start} {stop} {step}",
        parameter.value
    ))
}

/// Read a Spectre frequency-sweep density as a canonical variation keyword
/// and point count.
fn spectre_frequency_variation(
    instance: &SpectreInstance,
    parameters: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<(&'static str, String), SpectreModelAdapterError> {
    let decade = take_assignment(parameters, "dec");
    let linear = take_assignment(parameters, "lin");
    match (decade, linear) {
        (Some(decade), None) => Ok(("DEC", decade.value)),
        (None, Some(linear)) => Ok(("LIN", linear.value)),
        (Some(_), Some(_)) => Err(error(
            line,
            format!(
                "Spectre {} analysis '{}' declares both dec= and lin=",
                instance.master, instance.name
            ),
        )),
        (None, None) => Err(error(
            line,
            format!(
                "Spectre {} analysis '{}' requires dec= or lin=; a log= or values= sweep density has no canonical equivalent",
                instance.master, instance.name
            ),
        )),
    }
}

fn require_no_nodes(
    instance: &SpectreInstance,
    line: usize,
) -> Result<(), SpectreModelAdapterError> {
    if instance.nodes.is_empty() {
        return Ok(());
    }
    Err(error(
        line,
        format!(
            "Spectre {} analysis '{}' does not take a node list",
            instance.master, instance.name
        ),
    ))
}

fn reject_remaining_parameters(
    parameters: &[SpectreModelAssignment],
    instance: &SpectreInstance,
    line: usize,
) -> Result<(), SpectreModelAdapterError> {
    if parameters.is_empty() {
        return Ok(());
    }
    Err(error(
        line,
        format!(
            "Spectre {} statement '{}' has unsupported parameters: {}",
            instance.master,
            instance.name,
            parameters
                .iter()
                .map(|parameter| parameter.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    ))
}

/// Lower a Spectre `save` statement to the canonical `.SAVE` card.
///
/// Spectre names plain nets, terminal currents (`R1:1`), hierarchical paths
/// and wildcard scopes in the same list. Only plain nets have a canonical
/// `.SAVE` spelling; the rest are refused by entry rather than dropped, which
/// would silently narrow the requested output set.
fn adapt_save(rest: &str, line: usize) -> Result<String, SpectreModelAdapterError> {
    let mut signals = Vec::new();
    for entry in rest
        .split(|character: char| character.is_whitespace() || character == ',')
        .filter(|entry| !entry.is_empty())
    {
        if entry.contains('=') {
            return Err(error(
                line,
                format!(
                    "Spectre save option '{entry}' selects an output scope rather than a signal; the canonical .SAVE card names signals"
                ),
            ));
        }
        if entry.contains([':', '.', '*', '?', '[', ']']) {
            return Err(error(
                line,
                format!(
                    "Spectre save entry '{entry}' names a terminal current, a hierarchical path or a wildcard scope; the canonical .SAVE card has no equivalent selector and dropping the entry would narrow the requested output"
                ),
            ));
        }
        signals.push(format!("V({entry})"));
    }
    if signals.is_empty() {
        return Err(error(line, "Spectre save statement names no signals"));
    }
    Ok(format!(".SAVE {}", signals.join(" ")))
}

fn lower_two_terminal_primitive(
    instance: &SpectreInstance,
    prefix: &str,
    value_name: &str,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    if instance.nodes.len() != 2 {
        return Err(error(
            line,
            format!(
                "Spectre {} instance '{}' requires exactly two nodes",
                instance.master, instance.name
            ),
        ));
    }
    let mut parameters = instance.parameters.clone();
    let value = take_assignment(&mut parameters, value_name).ok_or_else(|| {
        error(
            line,
            format!(
                "Spectre {} instance '{}' requires {}=",
                instance.master, instance.name, value_name
            ),
        )
    })?;
    let mut lowered = format!(
        "{prefix}{} {} {}",
        instance.name,
        instance.nodes.join(" "),
        value.value
    );
    if !parameters.is_empty() {
        lowered.push(' ');
        lowered.push_str(&render_assignments(&parameters));
    }
    Ok(lowered)
}

fn render_model_or_subcircuit_instance(instance: &SpectreInstance, prefix: &str) -> String {
    let mut lowered = format!(
        "{prefix}{} {} {}",
        instance.name,
        instance.nodes.join(" "),
        instance.master
    );
    if !instance.parameters.is_empty() {
        lowered.push(' ');
        lowered.push_str(&render_assignments(&instance.parameters));
    }
    lowered
}

fn canonical_instance_prefix(canonical_type: &str) -> Option<&'static str> {
    match canonical_type.to_ascii_uppercase().as_str() {
        "R" => Some("R"),
        "C" => Some("C"),
        "L" => Some("L"),
        "D" => Some("D"),
        "NPN" | "PNP" => Some("Q"),
        "NMOS" | "PMOS" => Some("M"),
        "NJF" | "PJF" => Some("J"),
        _ => None,
    }
}

fn lower_independent_source(
    instance: &SpectreInstance,
    prefix: &str,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    if instance.nodes.len() != 2 {
        return Err(error(
            line,
            format!(
                "Spectre {} instance '{}' requires exactly two nodes",
                instance.master, instance.name
            ),
        ));
    }
    let mut parameters = instance.parameters.clone();
    let source_type = take_assignment(&mut parameters, "type")
        .map(|value| value.value.trim_matches(['\'', '"']).to_ascii_lowercase())
        .unwrap_or_else(|| "dc".to_owned());
    let dc = take_assignment(&mut parameters, "dc").map(|value| value.value);
    let waveform = match source_type.as_str() {
        "dc" => dc.unwrap_or_else(|| "0".to_owned()),
        "sine" | "sin" => {
            let offset = take_assignment(&mut parameters, "sinedc")
                .map(|value| value.value)
                .or(dc)
                .unwrap_or_else(|| "0".to_owned());
            let amplitude = required_assignment(&mut parameters, "ampl", line, instance)?;
            let frequency = required_assignment(&mut parameters, "freq", line, instance)?;
            let delay = optional_assignment(&mut parameters, "delay", "0");
            let damping = optional_assignment(&mut parameters, "damp", "0");
            let phase = optional_assignment(&mut parameters, "sinephase", "0");
            format!("SIN({offset} {amplitude} {frequency} {delay} {damping} {phase})")
        }
        "pulse" => {
            let initial = required_assignment(&mut parameters, "val0", line, instance)?;
            let pulsed = required_assignment(&mut parameters, "val1", line, instance)?;
            let delay = optional_assignment(&mut parameters, "delay", "0");
            let rise = required_assignment(&mut parameters, "rise", line, instance)?;
            let fall = required_assignment(&mut parameters, "fall", line, instance)?;
            let width = required_assignment(&mut parameters, "width", line, instance)?;
            let period = required_assignment(&mut parameters, "period", line, instance)?;
            format!("PULSE({initial} {pulsed} {delay} {rise} {fall} {width} {period})")
        }
        "exp" => {
            let initial = required_assignment(&mut parameters, "val0", line, instance)?;
            let pulsed = required_assignment(&mut parameters, "val1", line, instance)?;
            let td1 = required_assignment(&mut parameters, "td1", line, instance)?;
            let tau1 = required_assignment(&mut parameters, "tau1", line, instance)?;
            let td2 = required_assignment(&mut parameters, "td2", line, instance)?;
            let tau2 = required_assignment(&mut parameters, "tau2", line, instance)?;
            format!("EXP({initial} {pulsed} {td1} {tau1} {td2} {tau2})")
        }
        _ => {
            return Err(error(
                line,
                format!(
                    "Spectre {} instance '{}' uses unsupported source type '{source_type}'",
                    instance.master, instance.name
                ),
            ));
        }
    };

    let ac_magnitude = take_assignment(&mut parameters, "mag").map(|value| value.value);
    let ac_phase = take_assignment(&mut parameters, "phase").map(|value| value.value);
    if !parameters.is_empty() {
        return Err(error(
            line,
            format!(
                "Spectre {} instance '{}' has unsupported parameters: {}",
                instance.master,
                instance.name,
                parameters
                    .iter()
                    .map(|parameter| parameter.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    }
    let mut lowered = format!(
        "{prefix}{} {} {waveform}",
        instance.name,
        instance.nodes.join(" ")
    );
    if let Some(magnitude) = ac_magnitude {
        lowered.push_str(&format!(" AC {magnitude}"));
        if let Some(phase) = ac_phase {
            lowered.push(' ');
            lowered.push_str(&phase);
        }
    } else if ac_phase.is_some() {
        return Err(error(
            line,
            format!(
                "Spectre {} instance '{}' declares phase= without mag=",
                instance.master, instance.name
            ),
        ));
    }
    Ok(lowered)
}

fn required_assignment(
    assignments: &mut Vec<SpectreModelAssignment>,
    name: &str,
    line: usize,
    instance: &SpectreInstance,
) -> Result<String, SpectreModelAdapterError> {
    take_assignment(assignments, name)
        .map(|assignment| assignment.value)
        .ok_or_else(|| {
            error(
                line,
                format!(
                    "Spectre {} instance '{}' requires {name}= for its selected waveform",
                    instance.master, instance.name
                ),
            )
        })
}

fn optional_assignment(
    assignments: &mut Vec<SpectreModelAssignment>,
    name: &str,
    default: &str,
) -> String {
    take_assignment(assignments, name)
        .map(|assignment| assignment.value)
        .unwrap_or_else(|| default.to_owned())
}

fn render_assignments(assignments: &[SpectreModelAssignment]) -> String {
    assignments
        .iter()
        .map(|assignment| format!("{}={}", assignment.name, assignment.value))
        .collect::<Vec<_>>()
        .join(" ")
}

fn adapt_ahdl_include(rest: &str, line: usize) -> Result<String, SpectreModelAdapterError> {
    let rest = rest.trim();
    let (path, remainder) = if let Some(quote) = rest
        .chars()
        .next()
        .filter(|character| matches!(character, '\'' | '"'))
    {
        let quoted = &rest[quote.len_utf8()..];
        let end = quoted
            .find(quote)
            .ok_or_else(|| error(line, "Spectre ahdl_include path has no closing quote"))?;
        (&quoted[..end], quoted[end + quote.len_utf8()..].trim())
    } else {
        take_token(rest).ok_or_else(|| error(line, "Spectre ahdl_include has no path"))?
    };
    if !remainder.is_empty() {
        return Err(error(
            line,
            "Spectre ahdl_include contains unsupported trailing syntax",
        ));
    }
    Ok(format!(".veriloga \"{path}\""))
}

fn adapt_model(
    lines: &[&str],
    start: usize,
) -> Result<(String, String, usize), SpectreModelAdapterError> {
    let line_number = start + 1;
    let first = lines[start].trim();
    let (_, rest) = split_head(first);
    let (name, rest) = take_token(rest)
        .ok_or_else(|| error(line_number, "Spectre model declaration has no name"))?;
    let (model_type, parameters) = take_token(rest)
        .ok_or_else(|| error(line_number, "Spectre model declaration has no model type"))?;
    let mut parameters = parameters.to_owned();
    let mut consumed = 1usize;
    let mut brace_depth = brace_delta(&parameters);
    if brace_depth < 0 {
        return Err(error(
            line_number,
            "Spectre model declaration closes an unopened brace",
        ));
    }

    loop {
        let Some(next) = lines.get(start + consumed) else {
            if brace_depth > 0 {
                return Err(error(
                    line_number,
                    "Spectre model declaration has an unterminated parameter block",
                ));
            }
            break;
        };
        let trimmed = next.trim();
        let fragment = if brace_depth > 0 {
            trimmed
        } else if let Some(fragment) = trimmed.strip_prefix('+') {
            fragment.trim_start()
        } else {
            break;
        };

        if !parameters.trim_end().is_empty() && !fragment.is_empty() {
            parameters.push(' ');
        }
        parameters.push_str(fragment);
        consumed += 1;
        brace_depth += brace_delta(fragment);
        if brace_depth < 0 {
            return Err(error(
                start + consumed,
                "Spectre model declaration closes too many braces",
            ));
        }
    }

    Ok((
        render_model(name, model_type, &parameters, line_number)?,
        model_type.to_ascii_lowercase(),
        consumed,
    ))
}

fn render_model(
    name: &str,
    model_type: &str,
    parameters: &str,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    let parameters = parameters.trim().trim_end_matches(';').trim();
    let parameters = parameters
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .unwrap_or(parameters)
        .trim();
    let mut assignments = parse_spectre_model_assignments(parameters, line)?;
    let canonical_type = canonical_model_type(model_type, &mut assignments, line)?;
    if assignments.is_empty() {
        Ok(format!(".model {name} {canonical_type}"))
    } else {
        Ok(format!(
            ".model {name} {canonical_type} ( {} )",
            assignments
                .iter()
                .map(|assignment| format!("{}={}", assignment.name, assignment.value))
                .collect::<Vec<_>>()
                .join(" ")
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpectreModelAssignment {
    name: String,
    value: String,
}

fn parse_spectre_model_assignments(
    parameters: &str,
    line: usize,
) -> Result<Vec<SpectreModelAssignment>, SpectreModelAdapterError> {
    let bytes = parameters.as_bytes();
    let mut assignments = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        while index < bytes.len() && (bytes[index].is_ascii_whitespace() || bytes[index] == b',') {
            index += 1;
        }
        if index == bytes.len() {
            break;
        }
        let name_start = index;
        while index < bytes.len()
            && (bytes[index].is_ascii_alphanumeric() || matches!(bytes[index], b'_' | b'.' | b'$'))
        {
            index += 1;
        }
        if index == name_start {
            return Err(error(
                line,
                "Spectre model parameter name contains unsupported syntax",
            ));
        }
        let name = &parameters[name_start..index];
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if bytes.get(index) != Some(&b'=') {
            return Err(error(
                line,
                format!("Spectre model parameter '{name}' has no '=' assignment"),
            ));
        }
        index += 1;
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index == bytes.len() {
            return Err(error(
                line,
                format!("Spectre model parameter '{name}' has an empty value"),
            ));
        }
        let value_start = index;
        let opening = bytes[index];
        if matches!(opening, b'\'' | b'"') {
            index = scan_quoted_value(bytes, index, opening, line, name)?;
        } else if matches!(opening, b'{' | b'(' | b'[') {
            let closing = match opening {
                b'{' => b'}',
                b'(' => b')',
                b'[' => b']',
                _ => unreachable!("grouped Spectre value opening was validated"),
            };
            index = scan_balanced_value(bytes, index, opening, closing, line, name)?;
        } else {
            while index < bytes.len() && !bytes[index].is_ascii_whitespace() && bytes[index] != b','
            {
                index += 1;
            }
        }
        if index == value_start {
            return Err(error(
                line,
                format!("Spectre model parameter '{name}' has an empty value"),
            ));
        }
        if assignments
            .iter()
            .any(|assignment: &SpectreModelAssignment| assignment.name.eq_ignore_ascii_case(name))
        {
            return Err(error(
                line,
                format!("Spectre model parameter '{name}' is declared more than once"),
            ));
        }
        assignments.push(SpectreModelAssignment {
            name: name.to_owned(),
            value: parameters[value_start..index].to_owned(),
        });
    }
    Ok(assignments)
}

fn scan_quoted_value(
    bytes: &[u8],
    start: usize,
    quote: u8,
    line: usize,
    name: &str,
) -> Result<usize, SpectreModelAdapterError> {
    let mut index = start + 1;
    let mut escaped = false;
    while index < bytes.len() {
        let byte = bytes[index];
        index += 1;
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == quote {
            return Ok(index);
        }
    }
    Err(error(
        line,
        format!("Spectre model parameter '{name}' has an unterminated quoted value"),
    ))
}

fn scan_balanced_value(
    bytes: &[u8],
    start: usize,
    opening: u8,
    closing: u8,
    line: usize,
    name: &str,
) -> Result<usize, SpectreModelAdapterError> {
    let mut index = start;
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    while index < bytes.len() {
        let byte = bytes[index];
        index += 1;
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == active_quote {
                quote = None;
            }
            continue;
        }
        if matches!(byte, b'\'' | b'"') {
            quote = Some(byte);
        } else if byte == opening {
            depth += 1;
        } else if byte == closing {
            depth -= 1;
            if depth == 0 {
                return Ok(index);
            }
        }
    }
    Err(error(
        line,
        format!("Spectre model parameter '{name}' has an unterminated grouped value"),
    ))
}

fn canonical_model_type(
    model_type: &str,
    assignments: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    let model_type_lower = model_type.to_ascii_lowercase();
    let direct = match model_type_lower.as_str() {
        "d" | "diode" => Some("D"),
        "npn" => Some("NPN"),
        "pnp" => Some("PNP"),
        "nmos" => Some("NMOS"),
        "pmos" => Some("PMOS"),
        "njf" | "njfet" => Some("NJF"),
        "pjf" | "pjfet" => Some("PJF"),
        "r" | "res" | "resistor" => Some("R"),
        "c" | "capacitor" => Some("C"),
        _ => None,
    };
    if let Some(direct) = direct {
        validate_and_remove_redundant_polarity(direct, assignments, line)?;
        return Ok(direct.to_owned());
    }

    let level = match model_type_lower.as_str() {
        "mos1" => Some(1),
        "mos2" => Some(2),
        "mos3" => Some(3),
        "mos6" => Some(6),
        "bsim1" => Some(4),
        "bsim2" => Some(5),
        "bsim3" | "bsim3v3" => Some(8),
        "bsim4" | "bsim4v8" => Some(54),
        "bsimsoi" => Some(10),
        "ekv" | "ekv26" => Some(260),
        "ekv3" => Some(301),
        _ => None,
    }
    .ok_or_else(|| unsupported_construct(SpectreNamespace::ModelFamily, model_type, line))?;
    let polarity = take_assignment(assignments, "type").ok_or_else(|| {
        error(
            line,
            format!("Spectre model family '{model_type}' requires explicit type=n or type=p"),
        )
    })?;
    let canonical_type = canonical_mos_polarity(&polarity.value).ok_or_else(|| {
        error(
            line,
            format!(
                "Spectre model family '{model_type}' has unsupported polarity '{}'",
                polarity.value
            ),
        )
    })?;
    if model_type_lower == "bsimsoi" {
        validate_bsimsoi_version(assignments, line)?;
    }
    ensure_model_level(assignments, level, line, model_type)?;
    Ok(canonical_type.to_owned())
}

fn validate_bsimsoi_version(
    assignments: &[SpectreModelAssignment],
    line: usize,
) -> Result<(), SpectreModelAdapterError> {
    let Some(version) = assignments
        .iter()
        .find(|assignment| assignment.name.eq_ignore_ascii_case("version"))
    else {
        return Ok(());
    };
    let value = version.value.trim_matches(['\'', '"']);
    let supported = value
        .parse::<f64>()
        .is_ok_and(|value| value.is_finite() && (value - 3.2).abs() <= f64::EPSILON * 8.0);
    if supported {
        Ok(())
    } else {
        Err(error(
            line,
            format!(
                "Spectre model family 'bsimsoi' requests unsupported VERSION={}; the native BSIM3-SOI route is qualified for VERSION=3.2",
                version.value
            ),
        ))
    }
}

fn validate_and_remove_redundant_polarity(
    canonical_type: &str,
    assignments: &mut Vec<SpectreModelAssignment>,
    line: usize,
) -> Result<(), SpectreModelAdapterError> {
    let Some(polarity) = take_assignment(assignments, "type") else {
        return Ok(());
    };
    let expected = match canonical_type {
        "NMOS" => Some("NMOS"),
        "PMOS" => Some("PMOS"),
        _ => None,
    };
    if expected.is_some_and(|expected| canonical_mos_polarity(&polarity.value) != Some(expected)) {
        return Err(error(
            line,
            format!(
                "Spectre model type '{canonical_type}' conflicts with type={}",
                polarity.value
            ),
        ));
    }
    if expected.is_none() {
        assignments.push(polarity);
    }
    Ok(())
}

fn canonical_mos_polarity(value: &str) -> Option<&'static str> {
    match value
        .trim_matches(['\'', '"'])
        .to_ascii_lowercase()
        .as_str()
    {
        "n" | "nmos" | "nch" | "nchannel" => Some("NMOS"),
        "p" | "pmos" | "pch" | "pchannel" => Some("PMOS"),
        _ => None,
    }
}

fn take_assignment(
    assignments: &mut Vec<SpectreModelAssignment>,
    name: &str,
) -> Option<SpectreModelAssignment> {
    assignments
        .iter()
        .position(|assignment| assignment.name.eq_ignore_ascii_case(name))
        .map(|index| assignments.remove(index))
}

fn ensure_model_level(
    assignments: &mut Vec<SpectreModelAssignment>,
    expected: u32,
    line: usize,
    model_type: &str,
) -> Result<(), SpectreModelAdapterError> {
    if let Some(level) = assignments
        .iter()
        .find(|assignment| assignment.name.eq_ignore_ascii_case("level"))
    {
        let parsed = level.value.parse::<u32>().map_err(|_| {
            error(
                line,
                format!("Spectre model family '{model_type}' has a non-integer LEVEL"),
            )
        })?;
        if parsed != expected && !(model_type.eq_ignore_ascii_case("bsim4") && parsed == 14) {
            return Err(error(
                line,
                format!(
                    "Spectre model family '{model_type}' conflicts with LEVEL={parsed}; expected LEVEL={expected}"
                ),
            ));
        }
    } else {
        assignments.insert(
            0,
            SpectreModelAssignment {
                name: "level".to_owned(),
                value: expected.to_string(),
            },
        );
    }
    Ok(())
}

fn adapt_include(rest: &str, line: usize) -> Result<String, SpectreModelAdapterError> {
    let rest = rest.trim();
    let (path, remainder) = if let Some(quote) = rest
        .chars()
        .next()
        .filter(|character| matches!(character, '\'' | '"'))
    {
        let quoted = &rest[quote.len_utf8()..];
        let end = quoted
            .find(quote)
            .ok_or_else(|| error(line, "Spectre include path has no closing quote"))?;
        (&quoted[..end], quoted[end + quote.len_utf8()..].trim())
    } else {
        take_token(rest).ok_or_else(|| error(line, "Spectre include has no path"))?
    };
    let section = remainder.split_whitespace().find_map(|token| {
        token
            .split_once('=')
            .filter(|(key, _)| key.eq_ignore_ascii_case("section"))
            .map(|(_, value)| value.trim_matches(['\'', '"']))
    });
    if remainder
        .split_whitespace()
        .any(|token| !token.to_ascii_lowercase().starts_with("section="))
    {
        return Err(error(
            line,
            "Spectre include contains unsupported options beyond section=",
        ));
    }
    Ok(section.map_or_else(
        || format!(".include \"{path}\""),
        |section| format!(".lib \"{path}\" {section}"),
    ))
}

fn brace_delta(value: &str) -> i32 {
    value.chars().fold(0, |depth, character| match character {
        '{' => depth + 1,
        '}' => depth - 1,
        _ => depth,
    })
}

fn split_head(value: &str) -> (&str, &str) {
    take_token(value).unwrap_or((value, ""))
}

fn take_token(value: &str) -> Option<(&str, &str)> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    let end = value.find(char::is_whitespace).unwrap_or(value.len());
    Some((&value[..end], value[end..].trim()))
}

fn one_identifier<'a>(
    value: &'a str,
    line: usize,
    statement: &str,
) -> Result<&'a str, SpectreModelAdapterError> {
    let (name, remainder) =
        take_token(value).ok_or_else(|| error(line, format!("Spectre {statement} has no name")))?;
    if !remainder.is_empty() {
        return Err(error(
            line,
            format!("Spectre {statement} contains unsupported trailing syntax"),
        ));
    }
    Ok(name)
}

fn adapter_receipt(source: &str) -> String {
    format!("* RSpice spectre-model/1 presentation directive: {source}")
}

fn spice_interop_receipt(source: &str) -> String {
    format!("* RSpice spectre-spice/1 presentation directive: {source}")
}

fn error(line: usize, message: impl Into<String>) -> SpectreModelAdapterError {
    SpectreModelAdapterError {
        line,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_spectre_detection_is_case_insensitive_and_comment_aware() {
        assert!(is_native_spectre_source(
            Path::new("model.SCS"),
            "model junction diode is=1e-14\n"
        ));
        assert!(is_native_spectre_source(
            Path::new("model.lib"),
            "SiMuLaToR LaNg=SpEcTrE\n"
        ));
        assert!(!is_native_spectre_source(
            Path::new("model.lib"),
            "// simulator lang=spectre\n/*\nsimulator lang=spectre\n*/\n* simulator lang=spectre\n"
        ));
    }

    #[test]
    fn native_sections_and_multiline_models_adapt_without_losing_lines() {
        let source = "simulator lang=spectre\nsection tt\nmodel nch bsim4 {\n  type=n level=54\n}\nendsection tt\n";
        let adapted = adapt_spectre_model_library(Path::new("models.scs"), source)
            .expect("supported model library adapts");
        assert_eq!(adapted.lines().count(), source.lines().count());
        assert!(adapted.contains(".lib tt"), "{adapted}");
        assert!(
            adapted.contains(".model nch NMOS ( level=54 )"),
            "{adapted}"
        );
        assert!(adapted.contains(".endl tt"), "{adapted}");
    }

    #[test]
    fn named_mos_family_requires_polarity_and_injects_canonical_level() {
        let adapted = adapt_spectre_model_library(
            Path::new("models.scs"),
            "simulator lang=spectre\nmodel nch bsim4 type=n vth0=0.4\n",
        )
        .expect("supported named family translates exactly");
        assert!(
            adapted.contains(".model nch NMOS ( level=54 vth0=0.4 )"),
            "{adapted}"
        );

        let error = adapt_spectre_model_library(
            Path::new("models.scs"),
            "simulator lang=spectre\nmodel nch bsim4 vth0=0.4\n",
        )
        .expect_err("polarity cannot be guessed");
        assert_eq!(error.line, 2);
        assert!(error.message.contains("explicit type=n or type=p"));
    }

    #[test]
    fn bsimsoi_n_and_p_models_lower_with_line_preserving_continuations() {
        let source = "simulator lang=spectre\nmodel nfet bsimsoi\n+ type=n\n+ version=3.2\n+ tox=8e-9\nmodel pfet bsimsoi type=p version=3.2 tox=8e-9\n";
        let adapted = adapt_spectre_model_library(Path::new("bsimsoi.scs"), source)
            .expect("qualified Spectre BSIMSOI models lower to native BSIM3-SOI cards");

        assert_eq!(adapted.lines().count(), source.lines().count());
        assert!(
            adapted.contains(".model nfet NMOS ( level=10 version=3.2 tox=8e-9 )"),
            "{adapted}"
        );
        assert!(
            adapted.contains(".model pfet PMOS ( level=10 version=3.2 tox=8e-9 )"),
            "{adapted}"
        );
        assert_eq!(
            adapted
                .lines()
                .filter(|line| line.contains("spectre-model/2 continuation line"))
                .count(),
            3,
            "{adapted}"
        );
    }

    #[test]
    fn malformed_or_unsupported_bsimsoi_cards_fail_at_the_model_source_line() {
        for (source, expected) in [
            (
                "simulator lang=spectre\nmodel nfet bsimsoi version=3.2\n",
                "requires explicit type=n or type=p",
            ),
            (
                "simulator lang=spectre\nmodel nfet bsimsoi type=ambipolar version=3.2\n",
                "unsupported polarity",
            ),
            (
                "simulator lang=spectre\nmodel nfet bsimsoi type=n level=55 version=3.2\n",
                "conflicts with LEVEL=55; expected LEVEL=10",
            ),
            (
                "simulator lang=spectre\nmodel nfet bsimsoi type=n version=4.0\n",
                "qualified for VERSION=3.2",
            ),
            (
                "simulator lang=spectre\nmodel nfet bsimsoi\n+ type=n\n+ tox\n",
                "parameter 'tox' has no '=' assignment",
            ),
        ] {
            let error = adapt_spectre_model_library(Path::new("bsimsoi.scs"), source)
                .expect_err("unsupported BSIMSOI semantics must not be approximated");
            assert_eq!(error.line, 2, "{error}");
            assert!(error.message.contains(expected), "{error}");
        }
    }

    #[test]
    fn aliases_and_grouped_parameter_values_translate_without_token_loss() {
        let adapted = adapt_spectre_model_library(
            Path::new("models.scs"),
            "simulator lang=spectre\nmodel junction diode is=2e-14 note=\"qualified model\" expr={a + b}\n",
        )
        .expect("supported alias and grouped values translate");
        assert!(
            adapted
                .contains(".model junction D ( is=2e-14 note=\"qualified model\" expr={a + b} )"),
            "{adapted}"
        );
    }

    #[test]
    fn unsupported_model_family_fails_closed() {
        let error = adapt_spectre_model_library(
            Path::new("models.scs"),
            "simulator lang=spectre\nmodel mystery unverified_family gain=2\n",
        )
        .expect_err("unknown model families cannot become executable cards");
        assert_eq!(error.line, 2);
        assert!(
            error
                .message
                .contains("unknown native Spectre model family 'unverified_family'"),
            "{}",
            error.message
        );
    }

    #[test]
    fn native_subcircuits_parameters_and_instances_lower_from_typed_statements() {
        let source = "simulator lang=spectre\nglobal 0\nsubckt RC_sub (in out)\nparameters\n+ rval=1K\n+ cval=1u\nR1 (in mid) resistor r=rval m=2\nC1 (mid out) capacitor c=cval\nends RC_sub\nX1 (a b) RC_sub rval=2K\n";
        let adapted = adapt_spectre_model_library(Path::new("macro.scs"), source)
            .expect("native Spectre macromodel statements lower deterministically");
        assert_eq!(adapted.lines().count(), source.lines().count());
        assert!(adapted.contains(".global 0"), "{adapted}");
        assert!(adapted.contains(".subckt RC_sub in out"), "{adapted}");
        assert!(adapted.contains(".param rval=1K cval=1u"), "{adapted}");
        assert!(adapted.contains("RR1 in mid rval m=2"), "{adapted}");
        assert!(adapted.contains("CC1 mid out cval"), "{adapted}");
        assert!(adapted.contains(".ends RC_sub"), "{adapted}");
        assert!(adapted.contains("XX1 a b RC_sub rval=2K"), "{adapted}");
    }

    #[test]
    fn native_model_instances_resolve_forward_declarations_by_family() {
        let adapted = adapt_spectre_model_library(
            Path::new("macro.scs"),
            "simulator lang=spectre\nM1 (d g s b) nch w=1u l=100n\nmodel nch bsim4 type=n\n",
        )
        .expect("two-pass lowering resolves a later model declaration");
        assert!(adapted.contains("MM1 d g s b nch w=1u l=100n"), "{adapted}");
        assert!(
            adapted.contains(".model nch NMOS ( level=54 )"),
            "{adapted}"
        );
    }

    #[test]
    fn native_source_waveforms_and_ahdl_include_lower_explicitly() {
        let source = "simulator lang=spectre\nahdl_include \"models/device.va\"\nV1 (out 0) vsource type=pulse val0=0 val1=1 delay=1n rise=2n fall=2n width=5n period=10n\n";
        let adapted = adapt_spectre_model_library(Path::new("macro.scs"), source)
            .expect("supported source and AHDL syntax lowers");
        assert!(
            adapted.contains(".veriloga \"models/device.va\""),
            "{adapted}"
        );
        assert!(
            adapted.contains("VV1 out 0 PULSE(0 1 1n 2n 2n 5n 10n)"),
            "{adapted}"
        );
    }

    #[test]
    fn process_statistics_lower_to_an_executable_native_plan() {
        let source = "simulator lang=spectre\nparameters d1=2 d2=20\nstatistics {\n process {\n  vary d1 dist=gauss std=1\n  vary d2 dist=uniform N=2 percent=yes\n }\n}\nmodel junction diode is=2e-14\n";
        let adapted = adapt_spectre_model_library(Path::new("statistics.scs"), source)
            .expect("representable Spectre process statistics lower");
        assert_eq!(adapted.lines().count(), source.lines().count());
        assert!(adapted.contains(".RSPICE_SPECTRE_STAT S1~"), "{adapted}");
        assert!(adapted.contains(".model junction D"), "{adapted}");

        let deck = format!("statistics nominal\n{adapted}.end\n");
        let nominal =
            crate::Netlist::parse(&deck).expect("lowered statistics are valid executable SPICE");
        assert_eq!(nominal.params.get("d1"), Some(2.0));
        assert_eq!(nominal.params.get("d2"), Some(20.0));
        assert_eq!(nominal.spectre_statistics.variations.len(), 2, "{adapted}");
        assert!(nominal.spectre_statistics.variations[1].percent);
        let sample = nominal
            .spectre_statistics
            .sample_process(
                &nominal.params,
                &super::super::SpectreStatisticalCoordinate {
                    seed: 8,
                    monte_carlo_run: 2,
                    temperature_celsius: 27.0,
                    axes: vec![],
                },
            )
            .expect("native process plan samples");
        assert!(sample["D1"].is_finite());
        assert!((18.0..=22.0).contains(&sample["D2"]));
    }

    #[test]
    fn mismatch_correlation_and_lognormal_statistics_are_executable() {
        let source = "parameters d1=2 d2=3 d3=4 dvth=0\nstatistics {\n process {\n  vary d1 dist=gauss std=1\n  vary d2 dist=uniform N=0.5\n  vary d3 dist=lnorm std=0.1\n }\n mismatch {\n  vary dvth dist=gauss std=0.02 percent=no\n }\n correlate param=[d1 d2 d3] cc=0.25\n}\n";
        let adapted = adapt_spectre_model_library(Path::new("statistics.scs"), source)
            .expect("native statistical semantics lower");
        let deck = crate::Netlist::parse(&format!("native statistics\n{adapted}.end\n"))
            .expect("native statistical plan parses");
        assert_eq!(deck.spectre_statistics.variations.len(), 4, "{adapted}");
        assert_eq!(deck.spectre_statistics.correlations.len(), 1);
        let coordinate = super::super::SpectreStatisticalCoordinate {
            seed: 99,
            monte_carlo_run: 11,
            temperature_celsius: 125.0,
            axes: vec![("corner".into(), 1.0)],
        };
        let process = deck
            .spectre_statistics
            .sample_process(&deck.params, &coordinate)
            .expect("correlated process variables sample");
        let mismatch = deck
            .spectre_statistics
            .sample_mismatch(&deck.params, &process, "X1", &coordinate)
            .expect("per-instance mismatch samples");
        assert_eq!(process.len(), 3);
        assert_eq!(mismatch.len(), 1);
        assert!(process["D3"] > 0.0);
        assert!(mismatch["DVTH"].is_finite());
    }

    #[test]
    fn same_parameter_can_have_process_and_mismatch_variation() {
        let source = "parameters x=10 y=20\nstatistics {\n process {\n  vary x dist=gauss std=1\n  vary y dist=gauss std=2\n }\n mismatch {\n  vary x dist=gauss std=0.5\n  vary y dist=gauss std=1\n }\n correlate param=[x y] cc=0.2\n}\n";
        let adapted = adapt_spectre_model_library(Path::new("statistics.scs"), source)
            .expect("same parameter may vary in both statistical scopes");
        let deck = crate::Netlist::parse(&format!("scoped correlations\n{adapted}.end\n"))
            .expect("scoped correlation plan parses");
        assert_eq!(deck.spectre_statistics.variations.len(), 4);
        assert_eq!(deck.spectre_statistics.correlations.len(), 1);
        assert_eq!(
            deck.spectre_statistics.correlations[0].scope,
            super::super::SpectreVariationScope::Process
        );
    }

    #[test]
    fn malformed_or_unrepresented_statistics_fail_closed() {
        for (source, expected) in [
            (
                "statistics {\n process {\n  vary d1 dist=uniform std=1\n }\n}\n",
                "requires N=",
            ),
            (
                "statistics {\n process {\n  vary d1 dist=gauss N=1\n }\n}\n",
                "requires std=",
            ),
            (
                "statistics {\n process {\n  vary d1 dist=gauss mean=2 std=1\n }\n}\n",
                "current parameter value",
            ),
        ] {
            let error = adapt_spectre_model_library(Path::new("statistics.scs"), source)
                .expect_err("invalid statistics must not become inert metadata");
            assert!(error.message.contains(expected), "{error}");
        }

        let device_correlation = adapt_spectre_model_library(
            Path::new("statistics.scs"),
            "statistics {\n mismatch {\n  vary x dist=gauss std=1\n }\n correlate dev=[M1 M2] param=[x] cc=0.5\n}\n",
        )
        .expect_err("device-selection correlation must not be reinterpreted as parameter correlation");
        assert!(device_correlation.message.contains("dev="));
        assert!(device_correlation.message.contains("not represented"));

        let scoped_correlation = adapt_spectre_model_library(
            Path::new("statistics.scs"),
            "statistics {\n process {\n  vary x dist=gauss std=1\n  vary y dist=gauss std=1\n  correlate param=[x y] cc=0.5\n }\n}\n",
        )
        .expect_err("parameter correlation grammar belongs at statistics-block scope");
        assert!(scoped_correlation.message.contains("must be outside"));
    }

    #[test]
    fn statistics_correlations_fail_when_they_reference_undeclared_variations() {
        let error = adapt_spectre_model_library(
            Path::new("statistics.scs"),
            "statistics {\n process {\n  vary d1 dist=gauss std=1\n }\n correlate param=[d1 missing] cc=0.5\n}\n",
        )
        .expect_err("a correlation cannot target an undeclared variation");
        assert_eq!(error.line, 5);
        assert!(
            error
                .message
                .contains("undeclared process variation 'missing'")
        );
    }

    #[test]
    fn unknown_native_instance_master_fails_closed() {
        let error = adapt_spectre_model_library(
            Path::new("macro.scs"),
            "simulator lang=spectre\nU1 (a b) proprietary_device gain=2\n",
        )
        .expect_err("unknown instance semantics cannot be guessed");
        assert_eq!(error.line, 2);
        assert!(error.message.contains("no statement was discarded"));
    }

    #[test]
    fn the_construct_inventory_is_unambiguous_and_reachable() {
        for construct in SpectreConstruct::ALL {
            let names = construct.names();
            assert!(
                !names.is_empty(),
                "{construct:?} declares no spelling to match on"
            );
            for name in names {
                assert_eq!(
                    *name,
                    name.to_ascii_lowercase(),
                    "{construct:?} spelling '{name}' must be lowercase for case-insensitive lookup"
                );
                let resolved = SpectreConstruct::classify(construct.namespace(), name);
                assert_eq!(
                    resolved,
                    Some(*construct),
                    "'{name}' in {:?} resolves to {resolved:?} instead of {construct:?}; two \
                     constructs claim the same spelling in one namespace",
                    construct.namespace()
                );
            }
        }
    }

    #[test]
    fn unsupported_constructs_report_their_recorded_reason_not_a_generic_refusal() {
        for construct in SpectreConstruct::ALL {
            let name = construct.names()[0];
            let message = unsupported_construct(construct.namespace(), name, 7).message;
            match construct.support() {
                SpectreSupport::Lowered => assert!(
                    message.contains("adapter defect"),
                    "a lowered construct reaching the refusal path is a defect: {message}"
                ),
                SpectreSupport::Unsupported(reason) => assert!(
                    message.contains(reason) && !message.contains("unknown native Spectre"),
                    "{construct:?} must be refused with its recorded reason: {message}"
                ),
                SpectreSupport::OwnedElsewhere(owner) => assert!(
                    message.contains(owner) && !message.contains("unknown native Spectre"),
                    "{construct:?} must name its owner: {message}"
                ),
            }
        }
        assert!(
            unsupported_construct(SpectreNamespace::Master, "nothing_like_this", 7)
                .message
                .contains("unknown native Spectre instance master")
        );
    }

    #[test]
    fn spectre_comments_are_quote_aware_and_preserve_source_lines() {
        let source = "simulator lang=spectre // select language\n/* foundry header\n   retained as two blank lines */\nmodel junction diode is=2e-14 // nominal\nmodel url_note diode note=\"https://foundry.invalid/model\" is=3e-14\n";
        let adapted = adapt_spectre_model_library(Path::new("comments.scs"), source)
            .expect("Spectre comments are lexical trivia");
        assert_eq!(adapted.lines().count(), source.lines().count());
        assert!(
            adapted.contains(".model junction D ( is=2e-14 )"),
            "{adapted}"
        );
        assert!(
            adapted.contains("note=\"https://foundry.invalid/model\""),
            "{adapted}"
        );
        assert!(!adapted.contains("foundry header"), "{adapted}");
    }

    #[test]
    fn unterminated_spectre_block_comment_reports_its_opening_line() {
        let error = adapt_spectre_model_library(
            Path::new("comments.scs"),
            "simulator lang=spectre\nmodel junction diode /* never closed\n",
        )
        .expect_err("unterminated comments cannot silently discard source");
        assert_eq!(error.line, 2);
        assert!(error.message.contains("no closing '*/'"));
    }
}

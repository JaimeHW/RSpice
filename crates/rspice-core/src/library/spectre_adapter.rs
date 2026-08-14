//! Fail-closed Spectre model-library adapter.
//!
//! RSpice executes canonical SPICE cards. This module parses the Spectre
//! statements used by foundry model libraries and macromodels, builds a symbol
//! table for model and subcircuit masters, then lowers the typed statements to
//! a line-preserving canonical projection. Unsupported semantics remain
//! explicit errors instead of being discarded or guessed.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    subcircuits: HashSet<String>,
}

/// Adapt a supported Spectre model-library source without changing its line
/// count. Plain SPICE sources are returned by reference.
pub fn adapt_spectre_model_library<'a>(
    path: &Path,
    source: &'a str,
) -> Result<Cow<'a, str>, SpectreModelAdapterError> {
    let is_scs = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("scs"));
    let has_language_boundary = source.lines().any(|line| {
        line.trim()
            .to_ascii_lowercase()
            .starts_with("simulator lang=")
    });
    if !is_scs && !has_language_boundary {
        return Ok(Cow::Borrowed(source));
    }

    let lines = source.lines().collect::<Vec<_>>();
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
                let (adapted, consumed) = adapt_model(lines, index)?;
                let (name, canonical_type) = adapted_model_identity(&adapted, line_number)?;
                statements.push(SpectreStatement {
                    line: line_number,
                    consumed_lines: consumed,
                    kind: SpectreStatementKind::Model {
                        name,
                        canonical_type,
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
            "saveoptions" | "simulatoroptions" | "altergroup" => {
                return Err(error(
                    line_number,
                    format!("unsupported native Spectre model-library statement '{head}'"),
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
        let trimmed = strip_spectre_line_comment(raw).trim();
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
        .map(|variation| variation.parameter.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    for correlation in &correlations {
        for parameter in &correlation.parameters {
            if !declared.contains(&parameter.to_ascii_lowercase()) {
                return Err(error(
                    correlation.line,
                    format!("Spectre correlation references undeclared variation '{parameter}'"),
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

fn strip_spectre_line_comment(line: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    let mut previous = None;
    for (index, character) in line.char_indices() {
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == active_quote {
                quote = None;
            }
        } else if matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if character == '/' && previous == Some('/') {
            return &line[..index - 1];
        }
        previous = Some(character);
    }
    line
}

fn lower_statistics(
    statistics: &SpectreStatisticsBlock,
) -> Result<String, SpectreModelAdapterError> {
    if let Some(correlation) = statistics.correlations.first() {
        return Err(error(
            correlation.line,
            "Spectre statistical correlation cannot be represented by RSpice's independent deck-statistical sampler",
        ));
    }

    let mut assignments = Vec::with_capacity(statistics.variations.len());
    let mut names = HashSet::with_capacity(statistics.variations.len());
    for variation in &statistics.variations {
        if variation.scope == SpectreVariationScope::Mismatch {
            return Err(error(
                variation.line,
                format!(
                    "Spectre mismatch variation '{}' requires independent per-instance draws, which this model-library adapter cannot represent",
                    variation.parameter
                ),
            ));
        }
        if !valid_spice_identifier(&variation.parameter) {
            return Err(error(
                variation.line,
                format!(
                    "Spectre variation name '{}' is not a portable SPICE parameter identifier",
                    variation.parameter
                ),
            ));
        }
        if !names.insert(variation.parameter.to_ascii_lowercase()) {
            return Err(error(
                variation.line,
                format!(
                    "Spectre variation '{}' is declared more than once",
                    variation.parameter
                ),
            ));
        }

        let mut attributes = variation.attributes.clone();
        let mean = take_assignment(&mut attributes, "mean")
            .map(|assignment| assignment.value)
            .unwrap_or_else(|| "0".to_owned());
        let std = take_assignment(&mut attributes, "std")
            .ok_or_else(|| {
                error(
                    variation.line,
                    format!(
                        "Spectre variation '{}' must declare std= for executable lowering",
                        variation.parameter
                    ),
                )
            })?
            .value;
        if let Some(percent) = take_assignment(&mut attributes, "percent") {
            let value = percent.value.trim_matches(['\'', '"']).to_ascii_lowercase();
            if !matches!(value.as_str(), "no" | "false" | "0") {
                return Err(error(
                    variation.line,
                    format!(
                        "Spectre variation '{}' uses percent={}, whose nominal-relative semantics are not available at this global process-variable boundary",
                        variation.parameter, percent.value
                    ),
                ));
            }
        }
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

        let expression = match variation.distribution.as_str() {
            "gauss" | "gaussian" | "normal" => {
                format!("agauss(({mean}),({std}),1)")
            }
            "unif" | "uniform" => {
                // Spectre's std= is a standard deviation. RSpice's aunif
                // second operand is a half-range, which is sqrt(3) * sigma.
                format!("aunif(({mean}),1.7320508075688772935*({std}))")
            }
            "lnorm" | "lognormal" => {
                return Err(error(
                    variation.line,
                    format!(
                        "Spectre lognormal variation '{}' is not supported by the executable deck-statistical sampler",
                        variation.parameter
                    ),
                ));
            }
            distribution => {
                return Err(error(
                    variation.line,
                    format!(
                        "Spectre variation '{}' uses unsupported executable distribution '{distribution}'",
                        variation.parameter
                    ),
                ));
            }
        };
        assignments.push(format!("{}={{{expression}}}", variation.parameter));
    }
    Ok(format!(".param {}", assignments.join(" ")))
}

fn valid_spice_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

impl SpectreSymbols {
    fn from_statements(statements: &[SpectreStatement]) -> Result<Self, SpectreModelAdapterError> {
        let mut symbols = Self::default();
        for statement in statements {
            match &statement.kind {
                SpectreStatementKind::Model {
                    name,
                    canonical_type,
                    ..
                } => {
                    let key = name.to_ascii_lowercase();
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
                SpectreStatementKind::Lowered(_)
                | SpectreStatementKind::Statistics(_)
                | SpectreStatementKind::Instance(_) => {}
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
    let (nodes, remainder) = consume_parenthesized(remainder, line, "instance node list")?;
    let nodes = split_node_list(&nodes, line)?;
    if nodes.is_empty() {
        return Err(error(
            line,
            format!("Spectre instance '{name}' has no connected nodes"),
        ));
    }
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
        _ if symbols.subcircuits.contains(&master) => {
            Ok(render_model_or_subcircuit_instance(instance, "X"))
        }
        _ if symbols.models.contains_key(&master) => {
            let canonical = &symbols.models[&master];
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
        _ => Err(error(
            line,
            format!(
                "unsupported native Spectre instance master '{}'; no statement was discarded",
                instance.master
            ),
        )),
    }
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

fn adapt_model(lines: &[&str], start: usize) -> Result<(String, usize), SpectreModelAdapterError> {
    let line_number = start + 1;
    let first = lines[start].trim();
    let (_, rest) = split_head(first);
    let (name, rest) = take_token(rest)
        .ok_or_else(|| error(line_number, "Spectre model declaration has no name"))?;
    let (model_type, mut parameters) = take_token(rest)
        .ok_or_else(|| error(line_number, "Spectre model declaration has no model type"))?;
    let mut consumed = 1usize;
    let mut brace_depth = brace_delta(parameters);
    if brace_depth < 0 {
        return Err(error(
            line_number,
            "Spectre model declaration closes an unopened brace",
        ));
    }
    while brace_depth > 0 {
        let next = lines.get(start + consumed).ok_or_else(|| {
            error(
                line_number,
                "Spectre model declaration has an unterminated parameter block",
            )
        })?;
        parameters = parameters.trim_end();
        let mut joined = parameters.to_owned();
        joined.push(' ');
        joined.push_str(next.trim());
        // Keep the owned join alive by leaking only into the next iteration is
        // not acceptable; switch to the final owned accumulator below.
        let mut accumulator = joined;
        consumed += 1;
        brace_depth += brace_delta(next);
        while brace_depth > 0 {
            let next = lines.get(start + consumed).ok_or_else(|| {
                error(
                    line_number,
                    "Spectre model declaration has an unterminated parameter block",
                )
            })?;
            accumulator.push(' ');
            accumulator.push_str(next.trim());
            brace_depth += brace_delta(next);
            consumed += 1;
        }
        if brace_depth < 0 {
            return Err(error(
                start + consumed,
                "Spectre model declaration closes too many braces",
            ));
        }
        return Ok((
            render_model(name, model_type, &accumulator, line_number)?,
            consumed,
        ));
    }
    Ok((
        render_model(name, model_type, parameters, line_number)?,
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
        "ekv" | "ekv26" => Some(260),
        "ekv3" => Some(301),
        _ => None,
    }
    .ok_or_else(|| {
        error(
            line,
            format!(
                "unsupported native Spectre model family '{model_type}'; use a supported canonical model export"
            ),
        )
    })?;
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
    ensure_model_level(assignments, level, line, model_type)?;
    Ok(canonical_type.to_owned())
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
                .contains("unsupported native Spectre model family")
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
    fn process_statistics_lower_to_executable_deck_statistical_parameters() {
        let source = "simulator lang=spectre\nstatistics {\n process {\n  vary d1 dist=gauss mean=2 std=1\n  vary d2 dist=uniform std=2\n }\n}\nmodel junction diode is=2e-14\n";
        let adapted = adapt_spectre_model_library(Path::new("statistics.scs"), source)
            .expect("representable Spectre process statistics lower");
        assert_eq!(adapted.lines().count(), source.lines().count());
        assert!(adapted.contains("d1={agauss((2),(1),1)}"), "{adapted}");
        assert!(
            adapted.contains("d2={aunif((0),1.7320508075688772935*(2))}"),
            "{adapted}"
        );
        assert!(adapted.contains(".model junction D"), "{adapted}");

        let deck = format!("statistics nominal\n{adapted}.end\n");
        let nominal = crate::Netlist::parse_with_options(
            &deck,
            crate::netlist::NetlistParseOptions {
                statistical_mode: crate::netlist::StatisticalParamMode::Nominal,
                ..Default::default()
            },
        )
        .expect("lowered statistics are valid executable SPICE");
        assert_eq!(nominal.params.get("d1"), Some(2.0));
        assert_eq!(nominal.params.get("d2"), Some(0.0));
    }

    #[test]
    fn statistics_semantics_that_cannot_be_preserved_fail_closed() {
        for (source, expected) in [
            (
                "statistics {\n mismatch {\n  vary dvth dist=gauss std=1\n }\n}\n",
                "per-instance draws",
            ),
            (
                "statistics {\n process {\n  vary d1 dist=gauss std=1\n  vary d2 dist=gauss std=1\n }\n correlate param=[d1 d2] cc=0.5\n}\n",
                "correlation cannot be represented",
            ),
            (
                "statistics {\n process {\n  vary d1 dist=lnorm std=1\n }\n}\n",
                "lognormal variation",
            ),
        ] {
            let error = adapt_spectre_model_library(Path::new("statistics.scs"), source)
                .expect_err("unrepresentable statistics must not become inert metadata");
            assert!(error.message.contains(expected), "{error}");
        }
    }

    #[test]
    fn statistics_correlations_fail_when_they_reference_undeclared_variations() {
        let error = adapt_spectre_model_library(
            Path::new("statistics.scs"),
            "statistics {\n process {\n  vary d1 dist=gauss std=1\n }\n correlate param=[d1 missing] cc=0.5\n}\n",
        )
        .expect_err("a correlation cannot target an undeclared variation");
        assert_eq!(error.line, 5);
        assert!(error.message.contains("undeclared variation 'missing'"));
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
}

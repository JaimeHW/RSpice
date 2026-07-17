//! Explicit materialization of Xyce `.PREPROCESS ADDRESISTORS` output.
//!
//! Xyce writes a derived netlist and never mutates the circuit in the run that
//! requested it. This module follows that contract: parsing records typed
//! policy, while callers explicitly request a transactionally generated copy.

use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};

use super::expr::eval_expression;
use super::lexer::parse_spice_value_complete;
use super::{
    ConnectivityAnalysisError, Element, ElementKind, ElementProvenance, FlattenerConfig, Netlist,
    ParseError, ParseWithAbortError, XyceAddResistorMode, XyceAddResistorSpec,
    analyze_xyce_connectivity, ensure_parse_not_aborted,
    flatten_netlist_with_models_config_with_abort, flatten_netlist_with_models_with_abort,
    poll_parse_abort, poll_parse_text,
};

/// One validated ADDRESISTORS resistance in deterministic semantic order.
#[derive(Debug, Clone, PartialEq)]
pub struct XyceResolvedAddResistorSpec {
    pub mode: XyceAddResistorMode,
    pub raw_resistance: String,
    pub resistance: Value,
    pub source_line: usize,
}

/// One configured mode before topology determines whether its value is used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XyceConfiguredAddResistorSpec {
    pub mode: XyceAddResistorMode,
    pub raw_resistance: String,
    pub source_line: usize,
}

/// Exact generated resistor recorded by materialization.
#[derive(Debug, Clone, PartialEq)]
pub struct XyceGeneratedResistor {
    pub name: String,
    /// Canonical RSpice execution node used by the in-memory derived copy.
    pub node: String,
    /// Canonical Xyce 7.10 node spelling written to the replayable artifact.
    pub artifact_node: String,
    pub mode: XyceAddResistorMode,
    pub raw_resistance: String,
    pub resistance: Value,
}

/// Deterministic report for one ADDRESISTORS materialization.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct XyceAddResistorsReport {
    /// Configured modes in fixed semantic order, including zero-candidate
    /// modes whose resistance is intentionally never evaluated.
    pub configured_modes: Vec<XyceConfiguredAddResistorSpec>,
    /// Enabled modes in fixed ONETERMINAL, NODCPATH order.
    pub resolved_modes: Vec<XyceResolvedAddResistorSpec>,
    /// Enabled ONETERMINAL candidates, sorted by canonical node name.
    pub one_terminal_candidates: Vec<String>,
    /// Enabled NODCPATH candidates after ONETERMINAL overlap precedence.
    pub no_dc_path_candidates: Vec<String>,
    /// Generated cards in category and node order.
    pub generated: Vec<XyceGeneratedResistor>,
}

/// A derived semantic netlist, its replayable source artifact, and its exact
/// generation report.
///
/// `derived_source` is a root-deck artifact: it preserves root physical records
/// through the first logical `.END`, comments active ADDRESISTORS controls,
/// appends the generated cards, and supplies exactly one final `.END`.
/// `netlist.source_text` contains the same artifact, so source-based consumers
/// cannot accidentally replay the unmaterialized deck. The source path remains
/// available for include/model resource resolution.
#[derive(Debug, Clone)]
pub struct XyceAddResistorsMaterialization {
    pub netlist: Netlist,
    pub report: XyceAddResistorsReport,
    pub derived_source: String,
}

/// Typed failure from ADDRESISTORS materialization.
#[derive(Debug, Error)]
pub enum XyceAddResistorsMaterializationError {
    #[error("netlist has no active Xyce ADDRESISTORS policy")]
    MissingPolicy,
    #[error("netlist has no retained root source for the Xyce ADDRESISTORS artifact")]
    MissingSourceText,
    #[error("Xyce ADDRESISTORS materialization aborted")]
    Aborted,
    #[error("failed to flatten netlist before ADDRESISTORS materialization: {0}")]
    Flatten(#[source] ParseError),
    #[error("failed to analyze Xyce connectivity: {0}")]
    Connectivity(#[source] ConnectivityAnalysisError),
    #[error(
        "invalid {mode:?} ADDRESISTORS resistance '{raw_resistance}' at line {source_line}: {reason}"
    )]
    InvalidResistance {
        mode: XyceAddResistorMode,
        raw_resistance: String,
        source_line: usize,
        reason: String,
    },
    #[error("generated ADDRESISTORS element name '{name}' collides with an existing element")]
    NameCollision { name: String },
    #[error("cannot project ADDRESISTORS topology into canonical Xyce hierarchy: {reason}")]
    ArtifactTopologyProjection { reason: String },
    #[error("ADDRESISTORS materialization exceeds addressable allocation capacity")]
    CapacityExceeded,
}

impl Netlist {
    /// Materialize the Xyce ADDRESISTORS derived semantic copy.
    pub fn materialize_xyce_add_resistors(
        &self,
    ) -> Result<XyceAddResistorsMaterialization, XyceAddResistorsMaterializationError> {
        self.materialize_xyce_add_resistors_with_abort(&NoAbort)
    }

    /// Materialize the derived copy with cooperative cancellation.
    pub fn materialize_xyce_add_resistors_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<XyceAddResistorsMaterialization, XyceAddResistorsMaterializationError> {
        check_abort(abort)?;

        let mut report = XyceAddResistorsReport::default();
        let policy = self
            .options
            .add_resistors
            .as_ref()
            .filter(|policy| !policy.is_empty())
            .ok_or(XyceAddResistorsMaterializationError::MissingPolicy)?;
        for mode in [
            XyceAddResistorMode::OneTerminal,
            XyceAddResistorMode::NoDcPath,
        ] {
            if let Some(spec) = policy.spec(mode) {
                report.configured_modes.push(XyceConfiguredAddResistorSpec {
                    mode,
                    raw_resistance: spec.raw_resistance.clone(),
                    source_line: spec.source_line,
                });
            }
        }

        let flattened =
            flatten_netlist_with_models_with_abort(self, abort).map_err(|error| match error {
                ParseWithAbortError::Aborted => XyceAddResistorsMaterializationError::Aborted,
                ParseWithAbortError::Parse(error) => {
                    XyceAddResistorsMaterializationError::Flatten(error)
                }
            })?;
        check_abort(abort)?;
        let mut xyce_artifact_config = FlattenerConfig::default();
        xyce_artifact_config.hierarchy_separator = ':';
        let xyce_artifact_flattened =
            flatten_netlist_with_models_config_with_abort(self, xyce_artifact_config, abort)
                .map_err(|error| match error {
                    ParseWithAbortError::Aborted => XyceAddResistorsMaterializationError::Aborted,
                    ParseWithAbortError::Parse(error) => {
                        XyceAddResistorsMaterializationError::Flatten(error)
                    }
                })?;
        let artifact_node_projection = build_artifact_node_projection(
            &flattened.elements,
            &xyce_artifact_flattened.elements,
            abort,
        )?;
        check_abort(abort)?;
        let diagnostics = analyze_xyce_connectivity(&flattened.elements)
            .map_err(XyceAddResistorsMaterializationError::Connectivity)?;
        check_abort(abort)?;

        let one_enabled = policy.one_terminal.is_some();
        let no_dc_enabled = policy.no_dc_path.is_some();
        if one_enabled {
            report.one_terminal_candidates = diagnostics.one_device_terminal_nodes;
            sort_nodes(&mut report.one_terminal_candidates);
        }
        if no_dc_enabled {
            report.no_dc_path_candidates = diagnostics.no_dc_path_nodes;
            sort_nodes(&mut report.no_dc_path_candidates);
            if one_enabled {
                let one_terminal = report
                    .one_terminal_candidates
                    .iter()
                    .map(|node| node.to_ascii_uppercase())
                    .collect::<HashSet<_>>();
                report
                    .no_dc_path_candidates
                    .retain(|node| !one_terminal.contains(&node.to_ascii_uppercase()));
            }
        }

        for mode in [
            XyceAddResistorMode::OneTerminal,
            XyceAddResistorMode::NoDcPath,
        ] {
            let has_candidates = match mode {
                XyceAddResistorMode::OneTerminal => !report.one_terminal_candidates.is_empty(),
                XyceAddResistorMode::NoDcPath => !report.no_dc_path_candidates.is_empty(),
            };
            if has_candidates && let Some(spec) = policy.spec(mode) {
                report
                    .resolved_modes
                    .push(resolve_resistance(mode, spec, self, abort)?);
            }
        }

        let generated_count = report
            .one_terminal_candidates
            .len()
            .checked_add(report.no_dc_path_candidates.len())
            .ok_or(XyceAddResistorsMaterializationError::CapacityExceeded)?;
        report
            .generated
            .try_reserve(generated_count)
            .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;

        let existing_names = collect_existing_names(&flattened.elements, abort)?;
        append_report_records(
            &mut report,
            XyceAddResistorMode::OneTerminal,
            &existing_names,
            &artifact_node_projection,
            abort,
        )?;
        append_report_records(
            &mut report,
            XyceAddResistorMode::NoDcPath,
            &existing_names,
            &artifact_node_projection,
            abort,
        )?;

        let derived_source = render_derived_source(self, &report, abort)?;

        check_abort(abort)?;
        let mut materialized = clone_netlist_with_abort(self, abort)?;
        check_abort(abort)?;
        materialized
            .elements
            .try_reserve(report.generated.len())
            .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;
        for (index, generated) in report.generated.iter().enumerate() {
            poll(index, abort)?;
            materialized.elements.push(Element {
                name: generated.name.clone(),
                kind: ElementKind::Resistor {
                    value: generated.resistance,
                    value_expr: None,
                    model: None,
                    instance_params: Vec::new(),
                    deferred_params: Vec::new(),
                },
                nodes: vec![generated.node.clone(), "0".to_string()],
                provenance: ElementProvenance::GeneratedXyceAddResistor {
                    mode: generated.mode,
                },
            });
        }
        materialized.options.add_resistors = None;
        materialized.source_text = Some(derived_source.clone());
        check_abort(abort)?;

        Ok(XyceAddResistorsMaterialization {
            netlist: materialized,
            report,
            derived_source,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct RootPhysicalLine<'a> {
    content: &'a str,
    ending: &'a str,
}

#[derive(Debug, Default)]
struct RootLogicalCard {
    text: String,
    physical_indices: Vec<usize>,
}

fn render_derived_source(
    netlist: &Netlist,
    report: &XyceAddResistorsReport,
    abort: &dyn AbortSignal,
) -> Result<String, XyceAddResistorsMaterializationError> {
    let source = netlist
        .source_text
        .as_deref()
        .ok_or(XyceAddResistorsMaterializationError::MissingSourceText)?;
    poll_parse_text(abort, source).map_err(|_| XyceAddResistorsMaterializationError::Aborted)?;
    let lines = split_root_physical_lines(source, abort)?;
    let newline = lines
        .iter()
        .find_map(|line| (!line.ending.is_empty()).then_some(line.ending))
        .unwrap_or("\n");
    let (cutoff, commented) = classify_root_artifact_lines(&lines, abort)?;

    let retained_bytes = lines[..cutoff]
        .iter()
        .try_fold(0usize, |total, line| {
            total
                .checked_add(line.content.len())?
                .checked_add(line.ending.len())
        })
        .ok_or(XyceAddResistorsMaterializationError::CapacityExceeded)?;
    let generated_bytes = report
        .generated
        .iter()
        .try_fold(0usize, |total, generated| {
            total
                .checked_add(generated.name.len())?
                .checked_add(generated.artifact_node.len())?
                .checked_add(generated.raw_resistance.len())?
                .checked_add(32)?
                .checked_add(newline.len())?
                .checked_add(4)
        })
        .ok_or(XyceAddResistorsMaterializationError::CapacityExceeded)?;
    let capacity = retained_bytes
        .checked_add(commented.len().saturating_mul(2))
        .and_then(|total| total.checked_add(generated_bytes))
        .and_then(|total| total.checked_add(4 + newline.len() * 2))
        .ok_or(XyceAddResistorsMaterializationError::CapacityExceeded)?;
    let mut artifact = String::new();
    artifact
        .try_reserve(capacity)
        .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;

    for (index, line) in lines[..cutoff].iter().enumerate() {
        poll(index, abort)?;
        if commented.contains(&index) {
            artifact.push_str("* ");
        }
        artifact.push_str(line.content);
        artifact.push_str(line.ending);
    }
    if !artifact.is_empty() && !artifact.ends_with('\n') && !artifact.ends_with('\r') {
        artifact.push_str(newline);
    }
    for (index, generated) in report.generated.iter().enumerate() {
        poll(index, abort)?;
        artifact.push_str(&generated.name);
        artifact.push(' ');
        artifact.push_str(&generated.artifact_node);
        artifact.push_str(" 0 ");
        artifact.push_str(&artifact_resistance_token(generated));
        artifact.push_str(newline);
    }
    artifact.push_str(".END");
    artifact.push_str(newline);
    check_abort(abort)?;
    Ok(artifact)
}

fn artifact_resistance_token(generated: &XyceGeneratedResistor) -> String {
    let raw = generated.raw_resistance.as_str();
    let direct_numeric = parse_spice_value_complete(raw)
        .is_ok_and(|value| value.to_bits() == generated.resistance.to_bits());
    if direct_numeric {
        raw.to_string()
    } else {
        // Display is the shortest decimal representation that round-trips to
        // the same binary64 value. This turns a context-free expression such
        // as `1+2` into a valid resistor literal without losing precision.
        generated.resistance.to_string()
    }
}

fn split_root_physical_lines<'a>(
    source: &'a str,
    abort: &dyn AbortSignal,
) -> Result<Vec<RootPhysicalLine<'a>>, XyceAddResistorsMaterializationError> {
    let mut lines = Vec::new();
    lines
        .try_reserve(source.lines().count().saturating_add(1))
        .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;
    let mut start = 0usize;
    for (index, byte) in source.bytes().enumerate() {
        poll(index, abort)?;
        if byte != b'\n' {
            continue;
        }
        let (content_end, ending) = if index > start && source.as_bytes()[index - 1] == b'\r' {
            (index - 1, "\r\n")
        } else {
            (index, "\n")
        };
        lines.push(RootPhysicalLine {
            content: &source[start..content_end],
            ending,
        });
        start = index + 1;
    }
    if start < source.len() {
        lines.push(RootPhysicalLine {
            content: &source[start..],
            ending: "",
        });
    }
    check_abort(abort)?;
    Ok(lines)
}

fn classify_root_artifact_lines(
    lines: &[RootPhysicalLine<'_>],
    abort: &dyn AbortSignal,
) -> Result<(usize, HashSet<usize>), XyceAddResistorsMaterializationError> {
    let mut cutoff = lines.len();
    let mut commented = HashSet::new();
    commented
        .try_reserve(lines.len().min(16))
        .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;
    let mut logical = RootLogicalCard::default();

    let flush = |logical: &mut RootLogicalCard,
                 cutoff: &mut usize,
                 commented: &mut HashSet<usize>|
     -> Result<bool, XyceAddResistorsMaterializationError> {
        if logical.text.is_empty() {
            return Ok(false);
        }
        let mut fields = logical.text.split_whitespace();
        let head = fields.next().unwrap_or_default();
        if head.eq_ignore_ascii_case(".END") {
            *cutoff = logical.physical_indices[0];
            return Ok(true);
        }
        if head.eq_ignore_ascii_case(".PREPROCESS")
            && fields
                .next()
                .is_some_and(|field| field.eq_ignore_ascii_case("ADDRESISTORS"))
        {
            for index in &logical.physical_indices {
                commented.insert(*index);
            }
        }
        logical.text.clear();
        logical.physical_indices.clear();
        Ok(false)
    };

    // The root's first physical record is the SPICE title, including when it
    // happens to spell a directive.
    for (index, line) in lines.iter().enumerate().skip(1) {
        poll(index, abort)?;
        let without_comment = strip_artifact_inline_comment(line.content);
        let first_nonblank = without_comment.trim_start_matches([' ', '\t']);
        if first_nonblank.is_empty() || first_nonblank.starts_with('*') {
            continue;
        }
        if let Some(continuation) = first_nonblank.strip_prefix('+') {
            if !logical.text.is_empty() {
                logical.text.push(' ');
                logical.text.push_str(continuation.trim());
                logical.physical_indices.push(index);
            }
            continue;
        }
        // Xyce's root preprocessor treats an indented new physical card as a
        // comment; only an indented '+' can continue the current card.
        if without_comment.starts_with([' ', '\t']) {
            continue;
        }
        if flush(&mut logical, &mut cutoff, &mut commented)? {
            break;
        }
        logical.text.push_str(without_comment.trim_end());
        logical.physical_indices.push(index);
    }
    if cutoff == lines.len() {
        let _ = flush(&mut logical, &mut cutoff, &mut commented)?;
    }
    commented.retain(|index| *index < cutoff);
    check_abort(abort)?;
    Ok((cutoff, commented))
}

fn strip_artifact_inline_comment(line: &str) -> &str {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;
    let mut chars = line.char_indices().peekable();
    let mut previous = None;
    while let Some((index, character)) = chars.next() {
        if escaped {
            escaped = false;
            previous = Some(character);
            continue;
        }
        match character {
            '\\' if in_single_quote || in_double_quote => escaped = true,
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            ';' if !in_single_quote && !in_double_quote => return &line[..index],
            '$' if !in_single_quote && !in_double_quote => {
                if chars.peek().is_none_or(|(_, next)| next.is_whitespace()) {
                    return &line[..index];
                }
            }
            '/' if !in_single_quote && !in_double_quote => {
                if matches!(chars.peek(), Some((_, '/')))
                    && previous.map_or(true, char::is_whitespace)
                {
                    return &line[..index];
                }
            }
            _ => {}
        }
        previous = Some(character);
    }
    line
}

fn resolve_resistance(
    mode: XyceAddResistorMode,
    spec: &XyceAddResistorSpec,
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<XyceResolvedAddResistorSpec, XyceAddResistorsMaterializationError> {
    poll_parse_text(abort, &spec.raw_resistance)
        .map_err(|_| XyceAddResistorsMaterializationError::Aborted)?;
    let braced = spec
        .raw_resistance
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'));
    let quoted = spec
        .raw_resistance
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''));
    let expression = braced.or(quoted).unwrap_or(&spec.raw_resistance);
    let evaluated = || {
        eval_expression(expression, &netlist.params).map_err(|error| {
            XyceAddResistorsMaterializationError::InvalidResistance {
                mode,
                raw_resistance: spec.raw_resistance.clone(),
                source_line: spec.source_line,
                reason: error.to_string(),
            }
        })
    };
    let resistance = if braced.is_some() || quoted.is_some() {
        evaluated()?
    } else {
        match parse_spice_value_complete(expression) {
            Ok(value) => value,
            Err(_) => evaluated()?,
        }
    };
    if resistance.is_nan() || (resistance.is_infinite() && resistance.is_sign_negative()) {
        return Err(XyceAddResistorsMaterializationError::InvalidResistance {
            mode,
            raw_resistance: spec.raw_resistance.clone(),
            source_line: spec.source_line,
            reason: "resistance must resolve to a real value or positive infinity".to_string(),
        });
    }
    Ok(XyceResolvedAddResistorSpec {
        mode,
        raw_resistance: spec.raw_resistance.clone(),
        resistance,
        source_line: spec.source_line,
    })
}

fn collect_existing_names(
    elements: &[Element],
    abort: &dyn AbortSignal,
) -> Result<HashSet<String>, XyceAddResistorsMaterializationError> {
    let mut names = HashSet::new();
    names
        .try_reserve(elements.len())
        .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;
    for (index, element) in elements.iter().enumerate() {
        poll(index, abort)?;
        names.insert(element.name.to_ascii_uppercase());
    }
    Ok(names)
}

fn build_artifact_node_projection(
    canonical_elements: &[Element],
    xyce_elements: &[Element],
    abort: &dyn AbortSignal,
) -> Result<HashMap<String, String>, XyceAddResistorsMaterializationError> {
    if canonical_elements.len() != xyce_elements.len() {
        return Err(
            XyceAddResistorsMaterializationError::ArtifactTopologyProjection {
                reason: format!(
                    "canonical and Xyce flattening produced {} and {} elements",
                    canonical_elements.len(),
                    xyce_elements.len()
                ),
            },
        );
    }

    let mut projection = HashMap::new();
    for (index, (canonical, xyce)) in canonical_elements.iter().zip(xyce_elements).enumerate() {
        poll(index, abort)?;
        if canonical.nodes.len() != xyce.nodes.len() {
            return Err(
                XyceAddResistorsMaterializationError::ArtifactTopologyProjection {
                    reason: format!(
                        "element '{}' has {} canonical terminals but {} Xyce terminals",
                        canonical.name,
                        canonical.nodes.len(),
                        xyce.nodes.len()
                    ),
                },
            );
        }
        for (canonical_node, xyce_node) in canonical.nodes.iter().zip(&xyce.nodes) {
            insert_artifact_node_projection(
                &mut projection,
                canonical_node,
                xyce_node,
                &canonical.name,
            )?;
        }
        match (&canonical.kind, &xyce.kind) {
            (
                ElementKind::Vcvs {
                    control_nodes: canonical_nodes,
                    ..
                }
                | ElementKind::Vccs {
                    control_nodes: canonical_nodes,
                    ..
                },
                ElementKind::Vcvs {
                    control_nodes: xyce_nodes,
                    ..
                }
                | ElementKind::Vccs {
                    control_nodes: xyce_nodes,
                    ..
                },
            ) => {
                insert_artifact_node_projection(
                    &mut projection,
                    &canonical_nodes.0,
                    &xyce_nodes.0,
                    &canonical.name,
                )?;
                insert_artifact_node_projection(
                    &mut projection,
                    &canonical_nodes.1,
                    &xyce_nodes.1,
                    &canonical.name,
                )?;
            }
            (
                ElementKind::VSwitch {
                    control_pos: canonical_pos,
                    control_neg: canonical_neg,
                    ..
                },
                ElementKind::VSwitch {
                    control_pos: xyce_pos,
                    control_neg: xyce_neg,
                    ..
                },
            ) => {
                insert_artifact_node_projection(
                    &mut projection,
                    canonical_pos,
                    xyce_pos,
                    &canonical.name,
                )?;
                insert_artifact_node_projection(
                    &mut projection,
                    canonical_neg,
                    xyce_neg,
                    &canonical.name,
                )?;
            }
            (canonical_kind, xyce_kind)
                if std::mem::discriminant(canonical_kind) != std::mem::discriminant(xyce_kind) =>
            {
                return Err(
                    XyceAddResistorsMaterializationError::ArtifactTopologyProjection {
                        reason: format!(
                            "element '{}' changed kind between canonical and Xyce flattening",
                            canonical.name
                        ),
                    },
                );
            }
            _ => {}
        }
    }
    check_abort(abort)?;
    Ok(projection)
}

fn insert_artifact_node_projection(
    projection: &mut HashMap<String, String>,
    canonical_node: &str,
    xyce_node: &str,
    element: &str,
) -> Result<(), XyceAddResistorsMaterializationError> {
    let key = canonical_node.to_ascii_uppercase();
    let xyce_node = xyce_node.to_ascii_uppercase();
    if let Some(first) = projection.get(&key)
        && first != &xyce_node
    {
        return Err(
            XyceAddResistorsMaterializationError::ArtifactTopologyProjection {
                reason: format!(
                    "canonical node '{canonical_node}' maps to both '{first}' and '{xyce_node}' while processing '{element}'"
                ),
            },
        );
    }
    projection.insert(key, xyce_node);
    Ok(())
}

fn append_report_records(
    report: &mut XyceAddResistorsReport,
    mode: XyceAddResistorMode,
    existing_names: &HashSet<String>,
    artifact_node_projection: &HashMap<String, String>,
    abort: &dyn AbortSignal,
) -> Result<(), XyceAddResistorsMaterializationError> {
    let Some(resolved) = report.resolved_modes.iter().find(|spec| spec.mode == mode) else {
        return Ok(());
    };
    let raw_resistance = resolved.raw_resistance.clone();
    let resistance = resolved.resistance;
    let candidates = match mode {
        XyceAddResistorMode::OneTerminal => &report.one_terminal_candidates,
        XyceAddResistorMode::NoDcPath => &report.no_dc_path_candidates,
    };
    for (index, node) in candidates.iter().enumerate() {
        poll(index, abort)?;
        let ordinal = index
            .checked_add(1)
            .ok_or(XyceAddResistorsMaterializationError::CapacityExceeded)?;
        let name = match mode {
            XyceAddResistorMode::OneTerminal => format!("RONETERM{ordinal}"),
            XyceAddResistorMode::NoDcPath => format!("RNODCPATH{ordinal}"),
        };
        if existing_names.contains(&name.to_ascii_uppercase()) {
            return Err(XyceAddResistorsMaterializationError::NameCollision { name });
        }
        let artifact_node = artifact_node_projection
            .get(&node.to_ascii_uppercase())
            .cloned()
            .ok_or_else(
                || XyceAddResistorsMaterializationError::ArtifactTopologyProjection {
                    reason: format!(
                        "generated candidate '{node}' has no corresponding Xyce node spelling"
                    ),
                },
            )?;
        report.generated.push(XyceGeneratedResistor {
            name,
            node: node.clone(),
            artifact_node,
            mode,
            raw_resistance: raw_resistance.clone(),
            resistance,
        });
    }
    Ok(())
}

fn clone_netlist_with_abort(
    source: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<Netlist, XyceAddResistorsMaterializationError> {
    check_abort(abort)?;
    let title = source.title.clone();
    check_abort(abort)?;
    let elements = clone_slice(&source.elements, abort)?;
    let analyses = clone_slice(&source.analyses, abort)?;
    let fft_analyses = clone_slice(&source.fft_analyses, abort)?;
    let data_tables = clone_slice(&source.data_tables, abort)?;
    let models = clone_slice(&source.models, abort)?;
    let subcircuits = clone_slice(&source.subcircuits, abort)?;
    check_abort(abort)?;
    let params = source.params.clone();
    check_abort(abort)?;
    let initial_conditions = clone_slice(&source.initial_conditions, abort)?;
    let device_initial_conditions = source.device_initial_conditions.clone();
    check_abort(abort)?;
    let node_sets = clone_slice(&source.node_sets, abort)?;
    let startup_directives = clone_slice(&source.startup_directives, abort)?;
    let global_nodes = clone_string_set(&source.global_nodes, abort)?;
    let measurements = clone_slice(&source.measurements, abort)?;
    check_abort(abort)?;
    let saves = source.saves.clone();
    check_abort(abort)?;
    let output_requests = clone_slice(&source.output_requests, abort)?;
    let options = source.options.clone();
    let veriloga_includes = clone_slice(&source.veriloga_includes, abort)?;
    let spef_includes = clone_slice(&source.spef_includes, abort)?;
    let diagnostics = clone_slice(&source.diagnostics, abort)?;
    let source_path = source.source_path.clone();
    check_abort(abort)?;
    Ok(Netlist {
        title,
        elements,
        analyses,
        fft_analyses,
        data_tables,
        models,
        subcircuits,
        params,
        initial_conditions,
        device_initial_conditions,
        node_sets,
        startup_directives,
        global_nodes,
        measurements,
        saves,
        output_requests,
        options,
        veriloga_includes,
        spef_includes,
        diagnostics,
        source_text: None,
        source_path,
    })
}

fn clone_slice<T: Clone>(
    source: &[T],
    abort: &dyn AbortSignal,
) -> Result<Vec<T>, XyceAddResistorsMaterializationError> {
    let mut cloned = Vec::new();
    cloned
        .try_reserve(source.len())
        .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;
    for (index, value) in source.iter().enumerate() {
        poll(index, abort)?;
        cloned.push(value.clone());
    }
    check_abort(abort)?;
    Ok(cloned)
}

fn clone_string_set(
    source: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<HashSet<String>, XyceAddResistorsMaterializationError> {
    let mut cloned = HashSet::new();
    cloned
        .try_reserve(source.len())
        .map_err(|_| XyceAddResistorsMaterializationError::CapacityExceeded)?;
    for (index, value) in source.iter().enumerate() {
        poll(index, abort)?;
        cloned.insert(value.clone());
    }
    check_abort(abort)?;
    Ok(cloned)
}

fn sort_nodes(nodes: &mut [String]) {
    nodes.sort_by_key(|node| node.to_ascii_uppercase());
}

fn poll(index: usize, abort: &dyn AbortSignal) -> Result<(), XyceAddResistorsMaterializationError> {
    poll_parse_abort(abort, index).map_err(|_| XyceAddResistorsMaterializationError::Aborted)
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), XyceAddResistorsMaterializationError> {
    ensure_parse_not_aborted(abort).map_err(|_| XyceAddResistorsMaterializationError::Aborted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::ImmediateAbort;
    use crate::netlist::{ExpressionDialect, NetlistParseOptions, flatten_netlist_with_models};

    fn parse(source: &str) -> Netlist {
        Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce ADDRESISTORS fixture parses")
    }

    #[test]
    fn parser_records_both_modes_raw_tokens_and_only_exact_extra_warning() {
        let netlist = parse(
            "typed addresistors\n\
             .PARAM RH=2k\n\
             .PREPROCESS\n\
             + ADDRESISTORS nodcpath {RH*2} ignored\n\
             .PREPROCESS ADDRESISTORS OneTerminal 1meg\n\
             V1 1 0 1\n\
             .END\n",
        );
        let policy = netlist
            .options
            .add_resistors
            .as_ref()
            .expect("typed policy retained");
        assert_eq!(
            policy
                .no_dc_path
                .as_ref()
                .map(|spec| spec.raw_resistance.as_str()),
            Some("{RH*2}")
        );
        assert_eq!(
            policy
                .one_terminal
                .as_ref()
                .map(|spec| spec.raw_resistance.as_str()),
            Some("1meg")
        );
        assert_eq!(netlist.diagnostics.len(), 1);
        assert_eq!(netlist.diagnostics[0].code, "addresistors-extra-parameters");
        assert_eq!(netlist.diagnostics[0].line, 3);
    }

    #[test]
    fn parser_applies_title_indentation_continuation_include_and_physical_eof_rules() {
        let title = parse(
            ".PREPROCESS ADDRESISTORS ONETERMINAL 9\n\
             V1 1 0 1\n\
             .END\n",
        );
        assert!(title.options.add_resistors.is_none());

        let indented =
            parse("indented inert\n  .PREPROCESS ADDRESISTORS ONETERMINAL 9\nV1 1 0 1\n.END\n");
        assert!(indented.options.add_resistors.is_none());

        let after_end = parse(
            "physical eof\n\
             V1 1 0 1\n\
             .END\n\
             .PREPROCESS\n\
             + ADDRESISTORS NODCPATH 7k\n",
        );
        assert_eq!(
            after_end
                .options
                .add_resistors
                .as_ref()
                .and_then(|policy| policy.no_dc_path.as_ref())
                .map(|spec| spec.raw_resistance.as_str()),
            Some("7k")
        );

        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rspice-addresistors-include-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("create include fixture");
        let root = directory.join("root.cir");
        let child = directory.join("child.inc");
        std::fs::write(
            &child,
            ".PREPROCESS ADDRESISTORS ONETERMINAL 99\nRCHILD a b 1\n",
        )
        .expect("write child");
        let source = "included control inert\n.include child.inc\n.PREPROCESS ADDRESISTORS NODCPATH 5k\n.END\n";
        std::fs::write(&root, source).expect("write root");
        let included = Netlist::parse_with_path(source, &root).expect("include fixture parses");
        assert!(
            included
                .options
                .add_resistors
                .as_ref()
                .is_some_and(|policy| policy.one_terminal.is_none() && policy.no_dc_path.is_some())
        );
        std::fs::remove_dir_all(directory).expect("remove include fixture");
    }

    #[test]
    fn parser_rejects_missing_unknown_and_duplicate_modes_at_physical_origin() {
        for (source, line, fragment) in [
            (
                "missing\n.PREPROCESS ADDRESISTORS ONETERMINAL\n.END\n",
                2,
                "Missing resistance",
            ),
            (
                "unknown\n.PREPROCESS ADDRESISTORS FLOATING 1k\n.END\n",
                2,
                "Unknown argument FLOATING",
            ),
            (
                "duplicate\n.PREPROCESS ADDRESISTORS NODCPATH 1k\n.END\n.PREPROCESS ADDRESISTORS nodcpath 2k\n",
                4,
                "Multiple .PREPROCESS ADDRESISTORS NODCPATH",
            ),
        ] {
            let error = Netlist::parse_with_options(
                source,
                NetlistParseOptions {
                    expression_dialect: ExpressionDialect::Xyce,
                    ..Default::default()
                },
            )
            .expect_err("invalid ADDRESISTORS control is fatal");
            match error {
                ParseError::Syntax {
                    line: actual,
                    message,
                } => {
                    assert_eq!(actual, line);
                    assert!(message.contains(fragment), "{message}");
                }
                other => panic!("unexpected error: {other}"),
            }
        }
    }

    #[test]
    fn materialization_is_explicit_stable_and_oneterminal_wins_overlap() {
        let netlist = parse(
            "RC connectivity\n\
             .PARAM RH=2k\n\
             .PREPROCESS ADDRESISTORS NODCPATH {RH*2}\n\
             .PREPROCESS ADDRESISTORS ONETERMINAL 1meg\n\
             R1 1 2 1k\n\
             C1 2 0 1u\n\
             .END\n",
        );
        let original_flat = flatten_netlist_with_models(&netlist).expect("original flattens");
        assert_eq!(original_flat.elements.len(), 2);
        assert!(
            original_flat
                .elements
                .iter()
                .all(|element| !element.name.starts_with("RONETERM")
                    && !element.name.starts_with("RNODCPATH"))
        );

        let original_circuit = crate::Engine::default()
            .build_circuit(&netlist)
            .expect("first-run circuit builds");
        assert_eq!(original_circuit.device_count(), 2);

        let materialized = netlist
            .materialize_xyce_add_resistors()
            .expect("derived copy materializes");
        assert_eq!(materialized.report.one_terminal_candidates, ["1"]);
        assert_eq!(materialized.report.no_dc_path_candidates, ["2"]);
        assert_eq!(
            materialized
                .report
                .resolved_modes
                .iter()
                .map(|spec| (spec.mode, spec.raw_resistance.as_str(), spec.resistance))
                .collect::<Vec<_>>(),
            [
                (XyceAddResistorMode::OneTerminal, "1meg", 1.0e6),
                (XyceAddResistorMode::NoDcPath, "{RH*2}", 4.0e3),
            ]
        );
        assert_eq!(
            materialized
                .report
                .generated
                .iter()
                .map(|generated| (
                    generated.name.as_str(),
                    generated.node.as_str(),
                    generated.resistance,
                ))
                .collect::<Vec<_>>(),
            [("RONETERM1", "1", 1.0e6), ("RNODCPATH1", "2", 4.0e3)]
        );
        assert!(materialized.netlist.options.add_resistors.is_none());
        assert_eq!(
            materialized.netlist.source_text.as_deref(),
            Some(materialized.derived_source.as_str())
        );
        assert!(
            materialized
                .derived_source
                .contains("* .PREPROCESS ADDRESISTORS")
        );
        assert!(materialized.derived_source.ends_with(".END\n"));
        assert_eq!(netlist.elements.len(), 2);
        assert!(netlist.options.add_resistors.is_some());
        assert!(netlist.source_text.is_some());
        let generated_circuit = crate::Engine::default()
            .build_circuit(&materialized.netlist)
            .expect("derived circuit builds");
        assert_eq!(generated_circuit.device_count(), 4);
    }

    #[test]
    fn nodcpath_handles_capacitive_leaf_and_rejects_a_dc_bridge() {
        let capacitive = parse(
            "capacitive leaf\n\
             .PREPROCESS ADDRESISTORS NODCPATH 8k\n\
             V1 1 0 1\n\
             C1 1 2 1u\n\
             .END\n",
        )
        .materialize_xyce_add_resistors()
        .expect("capacitive leaf materializes");
        assert_eq!(capacitive.report.no_dc_path_candidates, ["2"]);

        let bridged = parse(
            "dc bridge\n\
             .PREPROCESS ADDRESISTORS NODCPATH 8k\n\
             V1 1 0 1\n\
             R1 1 2 1k\n\
             .END\n",
        )
        .materialize_xyce_add_resistors()
        .expect("DC bridge materializes with no generated cards");
        assert!(bridged.report.no_dc_path_candidates.is_empty());
        assert!(bridged.report.generated.is_empty());
        assert!(bridged.netlist.options.add_resistors.is_none());
    }

    #[test]
    fn subcircuit_nodes_and_hierarchical_metadata_are_preserved() {
        let netlist = parse(
            "hierarchical candidate\n\
             .PREPROCESS ADDRESISTORS ONETERMINAL 9k\n\
             .SUBCKT CELL a\n\
             C1 a mid 1u\n\
             .IC V(mid)=0.25\n\
             .ENDS CELL\n\
             V1 in 0 1\n\
             X1 in CELL\n\
             .END\n",
        );
        let materialized = netlist
            .materialize_xyce_add_resistors()
            .expect("hierarchical copy materializes");
        assert_eq!(materialized.report.one_terminal_candidates, ["X1.MID"]);
        assert_eq!(materialized.netlist.subcircuits.len(), 1);
        assert_eq!(
            materialized.netlist.subcircuits[0].initial_conditions.len(),
            1
        );
        assert!(materialized.netlist.subcircuits[0].node_sets.is_empty());
        let generated = materialized
            .netlist
            .elements
            .last()
            .expect("generated resistor");
        assert_eq!(generated.nodes, ["X1.MID", "0"]);
        assert_eq!(materialized.report.generated[0].artifact_node, "X1:MID");
        assert!(
            materialized
                .derived_source
                .contains("RONETERM1 X1:MID 0 9k\n")
        );
        assert!(matches!(
            generated.provenance,
            ElementProvenance::GeneratedXyceAddResistor {
                mode: XyceAddResistorMode::OneTerminal
            }
        ));

        let nodeset = parse(
            "hierarchical nodeset\n\
             .PREPROCESS ADDRESISTORS ONETERMINAL 9k\n\
             .SUBCKT CELL a\n\
             C1 a mid 1u\n\
             .NODESET V(mid)=0.5\n\
             .ENDS CELL\n\
             V1 in 0 1\n\
             X1 in CELL\n\
             .END\n",
        )
        .materialize_xyce_add_resistors()
        .expect("hierarchical nodeset copy materializes");
        assert_eq!(nodeset.netlist.subcircuits[0].node_sets.len(), 1);
    }

    #[test]
    fn derived_source_projects_nested_xyce_hierarchy_without_rewriting_literal_periods() {
        let materialized = parse(
            "nested hierarchy spelling\n\
             .PREPROCESS ADDRESISTORS ONETERMINAL 9k\n\
             .SUBCKT LEAF a\n\
             C1 a inner 1u\n\
             .ENDS LEAF\n\
             .SUBCKT OUTER a\n\
             X2 a LEAF\n\
             .ENDS OUTER\n\
             V1 input 0 1\n\
             X1 input OUTER\n\
             V2 named.with.period 0 1\n\
             .END\n",
        )
        .materialize_xyce_add_resistors()
        .expect("nested hierarchy materializes");

        assert_eq!(
            materialized.report.one_terminal_candidates,
            ["NAMED.WITH.PERIOD", "X1.X2.INNER"]
        );
        assert_eq!(
            materialized
                .report
                .generated
                .iter()
                .map(|generated| (generated.node.as_str(), generated.artifact_node.as_str()))
                .collect::<Vec<_>>(),
            [
                ("NAMED.WITH.PERIOD", "NAMED.WITH.PERIOD"),
                ("X1.X2.INNER", "X1:X2:INNER"),
            ]
        );
        assert!(
            materialized
                .derived_source
                .contains("RONETERM1 NAMED.WITH.PERIOD 0 9k\n")
        );
        assert!(
            materialized
                .derived_source
                .contains("RONETERM2 X1:X2:INNER 0 9k\n")
        );

        let reparsed = parse(&materialized.derived_source);
        let flattened =
            flatten_netlist_with_models(&reparsed).expect("canonical Xyce artifact reparses");
        let generated = flattened
            .elements
            .iter()
            .find(|element| element.name == "RONETERM2")
            .expect("nested generated resistor survives replay");
        assert_eq!(generated.nodes, ["X1.X2.INNER", "0"]);
    }

    #[test]
    fn collision_invalid_resistance_missing_policy_and_abort_are_typed() {
        let collision = parse(
            "collision\n\
             .PREPROCESS ADDRESISTORS ONETERMINAL 1k\n\
             RONETERM1 a b 1\n\
             .END\n",
        )
        .materialize_xyce_add_resistors()
        .expect_err("generated name collision is fatal");
        assert!(matches!(
            collision,
            XyceAddResistorsMaterializationError::NameCollision { ref name }
                if name == "RONETERM1"
        ));

        for raw in ["{UNKNOWN+}"] {
            let source = format!(
                "invalid resistance\n.PREPROCESS ADDRESISTORS ONETERMINAL {raw}\nV1 1 0 1\n.END\n"
            );
            let error = parse(&source)
                .materialize_xyce_add_resistors()
                .expect_err("invalid resistance is rejected at materialization");
            assert!(matches!(
                error,
                XyceAddResistorsMaterializationError::InvalidResistance { .. }
            ));
        }

        for (raw, expected) in [("0", 0.0), ("-1", -1.0), ("1e999", Value::INFINITY)] {
            let source = format!(
                "finite resistor\n.PREPROCESS ADDRESISTORS ONETERMINAL {raw}\nV1 1 0 1\n.END\n"
            );
            let materialized = parse(&source)
                .materialize_xyce_add_resistors()
                .expect("canonical resistor value is retained");
            assert_eq!(materialized.report.generated[0].resistance, expected);
            crate::Engine::default()
                .build_circuit(&materialized.netlist)
                .expect("materialized zero, negative, and open resistors build safely");
        }

        let no_policy = parse("no policy\nV1 1 0 1\n.END\n")
            .materialize_xyce_add_resistors()
            .expect_err("missing policy is explicit");
        assert!(matches!(
            no_policy,
            XyceAddResistorsMaterializationError::MissingPolicy
        ));

        let abort_netlist =
            parse("abort\n.PREPROCESS ADDRESISTORS ONETERMINAL 1k\nV1 1 0 1\n.END\n");
        let aborted = abort_netlist
            .materialize_xyce_add_resistors_with_abort(&ImmediateAbort)
            .expect_err("entry cancellation is typed");
        assert!(matches!(
            aborted,
            XyceAddResistorsMaterializationError::Aborted
        ));
        assert_eq!(abort_netlist.elements.len(), 1);
    }

    #[test]
    fn resistance_resolution_consumes_the_full_token_and_supports_xyce_forms() {
        for (raw, expected) in [
            ("1+2", 3.0),
            ("1G", 1.0e9),
            ("10kOhm", 1.0e4),
            ("1bogus", 1.0),
            ("{1+2}", 3.0),
            ("'1+2'", 3.0),
        ] {
            let source = format!(
                "strict resistance\n.PREPROCESS ADDRESISTORS ONETERMINAL {raw}\nV1 1 0 1\n.END\n"
            );
            let materialized = parse(&source)
                .materialize_xyce_add_resistors()
                .expect("complete numeric/expression form resolves");
            assert_eq!(materialized.report.resolved_modes[0].raw_resistance, raw);
            assert_eq!(materialized.report.resolved_modes[0].resistance, expected);
            let reparsed = parse(&materialized.derived_source);
            let flattened = flatten_netlist_with_models(&reparsed)
                .expect("rendered resistance form reparses and flattens");
            let rendered = flattened
                .elements
                .iter()
                .find(|element| element.name == "RONETERM1")
                .expect("rendered resistor exists");
            let rendered_value = match rendered.kind {
                ElementKind::Resistor { value, .. } => value,
                ref other => panic!("rendered {raw} as unexpected element {other:?}"),
            };
            assert_eq!(rendered_value, expected, "raw artifact token {raw}");
        }
    }

    #[test]
    fn zero_candidate_mode_preserves_raw_value_without_evaluating_it() {
        let materialized = parse(
            "red herring resistance\n\
             .PREPROCESS ADDRESISTORS NODCPATH {NOT_DEFINED+}\n\
             V1 1 0 1\n\
             R1 1 2 1k\n\
             .END\n",
        )
        .materialize_xyce_add_resistors()
        .expect("unused resistance is not evaluated");
        assert_eq!(materialized.report.configured_modes.len(), 1);
        assert_eq!(
            materialized.report.configured_modes[0].raw_resistance,
            "{NOT_DEFINED+}"
        );
        assert!(materialized.report.resolved_modes.is_empty());
        assert!(materialized.report.generated.is_empty());
    }

    #[test]
    fn replaceground_removeunused_and_generated_provenance_interact_exactly() {
        let netlist = parse(
            "preprocess interaction\n\
             .PREPROCESS ADDRESISTORS ONETERMINAL 3k\n\
             .PREPROCESS REMOVEUNUSED R\n\
             .PREPROCESS REPLACEGROUND TRUE\n\
             R_DROP GND GROUND 1\n\
             V1 keep GND 1\n\
             .END\n",
        );
        assert_eq!(netlist.elements.len(), 1);
        assert_eq!(netlist.elements[0].nodes, ["KEEP", "0"]);
        let materialized = netlist
            .materialize_xyce_add_resistors()
            .expect("interacting policies materialize");
        assert_eq!(materialized.report.one_terminal_candidates, ["KEEP"]);
        let flattened = flatten_netlist_with_models(&materialized.netlist)
            .expect("derived policy interaction flattens");
        assert_eq!(flattened.elements.len(), 2);
        assert!(flattened.elements.iter().any(|element| matches!(
            element.provenance,
            ElementProvenance::GeneratedXyceAddResistor { .. }
        )));
    }

    #[test]
    fn derived_source_is_deterministic_root_only_and_reparse_equivalent() {
        let source = concat!(
            ".PREPROCESS ADDRESISTORS ONETERMINAL 99\r\n",
            ".PARAM RH=4k\r\n",
            ".PREPROCESS\r\n",
            "+ ADDRESISTORS ONETERMINAL {RH}\r\n",
            ".SUBCKT CELL a\r\n",
            "C1 a leaf 1u\r\n",
            ".ENDS CELL\r\n",
            "V1 in 0 1\r\n",
            "X1 in CELL\r\n",
            ".END\r\n",
            "R_AFTER ignored 0 1\r\n",
            ".PREPROCESS ADDRESISTORS NODCPATH 7k\r\n",
        );
        let original = parse(source);
        let materialized = original
            .materialize_xyce_add_resistors()
            .expect("derived root artifact materializes");
        assert_eq!(
            materialized.derived_source,
            concat!(
                ".PREPROCESS ADDRESISTORS ONETERMINAL 99\r\n",
                ".PARAM RH=4k\r\n",
                "* .PREPROCESS\r\n",
                "* + ADDRESISTORS ONETERMINAL {RH}\r\n",
                ".SUBCKT CELL a\r\n",
                "C1 a leaf 1u\r\n",
                ".ENDS CELL\r\n",
                "V1 in 0 1\r\n",
                "X1 in CELL\r\n",
                "RONETERM1 X1:LEAF 0 4000\r\n",
                ".END\r\n",
            )
        );
        assert!(!materialized.derived_source.contains("R_AFTER"));
        assert_eq!(
            materialized
                .derived_source
                .lines()
                .filter(|line| line.eq_ignore_ascii_case(".END"))
                .count(),
            1
        );

        let reparsed = parse(&materialized.derived_source);
        assert!(reparsed.options.add_resistors.is_none());
        let expected = flatten_netlist_with_models(&materialized.netlist)
            .expect("semantic materialization flattens");
        let actual =
            flatten_netlist_with_models(&reparsed).expect("artifact reparses and flattens");
        let snapshot = |elements: &[Element]| {
            elements
                .iter()
                .map(|element| {
                    let value = match element.kind {
                        ElementKind::Resistor { value, .. } => Some(value),
                        ElementKind::Capacitor { value, .. } => Some(value),
                        _ => None,
                    };
                    (element.name.clone(), element.nodes.clone(), value)
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(snapshot(&expected.elements), snapshot(&actual.elements));
        assert_eq!(original.source_text.as_deref(), Some(source));
    }

    #[test]
    fn derived_source_zero_candidates_and_missing_source_are_explicit() {
        let original = parse(
            "zero artifact\n.PREPROCESS ADDRESISTORS NODCPATH {UNUSED+}\nV1 1 0 1\nR1 1 2 1k\n.END\n",
        );
        let materialized = original
            .materialize_xyce_add_resistors()
            .expect("zero-candidate copy still renders");
        assert!(materialized.report.generated.is_empty());
        assert_eq!(
            materialized.derived_source,
            "zero artifact\n* .PREPROCESS ADDRESISTORS NODCPATH {UNUSED+}\nV1 1 0 1\nR1 1 2 1k\n.END\n"
        );
        let reparsed = parse(&materialized.derived_source);
        assert_eq!(
            flatten_netlist_with_models(&materialized.netlist)
                .expect("semantic zero copy flattens")
                .elements
                .len(),
            flatten_netlist_with_models(&reparsed)
                .expect("rendered zero copy flattens")
                .elements
                .len()
        );

        let mut unavailable = original;
        unavailable.source_text = None;
        let error = unavailable
            .materialize_xyce_add_resistors()
            .expect_err("missing root source cannot fabricate an artifact");
        assert!(matches!(
            error,
            XyceAddResistorsMaterializationError::MissingSourceText
        ));
    }
}

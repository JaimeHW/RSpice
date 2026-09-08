//! Versioned import compatibility and the source adapters it qualifies.

use super::NetlistSourceDialect;
use rspice_core::{
    config::ExpressionDialect,
    netlist::{is_spice_end_card, strip_spice_inline_comment},
};
use serde::{Deserialize, Serialize};

/// Versioned, persisted netlist-semantics contract.
///
/// The source dialect is user-facing provenance; this profile is the exact
/// parser/device-default contract authenticated into prepared runs and worker
/// messages. Versioned variants ensure a future compatibility change cannot
/// silently reinterpret a previously reviewed project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetlistExecutionProfile {
    #[default]
    RSpiceCanonicalV1,
    Spice3NgspiceV1,
    Spice3NgspiceV2,
    HspiceDeclarativeV1,
    PspiceDeclarativeV1,
    PspiceDeclarativeV2,
    SpectreSpiceV1,
    AdsSpiceExportV1,
}

impl NetlistExecutionProfile {
    pub const fn id(self) -> &'static str {
        match self {
            Self::RSpiceCanonicalV1 => "rspice-canonical/1",
            Self::Spice3NgspiceV1 => "spice3-ngspice/1",
            Self::Spice3NgspiceV2 => "spice3-ngspice/2",
            Self::HspiceDeclarativeV1 => "hspice-declarative/1",
            Self::PspiceDeclarativeV1 => "pspice-declarative/1",
            Self::PspiceDeclarativeV2 => "pspice-declarative/2",
            Self::SpectreSpiceV1 => "spectre-spice/1",
            Self::AdsSpiceExportV1 => "ads-spice-export/1",
        }
    }

    pub const fn source_dialect(self) -> NetlistSourceDialect {
        match self {
            Self::RSpiceCanonicalV1 => NetlistSourceDialect::RSpice,
            Self::Spice3NgspiceV1 | Self::Spice3NgspiceV2 => NetlistSourceDialect::Spice3Ngspice,
            Self::HspiceDeclarativeV1 => NetlistSourceDialect::Hspice,
            Self::PspiceDeclarativeV1 | Self::PspiceDeclarativeV2 => NetlistSourceDialect::Pspice,
            Self::SpectreSpiceV1 => NetlistSourceDialect::Spectre,
            Self::AdsSpiceExportV1 => NetlistSourceDialect::Ads,
        }
    }

    pub const fn spice_dialect(self) -> rspice_core::SpiceDialect {
        match self {
            Self::RSpiceCanonicalV1 => rspice_core::SpiceDialect::BestAvailable,
            Self::Spice3NgspiceV1 | Self::Spice3NgspiceV2 => rspice_core::SpiceDialect::Ngspice,
            Self::HspiceDeclarativeV1 => rspice_core::SpiceDialect::BestAvailable,
            Self::PspiceDeclarativeV1 | Self::PspiceDeclarativeV2 => {
                rspice_core::SpiceDialect::BestAvailable
            }
            Self::SpectreSpiceV1 => rspice_core::SpiceDialect::BestAvailable,
            Self::AdsSpiceExportV1 => rspice_core::SpiceDialect::BestAvailable,
        }
    }

    pub const fn expression_dialect(self) -> rspice_core::config::ExpressionDialect {
        match self {
            Self::RSpiceCanonicalV1
            | Self::Spice3NgspiceV1
            | Self::Spice3NgspiceV2
            | Self::HspiceDeclarativeV1
            | Self::PspiceDeclarativeV1
            | Self::PspiceDeclarativeV2
            | Self::SpectreSpiceV1
            | Self::AdsSpiceExportV1 => rspice_core::config::ExpressionDialect::Ngspice,
        }
    }

    /// Reject source constructs outside the reviewed version's semantics.
    pub fn validate_source(self, source: &str) -> Result<(), String> {
        self.validate_profile_source(source, false)
    }

    /// Validate the line-preserving parser input produced by the adapter.
    pub fn validate_executable_source(self, source: &str) -> Result<(), String> {
        self.validate_profile_source(source, true)
    }

    fn marker(self, line: &str) -> bool {
        match self {
            Self::HspiceDeclarativeV1 => is_hspice_presentation_directive(line),
            Self::PspiceDeclarativeV1 => is_pspice_probe_marker(line),
            Self::PspiceDeclarativeV2 => is_pspice_v2_probe_marker(line),
            Self::SpectreSpiceV1 => line.eq_ignore_ascii_case("simulator lang=spice"),
            Self::AdsSpiceExportV1 => is_ads_spice_export_header(line),
            _ => false,
        }
    }

    fn adapts_presentation(self) -> bool {
        matches!(
            self,
            Self::HspiceDeclarativeV1 | Self::SpectreSpiceV1 | Self::AdsSpiceExportV1
        )
    }

    fn validate_profile_source(self, source: &str, executable: bool) -> Result<(), String> {
        let marker_name = match self {
            Self::HspiceDeclarativeV1 => {
                Some("qualified .OPTION POST or .PROTECT/.UNPROTECT marker")
            }
            Self::PspiceDeclarativeV1 => Some("PSpice .PROBE marker"),
            Self::PspiceDeclarativeV2 => Some("PSpice .PROBE, .PROBE64, or .PROBE/CSDF marker"),
            Self::SpectreSpiceV1 => Some("simulator lang=spice"),
            Self::AdsSpiceExportV1 => Some("qualified ADS SPICE-export Options header"),
            _ => None,
        };
        if let Some(marker_name) = marker_name {
            let exactly_one = matches!(self, Self::SpectreSpiceV1 | Self::AdsSpiceExportV1);
            validate_profile_markers(
                source,
                self.id(),
                marker_name,
                exactly_one,
                |index, line| {
                    if index == 0 && !exactly_one {
                        return false;
                    }
                    if executable && self.adapts_presentation() && index != 0 {
                        is_profile_adapter_receipt(line, self.id(), |original| {
                            self.marker(original)
                        })
                    } else {
                        self.marker(line)
                    }
                },
            )?;
        }
        self.validate_resolved_source(source)
    }

    /// Diagnostics that make a reviewed source semantically incomplete.
    pub(crate) fn diagnostic_is_semantic_loss(code: &str) -> bool {
        matches!(
            code,
            "unknown-option"
                | "unsupported-dot-command"
                | "control-command-dropped"
                | "invalid-option-defaulted"
        )
    }

    /// Validate constructs discovered only after canonical parsing and include
    /// expansion. Text-only profile checks cannot see a `PWL FILE` card inside
    /// an included deck, so import review must apply this second boundary to
    /// the resolved AST before calling the source executable.
    pub fn validate_parsed_netlist(self, netlist: &rspice_core::Netlist) -> Result<(), String> {
        if matches!(self, Self::PspiceDeclarativeV1 | Self::PspiceDeclarativeV2)
            && let Some((element, path)) = parsed_file_backed_pwl(netlist)
        {
            return Err(format!(
                "profile {} rejects unsealed file-backed PWL input on element '{}' ({})",
                self.id(),
                element,
                path
            ));
        }
        if self != Self::RSpiceCanonicalV1
            && netlist
                .options
                .spice_dialect
                .is_some_and(|dialect| dialect != self.spice_dialect())
        {
            return Err(format!(
                "profile {} conflicts with the authored RSPICE_DIALECT option",
                self.id()
            ));
        }
        Ok(())
    }

    /// Includes may change the root's presentation receipts. Its provenance
    /// was checked before expansion; the resolved body must still obey the
    /// profile's executable-command restrictions.
    pub(crate) fn validate_resolved_source(self, source: &str) -> Result<(), String> {
        if self == Self::Spice3NgspiceV2 {
            return visit_ngspice_v2_source(source, |_, _, _, _| {});
        }
        if matches!(self, Self::PspiceDeclarativeV1 | Self::PspiceDeclarativeV2)
            && let Some(line) = pspice_file_backed_pwl_line(source)
        {
            return Err(format!(
                "profile {} rejects unsealed file-backed PWL input on line {line}",
                self.id()
            ));
        }
        let rejected = source
            .lines()
            .enumerate()
            .skip(1)
            .take_while(|(_, raw)| !is_spice_end_card(raw, ExpressionDialect::Ngspice))
            .find_map(|(index, raw)| {
                let trimmed = code(raw);
                let head = trimmed.split_whitespace().next()?;
                let normalized = head.to_ascii_lowercase();
                let unsupported = match self {
                    Self::Spice3NgspiceV1 => matches!(
                        normalized.as_str(),
                        ".control" | ".endc" | "wrdata" | "setplot"
                    ),
                    Self::Spice3NgspiceV2 => unreachable!("handled before the generic scan"),
                    Self::PspiceDeclarativeV1 => {
                        matches!(
                            normalized.as_str(),
                            ".distribution" | ".stimulus" | ".probe64" | ".probe/csdf"
                        ) || normalized.starts_with(".probe/")
                            || is_pspice_v2_extended_source_line(trimmed)
                    }
                    Self::PspiceDeclarativeV2 => {
                        normalized == ".stimulus"
                            || (normalized.starts_with(".probe/") && normalized != ".probe/csdf")
                    }
                    Self::HspiceDeclarativeV1 => {
                        if normalized == ".alter" {
                            true
                        } else if normalized == ".protect" || normalized == ".unprotect" {
                            !matches!(
                                trimmed.to_ascii_lowercase().as_str(),
                                ".protect" | ".unprotect"
                            )
                        } else if normalized == ".option" {
                            !is_hspice_presentation_directive(trimmed)
                        } else {
                            false
                        }
                    }
                    Self::SpectreSpiceV1 => {
                        let lower = trimmed.to_ascii_lowercase();
                        (normalized == "simulator" && lower != "simulator lang=spice")
                            || matches!(
                                normalized.as_str(),
                                "ahdl_include" | "parameters" | "saveoptions"
                            )
                    }
                    Self::AdsSpiceExportV1 => {
                        matches!(
                            normalized.as_str(),
                            "#uselib" | "define" | "simulatoroptions"
                        ) || (normalized == "options" && !is_ads_spice_export_header(trimmed))
                    }
                    Self::RSpiceCanonicalV1 => false,
                };
                unsupported.then(|| (index + 1, head.to_owned()))
            });
        if let Some((line, command)) = rejected {
            return Err(format!(
                "profile {} rejects unsupported command '{command}' on line {line}",
                self.id()
            ));
        }
        Ok(())
    }

    /// Convert the explicitly qualified, simulation-neutral surface syntax
    /// of a foreign profile into the canonical parser input. The retained
    /// project source is never modified. Replacements preserve physical line
    /// count so parser diagnostics and source mapping remain exact.
    pub fn adapt_source<'a>(self, source: &'a str) -> Result<std::borrow::Cow<'a, str>, String> {
        if self == Self::Spice3NgspiceV2 {
            let mut adapted = String::with_capacity(source.len());
            visit_ngspice_v2_source(source, |raw, body, ending, kind| match kind {
                ControlRecord::Keep => adapted.push_str(raw),
                ControlRecord::Command => {
                    adapted.push('.');
                    adapted.push_str(body.trim().strip_prefix('.').unwrap_or(body.trim()));
                    adapted.push_str(ending);
                }
                ControlRecord::Boundary | ControlRecord::Comment => {
                    let kind = if matches!(kind, ControlRecord::Boundary) {
                        "declarative control boundary"
                    } else {
                        "control comment"
                    };
                    adapted.push_str(&format!(
                        "* RSpice {} {kind}: {}{ending}",
                        self.id(),
                        body.trim()
                    ));
                }
            })?;
            return Ok(if adapted == source {
                std::borrow::Cow::Borrowed(source)
            } else {
                std::borrow::Cow::Owned(adapted)
            });
        }
        self.validate_source(source)?;
        if !self.adapts_presentation() {
            return Ok(std::borrow::Cow::Borrowed(source));
        }
        let mut adapted = String::with_capacity(source.len());
        let mut seen_end = false;
        for (index, raw) in source.split_inclusive('\n').enumerate() {
            let (body, ending) = record_parts(raw);
            if index != 0 && !seen_end && self.marker(code(body)) {
                adapted.push_str(&format!(
                    "* RSpice {} presentation directive: {}{ending}",
                    self.id(),
                    body.trim()
                ));
            } else {
                adapted.push_str(raw);
            }
            seen_end |= index != 0 && is_spice_end_card(body, self.expression_dialect());
        }
        Ok(if adapted == source {
            std::borrow::Cow::Borrowed(source)
        } else {
            std::borrow::Cow::Owned(adapted)
        })
    }
}

fn code(raw: &str) -> &str {
    strip_spice_inline_comment(raw, ExpressionDialect::Ngspice).trim()
}

fn record_parts(raw: &str) -> (&str, &str) {
    raw.strip_suffix("\r\n").map_or_else(
        || {
            raw.strip_suffix('\n')
                .map_or((raw, ""), |body| (body, "\n"))
        },
        |body| (body, "\r\n"),
    )
}

fn validate_profile_markers(
    source: &str,
    profile: &str,
    marker_name: &str,
    exactly_one: bool,
    is_marker: impl Fn(usize, &str) -> bool,
) -> Result<(), String> {
    let mut marker_line = None;
    let mut seen_end = false;
    for (index, raw) in source.lines().enumerate() {
        let line = index + 1;
        if is_marker(index, code(raw)) {
            if seen_end {
                return Err(format!(
                    "profile {profile} rejects {marker_name} after .END on line {line}"
                ));
            }
            if exactly_one && let Some(first_line) = marker_line {
                return Err(format!(
                    "profile {profile} rejects duplicate {marker_name} on line {line}; first declared on line {first_line}"
                ));
            }
            marker_line.get_or_insert(line);
        }
        // A vendor export header may supply provenance on the title record,
        // but that record never terminates an otherwise valid SPICE deck.
        seen_end |= index != 0 && is_spice_end_card(raw, ExpressionDialect::Ngspice);
    }
    if marker_line.is_none() {
        let count = if exactly_one {
            "exactly one"
        } else {
            "at least one pre-.END"
        };
        return Err(format!("profile {profile} requires {count} {marker_name}"));
    }
    Ok(())
}

fn is_hspice_presentation_directive(line: &str) -> bool {
    let lower = line.trim().to_ascii_lowercase();
    if matches!(lower.as_str(), ".protect" | ".unprotect") {
        return true;
    }
    let Some((head, tail)) = lower.split_once(char::is_whitespace) else {
        return false;
    };
    if head != ".option" {
        return false;
    }
    let compact = tail
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    matches!(compact.as_str(), "post" | "post=0" | "post=1" | "post=2")
}

fn is_pspice_probe_marker(line: &str) -> bool {
    line.split_whitespace().next().is_some_and(|head| {
        let lower = head.to_ascii_lowercase();
        lower == ".probe" || lower.starts_with(".probe/")
    })
}

fn is_pspice_v2_probe_marker(line: &str) -> bool {
    line.split_whitespace().next().is_some_and(|head| {
        let lower = head.to_ascii_lowercase();
        matches!(lower.as_str(), ".probe" | ".probe64" | ".probe/csdf")
    })
}

/// Source forms first qualified by `pspice-declarative/2`. Keeping this scan
/// in the v1 validator prevents a persisted v1 review from silently acquiring
/// syntax that the v1 parser boundary rejected.
fn is_pspice_v2_extended_source_line(line: &str) -> bool {
    let trimmed = line.trim();
    let Some(head) = trimmed.split_whitespace().next() else {
        return false;
    };
    let rest = trimmed[head.len()..].trim_start();

    if head.eq_ignore_ascii_case(".lib") {
        let Some(first) = rest.chars().next() else {
            return false;
        };
        let operand = rest
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches(|character| matches!(character, '"' | '\''));
        return matches!(first, '"' | '\'')
            || operand.contains('/')
            || operand.contains('\\')
            || std::path::Path::new(operand).extension().is_some();
    }

    if head.eq_ignore_ascii_case(".nodeset") || head.eq_ignore_ascii_case(".ic") {
        return rest
            .split_whitespace()
            .any(|target| target.starts_with('('));
    }

    if head.eq_ignore_ascii_case(".mc") {
        return rest.split_whitespace().nth(1).is_some_and(|analysis| {
            matches!(analysis.to_ascii_lowercase().as_str(), "dc" | "ac" | "tran")
        });
    }

    let tokens = rest
        .to_ascii_lowercase()
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if tokens.iter().any(|token| token == "chebyshev")
        && (head == "+"
            || head.as_bytes().first().is_some_and(|byte| {
                byte.eq_ignore_ascii_case(&b'e') || byte.eq_ignore_ascii_case(&b'g')
            }))
    {
        return true;
    }
    if head.eq_ignore_ascii_case(".options") && tokens.iter().any(|token| token == "advconv") {
        return true;
    }
    if head.eq_ignore_ascii_case(".autoconverge") {
        return true;
    }
    if head.eq_ignore_ascii_case(".model") && tokens.get(1).is_some_and(|token| token == "trn") {
        return true;
    }
    if head
        .as_bytes()
        .first()
        .is_some_and(|byte| byte.eq_ignore_ascii_case(&b't'))
        && tokens.iter().any(|token| token == "ic")
    {
        return true;
    }
    if head
        .as_bytes()
        .first()
        .is_some_and(|byte| byte.eq_ignore_ascii_case(&b't'))
        && tokens.iter().any(|token| token == "len")
        && tokens
            .iter()
            .any(|token| matches!(token.as_str(), "r" | "l" | "g" | "c"))
    {
        return true;
    }

    let body = trimmed.strip_prefix('+').unwrap_or(trimmed).trim_start();
    let has_pwl_token = body.split_whitespace().any(|token| {
        let token = token.trim_start_matches('+').to_ascii_lowercase();
        token == "pwl" || token.starts_with("pwl(")
    });
    if !has_pwl_token {
        return false;
    }
    let compact = body
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase();
    compact.contains(")(")
        || compact.contains("repeatforever")
        || compact.contains("repeatfor")
        || compact.contains("time_scale_factor=")
        || compact.contains("value_scale_factor=")
}

/// Return the first physical line of a PSpice logical source card that combines
/// `PWL` with `FILE`. Continuation folding matters because Capture commonly
/// emits `PWL` on the element line and `+ FILE "..."` on the next line.
fn pspice_file_backed_pwl_line(source: &str) -> Option<usize> {
    let mut logical = String::new();
    let mut first_line = 0usize;

    let classify = |logical: &str| {
        let tokens = logical
            .to_ascii_lowercase()
            .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
            .filter(|token| !token.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        tokens.iter().any(|token| token == "pwl") && tokens.iter().any(|token| token == "file")
    };

    for (index, raw) in source
        .lines()
        .enumerate()
        .skip(1)
        .take_while(|(_, raw)| !is_spice_end_card(raw, ExpressionDialect::Ngspice))
    {
        let line = index + 1;
        let trimmed = code(raw);
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }
        if let Some(continuation) = trimmed.strip_prefix('+') {
            if !logical.is_empty() {
                logical.push(' ');
                logical.push_str(continuation.trim_start());
            }
            continue;
        }
        if !logical.is_empty() && classify(&logical) {
            return Some(first_line);
        }
        logical.clear();
        logical.push_str(trimmed);
        first_line = line;
    }
    (!logical.is_empty() && classify(&logical)).then_some(first_line)
}

fn parsed_file_backed_pwl(netlist: &rspice_core::Netlist) -> Option<(String, String)> {
    fn source_path(spec: &rspice_core::netlist::SourceSpec) -> Option<&str> {
        match spec {
            rspice_core::netlist::SourceSpec::PwlFile { path, .. } => Some(path),
            rspice_core::netlist::SourceSpec::Distortion { inner, .. }
            | rspice_core::netlist::SourceSpec::RfPort { inner, .. }
            | rspice_core::netlist::SourceSpec::DcTransient {
                transient: inner, ..
            }
            | rspice_core::netlist::SourceSpec::DcAcTransient {
                transient: inner, ..
            } => source_path(inner),
            _ => None,
        }
    }

    fn elements(elements: &[rspice_core::netlist::Element]) -> Option<(String, String)> {
        elements.iter().find_map(|element| {
            let spec = match &element.kind {
                rspice_core::netlist::ElementKind::VoltageSource(spec)
                | rspice_core::netlist::ElementKind::CurrentSource(spec) => spec,
                _ => return None,
            };
            source_path(spec).map(|path| (element.name.clone(), path.to_owned()))
        })
    }

    fn subcircuits(
        definitions: &[rspice_core::netlist::SubcircuitDef],
    ) -> Option<(String, String)> {
        definitions.iter().find_map(|definition| {
            elements(&definition.elements).or_else(|| subcircuits(&definition.nested_subcircuits))
        })
    }

    elements(&netlist.elements).or_else(|| subcircuits(&netlist.subcircuits))
}

fn is_profile_adapter_receipt(
    line: &str,
    profile: &str,
    is_original_marker: impl Fn(&str) -> bool,
) -> bool {
    line.strip_prefix("* RSpice ")
        .and_then(|rest| rest.strip_prefix(profile))
        .and_then(|rest| rest.strip_prefix(" presentation directive: "))
        .is_some_and(|original| is_original_marker(original.trim()))
}

#[derive(Clone, Copy)]
enum ControlRecord {
    Keep,
    Boundary,
    Comment,
    Command,
}

/// Validation and rewriting share one state machine. A semicolon outside
/// quotes starts a comment in ngspice control language, too.
fn visit_ngspice_v2_source(
    source: &str,
    mut visit: impl FnMut(&str, &str, &str, ControlRecord),
) -> Result<(), String> {
    let profile = NetlistExecutionProfile::Spice3NgspiceV2.id();
    let mut in_control = false;
    let mut seen_end = false;
    for (index, raw) in source.split_inclusive('\n').enumerate() {
        let (body, ending) = record_parts(raw);
        let trimmed = code(body);
        let head = trimmed.split_whitespace().next().unwrap_or("");
        let kind = if index == 0 {
            ControlRecord::Keep
        } else if head.eq_ignore_ascii_case(".control") {
            if seen_end {
                return Err(format!(
                    "profile {profile} rejects .CONTROL after .END on line {}",
                    index + 1
                ));
            }
            if in_control || !trimmed.eq_ignore_ascii_case(".control") {
                return Err(format!(
                    "profile {profile} rejects malformed or nested .CONTROL on line {}",
                    index + 1
                ));
            }
            in_control = true;
            ControlRecord::Boundary
        } else if head.eq_ignore_ascii_case(".endc") {
            if !in_control || !trimmed.eq_ignore_ascii_case(".endc") {
                return Err(format!(
                    "profile {profile} rejects unmatched or malformed .ENDC on line {}",
                    index + 1
                ));
            }
            in_control = false;
            ControlRecord::Boundary
        } else if in_control {
            if trimmed.is_empty() || trimmed.starts_with('*') {
                ControlRecord::Keep
            } else if trimmed.starts_with('$') {
                ControlRecord::Comment
            } else {
                let command = trimmed.strip_prefix('.').unwrap_or(trimmed);
                if !is_ngspice_v2_promotable_control_command(command) {
                    return Err(format!(
                        "profile {profile} rejects unsupported or malformed control command '{}' on line {}",
                        command.split_whitespace().next().unwrap_or(""),
                        index + 1
                    ));
                }
                ControlRecord::Command
            }
        } else {
            if !seen_end && matches!(head.to_ascii_lowercase().as_str(), "wrdata" | "setplot") {
                return Err(format!(
                    "profile {profile} rejects unsupported command '{head}' on line {}",
                    index + 1
                ));
            }
            seen_end |= is_spice_end_card(body, ExpressionDialect::Ngspice);
            ControlRecord::Keep
        };
        visit(raw, body, ending, kind);
    }
    if in_control {
        return Err(format!(
            "profile {profile} rejects .CONTROL without a matching .ENDC"
        ));
    }
    Ok(())
}

fn is_ngspice_v2_promotable_control_command(body: &str) -> bool {
    let tokens = body.split_whitespace().collect::<Vec<_>>();
    let Some(command) = tokens.first().map(|token| token.to_ascii_lowercase()) else {
        return false;
    };
    match command.as_str() {
        "op" => tokens.len() == 1,
        "save" => tokens.len() >= 2,
        "tran" => tokens.len() >= 3,
        "dc" | "ac" | "sp" => tokens.len() >= 5,
        "meas" | "measure" => {
            tokens.len() >= 5
                && matches!(
                    tokens[3].to_ascii_lowercase().as_str(),
                    "avg" | "max" | "min" | "pp" | "rms" | "integ"
                )
        }
        _ => false,
    }
}

fn is_ads_spice_export_header(line: &str) -> bool {
    let mut tokens = line.split_whitespace();
    if !tokens
        .next()
        .is_some_and(|head| head.eq_ignore_ascii_case("options"))
    {
        return false;
    }
    let mut resource_usage = false;
    let mut nutmeg_format = false;
    let mut top_design_name = false;
    for token in tokens {
        let lower = token.to_ascii_lowercase();
        if matches!(lower.as_str(), "resourceusage=yes" | "resourceusage=no") {
            if resource_usage {
                return false;
            }
            resource_usage = true;
        } else if matches!(lower.as_str(), "usenutmegformat=yes" | "usenutmegformat=no") {
            if nutmeg_format {
                return false;
            }
            nutmeg_format = true;
        } else if lower.starts_with("topdesignname=") {
            if top_design_name {
                return false;
            }
            let Some((_, name)) = token.split_once('=') else {
                return false;
            };
            if name.len() < 3 || !name.starts_with('"') || !name.ends_with('"') {
                return false;
            }
            let design_name = &name[1..name.len() - 1];
            if design_name.is_empty() || design_name.contains('"') {
                return false;
            }
            top_design_name = true;
        } else {
            return false;
        }
    }
    resource_usage && nutmeg_format && top_design_name
}

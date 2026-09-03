//! Attribute projections for the directive family: `.IC`/`.NODESET` conflicts
//! and `.PARAM` redefinition.

use super::ParseErrorAttributes;

fn startup_directive_kind_name(kind: rspice_core::netlist::StartupDirectiveKind) -> &'static str {
    match kind {
        rspice_core::netlist::StartupDirectiveKind::Ic => "ic",
        rspice_core::netlist::StartupDirectiveKind::NodeSet => "nodeset",
    }
}

pub(super) fn startup_directive_conflict_attributes(
    error: &rspice_core::netlist::StartupDirectiveConflictError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("conflicting_startup_directives");
    attributes.category = Some("startup_directive_validation");
    attributes.set_primary(&error.first);
    attributes.set_related(&error.conflicting);
    attributes.first_startup_kind = Some(startup_directive_kind_name(error.first_kind).to_string());
    attributes.conflicting_startup_kind =
        Some(startup_directive_kind_name(error.conflicting_kind).to_string());
    attributes
}

pub(super) fn parameter_redefinition_attributes(
    error: &rspice_core::netlist::ParameterRedefinitionError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("parameter_redefinition");
    attributes.category = Some("parameter_resolution");
    attributes.set_primary(&error.duplicate_origin);
    attributes.set_related(&error.first_origin);
    attributes.authored_name = Some(error.duplicate_name.clone());
    attributes.canonical_name = Some(error.canonical_name.clone());
    attributes.detail = Some(error.kind.to_string());
    attributes
}

pub(super) fn duplicate_model_parameter_attributes(
    error: &rspice_core::netlist::DuplicateModelParameterError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("duplicate_model_parameter");
    attributes.category = Some("model_validation");
    attributes.set_primary(&error.model_origin);
    attributes.authored_name = Some(error.model_name.clone());
    attributes.canonical_name = Some(error.canonical_model_name.clone());
    attributes.parameter_name = Some(error.parameter_name.clone());
    attributes.canonical_parameter_name = Some(error.canonical_parameter_name.clone());
    attributes
}

/// Stable machine token for one analysis-card failure mode.
fn analysis_card_issue_kind(issue: &rspice_core::netlist::AnalysisCardIssue) -> &'static str {
    use rspice_core::netlist::AnalysisCardIssue as Issue;
    match issue {
        Issue::MissingField { .. } => "missing_field",
        Issue::UnknownKeyword { .. } => "unknown_keyword",
        Issue::DuplicateKeyword { .. } => "duplicate_keyword",
        Issue::MissingKeywordValue { .. } => "missing_keyword_value",
        Issue::InvalidNumber { .. } => "invalid_number",
        Issue::InvalidChoice { .. } => "invalid_choice",
        Issue::ConflictingFields { .. } => "conflicting_fields",
        Issue::InvalidName { .. } => "invalid_name",
        Issue::UnhonourableField { .. } => "unhonourable_field",
        Issue::TrailingToken { .. } => "trailing_token",
        Issue::Rejected { .. } => "rejected_configuration",
    }
}

/// Which card field or keyword the failure is about, when it names one.
fn analysis_card_field(issue: &rspice_core::netlist::AnalysisCardIssue) -> Option<String> {
    use rspice_core::netlist::AnalysisCardIssue as Issue;
    match issue {
        Issue::MissingField { field }
        | Issue::InvalidNumber { field, .. }
        | Issue::InvalidChoice { field, .. }
        | Issue::InvalidName { field, .. }
        | Issue::UnhonourableField { field, .. } => Some((*field).to_string()),
        Issue::DuplicateKeyword { keyword } | Issue::MissingKeywordValue { keyword } => {
            Some((*keyword).to_string())
        }
        Issue::UnknownKeyword { keyword } => Some(keyword.clone()),
        Issue::ConflictingFields { first, .. } => Some((*first).to_string()),
        Issue::TrailingToken { .. } | Issue::Rejected { .. } => None,
    }
}

/// Project one authored analysis-card failure onto the flat attribute set.
pub(super) fn analysis_card_attributes(
    error: &rspice_core::netlist::AnalysisCardError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("analysis_card");
    attributes.category = Some("analysis_card_validation");
    attributes.line = Some(error.line);
    attributes.primary_line = Some(error.line);
    attributes.output_directive = Some(error.card.directive().to_string());
    attributes.reason = Some(analysis_card_issue_kind(&error.issue).to_string());
    attributes.parameter_name = analysis_card_field(&error.issue);
    attributes.detail = Some(error.issue.to_string());
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::{
        DuplicateModelParameterError, NetlistSourceLocation, ParameterDefinitionKind,
        ParameterRedefinitionError, StartupDirectiveConflictError, StartupDirectiveKind,
    };

    #[test]
    fn parameter_redefinition_exposes_both_origins_and_public_tags() {
        let attributes = parameter_redefinition_attributes(&ParameterRedefinitionError {
            duplicate_name: "fOo".into(),
            canonical_name: "FOO".into(),
            kind: ParameterDefinitionKind::Parameter,
            first_origin: NetlistSourceLocation {
                path: Some("first.cir".into()),
                line: 2,
            },
            duplicate_origin: NetlistSourceLocation {
                path: Some("duplicate.cir".into()),
                line: 7,
            },
        });
        assert_eq!(attributes.kind, "parameter_redefinition");
        assert_eq!(attributes.category, Some("parameter_resolution"));
        assert_eq!(attributes.line, Some(7));
        assert_eq!(attributes.related_line, Some(2));
        assert_eq!(attributes.authored_name.as_deref(), Some("fOo"));
        assert_eq!(attributes.canonical_name.as_deref(), Some("FOO"));
        assert_eq!(attributes.detail.as_deref(), Some(".PARAM"));

        let stub = include_str!("../../../rspice.pyi");
        assert!(stub.contains("\"parameter_redefinition\""));
        assert!(stub.contains("\"parameter_resolution\""));
    }

    #[test]
    fn startup_conflict_preserves_both_typed_origins_and_modes() {
        let attributes = startup_directive_conflict_attributes(&StartupDirectiveConflictError {
            first_kind: StartupDirectiveKind::Ic,
            first: NetlistSourceLocation::in_file("deck.cir", 8),
            conflicting_kind: StartupDirectiveKind::NodeSet,
            conflicting: NetlistSourceLocation::in_file("included.cir", 12),
        });

        assert_eq!(attributes.kind, "conflicting_startup_directives");
        assert_eq!(attributes.category, Some("startup_directive_validation"));
        assert_eq!(attributes.primary_line, Some(8));
        assert_eq!(attributes.primary_source.as_deref(), Some("deck.cir"));
        assert_eq!(attributes.related_line, Some(12));
        assert_eq!(attributes.related_source.as_deref(), Some("included.cir"));
        assert_eq!(attributes.first_startup_kind.as_deref(), Some("ic"));
        assert_eq!(
            attributes.conflicting_startup_kind.as_deref(),
            Some("nodeset")
        );
    }

    #[test]
    fn duplicate_model_parameter_exposes_model_parameter_and_origin() {
        let attributes = duplicate_model_parameter_attributes(&DuplicateModelParameterError {
            model_name: "mOd".into(),
            canonical_model_name: "MOD".into(),
            parameter_name: "iS".into(),
            canonical_parameter_name: "IS".into(),
            model_origin: NetlistSourceLocation::in_file("models.lib", 14),
        });
        assert_eq!(attributes.kind, "duplicate_model_parameter");
        assert_eq!(attributes.category, Some("model_validation"));
        assert_eq!(attributes.primary_line, Some(14));
        assert_eq!(attributes.primary_source.as_deref(), Some("models.lib"));
        assert_eq!(attributes.authored_name.as_deref(), Some("mOd"));
        assert_eq!(attributes.canonical_name.as_deref(), Some("MOD"));
        assert_eq!(attributes.parameter_name.as_deref(), Some("iS"));
        assert_eq!(attributes.canonical_parameter_name.as_deref(), Some("IS"));

        let stub = include_str!("../../../rspice.pyi");
        assert!(stub.contains("\"duplicate_model_parameter\""));
        assert!(stub.contains("\"model_validation\""));
    }
}

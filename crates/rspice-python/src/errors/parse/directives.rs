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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::{
        NetlistSourceLocation, ParameterDefinitionKind, ParameterRedefinitionError,
        StartupDirectiveConflictError, StartupDirectiveKind,
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
}

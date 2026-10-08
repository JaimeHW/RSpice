//! Shared assignment of instance overrides in their declaring scope.
use super::*;

pub(super) fn constants(source: &Module) -> DigitalConstants {
    DigitalConstants::from_module(source)
}

pub(super) fn close_override(
    declaration: &ParameterDecl,
    expression: Expression,
    parent: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Expression, String> {
    let mut declaration = declaration.clone();
    declaration.default = Some(expression);
    if declaration.packed_range.is_some() || declaration.signedness.is_some() {
        // Resolve the RHS in its parent. Width/sign assignment belongs to the
        // child's complete effective scope, after all overrides are installed.
        declaration.packed_range = None;
        declaration.signedness = None;
        declaration.type_is_explicit = false;
        declaration.param_type = ParamType::Real;
    }
    crate::canonical_ir::digital_lower::parameter_override_literal(&declaration, parent, time_scale)
}

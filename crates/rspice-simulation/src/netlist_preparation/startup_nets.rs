//! The design's nets an initial condition or nodeset can name.

use rspice_design::connectivity::summary::{DesignNet, NetDomain};
use rspice_simulation_contract::design_variable::DesignVariable;
use rspice_simulation_contract::options::{
    SimulationOptions, StartupKind, net_name_problem, parse_si_value, value_problem,
};

use crate::preparation::{PreparationError, PreparationStage};

/// The net of the run's root `name` names, compared as SPICE compares node
/// names, or why a startup card cannot hold it: no such net, ground, or a
/// net that carries no voltage.
pub fn startup_net<'a>(name: &str, nets: &'a [DesignNet]) -> Result<&'a DesignNet, String> {
    let name = name.trim();
    if let Some(problem) = net_name_problem(name) {
        return Err(problem);
    }
    let net = nets
        .iter()
        .find(|net| net.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| format!("the design has no net {name}"))?;
    match net.domain {
        NetDomain::Electrical | NetDomain::Mixed(_) => Ok(net),
        domain => Err(format!(
            "{} is a {} net, which holds no voltage",
            net.name,
            domain.label()
        )),
    }
}

/// The value `text` gives a row, as the row stores it, or why it cannot:
/// volts in SPICE notation, or an expression (braces optional) of the
/// plan's `variables`, which the deck evaluates against them, so a corner or
/// a sweep that moves a variable moves the value.
pub fn startup_value(text: &str, variables: &[DesignVariable]) -> Result<String, String> {
    let text = text.trim();
    let value = text
        .strip_prefix('{')
        .and_then(|inner| inner.strip_suffix('}'))
        .unwrap_or(text)
        .trim();
    if let Some(problem) = value_problem(value) {
        return Err(problem);
    }
    if parse_si_value(value).is_ok() {
        return Ok(value.to_owned());
    }
    use rspice_core::netlist::expr::{ExprError, ParamContext, evaluate, parse_expression};
    let parsed = parse_expression(value).map_err(|error| format!("{value}: {error}"))?;
    // An expression that evaluates alone is a constant, and must be a
    // voltage; one that names a parameter must name a design variable. Each
    // stands at 1 here: only whether the plan defines it is in question, so
    // a value the placeholder makes undefined is not refused.
    match evaluate(&parsed, &ParamContext::new()) {
        Ok(volts) if !volts.is_finite() => return Err(format!("{value} is not a voltage")),
        Ok(_) => return Ok(value.to_owned()),
        Err(ExprError::UndefinedParam(_)) => {}
        Err(error) => return Err(format!("{value}: {error}")),
    }
    let mut context = ParamContext::new();
    for variable in variables {
        context.set(&variable.name, 1.0);
    }
    match evaluate(&parsed, &context) {
        Err(error @ (ExprError::UnknownFunction(_) | ExprError::WrongArgCount(_))) => {
            Err(format!("{value}: {error}"))
        }
        Err(ExprError::UndefinedParam(name)) => {
            // The parser folds names to one case; name it as it was typed.
            let typed = value
                .split(|character: char| !(character.is_alphanumeric() || character == '_'))
                .find(|word| word.eq_ignore_ascii_case(&name))
                .unwrap_or(&name);
            Err(format!("{typed} is no design variable"))
        }
        _ => Ok(value.to_owned()),
    }
}

/// Refuse a run whose initial conditions or nodesets name a net the root no
/// longer has, or a design variable the plan no longer defines: the engine
/// would ignore the card or fail on it, and the run would start somewhere
/// the plan does not say.
pub fn check_startup_nets(
    options: &SimulationOptions,
    nets: &[DesignNet],
    variables: &[DesignVariable],
) -> Result<(), PreparationError> {
    for (kind, rows) in [
        (StartupKind::InitialCondition, &options.initial_conditions),
        (StartupKind::NodeSet, &options.nodesets),
    ] {
        for row in rows {
            startup_net(&row.net, nets)
                .and_then(|_| startup_value(&row.value, variables))
                .map_err(|problem| {
                    PreparationError::new(
                        PreparationStage::DesignChecks,
                        format!("{} {}: {problem}", kind.label(), row.net),
                    )
                })?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::startup_value;
    use rspice_simulation_contract::design_variable::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableScope,
        DesignVariableSweepEligibility,
    };
    use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;

    fn vdd() -> DesignVariable {
        DesignVariable::new(
            "vdd",
            "3.3V",
            DesignVariableQuantity::Voltage,
            DesignVariableScope::Testbench,
            "supply",
            None,
            DesignVariableSweepEligibility::NestedSweepAndOptimization,
            DesignVariableOverridePolicy::InheritOwnerOnly,
        )
        .expect("a valid design variable")
    }

    /// A number is kept as written; an expression is kept without its
    /// braces when it is a constant or names only the plan's variables.
    #[test]
    fn a_value_is_volts_or_an_expression_of_the_plans_variables() {
        let variables = [vdd()];
        assert_eq!(startup_value(" 250m ", &variables).as_deref(), Ok("250m"));
        assert_eq!(startup_value("{vdd/2}", &variables).as_deref(), Ok("vdd/2"));
        assert_eq!(
            startup_value("VDD*0.9", &variables).as_deref(),
            Ok("VDD*0.9")
        );
        assert_eq!(startup_value("1/4", &[]).as_deref(), Ok("1/4"));
    }

    /// A name the plan defines no variable for is named as it was typed; a
    /// constant with no value, a function the deck has not and text that
    /// breaks the card are refused.
    #[test]
    fn a_value_the_card_cannot_carry_is_refused_with_its_reason() {
        let variables = [vdd()];
        assert_eq!(
            startup_value("vdd/vRef", &variables),
            Err("vRef is no design variable".to_owned())
        );
        assert_eq!(
            startup_value("1/0", &variables),
            Err("1/0: Division by zero".to_owned())
        );
        assert_eq!(
            startup_value("", &variables),
            Err("has no value".to_owned())
        );
        assert!(startup_value("vdd)", &variables).is_err());
        assert!(startup_value("vdd*nofunc(1)", &variables).is_err());
    }
}

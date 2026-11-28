//! The Variables page shows each value as typed and the number it resolves
//! to, from the evaluator the run uses; a value that cannot resolve says why
//! on its own row.

use super::page_contracts::render_with;
use rspice_app_simulate::workbench::surfaces::simulate::fixtures::plan_id;
use rspice_app_state::state::{
    DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
    DesignVariableSweepEligibility, SimulationPage,
};

/// The Variables page over a plan holding `rows`, each added through the
/// workspace store.
fn page_with(rows: &[(&str, &str, DesignVariableQuantity)]) -> String {
    let rows = rows
        .iter()
        .map(|(name, value, quantity)| ((*name).to_owned(), (*value).to_owned(), *quantity))
        .collect::<Vec<_>>();
    render_with(SimulationPage::Variables, 1600.0, move |app| {
        let id = plan_id(&app.state.model);
        for (name, value, quantity) in rows {
            let variable = DesignVariable::new(
                name,
                value,
                quantity,
                DesignVariableScope::Testbench,
                "",
                None,
                DesignVariableSweepEligibility::NestedSweepAndOptimization,
                DesignVariableOverridePolicy::ExplicitTestLocalOverride,
            )
            .expect("a well-formed variable");
            app.state
                .model
                .workspace
                .content
                .add_design_variable(id, variable)
                .expect("the store takes the variable");
        }
    })
}

/// `W2 = 2*W1` and `VCM = VDD/2` show their text and, beside it, the number
/// the run reads, to the reader's four significant digits.
#[test]
fn a_variable_naming_another_shows_the_number_it_resolves_to() {
    let rendered = page_with(&[
        ("W2", "2*W1", DesignVariableQuantity::Dimensionless),
        ("VCM", "VDD/2", DesignVariableQuantity::Voltage),
        ("W1", "4u", DesignVariableQuantity::Dimensionless),
        ("VDD", "1.0", DesignVariableQuantity::Voltage),
    ]);
    for shown in ["2*W1", "VDD/2", "8.000 \u{b5}", "500.0 mV"] {
        assert!(
            rendered.lines().any(|line| line == shown),
            "the page does not show {shown}:\n{rendered}"
        );
    }
}

/// A loop and a name no variable defines are errors on their rows, naming
/// the variables involved.
#[test]
fn a_loop_or_an_unknown_name_is_an_error_on_its_row() {
    let rendered = page_with(&[
        ("A", "B", DesignVariableQuantity::Dimensionless),
        ("B", "A", DesignVariableQuantity::Dimensionless),
        ("VCM", "VDD/2", DesignVariableQuantity::Voltage),
    ]);
    for error in ["A and B name each other", "VDD is not defined"] {
        assert!(
            rendered.contains(error),
            "the page does not say {error}:\n{rendered}"
        );
    }
}

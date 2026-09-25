//! Converts engine convergence diagnostics into retained result evidence.

use rspice_results::convergence_attribution::{
    ConvergenceAttribution, ConvergenceFailureClass, ConvergenceSite, ConvergenceSiteKind,
};

pub(crate) fn from_core(
    diagnostic: &rspice_core::diagnostics::ConvergenceDiagnostic,
) -> ConvergenceAttribution {
    use rspice_core::diagnostics as core;
    ConvergenceAttribution {
        class: match diagnostic.class {
            core::ConvergenceFailureClass::NoDcPathToGround => {
                ConvergenceFailureClass::NoDcPathToGround
            }
            core::ConvergenceFailureClass::ConditioningDependentBias => {
                ConvergenceFailureClass::ConditioningDependentBias
            }
            core::ConvergenceFailureClass::SingularSystem => {
                ConvergenceFailureClass::SingularSystem
            }
            // The engine's class list is non-exhaustive. A class this
            // build does not know is still a solve that gave up, so it
            // reports as one rather than dropping the named objects.
            _ => ConvergenceFailureClass::NewtonNonConvergence,
        },
        sites: diagnostic
            .sites
            .iter()
            .map(|site| ConvergenceSite {
                name: site.name.clone(),
                kind: match site.kind {
                    core::ConvergenceSiteKind::Node => ConvergenceSiteKind::Node,
                    core::ConvergenceSiteKind::Branch => ConvergenceSiteKind::Branch,
                },
                residual: site.residual,
            })
            .collect(),
        elided_sites: diagnostic.elided_sites,
        failure_message: diagnostic.failure_message.clone(),
    }
}

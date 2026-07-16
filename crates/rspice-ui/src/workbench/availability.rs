//! Fail-closed route-executor availability for canonical GUI surfaces.
//!
//! Registry presence describes the governed design contract only. It never
//! proves that a Rust executor, engine, entitlement, platform qualification,
//! or sign-off path exists. This module is the single runtime gate between a
//! canonical [`SurfaceId`] and application navigation.

use super::{CapabilityWorkflowId, SurfaceId, SurfaceRoute};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceExecutionAvailability {
    /// A complete Rust route owner is registered for this surface. This says
    /// nothing about numerical, platform, release, or sign-off qualification.
    Available {
        executor: &'static str,
        evidence_boundary: &'static str,
    },
    /// The design identity is known but no complete route owner is available.
    Unavailable { reason: &'static str },
}

impl SurfaceExecutionAvailability {
    #[must_use]
    pub const fn can_open(self) -> bool {
        matches!(self, Self::Available { .. })
    }

    #[must_use]
    pub const fn reason(self) -> Option<&'static str> {
        match self {
            Self::Available { .. } => None,
            Self::Unavailable { reason } => Some(reason),
        }
    }
}

/// Runtime route availability. Keep this match explicit: adding a catalog
/// variant can never accidentally make it executable.
#[must_use]
pub const fn route_availability(route: SurfaceRoute) -> SurfaceExecutionAvailability {
    if route.object_ref().is_some() {
        return SurfaceExecutionAvailability::Unavailable {
            reason: "Object-aware routes remain unavailable until a runtime object resolver and consuming surface executor are registered.",
        };
    }
    if let Some(workflow) = route.capability_workflow_id() {
        return capability_workflow_availability(workflow);
    }
    surface_availability(route.surface_id())
}

/// Nested workflow availability is intentionally exhaustive. A newly added
/// workflow identity must choose an executor boundary here before navigation
/// can expose it; ownership by an available parent surface is never enough.
const fn capability_workflow_availability(
    workflow: CapabilityWorkflowId,
) -> SurfaceExecutionAvailability {
    match workflow {
        CapabilityWorkflowId::GenericMultidimensionalRunsetController
        | CapabilityWorkflowId::SourceLoadPullAnalysis
        | CapabilityWorkflowId::XParameterGeneration
        | CapabilityWorkflowId::ModulatedSignalCharacterization
        | CapabilityWorkflowId::CalibrationDeembeddingPlan
        | CapabilityWorkflowId::StatisticalDistributionAuthoring
        | CapabilityWorkflowId::ErcPerc
        | CapabilityWorkflowId::AntennaChecks
        | CapabilityWorkflowId::DensityFillDfm
        | CapabilityWorkflowId::GdsOasisStreamOut
        | CapabilityWorkflowId::TapeoutAssemblyHandoff => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui governed planned-workflow specification viewer",
            evidence_boundary: "Read-only design inspection is executable; the described engineering capability and operational menu entry remain unavailable.",
        },
        CapabilityWorkflowId::InteroperabilityMatrix | CapabilityWorkflowId::PlatformLifecycle => {
            SurfaceExecutionAvailability::Available {
                executor: "rspice-ui governed capability-contract inspection document",
                evidence_boundary: "Read-only contract inspection and local filtering are executable; displayed format, connector, browser, operating-system, and engine qualification records remain governed reference data rather than runtime availability claims.",
            }
        }
        CapabilityWorkflowId::TouchEditGuide => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui governed touch-editing guidance document",
            evidence_boundary: "Read-only gesture and precision guidance is executable; the document does not mutate a design or assert device-specific platform qualification.",
        },
    }
}

#[must_use]
pub const fn surface_availability(surface: SurfaceId) -> SurfaceExecutionAvailability {
    match surface {
        SurfaceId::Project
        | SurfaceId::Design
        | SurfaceId::Simulate
        | SurfaceId::Results
        | SurfaceId::Verify
        | SurfaceId::Models
        | SurfaceId::Netlist => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui canonical primary workspace router",
            evidence_boundary: "Rust route and interaction executor present; no platform, numerical, release, or sign-off qualification is implied.",
        },
        SurfaceId::Preferences => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui canonical Preferences manager",
            evidence_boundary: "Runtime-backed appearance, workspace, schematic-grid, recovery, accessibility, shortcut, capability, notification, and licensing controls are executable; unowned mockup settings remain omitted rather than inert.",
        },
        SurfaceId::AccountOrganization => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui local account, organization, and licensing boundary manager",
            evidence_boundary: "The verified on-device license and current local application process are executable data owners; account identity, organization policy, remote sessions, server licensing, and external administration remain explicitly unconfigured and expose no speculative actions.",
        },
        SurfaceId::FeatureAvailability => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui capability and platform matrix manager",
            evidence_boundary: "Read-only disclosure executor present; bundled design fixtures are not product readiness evidence.",
        },
        _ => SurfaceExecutionAvailability::Unavailable {
            reason: "A canonical GUI design exists, but no complete Rust route executor is registered for this surface.",
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("surface `{surface}` is unavailable: {reason}")]
pub struct SurfaceRouteUnavailable {
    pub surface: SurfaceId,
    pub reason: &'static str,
}

pub fn require_available(route: SurfaceRoute) -> Result<(), SurfaceRouteUnavailable> {
    match route_availability(route) {
        SurfaceExecutionAvailability::Available { .. } => Ok(()),
        SurfaceExecutionAvailability::Unavailable { reason } => Err(SurfaceRouteUnavailable {
            surface: route.surface_id(),
            reason,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ObjectRef, ProductObjectKind};
    use uuid::Uuid;

    #[test]
    fn catalog_presence_never_defaults_to_runtime_availability() {
        let available = SurfaceId::ALL
            .into_iter()
            .filter(|surface| surface_availability(*surface).can_open())
            .collect::<Vec<_>>();
        assert_eq!(
            available,
            [
                SurfaceId::Project,
                SurfaceId::Design,
                SurfaceId::Simulate,
                SurfaceId::Results,
                SurfaceId::Verify,
                SurfaceId::Models,
                SurfaceId::Netlist,
                SurfaceId::Preferences,
                SurfaceId::AccountOrganization,
                SurfaceId::FeatureAvailability,
            ]
        );
    }

    #[test]
    fn specialist_designs_fail_closed_until_an_executor_is_registered() {
        let route = SurfaceRoute::surface(SurfaceId::RfWorkbench);
        let error = require_available(route).expect_err("RF route is not implemented");
        assert_eq!(error.surface, SurfaceId::RfWorkbench);
        assert!(error.reason.contains("no complete Rust route executor"));
    }

    #[test]
    fn every_object_aware_route_fails_closed_without_a_runtime_resolver() {
        let object_ref = ObjectRef::new(
            ProductObjectKind::Project,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").expect("fixture UUID"),
        )
        .expect("non-nil object reference");

        for surface in SurfaceId::ALL {
            let availability = route_availability(SurfaceRoute::for_object(surface, object_ref));
            assert!(!availability.can_open(), "{surface} object route opened");
            assert!(
                availability
                    .reason()
                    .is_some_and(|reason| reason.contains("runtime object resolver")),
                "{surface} did not report the resolver boundary"
            );
        }
    }

    #[test]
    fn every_capability_inspection_route_has_an_explicit_read_only_executor() {
        for workflow in CapabilityWorkflowId::ALL {
            let availability = route_availability(SurfaceRoute::capability_workflow(workflow));
            assert!(availability.can_open(), "{workflow} should be inspectable");
        }
    }
}

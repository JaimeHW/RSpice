//! Strict, canonical deep-link routes for workbench surfaces.
//!
//! The base selector spelling comes from the surface registry. An optional
//! heterogeneous object reference extends that base with the canonical query
//! suffix `&object-kind=<kind>&object-id=<uuid>`. Parsing is intentionally
//! fail-closed: aliases, reordering, partial references, duplicate/extra
//! parameters, non-canonical UUID spelling, and unknown values are rejected.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::product::{ObjectRef, ProductObjectKind};

use super::{
    capability_workflow::CapabilityWorkflowId,
    surface_catalog::{SurfaceId, SurfaceIdParseError},
};

/// Stable route to one canonical GUI surface and, when applicable, one exact
/// product object. Selection, scroll, and other presentation state do not
/// belong in this identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceRoute {
    surface_id: SurfaceId,
    object_ref: Option<ObjectRef>,
    capability_workflow_id: Option<CapabilityWorkflowId>,
}

impl SurfaceRoute {
    #[must_use]
    pub const fn new(surface_id: SurfaceId, object_ref: Option<ObjectRef>) -> Self {
        Self {
            surface_id,
            object_ref,
            capability_workflow_id: None,
        }
    }

    #[must_use]
    pub const fn surface(surface_id: SurfaceId) -> Self {
        Self::new(surface_id, None)
    }

    #[must_use]
    pub const fn for_object(surface_id: SurfaceId, object_ref: ObjectRef) -> Self {
        Self::new(surface_id, Some(object_ref))
    }

    /// Construct the canonical deep link for a nested Feature Availability
    /// workflow. Workflow routes never carry a product-object reference.
    #[must_use]
    pub const fn capability_workflow(capability_workflow_id: CapabilityWorkflowId) -> Self {
        Self {
            surface_id: capability_workflow_id.owner_surface(),
            object_ref: None,
            capability_workflow_id: Some(capability_workflow_id),
        }
    }

    /// Canonical owning surface. Route identity is immutable after
    /// construction so a nested workflow can never be detached from its
    /// registered owner.
    #[must_use]
    pub const fn surface_id(self) -> SurfaceId {
        self.surface_id
    }

    /// Exact product-object identity, when this is an object-aware route.
    /// Nested workflow routes cannot carry an object reference.
    #[must_use]
    pub const fn object_ref(self) -> Option<ObjectRef> {
        self.object_ref
    }

    /// Nested workflow identity, when this route addresses one.
    #[must_use]
    pub const fn capability_workflow_id(self) -> Option<CapabilityWorkflowId> {
        self.capability_workflow_id
    }
}

impl fmt::Display for SurfaceRoute {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(capability_workflow_id) = self.capability_workflow_id {
            debug_assert_eq!(
                self.surface_id,
                capability_workflow_id.owner_surface(),
                "capability workflow must retain its canonical owner"
            );
            debug_assert!(
                self.object_ref.is_none(),
                "capability workflow routes cannot carry object references"
            );
            return formatter.write_str(capability_workflow_id.deep_link());
        }
        formatter.write_str(self.surface_id.deep_link())?;
        if let Some(object_ref) = self.object_ref {
            write!(
                formatter,
                "&object-kind={}&object-id={}",
                object_ref.kind().stable_id(),
                object_ref.id()
            )?;
        }
        Ok(())
    }
}

impl FromStr for SurfaceRoute {
    type Err = SurfaceRouteParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(query) = value.strip_prefix('?') else {
            return Err(SurfaceRouteParseError::MissingQueryPrefix);
        };
        if query.is_empty() || value.trim() != value || value.contains('#') {
            return Err(SurfaceRouteParseError::MalformedRoute(value.to_owned()));
        }

        let parameters = query.split('&').collect::<Vec<_>>();

        let (selector, stable_id) = parse_parameter(parameters[0])?;
        if selector != "view" && selector != "surface" {
            return Err(SurfaceRouteParseError::UnknownSelector(selector.to_owned()));
        }
        let surface_result = stable_id.parse::<SurfaceId>();
        if let Ok(capability_workflow_id) = stable_id.parse::<CapabilityWorkflowId>() {
            if value.split_once('&').map_or(value, |(base, _)| base)
                != capability_workflow_id.deep_link()
            {
                return Err(SurfaceRouteParseError::NonCanonicalSelector {
                    received: format!("?{selector}={stable_id}"),
                    expected: capability_workflow_id.deep_link(),
                });
            }
            if parameters.len() != 1 {
                return Err(
                    SurfaceRouteParseError::CapabilityWorkflowParametersUnsupported {
                        workflow: capability_workflow_id,
                        received: parameters.len() - 1,
                    },
                );
            }
            return Ok(Self::capability_workflow(capability_workflow_id));
        }

        let surface_id = surface_result.map_err(SurfaceRouteParseError::UnknownSurface)?;
        if value.split_once('&').map_or(value, |(base, _)| base) != surface_id.deep_link() {
            return Err(SurfaceRouteParseError::NonCanonicalSelector {
                received: format!("?{selector}={stable_id}"),
                expected: surface_id.deep_link(),
            });
        }

        if parameters.len() != 1 && parameters.len() != 3 {
            return Err(SurfaceRouteParseError::UnexpectedParameterCount(
                parameters.len(),
            ));
        }

        if parameters.len() == 1 {
            return Ok(Self::surface(surface_id));
        }

        let (kind_key, kind_value) = parse_parameter(parameters[1])?;
        if kind_key != "object-kind" {
            return Err(SurfaceRouteParseError::UnexpectedParameter {
                received: kind_key.to_owned(),
                expected: "object-kind",
            });
        }
        let (id_key, id_value) = parse_parameter(parameters[2])?;
        if id_key != "object-id" {
            return Err(SurfaceRouteParseError::UnexpectedParameter {
                received: id_key.to_owned(),
                expected: "object-id",
            });
        }

        let kind = ProductObjectKind::ALL
            .into_iter()
            .find(|kind| kind.stable_id() == kind_value)
            .ok_or_else(|| SurfaceRouteParseError::UnknownObjectKind(kind_value.to_owned()))?;
        let id = Uuid::parse_str(id_value)
            .map_err(|_| SurfaceRouteParseError::InvalidObjectId(id_value.to_owned()))?;
        if id.is_nil() {
            return Err(SurfaceRouteParseError::NilObjectId);
        }
        if id.to_string() != id_value {
            return Err(SurfaceRouteParseError::NonCanonicalObjectId {
                received: id_value.to_owned(),
                expected: id.to_string(),
            });
        }
        let object_ref =
            ObjectRef::new(kind, id).map_err(|_| SurfaceRouteParseError::NilObjectId)?;

        Ok(Self::for_object(surface_id, object_ref))
    }
}

fn parse_parameter(parameter: &str) -> Result<(&str, &str), SurfaceRouteParseError> {
    let Some((key, value)) = parameter.split_once('=') else {
        return Err(SurfaceRouteParseError::MalformedParameter(
            parameter.to_owned(),
        ));
    };
    if key.is_empty() || value.is_empty() || value.contains('=') {
        return Err(SurfaceRouteParseError::MalformedParameter(
            parameter.to_owned(),
        ));
    }
    Ok((key, value))
}

impl Serialize for SurfaceRoute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SurfaceRoute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SurfaceRouteParseError {
    #[error("surface route must start with `?`")]
    MissingQueryPrefix,
    #[error("malformed surface route `{0}`")]
    MalformedRoute(String),
    #[error("surface route must have exactly one or three parameters, received {0}")]
    UnexpectedParameterCount(usize),
    #[error("malformed surface-route parameter `{0}`")]
    MalformedParameter(String),
    #[error("unknown surface selector `{0}`; expected `view` or `surface`")]
    UnknownSelector(String),
    #[error(transparent)]
    UnknownSurface(#[from] SurfaceIdParseError),
    #[error("non-canonical surface selector `{received}`; expected `{expected}`")]
    NonCanonicalSelector {
        received: String,
        expected: &'static str,
    },
    #[error(
        "Feature Availability workflow `{workflow}` does not accept route parameters; received {received}"
    )]
    CapabilityWorkflowParametersUnsupported {
        workflow: CapabilityWorkflowId,
        received: usize,
    },
    #[error("unexpected route parameter `{received}`; expected `{expected}`")]
    UnexpectedParameter {
        received: String,
        expected: &'static str,
    },
    #[error("unknown product object kind `{0}`")]
    UnknownObjectKind(String),
    #[error("invalid product object UUID `{0}`")]
    InvalidObjectId(String),
    #[error("product object UUID must not be nil")]
    NilObjectId,
    #[error("non-canonical product object UUID `{received}`; expected `{expected}`")]
    NonCanonicalObjectId { received: String, expected: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project_ref() -> ObjectRef {
        ObjectRef::new(
            ProductObjectKind::Project,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").expect("fixture UUID"),
        )
        .expect("non-nil object reference")
    }

    #[test]
    fn every_base_route_uses_the_registry_spelling_and_roundtrips() {
        for surface_id in SurfaceId::ALL {
            let route = SurfaceRoute::surface(surface_id);
            assert_eq!(route.to_string(), surface_id.deep_link());
            assert_eq!(route.to_string().parse::<SurfaceRoute>(), Ok(route));

            let encoded = serde_json::to_string(&route).expect("route serializes");
            assert_eq!(
                serde_json::from_str::<SurfaceRoute>(&encoded).expect("route deserializes"),
                route
            );
        }
    }

    #[test]
    fn every_object_route_has_a_canonical_query_and_roundtrips() {
        let object_ref = project_ref();
        for surface_id in SurfaceId::ALL {
            let route = SurfaceRoute::for_object(surface_id, object_ref);
            let expected = format!(
                "{}&object-kind=project&object-id=123e4567-e89b-12d3-a456-426614174000",
                surface_id.deep_link()
            );
            assert_eq!(route.to_string(), expected);
            assert_eq!(expected.parse::<SurfaceRoute>(), Ok(route));

            let encoded = serde_json::to_string(&route).expect("route serializes");
            assert_eq!(
                serde_json::from_str::<SurfaceRoute>(&encoded).expect("route deserializes"),
                route
            );
        }
    }

    #[test]
    fn every_capability_workflow_route_is_canonical_owned_and_roundtrips() {
        for workflow in CapabilityWorkflowId::ALL {
            let route = SurfaceRoute::capability_workflow(workflow);
            assert_eq!(route.surface_id(), SurfaceId::FeatureAvailability);
            assert_eq!(route.object_ref(), None);
            assert_eq!(route.capability_workflow_id(), Some(workflow));
            assert_eq!(route.to_string(), workflow.deep_link());
            assert_eq!(route.to_string().parse::<SurfaceRoute>(), Ok(route));

            let encoded = serde_json::to_string(&route).expect("workflow route serializes");
            assert_eq!(encoded, format!("\"{}\"", workflow.deep_link()));
            assert_eq!(
                serde_json::from_str::<SurfaceRoute>(&encoded)
                    .expect("workflow route deserializes"),
                route
            );
        }
    }

    #[test]
    fn capability_workflow_routes_reject_aliases_and_all_extra_parameters() {
        for workflow in CapabilityWorkflowId::ALL {
            let object_route = format!(
                "?surface={workflow}&object-kind=project&object-id=123e4567-e89b-12d3-a456-426614174000"
            );
            assert_eq!(
                object_route.parse::<SurfaceRoute>(),
                Err(
                    SurfaceRouteParseError::CapabilityWorkflowParametersUnsupported {
                        workflow,
                        received: 2,
                    }
                )
            );

            for value in [
                format!("?view={workflow}"),
                object_route,
                format!("?surface={workflow}&extra=value"),
            ] {
                assert!(
                    value.parse::<SurfaceRoute>().is_err(),
                    "accepted invalid workflow route `{value}`"
                );
                assert!(
                    serde_json::from_str::<SurfaceRoute>(&format!("\"{value}\"")).is_err(),
                    "deserialized invalid workflow route `{value}`"
                );
            }
        }
    }

    #[test]
    fn unknown_alias_partial_and_malformed_routes_fail_closed() {
        let malformed = [
            "",
            "view=project",
            "?",
            "?view=",
            "?view=Project",
            "?view=unknown",
            "?surface=project",
            "?view=release-cockpit",
            "?foo=project",
            "?view=project#fragment",
            " ?view=project",
            "?view=project ",
            "?view=project&object-kind=project",
            "?view=project&object-id=123e4567-e89b-12d3-a456-426614174000",
            "?view=project&object-id=123e4567-e89b-12d3-a456-426614174000&object-kind=project",
            "?view=project&object-kind=project&object-kind=project",
            "?view=project&object-kind=&object-id=123e4567-e89b-12d3-a456-426614174000",
            "?view=project&object-kind=unknown&object-id=123e4567-e89b-12d3-a456-426614174000",
            "?view=project&object-kind=project&object-id=not-a-uuid",
            "?view=project&object-kind=project&object-id=00000000-0000-0000-0000-000000000000",
            "?view=project&object-kind=project&object-id=123E4567-E89B-12D3-A456-426614174000",
            "?view=project&object-kind=project&object-id=123e4567e89b12d3a456426614174000",
            "?view=project&object-kind=project&object-id=123e4567-e89b-12d3-a456-426614174000&extra=value",
        ];

        for value in malformed {
            assert!(
                value.parse::<SurfaceRoute>().is_err(),
                "accepted malformed route `{value}`"
            );
            assert!(
                serde_json::from_str::<SurfaceRoute>(&format!("\"{value}\"")).is_err(),
                "deserialized malformed route `{value}`"
            );
        }
    }
}

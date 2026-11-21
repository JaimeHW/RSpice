use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! typed_id {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Uuid::parse_str(value).map(Self)
            }
        }

        impl From<Uuid> for $name {
            fn from(value: Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$name> for Uuid {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

typed_id!(CircuitId);
typed_id!(PrincipalId);
typed_id!(RevisionId);
typed_id!(ShareId);
typed_id!(SimulationRunId);
typed_id!(WorkspaceId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_ids_round_trip_through_text_and_json() {
        let id = CircuitId::from_uuid(Uuid::from_u128(1));
        let text = id.to_string();

        assert_eq!(text.parse::<CircuitId>().expect("valid circuit ID"), id);
        assert_eq!(
            serde_json::from_str::<CircuitId>(
                &serde_json::to_string(&id).expect("serialize circuit ID")
            )
            .expect("deserialize circuit ID"),
            id
        );
        assert_eq!(id.as_uuid(), Uuid::from_u128(1));
    }

    #[test]
    fn distinct_id_types_remain_explicit() {
        let raw = Uuid::from_u128(2);
        let circuit = CircuitId::from_uuid(raw);
        let workspace = WorkspaceId::from_uuid(raw);

        assert_eq!(circuit.as_uuid(), workspace.as_uuid());
        assert_eq!(circuit.to_string(), workspace.to_string());
    }
}

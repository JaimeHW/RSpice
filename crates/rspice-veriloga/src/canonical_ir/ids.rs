use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $name(u32);

        impl $name {
            pub const fn new(index: u32) -> Self {
                Self(index)
            }

            pub const fn index(self) -> u32 {
                self.0
            }

            pub const fn next(self) -> Self {
                match self.0.checked_add(1) {
                    Some(index) => Self(index),
                    None => panic!("canonical IR id overflow"),
                }
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                let index = u32::try_from(value).expect("canonical IR id index exceeds u32::MAX");
                Self(index)
            }
        }

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0 as usize
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

id_type!(ModuleId);
id_type!(SourceId);
id_type!(SymbolId);
id_type!(PortId);
id_type!(DisciplineId);
id_type!(ParamId);
id_type!(VariableId);
id_type!(ArrayId);
id_type!(NodeId);
id_type!(BranchId);
id_type!(BranchUnknownId);
id_type!(StateId);
id_type!(EquationId);
id_type!(ContributionId);
id_type!(NoiseSourceId);
id_type!(RegionId);
id_type!(ExprId);
id_type!(ValueId);
id_type!(ScheduleId);

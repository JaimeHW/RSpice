//! Parameter resolution across the design hierarchy.
//!
//! [`ParamResolver`] answers a lookup at a hierarchy path from the scopes the
//! flattener captured: the nearest captured scope at or above the path wins,
//! and a path with no captured scope falls back to the global `.PARAM` values.

use super::hierarchy_path::HierarchyPath;
use crate::Value;
// Note: serde support not needed here - serialization handled at UI level
use std::collections::HashMap;

//=============================================================================
// Parameter Resolver
//=============================================================================

/// Resolves parameters across the design hierarchy with proper precedence
///
/// This implements the core of parameter resolution,
/// ensuring that more specific definitions override less specific ones.
#[derive(Debug, Clone, Default)]
pub struct ParamResolver {
    /// Global parameters (top-level .PARAM)
    global_params: HashMap<String, Value>,

    /// Complete real-valued snapshots captured by hierarchy expansion. An absent
    /// name in a snapshot must not fall back to an outer, shadowed binding.
    resolved_scopes: HashMap<Vec<String>, HashMap<String, Value>>,
}

impl ParamResolver {
    /// Create a new empty parameter resolver
    pub fn new() -> Self {
        Self::default()
    }

    //-------------------------------------------------------------------------
    // Parameter Definition
    //-------------------------------------------------------------------------

    /// Set a global parameter
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.global_params.insert(name.to_uppercase(), value);
    }

    /// Replace the values recorded for one completed parameter scope without
    /// evaluating its expressions again. Segment keys preserve hierarchy even
    /// when a literal instance name contains the display separator.
    pub(crate) fn set_resolved_scope(
        &mut self,
        path: &HierarchyPath,
        params: Vec<(String, Value)>,
    ) {
        self.resolved_scopes.insert(
            path.segments().iter().map(|s| s.to_uppercase()).collect(),
            params
                .into_iter()
                .map(|(n, v)| (n.to_uppercase(), v))
                .collect(),
        );
    }

    /// Clear data belonging to the previous expansion.
    pub(crate) fn clear_expansion(&mut self) {
        self.global_params.clear();
        self.resolved_scopes.clear();
    }

    //-------------------------------------------------------------------------
    // Parameter Resolution
    //-------------------------------------------------------------------------

    /// Resolve a parameter at the given hierarchical path
    ///
    /// Returns the value from the highest-precedence scope that defines
    /// this parameter. Returns None if parameter is not defined.
    ///
    /// The captured effective scope at the path or its nearest ancestor wins;
    /// a path with no captured scope falls back to the global parameters.
    ///
    /// Captured scopes include resolved subcircuit defaults and local parameters.
    /// They contain only materialized real values; symbolic, string and complex
    /// bindings return None and still shadow outer parameters.
    pub fn resolve(&self, name: &str, path: &HierarchyPath) -> Option<Value> {
        let name_upper = name.to_uppercase();

        let mut current = Some(path.clone());
        while let Some(p) = current {
            let key: Vec<_> = p.segments().iter().map(|s| s.to_uppercase()).collect();
            if let Some(params) = self.resolved_scopes.get(&key) {
                return params.get(&name_upper).copied();
            }
            current = p.parent();
        }

        self.global_params.get(&name_upper).copied()
    }

    /// Check if a parameter is defined at any scope
    pub fn is_defined(&self, name: &str, path: &HierarchyPath) -> bool {
        self.resolve(name, path).is_some()
    }

    //-------------------------------------------------------------------------
    // Utilities
    //-------------------------------------------------------------------------

    /// Clear all parameters
    pub fn clear(&mut self) {
        self.clear_expansion();
    }
}

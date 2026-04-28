//! Parameter Scope and Precedence Resolution
//!
//! Implements parameter scoping with hierarchical
//! precedence resolution. Parameters defined at different levels of the
//! design hierarchy have different priorities.
//!
//! # Scope Levels (Lowest to Highest Precedence)
//!
//! 1. **Global** - `.PARAM` at top level
//! 2. **Library** - Parameters from `.LIB` sections
//! 3. **Subcircuit** - Default parameters in `.SUBCKT ... PARAMS:`
//! 4. **Instance** - Parameters on subcircuit instances `X1 ... PARAM=val`
//! 5. **Local** - Path-specific occurrence overrides
//!
//! # Usage
//!
//! ```ignore
//! let mut resolver = ParamResolver::new();
//!
//! // Define parameters at different scopes
//! resolver.set_global("R", 1000.0);
//! resolver.set_subcircuit("RESISTOR_DIV", "R", 2000.0);
//! resolver.set_instance(&path, "R", 3000.0);
//!
//! // Resolve - higher precedence wins
//! let value = resolver.resolve("R", &path);
//! ```

use super::hierarchy_path::HierarchyPath;
use crate::Value;
// Note: serde support not needed here - serialization handled at UI level
use std::collections::HashMap;

//=============================================================================
// Scope Levels
//=============================================================================

/// Parameter scope levels with precedence ordering
///
/// Higher values override lower values when resolving parameters.
/// This follows standard conventions for parameter binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParamScope {
    /// Global parameters from top-level .PARAM statements
    /// Lowest precedence - can be overridden by anything
    Global = 0,

    /// Parameters from library/technology files (.LIB sections)
    Library = 1,

    /// Default parameters in subcircuit definitions
    /// `.SUBCKT name ports PARAMS: R=1k`
    Subcircuit = 2,

    /// Instance-level parameters on subcircuit calls
    /// `X1 n1 n2 SUBCKT R=2k`
    Instance = 3,

    /// Local path-specific overrides (occurrence binding)
    /// Most specific - allows targeting exact instances
    Local = 4,
}

impl ParamScope {
    /// Get display name for the scope level
    pub fn display_name(&self) -> &'static str {
        match self {
            ParamScope::Global => "Global",
            ParamScope::Library => "Library",
            ParamScope::Subcircuit => "Subcircuit",
            ParamScope::Instance => "Instance",
            ParamScope::Local => "Local",
        }
    }

    /// Get precedence as numeric value (higher = more specific)
    pub fn precedence(&self) -> u8 {
        *self as u8
    }
}

//=============================================================================
// Scoped Parameter
//=============================================================================

/// A parameter value with scope and path information
#[derive(Debug, Clone)]
pub struct ScopedParam {
    /// Parameter name  
    pub name: String,
    /// Resolved numeric value
    pub value: Value,
    /// Scope level for precedence resolution
    pub scope: ParamScope,
    /// Hierarchical path where this parameter applies
    /// For Global/Library scope, this is typically root
    pub path: HierarchyPath,
    /// Optional source identifier (subcircuit name, library name)
    pub source: Option<String>,
}

impl ScopedParam {
    /// Create a new scoped parameter
    pub fn new(name: impl Into<String>, value: Value, scope: ParamScope) -> Self {
        Self {
            name: name.into(),
            value,
            scope,
            path: HierarchyPath::root(),
            source: None,
        }
    }

    /// Set the hierarchical path
    pub fn with_path(mut self, path: HierarchyPath) -> Self {
        self.path = path;
        self
    }

    /// Set the source identifier
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Check if this parameter applies to the given path
    ///
    /// A parameter applies if its path is an ancestor of (or equal to)
    /// the query path.
    pub fn applies_to(&self, query_path: &HierarchyPath) -> bool {
        match self.scope {
            ParamScope::Global | ParamScope::Library => true,
            ParamScope::Subcircuit => {
                // Subcircuit defaults apply to any instance of that subcircuit
                true
            }
            ParamScope::Instance | ParamScope::Local => {
                // Must be exact path match or ancestor
                query_path.starts_with(&self.path)
            }
        }
    }
}

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

    /// Library-level parameters organized by library name
    library_params: HashMap<String, HashMap<String, Value>>,

    /// Subcircuit default parameters (name -> params)
    subcircuit_defaults: HashMap<String, HashMap<String, Value>>,

    /// Instance-level overrides indexed by full path string
    instance_overrides: HashMap<String, HashMap<String, Value>>,

    /// Local/occurrence-level overrides (most specific)
    local_overrides: HashMap<String, HashMap<String, Value>>,

    /// Active library context for library parameter lookup
    active_library: Option<String>,
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

    /// Set multiple global parameters
    pub fn set_globals(&mut self, params: &[(String, Value)]) {
        for (name, value) in params {
            self.set_global(name, *value);
        }
    }

    /// Set a library parameter
    pub fn set_library(&mut self, library: &str, name: &str, value: Value) {
        self.library_params
            .entry(library.to_string())
            .or_default()
            .insert(name.to_uppercase(), value);
    }

    /// Set the active library context
    pub fn set_active_library(&mut self, library: Option<String>) {
        self.active_library = library;
    }

    /// Set a subcircuit default parameter
    pub fn set_subcircuit_default(&mut self, subckt_name: &str, name: &str, value: Value) {
        self.subcircuit_defaults
            .entry(subckt_name.to_uppercase())
            .or_default()
            .insert(name.to_uppercase(), value);
    }

    /// Set subcircuit defaults from a parsed SubcircuitDef
    pub fn add_subcircuit_defaults(&mut self, subckt_name: &str, params: &[(String, Value)]) {
        let entry = self
            .subcircuit_defaults
            .entry(subckt_name.to_uppercase())
            .or_default();
        for (name, value) in params {
            entry.insert(name.to_uppercase(), *value);
        }
    }

    /// Set an instance-level parameter override
    pub fn set_instance(&mut self, path: &HierarchyPath, name: &str, value: Value) {
        let path_key = path.to_string().to_uppercase();
        self.instance_overrides
            .entry(path_key)
            .or_default()
            .insert(name.to_uppercase(), value);
    }

    /// Set instance overrides for a path
    pub fn add_instance_overrides(&mut self, path: &HierarchyPath, params: &[(String, Value)]) {
        let path_key = path.to_string().to_uppercase();
        let entry = self.instance_overrides.entry(path_key).or_default();
        for (name, value) in params {
            entry.insert(name.to_uppercase(), *value);
        }
    }

    /// Set a local/occurrence-level override
    pub fn set_local(&mut self, path: &HierarchyPath, name: &str, value: Value) {
        let path_key = path.to_string().to_uppercase();
        self.local_overrides
            .entry(path_key)
            .or_default()
            .insert(name.to_uppercase(), value);
    }

    //-------------------------------------------------------------------------
    // Parameter Resolution
    //-------------------------------------------------------------------------

    /// Resolve a parameter at the given hierarchical path
    ///
    /// Returns the value from the highest-precedence scope that defines
    /// this parameter. Returns None if parameter is not defined.
    ///
    /// Resolution order (highest to lowest precedence):
    /// 1. Local override at exact path
    /// 2. Instance override at path or ancestor
    /// 3. Subcircuit default (requires subcircuit name)
    /// 4. Library parameter (if active library set)
    /// 5. Global parameter
    pub fn resolve(&self, name: &str, path: &HierarchyPath) -> Option<Value> {
        let name_upper = name.to_uppercase();
        let path_str = path.to_string().to_uppercase();

        // 1. Check local override (exact path)
        if let Some(params) = self.local_overrides.get(&path_str)
            && let Some(&value) = params.get(&name_upper)
        {
            return Some(value);
        }

        // 2. Check instance overrides (path and ancestors)
        let mut current = Some(path.clone());
        while let Some(p) = current {
            let p_str = p.to_string().to_uppercase();
            if let Some(params) = self.instance_overrides.get(&p_str)
                && let Some(&value) = params.get(&name_upper)
            {
                return Some(value);
            }
            current = p.parent();
        }

        // 3. Library parameters (if active library set)
        if let Some(lib) = &self.active_library
            && let Some(params) = self.library_params.get(lib)
            && let Some(&value) = params.get(&name_upper)
        {
            return Some(value);
        }

        // 4. Global parameters
        self.global_params.get(&name_upper).copied()
    }

    /// Resolve with explicit subcircuit context
    ///
    /// Used during expansion when the subcircuit name is known.
    pub fn resolve_with_subcircuit(
        &self,
        name: &str,
        path: &HierarchyPath,
        subcircuit_name: &str,
    ) -> Option<Value> {
        // First try normal resolution
        if let Some(value) = self.resolve(name, path) {
            return Some(value);
        }

        // Then check subcircuit defaults
        let name_upper = name.to_uppercase();
        let subckt_upper = subcircuit_name.to_uppercase();

        self.subcircuit_defaults
            .get(&subckt_upper)
            .and_then(|params| params.get(&name_upper))
            .copied()
    }

    /// Implement pPar() style parent parameter lookup
    ///
    /// Looks up a parameter from the parent scope in the hierarchy.
    /// Returns the value defined at the parent instance level.
    pub fn parent_param(&self, name: &str, path: &HierarchyPath) -> Option<Value> {
        if let Some(parent) = path.parent() {
            self.resolve(name, &parent)
        } else {
            // At root, fall back to global
            self.global_params.get(&name.to_uppercase()).copied()
        }
    }

    /// Get all parameters in scope at a given path
    ///
    /// Returns a merged map with higher precedence values overriding lower.
    pub fn all_params_at(&self, path: &HierarchyPath) -> HashMap<String, Value> {
        let mut result = HashMap::new();

        // Start with globals (lowest precedence)
        for (name, &value) in &self.global_params {
            result.insert(name.clone(), value);
        }

        // Add library params
        if let Some(lib) = &self.active_library
            && let Some(params) = self.library_params.get(lib)
        {
            for (name, &value) in params {
                result.insert(name.clone(), value);
            }
        }

        // Add instance overrides from ancestors (closer ancestors override)
        let mut ancestors: Vec<_> = path.ancestors();
        ancestors.push(path.clone());

        for ancestor in ancestors {
            let p_str = ancestor.to_string().to_uppercase();
            if let Some(params) = self.instance_overrides.get(&p_str) {
                for (name, &value) in params {
                    result.insert(name.clone(), value);
                }
            }
        }

        // Add local overrides (highest precedence)
        let path_str = path.to_string().to_uppercase();
        if let Some(params) = self.local_overrides.get(&path_str) {
            for (name, &value) in params {
                result.insert(name.clone(), value);
            }
        }

        result
    }

    /// Check if a parameter is defined at any scope
    pub fn is_defined(&self, name: &str, path: &HierarchyPath) -> bool {
        self.resolve(name, path).is_some()
    }

    /// Get the scope level at which a parameter is defined
    pub fn get_defining_scope(&self, name: &str, path: &HierarchyPath) -> Option<ParamScope> {
        let name_upper = name.to_uppercase();
        let path_str = path.to_string().to_uppercase();

        // Check in precedence order
        if self
            .local_overrides
            .get(&path_str)
            .is_some_and(|p| p.contains_key(&name_upper))
        {
            return Some(ParamScope::Local);
        }

        let mut current = Some(path.clone());
        while let Some(p) = current {
            let p_str = p.to_string().to_uppercase();
            if self
                .instance_overrides
                .get(&p_str)
                .is_some_and(|p| p.contains_key(&name_upper))
            {
                return Some(ParamScope::Instance);
            }
            current = p.parent();
        }

        if let Some(lib) = &self.active_library
            && self
                .library_params
                .get(lib)
                .is_some_and(|p| p.contains_key(&name_upper))
        {
            return Some(ParamScope::Library);
        }

        if self.global_params.contains_key(&name_upper) {
            return Some(ParamScope::Global);
        }

        None
    }

    //-------------------------------------------------------------------------
    // Utilities
    //-------------------------------------------------------------------------

    /// Clear all parameters
    pub fn clear(&mut self) {
        self.global_params.clear();
        self.library_params.clear();
        self.subcircuit_defaults.clear();
        self.instance_overrides.clear();
        self.local_overrides.clear();
        self.active_library = None;
    }

    /// Get count of defined parameters at all scopes
    pub fn param_count(&self) -> usize {
        let global = self.global_params.len();
        let library: usize = self.library_params.values().map(|m| m.len()).sum();
        let subckt: usize = self.subcircuit_defaults.values().map(|m| m.len()).sum();
        let instance: usize = self.instance_overrides.values().map(|m| m.len()).sum();
        let local: usize = self.local_overrides.values().map(|m| m.len()).sum();
        global + library + subckt + instance + local
    }
}

//=============================================================================
// Local Options
//=============================================================================

/// Local simulation options that can be scoped to instances or subcircuits
///
/// Options like temp, scale, reltol can be applied locally.
/// When conflicts exist, lower hierarchy (more specific) wins.
#[derive(Debug, Clone, Default)]
pub struct LocalOptions {
    /// Temperature override (Celsius)
    pub temp: Option<Value>,
    /// Nominal temperature (Celsius)
    pub tnom: Option<Value>,
    /// Scale factor
    pub scale: Option<Value>,
    /// Relative tolerance override
    pub reltol: Option<Value>,
    /// Absolute tolerance override
    pub abstol: Option<Value>,
    /// Skip evaluation flag
    pub skip: Option<bool>,
    /// Custom options as key-value pairs
    pub custom: HashMap<String, Value>,
}

impl LocalOptions {
    /// Create empty options
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge with another options set (other takes precedence)
    pub fn merge(&mut self, other: &LocalOptions) {
        if other.temp.is_some() {
            self.temp = other.temp;
        }
        if other.tnom.is_some() {
            self.tnom = other.tnom;
        }
        if other.scale.is_some() {
            self.scale = other.scale;
        }
        if other.reltol.is_some() {
            self.reltol = other.reltol;
        }
        if other.abstol.is_some() {
            self.abstol = other.abstol;
        }
        if other.skip.is_some() {
            self.skip = other.skip;
        }
        for (key, &value) in &other.custom {
            self.custom.insert(key.clone(), value);
        }
    }

    /// Check if any options are set
    pub fn is_empty(&self) -> bool {
        self.temp.is_none()
            && self.tnom.is_none()
            && self.scale.is_none()
            && self.reltol.is_none()
            && self.abstol.is_none()
            && self.skip.is_none()
            && self.custom.is_empty()
    }
}

//=============================================================================
// Tests
//=============================================================================


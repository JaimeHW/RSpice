//! Component Manager
//!
//! Centralized component lifecycle management.
//! Coordinates creation, updates, and destruction of UI components.
//!
//! # Features
//!
//! - Component registration and tracking
//! - State management per component
//! - Lifecycle hooks (init, update, destroy)
//! - Dependency injection

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

use super::lock::{read_lock, write_lock};

// =============================================================================
// Component Identification
// =============================================================================

/// Unique component identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub u64);

impl ComponentId {
    /// Generate a new ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get inner value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component({})", self.0)
    }
}

/// Component type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    /// Schematic editor
    SchematicEditor,
    /// Waveform viewer
    WaveformViewer,
    /// Analysis panel
    AnalysisPanel,
    /// Properties panel
    PropertiesPanel,
    /// Project browser
    ProjectBrowser,
    /// Console/log view
    Console,
    /// Simulation controller
    SimController,
    /// Dialog window
    Dialog,
    /// Toolbar
    Toolbar,
    /// Custom component
    Custom,
}

// =============================================================================
// Component State
// =============================================================================

/// Lifecycle state of a component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ComponentState {
    /// Not yet initialized
    #[default]
    Created,
    /// Initializing
    Initializing,
    /// Ready for use
    Ready,
    /// Currently active/focused
    Active,
    /// Suspended/hidden
    Suspended,
    /// Being destroyed
    Destroying,
    /// Destroyed
    Destroyed,
    /// Error state
    Error,
}

impl ComponentState {
    /// Is component usable?
    pub fn is_available(&self) -> bool {
        matches!(self, ComponentState::Ready | ComponentState::Active)
    }

    /// Is component visible?
    pub fn is_visible(&self) -> bool {
        matches!(
            self,
            ComponentState::Ready | ComponentState::Active | ComponentState::Suspended
        )
    }
}

// =============================================================================
// Component Info
// =============================================================================

/// Metadata about a registered component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    /// Unique ID
    pub id: ComponentId,
    /// Human-readable name
    pub name: String,
    /// Component type
    pub comp_type: ComponentType,
    /// Current state
    pub state: ComponentState,
    /// Creation timestamp
    pub created_at: u64,
    /// Last update timestamp
    pub updated_at: u64,
    /// Whether component is singleton
    pub singleton: bool,
    /// Dependencies (component types required)
    pub dependencies: Vec<ComponentType>,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl Default for ComponentInfo {
    fn default() -> Self {
        Self {
            id: ComponentId(0),
            name: String::new(),
            comp_type: ComponentType::Custom,
            state: ComponentState::Created,
            created_at: 0,
            updated_at: 0,
            singleton: false,
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl ComponentInfo {
    /// Create new component info
    pub fn new(id: ComponentId, name: impl Into<String>, comp_type: ComponentType) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            id,
            name: name.into(),
            comp_type,
            created_at: now,
            updated_at: now,
            ..Default::default()
        }
    }

    /// Mark as singleton
    pub fn as_singleton(mut self) -> Self {
        self.singleton = true;
        self
    }

    /// Add dependency
    pub fn with_dependency(mut self, dep: ComponentType) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Update state
    pub fn set_state(&mut self, state: ComponentState) {
        self.state = state;
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }
}

// =============================================================================
// Component Manager
// =============================================================================

/// Central component lifecycle manager
pub struct ComponentManager {
    /// Registered components
    components: RwLock<HashMap<ComponentId, ComponentInfo>>,
    /// Next component ID
    next_id: RwLock<u64>,
    /// Type to ID mapping for singletons
    singletons: RwLock<HashMap<ComponentType, ComponentId>>,
}

impl Default for ComponentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentManager {
    /// Create new manager
    pub fn new() -> Self {
        Self {
            components: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
            singletons: RwLock::new(HashMap::new()),
        }
    }

    /// Generate next ID
    fn next_id(&self) -> ComponentId {
        let mut id = write_lock(&self.next_id, "ComponentManager::next_id");
        let current = *id;
        *id += 1;
        ComponentId(current)
    }

    /// Register a new component
    pub fn register(
        &self,
        name: impl Into<String>,
        comp_type: ComponentType,
    ) -> Option<ComponentId> {
        let id = self.next_id();
        let info = ComponentInfo::new(id, name, comp_type);

        let mut components = write_lock(&self.components, "ComponentManager::register");
        components.insert(id, info);

        Some(id)
    }

    /// Register a singleton component
    pub fn register_singleton(
        &self,
        name: impl Into<String>,
        comp_type: ComponentType,
    ) -> Option<ComponentId> {
        // Check if singleton already exists
        {
            let singletons = read_lock(
                &self.singletons,
                "ComponentManager::register_singleton(singletons-read)",
            );
            if let Some(&existing_id) = singletons.get(&comp_type) {
                return Some(existing_id);
            }
        }

        let id = self.next_id();
        let info = ComponentInfo::new(id, name, comp_type).as_singleton();

        let mut components = write_lock(
            &self.components,
            "ComponentManager::register_singleton(components-write)",
        );
        components.insert(id, info);

        let mut singletons = write_lock(
            &self.singletons,
            "ComponentManager::register_singleton(singletons-write)",
        );
        singletons.insert(comp_type, id);

        Some(id)
    }

    /// Get component by ID
    pub fn get(&self, id: ComponentId) -> Option<ComponentInfo> {
        let components = read_lock(&self.components, "ComponentManager::get");
        components.get(&id).cloned()
    }

    /// Get singleton by type
    pub fn get_singleton(&self, comp_type: ComponentType) -> Option<ComponentInfo> {
        let singletons = read_lock(&self.singletons, "ComponentManager::get_singleton");
        if let Some(&id) = singletons.get(&comp_type) {
            return self.get(id);
        }
        None
    }

    /// Update component state
    pub fn set_state(&self, id: ComponentId, state: ComponentState) {
        let mut components = write_lock(&self.components, "ComponentManager::set_state");
        if let Some(info) = components.get_mut(&id) {
            info.set_state(state);
        }
    }

    /// Initialize component
    pub fn init(&self, id: ComponentId) {
        self.set_state(id, ComponentState::Initializing);
        // Component-specific init would happen here
        self.set_state(id, ComponentState::Ready);
    }

    /// Activate component
    pub fn activate(&self, id: ComponentId) {
        self.set_state(id, ComponentState::Active);
    }

    /// Suspend component
    pub fn suspend(&self, id: ComponentId) {
        self.set_state(id, ComponentState::Suspended);
    }

    /// Destroy component
    pub fn destroy(&self, id: ComponentId) {
        self.set_state(id, ComponentState::Destroying);

        // Remove from singletons if applicable
        {
            let components = read_lock(
                &self.components,
                "ComponentManager::destroy(components-read-for-singleton)",
            );
            if let Some(info) = components.get(&id)
                && info.singleton
            {
                let mut singletons = write_lock(
                    &self.singletons,
                    "ComponentManager::destroy(singletons-write)",
                );
                singletons.remove(&info.comp_type);
            }
        }

        let mut components = write_lock(&self.components, "ComponentManager::destroy(components)");
        components.remove(&id);
    }

    /// Get all components of type
    pub fn get_by_type(&self, comp_type: ComponentType) -> Vec<ComponentInfo> {
        let components = read_lock(&self.components, "ComponentManager::get_by_type");
        components
            .values()
            .filter(|c| c.comp_type == comp_type)
            .cloned()
            .collect()
    }

    /// Get all active components
    pub fn get_active(&self) -> Vec<ComponentInfo> {
        let components = read_lock(&self.components, "ComponentManager::get_active");
        components
            .values()
            .filter(|c| c.state == ComponentState::Active)
            .cloned()
            .collect()
    }

    /// Get component count
    pub fn count(&self) -> usize {
        let components = read_lock(&self.components, "ComponentManager::count");
        components.len()
    }

    /// Check if dependencies are met
    pub fn dependencies_met(&self, id: ComponentId) -> bool {
        let components = read_lock(&self.components, "ComponentManager::dependencies_met");
        if let Some(info) = components.get(&id) {
            for dep_type in &info.dependencies {
                let found = components
                    .values()
                    .any(|c| c.comp_type == *dep_type && c.state.is_available());
                if !found {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // =========================================================================
    // ComponentId Tests
    // =========================================================================

    #[test]
    fn test_component_id() {
        let id = ComponentId::new(42);
        assert_eq!(id.value(), 42);
        assert_eq!(format!("{}", id), "Component(42)");
    }

    // =========================================================================
    // ComponentState Tests
    // =========================================================================

    #[test]
    fn test_component_state_available() {
        assert!(ComponentState::Ready.is_available());
        assert!(ComponentState::Active.is_available());
        assert!(!ComponentState::Created.is_available());
        assert!(!ComponentState::Destroyed.is_available());
    }

    #[test]
    fn test_component_state_visible() {
        assert!(ComponentState::Ready.is_visible());
        assert!(ComponentState::Active.is_visible());
        assert!(ComponentState::Suspended.is_visible());
        assert!(!ComponentState::Destroyed.is_visible());
    }

    // =========================================================================
    // ComponentInfo Tests
    // =========================================================================

    #[test]
    fn test_component_info_creation() {
        let info = ComponentInfo::new(ComponentId(1), "Test", ComponentType::SchematicEditor);
        assert_eq!(info.name, "Test");
        assert_eq!(info.comp_type, ComponentType::SchematicEditor);
        assert_eq!(info.state, ComponentState::Created);
    }

    #[test]
    fn test_component_info_singleton() {
        let info =
            ComponentInfo::new(ComponentId(1), "Test", ComponentType::Console).as_singleton();
        assert!(info.singleton);
    }

    #[test]
    fn test_component_info_dependencies() {
        let info = ComponentInfo::new(ComponentId(1), "Test", ComponentType::AnalysisPanel)
            .with_dependency(ComponentType::WaveformViewer);
        assert_eq!(info.dependencies.len(), 1);
    }

    // =========================================================================
    // ComponentManager Tests
    // =========================================================================

    #[test]
    fn test_manager_register() {
        let manager = ComponentManager::new();
        let id = manager
            .register("Test", ComponentType::SchematicEditor)
            .unwrap();

        assert!(id.value() > 0);
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_manager_get() {
        let manager = ComponentManager::new();
        let id = manager.register("Test", ComponentType::Console).unwrap();

        let info = manager.get(id).unwrap();
        assert_eq!(info.name, "Test");
    }

    #[test]
    fn test_manager_singleton() {
        let manager = ComponentManager::new();
        let id1 = manager
            .register_singleton("Console", ComponentType::Console)
            .unwrap();
        let id2 = manager
            .register_singleton("Console2", ComponentType::Console)
            .unwrap();

        assert_eq!(id1, id2);
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_manager_lifecycle() {
        let manager = ComponentManager::new();
        let id = manager.register("Test", ComponentType::Custom).unwrap();

        manager.init(id);
        let info = manager.get(id).unwrap();
        assert_eq!(info.state, ComponentState::Ready);

        manager.activate(id);
        let info = manager.get(id).unwrap();
        assert_eq!(info.state, ComponentState::Active);

        manager.suspend(id);
        let info = manager.get(id).unwrap();
        assert_eq!(info.state, ComponentState::Suspended);
    }

    #[test]
    fn test_manager_destroy() {
        let manager = ComponentManager::new();
        let id = manager.register("Test", ComponentType::Custom).unwrap();
        assert_eq!(manager.count(), 1);

        manager.destroy(id);
        assert_eq!(manager.count(), 0);
        assert!(manager.get(id).is_none());
    }

    #[test]
    fn test_manager_get_by_type() {
        let manager = ComponentManager::new();
        manager
            .register("Wave1", ComponentType::WaveformViewer)
            .unwrap();
        manager
            .register("Wave2", ComponentType::WaveformViewer)
            .unwrap();
        manager
            .register("Schem", ComponentType::SchematicEditor)
            .unwrap();

        let waveforms = manager.get_by_type(ComponentType::WaveformViewer);
        assert_eq!(waveforms.len(), 2);
    }

    #[test]
    fn test_manager_get_active() {
        let manager = ComponentManager::new();
        let id1 = manager.register("C1", ComponentType::Custom).unwrap();
        let id2 = manager.register("C2", ComponentType::Custom).unwrap();

        manager.init(id1);
        manager.activate(id1);
        manager.init(id2);

        let active = manager.get_active();
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn test_manager_recovers_from_poisoned_next_id_lock() {
        let manager = Arc::new(ComponentManager::new());
        let poison_manager = manager.clone();
        let _ = std::thread::spawn(move || {
            let _guard = poison_manager
                .next_id
                .write()
                .expect("next_id lock should be writable before poison");
            panic!("intentional lock poison for manager next_id");
        })
        .join();

        let id = manager
            .register("PostPoison", ComponentType::Custom)
            .expect("register should recover from poisoned id lock");
        assert!(id.value() > 0);
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_manager_recovers_from_poisoned_component_store_lock() {
        let manager = Arc::new(ComponentManager::new());
        let poison_manager = manager.clone();
        let _ = std::thread::spawn(move || {
            let _guard = poison_manager
                .components
                .write()
                .expect("components lock should be writable before poison");
            panic!("intentional lock poison for manager component map");
        })
        .join();

        let id = manager
            .register("Recovered", ComponentType::Custom)
            .expect("register should recover from poisoned component map lock");
        assert!(manager.get(id).is_some());
    }
}

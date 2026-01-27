//! Cross-Probe System Module
//!
//! Commercial-grade cross-probing between schematic and waveform viewers.
//!
//! # Features
//!
//! - Signal selection registry (central signal catalog)
//! - Schematic-to-waveform probing
//! - Waveform-to-schematic highlighting
//! - Bidirectional synchronization
//! - Multi-view coordination
//!
//! # Architecture
//!
//! Follows Cadence Spectre's cross-probe approach using a central registry that
//! maintains signal selection state and broadcasts changes to all connected views.
//!
//! # Modules
//!
//! - `manager` - Core cross-probe coordination logic
//! - `registry` - Signal registry for probe targets
//! - `signal` - Signal identification and paths
//! - `event` - Probe events and handlers
//! - `state` - UI state management
//! - `rendering` - Highlight rendering

pub mod event;
pub mod manager;
pub mod registry;
pub mod rendering;
pub mod signal;
pub mod state;

pub use event::{ProbeEvent, ProbeEventHandler};
pub use manager::{CrossProbeManager, ProbeLink, SchematicTarget, WaveformTarget};
pub use registry::{CrossProbeRegistry, ProbeTarget};
pub use rendering::{NodeHighlight, TraceHighlight};
pub use signal::{ProbeableSignal, SignalId, SignalPath};
pub use state::CrossProbeState;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _registry = CrossProbeRegistry::new();
    }

    #[test]
    fn test_signal_id() {
        let id1 = SignalId::new("v(out)");
        let id2 = SignalId::new("v(out)");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_signal_path() {
        let path = SignalPath::from_parts(&["top", "amp", "out"]);
        assert_eq!(path.full_path(), "top.amp.out");
    }

    #[test]
    fn test_manager_creation() {
        let _manager = CrossProbeManager::new();
    }
}

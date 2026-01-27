//! Integration Module
//!
//! Cross-view communication and coordination for the RSpice UI.
//! Provides event-driven communication between schematic, waveform, and analysis views.
//!
//! # Architecture
//!
//! - `event_bus` - Publish/subscribe event system
//! - `manager` - Component lifecycle management
//! - `state_sync` - Synchronized state across views

pub mod event_bus;
pub mod manager;
pub mod state_sync;

// Re-exports
pub use event_bus::{Event, EventBus, EventHandler, EventType};
pub use manager::{ComponentId, ComponentManager, ComponentState};
pub use state_sync::{StateSynchronizer, SyncState};

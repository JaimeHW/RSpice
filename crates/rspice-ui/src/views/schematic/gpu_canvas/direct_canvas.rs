//! Direct GPU Canvas Component
//!
//! Commercial-grade GPU canvas that renders directly to an HTMLCanvasElement
//! or native window surface, eliminating the slow PNG→base64→DOM pipeline.
//!
//! # Architecture (Cadence Spectre-style)
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                    DirectGpuCanvas Component                        │
//! │                                                                     │
//! │  ┌──────────────────────────────────────────────────────────────┐  │
//! │  │  HTMLCanvasElement (Web) / Native Window (Desktop)           │  │
//! │  │      ↓                                                        │  │
//! │  │  wgpu::Surface                                                │  │
//! │  │      ↓                                                        │  │
//! │  │  WebGpuRuntime::render_to_surface()                          │  │
//! │  │      ↓                                                        │  │
//! │  │  VSync Present (60fps)                                        │  │
//! │  └──────────────────────────────────────────────────────────────┘  │
//! │                                                                     │
//! │  ┌──────────────────────────────────────────────────────────────┐  │
//! │  │  RenderScheduler                                              │  │
//! │  │  - Coalesces input events                                     │  │
//! │  │  - Prioritizes camera updates (fast path)                     │  │
//! │  │  - Batches topology updates (full path)                       │  │
//! │  └──────────────────────────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Performance Characteristics
//!
//! - **Latency**: <16ms (single frame)
//! - **Throughput**: 60fps sustained during pan/zoom
//! - **Memory**: Zero intermediate buffers (direct GPU→display)
//!
//! # Platform Support
//!
//! - **Web**: WebGPU via HTMLCanvasElement (Chrome 113+, Safari 17+)
//! - **Desktop**: Native surface via WebView2 WebGPU or overlay window

use std::sync::Arc;

// =============================================================================
// Render Priority System
// =============================================================================

/// Render request priority levels.
///
/// Commercial-grade simulators prioritize user feedback over visual fidelity
/// during active interaction, then render full quality when idle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RenderPriority {
    /// Highest priority: Camera transform only (pan/zoom)
    /// No schematic data update, just matrix transform
    CameraOnly = 0,

    /// Medium priority: Selection change
    /// Update highlight state but not topology
    SelectionUpdate = 1,

    /// Low priority: Full topology update
    /// Regenerate all GPU buffers from schematic state
    FullUpdate = 2,

    /// Lowest priority: Idle polish
    /// Anti-aliasing, label refinement, etc.
    IdlePolish = 3,
}

impl RenderPriority {
    /// Get display name for logging
    pub fn name(&self) -> &'static str {
        match self {
            Self::CameraOnly => "Camera",
            Self::SelectionUpdate => "Selection",
            Self::FullUpdate => "Full",
            Self::IdlePolish => "Polish",
        }
    }

    /// Check if this is a fast-path operation
    pub fn is_fast_path(&self) -> bool {
        matches!(self, Self::CameraOnly)
    }
}

// =============================================================================
// Render Request
// =============================================================================

/// A request to render a frame.
///
/// Requests are coalesced by the scheduler based on priority.
#[derive(Debug, Clone)]
pub struct RenderRequest {
    /// Priority level
    pub priority: RenderPriority,

    /// Timestamp when request was created (for latency tracking)
    pub timestamp_ms: f64,

    /// Camera state if this is a camera update
    pub camera_position: Option<[f32; 2]>,
    pub camera_zoom: Option<f32>,

    /// Whether topology changed
    pub topology_version: Option<u64>,
}

impl RenderRequest {
    /// Create a camera-only update request (highest priority)
    pub fn camera_update(position: [f32; 2], zoom: f32) -> Self {
        Self {
            priority: RenderPriority::CameraOnly,
            timestamp_ms: current_time_ms(),
            camera_position: Some(position),
            camera_zoom: Some(zoom),
            topology_version: None,
        }
    }

    /// Create a full update request
    pub fn full_update(topology_version: u64) -> Self {
        Self {
            priority: RenderPriority::FullUpdate,
            timestamp_ms: current_time_ms(),
            camera_position: None,
            camera_zoom: None,
            topology_version: Some(topology_version),
        }
    }

    /// Create a selection update request
    pub fn selection_update() -> Self {
        Self {
            priority: RenderPriority::SelectionUpdate,
            timestamp_ms: current_time_ms(),
            camera_position: None,
            camera_zoom: None,
            topology_version: None,
        }
    }
}

/// Get current time in milliseconds
#[cfg(not(target_arch = "wasm32"))]
fn current_time_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

#[cfg(target_arch = "wasm32")]
fn current_time_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

// =============================================================================
// Render Scheduler
// =============================================================================

/// Coalesces and schedules render requests for optimal performance.
///
/// This implements the commercial-grade pattern of prioritizing user feedback:
/// 1. During active pan/zoom: Camera updates only (~0.5ms per frame)
/// 2. After interaction settles: Full topology sync (~5-10ms)
/// 3. During idle: Polish passes (anti-aliasing, etc.)
pub struct RenderScheduler {
    /// Pending requests, sorted by priority
    pending: Vec<RenderRequest>,

    /// Whether currently in active interaction (pan/zoom)
    in_interaction: bool,

    /// Frames since last interaction ended
    settle_frames: u32,

    /// Target settle delay in frames (e.g., 5 frames = ~80ms)
    settle_delay_frames: u32,

    /// Last rendered camera state (for dirty detection)
    last_camera_position: [f32; 2],
    last_camera_zoom: f32,

    /// Last rendered topology version
    last_topology_version: u64,

    /// Performance: total frames rendered
    total_frames: u64,

    /// Performance: fast path frames (camera only)
    fast_path_frames: u64,

    /// Whether any interaction has ever occurred
    had_interaction: bool,
}

impl RenderScheduler {
    /// Create a new render scheduler
    pub fn new() -> Self {
        Self {
            pending: Vec::with_capacity(16),
            in_interaction: false,
            settle_frames: 0,
            settle_delay_frames: 5,
            last_camera_position: [0.0, 0.0],
            last_camera_zoom: 1.0,
            last_topology_version: 0,
            total_frames: 0,
            fast_path_frames: 0,
            had_interaction: false,
        }
    }

    /// Mark interaction started (pan/zoom beginning)
    pub fn begin_interaction(&mut self) {
        self.in_interaction = true;
        self.settle_frames = 0;
        self.had_interaction = true;
    }

    /// Mark interaction ended (mouse up)
    pub fn end_interaction(&mut self) {
        self.in_interaction = false;
        self.settle_frames = 0;
    }

    /// Submit a render request
    pub fn submit(&mut self, request: RenderRequest) {
        // Commercial simulator coalescing strategy:
        //
        // SCOPE: Higher priority VALUE = broader scope (FullUpdate > CameraOnly)
        //   - FullUpdate (2) renders everything, subsumes CameraOnly (0)
        //   - CameraOnly (0) only updates camera transform
        //
        // PROCESSING ORDER: Lower priority VALUE = process first for responsiveness
        //   - CameraOnly (0) is processed before FullUpdate (2)
        //
        // Rules:
        // 1. Broader scope pending → narrower scope redundant, skip it
        // 2. Narrower scope pending + broader arriving → remove narrower
        // 3. Same scope → replace (only need one)

        // Check if a broader-scope request (higher value) is already pending
        let broader_pending = self.pending.iter().any(|r| r.priority > request.priority);
        if broader_pending {
            // This narrower-scope request is redundant
            return;
        }

        // Remove narrower-scope requests (lower value) and same-scope duplicates
        self.pending.retain(|r| r.priority > request.priority);

        // Add the new request
        self.pending.push(request);
    }

    /// Check if camera changed from last render
    pub fn camera_changed(&self, position: [f32; 2], zoom: f32) -> bool {
        (self.last_camera_position[0] - position[0]).abs() > 0.001
            || (self.last_camera_position[1] - position[1]).abs() > 0.001
            || (self.last_camera_zoom - zoom).abs() > 0.0001
    }

    /// Check if topology changed from last render
    pub fn topology_changed(&self, version: u64) -> bool {
        version != self.last_topology_version
    }

    /// Get next render to execute, if any
    ///
    /// Returns the highest-priority pending request, or None if nothing to do.
    pub fn next_render(&mut self) -> Option<RenderRequest> {
        if self.pending.is_empty() {
            // During settle period, nothing to render yet
            if !self.in_interaction && self.settle_frames < self.settle_delay_frames {
                self.settle_frames += 1;
                return None;
            }
            return None;
        }

        // Sort by priority (lower value = higher priority)
        self.pending.sort_by_key(|r| r.priority);

        // Take highest priority request
        let request = self.pending.remove(0);

        // Track statistics
        self.total_frames += 1;
        if request.priority.is_fast_path() {
            self.fast_path_frames += 1;
        }

        Some(request)
    }

    /// Mark render complete with final state
    pub fn render_complete(&mut self, position: [f32; 2], zoom: f32, topology_version: u64) {
        self.last_camera_position = position;
        self.last_camera_zoom = zoom;
        self.last_topology_version = topology_version;
    }

    /// Get fast path ratio (0.0 to 1.0)
    pub fn fast_path_ratio(&self) -> f32 {
        if self.total_frames == 0 {
            0.0
        } else {
            self.fast_path_frames as f32 / self.total_frames as f32
        }
    }

    /// Get total frame count
    pub fn total_frames(&self) -> u64 {
        self.total_frames
    }

    /// Check if in active interaction
    pub fn is_interacting(&self) -> bool {
        self.in_interaction
    }

    /// Check if settling after interaction
    pub fn is_settling(&self) -> bool {
        self.had_interaction
            && !self.in_interaction
            && self.settle_frames < self.settle_delay_frames
    }

    /// Clear all pending requests
    pub fn clear(&mut self) {
        self.pending.clear();
    }
}

impl Default for RenderScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Frame Timing
// =============================================================================

/// Frame timing tracker for performance monitoring
#[derive(Debug, Clone)]
pub struct FrameTiming {
    /// Frame start timestamp
    pub start_ms: f64,

    /// GPU buffer update time
    pub buffer_update_ms: f64,

    /// Render pass execution time
    pub render_ms: f64,

    /// Present time
    pub present_ms: f64,

    /// Total frame time
    pub total_ms: f64,

    /// Priority of this frame
    pub priority: RenderPriority,
}

impl FrameTiming {
    /// Create a new frame timing starting now
    pub fn begin(priority: RenderPriority) -> Self {
        Self {
            start_ms: current_time_ms(),
            buffer_update_ms: 0.0,
            render_ms: 0.0,
            present_ms: 0.0,
            total_ms: 0.0,
            priority,
        }
    }

    /// Mark buffer update complete
    pub fn mark_buffer_complete(&mut self) {
        self.buffer_update_ms = current_time_ms() - self.start_ms;
    }

    /// Mark render complete
    pub fn mark_render_complete(&mut self) {
        self.render_ms = current_time_ms() - self.start_ms - self.buffer_update_ms;
    }

    /// Mark frame complete
    pub fn mark_complete(&mut self) {
        self.total_ms = current_time_ms() - self.start_ms;
        self.present_ms = self.total_ms - self.buffer_update_ms - self.render_ms;
    }

    /// Check if frame was within budget (16.67ms for 60fps)
    pub fn within_budget(&self) -> bool {
        self.total_ms < 16.67
    }
}

/// Rolling frame statistics
#[derive(Debug, Clone, Default)]
pub struct FrameStats {
    /// Frame count
    pub frame_count: u64,

    /// Average frame time (exponential moving average)
    pub avg_frame_ms: f64,

    /// Average FPS
    pub fps: f64,

    /// Frames that exceeded 16.67ms budget
    pub dropped_frames: u64,

    /// Fast path percentage
    pub fast_path_pct: f64,
}

impl FrameStats {
    /// Update stats with a new frame timing
    pub fn update(&mut self, timing: &FrameTiming, scheduler: &RenderScheduler) {
        self.frame_count += 1;

        // Exponential moving average for smooth stats
        const ALPHA: f64 = 0.1;
        self.avg_frame_ms = self.avg_frame_ms * (1.0 - ALPHA) + timing.total_ms * ALPHA;

        if self.avg_frame_ms > 0.0 {
            self.fps = 1000.0 / self.avg_frame_ms;
        }

        if !timing.within_budget() {
            self.dropped_frames += 1;
        }

        self.fast_path_pct = scheduler.fast_path_ratio() as f64 * 100.0;
    }

    /// Format stats for display
    pub fn format(&self) -> String {
        format!(
            "{:.1}fps | {:.2}ms | {:.0}% fast",
            self.fps, self.avg_frame_ms, self.fast_path_pct
        )
    }
}

// =============================================================================
// Surface Configuration
// =============================================================================

/// Configuration for the GPU surface
#[derive(Debug, Clone)]
pub struct SurfaceConfig {
    /// Canvas element ID (web)
    pub canvas_id: String,

    /// Initial width
    pub width: u32,

    /// Initial height
    pub height: u32,

    /// VSync mode
    pub vsync: bool,

    /// MSAA sample count (1, 2, 4, 8)
    pub msaa_samples: u32,
}

impl Default for SurfaceConfig {
    fn default() -> Self {
        Self {
            canvas_id: "gpu-schematic-canvas".to_string(),
            width: 800,
            height: 600,
            vsync: true,
            msaa_samples: 1,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // RenderPriority Tests
    // =========================================================================

    #[test]
    fn test_priority_ordering() {
        assert!(RenderPriority::CameraOnly < RenderPriority::SelectionUpdate);
        assert!(RenderPriority::SelectionUpdate < RenderPriority::FullUpdate);
        assert!(RenderPriority::FullUpdate < RenderPriority::IdlePolish);
    }

    #[test]
    fn test_priority_names() {
        assert_eq!(RenderPriority::CameraOnly.name(), "Camera");
        assert_eq!(RenderPriority::SelectionUpdate.name(), "Selection");
        assert_eq!(RenderPriority::FullUpdate.name(), "Full");
        assert_eq!(RenderPriority::IdlePolish.name(), "Polish");
    }

    #[test]
    fn test_priority_fast_path() {
        assert!(RenderPriority::CameraOnly.is_fast_path());
        assert!(!RenderPriority::SelectionUpdate.is_fast_path());
        assert!(!RenderPriority::FullUpdate.is_fast_path());
        assert!(!RenderPriority::IdlePolish.is_fast_path());
    }

    // =========================================================================
    // RenderRequest Tests
    // =========================================================================

    #[test]
    fn test_camera_update_request() {
        let req = RenderRequest::camera_update([100.0, 200.0], 2.0);
        assert_eq!(req.priority, RenderPriority::CameraOnly);
        assert_eq!(req.camera_position, Some([100.0, 200.0]));
        assert_eq!(req.camera_zoom, Some(2.0));
        assert!(req.topology_version.is_none());
    }

    #[test]
    fn test_full_update_request() {
        let req = RenderRequest::full_update(42);
        assert_eq!(req.priority, RenderPriority::FullUpdate);
        assert!(req.camera_position.is_none());
        assert!(req.camera_zoom.is_none());
        assert_eq!(req.topology_version, Some(42));
    }

    #[test]
    fn test_selection_update_request() {
        let req = RenderRequest::selection_update();
        assert_eq!(req.priority, RenderPriority::SelectionUpdate);
        assert!(req.camera_position.is_none());
    }

    #[test]
    fn test_request_timestamp() {
        let req = RenderRequest::camera_update([0.0, 0.0], 1.0);
        // Timestamp should be positive (after epoch)
        assert!(req.timestamp_ms >= 0.0);
    }

    // =========================================================================
    // RenderScheduler Tests
    // =========================================================================

    #[test]
    fn test_scheduler_new() {
        let scheduler = RenderScheduler::new();
        assert!(!scheduler.is_interacting());
        assert!(!scheduler.is_settling());
        assert_eq!(scheduler.total_frames(), 0);
    }

    #[test]
    fn test_scheduler_interaction_state() {
        let mut scheduler = RenderScheduler::new();

        scheduler.begin_interaction();
        assert!(scheduler.is_interacting());

        scheduler.end_interaction();
        assert!(!scheduler.is_interacting());
        assert!(scheduler.is_settling());
    }

    #[test]
    fn test_scheduler_submit_and_next() {
        let mut scheduler = RenderScheduler::new();

        scheduler.submit(RenderRequest::full_update(1));
        let req = scheduler.next_render();

        assert!(req.is_some());
        assert_eq!(req.unwrap().priority, RenderPriority::FullUpdate);
    }

    #[test]
    fn test_scheduler_priority_ordering() {
        let mut scheduler = RenderScheduler::new();

        // Submit narrower scope first, then broader
        // CameraOnly is narrower (just camera), FullUpdate is broader (everything)
        scheduler.submit(RenderRequest::camera_update([0.0, 0.0], 1.0));
        scheduler.submit(RenderRequest::full_update(1));

        // FullUpdate subsumes CameraOnly - only FullUpdate should remain
        // This is commercial-grade behavior: broader scope wins
        let req1 = scheduler.next_render().unwrap();
        assert_eq!(req1.priority, RenderPriority::FullUpdate);
        assert!(scheduler.next_render().is_none());
    }

    #[test]
    fn test_scheduler_coalesces_same_scope() {
        let mut scheduler = RenderScheduler::new();

        // Submit multiple camera updates (same scope)
        scheduler.submit(RenderRequest::camera_update([0.0, 0.0], 1.0));
        scheduler.submit(RenderRequest::camera_update([10.0, 20.0], 2.0));
        scheduler.submit(RenderRequest::camera_update([30.0, 40.0], 3.0));

        // Only latest should remain (same scope replaces)
        let req = scheduler.next_render().unwrap();
        assert_eq!(req.priority, RenderPriority::CameraOnly);
        assert_eq!(req.camera_position, Some([30.0, 40.0]));
        assert_eq!(req.camera_zoom, Some(3.0));
        assert!(scheduler.next_render().is_none());
    }

    #[test]
    fn test_scheduler_camera_changed() {
        let mut scheduler = RenderScheduler::new();

        assert!(scheduler.camera_changed([100.0, 0.0], 1.0));

        scheduler.render_complete([100.0, 0.0], 1.0, 0);
        assert!(!scheduler.camera_changed([100.0, 0.0], 1.0));
        assert!(scheduler.camera_changed([100.1, 0.0], 1.0));
    }

    #[test]
    fn test_scheduler_topology_changed() {
        let mut scheduler = RenderScheduler::new();

        assert!(scheduler.topology_changed(1));

        scheduler.render_complete([0.0, 0.0], 1.0, 1);
        assert!(!scheduler.topology_changed(1));
        assert!(scheduler.topology_changed(2));
    }

    #[test]
    fn test_scheduler_fast_path_ratio() {
        let mut scheduler = RenderScheduler::new();

        // Initial ratio should be 0
        assert_eq!(scheduler.fast_path_ratio(), 0.0);

        // Submit and process fast path requests
        for _ in 0..8 {
            scheduler.submit(RenderRequest::camera_update([0.0, 0.0], 1.0));
            scheduler.next_render();
        }

        // Submit and process full update
        scheduler.submit(RenderRequest::full_update(1));
        scheduler.next_render();
        scheduler.submit(RenderRequest::full_update(2));
        scheduler.next_render();

        // 80% fast path
        assert!((scheduler.fast_path_ratio() - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_scheduler_clear() {
        let mut scheduler = RenderScheduler::new();

        scheduler.submit(RenderRequest::full_update(1));
        scheduler.submit(RenderRequest::camera_update([0.0, 0.0], 1.0));

        scheduler.clear();
        assert!(scheduler.next_render().is_none());
    }

    #[test]
    fn test_scheduler_settle_delay() {
        let mut scheduler = RenderScheduler::new();
        scheduler.settle_delay_frames = 3;

        // Start and end interaction
        scheduler.begin_interaction();
        scheduler.end_interaction();

        // Should be settling
        assert!(scheduler.is_settling());

        // No pending requests, should return None during settle
        assert!(scheduler.next_render().is_none());
        assert!(scheduler.is_settling()); // settle_frames = 1

        assert!(scheduler.next_render().is_none());
        assert!(scheduler.is_settling()); // settle_frames = 2

        assert!(scheduler.next_render().is_none());
        assert!(!scheduler.is_settling()); // settle_frames = 3, no longer settling
    }

    // =========================================================================
    // FrameTiming Tests
    // =========================================================================

    #[test]
    fn test_frame_timing_begin() {
        let timing = FrameTiming::begin(RenderPriority::CameraOnly);
        assert!(timing.start_ms > 0.0);
        assert_eq!(timing.priority, RenderPriority::CameraOnly);
    }

    #[test]
    fn test_frame_timing_marks() {
        let mut timing = FrameTiming::begin(RenderPriority::FullUpdate);

        timing.mark_buffer_complete();
        assert!(timing.buffer_update_ms >= 0.0);

        timing.mark_render_complete();
        assert!(timing.render_ms >= 0.0);

        timing.mark_complete();
        assert!(timing.total_ms >= 0.0);
    }

    #[test]
    fn test_frame_timing_within_budget() {
        let mut timing = FrameTiming::begin(RenderPriority::CameraOnly);
        timing.total_ms = 10.0; // Under 16.67ms
        assert!(timing.within_budget());

        timing.total_ms = 20.0; // Over 16.67ms
        assert!(!timing.within_budget());
    }

    // =========================================================================
    // FrameStats Tests
    // =========================================================================

    #[test]
    fn test_frame_stats_default() {
        let stats = FrameStats::default();
        assert_eq!(stats.frame_count, 0);
        assert_eq!(stats.fps, 0.0);
        assert_eq!(stats.dropped_frames, 0);
    }

    #[test]
    fn test_frame_stats_update() {
        let mut stats = FrameStats::default();
        let scheduler = RenderScheduler::new();

        let mut timing = FrameTiming::begin(RenderPriority::CameraOnly);
        timing.total_ms = 10.0;

        stats.update(&timing, &scheduler);

        assert_eq!(stats.frame_count, 1);
        assert!(stats.fps > 0.0);
    }

    #[test]
    fn test_frame_stats_dropped_frames() {
        let mut stats = FrameStats::default();
        let scheduler = RenderScheduler::new();

        // Frame under budget
        let mut timing1 = FrameTiming::begin(RenderPriority::CameraOnly);
        timing1.total_ms = 10.0;
        stats.update(&timing1, &scheduler);
        assert_eq!(stats.dropped_frames, 0);

        // Frame over budget
        let mut timing2 = FrameTiming::begin(RenderPriority::FullUpdate);
        timing2.total_ms = 25.0;
        stats.update(&timing2, &scheduler);
        assert_eq!(stats.dropped_frames, 1);
    }

    #[test]
    fn test_frame_stats_format() {
        let mut stats = FrameStats::default();
        stats.fps = 60.0;
        stats.avg_frame_ms = 16.67;
        stats.fast_path_pct = 75.0;

        let formatted = stats.format();
        assert!(formatted.contains("60.0fps"));
        assert!(formatted.contains("16.67ms"));
        assert!(formatted.contains("75% fast"));
    }

    // =========================================================================
    // SurfaceConfig Tests
    // =========================================================================

    #[test]
    fn test_surface_config_default() {
        let config = SurfaceConfig::default();
        assert_eq!(config.canvas_id, "gpu-schematic-canvas");
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert!(config.vsync);
        assert_eq!(config.msaa_samples, 1);
    }

    #[test]
    fn test_surface_config_custom() {
        let config = SurfaceConfig {
            canvas_id: "my-canvas".to_string(),
            width: 1920,
            height: 1080,
            vsync: false,
            msaa_samples: 4,
        };

        assert_eq!(config.canvas_id, "my-canvas");
        assert_eq!(config.width, 1920);
        assert!(!config.vsync);
        assert_eq!(config.msaa_samples, 4);
    }

    // =========================================================================
    // Integration Tests (Mock)
    // =========================================================================

    #[test]
    fn test_pan_workflow() {
        let mut scheduler = RenderScheduler::new();

        // Simulate pan start
        scheduler.begin_interaction();

        // Multiple camera updates during pan
        // Start from 1 to ensure first position differs from default [0,0]
        for i in 1..=10 {
            let pos = [i as f32 * 10.0, 0.0];
            if scheduler.camera_changed(pos, 1.0) {
                scheduler.submit(RenderRequest::camera_update(pos, 1.0));
            }

            let req = scheduler.next_render().unwrap();
            assert!(req.priority.is_fast_path());
            scheduler.render_complete(pos, 1.0, 0);
        }

        // All should be fast path
        assert_eq!(scheduler.fast_path_ratio(), 1.0);

        // Pan ends
        scheduler.end_interaction();
        assert!(scheduler.is_settling());
    }

    #[test]
    fn test_zoom_workflow() {
        let mut scheduler = RenderScheduler::new();

        scheduler.begin_interaction();

        // Zoom from 1.1 to 2.0 (start at 1.1 to differ from default 1.0)
        let zoom_levels = [1.1, 1.3, 1.5, 1.7, 1.9, 2.0];
        for zoom in zoom_levels {
            if scheduler.camera_changed([0.0, 0.0], zoom) {
                scheduler.submit(RenderRequest::camera_update([0.0, 0.0], zoom));
            }
            if let Some(_req) = scheduler.next_render() {
                scheduler.render_complete([0.0, 0.0], zoom, 0);
            }
        }

        scheduler.end_interaction();
    }

    #[test]
    fn test_topology_change_during_idle() {
        let mut scheduler = RenderScheduler::new();

        // Initial state
        scheduler.render_complete([0.0, 0.0], 1.0, 1);

        // Topology changes (component added)
        assert!(scheduler.topology_changed(2));
        scheduler.submit(RenderRequest::full_update(2));

        let req = scheduler.next_render().unwrap();
        assert_eq!(req.priority, RenderPriority::FullUpdate);
    }

    #[test]
    fn test_mixed_priority_workflow() {
        let mut scheduler = RenderScheduler::new();

        // Commercial behavior: broader scope subsumes narrower
        // FullUpdate(2) is broadest, so CameraOnly(0) and SelectionUpdate(1) are redundant
        scheduler.submit(RenderRequest::full_update(1));
        scheduler.submit(RenderRequest::selection_update()); // Redundant, skipped
        scheduler.submit(RenderRequest::camera_update([10.0, 20.0], 1.5)); // Redundant, skipped

        // Only FullUpdate should be pending
        let r1 = scheduler.next_render().unwrap();
        assert_eq!(r1.priority, RenderPriority::FullUpdate);

        // Nothing else pending
        assert!(scheduler.next_render().is_none());
    }
}

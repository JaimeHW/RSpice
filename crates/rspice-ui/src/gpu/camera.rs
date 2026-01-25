//! Camera System
//!
//! Orthographic camera for 2D schematic viewing with pan and zoom.
//! The camera provides world-to-screen and screen-to-world transformations.

use bytemuck::{Pod, Zeroable};

/// 4x4 matrix for GPU uniform buffer
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mat4 {
    /// Create identity matrix
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create orthographic projection matrix
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;

        Self {
            data: [
                [2.0 / width, 0.0, 0.0, 0.0],
                [0.0, 2.0 / height, 0.0, 0.0],
                [0.0, 0.0, -2.0 / depth, 0.0],
                [
                    -(right + left) / width,
                    -(top + bottom) / height,
                    -(far + near) / depth,
                    1.0,
                ],
            ],
        }
    }

    /// Create translation matrix
    pub fn translation(x: f32, y: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, 0.0, 1.0],
            ],
        }
    }

    /// Create scale matrix
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            data: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Multiply two matrices
    pub fn mul(&self, other: &Mat4) -> Mat4 {
        let mut result = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Mat4 { data: result }
    }
}

/// Camera uniform buffer data sent to GPU
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    /// Combined view-projection matrix
    pub view_proj: Mat4,
    /// Viewport dimensions (width, height, 1/width, 1/height)
    pub viewport: [f32; 4],
    /// Zoom level for LOD decisions in shader
    pub zoom: f32,
    /// Grid size in world units
    pub grid_size: f32,
    /// Padding for alignment
    pub _padding: [f32; 2],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: Mat4::identity(),
            viewport: [800.0, 600.0, 1.0 / 800.0, 1.0 / 600.0],
            zoom: 1.0,
            grid_size: 10.0,
            _padding: [0.0; 2],
        }
    }
}

/// Orthographic camera for 2D schematic viewing
#[derive(Debug, Clone)]
pub struct Camera {
    /// Camera position in world space (center of view)
    pub position: [f32; 2],

    /// Zoom level (1.0 = 100%, 2.0 = 200% magnification)
    pub zoom: f32,

    /// Viewport width in pixels
    pub viewport_width: f32,

    /// Viewport height in pixels
    pub viewport_height: f32,

    /// Grid size in world units
    pub grid_size: f32,

    /// Minimum zoom level
    pub min_zoom: f32,

    /// Maximum zoom level
    pub max_zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            zoom: 1.0,
            viewport_width: 800.0,
            viewport_height: 600.0,
            grid_size: 10.0,
            min_zoom: 0.1,
            max_zoom: 10.0,
        }
    }
}

impl Camera {
    /// Create a new camera with the given viewport size
    pub fn new(width: f32, height: f32, grid_size: f32) -> Self {
        Self {
            viewport_width: width,
            viewport_height: height,
            grid_size,
            ..Default::default()
        }
    }

    /// Update viewport dimensions
    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport_width = width.max(1.0);
        self.viewport_height = height.max(1.0);
    }

    /// Set zoom level, clamping to valid range
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(self.min_zoom, self.max_zoom);
    }

    /// Zoom by a factor (e.g., 1.1 for 10% zoom in)
    pub fn zoom_by(&mut self, factor: f32) {
        self.set_zoom(self.zoom * factor);
    }

    /// Pan by screen pixels
    pub fn pan_by_pixels(&mut self, dx: f32, dy: f32) {
        // Convert screen pixels to world units
        let world_dx = dx / (self.zoom * self.grid_size);
        let world_dy = dy / (self.zoom * self.grid_size);
        self.position[0] -= world_dx;
        self.position[1] -= world_dy;
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> [f32; 2] {
        // Screen center
        let cx = self.viewport_width / 2.0;
        let cy = self.viewport_height / 2.0;

        // Offset from center in pixels
        let dx = screen_x - cx;
        let dy = screen_y - cy;

        // Convert to world coordinates
        let world_x = self.position[0] + dx / (self.zoom * self.grid_size);
        let world_y = self.position[1] - dy / (self.zoom * self.grid_size); // Y is flipped

        [world_x, world_y]
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> [f32; 2] {
        let cx = self.viewport_width / 2.0;
        let cy = self.viewport_height / 2.0;

        let screen_x = cx + (world_x - self.position[0]) * self.zoom * self.grid_size;
        let screen_y = cy - (world_y - self.position[1]) * self.zoom * self.grid_size;

        [screen_x, screen_y]
    }

    /// Get the visible world bounds
    pub fn world_bounds(&self) -> WorldBounds {
        let half_width = self.viewport_width / (2.0 * self.zoom * self.grid_size);
        let half_height = self.viewport_height / (2.0 * self.zoom * self.grid_size);

        WorldBounds {
            min_x: self.position[0] - half_width,
            max_x: self.position[0] + half_width,
            min_y: self.position[1] - half_height,
            max_y: self.position[1] + half_height,
        }
    }

    /// Build the camera uniform buffer data for GPU
    pub fn build_uniform(&self) -> CameraUniform {
        let bounds = self.world_bounds();

        // Create orthographic projection from world bounds
        // Note: Y is flipped for screen coordinates (top = min_y, bottom = max_y)
        let proj = Mat4::orthographic(
            bounds.min_x,
            bounds.max_x,
            bounds.max_y, // bottom
            bounds.min_y, // top (flipped)
            -1.0,
            1.0,
        );

        CameraUniform {
            view_proj: proj,
            viewport: [
                self.viewport_width,
                self.viewport_height,
                1.0 / self.viewport_width,
                1.0 / self.viewport_height,
            ],
            zoom: self.zoom,
            grid_size: self.grid_size,
            _padding: [0.0; 2],
        }
    }
}

/// Visible world bounds
#[derive(Debug, Clone, Copy)]
pub struct WorldBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl WorldBounds {
    /// Check if a point is within bounds
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// Check if a rectangle intersects bounds
    pub fn intersects(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> bool {
        self.max_x >= min_x && self.min_x <= max_x && self.max_y >= min_y && self.min_y <= max_y
    }
}

/// Camera controller handling input events
#[derive(Debug, Default, Clone)]
pub struct CameraController {
    /// Is panning active
    pub is_panning: bool,

    /// Last mouse position for panning
    pub last_mouse: [f32; 2],
}

impl CameraController {
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin panning from the given position
    pub fn start_pan(&mut self, x: f32, y: f32) {
        self.is_panning = true;
        self.last_mouse = [x, y];
    }

    /// Update pan with new mouse position
    pub fn update_pan(&mut self, camera: &mut Camera, x: f32, y: f32) {
        if self.is_panning {
            let dx = x - self.last_mouse[0];
            let dy = y - self.last_mouse[1];
            camera.pan_by_pixels(dx, dy);
            self.last_mouse = [x, y];
        }
    }

    /// End panning
    pub fn end_pan(&mut self) {
        self.is_panning = false;
    }

    /// Handle scroll wheel zoom
    pub fn zoom_at(&mut self, camera: &mut Camera, screen_x: f32, screen_y: f32, delta: f32) {
        // Get world position under cursor before zoom
        let world_before = camera.screen_to_world(screen_x, screen_y);

        // Apply zoom
        let factor = if delta > 0.0 { 1.1 } else { 1.0 / 1.1 };
        camera.zoom_by(factor);

        // Get world position after zoom
        let world_after = camera.screen_to_world(screen_x, screen_y);

        // Adjust camera to keep cursor at same world position
        camera.position[0] += world_before[0] - world_after[0];
        camera.position[1] += world_before[1] - world_after[1];
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_default() {
        let camera = Camera::default();
        assert_eq!(camera.zoom, 1.0);
        assert_eq!(camera.position, [0.0, 0.0]);
    }

    #[test]
    fn test_screen_to_world_center() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        // Center of screen should map to camera position
        let world = camera.screen_to_world(400.0, 300.0);
        assert!((world[0] - 0.0).abs() < 0.001);
        assert!((world[1] - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_world_to_screen_roundtrip() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        let world = [5.0, 10.0];
        let screen = camera.world_to_screen(world[0], world[1]);
        let back = camera.screen_to_world(screen[0], screen[1]);
        assert!((back[0] - world[0]).abs() < 0.001);
        assert!((back[1] - world[1]).abs() < 0.001);
    }

    #[test]
    fn test_zoom_clamp() {
        let mut camera = Camera::default();
        camera.set_zoom(0.01); // Below min
        assert!(camera.zoom >= camera.min_zoom);

        camera.set_zoom(100.0); // Above max
        assert!(camera.zoom <= camera.max_zoom);
    }

    #[test]
    fn test_world_bounds() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        let bounds = camera.world_bounds();
        assert!(bounds.min_x < bounds.max_x);
        assert!(bounds.min_y < bounds.max_y);
        // Center should be within bounds
        assert!(bounds.contains(0.0, 0.0));
    }

    #[test]
    fn test_mat4_identity() {
        let m = Mat4::identity();
        assert_eq!(m.data[0][0], 1.0);
        assert_eq!(m.data[1][1], 1.0);
        assert_eq!(m.data[2][2], 1.0);
        assert_eq!(m.data[3][3], 1.0);
    }
}

//! GPU Buffer Management
//!
//! Dynamic buffer pools for efficient GPU memory usage.
//! Buffers are grown as needed and reused across frames.

use std::sync::Arc;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

/// Buffer pool for dynamic vertex/instance data
#[derive(Debug)]
pub struct DynamicBuffer {
    /// GPU buffer
    buffer: Option<Buffer>,

    /// Current capacity in bytes
    capacity: usize,

    /// Current used size in bytes
    size: usize,

    /// Buffer usage flags
    usage: BufferUsages,

    /// Label for debugging
    label: String,
}

impl DynamicBuffer {
    /// Create a new dynamic buffer
    pub fn new(label: impl Into<String>, usage: BufferUsages) -> Self {
        Self {
            buffer: None,
            capacity: 0,
            size: 0,
            usage,
            label: label.into(),
        }
    }

    /// Get current buffer, if any
    pub fn buffer(&self) -> Option<&Buffer> {
        self.buffer.as_ref()
    }

    /// Get current size in bytes
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get current capacity in bytes
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Check if buffer needs reallocation for given size
    pub fn needs_realloc(&self, required_size: usize) -> bool {
        required_size > self.capacity
    }

    /// Ensure buffer has at least the given capacity
    pub fn ensure_capacity(&mut self, device: &Device, required_size: usize) {
        if required_size > self.capacity {
            // Grow by 1.5x or to required size, whichever is larger
            let new_capacity = (self.capacity * 3 / 2).max(required_size).max(1024);

            self.buffer = Some(device.create_buffer(&BufferDescriptor {
                label: Some(&self.label),
                size: new_capacity as u64,
                usage: self.usage | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));

            self.capacity = new_capacity;
            log::debug!(
                "Reallocated buffer '{}': {} -> {} bytes",
                self.label,
                self.size,
                new_capacity
            );
        }
    }

    /// Write data to buffer, reallocating if necessary
    pub fn write<T: bytemuck::Pod>(&mut self, device: &Device, queue: &Queue, data: &[T]) {
        let byte_size = std::mem::size_of_val(data);
        self.ensure_capacity(device, byte_size);

        if let Some(buffer) = &self.buffer {
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(data));
        }

        self.size = byte_size;
    }

    /// Clear buffer (just resets size, doesn't deallocate)
    pub fn clear(&mut self) {
        self.size = 0;
    }
}

/// Uniform buffer with typed data
pub struct UniformBuffer<T: bytemuck::Pod> {
    /// GPU buffer
    buffer: Buffer,

    /// Current value
    value: T,
}

impl<T: bytemuck::Pod + Clone> UniformBuffer<T> {
    /// Create a new uniform buffer with initial value
    pub fn new(device: &Device, label: &str, initial: T) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size: std::mem::size_of::<T>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            value: initial,
        }
    }

    /// Get buffer reference
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get current value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Update value and upload to GPU
    pub fn set(&mut self, queue: &Queue, value: T) {
        self.value = value.clone();
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.value));
    }

    /// Upload current value to GPU
    pub fn upload(&self, queue: &Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.value));
    }
}

/// Buffer pool managing multiple dynamic buffers
#[derive(Debug)]
pub struct BufferPool {
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl BufferPool {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self { device, queue }
    }

    /// Create a new dynamic vertex buffer
    pub fn create_vertex_buffer(&self, label: impl Into<String>) -> DynamicBuffer {
        DynamicBuffer::new(label, BufferUsages::VERTEX)
    }

    /// Create a new dynamic index buffer
    pub fn create_index_buffer(&self, label: impl Into<String>) -> DynamicBuffer {
        DynamicBuffer::new(label, BufferUsages::INDEX)
    }

    /// Create a new dynamic instance buffer
    pub fn create_instance_buffer(&self, label: impl Into<String>) -> DynamicBuffer {
        DynamicBuffer::new(label, BufferUsages::VERTEX)
    }

    /// Device reference
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Queue reference
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_buffer_growth() {
        let buf = DynamicBuffer::new("test", BufferUsages::VERTEX);
        assert!(buf.needs_realloc(100));
        assert!(buf.buffer().is_none());
    }
}

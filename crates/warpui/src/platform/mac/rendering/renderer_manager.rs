use pathfinder_geometry::vector::Vector2F;

use super::{
    metal,
    renderer::{Device, Renderer},
};

pub struct RendererManager {
    metal_renderer_manager: metal::RendererManager,
    #[cfg(wgpu)]
    wgpu_renderer_manager: super::wgpu::RendererManager,
}

impl Default for RendererManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RendererManager {
    pub fn new() -> Self {
        Self {
            metal_renderer_manager: metal::RendererManager::new(),
            #[cfg(wgpu)]
            wgpu_renderer_manager: super::wgpu::RendererManager::new(),
        }
    }

    /// Returns a [`Renderer`] that can be used to render on the given [`Device`].
    ///
    /// May fail if the underlying renderer cannot be initialized (e.g. shader compilation
    /// failure, GPU driver issue). Callers should log and degrade gracefully — historically
    /// this codepath aborted the process on any GPU init failure, which made app launch
    /// failures impossible to diagnose.
    #[allow(unused_variables)]
    pub fn renderer_for_device(
        &mut self,
        device: &Device,
        window_size: Vector2F,
    ) -> anyhow::Result<&mut dyn Renderer> {
        let renderer: &mut dyn Renderer = match device {
            Device::Metal(device) => self.metal_renderer_manager.renderer_for_device(device)?,
            #[cfg(wgpu)]
            Device::WGPU(resources) => self
                .wgpu_renderer_manager
                .renderer_for_resources(resources, window_size),
        };
        Ok(renderer)
    }
}

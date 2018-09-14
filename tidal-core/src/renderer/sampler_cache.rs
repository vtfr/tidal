use std::collections::hash_map::Entry;
use std::collections::HashMap;

use wgpu::Label;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct SamplerDescriptor {
    pub address_mode_u: wgpu::AddressMode,
    pub address_mode_v: wgpu::AddressMode,
    pub mag_filter: wgpu::FilterMode,
    pub min_filter: wgpu::FilterMode,
    pub mipmap_filter: wgpu::FilterMode,
}

impl Into<wgpu::SamplerDescriptor<'static>> for &SamplerDescriptor {
    fn into(self) -> wgpu::SamplerDescriptor<'static> {
        wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: Default::default(),
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            mipmap_filter: self.mipmap_filter,
            ..Default::default()
        }
    }
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct SamplerCache {
    cache: HashMap<SamplerDescriptor, wgpu::Sampler>,
}

impl SamplerCache {
    /// Creates a new [`wgpu::Sampler`] sampler if no sampler already exists for that specification.
    pub fn create_sampler(
        &mut self,
        device: &wgpu::Device,
        descriptor: &SamplerDescriptor,
    ) -> &wgpu::Sampler {
        self.cache
            .entry(descriptor.clone())
            .or_insert_with(|| device.create_sampler(&descriptor.into()))
    }
}

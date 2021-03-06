use super::RawTexture;
use crate::graphic::Gpu;
use image::DynamicImage;

pub struct Texture {
    view: wgpu::TextureView,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_raw(raw: &RawTexture, width: u32, height: u32) -> Self {
        // Create texture view
        let view = raw.as_raw().create_view(&wgpu::TextureViewDescriptor {
            format: wgpu::TextureFormat::Bgra8Unorm,
            dimension: wgpu::TextureViewDimension::D2,
            aspect: wgpu::TextureAspect::default(),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            array_layer_count: 1,
        });

        Self {
            view,
            width,
            height,
        }
    }

    pub fn from_bytes(gpu: &mut Gpu, width: u32, height: u32, bytes: &[u8]) -> Self {
        let raw =
            RawTexture::from_bytes(gpu, width, height, wgpu::TextureUsage::SAMPLED, bytes);
        Self::from_raw(&raw, width, height)
    }

    pub fn from_image(gpu: &mut Gpu, img: &DynamicImage) -> Self {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        Self::from_bytes(gpu, width, height, bytes)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl super::AsTextureView for Texture {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl super::AsTextureView for &Texture {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

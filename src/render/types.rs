use crate::{context::{Context}};

pub struct Framebuffer {
    texture: wgpu::Texture,
    sample_count: u32,
}

impl Framebuffer {
    pub fn new(context: &Context, sample_count: u32) -> Self
    {  
        let texture = context.device.create_texture(&wgpu::TextureDescriptor
        { 
            label: Some("Texture"), 
            size: wgpu::Extent3d {width: context.config.width, height: context.config.height, depth_or_array_layers: 1}, 
            mip_level_count: 1, 
            sample_count, 
            dimension: wgpu::TextureDimension::D2, 
            format: context.config.format, 
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, 
            view_formats: &context.config.view_formats,
        });

        Framebuffer { texture, sample_count }
    } 

    pub fn resize(&mut self, context: &Context, width: u32, height: u32)
    {
        if width > 0 && height > 0
        {
            let samples = self.texture.sample_count();
            self.create_buffer(context, samples, width, height);
        }
    }

    pub fn change_sample_count(&mut self, context: &Context, sample_count: u32) -> bool
    {
        if context.features.texture_features.sample_count_supported(sample_count)
        {
            let width = self.texture.width();
            let height = self.texture.height();
            self.sample_count = sample_count;
            self.create_buffer(context, sample_count, width, height);
            return true;
        }

        false
    }

    pub fn sample_count(&self) -> u32
    {
        self.sample_count
    }

    fn create_buffer(&mut self, context: &Context, sample_count: u32, width: u32, height: u32)
    {
        self.texture = context.device.create_texture(&wgpu::TextureDescriptor
        { 
            label: Some("Texture"), 
            size: wgpu::Extent3d {width, height, depth_or_array_layers: 1}, 
            mip_level_count: 1, 
            sample_count, 
            dimension: wgpu::TextureDimension::D2, 
            format: context.config.format, 
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, 
            view_formats: &context.config.view_formats,
        });
    }
}

impl From<&Framebuffer> for wgpu::TextureView {

    fn from(value: &Framebuffer) -> Self {
        value.texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}

#[repr(C)]
#[derive(wgpu_macros::VertexLayout, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2D {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct Texture2D {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture2D {

    pub fn new(context: &Context, name: Option<&str>, bytes: &[u8], format: image::ImageFormat) -> Result<Texture2D, &'static str>
    {
        if let Ok(image) = image::load_from_memory_with_format(bytes, format)
        {
            //Potentially also support other color formats e.g. rgba16 (Need to do more research on this)
            let rgba = image.to_rgba8();
            let dim = rgba.dimensions();

            let extend = wgpu::Extent3d {
                width: dim.0,
                height: dim.1,
                depth_or_array_layers: 1,
            };

            let texture = context.device.create_texture(
                &wgpu::TextureDescriptor {
                    label: name,
                    size: extend,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                }
            );

            context.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All
                },
                &rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dim.0), 
                    rows_per_image: Some(dim.1),
                },
                extend
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            Ok(Texture2D { texture, view, sampler })
        }
        else
        {
            log::error!("Failed to parse image {}. Did you choose a supported format?", name.unwrap_or("[UNKNOWN_IMAGE]"));
            Err("Failed to parse image from bytes.")
        }
    }

    pub fn texture(&self) -> &wgpu::Texture
    {
        &self.texture
    }

    pub fn view(&self) -> &wgpu::TextureView
    {
        &self.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler
    {
        &self.sampler
    }

}

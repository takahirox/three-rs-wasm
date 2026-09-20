pub(crate) fn texture(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::TextureView {
    let float32 = device
        .features()
        .contains(wgpu::Features::FLOAT32_FILTERABLE);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("r186 LTC tables"),
        size: wgpu::Extent3d {
            width: 64,
            height: 64,
            depth_or_array_layers: 2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: if float32 {
            wgpu::TextureFormat::Rgba32Float
        } else {
            wgpu::TextureFormat::Rgba16Float
        },
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        texture.as_image_copy(),
        if float32 {
            include_bytes!("shaders/ltc-r186-f32.bin").as_slice()
        } else {
            include_bytes!("shaders/ltc-r186.bin").as_slice()
        },
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(if float32 { 1024 } else { 512 }),
            rows_per_image: Some(64),
        },
        texture.size(),
    );
    texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    })
}

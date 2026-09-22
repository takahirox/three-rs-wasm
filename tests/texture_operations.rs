use three_rs_wasm::{
    material::{Filter, Texture},
    renderer::*,
    tsl::{self, *},
};
#[test]
fn explicit_mips_and_resident_region_copy_preserve_unwritten_texels() {
    pollster::block_on(async {
        let r = Renderer::new().await.unwrap();
        let level = |size, color: [u8; 4]| {
            let mut t = Texture::from_rgba(size, size, color.repeat((size * size) as usize), false)
                .unwrap();
            t.filter = Filter::Nearest;
            t.mipmap_filter = Some(Filter::Nearest);
            t
        };
        let levels = [
            level(4, [255, 0, 0, 255]),
            level(2, [0, 255, 0, 255]),
            level(1, [0, 0, 255, 255]),
        ];
        let target = RenderTarget::with_options(
            &r.device,
            4,
            4,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba8Unorm,
                depth_buffer: false,
                ..Default::default()
            },
        )
        .unwrap();
        let input = RenderTarget::new(&r.device, 4, 4).unwrap();
        let gpu = GpuTexture::from_rgba_mipmaps(&r, &levels).unwrap();
        for (i, color) in [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]]
            .into_iter()
            .enumerate()
        {
            let pass = effect_with_textures(
                &r,
                wgpu::TextureFormat::Rgba8Unorm,
                &tsl::Texture::External(0).sample_level(uv(), float(i as f32)),
                &[(&gpu.view, &gpu.sampler)],
            )
            .await
            .unwrap();
            pass.apply(&r, &input, None, &target).unwrap();
            assert!(
                r.read_rgba(&target)
                    .unwrap()
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .all(|p| *p == color)
            );
        }
        let source = GpuTexture::from_rgba_mipmaps(&r, &[level(2, [255, 255, 0, 255])]).unwrap();
        gpu.copy_region_from(&r, &source, [0, 0], [1, 2], [2, 2])
            .unwrap();
        let pass = effect_with_textures(
            &r,
            wgpu::TextureFormat::Rgba8Unorm,
            &tsl::Texture::External(0).sample_level(uv(), float(0.)),
            &[(&gpu.view, &gpu.sampler)],
        )
        .await
        .unwrap();
        pass.apply(&r, &input, None, &target).unwrap();
        let pixels = r.read_rgba(&target).unwrap();
        for y in 0..4 {
            for x in 0..4 {
                assert_eq!(
                    &pixels[(y * 4 + x) * 4..][..4],
                    if (1..3).contains(&x) && y >= 2 {
                        &[255, 255, 0, 255]
                    } else {
                        &[255, 0, 0, 255]
                    }
                );
            }
        }
        assert!(
            gpu.copy_region_from(&r, &source, [0, 0], [3, 2], [2, 2])
                .is_err()
        );
        assert!(
            gpu.copy_region_from(&r, &source, [u32::MAX, 0], [0, 0], [1, 1])
                .is_err()
        );
        assert!(
            gpu.copy_region_from(&r, &source, [0, 0], [0, 0], [0, 1])
                .is_err()
        );
        assert!(
            gpu.copy_region_from(&r, &gpu, [0, 0], [0, 0], [1, 1])
                .is_err()
        );
        let mut srgb = level(1, [255; 4]);
        srgb.srgb = true;
        let other = GpuTexture::from_rgba_mipmaps(&r, &[srgb]).unwrap();
        assert!(
            gpu.copy_region_from(&r, &other, [0, 0], [0, 0], [1, 1])
                .is_err()
        );
        assert!(GpuTexture::from_rgba_mipmaps(&r, &[]).is_err());
        assert!(
            GpuTexture::from_rgba_mipmaps(&r, &[levels[0].clone(), levels[2].clone()]).is_err()
        );
    });
}

//! Rust-owned browser application used by the web validation suite.
use crate::{
    Error, Result, camera::*, geometry::*, material::*, math::*, raycast::*, renderer::*, scene::*,
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::{JsCast, prelude::*};
#[cfg(feature = "afterimage_webgl")]
mod afterimage_webgl;
#[cfg(feature = "ammo")]
mod ammo;
#[cfg(feature = "animation_pointer")]
mod animation_pointer;
#[cfg(feature = "arcball")]
mod arcball;
#[cfg(feature = "ascii")]
mod ascii;
#[cfg(feature = "attractors")]
mod attractors;
#[cfg(feature = "audio_timing")]
mod audio_timing;
#[cfg(feature = "audio_visual")]
mod audio_visual;
#[cfg(feature = "backdrop_water")]
mod backdrop_water;
#[cfg(feature = "ballpool")]
mod ballpool;
#[cfg(feature = "batch_lod_bvh")]
mod batch_lod_bvh;
#[cfg(feature = "birds_gltf")]
mod birds_gltf;
#[cfg(feature = "buffer_particles")]
mod buffer_particles;
#[cfg(feature = "buffergeometry_lines")]
mod buffergeometry_lines;
#[cfg(feature = "caustics")]
mod caustics;
#[cfg(feature = "channels")]
mod channels;
#[cfg(feature = "clipping_stencil")]
mod clipping_stencil;
#[cfg(feature = "composer_passes")]
mod composer_passes;
#[cfg(feature = "compute_birds")]
mod compute_birds;
#[cfg(feature = "compute_cloth")]
mod compute_cloth;
#[cfg(feature = "compute_reduce")]
mod compute_reduce;
#[cfg(feature = "compute_water")]
mod compute_water;
#[cfg(feature = "controls_attributes")]
mod controls_attributes;
#[cfg(feature = "convex_hull")]
mod convex_hull;
#[cfg(feature = "csg_eval")]
mod csg_eval;
#[cfg(feature = "css2d_label")]
mod css2d_label;
#[cfg(feature = "css3d_common")]
mod css3d_common;
#[cfg(feature = "css3d_mixed")]
mod css3d_mixed;
#[cfg(feature = "css3d_molecules")]
mod css3d_molecules;
#[cfg(feature = "css3d_orthographic")]
mod css3d_orthographic;
#[cfg(feature = "css3d_periodictable")]
mod css3d_periodictable;
#[cfg(feature = "css3d_sandbox")]
mod css3d_sandbox;
#[cfg(feature = "css3d_sprites")]
mod css3d_sprites;
#[cfg(feature = "cubemap_dynamic")]
mod cubemap_dynamic;
#[cfg(feature = "cubemap_webgl")]
mod cubemap_webgl;
#[cfg(feature = "custom_fog")]
mod custom_fog;
#[cfg(feature = "dds")]
mod dds;
#[cfg(feature = "deferred")]
mod deferred;
#[cfg(feature = "dof2")]
mod dof2;
#[cfg(feature = "dof_webgl")]
mod dof_webgl;
#[cfg(feature = "draco_variants")]
mod draco_variants;
#[cfg(feature = "elements_text")]
mod elements_text;
#[cfg(feature = "environment_materials")]
mod environment_materials;
#[cfg(feature = "envmap_exr")]
mod envmap_exr;
#[cfg(feature = "envmaps_fasthdr")]
mod envmaps_fasthdr;
#[cfg(feature = "equirectangular")]
mod equirectangular;
mod example;
mod expanded;
#[cfg(feature = "expanded_geometry_colors")]
mod expanded_geometry_colors;
#[cfg(feature = "expanded_indexed")]
mod expanded_indexed;
#[cfg(feature = "expanded_lines")]
mod expanded_lines;
#[cfg(feature = "expanded_morph_models")]
mod expanded_morph_models;
#[cfg(feature = "expanded_triangles")]
mod expanded_triangles;
#[cfg(feature = "exporter_gcode")]
mod exporter_gcode;
#[cfg(feature = "exporter_gltf")]
mod exporter_gltf;
#[cfg(feature = "exporters_matcap")]
mod exporters_matcap;
#[cfg(feature = "exporters_video")]
mod exporters_video;
#[cfg(feature = "fbx_loader")]
mod fbx_loader;
mod fbx_models;
#[cfg(feature = "fbx_nurbs")]
mod fbx_nurbs;
#[cfg(feature = "fluid")]
mod fluid;
#[cfg(feature = "fog_scattering")]
mod fog_scattering;
#[cfg(feature = "fog_volume")]
mod fog_volume;
#[cfg(feature = "fxaa_webgl")]
mod fxaa_webgl;
#[cfg(feature = "gallery_random")]
mod gallery_random;
#[cfg(feature = "games_fps")]
mod games_fps;
#[cfg(feature = "gaussian_splat")]
mod gaussian_splat;
#[cfg(feature = "generator_building")]
mod generator_building;
#[cfg(feature = "generator_city")]
mod generator_city;
#[cfg(feature = "geometries")]
mod geometries;
#[cfg(feature = "geometry_csg")]
mod geometry_csg;
#[cfg(feature = "geometry_cube")]
mod geometry_cube;
#[cfg(feature = "geometry_loft")]
mod geometry_loft;
#[cfg(feature = "geometry_materials")]
mod geometry_materials;
#[cfg(feature = "glitch")]
mod glitch;
#[cfg(feature = "gltf_avif")]
mod gltf_avif;
#[cfg(feature = "gltf_examples")]
mod gltf_examples;
#[cfg(feature = "gltf_instancing")]
mod gltf_instancing;
#[cfg(feature = "gltf_normals")]
mod gltf_normals;
mod gltf_viewer;
#[cfg(feature = "godrays")]
mod godrays;
#[cfg(feature = "gpgpu_protoplanet")]
mod gpgpu_protoplanet;
#[cfg(feature = "gtao")]
mod gtao;
#[cfg(feature = "gtao_webgl")]
mod gtao_webgl;
#[cfg(feature = "hdr")]
mod hdr;
#[cfg(feature = "helpers_formats")]
mod helpers_formats;
#[cfg(feature = "html_texture")]
mod html_texture;
#[cfg(feature = "ies_spotlight")]
mod ies_spotlight;
#[cfg(feature = "ifc_loader")]
mod ifc_loader;
#[cfg(feature = "instancing_morph")]
mod instancing_morph;
#[cfg(feature = "interactive_objects")]
mod interactive_objects;
#[cfg(feature = "interactive_scenes")]
mod interactive_scenes;
#[cfg(feature = "interactive_shaders")]
mod interactive_shaders;
#[cfg(feature = "jolt")]
mod jolt;
#[cfg(feature = "kinect")]
mod kinect;
#[cfg(feature = "ktx")]
mod ktx;
#[cfg(feature = "ktx2")]
mod ktx2;
#[cfg(feature = "ldraw")]
mod ldraw;
#[cfg(feature = "ldraw_loader")]
mod ldraw_loader;
#[cfg(feature = "lightprobes")]
mod lightprobes;
#[cfg(feature = "lightprobes_sponza")]
mod lightprobes_sponza;
#[cfg(feature = "lights_clustered")]
mod lights_clustered;
#[cfg(feature = "lights_dynamic")]
mod lights_dynamic;
#[cfg(feature = "lights_probes")]
mod lights_probes;
#[cfg(feature = "lights_projector")]
mod lights_projector;
#[cfg(feature = "lines_indexed")]
mod lines_indexed;
#[cfg(feature = "lines_raycast")]
mod lines_raycast;
#[cfg(feature = "linked_particles")]
mod linked_particles;
#[cfg(feature = "loader_3dm")]
mod loader_3dm;
#[cfg(feature = "loader_gltf")]
mod loader_gltf;
#[cfg(feature = "loader_materialx")]
mod loader_materialx;
#[cfg(feature = "log_depth")]
mod log_depth;
#[cfg(feature = "lottie")]
mod lottie;
#[cfg(feature = "lut_webgl")]
mod lut_webgl;
#[cfg(feature = "marching_cubes")]
mod marching_cubes;
#[cfg(feature = "marching_tables")]
mod marching_tables;
#[cfg(feature = "material_textures")]
mod material_textures;
#[cfg(feature = "md2")]
mod md2;
#[cfg(feature = "md2_control")]
mod md2_control;
#[cfg(feature = "memory_outline")]
mod memory_outline;
#[cfg(feature = "mesh_batch")]
mod mesh_batch;
#[cfg(feature = "batch_lod_bvh")]
mod meshopt;
#[cfg(feature = "mirror_webgl")]
mod mirror_webgl;
#[cfg(feature = "models_modifiers")]
mod models_modifiers;
#[cfg(feature = "morphtargets")]
mod morphtargets;
#[cfg(feature = "morphtargets_webcam")]
mod morphtargets_webcam;
#[cfg(feature = "msaa_renderbuffers")]
mod msaa_renderbuffers;
#[cfg(feature = "mvp")]
mod mvp;
#[cfg(feature = "offscreen")]
mod offscreen;
#[cfg(feature = "outline")]
mod outline;
#[cfg(feature = "passes_decals")]
mod passes_decals;
#[cfg(feature = "periodic_table")]
mod periodic_table;
#[cfg(feature = "physics_ammo_break")]
mod physics_ammo_break;
#[cfg(feature = "physics_ammo_cloth")]
mod physics_ammo_cloth;
#[cfg(feature = "physics_ammo_instancing")]
mod physics_ammo_instancing;
#[cfg(feature = "physics_ammo_rope")]
mod physics_ammo_rope;
#[cfg(feature = "physics_ammo_terrain")]
mod physics_ammo_terrain;
#[cfg(feature = "physics_ammo_volume")]
mod physics_ammo_volume;
#[cfg(feature = "physics_jolt_instancing")]
mod physics_jolt_instancing;
#[cfg(feature = "physics_rapier_basic")]
mod physics_rapier_basic;
#[cfg(feature = "physics_rapier_character_controller")]
mod physics_rapier_character_controller;
#[cfg(feature = "physics_rapier_instancing")]
mod physics_rapier_instancing;
#[cfg(feature = "physics_rapier_joints")]
mod physics_rapier_joints;
#[cfg(feature = "physics_rapier_terrain")]
mod physics_rapier_terrain;
#[cfg(feature = "physics_rapier_vehicle_controller")]
mod physics_rapier_vehicle_controller;
#[cfg(feature = "picking")]
mod picking;
#[cfg(feature = "picking_buffers")]
mod picking_buffers;
#[cfg(feature = "pmrem")]
mod pmrem;
#[cfg(feature = "pmrem_cube_uv")]
mod pmrem_cube_uv;
#[cfg(feature = "pmrem_equirectangular")]
mod pmrem_equirectangular;
#[cfg(feature = "pmrem_test")]
mod pmrem_test;
#[cfg(feature = "point_clouds")]
mod point_clouds;
#[cfg(feature = "point_lights")]
mod point_lights;
#[cfg(feature = "points_dynamic")]
mod points_dynamic;
#[cfg(feature = "portal_webgl")]
mod portal_webgl;
#[cfg(feature = "postprocessing_advanced")]
mod postprocessing_advanced;
#[cfg(feature = "probes_hdr")]
mod probes_hdr;
#[cfg(feature = "progressive_lod")]
mod progressive_lod;
#[cfg(feature = "pvr")]
mod pvr;
#[cfg(feature = "pvrtc")]
mod pvrtc;
#[cfg(feature = "random_uv")]
mod random_uv;
#[cfg(feature = "rapier_common")]
mod rapier_common;
#[cfg(feature = "raycaster_bvh")]
mod raycaster_bvh;
#[cfg(feature = "raycaster_helper")]
mod raycaster_helper;
#[cfg(feature = "refraction_loaders")]
mod refraction_loaders;
#[cfg(feature = "refraction_webgl")]
mod refraction_webgl;
#[cfg(feature = "render_bundle")]
mod render_bundle;
#[cfg(feature = "retargeting")]
mod retargeting;
#[cfg(feature = "retargeting_readyplayer")]
mod retargeting_readyplayer;
#[cfg(feature = "retro")]
mod retro;
#[cfg(feature = "reversed_depth")]
mod reversed_depth;
#[cfg(feature = "loader_3dm")]
mod rhino3dm_loader;
#[cfg(feature = "robot")]
mod robot;
#[cfg(feature = "roller_coaster")]
mod roller_coaster;
#[cfg(feature = "room_environment")]
mod room_environment;
#[cfg(feature = "sao")]
mod sao;
#[cfg(feature = "sculpt")]
mod sculpt;
#[cfg(feature = "selection_views")]
mod selection_views;
#[cfg(feature = "shader_geometry")]
mod shader_geometry;
#[cfg(feature = "shadow_array")]
mod shadow_array;
#[cfg(feature = "shadow_rtt")]
mod shadow_rtt;
#[cfg(feature = "shadowmap_csm")]
mod shadowmap_csm;
#[cfg(feature = "shadowmap_opacity")]
mod shadowmap_opacity;
#[cfg(feature = "shadowmap_pcss")]
mod shadowmap_pcss;
#[cfg(feature = "shadowmap_performance")]
mod shadowmap_performance;
#[cfg(feature = "shadowmap_progressive")]
mod shadowmap_progressive;
#[cfg(feature = "shadowmap_viewer")]
mod shadowmap_viewer;
#[cfg(feature = "shadowmap_vsm")]
mod shadowmap_vsm;
#[cfg(feature = "shadowmap_webgl")]
mod shadowmap_webgl;
#[cfg(feature = "shapes")]
mod shapes;
#[cfg(feature = "shapes_lights")]
mod shapes_lights;
#[cfg(feature = "simple_gi")]
mod simple_gi;
#[cfg(feature = "skinning_instances")]
mod skinning_instances;
#[cfg(feature = "sky_water")]
mod sky_water;
#[cfg(feature = "sobel_webgl")]
mod sobel_webgl;
#[cfg(feature = "sort_bitonic")]
mod sort_bitonic;
#[cfg(feature = "spline_editor")]
mod spline_editor;
#[cfg(feature = "spot_skinning")]
mod spot_skinning;
#[cfg(feature = "ssao")]
mod ssao;
#[cfg(feature = "ssgi")]
mod ssgi;
#[cfg(feature = "ssr")]
mod ssr;
#[cfg(feature = "ssr_denoise")]
mod ssr_denoise;
#[cfg(feature = "ssr_webgl")]
mod ssr_webgl;
#[cfg(feature = "sss")]
mod sss;
#[cfg(feature = "stereo_loaders")]
mod stereo_loaders;
#[cfg(feature = "subdivision")]
mod subdivision;
#[cfg(feature = "subsurface")]
mod subsurface;
#[cfg(feature = "svg_lines")]
mod svg_lines;
#[cfg(feature = "svg_loader")]
mod svg_loader;
#[cfg(feature = "svg_sandbox")]
mod svg_sandbox;
#[cfg(feature = "taa")]
mod taa;
#[cfg(feature = "teapot_data")]
mod teapot_data;
#[cfg(feature = "terrain_loaders")]
mod terrain_loaders;
#[cfg(feature = "test_memory")]
mod test_memory;
#[cfg(feature = "test_memory2")]
mod test_memory2;
#[cfg(feature = "text_clipping")]
mod text_clipping;
#[cfg(feature = "text_shapes")]
mod text_shapes;
#[cfg(feature = "texture_flares")]
mod texture_flares;
#[cfg(feature = "texture_rotation")]
mod texture_rotation;
#[cfg(feature = "texture_volumes")]
mod texture_volumes;
#[cfg(feature = "three_mixer")]
mod three_mixer;
#[cfg(feature = "threemf")]
mod threemf;
#[cfg(feature = "tiles3d")]
mod tiles3d;
#[cfg(feature = "trackball_sprites")]
mod trackball_sprites;
#[cfg(feature = "transform_controls")]
mod transform_controls;
#[cfg(feature = "transform_curves")]
mod transform_curves;
#[cfg(feature = "transmission_alpha")]
mod transmission_alpha;
#[cfg(feature = "tsl_compute")]
mod tsl_compute;
#[cfg(feature = "tsl_environment")]
mod tsl_environment;
#[cfg(feature = "tsl_examples")]
mod tsl_examples;
#[cfg(feature = "tsl_extended")]
mod tsl_extended;
#[cfg(feature = "tsl_filters")]
mod tsl_filters;
#[cfg(feature = "tsl_graph")]
mod tsl_graph;
#[cfg(feature = "tsl_lighting")]
mod tsl_lighting;
#[cfg(feature = "tsl_materials")]
mod tsl_materials;
#[cfg(feature = "tsl_next")]
mod tsl_next;
#[cfg(feature = "tsl_particles")]
mod tsl_particles;
#[cfg(feature = "tsl_passes")]
mod tsl_passes;
#[cfg(feature = "tsl_primitives")]
mod tsl_primitives;
#[cfg(feature = "tsl_procedural")]
mod tsl_procedural;
#[cfg(feature = "tsl_surface")]
mod tsl_surface;
#[cfg(feature = "tsl_viewport")]
mod tsl_viewport;
mod ultra_hdr;
#[cfg(feature = "usdz")]
mod usdz;
#[cfg(feature = "usdz_exporter")]
mod usdz_exporter;
#[cfg(feature = "uv_tests")]
mod uv_tests;
#[cfg(feature = "video_frame")]
mod video_frame;
#[cfg(feature = "views_loaders")]
mod views_loaders;
#[cfg(feature = "volume_caustics")]
mod volume_caustics;
#[cfg(feature = "volume_fire")]
mod volume_fire;
#[cfg(feature = "volume_instancing")]
mod volume_instancing;
#[cfg(feature = "volume_lighting")]
mod volume_lighting;
#[cfg(feature = "volume_rectarea")]
mod volume_rectarea;
#[cfg(feature = "volume_traa")]
mod volume_traa;
#[cfg(feature = "vrml")]
mod vrml;
#[cfg(feature = "vxgi")]
mod vxgi;
#[cfg(feature = "vxgi_sponza")]
mod vxgi_sponza;
#[cfg(feature = "watch")]
mod watch;
#[cfg(feature = "water")]
mod water;
#[cfg(feature = "webcam")]
mod webcam;
#[cfg(feature = "wgsl_bind")]
mod wgsl_bind;
#[cfg(feature = "wide_gamut")]
mod wide_gamut;
#[cfg(feature = "xr_app")]
mod xr_app;
#[cfg(feature = "xr_controller_model")]
mod xr_controller_model;
#[cfg(feature = "xr_random")]
mod xr_random;
#[cfg(feature = "example_xr_app")]
pub use xr_app::XrApp;

// Demo assets live under web/ both locally and below a static hosting prefix.
/// Calls `globalThis[ name ]()` when the page defines it.
fn page_hook(name: &str) {
    if let Ok(f) = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str(name))
        && let Some(f) = f.dyn_ref::<js_sys::Function>()
    {
        let _ = f.call0(&JsValue::NULL);
    }
}

type AnimationCallback = Closure<dyn FnMut(f64)>;
struct State {
    timer: crate::time::Timer,
    renderer: Renderer,
    surface: wgpu::Surface<'static>,
    configuration: wgpu::SurfaceConfiguration,
    target: RenderTarget,
    scene: Scene,
    camera: Object3D,
    canvas: web_sys::HtmlCanvasElement,
    frame: u32,
    animation: Option<AnimationCallback>,
    request: Option<i32>,
    paused: bool,
    options: PageOptions,
    load_generation: u64,
    /// The example the frame loop drives.
    demo: expanded::Demo,
    /// The last frame presented a gallery scene's own output target, which
    /// may read the main target.
    presented_output: bool,
}
/// navigator.gpu.getPreferredCanvasFormat(): the format of the canvas and of
/// the 8-bit targets that render into it.
pub(crate) fn canvas_format() -> wgpu::TextureFormat {
    let preferred = (|| {
        let gpu = js_sys::Reflect::get(&web_sys::window()?.navigator(), &"gpu".into()).ok()?;
        let method = js_sys::Reflect::get(&gpu, &"getPreferredCanvasFormat".into()).ok()?;
        js_sys::Function::from(method).call0(&gpu).ok()?.as_string()
    })();
    if preferred.as_deref() == Some("bgra8unorm") {
        wgpu::TextureFormat::Bgra8Unorm
    } else {
        wgpu::TextureFormat::Rgba8Unorm
    }
}
/// The target the scene renders into.
#[derive(Clone, Copy, Debug, PartialEq)]
enum TargetFormat {
    /// The canvas's format: display values.
    Canvas,
    /// The canvas format's sRGB view: linear 8-bit values the view encodes.
    CanvasSrgb,
    /// Linear half-float, tone-mapped and encoded by the output pass.
    Half,
}
/// How an example's page renders, as its code sets the renderer up
/// ( `WebGPURenderer( { antialias, alpha, stencil } )`, its output buffer
/// type and output pass, `setAnimationLoop` ).
#[derive(Clone, Copy, Debug)]
struct PageOptions {
    /// The canvas composites premultiplied alpha ( alpha: true ).
    premultiplied_canvas: bool,
    /// The main target's MSAA samples ( antialias ), 0 for none.
    samples: u32,
    /// Tone mapping and sRGB encoding happen in the shader.
    encode_srgb: bool,
    format: TargetFormat,
    stencil: bool,
    /// The target holds display values: raw shader output or output encoded
    /// in the shader.
    raw_output: bool,
    /// Linear, premultiplied output, encoded on output.
    premultiplied_output: bool,
    /// The scene renders only on load, input and resize.
    on_demand: bool,
    /// Pausing stops the animation loop.
    stop_when_paused: bool,
    /// The example presents into canvases of its own ( UVsDebug ).
    offscreen_output: bool,
    /// Pointer input picks in the example's render-target view.
    rtt: bool,
    /// The page's sheen slider drives the scene's materials.
    sheen_controls: bool,
}
impl PageOptions {
    /// A gallery example's defaults: MSAA, a half-float target tone-mapped and
    /// encoded by the output pass, an animation loop that pausing stops.
    const GALLERY: Self = Self {
        premultiplied_canvas: false,
        samples: 4,
        encode_srgb: false,
        format: TargetFormat::Half,
        stencil: false,
        raw_output: false,
        premultiplied_output: false,
        on_demand: false,
        stop_when_paused: true,
        offscreen_output: false,
        rtt: false,
        sheen_controls: false,
    };
}
impl State {
    fn request_render(&mut self) {
        if self.request.is_none()
            && let Some(callback) = &self.animation
        {
            self.request = web_sys::window().and_then(|w| {
                w.request_animation_frame(callback.as_ref().unchecked_ref())
                    .ok()
            });
        }
    }
    fn render(&mut self) -> Result<()> {
        self.timer.update();
        let _ = self
            .canvas
            .set_attribute("data-delta", &self.timer.get_delta().to_string());
        let width = self.canvas.width();
        let height = self.canvas.height();
        // Scene loaders can replace the camera without resizing the render target.
        // Synchronize its projection on the first frame as well as after resizes.
        if let NodeKind::Camera(Camera::Perspective(camera)) =
            &mut self.scene.get_mut(self.camera)?.kind
        {
            camera.aspect = width as f64 / height as f64;
        }
        if width != self.target.width || height != self.target.height {
            self.target.set_size(&self.renderer.device, width, height)?;
            self.configuration.width = width;
            self.configuration.height = height;
            self.surface
                .configure(&self.renderer.device, &self.configuration);
        }
        self.demo.update(
            &mut self.scene,
            self.camera,
            self.timer.get_delta().min(0.1),
            !self.paused,
        )?;
        self.demo.prepare(
            &self.renderer,
            &mut self.scene,
            self.camera,
            width as f64 / height as f64,
        )?;
        let format = if self.options.raw_output {
            self.configuration.format
        } else {
            self.configuration.format.add_srgb_suffix()
        };
        let premultiplied = self.options.premultiplied_output;
        // A target holds display values when the output pass would only copy
        // it: no tone mapping or encoding left, in the canvas view's format.
        let displays = |target: &RenderTarget, scene_tone: ToneMapping| {
            !premultiplied
                && (target.options.encode_srgb || scene_tone == ToneMapping::None)
                && target.options.format == format
        };
        let scene_tone = self.scene.output_tone_mapping();
        // Like three, render straight into the canvas when no output pass is
        // needed: the main target, or the example's own output target ( which
        // the last frame presented ), renders into the canvas's texture.
        let main = !self.options.offscreen_output
            && !self.presented_output
            && displays(&self.target, scene_tone);
        let output = !self.options.offscreen_output
            && self.presented_output
            && self
                .demo
                .output_target_mut()
                .is_some_and(|t| displays(t, scene_tone));
        let frame = if main || output {
            Some(
                self.surface
                    .get_current_texture()
                    .map_err(|e| Error::Gpu(e.to_string()))?,
            )
        } else {
            None
        };
        // A canvas texture a target cannot render into falls back to the
        // output pass.
        let main_alias = frame
            .as_ref()
            .filter(|_| main)
            .and_then(|frame| self.target.alias_color(&frame.texture).ok());
        let output_alias = match &frame {
            Some(frame) if output => self
                .demo
                .output_target_mut()
                .and_then(|t| t.alias_color(&frame.texture).ok()),
            _ => None,
        };
        self.renderer
            .set_output_transform(self.scene.exposure, scene_tone);
        let rendered = (|| -> Result<()> {
            let handled =
                self.demo
                    .render(&self.renderer, &mut self.scene, self.camera, &self.target)?;
            if !handled {
                self.renderer
                    .render(&mut self.scene, self.camera, &self.target)?;
            }
            Ok(())
        })();
        let presentation_texture = self
            .demo
            .output_target()
            .unwrap_or(&self.target)
            .texture
            .clone();
        // Rendered into the canvas: the presented target still renders into its
        // texture ( an example may have replaced its output target ).
        let direct = frame
            .as_ref()
            .is_some_and(|frame| presentation_texture == frame.texture);
        if let Some(alias) = main_alias {
            self.target.restore_color(alias);
        }
        if let (Some(alias), Some(frame)) = (output_alias, &frame)
            && let Some(target) = self.demo.output_target_mut()
            && target.texture == frame.texture
        {
            target.restore_color(alias);
        }
        rendered?;
        if self.options.offscreen_output {
            self.frame += 1;
            let _ = self
                .canvas
                .set_attribute("data-frames", &self.frame.to_string());
            return Ok(());
        }
        let frame = match frame {
            Some(frame) => frame,
            None => self
                .surface
                .get_current_texture()
                .map_err(|e| Error::Gpu(e.to_string()))?,
        };
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(format),
            ..Default::default()
        });
        let presentation = self.demo.output_target().unwrap_or(&self.target);
        let presents_main = std::ptr::eq(presentation, &self.target);
        if direct {
            // Rendered into the canvas.
        } else if premultiplied {
            self.renderer.blit_premultiplied_srgb(
                presentation,
                &view,
                format,
                self.scene.exposure,
                self.scene.output_tone_mapping(),
            );
        } else {
            self.renderer.blit_with_tone_mapping(
                presentation,
                &view,
                format,
                self.scene.exposure,
                if presentation.options.encode_srgb {
                    ToneMapping::None
                } else {
                    self.scene.output_tone_mapping()
                },
            );
        }
        self.presented_output = !presents_main;
        // How the frame reached the canvas, for the presentation audit
        // ( docs/performance-parity.md ).
        let path = format!(
            "{} {:?} {:?}",
            match (direct, presents_main) {
                (true, true) => "direct",
                (true, false) => "direct-output",
                (false, true) => "target",
                (false, false) => "output",
            },
            presentation.options.format,
            self.scene.output_tone_mapping()
        );
        if self.canvas.get_attribute("data-present").as_deref() != Some(path.as_str()) {
            let _ = self.canvas.set_attribute("data-present", &path);
        }
        frame.present();
        self.demo.presented(&self.canvas)?;
        self.frame += 1;
        let _ = self
            .canvas
            .set_attribute("data-frames", &self.frame.to_string());
        Ok(())
    }
}

/// webgpu_tsl_transpiler: `new Transpiler( new GLSLDecoder(), encoder ).parse(
/// glsl )` with the TSL or WGSL encoder; it throws the original's error message.
#[cfg(feature = "example_tsl_transpiler")]
#[wasm_bindgen]
pub fn tsl_transpile(glsl: &str, encoder: &str) -> std::result::Result<String, JsValue> {
    let encoder = match encoder {
        "TSL" => crate::transpiler::Encoder::Tsl,
        "WGSL" => crate::transpiler::Encoder::Wgsl,
        other => return Err(JsValue::from_str(&format!("Unknown encoder: {other}"))),
    };
    crate::transpiler::transpile(glsl, encoder).map_err(|e| JsValue::from_str(&e))
}

/// The run-time UltraHDR (or Radiance HDR) reconstruction as RGBA half-float
/// bits, compared against the original UltraHDRLoader's output.
#[wasm_bindgen]
pub async fn decode_environment(bytes: Vec<u8>) -> std::result::Result<Vec<u16>, JsValue> {
    let image = ultra_hdr::decode_environment(&bytes)
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(image.rgba.iter().map(|v| v.to_bits()).collect())
}

/// The FBXLoader port's parse in canonical form ( `fbx::canonical` ), compared
/// with the original loader's in tests/browser/fbx-loader.spec.js.
#[cfg(feature = "example_fbx_loader")]
#[wasm_bindgen]
pub fn fbx_canonical(
    bytes: Vec<u8>,
    path: String,
    trim: bool,
    width: f64,
    height: f64,
) -> std::result::Result<String, JsValue> {
    let options = crate::fbx::Options {
        trim_animation_clips: trim,
        path,
        window_size: (width, height),
    };
    crate::fbx::parse(&bytes, &options)
        .map(|s| crate::fbx::canonical(&s))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// The VRMLLoader port's parse in canonical form ( `vrml::canonical` ),
/// compared with the original loader's in tests/browser/vrml-loader.spec.js.
#[cfg(feature = "example_vrml")]
#[wasm_bindgen]
pub fn vrml_canonical(
    text: String,
    path: String,
    width: f64,
    height: f64,
) -> std::result::Result<String, JsValue> {
    let options = crate::vrml::Options {
        path,
        window_size: (width, height),
    };
    crate::vrml::parse(&text, &options)
        .map(|s| crate::vrml::canonical(&s, false))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// The USDLoader port's parse in canonical form ( `usd::canonical` ),
/// compared with the original loader's in tests/browser/usd-loader.spec.js.
#[cfg(feature = "example_usdz")]
#[wasm_bindgen]
pub fn usd_canonical(bytes: Vec<u8>, path: String) -> std::result::Result<String, JsValue> {
    crate::usd::parse(&bytes, &path)
        .map(|s| crate::usd::canonical(&s, false))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// The Rhino3dmLoader port's object tree for a 3DM file in canonical form,
/// compared with the original loader's in tests/browser/rhino3dm-loader.spec.js.
#[cfg(feature = "example_loader_3dm")]
#[wasm_bindgen]
pub async fn rhino3dm_canonical(bytes: Vec<u8>) -> std::result::Result<String, JsValue> {
    let run = async {
        let rhino = rhino3dm_loader::library().await?;
        let decoded = rhino3dm_loader::decode_objects(&rhino, &bytes, 3.0)?;
        let tree = rhino3dm_loader::create_geometry(&decoded)?;
        Ok::<_, crate::Error>(rhino3dm_loader::canonical(&tree))
    };
    run.await.map_err(|e| JsValue::from_str(&e.to_string()))
}

/// The JavaScript bootstrap owns one app lifetime. Scene state, rendering,
/// animation and input handling all stay in Rust.
#[wasm_bindgen]
pub struct BrowserApp {
    state: Rc<RefCell<State>>,
    pointer: Closure<dyn FnMut(web_sys::PointerEvent)>,
}
#[wasm_bindgen]
impl BrowserApp {
    /// Request a frame after a canvas resize, including on-demand examples.
    pub fn request_render(&self) {
        self.state.borrow_mut().request_render();
    }
    /// Configure MSAA for matched-quality renderer comparisons (default unchanged).
    pub fn set_samples(&self, samples: u32) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let mut options = state.target.options.clone();
        options.samples = samples;
        state.target = RenderTarget::with_options(
            &state.renderer.device,
            state.canvas.width(),
            state.canvas.height(),
            options,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.target.set_canvas(true);
        state.request_render();
        Ok(())
    }
    /// Presentation controls; all scene and deformation state remains in Rust.
    pub fn point_lights_controls(&self, paused: bool, amount: f64, speed: f64) {
        let mut state = self.state.borrow_mut();
        state.paused = paused;
        state.demo.content.on_point_lights_controls(amount, speed);
    }
    pub async fn load_model(&self, example: u32) -> std::result::Result<bool, JsValue> {
        let (generation, load) = {
            let mut state = self.state.borrow_mut();
            state.load_generation += 1;
            let environment = state.scene.environment.clone();
            (
                state.load_generation,
                state.demo.content.on_load_model(example, environment),
            )
        };
        let Some(load) = load else {
            return Err(JsValue::from_str("glTF viewer required"));
        };
        let loaded = load.await;
        let mut state = self.state.borrow_mut();
        if state.load_generation != generation {
            return Ok(false);
        }
        let (mut scene, camera, content) = loaded.map_err(|e| JsValue::from_str(&e.to_string()))?;
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene
            .get_mut(camera)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .kind
        {
            c.aspect = state.canvas.width() as f64 / state.canvas.height() as f64;
        }
        scene.environment_intensity = state.scene.environment_intensity;
        scene.background_environment = state.scene.background_environment;
        scene.exposure = state.scene.exposure;
        scene.background_blur = state.scene.background_blur;
        scene.environment_rotation = state.scene.environment_rotation;
        state.scene = scene;
        state.camera = camera;
        state.demo.content = content;
        state.renderer.collect_resources();
        Ok(true)
    }
    pub async fn load_environment(&self, url: String) -> std::result::Result<bool, JsValue> {
        let generation = {
            let mut state = self.state.borrow_mut();
            state.load_generation += 1;
            state.load_generation
        };
        let image = async { ultra_hdr::environment(&url).await }.await;
        let mut state = self.state.borrow_mut();
        if state.load_generation != generation {
            return Ok(false);
        }
        let image = image.map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.scene.environment = Some(Arc::new(image));
        state.renderer.collect_resources();
        Ok(true)
    }
    pub fn gltf_controls(
        &self,
        exposure: f64,
        intensity: f64,
        rotation: f64,
        blur: f64,
        background: bool,
    ) {
        if ![exposure, intensity, rotation, blur]
            .iter()
            .all(|v| v.is_finite())
        {
            return;
        }
        let mut state = self.state.borrow_mut();
        state.scene.exposure = exposure.max(0.0);
        state.scene.environment_intensity = intensity.max(0.0);
        state.scene.environment_rotation = rotation;
        state.scene.background_blur = blur.clamp(0.0, 1.0);
        state.scene.background_environment = background;
    }
    /// `[filename, bytes]` of the last exporter download, or null.
    pub fn gallery_take_export(&self) -> JsValue {
        let mut state = self.state.borrow_mut();
        let state = &mut *state;
        if let Some((name, bytes)) = state.demo.take_export(&state.renderer) {
            let out = js_sys::Array::new();
            out.push(&JsValue::from_str(&name));
            out.push(&js_sys::Uint8Array::from(bytes.as_slice()));
            return out.into();
        }
        JsValue::NULL
    }
    pub fn gallery_status(&self) -> String {
        self.state.borrow().demo.status()
    }
    pub fn gallery_dragging(&self, value: bool) {
        self.state.borrow_mut().demo.dragging(value);
    }
    pub fn gallery_canvases(&self, canvases: js_sys::Array) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State { renderer, demo, .. } = &mut *state;
        demo.attach_canvases(renderer, canvases)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.request_render();
        Ok(())
    }
    pub fn gallery_viewport(
        &self,
        index: usize,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        state
            .demo
            .viewport(index, [x, y, width, height])
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.request_render();
        Ok(())
    }
    pub fn gallery_input(
        &self,
        dx: f64,
        dy: f64,
        wheel: f64,
        dragging: bool,
    ) -> std::result::Result<(), JsValue> {
        if ![dx, dy, wheel].iter().all(|v| v.is_finite()) {
            return Ok(());
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        let height = state.canvas.client_height().max(1) as f64;
        let State {
            scene,
            camera,
            demo,
            ..
        } = &mut *state;
        demo.content
            .on_scene_input(dx, dy, wheel, dragging)
            .unwrap_or_else(|| demo.input(scene, *camera, dx, dy, wheel, false, height))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn gallery_pan(&self, dx: f64, dy: f64) -> std::result::Result<(), JsValue> {
        if !dx.is_finite() || !dy.is_finite() {
            return Err(JsValue::from_str("invalid pan"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        let height = state.canvas.client_height().max(1) as f64;
        let State {
            scene,
            camera,
            demo,
            ..
        } = &mut *state;
        demo.input(scene, *camera, dx, dy, 0.0, true, height)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn gallery_select(&self, x: f64, y: f64) -> std::result::Result<(), JsValue> {
        if !x.is_finite() || !y.is_finite() {
            return Err(JsValue::from_str("pointer coordinates"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        let State {
            scene,
            camera,
            demo,
            ..
        } = &mut *state;
        demo.select(scene, *camera, x, y)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    /// Drawing-canvas pointer input: 0 down, 1 move, 2 up or leave (offset coordinates).
    pub fn gallery_draw(&self, kind: u32, x: f64, y: f64) -> std::result::Result<(), JsValue> {
        if !x.is_finite() || !y.is_finite() {
            return Err(JsValue::from_str("drawing coordinates"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        state
            .demo
            .draw(kind, x, y)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    /// Keyboard state for examples that track modifier keys (keyCode, pressed).
    pub fn gallery_key(&self, code: u32, down: bool) {
        self.state.borrow_mut().demo.key(code, down);
    }
    /// Comparison slider position in CSS pixels.
    pub fn gallery_slider(&self, x: f64) {
        if !x.is_finite() {
            return;
        }
        let mut state = self.state.borrow_mut();
        state.demo.slider(x);
        state.request_render();
    }
    pub fn gallery_pointer(&self, x: f64, y: f64) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        let mut state = self.state.borrow_mut();
        let width = state.canvas.client_width() as f64;
        let height = state.canvas.client_height() as f64;
        let is_rtt = state.options.rtt;
        let mut redraw = false;
        let State {
            renderer,
            scene,
            camera,
            demo,
            canvas,
            ..
        } = &mut *state;
        match demo.gpu_pointer(renderer, scene, *camera, x, y) {
            Ok(true) => redraw = true,
            Ok(false) => {}
            Err(e) => {
                let _ = canvas.set_attribute("data-error", &e.to_string());
            }
        }
        if is_rtt {
            demo.pointer(x, y);
        } else {
            demo.pointer(x * width / 2.0, -y * height / 2.0);
        }
        // On-demand scenes redraw when the pointer moves something.
        if redraw {
            state.request_render();
        }
    }
    /// Animation clips available in the active gallery scene.
    pub fn animation_names(&self) -> String {
        self.state
            .borrow()
            .demo
            .content
            .on_animation_names()
            .unwrap_or_else(|| "[]".into())
    }
    pub fn select_animation(&self, index: usize) -> std::result::Result<(), JsValue> {
        self.state
            .borrow_mut()
            .demo
            .content
            .on_select_animation(index)
            .unwrap_or(Ok(()))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    /// Freeze a selected clip for deterministic browser/reference comparisons.
    pub fn animation_time(&self, index: usize, time: f64) -> std::result::Result<(), JsValue> {
        if !time.is_finite() || time < 0.0 {
            return Err(JsValue::from_str("invalid animation time"));
        }
        let mut state = self.state.borrow_mut();
        state.paused = true;
        state
            .demo
            .content
            .on_animation_time(index, time)
            .unwrap_or(Ok(()))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    /// Serialize the live scene for debugging and upstream behavioral comparisons.
    pub fn transfer_counts(&self) -> String {
        serde_json::to_string(&self.state.borrow().renderer.transfer_counts())
            .expect("transfer counts")
    }
    pub fn gallery_wireframe(&self, enabled: bool) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State { scene, demo, .. } = &mut *state;
        demo.wireframe(scene, enabled)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn gallery_morph(&self, spherify: f64, twist: f64) -> std::result::Result<(), JsValue> {
        if ![spherify, twist]
            .iter()
            .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        {
            return Err(JsValue::from_str("invalid morph weights"));
        }
        let mut state = self.state.borrow_mut();
        let State { scene, demo, .. } = &mut *state;
        demo.morph(scene, [spherify, twist])
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    /// Freeze a procedural gallery scene at a reproducible elapsed time.
    pub fn gallery_time(&self, seconds: f64) -> std::result::Result<(), JsValue> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(JsValue::from_str("invalid gallery time"));
        }
        let mut state = self.state.borrow_mut();
        state.request_render();
        state.demo.seek(seconds);
        state.paused = true;
        Ok(())
    }
    /// webaudio_visualizer: the analyser's frequency bins for this frame.
    pub fn audio_data(&self, bytes: Vec<u8>) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        state
            .demo
            .audio_data(&bytes)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.request_render();
        Ok(())
    }
    /// webaudio_timing: the listener and sources' placement and pending plays.
    pub fn audio_frame(&self) -> std::result::Result<Vec<f32>, JsValue> {
        let mut state = self.state.borrow_mut();
        let State {
            scene,
            camera,
            demo,
            ..
        } = &mut *state;
        demo.audio_frame(scene, *camera)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    #[cfg(feature = "example_tsl_procedural")]
    pub fn audio_play(&self) -> std::result::Result<bool, JsValue> {
        let state = self.state.borrow();
        state
            .demo
            .audio()
            .and_then(|a| a.play(&state.renderer))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    #[cfg(feature = "example_tsl_procedural")]
    pub fn audio_result(&self) -> std::result::Result<Vec<f32>, JsValue> {
        let state = self.state.borrow();
        state
            .demo
            .audio()
            .and_then(|a| a.take().unwrap_or_else(|| Ok(Vec::new())))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    #[cfg(feature = "example_tsl_procedural")]
    pub fn audio_spectrum(&self, bytes: Vec<u8>) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        state
            .demo
            .audio()
            .and_then(|a| a.spectrum(&state.renderer, &bytes))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.request_render();
        Ok(())
    }
    pub fn tsl_parameter(&self, index: usize, value: f32) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        state
            .demo
            .tsl_parameter(index, value)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        state.request_render();
        Ok(())
    }
    pub fn gallery_sheen(&self, value: f64) -> std::result::Result<(), JsValue> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(JsValue::from_str("invalid sheen"));
        }
        let mut state = self.state.borrow_mut();
        if !state.options.sheen_controls {
            return Err(JsValue::from_str("Sheen example required"));
        }
        let roots = state.scene.roots().to_vec();
        for root in roots {
            for h in state
                .scene
                .traverse(root, true)
                .map_err(|e| JsValue::from_str(&e.to_string()))?
            {
                if let NodeKind::Mesh(mesh) = &mut state
                    .scene
                    .get_mut(h)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?
                    .kind
                {
                    for m in &mut mesh.materials {
                        if let Material::Physical(p) = Arc::make_mut(m) {
                            p.sheen = value;
                        }
                    }
                }
            }
        }
        state.request_render();
        Ok(())
    }
    pub fn scene_json(&self) -> std::result::Result<String, JsValue> {
        self.state
            .borrow()
            .scene
            .to_json()
            .map(|v| v.to_string())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn texture_transform(&self, values: Vec<f64>) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State { scene, demo, .. } = &mut *state;
        demo.content
            .on_texture_transform(scene, &values)
            .unwrap_or(Ok(()))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn background_intensity(&self, value: f64) {
        if value.is_finite() {
            self.state.borrow_mut().scene.background_intensity = value.max(0.0);
        }
    }
    pub fn gallery_pmrem(&self, enabled: bool) -> std::result::Result<(), JsValue> {
        let mut state = self.state.borrow_mut();
        let State { scene, demo, .. } = &mut *state;
        demo.content
            .on_pmrem(scene, enabled)
            .unwrap_or(Ok(()))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    pub fn resource_counts(&self) -> Vec<f64> {
        let (resident, uploads, filters) = self.state.borrow().renderer.resource_counts();
        vec![resident as f64, uploads as f64, filters as f64]
    }
    pub fn gltf_view(
        &self,
        yaw: f64,
        pitch: f64,
        distance: f64,
        exposure: f64,
        rotation: f64,
        blur: f64,
    ) {
        if ![yaw, pitch, distance, exposure, rotation, blur]
            .iter()
            .all(|v| v.is_finite())
            || distance <= 0.0
        {
            return;
        }
        let mut state = self.state.borrow_mut();
        state.demo.content.on_gltf_view(yaw, pitch, distance);
        state.scene.exposure = exposure.max(0.0);
        state.scene.environment_rotation = rotation;
        state.scene.background_blur = blur.clamp(0.0, 1.0);
        state.request_render();
    }
    pub fn orbit(&self, dx: f64, dy: f64, zoom: f64) {
        if dx.is_finite() && dy.is_finite() && zoom.is_finite() {
            self.state.borrow_mut().demo.content.on_orbit(dx, dy, zoom);
        }
    }
    /// Rebuild the example's object ( the MVP page's cube ), then the main
    /// target, paused.
    pub fn rebuild(&self) -> std::result::Result<(), JsValue> {
        let result = (|| -> Result<()> {
            let mut state = self.state.borrow_mut();
            let State { scene, demo, .. } = &mut *state;
            let rebuilds = demo
                .content
                .on_rebuild(scene)
                .ok_or(Error::Invalid("rebuild example required"))??;
            state.paused = true;
            state.target = RenderTarget::with_options(
                &state.renderer.device,
                state.canvas.width(),
                state.canvas.height(),
                state.target.options.clone(),
            )?;
            state.target.set_canvas(true);
            state
                .canvas
                .set_attribute("data-rebuilt", "true")
                .map_err(|e| Error::Asset(format!("{e:?}")))?;
            let _ = state
                .canvas
                .set_attribute("data-rebuilds", &rebuilds.to_string());
            Ok(())
        })();
        result.map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
/// Whether the ray through `ndc` hits `mesh` ( the MVP page's picking ).
fn pick(scene: &mut Scene, camera: Object3D, mesh: Object3D, ndc: Vector2) -> Result<bool> {
    let mut raycaster = Raycaster::default();
    scene.update()?;
    let (camera, world) = scene.camera(camera)?;
    raycaster.set_from_camera(ndc, camera, world)?;
    Ok(!raycaster.intersect_object(scene, mesh, false)?.is_empty())
}
/// The renderer, canvas surface, main target and initial scene an example's
/// program builds on, before its frame loop starts ( `Page::start` ), as a
/// three.js page creates its renderer and scene and then sets its animation
/// loop.
struct Page {
    renderer: Renderer,
    surface: wgpu::Surface<'static>,
    configuration: wgpu::SurfaceConfiguration,
    target: RenderTarget,
    scene: Scene,
    camera: Object3D,
    mesh: Object3D,
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
    options: PageOptions,
}
impl Page {
    async fn new(
        canvas: web_sys::HtmlCanvasElement,
        animate: bool,
        options: PageOptions,
    ) -> Result<Self> {
        let renderer = Renderer::new().await?;
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
            .map_err(|e| Error::Gpu(e.to_string()))?;
        let mut configuration = surface
            .get_default_config(&renderer.adapter, canvas.width(), canvas.height())
            .ok_or(Error::Gpu("surface configuration unavailable".into()))?;
        if options.premultiplied_canvas {
            configuration.alpha_mode = wgpu::CompositeAlphaMode::PreMultiplied;
        }
        configuration.view_formats = vec![configuration.format.add_srgb_suffix()];
        // The canvas texture serves as the main target's color when no
        // output pass is needed, so it takes the target's usages.
        configuration.usage = wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST;
        surface.configure(&renderer.device, &configuration);
        let mut target = RenderTarget::with_options(
            &renderer.device,
            canvas.width(),
            canvas.height(),
            RenderTargetOptions {
                samples: options.samples,
                encode_srgb: options.encode_srgb,
                format: match options.format {
                    TargetFormat::Canvas => configuration.format,
                    TargetFormat::CanvasSrgb => configuration.format.add_srgb_suffix(),
                    TargetFormat::Half => wgpu::TextureFormat::Rgba16Float,
                },
                // WebGLRenderer( { stencil: true } ) for ShadowMesh.
                stencil_buffer: options.stencil,
                ..Default::default()
            },
        )?;
        target.set_canvas(true);
        let mut scene = Scene::new();
        scene.background = Color::from_hex(0x102030);
        let camera = scene.insert(NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            aspect: canvas.width() as f64 / canvas.height() as f64,
            ..Default::default()
        })));
        scene.get_mut(camera)?.position.z = 5.0;
        let mut material = Material::default();
        material.properties_mut().color = Color::from_hex(0xee6633);
        let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(1.5, 1.5, 1.5)?),
            Arc::new(material),
        )));
        Ok(Self {
            renderer,
            surface,
            configuration,
            target,
            scene,
            camera,
            mesh,
            canvas,
            animate,
            options,
        })
    }
    /// A gallery scene's start ( three.js examples build theirs from an empty
    /// scene ): the placeholder cube removed, a black background; the scene,
    /// camera, renderer and camera aspect.
    fn gallery_scene(&mut self) -> Result<(&mut Scene, Object3D, &Renderer, f64)> {
        self.scene.dispose(self.mesh)?;
        self.scene.background = Color::BLACK;
        let aspect = match &self.scene.get(self.camera)?.kind {
            NodeKind::Camera(Camera::Perspective(p)) => p.aspect,
            _ => 1.0,
        };
        Ok((&mut self.scene, self.camera, &self.renderer, aspect))
    }
    /// Start the frame loop on `demo`.
    #[inline(never)]
    fn start(self, demo: expanded::Demo) -> Result<BrowserApp> {
        let Page {
            renderer,
            surface,
            configuration,
            target,
            scene,
            camera,
            canvas,
            animate,
            options,
            ..
        } = self;
        let mut timer = crate::time::Timer::default();
        timer.connect(
            web_sys::window()
                .and_then(|w| w.document())
                .ok_or(Error::Asset("document unavailable".into()))?,
        )?;
        let state = Rc::new(RefCell::new(State {
            timer,
            renderer,
            surface,
            configuration,
            target,
            scene,
            camera,
            canvas: canvas.clone(),
            frame: 0,
            animation: None,
            request: None,
            paused: !animate,
            options,
            load_generation: 0,
            demo,
            presented_output: true,
        }));
        let weak = Rc::downgrade(&state);
        let animation = Closure::wrap(Box::new(move |_time: f64| {
            if let Some(state) = weak.upgrade() {
                let mut state = state.borrow_mut();
                state.request = None;
                // The page's stats.js and Inspector hooks around the frame
                // ( web/examples/ui/ ), where the original calls them.
                page_hook("galleryFrameBegin");
                if let Err(error) = state.render() {
                    let _ = state.canvas.set_attribute("data-error", &error.to_string());
                    return;
                }
                page_hook("galleryFrameEnd");
                // These official static scenes render only on load, input and resize.
                if !(state.options.on_demand || state.paused && state.options.stop_when_paused) {
                    state.request_render();
                }
            }
        }) as Box<dyn FnMut(f64)>);
        state.borrow_mut().animation = Some(animation);
        let weak = Rc::downgrade(&state);
        let pointer = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
            if let Some(state) = weak.upgrade() {
                let mut state = state.borrow_mut();
                let ndc = Vector2::new(
                    event.offset_x() as f64 / state.canvas.client_width() as f64 * 2.0 - 1.0,
                    1.0 - event.offset_y() as f64 / state.canvas.client_height() as f64 * 2.0,
                );
                let State {
                    scene,
                    camera,
                    demo,
                    ..
                } = &mut *state;
                match demo.content.on_pick(scene, *camera, ndc) {
                    None => {}
                    Some(Ok(selected)) => {
                        state.paused = selected;
                        let _ = state.canvas.set_attribute(
                            "data-selected",
                            if selected { "true" } else { "false" },
                        );
                    }
                    Some(Err(error)) => {
                        let _ = state.canvas.set_attribute("data-error", &error.to_string());
                    }
                }
            }
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
        canvas
            .add_event_listener_with_callback("pointerdown", pointer.as_ref().unchecked_ref())
            .map_err(|e| Error::Asset(format!("{e:?}")))?;
        {
            let mut s = state.borrow_mut();
            s.request = Some(
                web_sys::window()
                    .ok_or(Error::Asset("window unavailable".into()))?
                    .request_animation_frame(
                        s.animation
                            .as_ref()
                            .expect("animation installed")
                            .as_ref()
                            .unchecked_ref(),
                    )
                    .map_err(|e| Error::Asset(format!("{e:?}")))?,
            );
        }
        Ok(BrowserApp { state, pointer })
    }
}
/// An example's program: its page set up with `options`, its scene built by
/// `build`, then its frame loop started, as a three.js example creates its
/// renderer and scene and sets its animation loop.
async fn program(
    canvas: web_sys::HtmlCanvasElement,
    animate: bool,
    options: PageOptions,
    build: impl AsyncFnOnce(&mut Page) -> Result<expanded::Demo>,
) -> std::result::Result<BrowserApp, JsValue> {
    async move {
        // Boxed and started out of line: every program shares one copy of the
        // page's setup rather than inlining it.
        let setup: std::pin::Pin<Box<dyn std::future::Future<Output = Result<Page>>>> =
            Box::pin(Page::new(canvas, animate, options));
        let mut page = setup.await?;
        let demo = build(&mut page).await?;
        page.start(demo)
    }
    .await
    .map_err(|e| JsValue::from_str(&e.to_string()))
}
impl Drop for BrowserApp {
    fn drop(&mut self) {
        let mut s = self.state.borrow_mut();
        if let Some(id) = s.request.take()
            && let Some(window) = web_sys::window()
        {
            let _ = window.cancel_animation_frame(id);
        }
        let _ = s.canvas.remove_event_listener_with_callback(
            "pointerdown",
            self.pointer.as_ref().unchecked_ref(),
        );
        s.animation = None;
        s.renderer.device.destroy();
    }
}

/// The ballpool's rigid-body world ( a port of @perplexdotgg/bounce ) for the
/// browser oracle, which steps it beside the original package and compares
/// every body's state bit for bit.
#[cfg(feature = "example_ballpool")]
#[wasm_bindgen]
pub struct BounceWorld {
    world: crate::bounce::World,
}

#[cfg(feature = "example_ballpool")]
impl Default for BounceWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "example_ballpool")]
#[wasm_bindgen]
impl BounceWorld {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BounceWorld {
        use crate::bounce::{Options, Vec3};
        BounceWorld {
            world: crate::bounce::World::new(Options {
                gravity: Vec3::new(0., -9.81, 0.),
                restitution: 0.4,
                friction: 0.5,
                solve_velocity_iterations: 6,
                solve_position_iterations: 2,
                linear_damping: 0.1,
                angular_damping: 0.1,
            }),
        }
    }
    #[wasm_bindgen(js_name = createStaticBox)]
    pub fn create_static_box(&mut self, size: &[f64], position: &[f64]) -> usize {
        let shape = self.world.create_box(size[0], size[1], size[2]);
        self.world.create_static_body(shape, vec3_of(position))
    }
    #[wasm_bindgen(js_name = createSphere)]
    pub fn create_sphere(&mut self, radius: f64) -> usize {
        self.world.create_sphere(radius)
    }
    #[wasm_bindgen(js_name = createDynamicBody)]
    pub fn create_dynamic_body(
        &mut self,
        shape: usize,
        position: &[f64],
        mass: f64,
        restitution: f64,
        friction: f64,
    ) -> usize {
        self.world
            .create_dynamic_body(shape, vec3_of(position), mass, restitution, friction)
    }
    pub fn respawn(&mut self, body: usize, position: &[f64]) {
        self.world.respawn(body, vec3_of(position));
    }
    #[wasm_bindgen(js_name = applyLinearImpulse)]
    pub fn apply_linear_impulse(&mut self, body: usize, impulse: &[f64]) {
        self.world.apply_linear_impulse(body, vec3_of(impulse));
    }
    /// advanceTime( step, time ) at `now` ( performance.now() / 1000 ).
    #[wasm_bindgen(js_name = advanceTime)]
    pub fn advance_time(&mut self, step: f64, time: f64, now: f64) {
        self.world.advance_time(step, time, now);
    }
    /// position, orientation, isSleeping, linearVelocity, angularVelocity.
    pub fn state(&self, body: usize) -> Vec<f64> {
        let b = self.world.body(body);
        let (p, q, v, w) = (
            b.position,
            b.orientation,
            b.linear_velocity,
            b.angular_velocity,
        );
        vec![
            p.x,
            p.y,
            p.z,
            q.x,
            q.y,
            q.z,
            q.w,
            if b.is_sleeping { 1. } else { 0. },
            v.x,
            v.y,
            v.z,
            w.x,
            w.y,
            w.z,
        ]
    }
}

fn vec3_of(v: &[f64]) -> crate::bounce::Vec3 {
    crate::bounce::Vec3::new(v[0], v[1], v[2])
}

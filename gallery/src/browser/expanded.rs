//! Individually ported additional official scenes; never executes upstream JavaScript.
use super::gltf_viewer::{OrbitViewer, decode_image, fetch, load_asset};
use crate::{Result, camera::*, geometry::*, material::*, math::*, scene::*};
use std::sync::Arc;

enum Content {
    #[cfg(feature = "lights_probes")]
    LightsProbes(Box<super::lights_probes::Demo>),
    #[cfg(feature = "exporters_video")]
    ExportersVideo(Box<super::exporters_video::Demo>),
    #[cfg(feature = "shadow_rtt")]
    ShadowRtt(Box<super::shadow_rtt::Demo>),
    #[cfg(feature = "spot_skinning")]
    SpotSkinning(Box<super::spot_skinning::Demo>),
    #[cfg(feature = "transform_curves")]
    TransformCurves(Box<super::transform_curves::Demo>),
    #[cfg(feature = "gltf_normals")]
    GltfNormals(Box<super::gltf_normals::Demo>),
    #[cfg(feature = "raycaster_helper")]
    RaycasterHelper(Box<super::raycaster_helper::Demo>),
    #[cfg(feature = "audio_timing")]
    AudioTiming(Box<super::audio_timing::Demo>),
    #[cfg(feature = "md2")]
    Md2(Box<super::md2::Demo>),
    #[cfg(feature = "md2_control")]
    Md2Control(Box<super::md2_control::Demo>),
    #[cfg(feature = "envmap_exr")]
    EnvmapExr(Box<super::envmap_exr::Demo>),
    #[cfg(feature = "dds")]
    Dds(Box<super::dds::Demo>),
    #[cfg(feature = "kinect")]
    Kinect(Box<super::kinect::Demo>),
    #[cfg(feature = "channels")]
    Channels(Box<super::channels::Demo>),
    #[cfg(feature = "wide_gamut")]
    WideGamut(Box<super::wide_gamut::Demo>),
    #[cfg(feature = "uv_tests")]
    UvTests(Box<super::uv_tests::Demo>),
    #[cfg(feature = "clipping_stencil")]
    ClippingStencil(Box<super::clipping_stencil::Demo>),
    #[cfg(feature = "ascii")]
    Ascii(Box<super::ascii::Demo>),
    #[cfg(feature = "vxgi_sponza")]
    VxgiSponza(Box<super::vxgi_sponza::Demo>),
    #[cfg(feature = "lightprobes_sponza")]
    LightprobesSponza(Box<super::lightprobes_sponza::Demo>),
    #[cfg(feature = "svg_sandbox")]
    SvgSandbox(Box<super::svg_sandbox::Demo>),
    #[cfg(feature = "svg_lines")]
    SvgLines(Box<super::svg_lines::Demo>),
    #[cfg(feature = "css3d_mixed")]
    Css3dMixed(Box<super::css3d_mixed::Demo>),
    #[cfg(feature = "css3d_molecules")]
    Css3dMolecules(Box<super::css3d_molecules::Demo>),
    #[cfg(feature = "css3d_periodictable")]
    Css3dPeriodictable(Box<super::css3d_periodictable::Demo>),
    #[cfg(feature = "css3d_sprites")]
    Css3dSprites(Box<super::css3d_sprites::Demo>),
    #[cfg(feature = "css3d_orthographic")]
    Css3dOrthographic(Box<super::css3d_orthographic::Demo>),
    #[cfg(feature = "css3d_sandbox")]
    Css3dSandbox(Box<super::css3d_sandbox::Demo>),
    #[cfg(feature = "css2d_label")]
    Css2dLabel(Box<super::css2d_label::Demo>),
    #[cfg(feature = "animation_pointer")]
    AnimationPointer(Box<super::animation_pointer::Demo>),
    #[cfg(feature = "envmaps_fasthdr")]
    EnvmapsFasthdr(Box<super::envmaps_fasthdr::Demo>),
    #[cfg(feature = "physics_rapier_vehicle_controller")]
    PhysicsRapierVehicleController(Box<super::physics_rapier_vehicle_controller::Demo>),
    #[cfg(feature = "physics_rapier_character_controller")]
    PhysicsRapierCharacterController(Box<super::physics_rapier_character_controller::Demo>),
    #[cfg(feature = "physics_rapier_terrain")]
    PhysicsRapierTerrain(Box<super::physics_rapier_terrain::Demo>),
    #[cfg(feature = "physics_rapier_joints")]
    PhysicsRapierJoints(Box<super::physics_rapier_joints::Demo>),
    #[cfg(feature = "physics_rapier_instancing")]
    PhysicsRapierInstancing(Box<super::physics_rapier_instancing::Demo>),
    #[cfg(feature = "physics_rapier_basic")]
    PhysicsRapierBasic(Box<super::physics_rapier_basic::Demo>),
    #[cfg(feature = "cubemap_webgl")]
    CubemapWebgl(Box<super::cubemap_webgl::Demo>),
    #[cfg(feature = "ssr_webgl")]
    SsrWebgl(Box<super::ssr_webgl::Demo>),
    #[cfg(feature = "lut_webgl")]
    LutWebgl(Box<super::lut_webgl::Demo>),
    #[cfg(feature = "shadowmap_webgl")]
    ShadowmapWebgl(Box<super::shadowmap_webgl::Demo>),
    #[cfg(feature = "dof_webgl")]
    DofWebgl(Box<super::dof_webgl::Demo>),
    #[cfg(feature = "sobel_webgl")]
    SobelWebgl(Box<super::sobel_webgl::Demo>),
    #[cfg(feature = "portal_webgl")]
    PortalWebgl(Box<super::portal_webgl::Demo>),
    #[cfg(feature = "refraction_webgl")]
    RefractionWebgl(Box<super::refraction_webgl::Demo>),
    #[cfg(feature = "mirror_webgl")]
    MirrorWebgl(Box<super::mirror_webgl::Demo>),
    #[cfg(feature = "afterimage_webgl")]
    AfterimageWebgl(Box<super::afterimage_webgl::Demo>),
    #[cfg(feature = "msaa_renderbuffers")]
    MsaaRenderbuffers(Box<super::msaa_renderbuffers::Demo>),
    #[cfg(feature = "fxaa_webgl")]
    FxaaWebgl(Box<super::fxaa_webgl::Demo>),
    #[cfg(feature = "loader_materialx")]
    LoaderMaterialx(Box<super::loader_materialx::Demo>),
    #[cfg(feature = "compute_reduce")]
    ComputeReduce(Box<super::compute_reduce::Demo>),
    #[cfg(feature = "generator_city")]
    GeneratorCity(Box<super::generator_city::Demo>),
    #[cfg(feature = "geometry_csg")]
    GeometryCsg(Box<super::geometry_csg::Demo>),
    #[cfg(feature = "ldraw")]
    Ldraw(Box<super::ldraw::Demo>),
    #[cfg(feature = "batch_lod_bvh")]
    BatchLodBvh(Box<super::batch_lod_bvh::Demo>),
    #[cfg(feature = "volume_fire")]
    VolumeFire(Box<super::volume_fire::Demo>),
    #[cfg(feature = "offscreen")]
    Offscreen(Box<super::offscreen::Demo>),
    #[cfg(feature = "lottie")]
    Lottie(Box<super::lottie::Demo>),
    #[cfg(feature = "watch")]
    Watch(Box<super::watch::Demo>),
    #[cfg(feature = "ifc_loader")]
    IfcLoader(Box<super::ifc_loader::Demo>),
    #[cfg(feature = "loader_3dm")]
    Loader3dm(Box<super::loader_3dm::Demo>),
    #[cfg(feature = "webcam")]
    Webcam(Box<super::webcam::Demo>),
    #[cfg(feature = "transmission_alpha")]
    TransmissionAlpha(Box<super::transmission_alpha::Demo>),
    #[cfg(feature = "random_uv")]
    RandomUv(Box<super::random_uv::Demo>),
    #[cfg(feature = "games_fps")]
    GamesFps(Box<super::games_fps::Demo>),
    #[cfg(feature = "fbx_loader")]
    FbxLoader(Box<super::fbx_loader::Demo>),
    #[cfg(feature = "svg_loader")]
    SvgLoader(Box<super::svg_loader::Demo>),
    #[cfg(feature = "exporter_gcode")]
    ExporterGcode(Box<super::exporter_gcode::Demo>),
    #[cfg(feature = "video_frame")]
    VideoFrame(Box<super::video_frame::Demo>),
    #[cfg(feature = "arcball")]
    Arcball(Box<super::arcball::Demo>),
    #[cfg(feature = "exporter_gltf")]
    ExporterGltf(Box<super::exporter_gltf::Demo>),
    #[cfg(feature = "spline_editor")]
    SplineEditor(Box<super::spline_editor::Demo>),
    #[cfg(feature = "raycaster_bvh")]
    RaycasterBvh(Box<super::raycaster_bvh::Demo>),
    #[cfg(feature = "subdivision")]
    Subdivision(Box<super::subdivision::Demo>),
    #[cfg(feature = "gtao_webgl")]
    GtaoWebgl(Box<super::gtao_webgl::Demo>),
    #[cfg(feature = "shadowmap_performance")]
    ShadowmapPerformance(Box<super::shadowmap_performance::Demo>),
    #[cfg(feature = "usdz_exporter")]
    UsdzExporter(Box<super::usdz_exporter::Demo>),
    #[cfg(feature = "usdz")]
    Usdz(Box<super::usdz::Demo>),
    #[cfg(feature = "vrml")]
    Vrml(Box<super::vrml::Demo>),
    #[cfg(feature = "birds_gltf")]
    BirdsGltf(Box<super::birds_gltf::Demo>),
    #[cfg(feature = "subsurface")]
    Subsurface(Box<super::subsurface::Demo>),
    #[cfg(feature = "dof2")]
    Dof2(Box<super::dof2::Demo>),
    #[cfg(feature = "simple_gi")]
    SimpleGi(Box<super::simple_gi::Demo>),
    #[cfg(feature = "ssr_denoise")]
    SsrDenoise(Box<super::ssr_denoise::Demo>),
    #[cfg(feature = "retargeting_readyplayer")]
    RetargetingReadyplayer(Box<super::retargeting_readyplayer::Demo>),
    #[cfg(feature = "shadowmap_progressive")]
    ShadowmapProgressive(Box<super::shadowmap_progressive::Demo>),
    #[cfg(feature = "compute_rasterizer_ibl")]
    ComputeRasterizerIbl(Box<super::compute_rasterizer_ibl::Demo>),
    #[cfg(feature = "sculpt")]
    Sculpt(Box<super::sculpt::Demo>),
    #[cfg(feature = "geometry_loft")]
    GeometryLoft(Box<super::geometry_loft::Demo>),
    #[cfg(feature = "generator_building")]
    GeneratorBuilding(Box<super::generator_building::Demo>),
    #[cfg(feature = "compute_rasterizer")]
    ComputeRasterizer(Box<super::compute_rasterizer::Demo>),
    #[cfg(feature = "shadow_array")]
    ShadowArray(Box<super::shadow_array::Demo>),
    #[cfg(feature = "gtao")]
    Gtao(Box<super::gtao::Demo>),
    #[cfg(feature = "vxgi")]
    Vxgi(Box<super::vxgi::Demo>),
    #[cfg(feature = "custom_fog")]
    CustomFog(Box<super::custom_fog::Demo>),
    #[cfg(feature = "water")]
    Water(Box<super::water::Demo>),
    #[cfg(feature = "gaussian_splat")]
    GaussianSplat(Box<super::gaussian_splat::Demo>),
    #[cfg(feature = "retargeting")]
    Retargeting(Box<super::retargeting::Demo>),
    #[cfg(feature = "skinning_instances")]
    SkinningInstances(Box<super::skinning_instances::Demo>),
    #[cfg(feature = "fog_volume")]
    FogVolume(Box<super::fog_volume::Demo>),
    #[cfg(feature = "backdrop_water")]
    BackdropWater(Box<super::backdrop_water::Demo>),
    #[cfg(feature = "fog_scattering")]
    FogScattering(Box<super::fog_scattering::Demo>),
    #[cfg(feature = "ssr")]
    Ssr(Box<super::ssr::Demo>),
    #[cfg(feature = "sss")]
    Sss(Box<super::sss::Demo>),
    #[cfg(feature = "ssgi")]
    Ssgi(Box<super::ssgi::Demo>),
    #[cfg(feature = "ballpool")]
    Ballpool(Box<super::ballpool::Demo>),
    #[cfg(feature = "pathtracer")]
    Pathtracer(Box<super::pathtracer::Demo>),
    #[cfg(feature = "progressive_lod")]
    ProgressiveLod(Box<super::progressive_lod::Demo>),
    #[cfg(feature = "morphtargets_webcam")]
    MorphtargetsWebcam(Box<super::morphtargets_webcam::Demo>),
    #[cfg(feature = "tiles3d")]
    Tiles3d(Box<super::tiles3d::Demo>),
    #[cfg(feature = "physics_ammo_instancing")]
    PhysicsAmmoInstancing(Box<super::physics_ammo_instancing::Demo>),
    #[cfg(feature = "physics_jolt_instancing")]
    PhysicsJoltInstancing(Box<super::physics_jolt_instancing::Demo>),
    #[cfg(feature = "physics_ammo_rope")]
    PhysicsAmmoRope(Box<super::physics_ammo_rope::Demo>),
    #[cfg(feature = "physics_ammo_cloth")]
    PhysicsAmmoCloth(Box<super::physics_ammo_cloth::Demo>),
    #[cfg(feature = "physics_ammo_volume")]
    PhysicsAmmoVolume(Box<super::physics_ammo_volume::Demo>),
    #[cfg(feature = "physics_ammo_terrain")]
    PhysicsAmmoTerrain(Box<super::physics_ammo_terrain::Demo>),
    #[cfg(feature = "physics_ammo_break")]
    PhysicsAmmoBreak(Box<super::physics_ammo_break::Demo>),
    #[cfg(feature = "html_texture")]
    HtmlTexture(Box<super::html_texture::Demo>),
    #[cfg(feature = "compute_water")]
    ComputeWater(Box<super::compute_water::Demo>),
    #[cfg(feature = "tsl_graph")]
    TslGraph(Box<super::tsl_graph::Demo>),
    #[cfg(feature = "lightprobes")]
    LightprobesComplex(Box<super::lightprobes::Demo>),
    #[cfg(feature = "lightprobes")]
    Lightprobes(Box<super::lightprobes::Demo>),
    #[cfg(feature = "fluid")]
    Fluid(Box<super::fluid::Demo>),
    #[cfg(feature = "hdr")]
    Hdr(Box<super::hdr::Demo>),
    #[cfg(feature = "volume_caustics")]
    VolumeCaustics(Box<super::volume_caustics::Demo>),
    #[cfg(feature = "compute_cloth")]
    ComputeCloth(Box<super::compute_cloth::Demo>),
    #[cfg(feature = "deferred")]
    Deferred(Box<super::deferred::Demo>),
    #[cfg(feature = "cubemap_dynamic")]
    CubemapDynamic(Box<super::cubemap_dynamic::Demo>),
    #[cfg(feature = "retro")]
    Retro(Box<super::retro::Demo>),
    #[cfg(feature = "gpgpu_protoplanet")]
    GpgpuProtoplanet(Box<super::gpgpu_protoplanet::Demo>),
    #[cfg(feature = "shadowmap_pcss")]
    ShadowmapPcss(Box<super::shadowmap_pcss::Demo>),
    #[cfg(feature = "volume_traa")]
    VolumeTraa(Box<super::volume_traa::Demo>),
    #[cfg(feature = "volume_rectarea")]
    VolumeRectarea(Box<super::volume_rectarea::Demo>),
    #[cfg(feature = "memory_outline")]
    MemoryOutline(Box<super::memory_outline::Demo>),
    #[cfg(feature = "godrays")]
    Godrays(Box<super::godrays::Demo>),
    #[cfg(feature = "volume_lighting")]
    VolumeLighting(Box<super::volume_lighting::Demo>),
    #[cfg(feature = "instancing_morph")]
    InstancingMorph(Box<super::instancing_morph::Demo>),
    #[cfg(feature = "shadowmap_csm")]
    ShadowmapCsm(Box<super::shadowmap_csm::Demo>),
    #[cfg(feature = "caustics")]
    Caustics(Box<super::caustics::Demo>),
    #[cfg(feature = "shadowmap_opacity")]
    ShadowmapOpacity(Box<super::shadowmap_opacity::Demo>),
    #[cfg(feature = "lights_projector")]
    LightsProjector(Box<super::lights_projector::Demo>),
    #[cfg(feature = "shadowmap_vsm")]
    ShadowmapVsm(Box<super::shadowmap_vsm::Demo>),
    #[cfg(feature = "lights_dynamic")]
    LightsDynamic(Box<super::lights_dynamic::Demo>),
    #[cfg(feature = "lights_clustered")]
    LightsClustered(Box<super::lights_clustered::Demo>),
    #[cfg(feature = "linked_particles")]
    LinkedParticles(Box<super::linked_particles::Demo>),
    #[cfg(feature = "attractors")]
    Attractors(Box<super::attractors::Demo>),
    #[cfg(feature = "sort_bitonic")]
    SortBitonic(Box<super::sort_bitonic::Demo>),
    #[cfg(feature = "compute_birds")]
    ComputeBirds(Box<super::compute_birds::Demo>),
    #[cfg(feature = "fbx_nurbs")]
    FbxNurbs(Box<super::fbx_nurbs::Demo>),
    #[cfg(feature = "points_dynamic")]
    PointsDynamic(Box<super::points_dynamic::Demo>),
    #[cfg(feature = "postprocessing_advanced")]
    PostprocessingAdvanced(Box<super::postprocessing_advanced::Demo>),
    #[cfg(feature = "ies_spotlight")]
    IesSpotlight(Box<super::ies_spotlight::Demo>),
    #[cfg(feature = "test_memory2")]
    TestMemory2(Box<super::test_memory2::Demo>),
    #[cfg(feature = "marching_cubes")]
    MarchingCubes(Box<super::marching_cubes::Demo>),
    #[cfg(feature = "render_bundle")]
    RenderBundle(Box<super::render_bundle::Demo>),
    #[cfg(feature = "mesh_batch")]
    MeshBatch(Box<super::mesh_batch::Demo>),
    #[cfg(feature = "volume_instancing")]
    VolumeInstancing(Box<super::volume_instancing::Demo>),
    #[cfg(feature = "test_memory")]
    TestMemory(Box<super::test_memory::Demo>),
    #[cfg(feature = "ktx")]
    Ktx(Box<super::ktx::Demo>),
    #[cfg(feature = "pvr")]
    Pvr(Box<super::pvr::Demo>),
    #[cfg(feature = "lines_raycast")]
    LinesRaycast(Box<super::lines_raycast::Demo>),
    #[cfg(feature = "log_depth")]
    LogDepth(Box<super::log_depth::Demo>),
    #[cfg(feature = "reversed_depth")]
    ReversedDepth(Box<super::reversed_depth::Demo>),
    #[cfg(feature = "elements_text")]
    ElementsText(Box<super::elements_text::Demo>),
    #[cfg(feature = "ktx2")]
    Ktx2(Box<super::ktx2::Demo>),
    #[cfg(feature = "outline")]
    Outline(Box<super::outline::Demo>),
    #[cfg(feature = "taa")]
    Taa(Box<super::taa::Demo>),
    #[cfg(feature = "sao")]
    Sao(Box<super::sao::Demo>),
    #[cfg(feature = "glitch")]
    Glitch(Box<super::glitch::Demo>),
    #[cfg(feature = "ssao")]
    Ssao(Box<super::ssao::Demo>),
    #[cfg(feature = "shadowmap_viewer")]
    ShadowmapViewer(Box<super::shadowmap_viewer::Demo>),
    #[cfg(feature = "audio_visual")]
    AudioVisual(Box<super::audio_visual::Demo>),
    #[cfg(feature = "texture_volumes")]
    TextureVolumes(Box<super::texture_volumes::Demo>),
    #[cfg(feature = "refraction_loaders")]
    ColladaAnim(Box<super::refraction_loaders::collada_anim::Demo>),
    #[cfg(feature = "threemf")]
    ThreeMf(Box<super::threemf::Demo>),
    #[cfg(feature = "texture_flares")]
    TextureFlares(Box<super::texture_flares::Demo>),
    #[cfg(feature = "draco_variants")]
    DracoVariants(Box<super::draco_variants::Demo>),
    #[cfg(feature = "passes_decals")]
    PassesDecals(Box<super::passes_decals::Demo>),
    #[cfg(feature = "probes_hdr")]
    ProbesHdr(Box<super::probes_hdr::Demo>),
    #[cfg(feature = "sky_water")]
    SkyWater(Box<super::sky_water::Demo>),
    #[cfg(feature = "exporters_matcap")]
    ExportersMatcap(Box<super::exporters_matcap::Demo>),
    #[cfg(feature = "selection_views")]
    SelectionViews(Box<super::selection_views::Demo>),
    #[cfg(feature = "text_clipping")]
    TextClipping(Box<super::text_clipping::Demo>),
    #[cfg(feature = "shapes_lights")]
    ShapesLights(Box<super::shapes_lights::Demo>),
    #[cfg(feature = "refraction_loaders")]
    RefractionLoaders(Box<super::refraction_loaders::Demo>),
    #[cfg(feature = "helpers_formats")]
    HelpersFormats(Box<super::helpers_formats::Demo>),
    #[cfg(feature = "picking_buffers")]
    PickingBuffers(Box<super::picking_buffers::Demo>),
    #[cfg(feature = "models_modifiers")]
    ModelsModifiers(Box<super::models_modifiers::Demo>),
    #[cfg(feature = "terrain_loaders")]
    TerrainLoaders(Box<super::terrain_loaders::Demo>),
    #[cfg(feature = "trackball_sprites")]
    TrackballSprites(Box<super::trackball_sprites::Demo>),
    #[cfg(feature = "controls_attributes")]
    ControlsAttributes(Box<super::controls_attributes::Demo>),
    #[cfg(feature = "stereo_loaders")]
    StereoLoaders(Box<super::stereo_loaders::Demo>),
    #[cfg(feature = "views_loaders")]
    ViewsLoaders(Box<super::views_loaders::Demo>),
    #[cfg(feature = "interactive_scenes")]
    InteractiveScenes(Box<super::interactive_scenes::Demo>),
    #[cfg(feature = "interactive_objects")]
    InteractiveObjects(Box<super::interactive_objects::Demo>),
    #[cfg(feature = "interactive_shaders")]
    InteractiveShaders(Box<super::interactive_shaders::Demo>),
    #[cfg(feature = "environment_materials")]
    EnvironmentMaterials(Box<super::environment_materials::Demo>),
    #[cfg(feature = "geometry_materials")]
    GeometryMaterials(Box<super::geometry_materials::Demo>),
    #[cfg(feature = "shader_geometry")]
    ShaderGeometry(Box<super::shader_geometry::Demo>),
    #[cfg(feature = "point_clouds")]
    PointClouds(Box<super::point_clouds::Demo>),
    #[cfg(feature = "buffer_particles")]
    BufferParticles(Box<super::buffer_particles::Demo>),
    #[cfg(feature = "shapes")]
    Shapes(Box<super::shapes::Demo>),
    #[cfg(feature = "material_textures")]
    MaterialTextures(Box<super::material_textures::Demo>),
    #[cfg(feature = "tsl_procedural")]
    TslProcedural(Box<super::tsl_procedural::Demo>),
    #[cfg(feature = "tsl_primitives")]
    TslPrimitives(Box<super::tsl_primitives::Demo>),
    #[cfg(feature = "tsl_materials")]
    TslMaterials(Box<super::tsl_materials::Demo>),
    #[cfg(feature = "tsl_viewport")]
    TslViewport(Box<super::tsl_viewport::Demo>),
    #[cfg(feature = "tsl_lighting")]
    TslLighting(Box<super::tsl_lighting::Demo>),
    #[cfg(feature = "tsl_environment")]
    TslEnvironment(Box<super::tsl_environment::Demo>),
    #[cfg(feature = "tsl_next")]
    TslNext(Box<super::tsl_next::Demo>),
    #[cfg(feature = "tsl_extended")]
    TslExtended(Box<super::tsl_extended::Demo>),
    #[cfg(feature = "tsl_surface")]
    TslSurface(Box<super::tsl_surface::Demo>),
    #[cfg(feature = "tsl_compute")]
    TslCompute(Box<super::tsl_compute::Demo>),
    #[cfg(feature = "tsl_particles")]
    TslParticles(Box<super::tsl_particles::Demo>),
    #[cfg(feature = "tsl_filters")]
    TslFilter(Box<super::tsl_filters::Demo>),
    Avif,
    #[cfg(feature = "tsl_passes")]
    TslPass(Box<super::tsl_passes::Demo>),
    #[cfg(feature = "tsl_examples")]
    Tsl(Box<super::tsl_examples::Demo>),
    #[cfg(feature = "gltf_examples")]
    Gltf(Box<super::gltf_examples::Demo>),
    Instancing,
    Triangles {
        objects: Vec<Object3D>,
        raw: bool,
    },
    Horse(super::expanded_morph_models::Horse),
    Sphere(super::expanded_morph_models::Sphere),
    Indexed(Object3D),
    ColorLines {
        objects: Vec<Object3D>,
        pointer: Vector2,
    },
    Geometries(Vec<Object3D>),
    Morph(Object3D),
    Dashed(Vec<Object3D>),
    AreaLights(Vec<Object3D>),
    VertexColors {
        pointer: Vector2,
    },
}
pub(super) struct Demo {
    viewer: OrbitViewer,
    near: f64,
    far: f64,
    content: Content,
    elapsed: f64,
}
impl Demo {
    #[cfg(feature = "tsl_procedural")]
    pub fn audio(&self) -> crate::Result<&super::tsl_procedural::audio::Audio> {
        if let Content::TslProcedural(p) = &self.content {
            p.audio()
        } else {
            Err(crate::Error::Invalid("not an audio example"))
        }
    }

    pub async fn create(
        scene: &mut Scene,
        camera: Object3D,
        example: u32,
        renderer: &crate::renderer::Renderer,
    ) -> Result<Self> {
        let aspect = match scene.camera(camera)?.0 {
            Camera::Perspective(c) => c.aspect,
            _ => 1.0,
        };
        match example {
            #[cfg(feature = "threemf")]
            318 | 319 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ThreeMf(Box::new(
                    super::threemf::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "refraction_loaders")]
            316 | 317 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ColladaAnim(Box::new(
                    super::refraction_loaders::collada_anim::Demo::create(
                        scene, camera, example, renderer,
                    )
                    .await?,
                )),
            }),
            #[cfg(feature = "texture_volumes")]
            314 | 315 | 321 | 324..=326 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::TextureVolumes(Box::new(
                    super::texture_volumes::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "gltf_normals")]
            313 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::GltfNormals(Box::new(
                    super::gltf_normals::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "raycaster_helper")]
            322 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::RaycasterHelper(Box::new(
                    super::raycaster_helper::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "audio_timing")]
            323 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::AudioTiming(Box::new(
                    super::audio_timing::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "audio_visual")]
            328..=330 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::AudioVisual(Box::new(
                    super::audio_visual::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "md2")]
            327 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::Md2(Box::new(
                    super::md2::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "md2_control")]
            331 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::Md2Control(Box::new(
                    super::md2_control::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "envmap_exr")]
            332 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::EnvmapExr(Box::new(
                    super::envmap_exr::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "dds")]
            335 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 100.,
                elapsed: 0.,
                content: Content::Dds(Box::new(
                    super::dds::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "kinect")]
            336 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::Kinect(Box::new(
                    super::kinect::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "channels")]
            337 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 500.,
                far: 3000.,
                elapsed: 0.,
                content: Content::Channels(Box::new(
                    super::channels::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "wide_gamut")]
            338 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::WideGamut(Box::new(
                    super::wide_gamut::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "uv_tests")]
            339 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::UvTests(Box::new(
                    super::uv_tests::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "clipping_stencil")]
            340 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 100.,
                elapsed: 0.,
                content: Content::ClippingStencil(Box::new(
                    super::clipping_stencil::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ascii")]
            341 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::Ascii(Box::new(
                    super::ascii::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "glitch")]
            342 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::Glitch(Box::new(
                    super::glitch::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ssao")]
            343 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 100.,
                far: 700.,
                elapsed: 0.,
                content: Content::Ssao(Box::new(
                    super::ssao::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "sao")]
            344 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 3.,
                far: 10.,
                elapsed: 0.,
                content: Content::Sao(Box::new(
                    super::sao::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "taa")]
            345 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::Taa(Box::new(
                    super::taa::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "outline")]
            346 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Outline(Box::new(
                    super::outline::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ktx2")]
            347 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10.,
                elapsed: 0.,
                content: Content::Ktx2(Box::new(
                    super::ktx2::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "elements_text")]
            348 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::ElementsText(Box::new(
                    super::elements_text::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "reversed_depth")]
            349 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 5.,
                far: 9999.,
                elapsed: 0.,
                content: Content::ReversedDepth(Box::new(
                    super::reversed_depth::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "log_depth")]
            350 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1e-6,
                far: 1e27,
                elapsed: 0.,
                content: Content::LogDepth(Box::new(
                    super::log_depth::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lines_raycast")]
            351 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::LinesRaycast(Box::new(
                    super::lines_raycast::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "pvr")]
            352 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::Pvr(Box::new(
                    super::pvr::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ktx")]
            353 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::Ktx(Box::new(
                    super::ktx::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "test_memory")]
            354 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::TestMemory(Box::new(
                    super::test_memory::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "volume_instancing")]
            355 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1000.,
                elapsed: 0.,
                content: Content::VolumeInstancing(Box::new(
                    super::volume_instancing::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "mesh_batch")]
            356 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 100.,
                elapsed: 0.,
                content: Content::MeshBatch(Box::new(
                    super::mesh_batch::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "render_bundle")]
            357 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 100.,
                elapsed: 0.,
                content: Content::RenderBundle(Box::new(
                    super::render_bundle::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "marching_cubes")]
            358 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::MarchingCubes(Box::new(
                    super::marching_cubes::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "test_memory2")]
            359 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::TestMemory2(Box::new(
                    super::test_memory2::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ies_spotlight")]
            360 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::IesSpotlight(Box::new(
                    super::ies_spotlight::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "postprocessing_advanced")]
            361 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::PostprocessingAdvanced(Box::new(
                    super::postprocessing_advanced::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "points_dynamic")]
            362 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 50000.,
                elapsed: 0.,
                content: Content::PointsDynamic(Box::new(
                    super::points_dynamic::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "fbx_nurbs")]
            363 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::FbxNurbs(Box::new(
                    super::fbx_nurbs::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "compute_birds")]
            364 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ComputeBirds(Box::new(
                    super::compute_birds::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "sort_bitonic")]
            365 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.,
                far: 2.,
                elapsed: 0.,
                content: Content::SortBitonic(Box::new(
                    super::sort_bitonic::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "attractors")]
            366 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Attractors(Box::new(
                    super::attractors::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "linked_particles")]
            367 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 200.,
                elapsed: 0.,
                content: Content::LinkedParticles(Box::new(
                    super::linked_particles::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lights_clustered")]
            368 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 200.,
                elapsed: 0.,
                content: Content::LightsClustered(Box::new(
                    super::lights_clustered::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lights_dynamic")]
            369 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 200.,
                elapsed: 0.,
                content: Content::LightsDynamic(Box::new(
                    super::lights_dynamic::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadowmap_vsm")]
            370 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::ShadowmapVsm(Box::new(
                    super::shadowmap_vsm::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lights_projector")]
            371 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::LightsProjector(Box::new(
                    super::lights_projector::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadowmap_opacity")]
            372 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 40.,
                elapsed: 0.,
                content: Content::ShadowmapOpacity(Box::new(
                    super::shadowmap_opacity::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "caustics")]
            373 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.025,
                far: 5.,
                elapsed: 0.,
                content: Content::Caustics(Box::new(
                    super::caustics::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadowmap_csm")]
            374 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::ShadowmapCsm(Box::new(
                    super::shadowmap_csm::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "instancing_morph")]
            375 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 100.,
                far: 10000.,
                elapsed: 0.,
                content: Content::InstancingMorph(Box::new(
                    super::instancing_morph::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "volume_lighting")]
            376 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::VolumeLighting(Box::new(
                    super::volume_lighting::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "godrays")]
            377 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1000.,
                elapsed: 0.,
                content: Content::Godrays(Box::new(
                    super::godrays::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "memory_outline")]
            378 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::MemoryOutline(Box::new(
                    super::memory_outline::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "volume_rectarea")]
            379 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 250.,
                elapsed: 0.,
                content: Content::VolumeRectarea(Box::new(
                    super::volume_rectarea::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "volume_traa")]
            380 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::VolumeTraa(Box::new(
                    super::volume_traa::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadowmap_pcss")]
            381 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::ShadowmapPcss(Box::new(
                    super::shadowmap_pcss::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "gpgpu_protoplanet")]
            384 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 5.,
                far: 15000.,
                elapsed: 0.,
                content: Content::GpgpuProtoplanet(Box::new(
                    super::gpgpu_protoplanet::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "retro")]
            385 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Retro(Box::new(
                    super::retro::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "cubemap_dynamic")]
            386 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::CubemapDynamic(Box::new(
                    super::cubemap_dynamic::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "deferred")]
            387 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 20.,
                elapsed: 0.,
                content: Content::Deferred(Box::new(
                    super::deferred::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "compute_cloth")]
            388 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.01,
                far: 10.,
                elapsed: 0.,
                content: Content::ComputeCloth(Box::new(
                    super::compute_cloth::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "volume_caustics")]
            389 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.025,
                far: 5.,
                elapsed: 0.,
                content: Content::VolumeCaustics(Box::new(
                    super::volume_caustics::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "hdr")]
            390 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2.,
                elapsed: 0.,
                content: Content::Hdr(Box::new(
                    super::hdr::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "fluid")]
            391 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.01,
                far: 10.,
                elapsed: 0.,
                content: Content::Fluid(Box::new(
                    super::fluid::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lightprobes")]
            392 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Lightprobes(Box::new(
                    super::lightprobes::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lightprobes")]
            393 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::LightprobesComplex(Box::new(
                    super::lightprobes::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_graph")]
            394 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 200.,
                elapsed: 0.,
                content: Content::TslGraph(Box::new(
                    super::tsl_graph::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "compute_water")]
            395 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 3000.,
                elapsed: 0.,
                content: Content::ComputeWater(Box::new(
                    super::compute_water::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ssgi")]
            396 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Ssgi(Box::new(
                    super::ssgi::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "pathtracer")]
            485 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::Pathtracer(Box::new(
                    super::pathtracer::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "progressive_lod")]
            486 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 40.,
                elapsed: 0.,
                content: Content::ProgressiveLod(Box::new(
                    super::progressive_lod::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "html_texture")]
            496 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::HtmlTexture(Box::new(
                    super::html_texture::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "physics_ammo_break")]
            495 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.2,
                far: 2000.,
                elapsed: 0.,
                content: Content::PhysicsAmmoBreak(Box::new(
                    super::physics_ammo_break::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_ammo_terrain")]
            494 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.2,
                far: 2000.,
                elapsed: 0.,
                content: Content::PhysicsAmmoTerrain(Box::new(
                    super::physics_ammo_terrain::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_ammo_volume")]
            493 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.2,
                far: 2000.,
                elapsed: 0.,
                content: Content::PhysicsAmmoVolume(Box::new(
                    super::physics_ammo_volume::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_ammo_cloth")]
            492 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.2,
                far: 2000.,
                elapsed: 0.,
                content: Content::PhysicsAmmoCloth(Box::new(
                    super::physics_ammo_cloth::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_ammo_rope")]
            491 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.2,
                far: 2000.,
                elapsed: 0.,
                content: Content::PhysicsAmmoRope(Box::new(
                    super::physics_ammo_rope::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_jolt_instancing")]
            490 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::PhysicsJoltInstancing(Box::new(
                    super::physics_jolt_instancing::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_ammo_instancing")]
            489 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::PhysicsAmmoInstancing(Box::new(
                    super::physics_ammo_instancing::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "tiles3d")]
            488 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 10.,
                far: 1e6,
                elapsed: 0.,
                content: Content::Tiles3d(Box::new(
                    super::tiles3d::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "morphtargets_webcam")]
            487 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::MorphtargetsWebcam(Box::new(
                    super::morphtargets_webcam::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "ballpool")]
            484 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Ballpool(Box::new(
                    super::ballpool::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "sss")]
            397 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Sss(Box::new(
                    super::sss::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ssr")]
            398 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 50.,
                elapsed: 0.,
                content: Content::Ssr(Box::new(
                    super::ssr::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "fog_scattering")]
            399 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 120.,
                elapsed: 0.,
                content: Content::FogScattering(Box::new(
                    super::fog_scattering::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "backdrop_water")]
            400 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 30.,
                elapsed: 0.,
                content: Content::BackdropWater(Box::new(
                    super::backdrop_water::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "fog_volume")]
            401 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::FogVolume(Box::new(
                    super::fog_volume::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "skinning_instances")]
            402 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.01,
                far: 60.,
                elapsed: 0.,
                content: Content::SkinningInstances(Box::new(
                    super::skinning_instances::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "retargeting")]
            403 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 50.,
                elapsed: 0.,
                content: Content::Retargeting(Box::new(
                    super::retargeting::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "gaussian_splat")]
            404 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.01,
                far: 100.,
                elapsed: 0.,
                content: Content::GaussianSplat(Box::new(
                    super::gaussian_splat::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "water")]
            405 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 200.,
                elapsed: 0.,
                content: Content::Water(Box::new(
                    super::water::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "custom_fog")]
            406 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 20000.,
                elapsed: 0.,
                content: Content::CustomFog(Box::new(
                    super::custom_fog::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "vxgi")]
            407 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Vxgi(Box::new(
                    super::vxgi::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "gtao")]
            408 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 50.,
                elapsed: 0.,
                content: Content::Gtao(Box::new(
                    super::gtao::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadow_array")]
            409 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::ShadowArray(Box::new(
                    super::shadow_array::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "compute_rasterizer")]
            410 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 1000000.,
                elapsed: 0.,
                content: Content::ComputeRasterizer(Box::new(
                    super::compute_rasterizer::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "generator_building")]
            411 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 20000.,
                elapsed: 0.,
                content: Content::GeneratorBuilding(Box::new(
                    super::generator_building::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "geometry_loft")]
            412 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::GeometryLoft(Box::new(
                    super::geometry_loft::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "sculpt")]
            413 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Sculpt(Box::new(
                    super::sculpt::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "compute_rasterizer_ibl")]
            414 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 1000000.,
                elapsed: 0.,
                content: Content::ComputeRasterizerIbl(Box::new(
                    super::compute_rasterizer_ibl::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "shadowmap_progressive")]
            415 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::ShadowmapProgressive(Box::new(
                    super::shadowmap_progressive::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "retargeting_readyplayer")]
            416 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 50.,
                elapsed: 0.,
                content: Content::RetargetingReadyplayer(Box::new(
                    super::retargeting_readyplayer::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "ssr_denoise")]
            417 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 8.,
                elapsed: 0.,
                content: Content::SsrDenoise(Box::new(
                    super::ssr_denoise::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "simple_gi")]
            418 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::SimpleGi(Box::new(
                    super::simple_gi::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "dof2")]
            419 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 3000.,
                elapsed: 0.,
                content: Content::Dof2(Box::new(
                    super::dof2::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "subsurface")]
            420 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::Subsurface(Box::new(
                    super::subsurface::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "birds_gltf")]
            421 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 3000.,
                elapsed: 0.,
                content: Content::BirdsGltf(Box::new(
                    super::birds_gltf::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "vrml")]
            422 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1e10,
                elapsed: 0.,
                content: Content::Vrml(Box::new(
                    super::vrml::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "usdz")]
            423 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Usdz(Box::new(
                    super::usdz::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "usdz_exporter")]
            424 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 20.,
                elapsed: 0.,
                content: Content::UsdzExporter(Box::new(
                    super::usdz_exporter::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadowmap_performance")]
            425 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 5.,
                far: 3000.,
                elapsed: 0.,
                content: Content::ShadowmapPerformance(Box::new(
                    super::shadowmap_performance::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "gtao_webgl")]
            426 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 100.,
                elapsed: 0.,
                content: Content::GtaoWebgl(Box::new(
                    super::gtao_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "subdivision")]
            427 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 2000.,
                elapsed: 0.,
                content: Content::Subdivision(Box::new(
                    super::subdivision::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "raycaster_bvh")]
            428 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 100.,
                elapsed: 0.,
                content: Content::RaycasterBvh(Box::new(
                    super::raycaster_bvh::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "spline_editor")]
            429 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::SplineEditor(Box::new(
                    super::spline_editor::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "exporter_gltf")]
            430 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::ExporterGltf(Box::new(
                    super::exporter_gltf::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "arcball")]
            431 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.01,
                far: 2000.,
                elapsed: 0.,
                content: Content::Arcball(Box::new(
                    super::arcball::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "video_frame")]
            432 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 10.,
                elapsed: 0.,
                content: Content::VideoFrame(Box::new(
                    super::video_frame::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "exporter_gcode")]
            433 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::ExporterGcode(Box::new(
                    super::exporter_gcode::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "svg_loader")]
            434 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::SvgLoader(Box::new(
                    super::svg_loader::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "fbx_loader")]
            435 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::FbxLoader(Box::new(
                    super::fbx_loader::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "games_fps")]
            436 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1000.,
                elapsed: 0.,
                content: Content::GamesFps(Box::new(
                    super::games_fps::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "random_uv")]
            437 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 20.,
                elapsed: 0.,
                content: Content::RandomUv(Box::new(
                    super::random_uv::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "transmission_alpha")]
            438 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::TransmissionAlpha(Box::new(
                    super::transmission_alpha::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "webcam")]
            439 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::Webcam(Box::new(
                    super::webcam::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "loader_3dm")]
            440 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::Loader3dm(Box::new(
                    super::loader_3dm::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ifc_loader")]
            441 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1000.,
                elapsed: 0.,
                content: Content::IfcLoader(Box::new(
                    super::ifc_loader::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "watch")]
            442 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 20.,
                elapsed: 0.,
                content: Content::Watch(Box::new(
                    super::watch::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lottie")]
            443 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 10.,
                elapsed: 0.,
                content: Content::Lottie(Box::new(
                    super::lottie::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "offscreen")]
            444 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::Offscreen(Box::new(
                    super::offscreen::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "volume_fire")]
            445 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::VolumeFire(Box::new(
                    super::volume_fire::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "batch_lod_bvh")]
            446 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1000.,
                elapsed: 0.,
                content: Content::BatchLodBvh(Box::new(
                    super::batch_lod_bvh::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ldraw")]
            447 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::Ldraw(Box::new(
                    super::ldraw::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "geometry_csg")]
            448 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 100.,
                elapsed: 0.,
                content: Content::GeometryCsg(Box::new(
                    super::geometry_csg::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "generator_city")]
            449 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 20000.,
                elapsed: 0.,
                content: Content::GeneratorCity(Box::new(
                    super::generator_city::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "compute_reduce")]
            450 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.,
                far: 2.,
                elapsed: 0.,
                content: Content::ComputeReduce(Box::new(
                    super::compute_reduce::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "loader_materialx")]
            451 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 200.,
                elapsed: 0.,
                content: Content::LoaderMaterialx(Box::new(
                    super::loader_materialx::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "fxaa_webgl")]
            452 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 2000.,
                elapsed: 0.,
                content: Content::FxaaWebgl(Box::new(
                    super::fxaa_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "msaa_renderbuffers")]
            453 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 10.,
                far: 2000.,
                elapsed: 0.,
                content: Content::MsaaRenderbuffers(Box::new(
                    super::msaa_renderbuffers::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "afterimage_webgl")]
            454 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::AfterimageWebgl(Box::new(
                    super::afterimage_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "mirror_webgl")]
            455 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 500.,
                elapsed: 0.,
                content: Content::MirrorWebgl(Box::new(
                    super::mirror_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "refraction_webgl")]
            456 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 500.,
                elapsed: 0.,
                content: Content::RefractionWebgl(Box::new(
                    super::refraction_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "portal_webgl")]
            457 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::PortalWebgl(Box::new(
                    super::portal_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "sobel_webgl")]
            458 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::SobelWebgl(Box::new(
                    super::sobel_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "dof_webgl")]
            459 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 3000.,
                elapsed: 0.,
                content: Content::DofWebgl(Box::new(
                    super::dof_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadowmap_webgl")]
            460 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 10.,
                far: 3000.,
                elapsed: 0.,
                content: Content::ShadowmapWebgl(Box::new(
                    super::shadowmap_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lut_webgl")]
            461 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.25,
                far: 20.,
                elapsed: 0.,
                content: Content::LutWebgl(Box::new(
                    super::lut_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "ssr_webgl")]
            462 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 15.,
                elapsed: 0.,
                content: Content::SsrWebgl(Box::new(
                    super::ssr_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "cubemap_webgl")]
            463 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::CubemapWebgl(Box::new(
                    super::cubemap_webgl::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "physics_rapier_basic")]
            464 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::PhysicsRapierBasic(Box::new(
                    super::physics_rapier_basic::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_rapier_instancing")]
            465 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::PhysicsRapierInstancing(Box::new(
                    super::physics_rapier_instancing::Demo::create(
                        scene, camera, example, renderer,
                    )
                    .await?,
                )),
            }),
            #[cfg(feature = "physics_rapier_joints")]
            466 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::PhysicsRapierJoints(Box::new(
                    super::physics_rapier_joints::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_rapier_terrain")]
            467 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.2,
                far: 2000.,
                elapsed: 0.,
                content: Content::PhysicsRapierTerrain(Box::new(
                    super::physics_rapier_terrain::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "physics_rapier_character_controller")]
            468 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::PhysicsRapierCharacterController(Box::new(
                    super::physics_rapier_character_controller::Demo::create(
                        scene, camera, example, renderer,
                    )
                    .await?,
                )),
            }),
            #[cfg(feature = "physics_rapier_vehicle_controller")]
            469 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::PhysicsRapierVehicleController(Box::new(
                    super::physics_rapier_vehicle_controller::Demo::create(
                        scene, camera, example, renderer,
                    )
                    .await?,
                )),
            }),
            #[cfg(feature = "envmaps_fasthdr")]
            470 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 50.,
                elapsed: 0.,
                content: Content::EnvmapsFasthdr(Box::new(
                    super::envmaps_fasthdr::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "animation_pointer")]
            471 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.2,
                far: 100.,
                elapsed: 0.,
                content: Content::AnimationPointer(Box::new(
                    super::animation_pointer::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "css2d_label")]
            472 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 200.,
                elapsed: 0.,
                content: Content::Css2dLabel(Box::new(
                    super::css2d_label::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "css3d_sandbox")]
            473 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::Css3dSandbox(Box::new(
                    super::css3d_sandbox::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "css3d_orthographic")]
            474 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 1000.,
                elapsed: 0.,
                content: Content::Css3dOrthographic(Box::new(
                    super::css3d_orthographic::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "css3d_sprites")]
            475 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::Css3dSprites(Box::new(
                    super::css3d_sprites::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "css3d_periodictable")]
            476 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::Css3dPeriodictable(Box::new(
                    super::css3d_periodictable::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "css3d_molecules")]
            477 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::Css3dMolecules(Box::new(
                    super::css3d_molecules::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "css3d_mixed")]
            478 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::Css3dMixed(Box::new(
                    super::css3d_mixed::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "svg_lines")]
            479 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 100.,
                elapsed: 0.,
                content: Content::SvgLines(Box::new(
                    super::svg_lines::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "svg_sandbox")]
            480 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::SvgSandbox(Box::new(
                    super::svg_sandbox::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lightprobes_sponza")]
            481 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1000.,
                elapsed: 0.,
                // Boxed: the Sponza loaders' futures would otherwise grow every
                // example's create future past the wasm stack.
                content: Content::LightprobesSponza(Box::new(
                    Box::pin(super::lightprobes_sponza::Demo::create(
                        scene, camera, example, renderer,
                    ))
                    .await?,
                )),
            }),
            #[cfg(feature = "vxgi_sponza")]
            482 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 1000.,
                elapsed: 0.,
                content: Content::VxgiSponza(Box::new(
                    Box::pin(super::vxgi_sponza::Demo::create(
                        scene, camera, example, renderer,
                    ))
                    .await?,
                )),
            }),
            #[cfg(feature = "shadowmap_viewer")]
            333 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ShadowmapViewer(Box::new(
                    super::shadowmap_viewer::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "transform_curves")]
            308..=310 | 312 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::TransformCurves(Box::new(
                    super::transform_curves::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "spot_skinning")]
            303..=307 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::SpotSkinning(Box::new(
                    super::spot_skinning::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "texture_flares")]
            298..=302 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::TextureFlares(Box::new(
                    super::texture_flares::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "draco_variants")]
            293..=297 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::DracoVariants(Box::new(
                    super::draco_variants::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "passes_decals")]
            288..=292 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::PassesDecals(Box::new(
                    super::passes_decals::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "probes_hdr")]
            283..=287 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::ProbesHdr(Box::new(
                    super::probes_hdr::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shadow_rtt")]
            278..=282 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::ShadowRtt(Box::new(
                    super::shadow_rtt::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "exporters_video")]
            273..=277 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::ExportersVideo(Box::new(
                    super::exporters_video::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "lights_probes")]
            268..=272 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::LightsProbes(Box::new(
                    super::lights_probes::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "sky_water")]
            263..=267 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::SkyWater(Box::new(
                    super::sky_water::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "exporters_matcap")]
            258..=262 | 383 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::ExportersMatcap(Box::new(
                    super::exporters_matcap::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "selection_views")]
            253..=257 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::SelectionViews(Box::new(super::selection_views::Demo::create(
                    scene, camera, example,
                )?)),
            }),
            #[cfg(feature = "text_clipping")]
            248..=252 | 320 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::TextClipping(Box::new(
                    super::text_clipping::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shapes_lights")]
            243..=247 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.1,
                far: 5000.,
                elapsed: 0.,
                content: Content::ShapesLights(Box::new(
                    super::shapes_lights::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "refraction_loaders")]
            238..=242 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.01,
                far: 100000.,
                elapsed: 0.,
                content: Content::RefractionLoaders(Box::new(
                    super::refraction_loaders::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "helpers_formats")]
            233..=237 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 0.01,
                far: 5000.,
                elapsed: 0.,
                content: Content::HelpersFormats(Box::new(
                    super::helpers_formats::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "picking_buffers")]
            228..=232 | 382 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 10000.,
                elapsed: 0.,
                content: Content::PickingBuffers(Box::new(
                    super::picking_buffers::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "models_modifiers")]
            223..=227 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ModelsModifiers(Box::new(
                    super::models_modifiers::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "terrain_loaders")]
            218..=222 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::TerrainLoaders(Box::new(
                    super::terrain_loaders::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "trackball_sprites")]
            213..=217 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::TrackballSprites(Box::new(
                    super::trackball_sprites::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "controls_attributes")]
            208..=212 | 311 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ControlsAttributes(Box::new(
                    super::controls_attributes::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "stereo_loaders")]
            203..=207 | 334 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::StereoLoaders(Box::new(
                    super::stereo_loaders::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "views_loaders")]
            198..=202 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ViewsLoaders(Box::new(
                    super::views_loaders::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "interactive_scenes")]
            193..=197 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::InteractiveScenes(Box::new(
                    super::interactive_scenes::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "interactive_objects")]
            188..=192 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::InteractiveObjects(Box::new(
                    super::interactive_objects::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "interactive_shaders")]
            183..=187 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::InteractiveShaders(Box::new(
                    super::interactive_shaders::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "environment_materials")]
            178..=182 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::EnvironmentMaterials(Box::new(
                    super::environment_materials::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "geometry_materials")]
            173 | 176 | 177 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::GeometryMaterials(Box::new(
                    super::geometry_materials::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "shader_geometry")]
            168..=172 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::ShaderGeometry(Box::new(
                    super::shader_geometry::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "point_clouds")]
            163..=167 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::PointClouds(Box::new(
                    super::point_clouds::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "buffer_particles")]
            158..=162 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::BufferParticles(Box::new(
                    super::buffer_particles::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "shapes")]
            153..=157 | 175 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::Shapes(Box::new(
                    super::shapes::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "material_textures")]
            148..=152 | 174 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.),
                near: 1.,
                far: 5000.,
                elapsed: 0.,
                content: Content::MaterialTextures(Box::new(
                    super::material_textures::Demo::create(scene, camera, example, renderer)
                        .await?,
                )),
            }),
            #[cfg(feature = "tsl_procedural")]
            128..=147 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 1.0,
                far: 5000.0,
                elapsed: 0.0,
                content: Content::TslProcedural(Box::new(
                    super::tsl_procedural::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_primitives")]
            123..=127 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 1.0,
                far: 5000.0,
                elapsed: 0.0,
                content: Content::TslPrimitives(Box::new(
                    super::tsl_primitives::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_materials")]
            118..=122 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 1.0,
                far: 5000.0,
                elapsed: 0.0,
                content: Content::TslMaterials(Box::new(
                    super::tsl_materials::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_viewport")]
            113..=117 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.1,
                far: 100.0,
                elapsed: 0.0,
                content: Content::TslViewport(Box::new(
                    super::tsl_viewport::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_lighting")]
            108..=112 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.1,
                far: 100.0,
                elapsed: 0.0,
                content: Content::TslLighting(Box::new(
                    super::tsl_lighting::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_environment")]
            103..=107 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.1,
                far: 100.0,
                elapsed: 0.0,
                content: Content::TslEnvironment(Box::new(
                    super::tsl_environment::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_next")]
            98..=102 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.1,
                far: 100.0,
                elapsed: 0.0,
                content: Content::TslNext(Box::new(
                    super::tsl_next::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_extended")]
            78..=97 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 1.0,
                far: 1000.0,
                elapsed: 0.0,
                content: Content::TslExtended(Box::new(
                    super::tsl_extended::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_surface")]
            58..=77 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 1.0,
                far: 1000.0,
                elapsed: 0.0,
                content: Content::TslSurface(Box::new(
                    super::tsl_surface::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_compute")]
            53..=57 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.0,
                far: 2.0,
                elapsed: 0.0,
                content: Content::TslCompute(Box::new(
                    super::tsl_compute::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_particles")]
            48..=52 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.0,
                far: 2.0,
                elapsed: 0.0,
                content: Content::TslParticles(Box::new(
                    super::tsl_particles::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_filters")]
            43..=47 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.0,
                far: 2.0,
                elapsed: 0.0,
                content: Content::TslFilter(Box::new(
                    super::tsl_filters::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_passes")]
            38..=42 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.0,
                far: 2.0,
                elapsed: 0.0,
                content: Content::TslPass(Box::new(
                    super::tsl_passes::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "tsl_examples")]
            35..=37 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.0,
                far: 2.0,
                elapsed: 0.0,
                content: Content::Tsl(Box::new(
                    super::tsl_examples::Demo::create(scene, camera, example, renderer).await?,
                )),
            }),
            #[cfg(feature = "gltf_examples")]
            29..=34 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1.0),
                near: 0.1,
                far: 100.0,
                elapsed: 0.0,
                content: Content::Gltf(Box::new(
                    super::gltf_examples::Demo::create(scene, camera, example).await?,
                )),
            }),
            28 => {
                let (asset, buffers, images) =
                    load_asset("/web/models/AVIFTest/forest_house.glb").await?;
                crate::gltf::import_decoded(&asset, &buffers, &images)?.instantiate(scene)?;
                scene.background = Color::from_hex(0xf6eedc);
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 0.1,
                        far: 100.0,
                        ..Default::default()
                    }));
                let center = Vector3::new(0.0, 2.0, 0.0);
                let offset = Vector3::new(1.5, 4.0, 9.0) - center;
                let mut viewer = OrbitViewer::from_camera(center, offset.length());
                viewer.fixture(
                    offset.x.atan2(offset.z),
                    (offset.y / offset.length()).asin(),
                    1.8,
                );
                Ok(Self {
                    viewer,
                    near: 0.1,
                    far: 100.0,
                    content: Content::Avif,
                    elapsed: 0.0,
                })
            }
            26 | 27 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 2.0),
                near: 1.0,
                far: 3500.0,
                elapsed: 0.0,
                content: Content::Triangles {
                    objects: super::expanded_triangles::create(
                        scene,
                        camera,
                        aspect,
                        example == 27,
                        renderer,
                    )
                    .await?,
                    raw: example == 27,
                },
            }),
            16 => {
                let (asset, buffers, images) = load_asset(
                    "/web/models/DamagedHelmet/glTF-instancing/DamagedHelmetGpuInstancing.gltf",
                )
                .await?;
                crate::gltf::import_animated_decoded(&asset, &buffers, &images)?
                    .instantiate(scene)?;
                // r186 GLTFLoader applies assignFinalMaterial twice to InstancedMesh,
                // flipping derivative-tangent normalScale.y twice. Match this example's
                // actual material without changing the glTF importer's convention.
                for root in scene.roots().to_vec() {
                    for h in scene.traverse(root, true)? {
                        if let NodeKind::Mesh(mesh) = &mut scene.get_mut(h)?.kind {
                            for material in &mut mesh.materials {
                                if let Material::Standard(p) = Arc::make_mut(material) {
                                    p.normal_scale.y *= -1.0;
                                }
                            }
                        }
                    }
                }
                scene.environment = Some(Arc::new(crate::environment::EnvironmentMap::from_hdr(
                    &fetch("/web/environments/royal_esplanade_2k.hdr").await?,
                )?));
                scene.background_environment = true;
                scene.aces_tone_mapping = true;
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 0.25,
                        far: 20.0,
                        ..Default::default()
                    }));
                let center = Vector3::new(0.0, 0.25, 0.0);
                let offset = Vector3::new(-0.90, 0.41, -0.89) - center;
                let mut viewer = OrbitViewer::from_camera(center, offset.length());
                viewer.fixture(
                    offset.x.atan2(offset.z),
                    (offset.y / offset.length()).asin(),
                    1.8,
                );
                Ok(Self {
                    viewer,
                    near: 0.25,
                    far: 20.0,
                    content: Content::Instancing,
                    elapsed: 0.0,
                })
            }
            17 => {
                use std::f64::consts::{PI, TAU};
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 1.0,
                        far: 2000.0,
                        ..Default::default()
                    }));
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0xcccccc),
                    intensity: 1.5,
                }));
                let point = scene.insert(NodeKind::Light(Light::Point {
                    color: Color::WHITE,
                    intensity: 2.5,
                    distance: 0.0,
                    decay: 0.0,
                }));
                scene.add(camera, point)?;
                let mut map =
                    decode_image(&fetch("/web/gallery/assets/uv-grid.jpg").await?).await?;
                map.wrap_s = Wrapping::Repeat;
                map.wrap_t = Wrapping::Repeat;
                map.anisotropy = 16;
                map.srgb = true;
                map.mipmap_filter = Some(Filter::Linear);
                let mut material = MeshPhongMaterial::default();
                material.properties.map = Some(Arc::new(map));
                material.properties.side = Side::Double;
                let material = Arc::new(Material::Phong(material));
                let points = (0..50)
                    .map(|i| {
                        let i = i as f64;
                        Vector2::new(
                            (i * 0.2).sin() * (i * 0.1).sin() * 15.0 + 50.0,
                            (i - 5.0) * 2.0,
                        )
                    })
                    .collect::<Vec<_>>();
                let mut plane = ParametricGeometry::build(|u, v| Vector3::new(u, 0.0, v), 10, 10)?;
                plane.scale(Vector3::splat(100.0))?;
                plane.center()?;
                let klein = ParametricGeometry::build(
                    |v, u| {
                        let u = u * TAU;
                        let v = v * TAU;
                        let r = 2.0 * (1.0 - u.cos() / 2.0);
                        let (x, z) = if u < PI {
                            (
                                3.0 * u.cos() * (1.0 + u.sin()) + r * u.cos() * v.cos(),
                                -8.0 * u.sin() - r * u.sin() * v.cos(),
                            )
                        } else {
                            (
                                3.0 * u.cos() * (1.0 + u.sin()) + r * (v + PI).cos(),
                                -8.0 * u.sin(),
                            )
                        };
                        Vector3::new(x, -r * v.sin(), z)
                    },
                    20,
                    20,
                )?;
                let mobius = ParametricGeometry::build(
                    |u, t| {
                        let u = u - 0.5;
                        let v = t * TAU;
                        Vector3::new(
                            v.cos() * (2.0 + u * (v / 2.0).cos()),
                            v.sin() * (2.0 + u * (v / 2.0).cos()),
                            u * (v / 2.0).sin(),
                        )
                    },
                    20,
                    20,
                )?;
                let geometries = [
                    SphereGeometry::build(75.0, 20, 10)?,
                    IcosahedronGeometry::build(75.0, 0)?,
                    OctahedronGeometry::build(75.0, 0)?,
                    TetrahedronGeometry::build(75.0, 0)?,
                    PlaneGeometry::build(100.0, 100.0, 4, 4)?,
                    BoxGeometry::segmented(100.0, 100.0, 100.0, 4, 4, 4)?,
                    CircleGeometry::build(50.0, 20, 0.0, TAU)?,
                    RingGeometry::build(10.0, 50.0, 20, 5, 0.0, TAU)?,
                    CylinderGeometry::build(25.0, 75.0, 100.0, 40, 5, false, 0.0, TAU)?,
                    LatheGeometry::build(&points, 20, 0.0, TAU)?,
                    TorusGeometry::build(50.0, 20.0, 20, 20, TAU, 0.0, TAU)?,
                    TorusKnotGeometry::build(50.0, 10.0, 50, 20, 2, 3)?,
                    CapsuleGeometry::build(20.0, 50.0, 4, 8, 1)?,
                    plane,
                    klein,
                    mobius,
                ];
                let mut shapes = Vec::new();
                for (i, geometry) in geometries.into_iter().enumerate() {
                    let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                        Arc::new(geometry),
                        material.clone(),
                    )));
                    scene.get_mut(mesh)?.position = Vector3::new(
                        (i % 4) as f64 * 200.0 - 300.0,
                        0.0,
                        300.0 - (i / 4) as f64 * 200.0,
                    );
                    if i == 14 {
                        scene.get_mut(mesh)?.scale = Vector3::splat(5.0);
                    }
                    if i == 15 {
                        scene.get_mut(mesh)?.scale = Vector3::splat(30.0);
                    }
                    shapes.push(mesh);
                }
                Ok(Self {
                    viewer: OrbitViewer::from_camera(Vector3::ZERO, 800.0),
                    near: 1.0,
                    far: 2000.0,
                    content: Content::Geometries(shapes),
                    elapsed: 0.0,
                })
            }
            18 => {
                scene.background = Color::from_hex(0x8fbcd4);
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 1.0,
                        far: 20.0,
                        ..Default::default()
                    }));
                scene.insert(NodeKind::Light(Light::Ambient {
                    color: Color::from_hex(0x8fbcd4),
                    intensity: 1.5,
                }));
                let light = scene.insert(NodeKind::Light(Light::Point {
                    color: Color::WHITE,
                    intensity: 200.0,
                    distance: 0.0,
                    decay: 2.0,
                }));
                scene.add(camera, light)?;
                let mut geometry = BoxGeometry::segmented(2.0, 2.0, 2.0, 32, 32, 32)?;
                let mut sphere = Vec::new();
                let mut twist = Vec::new();
                for p in geometry.positions()? {
                    let Vector3 { x, y, z } = p;
                    sphere.extend(
                        [
                            x * (1.0 - y * y / 2.0 - z * z / 2.0 + y * y * z * z / 3.0).sqrt(),
                            y * (1.0 - z * z / 2.0 - x * x / 2.0 + z * z * x * x / 3.0).sqrt(),
                            z * (1.0 - x * x / 2.0 - y * y / 2.0 + x * x * y * y / 3.0).sqrt(),
                        ]
                        .map(|v| v as f32),
                    );
                    let q = Quaternion::from_axis_angle(Vector3::X, std::f64::consts::PI * x / 2.0);
                    twist.extend(
                        (q * Vector3::new(x * 2.0, y, z))
                            .to_array()
                            .map(|v| v as f32),
                    );
                }
                geometry.morph_attributes.insert(
                    "position".into(),
                    vec![
                        Attribute::F32(crate::attribute::BufferAttribute::new(sphere, 3, false)?),
                        Attribute::F32(crate::attribute::BufferAttribute::new(twist, 3, false)?),
                    ],
                );
                let mut material = MeshPhongMaterial::default();
                material.properties.color = Color::from_hex(0xff0000);
                material.properties.flat_shading = true;
                let mesh = scene.insert(NodeKind::Mesh(Mesh::new(
                    Arc::new(geometry),
                    Arc::new(Material::Phong(material)),
                )));
                scene.get_mut(mesh)?.morph_weights = vec![0.0, 0.0];
                Ok(Self {
                    viewer: OrbitViewer::from_camera(Vector3::ZERO, 10.0),
                    near: 1.0,
                    far: 20.0,
                    content: Content::Morph(mesh),
                    elapsed: 0.0,
                })
            }
            19 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 150.0),
                near: 1.0,
                far: 200.0,
                content: Content::Dashed(super::expanded_lines::dashed(scene, camera, aspect)?),
                elapsed: 0.0,
            }),
            20 => {
                scene.get_mut(camera)?.kind =
                    NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
                        fov: 45.0,
                        aspect,
                        near: 1.0,
                        far: 1000.0,
                        ..Default::default()
                    }));
                let center = Vector3::new(0.0, 5.5, 0.0);
                let offset = Vector3::new(0.0, 5.0, -15.0) - center;
                let mut viewer = OrbitViewer::from_camera(center, offset.length());
                viewer.fixture(
                    offset.x.atan2(offset.z),
                    (offset.y / offset.length()).asin(),
                    1.8,
                );
                Ok(Self {
                    viewer,
                    near: 1.0,
                    far: 1000.0,
                    content: Content::AreaLights(super::expanded_lights::rect_area(scene)?),
                    elapsed: 0.0,
                })
            }
            24 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 600.0),
                near: 1.0,
                far: 10000.0,
                content: Content::Horse(
                    super::expanded_morph_models::Horse::create(scene, camera, aspect).await?,
                ),
                elapsed: 0.0,
            }),
            25 => {
                let mut viewer = OrbitViewer::from_camera(Vector3::ZERO, 50.0_f64.sqrt());
                viewer.fixture(0.0, std::f64::consts::FRAC_PI_4, 1.8);
                Ok(Self {
                    viewer,
                    near: 0.2,
                    far: 100.0,
                    content: Content::Sphere(
                        super::expanded_morph_models::Sphere::create(scene, camera, aspect).await?,
                    ),
                    elapsed: 0.0,
                })
            }
            23 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 1000.0),
                near: 1.0,
                far: 10000.0,
                content: Content::ColorLines {
                    objects: super::expanded_lines::colors(scene, camera, aspect)?,
                    pointer: Vector2::ZERO,
                },
                elapsed: 0.0,
            }),
            22 => Ok(Self {
                viewer: OrbitViewer::from_camera(Vector3::ZERO, 64.0),
                near: 1.0,
                far: 3500.0,
                content: Content::Indexed(super::expanded_indexed::create(scene, camera, aspect)?),
                elapsed: 0.0,
            }),
            21 => {
                super::expanded_geometry_colors::create(scene, camera, aspect)?;
                Ok(Self {
                    viewer: OrbitViewer::from_camera(Vector3::ZERO, 1800.0),
                    near: 1.0,
                    far: 10000.0,
                    content: Content::VertexColors {
                        pointer: Vector2::ZERO,
                    },
                    elapsed: 0.0,
                })
            }
            _ => Err(crate::Error::Invalid("expanded example id")),
        }
    }
    pub fn update(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        delta: f64,
        animate: bool,
    ) -> Result<()> {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "exporters_video")]
        if let Content::ExportersVideo(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "texture_volumes")]
        if let Content::TextureVolumes(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::ColladaAnim(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "threemf")]
        if let Content::ThreeMf(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "gltf_normals")]
        if let Content::GltfNormals(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "raycaster_helper")]
        if let Content::RaycasterHelper(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "audio_timing")]
        if let Content::AudioTiming(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "audio_visual")]
        if let Content::AudioVisual(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "md2")]
        if let Content::Md2(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "md2_control")]
        if let Content::Md2Control(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "envmap_exr")]
        if let Content::EnvmapExr(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "dds")]
        if let Content::Dds(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "kinect")]
        if let Content::Kinect(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "channels")]
        if let Content::Channels(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "wide_gamut")]
        if let Content::WideGamut(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "uv_tests")]
        if let Content::UvTests(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "clipping_stencil")]
        if let Content::ClippingStencil(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ascii")]
        if let Content::Ascii(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lightprobes_sponza")]
        if let Content::LightprobesSponza(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "svg_sandbox")]
        if let Content::SvgSandbox(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "svg_lines")]
        if let Content::SvgLines(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "css3d_mixed")]
        if let Content::Css3dMixed(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "css3d_molecules")]
        if let Content::Css3dMolecules(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "css3d_periodictable")]
        if let Content::Css3dPeriodictable(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "css3d_sprites")]
        if let Content::Css3dSprites(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "css3d_orthographic")]
        if let Content::Css3dOrthographic(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "css3d_sandbox")]
        if let Content::Css3dSandbox(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "css2d_label")]
        if let Content::Css2dLabel(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "animation_pointer")]
        if let Content::AnimationPointer(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "envmaps_fasthdr")]
        if let Content::EnvmapsFasthdr(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_rapier_vehicle_controller")]
        if let Content::PhysicsRapierVehicleController(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_rapier_character_controller")]
        if let Content::PhysicsRapierCharacterController(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_rapier_terrain")]
        if let Content::PhysicsRapierTerrain(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_rapier_joints")]
        if let Content::PhysicsRapierJoints(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_rapier_instancing")]
        if let Content::PhysicsRapierInstancing(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_ammo_instancing")]
        if let Content::PhysicsAmmoInstancing(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_jolt_instancing")]
        if let Content::PhysicsJoltInstancing(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_ammo_rope")]
        if let Content::PhysicsAmmoRope(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_ammo_cloth")]
        if let Content::PhysicsAmmoCloth(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_ammo_volume")]
        if let Content::PhysicsAmmoVolume(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_ammo_terrain")]
        if let Content::PhysicsAmmoTerrain(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_ammo_break")]
        if let Content::PhysicsAmmoBreak(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "html_texture")]
        if let Content::HtmlTexture(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "physics_rapier_basic")]
        if let Content::PhysicsRapierBasic(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "geometry_csg")]
        if let Content::GeometryCsg(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ldraw")]
        if let Content::Ldraw(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "batch_lod_bvh")]
        if let Content::BatchLodBvh(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "offscreen")]
        if let Content::Offscreen(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lottie")]
        if let Content::Lottie(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "watch")]
        if let Content::Watch(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ifc_loader")]
        if let Content::IfcLoader(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "loader_3dm")]
        if let Content::Loader3dm(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "webcam")]
        if let Content::Webcam(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "transmission_alpha")]
        if let Content::TransmissionAlpha(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "random_uv")]
        if let Content::RandomUv(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "games_fps")]
        if let Content::GamesFps(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "fbx_loader")]
        if let Content::FbxLoader(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "svg_loader")]
        if let Content::SvgLoader(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "exporter_gcode")]
        if let Content::ExporterGcode(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "video_frame")]
        if let Content::VideoFrame(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "arcball")]
        if let Content::Arcball(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "exporter_gltf")]
        if let Content::ExporterGltf(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "spline_editor")]
        if let Content::SplineEditor(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "raycaster_bvh")]
        if let Content::RaycasterBvh(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "subdivision")]
        if let Content::Subdivision(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_performance")]
        if let Content::ShadowmapPerformance(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "usdz_exporter")]
        if let Content::UsdzExporter(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "morphtargets_webcam")]
        if let Content::MorphtargetsWebcam(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_pcss")]
        if let Content::ShadowmapPcss(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "fbx_nurbs")]
        if let Content::FbxNurbs(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ies_spotlight")]
        if let Content::IesSpotlight(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "marching_cubes")]
        if let Content::MarchingCubes(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "test_memory")]
        if let Content::TestMemory(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ktx")]
        if let Content::Ktx(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "pvr")]
        if let Content::Pvr(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "lines_raycast")]
        if let Content::LinesRaycast(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "spot_skinning")]
        if let Content::SpotSkinning(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "draco_variants")]
        if let Content::DracoVariants(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "probes_hdr")]
        if let Content::ProbesHdr(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "exporters_matcap")]
        if let Content::ExportersMatcap(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shapes_lights")]
        if let Content::ShapesLights(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::RefractionLoaders(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "helpers_formats")]
        if let Content::HelpersFormats(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "models_modifiers")]
        if let Content::ModelsModifiers(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "interactive_objects")]
        if let Content::InteractiveObjects(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "interactive_shaders")]
        if let Content::InteractiveShaders(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "environment_materials")]
        if let Content::EnvironmentMaterials(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "geometry_materials")]
        if let Content::GeometryMaterials(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shader_geometry")]
        if let Content::ShaderGeometry(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "point_clouds")]
        if let Content::PointClouds(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "buffer_particles")]
        if let Content::BufferParticles(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "shapes")]
        if let Content::Shapes(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "material_textures")]
        if let Content::MaterialTextures(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_procedural")]
        if let Content::TslProcedural(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_primitives")]
        if let Content::TslPrimitives(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_materials")]
        if let Content::TslMaterials(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_viewport")]
        if let Content::TslViewport(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_lighting")]
        if let Content::TslLighting(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_environment")]
        if let Content::TslEnvironment(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_next")]
        if let Content::TslNext(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_particles")]
        if let Content::TslParticles(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_filters")]
        if let Content::TslFilter(demo) = &mut self.content {
            return demo.update(scene, delta, animate);
        }
        #[cfg(feature = "tsl_passes")]
        if let Content::TslPass(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        #[cfg(feature = "tsl_examples")]
        if let Content::Tsl(demo) = &mut self.content {
            return demo.update(scene, delta, animate);
        }
        #[cfg(feature = "gltf_examples")]
        if let Content::Gltf(demo) = &mut self.content {
            return demo.update(scene, camera, delta, animate);
        }
        if let Content::Triangles { objects, raw } = &self.content {
            if animate {
                self.elapsed += delta;
            }
            return super::expanded_triangles::update(scene, objects, self.elapsed, *raw);
        }
        if let Content::Horse(horse) = &mut self.content {
            if animate {
                self.elapsed += delta;
            }
            return horse.update(scene, camera, self.elapsed, animate);
        }
        if let Content::Sphere(sphere) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            sphere.update(scene, self.elapsed)?;
        }
        if let Content::ColorLines { objects, pointer } = &self.content {
            if animate {
                self.elapsed += delta;
                let p = &mut scene.get_mut(camera)?.position;
                p.x += (pointer.x - p.x) * 0.05;
                p.y += (-pointer.y + 200.0 - p.y) * 0.05;
            }
            scene.look_at(camera, Vector3::ZERO)?;
            for (i, &object) in objects.iter().enumerate() {
                scene.get_mut(object)?.quaternion = Quaternion::from_rotation_y(
                    self.elapsed * 0.5 * if i % 2 == 1 { 1.0 } else { -1.0 },
                );
            }
            return Ok(());
        }
        if let Content::VertexColors { pointer } = &self.content {
            if animate {
                let p = &mut scene.get_mut(camera)?.position;
                p.x += (pointer.x - p.x) * 0.05;
                p.y += (-pointer.y - p.y) * 0.05;
            }
            scene.look_at(camera, Vector3::ZERO)?;
            return Ok(());
        }
        if let Content::Indexed(mesh) = self.content {
            if animate {
                self.elapsed += delta;
            }
            scene.get_mut(mesh)?.quaternion = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                self.elapsed * 0.25,
                self.elapsed * 0.5,
                0.0,
            );
            return Ok(());
        }
        if let Content::Dashed(objects) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            for &object in objects {
                scene.get_mut(object)?.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.elapsed * 0.25,
                    self.elapsed * 0.25,
                    0.0,
                );
            }
            return Ok(());
        }
        if let Content::Geometries(shapes) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            let t = self.elapsed * 0.1;
            scene.get_mut(camera)?.position = Vector3::new(t.cos() * 800.0, 500.0, t.sin() * 800.0);
            scene.look_at(camera, Vector3::ZERO)?;
            for &mesh in shapes {
                scene.get_mut(mesh)?.quaternion =
                    Quaternion::from_euler(glam::EulerRot::XYZ, t * 5.0, t * 2.5, 0.0);
            }
            return Ok(());
        }
        if let Content::AreaLights(lights) = &self.content {
            if animate {
                self.elapsed += delta;
            }
            for (&light, speed) in lights.iter().zip([-1.0, 0.5, 1.0]) {
                scene.get_mut(light)?.quaternion =
                    Quaternion::from_rotation_y(self.elapsed * speed);
            }
        }
        self.viewer.update(scene, camera)?;
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(camera)?.kind {
            c.near = self.near;
            c.far = self.far;
        }
        Ok(())
    }
    pub fn wireframe(&mut self, scene: &mut Scene, enabled: bool) -> Result<()> {
        if let Content::Indexed(mesh) = self.content {
            if let NodeKind::Mesh(mesh) = &mut scene.get_mut(mesh)?.kind {
                Arc::make_mut(&mut mesh.materials[0])
                    .properties_mut()
                    .wireframe = enabled;
            }
            Ok(())
        } else {
            Err(crate::Error::Invalid("indexed example"))
        }
    }
    pub fn morph(&mut self, scene: &mut Scene, weights: [f64; 2]) -> Result<()> {
        if let Content::Morph(mesh) = self.content {
            scene.get_mut(mesh)?.morph_weights.copy_from_slice(&weights);
            Ok(())
        } else {
            Err(crate::Error::Invalid("morph example"))
        }
    }
    pub fn select(&mut self, scene: &mut Scene, cam: Object3D, x: f64, y: f64) -> Result<()> {
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &mut self.content {
            return demo.select(scene, cam, x, y);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            return demo.select(scene, cam, x, y);
        }
        #[cfg(feature = "tsl_lighting")]
        if let Content::TslLighting(demo) = &mut self.content {
            demo.select(scene, cam, x, y)?;
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            demo.select(scene, cam, x, y)?;
        }
        Ok(())
    }
    pub fn gpu_pointer(
        &mut self,
        r: &crate::renderer::Renderer,
        scene: &mut Scene,
        cam: Object3D,
        x: f64,
        y: f64,
    ) -> Result<bool> {
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            return demo.gpu_pointer(x, y);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            return demo.gpu_pointer(scene, cam, x, y);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::RefractionLoaders(demo) = &mut self.content {
            demo.gpu_pointer(x, y);
            return Ok(false);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            demo.gpu_pointer(x, y);
            return Ok(false);
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            demo.pointer_move(scene, cam, x, y)?;
            return Ok(false);
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            demo.gpu_pointer(x, y);
            return Ok(false);
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &mut self.content {
            demo.gpu_pointer(x, y);
            return Ok(false);
        }
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &mut self.content {
            demo.gpu_pointer(x, y);
            return Ok(false);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            return demo.gpu_pointer(scene, cam, x, y);
        }
        #[cfg(feature = "interactive_objects")]
        if let Content::InteractiveObjects(demo) = &mut self.content {
            demo.gpu_pointer(scene, cam, x, y)?;
        }
        #[cfg(feature = "interactive_shaders")]
        if let Content::InteractiveShaders(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        #[cfg(feature = "material_textures")]
        if let Content::MaterialTextures(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        #[cfg(feature = "tsl_procedural")]
        if let Content::TslProcedural(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            demo.gpu_pointer(r, scene, cam, x, y)?;
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            demo.gpu_pointer(scene, cam, x, y)?;
        }
        Ok(false)
    }
    pub fn pointer(&mut self, x: f64, y: f64) {
        #[cfg(feature = "environment_materials")]
        if let Content::EnvironmentMaterials(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        #[cfg(feature = "point_clouds")]
        if let Content::PointClouds(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        #[cfg(feature = "tsl_particles")]
        if let Content::TslParticles(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        #[cfg(feature = "tsl_passes")]
        if let Content::TslPass(demo) = &mut self.content {
            demo.pointer(x, y);
        }
        if let Content::VertexColors { pointer } | Content::ColorLines { pointer, .. } =
            &mut self.content
        {
            *pointer = Vector2::new(x, y);
        }
    }
    pub fn dragging(&mut self, value: bool) {
        #[cfg(feature = "tsl_procedural")]
        if let Content::TslProcedural(demo) = &mut self.content {
            demo.dragging(value);
        }
        #[cfg(feature = "tsl_materials")]
        if let Content::TslMaterials(demo) = &mut self.content {
            demo.dragging(value);
        }
        #[cfg(feature = "tsl_viewport")]
        if let Content::TslViewport(demo) = &mut self.content {
            demo.dragging(value);
        }
        #[cfg(feature = "tsl_environment")]
        if let Content::TslEnvironment(demo) = &mut self.content {
            demo.dragging(value);
        }
        #[cfg(feature = "tsl_next")]
        if let Content::TslNext(demo) = &mut self.content {
            demo.dragging(value);
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            demo.dragging(value);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            demo.dragging(value);
        }
        #[cfg(feature = "gltf_examples")]
        if let Content::Gltf(demo) = &mut self.content {
            demo.dragging(value);
        }
    }
    pub fn render(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut Scene,
        camera: Object3D,
        target: &crate::renderer::RenderTarget,
    ) -> Result<bool> {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "material_textures")]
        if let Content::MaterialTextures(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_procedural")]
        if let Content::TslProcedural(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_primitives")]
        if let Content::TslPrimitives(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_materials")]
        if let Content::TslMaterials(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_viewport")]
        if let Content::TslViewport(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_lighting")]
        if let Content::TslLighting(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_environment")]
        if let Content::TslEnvironment(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_next")]
        if let Content::TslNext(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_particles")]
        if let Content::TslParticles(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_filters")]
        if let Content::TslFilter(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        #[cfg(feature = "tsl_passes")]
        if let Content::TslPass(demo) = &mut self.content {
            return demo.render(renderer, scene, camera, target);
        }
        Ok(false)
    }
    pub fn output_target(&self) -> Option<&crate::renderer::RenderTarget> {
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &self.content
            && let Some(target) = demo.output()
        {
            return Some(target);
        }
        #[cfg(feature = "material_textures")]
        if let Content::MaterialTextures(demo) = &self.content {
            return demo.output();
        }
        #[cfg(feature = "tsl_primitives")]
        if let Content::TslPrimitives(demo) = &self.content {
            return demo.output();
        }
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(demo) = &self.content {
            return demo.output_target();
        }
        #[cfg(feature = "tsl_particles")]
        if let Content::TslParticles(demo) = &self.content {
            return demo.output();
        }
        None
    }
    /// The target `output_target` returns, to render into the canvas ( see
    /// RenderTarget::alias_color ).
    pub fn output_target_mut(&mut self) -> Option<&mut crate::renderer::RenderTarget> {
        match &mut self.content {
            #[cfg(feature = "retro")]
            Content::Retro(demo) => demo.output_mut(),
            #[cfg(feature = "fluid")]
            Content::Fluid(demo) => demo.output_mut(),
            #[cfg(feature = "lightprobes")]
            Content::Lightprobes(demo) => demo.output_mut(),
            #[cfg(feature = "tsl_graph")]
            Content::TslGraph(demo) => demo.output_mut(),
            #[cfg(feature = "compute_water")]
            Content::ComputeWater(demo) => demo.output_mut(),
            #[cfg(feature = "ssgi")]
            Content::Ssgi(demo) => demo.output_mut(),
            #[cfg(feature = "ballpool")]
            Content::Ballpool(demo) => demo.output_mut(),
            #[cfg(feature = "pathtracer")]
            Content::Pathtracer(demo) => demo.output_mut(),
            #[cfg(feature = "tiles3d")]
            Content::Tiles3d(demo) => demo.output_mut(),
            #[cfg(feature = "sss")]
            Content::Sss(demo) => demo.output_mut(),
            #[cfg(feature = "ssr")]
            Content::Ssr(demo) => demo.output_mut(),
            #[cfg(feature = "retargeting")]
            Content::Retargeting(demo) => demo.output_mut(),
            #[cfg(feature = "gaussian_splat")]
            Content::GaussianSplat(demo) => demo.output_mut(),
            #[cfg(feature = "water")]
            Content::Water(demo) => demo.output_mut(),
            #[cfg(feature = "custom_fog")]
            Content::CustomFog(demo) => demo.output_mut(),
            #[cfg(feature = "vxgi")]
            Content::Vxgi(demo) => demo.output_mut(),
            #[cfg(feature = "vxgi_sponza")]
            Content::VxgiSponza(demo) => demo.output_mut(),
            #[cfg(feature = "gtao")]
            Content::Gtao(demo) => demo.output_mut(),
            #[cfg(feature = "compute_rasterizer")]
            Content::ComputeRasterizer(demo) => demo.output_mut(),
            #[cfg(feature = "compute_rasterizer_ibl")]
            Content::ComputeRasterizerIbl(demo) => demo.output_mut(),
            #[cfg(feature = "shadowmap_progressive")]
            Content::ShadowmapProgressive(demo) => demo.output_mut(),
            #[cfg(feature = "retargeting_readyplayer")]
            Content::RetargetingReadyplayer(demo) => demo.output_mut(),
            #[cfg(feature = "ssr_denoise")]
            Content::SsrDenoise(demo) => demo.output_mut(),
            #[cfg(feature = "simple_gi")]
            Content::SimpleGi(demo) => demo.output_mut(),
            #[cfg(feature = "dof2")]
            Content::Dof2(demo) => demo.output_mut(),
            #[cfg(feature = "subsurface")]
            Content::Subsurface(demo) => demo.output_mut(),
            #[cfg(feature = "birds_gltf")]
            Content::BirdsGltf(demo) => demo.output_mut(),
            #[cfg(feature = "vrml")]
            Content::Vrml(demo) => demo.output_mut(),
            #[cfg(feature = "usdz")]
            Content::Usdz(demo) => demo.output_mut(),
            #[cfg(feature = "gtao_webgl")]
            Content::GtaoWebgl(demo) => demo.output_mut(),
            #[cfg(feature = "generator_building")]
            Content::GeneratorBuilding(demo) => demo.output_mut(),
            #[cfg(feature = "geometry_loft")]
            Content::GeometryLoft(demo) => demo.output_mut(),
            #[cfg(feature = "sculpt")]
            Content::Sculpt(demo) => demo.output_mut(),
            #[cfg(feature = "shadow_array")]
            Content::ShadowArray(demo) => demo.output_mut(),
            #[cfg(feature = "skinning_instances")]
            Content::SkinningInstances(demo) => demo.output_mut(),
            #[cfg(feature = "fog_volume")]
            Content::FogVolume(demo) => demo.output_mut(),
            #[cfg(feature = "backdrop_water")]
            Content::BackdropWater(demo) => demo.output_mut(),
            #[cfg(feature = "volume_fire")]
            Content::VolumeFire(demo) => demo.output_mut(),
            #[cfg(feature = "generator_city")]
            Content::GeneratorCity(demo) => demo.output_mut(),
            #[cfg(feature = "compute_reduce")]
            Content::ComputeReduce(demo) => demo.output_mut(),
            #[cfg(feature = "loader_materialx")]
            Content::LoaderMaterialx(demo) => demo.output_mut(),
            #[cfg(feature = "fxaa_webgl")]
            Content::FxaaWebgl(demo) => demo.output_mut(),
            #[cfg(feature = "msaa_renderbuffers")]
            Content::MsaaRenderbuffers(demo) => demo.output_mut(),
            #[cfg(feature = "shadowmap_webgl")]
            Content::ShadowmapWebgl(demo) => demo.output_mut(),
            #[cfg(feature = "fog_scattering")]
            Content::FogScattering(demo) => demo.output_mut(),
            #[cfg(feature = "lightprobes")]
            Content::LightprobesComplex(demo) => demo.output_mut(),
            #[cfg(feature = "hdr")]
            Content::Hdr(demo) => demo.output_mut(),
            #[cfg(feature = "volume_caustics")]
            Content::VolumeCaustics(demo) => demo.output_mut(),
            #[cfg(feature = "compute_cloth")]
            Content::ComputeCloth(demo) => demo.output_mut(),
            #[cfg(feature = "deferred")]
            Content::Deferred(demo) => demo.output_mut(),
            #[cfg(feature = "cubemap_dynamic")]
            Content::CubemapDynamic(demo) => demo.output_mut(),
            #[cfg(feature = "gpgpu_protoplanet")]
            Content::GpgpuProtoplanet(demo) => demo.output_mut(),
            #[cfg(feature = "volume_traa")]
            Content::VolumeTraa(demo) => demo.output_mut(),
            #[cfg(feature = "volume_rectarea")]
            Content::VolumeRectarea(demo) => demo.output_mut(),
            #[cfg(feature = "memory_outline")]
            Content::MemoryOutline(demo) => demo.output_mut(),
            #[cfg(feature = "godrays")]
            Content::Godrays(demo) => demo.output_mut(),
            #[cfg(feature = "volume_lighting")]
            Content::VolumeLighting(demo) => demo.output_mut(),
            #[cfg(feature = "instancing_morph")]
            Content::InstancingMorph(demo) => demo.output_mut(),
            #[cfg(feature = "shadowmap_csm")]
            Content::ShadowmapCsm(demo) => demo.output_mut(),
            #[cfg(feature = "caustics")]
            Content::Caustics(demo) => demo.output_mut(),
            #[cfg(feature = "shadowmap_opacity")]
            Content::ShadowmapOpacity(demo) => demo.output_mut(),
            #[cfg(feature = "lights_projector")]
            Content::LightsProjector(demo) => demo.output_mut(),
            #[cfg(feature = "shadowmap_vsm")]
            Content::ShadowmapVsm(demo) => demo.output_mut(),
            #[cfg(feature = "lights_dynamic")]
            Content::LightsDynamic(demo) => demo.output_mut(),
            #[cfg(feature = "shadowmap_viewer")]
            Content::ShadowmapViewer(demo) => demo.output_mut(),
            #[cfg(feature = "ktx2")]
            Content::Ktx2(demo) => demo.output_mut(),
            #[cfg(feature = "elements_text")]
            Content::ElementsText(demo) => demo.output_mut(),
            #[cfg(feature = "test_memory2")]
            Content::TestMemory2(demo) => demo.output_mut(),
            #[cfg(feature = "lights_clustered")]
            Content::LightsClustered(demo) => demo.output_mut(),
            #[cfg(feature = "linked_particles")]
            Content::LinkedParticles(demo) => demo.output_mut(),
            #[cfg(feature = "attractors")]
            Content::Attractors(demo) => demo.output_mut(),
            #[cfg(feature = "sort_bitonic")]
            Content::SortBitonic(demo) => demo.output_mut(),
            #[cfg(feature = "compute_birds")]
            Content::ComputeBirds(demo) => demo.output_mut(),
            #[cfg(feature = "points_dynamic")]
            Content::PointsDynamic(demo) => demo.output_mut(),
            #[cfg(feature = "postprocessing_advanced")]
            Content::PostprocessingAdvanced(demo) => demo.output_mut(),
            #[cfg(feature = "render_bundle")]
            Content::RenderBundle(demo) => demo.output_mut(),
            #[cfg(feature = "mesh_batch")]
            Content::MeshBatch(demo) => demo.output_mut(),
            #[cfg(feature = "volume_instancing")]
            Content::VolumeInstancing(demo) => demo.output_mut(),
            #[cfg(feature = "log_depth")]
            Content::LogDepth(demo) => demo.output_mut(),
            #[cfg(feature = "reversed_depth")]
            Content::ReversedDepth(demo) => demo.output_mut(),
            #[cfg(feature = "shadow_rtt")]
            Content::ShadowRtt(demo) => demo.output_mut(),
            #[cfg(feature = "selection_views")]
            Content::SelectionViews(demo) => demo.output_mut(),
            #[cfg(feature = "picking_buffers")]
            Content::PickingBuffers(demo) => demo.output_mut(),
            #[cfg(feature = "trackball_sprites")]
            Content::TrackballSprites(demo) => demo.output_mut(),
            #[cfg(feature = "controls_attributes")]
            Content::ControlsAttributes(demo) => demo.output_mut(),
            #[cfg(feature = "stereo_loaders")]
            Content::StereoLoaders(demo) => demo.output_mut(),
            #[cfg(feature = "views_loaders")]
            Content::ViewsLoaders(demo) => demo.output_mut(),
            #[cfg(feature = "interactive_scenes")]
            Content::InteractiveScenes(demo) => demo.output_mut(),
            #[cfg(feature = "material_textures")]
            Content::MaterialTextures(demo) => demo.output_mut(),
            #[cfg(feature = "tsl_primitives")]
            Content::TslPrimitives(demo) => demo.output_mut(),
            #[cfg(feature = "tsl_extended")]
            Content::TslExtended(demo) => demo.output_target_mut(),
            #[cfg(feature = "tsl_particles")]
            Content::TslParticles(demo) => demo.output_mut(),
            _ => None,
        }
    }
    pub fn prepare(
        &mut self,
        renderer: &crate::renderer::Renderer,
        scene: &mut Scene,
        camera: Object3D,
        aspect: f64,
    ) -> Result<()> {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "exporters_video")]
        if let Content::ExportersVideo(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera)?;
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "texture_volumes")]
        if let Content::TextureVolumes(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::ColladaAnim(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "threemf")]
        if let Content::ThreeMf(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "gltf_normals")]
        if let Content::GltfNormals(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "raycaster_helper")]
        if let Content::RaycasterHelper(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "audio_timing")]
        if let Content::AudioTiming(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "audio_visual")]
        if let Content::AudioVisual(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "md2")]
        if let Content::Md2(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "md2_control")]
        if let Content::Md2Control(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "envmap_exr")]
        if let Content::EnvmapExr(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "dds")]
        if let Content::Dds(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "kinect")]
        if let Content::Kinect(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "channels")]
        if let Content::Channels(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "wide_gamut")]
        if let Content::WideGamut(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "uv_tests")]
        if let Content::UvTests(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "clipping_stencil")]
        if let Content::ClippingStencil(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ascii")]
        if let Content::Ascii(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lightprobes_sponza")]
        if let Content::LightprobesSponza(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "svg_sandbox")]
        if let Content::SvgSandbox(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "svg_lines")]
        if let Content::SvgLines(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "css3d_mixed")]
        if let Content::Css3dMixed(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "css3d_molecules")]
        if let Content::Css3dMolecules(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "css3d_periodictable")]
        if let Content::Css3dPeriodictable(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "css3d_sprites")]
        if let Content::Css3dSprites(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "css3d_orthographic")]
        if let Content::Css3dOrthographic(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "css3d_sandbox")]
        if let Content::Css3dSandbox(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "css2d_label")]
        if let Content::Css2dLabel(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "animation_pointer")]
        if let Content::AnimationPointer(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "envmaps_fasthdr")]
        if let Content::EnvmapsFasthdr(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_rapier_vehicle_controller")]
        if let Content::PhysicsRapierVehicleController(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_rapier_character_controller")]
        if let Content::PhysicsRapierCharacterController(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_rapier_terrain")]
        if let Content::PhysicsRapierTerrain(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_rapier_joints")]
        if let Content::PhysicsRapierJoints(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_rapier_instancing")]
        if let Content::PhysicsRapierInstancing(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_ammo_instancing")]
        if let Content::PhysicsAmmoInstancing(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_jolt_instancing")]
        if let Content::PhysicsJoltInstancing(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_ammo_rope")]
        if let Content::PhysicsAmmoRope(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_ammo_cloth")]
        if let Content::PhysicsAmmoCloth(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_ammo_volume")]
        if let Content::PhysicsAmmoVolume(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_ammo_terrain")]
        if let Content::PhysicsAmmoTerrain(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_ammo_break")]
        if let Content::PhysicsAmmoBreak(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "html_texture")]
        if let Content::HtmlTexture(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "physics_rapier_basic")]
        if let Content::PhysicsRapierBasic(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "geometry_csg")]
        if let Content::GeometryCsg(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ldraw")]
        if let Content::Ldraw(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "batch_lod_bvh")]
        if let Content::BatchLodBvh(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "offscreen")]
        if let Content::Offscreen(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lottie")]
        if let Content::Lottie(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "watch")]
        if let Content::Watch(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ifc_loader")]
        if let Content::IfcLoader(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "loader_3dm")]
        if let Content::Loader3dm(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "webcam")]
        if let Content::Webcam(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "transmission_alpha")]
        if let Content::TransmissionAlpha(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "random_uv")]
        if let Content::RandomUv(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "games_fps")]
        if let Content::GamesFps(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "fbx_loader")]
        if let Content::FbxLoader(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "svg_loader")]
        if let Content::SvgLoader(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "exporter_gcode")]
        if let Content::ExporterGcode(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "video_frame")]
        if let Content::VideoFrame(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "arcball")]
        if let Content::Arcball(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "exporter_gltf")]
        if let Content::ExporterGltf(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "spline_editor")]
        if let Content::SplineEditor(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "raycaster_bvh")]
        if let Content::RaycasterBvh(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "subdivision")]
        if let Content::Subdivision(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_performance")]
        if let Content::ShadowmapPerformance(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "usdz_exporter")]
        if let Content::UsdzExporter(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "morphtargets_webcam")]
        if let Content::MorphtargetsWebcam(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_pcss")]
        if let Content::ShadowmapPcss(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "fbx_nurbs")]
        if let Content::FbxNurbs(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ies_spotlight")]
        if let Content::IesSpotlight(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "marching_cubes")]
        if let Content::MarchingCubes(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "test_memory")]
        if let Content::TestMemory(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ktx")]
        if let Content::Ktx(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "pvr")]
        if let Content::Pvr(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "lines_raycast")]
        if let Content::LinesRaycast(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "spot_skinning")]
        if let Content::SpotSkinning(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "draco_variants")]
        if let Content::DracoVariants(demo) = &mut self.content {
            demo.prepare(scene, camera, renderer)?;
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "probes_hdr")]
        if let Content::ProbesHdr(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera)?;
        }
        #[cfg(feature = "exporters_matcap")]
        if let Content::ExportersMatcap(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera)?;
        }
        #[cfg(feature = "shapes_lights")]
        if let Content::ShapesLights(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::RefractionLoaders(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "helpers_formats")]
        if let Content::HelpersFormats(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera)?;
        }
        #[cfg(feature = "models_modifiers")]
        if let Content::ModelsModifiers(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera)?;
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera)?;
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            demo.prepare(scene, camera)?;
        }
        #[cfg(feature = "interactive_objects")]
        if let Content::InteractiveObjects(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera, aspect)?;
        }
        #[cfg(feature = "interactive_shaders")]
        if let Content::InteractiveShaders(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera, aspect)?;
        }
        #[cfg(feature = "environment_materials")]
        if let Content::EnvironmentMaterials(demo) = &mut self.content {
            demo.prepare(scene, camera, aspect)?;
        }
        #[cfg(feature = "buffer_particles")]
        if let Content::BufferParticles(demo) = &mut self.content {
            demo.prepare(renderer)?;
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            demo.prepare(scene, camera, aspect)?;
        }
        #[cfg(feature = "tsl_examples")]
        if let Content::Tsl(demo) = &mut self.content {
            demo.prepare(renderer, scene, camera, aspect)?;
        }
        Ok(())
    }
    pub fn tsl_parameter(&mut self, index: usize, value: f32) -> Result<()> {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "exporters_video")]
        if let Content::ExportersVideo(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "texture_volumes")]
        if let Content::TextureVolumes(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::ColladaAnim(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "threemf")]
        if let Content::ThreeMf(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "gltf_normals")]
        if let Content::GltfNormals(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "raycaster_helper")]
        if let Content::RaycasterHelper(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "audio_timing")]
        if let Content::AudioTiming(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "audio_visual")]
        if let Content::AudioVisual(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "md2")]
        if let Content::Md2(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "md2_control")]
        if let Content::Md2Control(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "envmap_exr")]
        if let Content::EnvmapExr(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "dds")]
        if let Content::Dds(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "kinect")]
        if let Content::Kinect(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "channels")]
        if let Content::Channels(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "wide_gamut")]
        if let Content::WideGamut(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "uv_tests")]
        if let Content::UvTests(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "clipping_stencil")]
        if let Content::ClippingStencil(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ascii")]
        if let Content::Ascii(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lightprobes_sponza")]
        if let Content::LightprobesSponza(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "svg_sandbox")]
        if let Content::SvgSandbox(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "svg_lines")]
        if let Content::SvgLines(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "css3d_mixed")]
        if let Content::Css3dMixed(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "css3d_molecules")]
        if let Content::Css3dMolecules(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "css3d_periodictable")]
        if let Content::Css3dPeriodictable(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "css3d_sprites")]
        if let Content::Css3dSprites(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "css3d_orthographic")]
        if let Content::Css3dOrthographic(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "css3d_sandbox")]
        if let Content::Css3dSandbox(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "css2d_label")]
        if let Content::Css2dLabel(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "animation_pointer")]
        if let Content::AnimationPointer(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "envmaps_fasthdr")]
        if let Content::EnvmapsFasthdr(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_rapier_vehicle_controller")]
        if let Content::PhysicsRapierVehicleController(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_rapier_character_controller")]
        if let Content::PhysicsRapierCharacterController(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_rapier_terrain")]
        if let Content::PhysicsRapierTerrain(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_rapier_joints")]
        if let Content::PhysicsRapierJoints(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_rapier_instancing")]
        if let Content::PhysicsRapierInstancing(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_ammo_instancing")]
        if let Content::PhysicsAmmoInstancing(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_jolt_instancing")]
        if let Content::PhysicsJoltInstancing(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_ammo_rope")]
        if let Content::PhysicsAmmoRope(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_ammo_cloth")]
        if let Content::PhysicsAmmoCloth(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_ammo_volume")]
        if let Content::PhysicsAmmoVolume(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_ammo_terrain")]
        if let Content::PhysicsAmmoTerrain(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_ammo_break")]
        if let Content::PhysicsAmmoBreak(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "html_texture")]
        if let Content::HtmlTexture(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "physics_rapier_basic")]
        if let Content::PhysicsRapierBasic(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "geometry_csg")]
        if let Content::GeometryCsg(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ldraw")]
        if let Content::Ldraw(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "batch_lod_bvh")]
        if let Content::BatchLodBvh(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "offscreen")]
        if let Content::Offscreen(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lottie")]
        if let Content::Lottie(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "watch")]
        if let Content::Watch(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ifc_loader")]
        if let Content::IfcLoader(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "loader_3dm")]
        if let Content::Loader3dm(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "webcam")]
        if let Content::Webcam(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "transmission_alpha")]
        if let Content::TransmissionAlpha(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "random_uv")]
        if let Content::RandomUv(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "games_fps")]
        if let Content::GamesFps(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "fbx_loader")]
        if let Content::FbxLoader(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "svg_loader")]
        if let Content::SvgLoader(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "exporter_gcode")]
        if let Content::ExporterGcode(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "video_frame")]
        if let Content::VideoFrame(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "arcball")]
        if let Content::Arcball(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "exporter_gltf")]
        if let Content::ExporterGltf(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "spline_editor")]
        if let Content::SplineEditor(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "raycaster_bvh")]
        if let Content::RaycasterBvh(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "subdivision")]
        if let Content::Subdivision(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_performance")]
        if let Content::ShadowmapPerformance(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "usdz_exporter")]
        if let Content::UsdzExporter(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "morphtargets_webcam")]
        if let Content::MorphtargetsWebcam(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_pcss")]
        if let Content::ShadowmapPcss(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "fbx_nurbs")]
        if let Content::FbxNurbs(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ies_spotlight")]
        if let Content::IesSpotlight(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "marching_cubes")]
        if let Content::MarchingCubes(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "test_memory")]
        if let Content::TestMemory(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ktx")]
        if let Content::Ktx(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "pvr")]
        if let Content::Pvr(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "lines_raycast")]
        if let Content::LinesRaycast(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "spot_skinning")]
        if let Content::SpotSkinning(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "draco_variants")]
        if let Content::DracoVariants(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "probes_hdr")]
        if let Content::ProbesHdr(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "exporters_matcap")]
        if let Content::ExportersMatcap(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shapes_lights")]
        if let Content::ShapesLights(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::RefractionLoaders(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "helpers_formats")]
        if let Content::HelpersFormats(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "models_modifiers")]
        if let Content::ModelsModifiers(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "interactive_objects")]
        if let Content::InteractiveObjects(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "interactive_shaders")]
        if let Content::InteractiveShaders(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "environment_materials")]
        if let Content::EnvironmentMaterials(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "geometry_materials")]
        if let Content::GeometryMaterials(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shader_geometry")]
        if let Content::ShaderGeometry(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "point_clouds")]
        if let Content::PointClouds(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "buffer_particles")]
        if let Content::BufferParticles(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "shapes")]
        if let Content::Shapes(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "material_textures")]
        if let Content::MaterialTextures(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_procedural")]
        if let Content::TslProcedural(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_primitives")]
        if let Content::TslPrimitives(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_materials")]
        if let Content::TslMaterials(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_viewport")]
        if let Content::TslViewport(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_lighting")]
        if let Content::TslLighting(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_environment")]
        if let Content::TslEnvironment(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_next")]
        if let Content::TslNext(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_particles")]
        if let Content::TslParticles(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_filters")]
        if let Content::TslFilter(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_passes")]
        if let Content::TslPass(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        #[cfg(feature = "tsl_examples")]
        if let Content::Tsl(demo) = &mut self.content {
            return demo.parameter(index, value);
        }
        Err(crate::Error::Invalid("not a TSL example"))
    }
    /// webaudio_timing's listener, sources and pending plays.
    pub fn audio_frame(
        &mut self,
        scene: &mut crate::scene::Scene,
        camera: crate::scene::Object3D,
    ) -> crate::Result<Vec<f32>> {
        match &mut self.content {
            #[cfg(feature = "audio_timing")]
            Content::AudioTiming(demo) => demo.audio_frame(scene, camera),
            #[cfg(feature = "audio_visual")]
            Content::AudioVisual(demo) => demo.audio_frame(scene, camera),
            _ => Err(crate::Error::Invalid("not a positional audio example")),
        }
    }
    /// webaudio_visualizer's analyser bins.
    pub fn audio_data(&mut self, bytes: &[u8]) -> crate::Result<()> {
        match &mut self.content {
            #[cfg(feature = "audio_visual")]
            Content::AudioVisual(demo) => demo.audio_data(bytes),
            _ => Err(crate::Error::Invalid("not an analyser example")),
        }
    }
    /// A file produced by an exporter button, taken once.
    pub fn take_export(
        &mut self,
        renderer: &crate::renderer::Renderer,
    ) -> Option<(String, Vec<u8>)> {
        match &mut self.content {
            #[cfg(feature = "exporters_matcap")]
            Content::ExportersMatcap(demo) => demo.take_export(),
            #[cfg(feature = "gltf_normals")]
            Content::GltfNormals(demo) => demo.take_export(),
            #[cfg(feature = "raycaster_helper")]
            Content::RaycasterHelper(demo) => demo.take_export(),
            #[cfg(feature = "exporters_video")]
            Content::ExportersVideo(demo) => demo.take_export(renderer),
            #[cfg(feature = "pathtracer")]
            Content::Pathtracer(demo) => demo.take_export(),
            _ => None,
        }
    }
    pub fn status(&self) -> String {
        #[cfg(feature = "buffer_particles")]
        if let Content::BufferParticles(demo) = &self.content {
            return demo.status();
        }
        String::new()
    }
    pub fn key(&mut self, code: u32, down: bool) {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "exporters_video")]
        if let Content::ExportersVideo(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "texture_volumes")]
        if let Content::TextureVolumes(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::ColladaAnim(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "threemf")]
        if let Content::ThreeMf(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "gltf_normals")]
        if let Content::GltfNormals(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "raycaster_helper")]
        if let Content::RaycasterHelper(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "audio_timing")]
        if let Content::AudioTiming(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "audio_visual")]
        if let Content::AudioVisual(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "md2")]
        if let Content::Md2(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "md2_control")]
        if let Content::Md2Control(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "envmap_exr")]
        if let Content::EnvmapExr(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "dds")]
        if let Content::Dds(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "kinect")]
        if let Content::Kinect(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "channels")]
        if let Content::Channels(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "wide_gamut")]
        if let Content::WideGamut(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "uv_tests")]
        if let Content::UvTests(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "clipping_stencil")]
        if let Content::ClippingStencil(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ascii")]
        if let Content::Ascii(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lightprobes_sponza")]
        if let Content::LightprobesSponza(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "svg_sandbox")]
        if let Content::SvgSandbox(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "svg_lines")]
        if let Content::SvgLines(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "css3d_mixed")]
        if let Content::Css3dMixed(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "css3d_molecules")]
        if let Content::Css3dMolecules(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "css3d_periodictable")]
        if let Content::Css3dPeriodictable(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "css3d_sprites")]
        if let Content::Css3dSprites(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "css3d_orthographic")]
        if let Content::Css3dOrthographic(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "css3d_sandbox")]
        if let Content::Css3dSandbox(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "css2d_label")]
        if let Content::Css2dLabel(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "animation_pointer")]
        if let Content::AnimationPointer(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "envmaps_fasthdr")]
        if let Content::EnvmapsFasthdr(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_rapier_vehicle_controller")]
        if let Content::PhysicsRapierVehicleController(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_rapier_character_controller")]
        if let Content::PhysicsRapierCharacterController(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_rapier_terrain")]
        if let Content::PhysicsRapierTerrain(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_rapier_joints")]
        if let Content::PhysicsRapierJoints(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_rapier_instancing")]
        if let Content::PhysicsRapierInstancing(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_ammo_instancing")]
        if let Content::PhysicsAmmoInstancing(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_jolt_instancing")]
        if let Content::PhysicsJoltInstancing(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_ammo_rope")]
        if let Content::PhysicsAmmoRope(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_ammo_cloth")]
        if let Content::PhysicsAmmoCloth(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_ammo_volume")]
        if let Content::PhysicsAmmoVolume(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_ammo_terrain")]
        if let Content::PhysicsAmmoTerrain(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_ammo_break")]
        if let Content::PhysicsAmmoBreak(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "html_texture")]
        if let Content::HtmlTexture(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "physics_rapier_basic")]
        if let Content::PhysicsRapierBasic(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "geometry_csg")]
        if let Content::GeometryCsg(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ldraw")]
        if let Content::Ldraw(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "batch_lod_bvh")]
        if let Content::BatchLodBvh(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "offscreen")]
        if let Content::Offscreen(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lottie")]
        if let Content::Lottie(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "watch")]
        if let Content::Watch(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ifc_loader")]
        if let Content::IfcLoader(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "loader_3dm")]
        if let Content::Loader3dm(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "webcam")]
        if let Content::Webcam(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "transmission_alpha")]
        if let Content::TransmissionAlpha(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "random_uv")]
        if let Content::RandomUv(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "games_fps")]
        if let Content::GamesFps(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "fbx_loader")]
        if let Content::FbxLoader(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "svg_loader")]
        if let Content::SvgLoader(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "exporter_gcode")]
        if let Content::ExporterGcode(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "video_frame")]
        if let Content::VideoFrame(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "arcball")]
        if let Content::Arcball(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "exporter_gltf")]
        if let Content::ExporterGltf(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "spline_editor")]
        if let Content::SplineEditor(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "raycaster_bvh")]
        if let Content::RaycasterBvh(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "subdivision")]
        if let Content::Subdivision(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_performance")]
        if let Content::ShadowmapPerformance(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "usdz_exporter")]
        if let Content::UsdzExporter(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "morphtargets_webcam")]
        if let Content::MorphtargetsWebcam(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_pcss")]
        if let Content::ShadowmapPcss(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "fbx_nurbs")]
        if let Content::FbxNurbs(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ies_spotlight")]
        if let Content::IesSpotlight(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "marching_cubes")]
        if let Content::MarchingCubes(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "test_memory")]
        if let Content::TestMemory(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ktx")]
        if let Content::Ktx(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "pvr")]
        if let Content::Pvr(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "lines_raycast")]
        if let Content::LinesRaycast(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "spot_skinning")]
        if let Content::SpotSkinning(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "draco_variants")]
        if let Content::DracoVariants(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "probes_hdr")]
        if let Content::ProbesHdr(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "shapes_lights")]
        if let Content::ShapesLights(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "helpers_formats")]
        if let Content::HelpersFormats(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "models_modifiers")]
        if let Content::ModelsModifiers(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            demo.key(code, down);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            demo.key(code, down);
        }
    }
    pub fn slider(&mut self, x: f64) {
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            demo.slider(x);
        }
    }
    pub fn draw(&mut self, kind: u32, x: f64, y: f64) -> Result<()> {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "exporters_video")]
        if let Content::ExportersVideo(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "texture_volumes")]
        if let Content::TextureVolumes(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::ColladaAnim(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "threemf")]
        if let Content::ThreeMf(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "gltf_normals")]
        if let Content::GltfNormals(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "raycaster_helper")]
        if let Content::RaycasterHelper(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "audio_timing")]
        if let Content::AudioTiming(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "audio_visual")]
        if let Content::AudioVisual(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "md2")]
        if let Content::Md2(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "md2_control")]
        if let Content::Md2Control(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "envmap_exr")]
        if let Content::EnvmapExr(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "dds")]
        if let Content::Dds(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "kinect")]
        if let Content::Kinect(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "channels")]
        if let Content::Channels(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "wide_gamut")]
        if let Content::WideGamut(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "uv_tests")]
        if let Content::UvTests(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "clipping_stencil")]
        if let Content::ClippingStencil(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ascii")]
        if let Content::Ascii(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lightprobes_sponza")]
        if let Content::LightprobesSponza(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "svg_sandbox")]
        if let Content::SvgSandbox(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "svg_lines")]
        if let Content::SvgLines(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "css3d_mixed")]
        if let Content::Css3dMixed(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "css3d_molecules")]
        if let Content::Css3dMolecules(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "css3d_periodictable")]
        if let Content::Css3dPeriodictable(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "css3d_sprites")]
        if let Content::Css3dSprites(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "css3d_orthographic")]
        if let Content::Css3dOrthographic(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "css3d_sandbox")]
        if let Content::Css3dSandbox(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "css2d_label")]
        if let Content::Css2dLabel(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "animation_pointer")]
        if let Content::AnimationPointer(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "envmaps_fasthdr")]
        if let Content::EnvmapsFasthdr(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_rapier_vehicle_controller")]
        if let Content::PhysicsRapierVehicleController(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_rapier_character_controller")]
        if let Content::PhysicsRapierCharacterController(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_rapier_terrain")]
        if let Content::PhysicsRapierTerrain(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_rapier_joints")]
        if let Content::PhysicsRapierJoints(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_rapier_instancing")]
        if let Content::PhysicsRapierInstancing(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_ammo_instancing")]
        if let Content::PhysicsAmmoInstancing(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_jolt_instancing")]
        if let Content::PhysicsJoltInstancing(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_ammo_rope")]
        if let Content::PhysicsAmmoRope(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_ammo_cloth")]
        if let Content::PhysicsAmmoCloth(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_ammo_volume")]
        if let Content::PhysicsAmmoVolume(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_ammo_terrain")]
        if let Content::PhysicsAmmoTerrain(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_ammo_break")]
        if let Content::PhysicsAmmoBreak(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "html_texture")]
        if let Content::HtmlTexture(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "physics_rapier_basic")]
        if let Content::PhysicsRapierBasic(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "geometry_csg")]
        if let Content::GeometryCsg(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ldraw")]
        if let Content::Ldraw(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "batch_lod_bvh")]
        if let Content::BatchLodBvh(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "offscreen")]
        if let Content::Offscreen(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lottie")]
        if let Content::Lottie(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "watch")]
        if let Content::Watch(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ifc_loader")]
        if let Content::IfcLoader(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "loader_3dm")]
        if let Content::Loader3dm(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "webcam")]
        if let Content::Webcam(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "transmission_alpha")]
        if let Content::TransmissionAlpha(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "random_uv")]
        if let Content::RandomUv(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "games_fps")]
        if let Content::GamesFps(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "fbx_loader")]
        if let Content::FbxLoader(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "svg_loader")]
        if let Content::SvgLoader(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "exporter_gcode")]
        if let Content::ExporterGcode(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "video_frame")]
        if let Content::VideoFrame(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "arcball")]
        if let Content::Arcball(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "exporter_gltf")]
        if let Content::ExporterGltf(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "spline_editor")]
        if let Content::SplineEditor(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "raycaster_bvh")]
        if let Content::RaycasterBvh(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "subdivision")]
        if let Content::Subdivision(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_performance")]
        if let Content::ShadowmapPerformance(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "usdz_exporter")]
        if let Content::UsdzExporter(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "morphtargets_webcam")]
        if let Content::MorphtargetsWebcam(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_pcss")]
        if let Content::ShadowmapPcss(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "fbx_nurbs")]
        if let Content::FbxNurbs(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ies_spotlight")]
        if let Content::IesSpotlight(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "marching_cubes")]
        if let Content::MarchingCubes(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "test_memory")]
        if let Content::TestMemory(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ktx")]
        if let Content::Ktx(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "pvr")]
        if let Content::Pvr(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "lines_raycast")]
        if let Content::LinesRaycast(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "spot_skinning")]
        if let Content::SpotSkinning(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "draco_variants")]
        if let Content::DracoVariants(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "probes_hdr")]
        if let Content::ProbesHdr(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "shapes_lights")]
        if let Content::ShapesLights(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "helpers_formats")]
        if let Content::HelpersFormats(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "models_modifiers")]
        if let Content::ModelsModifiers(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            demo.draw(kind, x, y);
            return Ok(());
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            return demo.pointer(kind, x, y);
        }
        #[cfg(feature = "interactive_objects")]
        if let Content::InteractiveObjects(demo) = &mut self.content {
            return demo.draw(kind, x, y);
        }
        Err(crate::Error::Invalid("not a drawing example"))
    }
    pub fn seek(&mut self, seconds: f64) {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "exporters_video")]
        if let Content::ExportersVideo(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "texture_volumes")]
        if let Content::TextureVolumes(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::ColladaAnim(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "threemf")]
        if let Content::ThreeMf(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "gltf_normals")]
        if let Content::GltfNormals(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "raycaster_helper")]
        if let Content::RaycasterHelper(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "audio_timing")]
        if let Content::AudioTiming(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "audio_visual")]
        if let Content::AudioVisual(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "md2")]
        if let Content::Md2(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "md2_control")]
        if let Content::Md2Control(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "envmap_exr")]
        if let Content::EnvmapExr(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "dds")]
        if let Content::Dds(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "kinect")]
        if let Content::Kinect(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "channels")]
        if let Content::Channels(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "wide_gamut")]
        if let Content::WideGamut(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "uv_tests")]
        if let Content::UvTests(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "clipping_stencil")]
        if let Content::ClippingStencil(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ascii")]
        if let Content::Ascii(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lightprobes_sponza")]
        if let Content::LightprobesSponza(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "svg_sandbox")]
        if let Content::SvgSandbox(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "svg_lines")]
        if let Content::SvgLines(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "css3d_mixed")]
        if let Content::Css3dMixed(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "css3d_molecules")]
        if let Content::Css3dMolecules(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "css3d_periodictable")]
        if let Content::Css3dPeriodictable(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "css3d_sprites")]
        if let Content::Css3dSprites(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "css3d_orthographic")]
        if let Content::Css3dOrthographic(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "css3d_sandbox")]
        if let Content::Css3dSandbox(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "css2d_label")]
        if let Content::Css2dLabel(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "animation_pointer")]
        if let Content::AnimationPointer(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "envmaps_fasthdr")]
        if let Content::EnvmapsFasthdr(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_rapier_vehicle_controller")]
        if let Content::PhysicsRapierVehicleController(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_rapier_character_controller")]
        if let Content::PhysicsRapierCharacterController(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_rapier_terrain")]
        if let Content::PhysicsRapierTerrain(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_rapier_joints")]
        if let Content::PhysicsRapierJoints(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_rapier_instancing")]
        if let Content::PhysicsRapierInstancing(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_ammo_instancing")]
        if let Content::PhysicsAmmoInstancing(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_jolt_instancing")]
        if let Content::PhysicsJoltInstancing(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_ammo_rope")]
        if let Content::PhysicsAmmoRope(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_ammo_cloth")]
        if let Content::PhysicsAmmoCloth(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_ammo_volume")]
        if let Content::PhysicsAmmoVolume(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_ammo_terrain")]
        if let Content::PhysicsAmmoTerrain(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_ammo_break")]
        if let Content::PhysicsAmmoBreak(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "html_texture")]
        if let Content::HtmlTexture(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "physics_rapier_basic")]
        if let Content::PhysicsRapierBasic(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "geometry_csg")]
        if let Content::GeometryCsg(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ldraw")]
        if let Content::Ldraw(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "batch_lod_bvh")]
        if let Content::BatchLodBvh(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "offscreen")]
        if let Content::Offscreen(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lottie")]
        if let Content::Lottie(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "watch")]
        if let Content::Watch(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ifc_loader")]
        if let Content::IfcLoader(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "loader_3dm")]
        if let Content::Loader3dm(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "webcam")]
        if let Content::Webcam(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "transmission_alpha")]
        if let Content::TransmissionAlpha(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "random_uv")]
        if let Content::RandomUv(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "games_fps")]
        if let Content::GamesFps(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "fbx_loader")]
        if let Content::FbxLoader(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "svg_loader")]
        if let Content::SvgLoader(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "exporter_gcode")]
        if let Content::ExporterGcode(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "video_frame")]
        if let Content::VideoFrame(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "arcball")]
        if let Content::Arcball(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "exporter_gltf")]
        if let Content::ExporterGltf(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "spline_editor")]
        if let Content::SplineEditor(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "raycaster_bvh")]
        if let Content::RaycasterBvh(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "subdivision")]
        if let Content::Subdivision(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_performance")]
        if let Content::ShadowmapPerformance(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "usdz_exporter")]
        if let Content::UsdzExporter(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "morphtargets_webcam")]
        if let Content::MorphtargetsWebcam(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_pcss")]
        if let Content::ShadowmapPcss(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "fbx_nurbs")]
        if let Content::FbxNurbs(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ies_spotlight")]
        if let Content::IesSpotlight(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "marching_cubes")]
        if let Content::MarchingCubes(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "test_memory")]
        if let Content::TestMemory(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ktx")]
        if let Content::Ktx(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "pvr")]
        if let Content::Pvr(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "lines_raycast")]
        if let Content::LinesRaycast(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "spot_skinning")]
        if let Content::SpotSkinning(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "draco_variants")]
        if let Content::DracoVariants(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "probes_hdr")]
        if let Content::ProbesHdr(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "exporters_matcap")]
        if let Content::ExportersMatcap(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "selection_views")]
        if let Content::SelectionViews(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shapes_lights")]
        if let Content::ShapesLights(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::RefractionLoaders(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "helpers_formats")]
        if let Content::HelpersFormats(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "models_modifiers")]
        if let Content::ModelsModifiers(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "interactive_objects")]
        if let Content::InteractiveObjects(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "interactive_shaders")]
        if let Content::InteractiveShaders(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "environment_materials")]
        if let Content::EnvironmentMaterials(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "geometry_materials")]
        if let Content::GeometryMaterials(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shader_geometry")]
        if let Content::ShaderGeometry(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "point_clouds")]
        if let Content::PointClouds(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "buffer_particles")]
        if let Content::BufferParticles(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "shapes")]
        if let Content::Shapes(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "material_textures")]
        if let Content::MaterialTextures(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_procedural")]
        if let Content::TslProcedural(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_primitives")]
        if let Content::TslPrimitives(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_materials")]
        if let Content::TslMaterials(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_viewport")]
        if let Content::TslViewport(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_lighting")]
        if let Content::TslLighting(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_environment")]
        if let Content::TslEnvironment(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_next")]
        if let Content::TslNext(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_particles")]
        if let Content::TslParticles(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_filters")]
        if let Content::TslFilter(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_passes")]
        if let Content::TslPass(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "tsl_examples")]
        if let Content::Tsl(demo) = &mut self.content {
            demo.seek(seconds);
        }
        #[cfg(feature = "gltf_examples")]
        if let Content::Gltf(demo) = &mut self.content {
            demo.seek(seconds);
        }
        self.elapsed = seconds;
        if let Content::Horse(horse) = &mut self.content {
            horse.theta = seconds * 6.0;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        scene: &mut Scene,
        camera: Object3D,
        dx: f64,
        dy: f64,
        wheel: f64,
        pan: bool,
        height: f64,
    ) -> Result<()> {
        #[cfg(feature = "lights_probes")]
        if let Content::LightsProbes(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "exporters_video")]
        if let Content::ExportersVideo(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadow_rtt")]
        if let Content::ShadowRtt(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "texture_flares")]
        if let Content::TextureFlares(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "transform_curves")]
        if let Content::TransformCurves(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "texture_volumes")]
        if let Content::TextureVolumes(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::ColladaAnim(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "threemf")]
        if let Content::ThreeMf(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "gltf_normals")]
        if let Content::GltfNormals(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "raycaster_helper")]
        if let Content::RaycasterHelper(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "audio_timing")]
        if let Content::AudioTiming(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "audio_visual")]
        if let Content::AudioVisual(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "md2")]
        if let Content::Md2(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "md2_control")]
        if let Content::Md2Control(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "envmap_exr")]
        if let Content::EnvmapExr(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "dds")]
        if let Content::Dds(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "kinect")]
        if let Content::Kinect(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "channels")]
        if let Content::Channels(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "wide_gamut")]
        if let Content::WideGamut(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "uv_tests")]
        if let Content::UvTests(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "clipping_stencil")]
        if let Content::ClippingStencil(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ascii")]
        if let Content::Ascii(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "vxgi_sponza")]
        if let Content::VxgiSponza(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lightprobes_sponza")]
        if let Content::LightprobesSponza(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "svg_sandbox")]
        if let Content::SvgSandbox(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "svg_lines")]
        if let Content::SvgLines(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "css3d_mixed")]
        if let Content::Css3dMixed(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "css3d_molecules")]
        if let Content::Css3dMolecules(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "css3d_periodictable")]
        if let Content::Css3dPeriodictable(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "css3d_sprites")]
        if let Content::Css3dSprites(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "css3d_orthographic")]
        if let Content::Css3dOrthographic(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "css3d_sandbox")]
        if let Content::Css3dSandbox(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "css2d_label")]
        if let Content::Css2dLabel(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "animation_pointer")]
        if let Content::AnimationPointer(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "envmaps_fasthdr")]
        if let Content::EnvmapsFasthdr(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_rapier_vehicle_controller")]
        if let Content::PhysicsRapierVehicleController(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_rapier_character_controller")]
        if let Content::PhysicsRapierCharacterController(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_rapier_terrain")]
        if let Content::PhysicsRapierTerrain(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_rapier_joints")]
        if let Content::PhysicsRapierJoints(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_rapier_instancing")]
        if let Content::PhysicsRapierInstancing(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_ammo_instancing")]
        if let Content::PhysicsAmmoInstancing(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_jolt_instancing")]
        if let Content::PhysicsJoltInstancing(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_ammo_rope")]
        if let Content::PhysicsAmmoRope(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_ammo_cloth")]
        if let Content::PhysicsAmmoCloth(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_ammo_volume")]
        if let Content::PhysicsAmmoVolume(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_ammo_terrain")]
        if let Content::PhysicsAmmoTerrain(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_ammo_break")]
        if let Content::PhysicsAmmoBreak(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "html_texture")]
        if let Content::HtmlTexture(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "physics_rapier_basic")]
        if let Content::PhysicsRapierBasic(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "cubemap_webgl")]
        if let Content::CubemapWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ssr_webgl")]
        if let Content::SsrWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lut_webgl")]
        if let Content::LutWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_webgl")]
        if let Content::ShadowmapWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "dof_webgl")]
        if let Content::DofWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "sobel_webgl")]
        if let Content::SobelWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "portal_webgl")]
        if let Content::PortalWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "refraction_webgl")]
        if let Content::RefractionWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "mirror_webgl")]
        if let Content::MirrorWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "afterimage_webgl")]
        if let Content::AfterimageWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "msaa_renderbuffers")]
        if let Content::MsaaRenderbuffers(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "fxaa_webgl")]
        if let Content::FxaaWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "loader_materialx")]
        if let Content::LoaderMaterialx(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "compute_reduce")]
        if let Content::ComputeReduce(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "generator_city")]
        if let Content::GeneratorCity(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "geometry_csg")]
        if let Content::GeometryCsg(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ldraw")]
        if let Content::Ldraw(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "batch_lod_bvh")]
        if let Content::BatchLodBvh(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "volume_fire")]
        if let Content::VolumeFire(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "offscreen")]
        if let Content::Offscreen(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lottie")]
        if let Content::Lottie(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "watch")]
        if let Content::Watch(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ifc_loader")]
        if let Content::IfcLoader(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "loader_3dm")]
        if let Content::Loader3dm(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "webcam")]
        if let Content::Webcam(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "transmission_alpha")]
        if let Content::TransmissionAlpha(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "random_uv")]
        if let Content::RandomUv(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "games_fps")]
        if let Content::GamesFps(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "fbx_loader")]
        if let Content::FbxLoader(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "svg_loader")]
        if let Content::SvgLoader(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "exporter_gcode")]
        if let Content::ExporterGcode(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "video_frame")]
        if let Content::VideoFrame(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "arcball")]
        if let Content::Arcball(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "exporter_gltf")]
        if let Content::ExporterGltf(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "spline_editor")]
        if let Content::SplineEditor(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "raycaster_bvh")]
        if let Content::RaycasterBvh(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "subdivision")]
        if let Content::Subdivision(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "gtao_webgl")]
        if let Content::GtaoWebgl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_performance")]
        if let Content::ShadowmapPerformance(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "usdz_exporter")]
        if let Content::UsdzExporter(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "usdz")]
        if let Content::Usdz(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "vrml")]
        if let Content::Vrml(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "birds_gltf")]
        if let Content::BirdsGltf(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "subsurface")]
        if let Content::Subsurface(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "dof2")]
        if let Content::Dof2(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "simple_gi")]
        if let Content::SimpleGi(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ssr_denoise")]
        if let Content::SsrDenoise(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "retargeting_readyplayer")]
        if let Content::RetargetingReadyplayer(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_progressive")]
        if let Content::ShadowmapProgressive(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "compute_rasterizer_ibl")]
        if let Content::ComputeRasterizerIbl(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "sculpt")]
        if let Content::Sculpt(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "geometry_loft")]
        if let Content::GeometryLoft(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "generator_building")]
        if let Content::GeneratorBuilding(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "compute_rasterizer")]
        if let Content::ComputeRasterizer(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadow_array")]
        if let Content::ShadowArray(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "gtao")]
        if let Content::Gtao(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "vxgi")]
        if let Content::Vxgi(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "custom_fog")]
        if let Content::CustomFog(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "water")]
        if let Content::Water(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "gaussian_splat")]
        if let Content::GaussianSplat(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "retargeting")]
        if let Content::Retargeting(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "skinning_instances")]
        if let Content::SkinningInstances(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "fog_volume")]
        if let Content::FogVolume(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "backdrop_water")]
        if let Content::BackdropWater(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "fog_scattering")]
        if let Content::FogScattering(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ssr")]
        if let Content::Ssr(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "sss")]
        if let Content::Sss(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ssgi")]
        if let Content::Ssgi(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ballpool")]
        if let Content::Ballpool(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "pathtracer")]
        if let Content::Pathtracer(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "progressive_lod")]
        if let Content::ProgressiveLod(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "morphtargets_webcam")]
        if let Content::MorphtargetsWebcam(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tiles3d")]
        if let Content::Tiles3d(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "compute_water")]
        if let Content::ComputeWater(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_graph")]
        if let Content::TslGraph(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::LightprobesComplex(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lightprobes")]
        if let Content::Lightprobes(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "fluid")]
        if let Content::Fluid(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "hdr")]
        if let Content::Hdr(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "volume_caustics")]
        if let Content::VolumeCaustics(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "compute_cloth")]
        if let Content::ComputeCloth(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "deferred")]
        if let Content::Deferred(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "cubemap_dynamic")]
        if let Content::CubemapDynamic(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "retro")]
        if let Content::Retro(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "gpgpu_protoplanet")]
        if let Content::GpgpuProtoplanet(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_pcss")]
        if let Content::ShadowmapPcss(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "volume_traa")]
        if let Content::VolumeTraa(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "volume_rectarea")]
        if let Content::VolumeRectarea(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "memory_outline")]
        if let Content::MemoryOutline(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "godrays")]
        if let Content::Godrays(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "volume_lighting")]
        if let Content::VolumeLighting(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "instancing_morph")]
        if let Content::InstancingMorph(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_csm")]
        if let Content::ShadowmapCsm(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "caustics")]
        if let Content::Caustics(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_opacity")]
        if let Content::ShadowmapOpacity(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lights_projector")]
        if let Content::LightsProjector(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_vsm")]
        if let Content::ShadowmapVsm(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lights_dynamic")]
        if let Content::LightsDynamic(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lights_clustered")]
        if let Content::LightsClustered(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "linked_particles")]
        if let Content::LinkedParticles(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "attractors")]
        if let Content::Attractors(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "sort_bitonic")]
        if let Content::SortBitonic(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "compute_birds")]
        if let Content::ComputeBirds(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "fbx_nurbs")]
        if let Content::FbxNurbs(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "points_dynamic")]
        if let Content::PointsDynamic(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "postprocessing_advanced")]
        if let Content::PostprocessingAdvanced(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ies_spotlight")]
        if let Content::IesSpotlight(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "test_memory2")]
        if let Content::TestMemory2(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "marching_cubes")]
        if let Content::MarchingCubes(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "render_bundle")]
        if let Content::RenderBundle(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "mesh_batch")]
        if let Content::MeshBatch(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "volume_instancing")]
        if let Content::VolumeInstancing(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "test_memory")]
        if let Content::TestMemory(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ktx")]
        if let Content::Ktx(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "pvr")]
        if let Content::Pvr(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "lines_raycast")]
        if let Content::LinesRaycast(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "log_depth")]
        if let Content::LogDepth(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "reversed_depth")]
        if let Content::ReversedDepth(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "elements_text")]
        if let Content::ElementsText(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ktx2")]
        if let Content::Ktx2(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "outline")]
        if let Content::Outline(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "taa")]
        if let Content::Taa(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "sao")]
        if let Content::Sao(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "ssao")]
        if let Content::Ssao(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "glitch")]
        if let Content::Glitch(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shadowmap_viewer")]
        if let Content::ShadowmapViewer(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "spot_skinning")]
        if let Content::SpotSkinning(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "draco_variants")]
        if let Content::DracoVariants(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "passes_decals")]
        if let Content::PassesDecals(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "probes_hdr")]
        if let Content::ProbesHdr(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "sky_water")]
        if let Content::SkyWater(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "exporters_matcap")]
        if let Content::ExportersMatcap(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "text_clipping")]
        if let Content::TextClipping(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shapes_lights")]
        if let Content::ShapesLights(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "refraction_loaders")]
        if let Content::RefractionLoaders(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "helpers_formats")]
        if let Content::HelpersFormats(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "picking_buffers")]
        if let Content::PickingBuffers(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "models_modifiers")]
        if let Content::ModelsModifiers(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "terrain_loaders")]
        if let Content::TerrainLoaders(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "trackball_sprites")]
        if let Content::TrackballSprites(demo) = &mut self.content {
            demo.wheel(wheel);
            return Ok(());
        }
        #[cfg(feature = "controls_attributes")]
        if let Content::ControlsAttributes(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "stereo_loaders")]
        if let Content::StereoLoaders(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "views_loaders")]
        if let Content::ViewsLoaders(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "interactive_scenes")]
        if let Content::InteractiveScenes(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "interactive_objects")]
        if let Content::InteractiveObjects(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "interactive_shaders")]
        if matches!(self.content, Content::InteractiveShaders(_)) {
            return Ok(());
        }
        #[cfg(feature = "environment_materials")]
        if let Content::EnvironmentMaterials(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "geometry_materials")]
        if let Content::GeometryMaterials(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shader_geometry")]
        if let Content::ShaderGeometry(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "shapes")]
        if let Content::Shapes(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "material_textures")]
        if let Content::MaterialTextures(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_procedural")]
        if let Content::TslProcedural(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_primitives")]
        if let Content::TslPrimitives(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_materials")]
        if let Content::TslMaterials(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_viewport")]
        if let Content::TslViewport(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_lighting")]
        if let Content::TslLighting(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_environment")]
        if let Content::TslEnvironment(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_next")]
        if let Content::TslNext(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_surface")]
        if let Content::TslSurface(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_compute")]
        if let Content::TslCompute(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_particles")]
        if let Content::TslParticles(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "gltf_examples")]
        if let Content::Gltf(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_passes")]
        if let Content::TslPass(demo) = &mut self.content {
            return demo.input(scene, camera, dx, dy, wheel, pan, height);
        }
        #[cfg(feature = "tsl_examples")]
        if matches!(self.content, Content::Tsl(_)) {
            return Ok(());
        }
        if matches!(
            self.content,
            Content::Triangles { .. }
                | Content::Horse(_)
                | Content::ColorLines { .. }
                | Content::Indexed(_)
                | Content::Geometries(_)
                | Content::Dashed(_)
                | Content::VertexColors { .. }
        ) {
            return Ok(());
        }
        if pan {
            self.viewer.pan_pixels(scene, camera, dx, dy, height)?;
        } else {
            let (wheel, min, max) = if matches!(self.content, Content::Morph(_)) {
                (0.0, 0.0, f64::INFINITY)
            } else if matches!(self.content, Content::Sphere(_)) {
                (wheel, 1.0, 20.0)
            } else if matches!(self.content, Content::Instancing) {
                (wheel, 0.2, 10.0)
            } else {
                (wheel, 0.0, f64::INFINITY)
            };
            self.viewer.orbit_pixels(dx, dy, wheel, height, min, max);
        }
        Ok(())
    }
}

impl Demo {
    pub fn viewport(&mut self, index: usize, rectangle: [f64; 4]) -> Result<()> {
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(d) = &mut self.content {
            return d.viewport(index, rectangle);
        }
        Err(crate::Error::Invalid("multiple elements example"))
    }
}

impl Demo {
    /// After the frame is submitted, in the same task: effects that read the
    /// canvas back (AsciiEffect).
    pub fn presented(&mut self, canvas: &web_sys::HtmlCanvasElement) -> Result<()> {
        #[cfg(feature = "ascii")]
        if let Content::Ascii(d) = &mut self.content {
            d.presented(canvas)?;
        }
        Ok(())
    }
    pub fn attach_canvases(
        &mut self,
        r: &crate::renderer::Renderer,
        canvases: js_sys::Array,
    ) -> Result<()> {
        #[cfg(feature = "tsl_extended")]
        if let Content::TslExtended(d) = &mut self.content {
            return d.attach_canvases(r, canvases);
        }
        Err(crate::Error::Invalid("multiple canvases example"))
    }
}

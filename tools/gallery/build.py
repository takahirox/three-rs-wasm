#!/usr/bin/env python3
"""Inventory every pinned example; vendor only the gallery UI, never its renderer."""
import hashlib
import json
import re
import sys
import tarfile
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REV = json.loads((ROOT / 'compat/three-r186/api.json').read_text())['baseline']['upstream_commit']
ARCHIVE = ROOT / '.cache' / f'three-{REV}.tar.gz'
OUT = ROOT / 'web/gallery'
FEATURES = {
 'programmable-shading': ('Programmable materials / TSL equivalents', r"from\s*['\"]three/tsl['\"]|\b(?:ShaderMaterial|RawShaderMaterial)\b"),
 'gpu-compute': ('GPU compute and storage buffers', r'\b(?:computeAsync|compute|StorageBufferAttribute|instancedArray|storage|storageObject)\s*\('),
 'animation-skinning': ('Animation mixer and skeletal animation', r'\b(?:AnimationMixer|SkinnedMesh|Skeleton|AnimationClip|skinning)\b'),
 'morph-targets': ('Morph target animation', r'\b(?:morphTargetInfluences|morphAttributes|morphTargets)\b'),
 'shadows': ('Shadow maps and shadow filtering', r'\b(?:castShadow|receiveShadow|shadowMap)\b'),
 'physical-materials': ('Transmission, clearcoat, sheen, anisotropy and related PBR extensions', r'\b(?:transmission|clearcoat|sheen|anisotropy|iridescence|dispersion|thickness)\s*[:=,.]'),
 'postprocessing': ('Postprocessing passes and temporal history', r'\b(?:EffectComposer|RenderPipeline|PostProcessing|SSAOPass|UnrealBloomPass|bloom|ssr|ssao|traa|smaa|fxaa|gaussianBlur)\b'),
 'instancing-batching': ('Instance transforms and batched drawing', r'\b(?:InstancedMesh|BatchedMesh|instanceIndex|instancedArray)\b'),
 'procedural-geometry': ('Additional procedural geometry builders', r'\b(?:TorusKnotGeometry|TorusGeometry|TubeGeometry|CylinderGeometry|ConeGeometry|IcosahedronGeometry|DodecahedronGeometry|OctahedronGeometry|TetrahedronGeometry|ExtrudeGeometry|ShapeGeometry|TextGeometry|LatheGeometry|CapsuleGeometry|RingGeometry|CircleGeometry)\b'),
 'asset-loaders': ('Additional loaders and compressed assets', r'\b(?:DRACOLoader|KTX2Loader|KTXLoader|TIFFLoader|XYZLoader|EXRLoader|FBXLoader|OBJLoader|PLYLoader|STLLoader|ColladaLoader|USDLoader|LDrawLoader|3MFLoader|VOXLoader)\b'),
 'advanced-lights': ('Hemisphere/spot/area lights, light probes and baking', r'\b(?:HemisphereLight|SpotLight|RectAreaLight|LightProbe|LightProbeGenerator|LightProbeHelper)\b'),
 'other-materials': ('Phong, Lambert, normal, depth, toon and matcap materials', r'\b(?:MeshPhongMaterial|MeshPhongNodeMaterial|MeshLambertMaterial|MeshLambertNodeMaterial|MeshNormalMaterial|MeshNormalNodeMaterial|MeshDepthMaterial|MeshToonMaterial|MeshToonNodeMaterial|MeshMatcapMaterial|MeshMatcapNodeMaterial)\b'),
 'wireframe-helpers': ('Wireframe materials and scene helpers', r'\b(?:wireframe|CameraHelper|BoxHelper|Box3Helper|AxesHelper|GridHelper|PolarGridHelper|VertexNormalsHelper|VertexTangentsHelper|SkeletonHelper)\b'),
 'curves': ('Curve interpolation and path builders', r'\b(?:CatmullRomCurve3|CubicBezierCurve3|QuadraticBezierCurve3|CurvePath|hilbert3D)\b'),
 'dynamic-textures': ('Canvas, HTML, video and partial texture updates', r'\b(?:CanvasTexture|HTMLTexture|VideoTexture|VideoFrameTexture|copyTextureToTexture|addUpdateRange)\b'),
 'custom-blending': ('Configurable blend equations and factors', r'\b(?:CustomBlending|AdditiveBlending|SubtractiveBlending|MultiplyBlending|blendSrc|blendDst|blendEquation)\b'),
 'stereo-effects': ('Stereo, anaglyph and parallax-barrier effects', r'\b(?:StereoEffect|AnaglyphEffect|ParallaxBarrierEffect)\b'),
 'volume-textures': ('Volume rendering and layered textures', r'\b(?:Data3DTexture|DataArrayTexture|CompressedArrayTexture|VolumeNodeMaterial)\b'),
 'clipping-stencil': ('Clipping planes and stencil operations', r'\b(?:clippingPlanes|ClippingGroup|stencilWrite|stencilFunc)\b'),
 'fog': ('Distance and height fog', r'\b(?:Fog|FogExp2|fogNode)\b'),
 'wide-lines': ('Wide / dashed line rendering', r'\b(?:Line2|LineMaterial|LineDashedMaterial)\b'),
 'webxr': ('WebXR sessions, controllers and XR render targets', r'\b(?:VRButton|ARButton|XRButton|XRControllerModelFactory|XRHandModelFactory)\b|renderer\.xr\b'),
 'physics': ('Physics integration', r'\b(?:Ammo|RAPIER|Rapier|Jolt|OIMO|CANNON|RapierPhysics|JoltPhysics|AmmoPhysics)\b'),
 'audio': ('Spatial audio and audio analysis', r'\b(?:AudioListener|AudioLoader|AudioAnalyser|PositionalAudio)\b'),
 'camera-controls': ('Full camera controls: pan, touch, damping and control variants', r'\b(?:OrbitControls|TrackballControls|MapControls|FlyControls|FirstPersonControls|TransformControls)\b'),
 'inspector': ('Inspector and per-example GUI parity', r'\bInspector\b'),
 'dom-svg-rendering': ('CSS2D/CSS3D/SVG scene renderers', r'\b(?:CSS2DRenderer|CSS3DRenderer|SVGRenderer)\b'),
}
# Reviewed scene equivalents, not an automatic name-prefix heuristic.
WEBGPU_EQUIVALENTS = {'webgl_lensflares':'webgpu_lensflares','webgl_morphtargets_face':'webgpu_morphtargets_face','webgl_lightprobe_cubecamera':'webgpu_lightprobe_cubecamera','webgl_performance':'webgpu_performance','webgl_materials_video':'webgpu_materials_video','webgl_materials_envmaps':'webgpu_materials_envmaps','webgl_materials_displacementmap':'webgpu_materials_displacementmap','webgl_clipping':'webgpu_clipping','webgl_materials_texture_manualmipmap':'webgpu_materials_texture_manualmipmap','webgl_materials_texture_anisotropy':'webgpu_textures_anisotropy','webgl_materials_texture_partialupdate':'webgpu_textures_partialupdate','webgl_loader_gltf_compressed': 'webgpu_loader_gltf_compressed', 'webgl_loader_gltf_dispersion': 'webgpu_loader_gltf_dispersion', 'webgl_loader_gltf': 'webgpu_loader_gltf', 'webgl_morphtargets': 'webgpu_morphtargets', 'webgl_pmrem_test': 'webgpu_pmrem_test', 'webgl_pmrem_equirectangular': 'webgpu_pmrem_equirectangular', 'webgl_panorama_equirectangular': 'webgpu_equirectangular', 'webgl_lights_rectarealight': 'webgpu_lights_rectarealight', 'webgl_camera_array': 'webgpu_camera_array', 'webgl_materials_matcap': 'webgpu_materials_matcap', 'webgl_lights_physical': 'webgpu_lights_physical', 'webgl_shaders_sky': 'webgpu_sky', 'webgl_shaders_ocean': 'webgpu_ocean', 'webgl_video_panorama_equirectangular': 'webgpu_video_panorama', 'webgl_lights_sunlight': 'webgpu_lights_sunlight', 'webgl_materials_physical_clearcoat': 'webgpu_clearcoat', 'webgl_shadowmap_pointlight': 'webgpu_shadowmap_pointlight', 'webgl_lightprobe': 'webgpu_lightprobe', 'webgl_tonemapping': 'webgpu_tonemapping'}
PORTS = {
 'webgl_raycaster_texture': {'example':298,'source':'src/browser/texture_flares.rs','test':'tests/browser/texture-flares.spec.js','limitations':['ブラウザのCanvas 2Dで描く格子画像と黄色の十字（GPUコピーとミップマップ再生成で3つのテクスチャに反映）、ポインタのレイキャストとtransformUv、立方体・平面・円のUV、円テクスチャのラップ・offset・repeat・rotationの操作をRustで再現。ラップの切り替えはテクスチャを共有するサンプラー別のプログラムで行い、原本の再アップロードは行わない。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_texture3d_partialupdate': {'example':299,'source':'src/browser/texture_flares.rs','test':'tests/browser/texture-flares.spec.js','limitations':['128³のData3DTexture、1.5秒ごとにCPUのImprovedNoiseで生成する30³ブロックの部分書き込み（原本と同じCPU生成）、フレーム番号とgl_FragCoordによるジッター付きレイマーチ、キャンバスのグラデーション空、OrbitControls、全4項目の操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_materials_cubemap_render_to_mipmaps': {'example':300,'source':'src/browser/texture_flares.rs','test':'tests/browser/texture-flares.spec.js','limitations':['半精度キューブレンダーターゲットのレベル0〜8への面ごとの描画（レベル別の色付け、ソースキューブの暗黙LODサンプリング）、頂点ごとの反射ベクトルによる2つの環境マップ球（CubeTextureのx反転）、極角制限付きOrbitControlsをRustで再現。性能の完全な同等性は未保証。']},
 'webgpu_lensflares': {'example':301,'source':'src/browser/texture_flares.rs','test':'tests/browser/texture-flares.spec.js','limitations':['3000個のPhongの箱、3つの点光源、LensflareMesh（16×16のフレームバッファ退避、マゼンタの深度テストによる遮蔽マップ、復元、頂点で9テクセルを読む加算要素、光源色のsRGB→線形の一度きりの変換）、フォグ、FlyControls（ポインタとキー）をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_materials_car': {'example':302,'source':'src/browser/texture_flares.rs','test':'tests/browser/texture-flares.spec.js','limitations':['Draco圧縮のferrari glTF、クリアコートの車体・金属の細部・透過ガラス、HDR環境、フォグ、ACES（露出0.85）、半透明のGridHelperの移動と車輪の回転、乗算合成のAO平面、OrbitControls、3色の入力をRustで再現。Stats外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_ubo_arrays': {'example':293,'source':'src/browser/draco_variants.rs','test':'tests/browser/draco-variants.spec.js','limitations':['300個の点光源の位置と色（LightingDataブロック、毎フレームCPUで位置を更新して書き込み、原本と同じ）、ワールド位置と距離減衰の光ループ、平面と100個の球、パンなしのOrbitControls、countの操作をRustで再現。ブロックはWebGPUのストレージバッファとして束縛。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_loader_draco': {'example':294,'source':'src/browser/draco_variants.rs','test':'tests/browser/draco-variants.spec.js','limitations':['DRACOLoaderによるbunny.drcのデコード（位置・法線・色・UV、面インデックス）とcomputeVertexNormals、半球光とPCFの影付きスポットライト、フォグ、Date.nowで回るカメラをRustで再現。性能の完全な同等性は未保証。']},
 'webgl_animation_keyframes': {'example':295,'source':'src/browser/draco_variants.rs','test':'tests/browser/draco-variants.spec.js','limitations':['Skyとその一度だけのPMREM（fromScene）による環境、Draco圧縮のLittlestTokyo glTFとキーフレームアニメーション、ACES、減衰付きOrbitControlsをRustで再現。Stats外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_loader_gltf_variants': {'example':296,'source':'src/browser/draco_variants.rs','test':'tests/browser/draco-variants.spec.js','limitations':['HDRの環境と背景、ACES、KHR_materials_variantsの靴（バリアントごとのマテリアル、元のマテリアルへの復帰）、変更時のみの描画、OrbitControls、Variantの操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_morphtargets_face': {'example':297,'source':'src/browser/draco_variants.rs','test':'tests/browser/draco-variants.spec.js','limitations':['RoomEnvironmentのPMREM（sigma 0.04）、ACES、meshoptとKTX2のfacecap glTF、52個のモーフターゲットのアニメーション、方位角と距離の制限付き減衰OrbitControlsをRustで再現。ミキサーが毎フレーム上書きするlisten表示のモーフスライダーとInspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_postprocessing_backgrounds': {'example':288,'source':'src/browser/passes_decals.rs','test':'tests/browser/passes-decals.spec.js','limitations':['EffectComposerのClearPass（色とアルファ）、TexturePass（木目テクスチャと不透明度）、CubeTexturePass（NoColorSpaceのpisaキューブ、10単位の裏面ボックス、カメラの回転と投影を共有）、RenderPass（3つの点光源とStandardの球）、OutputPass、ズームなしのOrbitControls、全8項目の操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_shader_lava': {'example':289,'source':'src/browser/passes_decals.rs','test':'tests/browser/passes-decals.spec.js','limitations':['溶岩のGLSLシェーダー（雲と溶岩テクスチャ、uvScale、gl_FragCoord.z/wによるフォグ）を持つトーラス、BloomPass（25タップのガウス畳み込みを横と縦、加算合成）、OutputPass、delta×5の時間と回転をRustで再現。性能の完全な同等性は未保証。']},
 'webgl_ubo': {'example':290,'source':'src/browser/passes_decals.rs','test':'tests/browser/passes-decals.spec.js','limitations':['ViewDataとLightingDataのユニフォームブロックを共有する2つのRawShaderMaterial（視点空間のPhong、sRGB出力）、200個の四面体と木箱、毎フレームの回転をRustで再現。ViewDataはフレーム共通のカメラユニフォーム、変化しないLightingDataは両プログラムの定数として実装。性能の完全な同等性は未保証。']},
 'webgl_postprocessing_rgb_halftone': {'example':291,'source':'src/browser/passes_decals.rs','test':'tests/browser/passes-decals.spec.js','limitations':['50個の法線とUVのシェーダー立方体、Phongの床と回転する点光源、HalftonePass（全5形状、チャンネルごとの角度、散乱、ブレンドモード、グレースケール、無効化）、OrbitControls、全10項目の操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_decals': {'example':292,'source':'src/browser/passes_decals.rs','test':'tests/browser/passes-decals.spec.js','limitations':['LeePerrySmithの頭部（Phong、カラー・スペキュラー・法線マップ）、ポインタのレイキャストと法線ライン、クリックで生成するDecalGeometry（6平面のクリッピング、ランダムな回転・大きさ・色、polygonOffset）、Clearボタン、OrbitControlsをRustで再現。デカールの生成は原本と同じくクリック時のCPU処理。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_lightprobe_cubecamera': {'example':283,'source':'src/browser/probes_hdr.rs','test':'tests/browser/probes-hdr.spec.js','limitations':['CubeCameraで捉えたpisa背景からのLightProbeGenerator.fromCubeRenderTarget（8bit線形のキューブと同じ値からCPUでSH係数を一度だけ計算）、LightProbeHelper、背景キューブ、変更時のみの描画をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_materials_envmaps_hdr': {'example':284,'source':'src/browser/probes_hdr.rs','test':'tests/browser/probes-hdr.spec.js','limitations':['Generated（DebugEnvironmentのPMREM）・LDR・HDRキューブの環境マップ切り替え、生キューブ背景とPMREM背景、デバッグ用のPMREMアトラス表示、粗さ・金属度・露出、ACESをRustで再現。Stats表示とlil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_loader_texture_ultrahdr': {'example':285,'source':'src/browser/probes_hdr.rs','test':'tests/browser/probes-hdr.spec.js','limitations':['UltraHDRLoader（MPFとXMPの解析、ブラウザでのJPEGデコードとゲインマップ拡大、復元式とtoHalfFloat）、環境と背景、自動回転、解像度の切り替え（非同期再読み込み）をRustで再現。FloatType選択時も半精度で保持。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_materials_transmission': {'example':286,'source':'src/browser/probes_hdr.rs','test':'tests/browser/probes-hdr.spec.js','limitations':['UltraHDRの環境と背景、透過（transmission）・IOR・厚み・スペキュラーを持つ両面の物理マテリアル、アルファマップの縞、全12項目の操作をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_performance': {'example':287,'source':'src/browser/probes_hdr.rs','test':'tests/browser/probes-hdr.spec.js','limitations':['UltraHDRの環境、798メッシュのダンジョンglTF（WebP）、ACES、OrbitControls、static切り替えをRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_shadowmesh': {'example':278,'source':'src/browser/shadow_rtt.rs','test':'tests/browser/shadow-rtt.spec.js','limitations':['ShadowMesh（光源の同次座標による平面投影、ステンシルで二重描画を防止、前フレームのワールド行列を使用）、5つの物体の回転と移動、ArrowHelper、平行光源と点光源の切り替えボタンをRustで再現。性能の完全な同等性は未保証。']},
 'webgl_instancing_dynamic': {'example':279,'source':'src/browser/shadow_rtt.rs','test':'tests/browser/shadow-rtt.spec.js','limitations':['10,000個のInstancedMesh（毎フレームの行列更新とTWEENによる色の切り替え、原本と同じCPU更新）、RoomEnvironmentの環境、Neutralトーンマッピング、カメラの軌道とup変化をRustで再現。性能の完全な同等性は未保証。']},
 'webgl_depth_texture': {'example':280,'source':'src/browser/shadow_rtt.rs','test':'tests/browser/shadow-rtt.spec.js','limitations':['DepthTexture付きのレンダーターゲット、深度の線形化ポストパス、50個のトーラスノットのInstancedMesh、減衰付きOrbitControls（描画後の更新）をRustで再現。深度の形式と型は表示に影響しない。WebGPUのMSAAは4サンプルのみのため、0以外のサンプル数は4として扱う。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_rtt': {'example':281,'source':'src/browser/shadow_rtt.rs','test':'tests/browser/shadow-rtt.spec.js','limitations':['レンダーターゲットへの描画（UVとtimeのシェーダー、2つのPhongトーラス）、画面クアッドと25個の球へのテクスチャ、autoClearなしの2段描画、マウス追従カメラをRustで再現。原本はリサイズに対応しないため、リサイズ後の表示は未比較。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_materials_normalmap': {'example':282,'source':'src/browser/shadow_rtt.rs','test':'tests/browser/shadow-rtt.spec.js','limitations':['LeePerrySmithの頭部（Phong、カラー・スペキュラー・法線マップ）、EffectComposer（BleachBypass、ColorCorrection、OutputPass、FXAA）、減衰付きOrbitControls、法線マップの切り替えと強度をRustで再現。devicePixelRatioを使わない原本と同じく1倍で描画。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'misc_exporter_exr': {'example':273,'source':'src/browser/exporters_video.rs','test':'tests/browser/exporters-video.spec.js','limitations':['EXRExporter（ZIP・ZIPS・無圧縮、Half/Float）、PMREMの背景とデータテクスチャ、減衰付きOrbitControls（rotateSpeed −0.25）をRustで再現。PMREMの書き出しはGPUからの読み戻し。zlib圧縮後のバイト列は実装の違いで異なり、展開後の画素で比較。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'misc_exporter_ktx2': {'example':274,'source':'src/browser/exporters_video.rs','test':'tests/browser/exporters-video.spec.js','limitations':['KTX2Exporter（ktx-parseのwrite、無圧縮のRGBA Float/Half）、PMREMの背景（AgX）とデータテクスチャ、減衰付きOrbitControlsをRustで再現。PMREMの書き出しはGPUからの読み戻し。lil-gui外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_materials_video': {'example':275,'source':'src/browser/exporters_video.rs','test':'tests/browser/exporters-video.spec.js','limitations':['VideoTexture（新しいフレームのみ転送）を貼った200個のPhongキューブ（UVで分割）、色相の時間変化、1000フレーム周期の移動と反転、マウス追従カメラをRustで再現。性能の完全な同等性は未保証。']},
 'webgpu_compile_async': {'example':276,'source':'src/browser/exporters_video.rs','test':'tests/browser/exporters-video.spec.js','limitations':['MaterialXノイズ（Perlin・Worley・Cell・Fractal）とhashによる256種の固有マテリアルを表示前にビルドし、1秒後に追加。法線マテリアルの球の往復、リサイズ時の12単位のフラスタムをRustで再現。モード切替ボタン（ページ再読み込み）と計測表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_materials_normalmap_object_space': {'example':277,'source':'src/browser/exporters_video.rs','test':'tests/browser/exporters-video.spec.js','limitations':['オブジェクト空間法線マップ（法線属性を削除、両面、裏面で反転）、カメラに付けた点光源、Nefertitiの縮小と中心合わせ、変更時のみの描画をRustで再現。devicePixelRatioを使わない原本と同じく1倍で描画。性能の完全な同等性は未保証。']},
 'webgl_materials_cubemap': {'example':243,'source':'src/browser/shapes_lights.rs','test':'tests/browser/shapes-lights.spec.js','limitations':['OBJLoaderの頭部とLambertのenvmap（反射・屈折、Multiply・Mix合成）、背景キューブ、極角制限付きOrbitControlsをRustで再現。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_loader_stl': {'example':244,'source':'src/browser/shapes_lights.rs','test':'tests/browser/shapes-lights.spec.js','limitations':['STLLoader（ASCII・バイナリ・COLOR=ヘッダの頂点色）、フォグ、半球光と2灯の影をRustで再現。影のPCFの回転ノイズはWebGLと画面座標の上下が逆のため縁が異なる。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_geometry_extrude_shapes': {'example':245,'source':'src/browser/shapes_lights.rs','test':'tests/browser/shapes-lights.spec.js','limitations':['ExtrudeGeometry（パス押し出し・ベベル）、Earcut（穴なし単純多角形）、CatmullRomCurve3の等弧長点とFrenetフレームをRustで再現し、原本の頂点と一致を確認。TrackballControlsは60fps相当の時間ステップ。UV属性は未生成（材質が不使用）。性能の完全な同等性は未保証。']},
 'webgl_lights_spotlights': {'example':246,'source':'src/browser/shapes_lights.rs','test':'tests/browser/shapes-lights.spec.js','limitations':['TWEEN（Quadratic.Out）による3灯のスポットライトの角度・半影・位置の補間、5秒ごとの再設定、スポットライトの影とSpotLightHelperをRustで再現。性能の完全な同等性は未保証。']},
 'webgpu_clearcoat': {'example':268,'source':'src/browser/lights_probes.rs','test':'tests/browser/lights-probes.spec.js','limitations':['クリアコート付きMeshPhysicalMaterial（法線マップ、クリアコート法線マップ、FlakesTextureのキャンバス描画）、HDRキューブ環境、点光源をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'misc_controls_fly': {'example':269,'source':'src/browser/lights_probes.rs','test':'tests/browser/lights-probes.spec.js','limitations':['FlyControls（ポインタ位置によるヨー・ピッチ、キー移動・ロール）、法線・スペキュラーマップ付きの地球と雲・月、星のPoints（r186のWebGPUと同じネイティブ1ピクセル点）、FogExp2、FilmNodeのグレインをRustで再現。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'webgpu_shadowmap_pointlight': {'example':270,'source':'src/browser/lights_probes.rs','test':'tests/browser/lights-probes.spec.js','limitations':['2つの点光源のキューブ影（半径10のPCF、bias）、alphaMap＋alphaTestで穴の空いた球（影にも反映）、BackSideの部屋をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'webgpu_lightprobe': {'example':271,'source':'src/browser/lights_probes.rs','test':'tests/browser/lights-probes.spec.js','limitations':['LightProbeGenerator.fromCubeTexture（CPUでSH係数を計算）、LightProbeの照度、キューブ環境マップの球、LightProbeHelperをRustで再現。r186はライト強度の変更をメッシュの次回リフレッシュまで反映しないため、比較では参照側を強制リフレッシュ（移植版は即時反映）。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'webgpu_tonemapping': {'example':272,'source':'src/browser/lights_probes.rs','test':'tests/browser/lights-probes.spec.js','limitations':['7種のトーンマッピング（None、Linear、Reinhard、Cineon、ACESFilmic、AgX、Neutral）、露出、背景のぼかしと強度、Dracoのglb、ダンピング付きOrbitControlsをRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'webgpu_sky': {'example':263,'source':'src/browser/sky_water.rs','test':'tests/browser/sky-water.spec.js','limitations':['SkyMesh（Preetham大気散乱とr186の雲層）とCubeCameraによる毎フレームの6面キャプチャ、反射球、ACESトーンマッピングをRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'webgpu_lights_sunlight': {'example':264,'source':'src/browser/sky_water.rs','test':'tests/browser/sky-water.spec.js','limitations':['SunLight（2カスケード影）、SkyMeshから生成するPMREM環境、フォグ色と太陽色の補間、InstancedMeshの柱と塔、FirstPersonControlsをRustで再現。カスケード色分け表示（show cascades）は未移植。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'misc_controls_pointerlock': {'example':265,'source':'src/browser/sky_water.rs','test':'tests/browser/sky-water.spec.js','limitations':['PointerLockControls（視点回転・前後左右移動）、重力とジャンプ、下向きレイによる箱への着地、ジッター付き床と500個の箱をRustで再現。ポインタロックはギャラリーのクリックで取得。性能の完全な同等性は未保証。']}, 'webgpu_video_panorama': {'example':266,'source':'src/browser/sky_water.rs','test':'tests/browser/sky-water.spec.js','limitations':['VideoTexture（新しい動画フレームごとにcopyExternalImageToTextureで転送、flipY）と内向き球、ドラッグによる経度・緯度の視点操作をRustで再現。性能の完全な同等性は未保証。']}, 'webgpu_ocean': {'example':267,'source':'src/browser/sky_water.rs','test':'tests/browser/sky-water.spec.js','limitations':['WaterMesh（法線テクスチャのノイズ、半解像度のReflector、フレネル）、SkyMesh（雲）、空から生成するPMREM環境、ブルーム後処理をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'misc_exporter_stl': {'example':258,'source':'src/browser/exporters_matcap.rs','test':'tests/browser/exporters-matcap.spec.js','limitations':['STLExporter（ASCII・バイナリ）をRustで再現し、書き出したファイルが原本とバイト単位で一致することを確認。シーン（影・フォグ・グリッド）も再現。性能の完全な同等性は未保証。']}, 'misc_exporter_ply': {'example':259,'source':'src/browser/exporters_matcap.rs','test':'tests/browser/exporters-matcap.spec.js','limitations':['PLYExporter（ASCII・バイナリBE/LE、法線・UV・Uint8頂点色）をRustで再現し、書き出したファイルが原本とバイト単位で一致することを確認。性能の完全な同等性は未保証。']}, 'misc_exporter_obj': {'example':260,'source':'src/browser/exporters_matcap.rs','test':'tests/browser/exporters-matcap.spec.js','limitations':['OBJExporter（メッシュ・点群、変換済み複数オブジェクト）と6種のジオメトリ切替をRustで再現し、書き出したファイルが原本とバイト単位で一致することを確認。性能の完全な同等性は未保証。']}, 'webgpu_materials_matcap': {'example':261,'source':'src/browser/exporters_matcap.rs','test':'tests/browser/exporters-matcap.spec.js','limitations':['EXRLoader（ZIP圧縮・FLOATチャネルのHalf変換）、matcapUVによるEXRマットキャップ、法線マップ、ACESトーンマッピングをRustで再現。ドラッグ＆ドロップによるマットキャップ差し替えは未移植。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'webgpu_lights_physical': {'example':262,'source':'src/browser/exporters_matcap.rs','test':'tests/browser/exporters-matcap.spec.js','limitations':['光束（lm）・照度（lx）による点光源と半球光、点光源の影、バンプ・ラフネス・メタルネスマップ付きStandard材質、Reinhardトーンマッピングと露出をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。']}, 'misc_animation_groups': {'example':253,'source':'src/browser/selection_views.rs','test':'tests/browser/selection-views.spec.js','limitations':['AnimationObjectGroupで25個の箱が共有するクォータニオン・離散カラー・不透明度のキーフレームをRustで再現（3秒ループ）。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'misc_animation_keys': {'example':254,'source':'src/browser/selection_views.rs','test':'tests/browser/selection-views.spec.js','limitations':['位置・スケール・クォータニオン・離散カラー・不透明度のキーフレームトラックとAxesHelperをRustで再現。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'misc_controls_drag': {'example':255,'source':'src/browser/selection_views.rs','test':'tests/browser/selection-views.spec.js','limitations':['DragControls（左ドラッグで移動、右ドラッグで回転）、Shift+クリックのグループ選択とemissive表示、オンデマンド描画をRustで再現。ホバー時のカーソル変更とタッチ操作（Mキー）は未移植。性能の完全な同等性は未保証。']}, 'misc_boxselection': {'example':256,'source':'src/browser/selection_views.rs','test':'tests/browser/selection-views.spec.js','limitations':['SelectionBoxの視錐台選択（NDC始点・終点、emissive表示）をRustで再現し、SelectionHelperの矩形はDOMで表示。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'webgpu_camera_array': {'example':257,'source':'src/browser/selection_views.rs','test':'tests/browser/selection-views.spec.js','limitations':['ArrayCameraの6×6サブカメラを上端基準ビューポートで描画し、影マップは1回だけ描画（shadow autoUpdate相当）。性能の完全な同等性は未保証。']}, 'webgl_clipping_advanced': {'example':248,'source':'src/browser/text_clipping.rs','test':'tests/browser/text-clipping.spec.js','limitations':['四面体のローカルクリッピング平面（毎フレーム変換）、回転する円筒状のグローバル平面、clipShadows付きInstancedMeshとスポット光・平行光源の影、平面の可視化をRustで再現。GUIはチェックボックスで表現（Visualizeのlisten表示更新は未移植）。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'webgl_geometry_extrude_splines': {'example':249,'source':'src/browser/text_clipping.rs','test':'tests/browser/text-clipping.spec.js','limitations':['CurveExtrasの14曲線とCatmullRom曲線、TubeGeometry（Frenetフレーム）、ワイヤーフレーム、スプラインカメラ・CameraHelper・lookAheadをRustで再現。パラメータ変更時のチューブ再生成は原本と同じくCPUで行う。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'webgl_geometry_text': {'example':250,'source':'src/browser/text_clipping.rs','test':'tests/browser/text-clipping.spec.js','limitations':['FontLoaderの書体JSON、ShapePath.toShapes、穴付きEarcut、ベベル付きExtrudeGeometry（TextGeometry）をRustで再現し、原本の頂点と一致を確認。10種の書体は起動時に一括取得（原本は選択時に取得）。キー入力（keydown/keypress）とドラッグ回転、4つのボタンを移植。性能の完全な同等性は未保証。']}, 'webgl_modifier_tessellation': {'example':251,'source':'src/browser/text_clipping.rs','test':'tests/browser/text-clipping.spec.js','limitations':['TextGeometryのcenter()、TessellateModifier、面ごとの乱数色と変位、生のShaderMaterial出力をRustで再現。変位は頂点属性として常駐。TrackballControlsは60fps相当の時間ステップ。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'webgl_custom_attributes_lines': {'example':252,'source':'src/browser/text_clipping.rs','test':'tests/browser/text-clipping.spec.js','limitations':['TextGeometryの頂点列をLINE_STRIPで描画し、加算合成・深度テストなしの生ShaderMaterialを再現。変位属性の乱歩は原本と同じく毎フレームCPUで更新し全量を転送。Stats表示は未移植。性能の完全な同等性は未保証。']}, 'webgl_lights_hemisphere': {'example':247,'source':'src/browser/shapes_lights.rs','test':'tests/browser/shapes-lights.spec.js','limitations':['フラミンゴのモーフアニメーション、半球光と平行光源の影、各ヘルパー、空のグラデーションシェーダー、フォグをRustで再現。GUIのトグルはチェックボックスで表現。影の強さ（shadowIntensity）は未対応。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_materials_cubemap_refraction': {'example':238,'source':'src/browser/refraction_loaders.rs','test':'tests/browser/refraction-loaders.spec.js','limitations':['PLYLoader（バイナリ）とcomputeVertexNormals、Phongにenvmap_fragmentの屈折（CubeRefractionMapping・MultiplyOperation）と背景キューブを加えてRustで再現。マウス追従のカメラは60fps相当の時間ステップ。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_loader_ply': {'example':239,'source':'src/browser/refraction_loaders.rs','test':'tests/browser/refraction-loaders.spec.js','limitations':['PLYLoader（ASCII・バイナリ）、フラットシェーディング、半球光と2灯の平行光源の影、線形フォグをRustで再現。影のPCFはWebGLと同じVogel円盤とIGNだが、画面座標の上下が逆のため影の縁の回転が異なる。性能の完全な同等性は未保証。']},
 'webgl_loader_kmz': {'example':240,'source':'src/browser/refraction_loaders.rs','test':'tests/browser/refraction-loaders.spec.js','limitations':['KMZLoaderのzip展開とdoc.kmlのモデル参照、ColladaLoaderの静的メッシュ・Phong材質・Z-up回転をRustで再現。変更時のみ描画。性能の完全な同等性は未保証。']},
 'webgl_loader_collada': {'example':241,'source':'src/browser/refraction_loaders.rs','test':'tests/browser/refraction-loaders.spec.js','limitations':['ColladaLoaderの静的メッシュ（polylist・材質グループ・テクスチャ・ノード行列・Z-up）をRustで再現。スキン・アニメーション・キネマティクスは未対応（このアセットは不使用）。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_loader_texture_exr': {'example':242,'source':'src/browser/refraction_loaders.rs','test':'tests/browser/refraction-loaders.spec.js','limitations':['EXRLoaderのPIZ圧縮（ハフマン・ウェーブレット・LUT）をRustで再現し、原本の出力とバイト単位で一致を確認。Reinhardトーンマッピングとexposure操作。変更時のみ描画。性能の完全な同等性は未保証。']},
 'webgl_loader_pdb': {'example':233,'source':'src/browser/helpers_formats.rs','test':'tests/browser/helpers-formats.spec.js','limitations':['PDBLoaderの解析、原子・結合ごとのメッシュ、CSS2DRendererのラベル配置と重なり順をRust/DOMで再現。分子ごとのメッシュは一度だけ構築して常駐（原本は切替ごとに再構築）。ラベル層の書体は原本のmain.cssと同じ指定。性能の完全な同等性は未保証。']},
 'webgl_helpers': {'example':234,'source':'src/browser/helpers_formats.rs','test':'tests/browser/helpers-formats.spec.js','limitations':['頂点法線・接線ヘルパー、BoxHelper、Wireframe/EdgesGeometry、PolarGrid、PointLightHelperをRustで再現。静止メッシュのヘルパー頂点は一度だけ計算して常駐（原本は毎フレーム同じ値を再計算・再送信）。性能の完全な同等性は未保証。']},
 'webgl_modifier_simplifier': {'example':235,'source':'src/browser/helpers_formats.rs','test':'tests/browser/helpers-formats.spec.js','limitations':['SimplifyModifierをmeshoptimizer 1.1のRust移植（optimesh）で再現し、原本のwasm版と同一の結果を確認。比率ごとの結果は一度だけ構築して常駐。変更時のみ描画。性能の完全な同等性は未保証。']},
 'webgl_loader_amf': {'example':236,'source':'src/browser/helpers_formats.rs','test':'tests/browser/helpers-formats.spec.js','limitations':['AMFLoaderのzip展開・XML解析・材質と色の規則をRustで再現。Z-upのOrbitControls。変更時のみ描画。性能の完全な同等性は未保証。']},
 'webgl_loader_texture_tiff': {'example':237,'source':'src/browser/helpers_formats.rs','test':'tests/browser/helpers-formats.spec.js','limitations':['TIFFLoader（UTIF）の無圧縮・LZW・JPEG（pdf.jsのベースラインデコーダ）をRustで再現し、原本とバイト単位で一致を確認。変更時のみ描画。性能の完全な同等性は未保証。']},
 'webgl_loader_bvh': {'example':228,'source':'src/browser/picking_buffers.rs','test':'tests/browser/picking-buffers.spec.js','limitations':['BVHLoaderの階層・モーション解析とAnimationMixerの線形補間／slerpFlat・ループをRustで再現。SkeletonHelperの頂点位置は原本同様に毎フレームCPUで書き込み。性能の完全な同等性は未保証。']},
 'webgl_framebuffer_texture': {'example':229,'source':'src/browser/picking_buffers.rs','test':'tests/browser/picking-buffers.spec.js','limitations':['Gosper曲線の毎フレーム色更新（原本同様CPU→GPU）とcopyFramebufferToTexture相当のテクスチャコピー、スプライトHUDをRustで再現。選択枠はDOMで表示。性能の完全な同等性は未保証。']},
 'webgl_read_float_buffer': {'example':230,'source':'src/browser/picking_buffers.rs','test':'tests/browser/picking-buffers.spec.js','limitations':['Float32レンダーターゲットへの描画と画面表示、マウス位置の値読み出し（非同期コピーで次フレーム以降に表示）をRustで再現。原本はリサイズ非対応のため、初期サイズを保持。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_interactive_cubes_gpu': {'example':231,'source':'src/browser/picking_buffers.rs','test':'tests/browser/picking-buffers.spec.js','limitations':['5000個の結合ボックスとGPUピッキング（1×1のビューオフセット描画と非同期読み出し）、TrackballControlsをRustで再現。IDは整数ターゲットの代わりにFloat32ターゲットへ書き込み（5000以下で厳密）。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_instancing_performance': {'example':232,'source':'src/browser/picking_buffers.rs','test':'tests/browser/picking-buffers.spec.js','limitations':['INSTANCED・MERGED・NAIVEの3方式とcountをRustで再現し、原本同様に変更ごとに再構築。autoRotateは60fps相当の時間ステップ。GPUメモリ表示とStatsは未移植。性能の完全な同等性は未保証。']},
 'webgl_loader_mdd': {'example':223,'source':'src/browser/models_modifiers.rs','test':'tests/browser/models-modifiers.spec.js','limitations':['MDDLoaderのモーフターゲットとAnimationMixerの線形補間・ループをRustで再現し、GPUでモーフ合成。性能の完全な同等性は未保証。']},
 'webgl_modifier_edgesplit': {'example':224,'source':'src/browser/models_modifiers.rs','test':'tests/browser/models-modifiers.spec.js','limitations':['OBJLoader・mergeVertices・EdgeSplitModifierをRustで再現。パラメータの組ごとのジオメトリは一度だけ構築して常駐（原本は変更ごとに再構築）。変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_loader_3ds': {'example':225,'source':'src/browser/models_modifiers.rs','test':'tests/browser/models-modifiers.spec.js','limitations':['TDSLoaderのチャンク解析とPhongマテリアル、法線マップ、TrackballControlsをRustで再現。更新は60fps相当の時間ステップ。性能の完全な同等性は未保証。']},
 'webgl_geometry_teapot': {'example':226,'source':'src/browser/models_modifiers.rs','test':'tests/browser/models-modifiers.spec.js','limitations':['TeapotGeometryのベジェ面分割と6種のシェーディング（反射はPhong結果に環境キューブを乗算）をRustで再現。組ごとのジオメトリは常駐。変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_instancing_scatter': {'example':227,'source':'src/browser/models_modifiers.rs','test':'tests/browser/models-modifiers.spec.js','limitations':['MeshSurfaceSamplerと毎フレームのCPUインスタンス行列更新（原本と同じ）を60fps相当の時間基準で再現。resampleボタンは未移植。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_geometry_terrain': {'example':218,'source':'src/browser/terrain_loaders.rs','test':'tests/browser/terrain-loaders.spec.js','limitations':['ImprovedNoise地形と原本の正弦乱数、Canvas2Dでの陰影テクスチャ拡大をRustで再現。FirstPersonControlsを再現（長さ0のフレームでは更新しない）。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_geometry_terrain_raycast': {'example':219,'source':'src/browser/terrain_loaders.rs','test':'tests/browser/terrain-loaders.spec.js','limitations':['地形とテクスチャ生成をRustで再現。ポインタ移動ごとのCPUレイキャスト（原本と同じ）で円錐を面法線へ向ける。OrbitControlsのキーボード操作は未移植。性能の完全な同等性は未保証。']},
 'webgl_loader_gcode': {'example':220,'source':'src/browser/terrain_loaders.rs','test':'tests/browser/terrain-loaders.spec.js','limitations':['GCodeLoaderの解析をRustで再現し、各ファイルの線分は一度だけ構築して常駐（原本は切替ごとに再解析）。変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_loader_vox': {'example':221,'source':'src/browser/terrain_loaders.rs','test':'tests/browser/terrain-loaders.spec.js','limitations':['VOXLoaderのチャンク解析と貪欲メッシュ化、パレット色をRustで再現。性能の完全な同等性は未保証。']},
 'webgl_loader_obj': {'example':222,'source':'src/browser/terrain_loaders.rs','test':'tests/browser/terrain-loaders.spec.js','limitations':['OBJLoader（オブジェクト・usemtlグループ）とMTLLoader（Kd・Ks・Ns・map_Kd）をRustで再現。同じ画像は1回だけ読み込み共有。性能の完全な同等性は未保証。']},
 'webgl_loader_texture_hdr': {'example':213,'source':'src/browser/trackball_sprites.rs','test':'tests/browser/trackball-sprites.spec.js','limitations':['HDRLoaderのRGBEデコードと半精度変換（切り捨て）をRustで再現。Reinhardトーンマッピングと露出を再現し、変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_geometry_minecraft': {'example':214,'source':'src/browser/trackball_sprites.rs','test':'tests/browser/trackball-sprites.spec.js','limitations':['ImprovedNoiseの地形と面の結合をRustで再現し、1つの常駐ジオメトリで描画（原本と同じ）。FirstPersonControlsを再現（長さ0のフレームでは更新しない）。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'misc_controls_trackball': {'example':215,'source':'src/browser/trackball_sprites.rs','test':'tests/browser/trackball-sprites.spec.js','limitations':['TrackballControls（回転・ズーム・パンの減衰、A/S/Dキー、正射影カメラ切替）をRustで再現。更新は60fps相当の時間ステップ。タッチ操作とmultiTouchRollは未検証。Stats表示は未移植。性能の完全な同等性は未保証。']},
 'webgl_sprites': {'example':216,'source':'src/browser/trackball_sprites.rs','test':'tests/browser/trackball-sprites.spec.js','limitations':['SpriteMaterialの頂点処理（回転・中心・サイズ）とsRGB変換後のフォグをシェーダーで再現。フレーム毎の回転を60fps相当の時間基準で再現。性能の完全な同等性は未保証。']},
 'webgl_lod': {'example':217,'source':'src/browser/trackball_sprites.rs','test':'tests/browser/trackball-sprites.spec.js','limitations':['LODの距離選択とFlyControls（マウス位置での旋回・ボタン前後移動・キー操作）をRustで再現。性能の完全な同等性は未保証。']},
 'misc_controls_orbit': {'example':208,'source':'src/browser/controls_attributes.rs','test':'tests/browser/controls-attributes.spec.js','limitations':['OrbitControls（減衰・極角制限・地面平面パン）をRustで再現。キーボード操作とカーソル形状は未移植。性能の完全な同等性は未保証。']},
 'misc_controls_map': {'example':209,'source':'src/browser/controls_attributes.rs','test':'tests/browser/controls-attributes.spec.js','limitations':['MapControls（左ドラッグのパン・右ドラッグの回転・zoomToCursor・screenSpacePanning）をRustで再現。タッチ操作は未検証。性能の完全な同等性は未保証。']},
 'webgl_camera': {'example':210,'source':'src/browser/controls_attributes.rs','test':'tests/browser/controls-attributes.spec.js','limitations':['Stats表示は未移植。CameraHelperの頂点はGPUで逆射影（原本はCPUで毎フレーム書き換え）。2つのビューポートとO/Pキー切替を再現。性能の完全な同等性は未保証。']},
 'webgl_custom_attributes': {'example':211,'source':'src/browser/controls_attributes.rs','test':'tests/browser/controls-attributes.spec.js','limitations':['Stats表示は未移植。毎フレームのCPUノイズ・HSL変化を60fps相当の時間基準で再現し、変位属性のみ常駐GPUバッファへ書き込み（原本と同じ）。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_drawrange': {'example':212,'source':'src/browser/controls_attributes.rs','test':'tests/browser/controls-attributes.spec.js','limitations':['Stats表示は未移植。毎フレームの粒子移動と接続線の探索は原本と同じCPU処理で、60fps相当の時間基準で再現。更新分のみ常駐GPUバッファへ書き込み。性能の完全な同等性は未保証。']},
 'webgl_effects_stereo': {'example':203,'source':'src/browser/stereo_loaders.rs','test':'tests/browser/stereo-loaders.spec.js','limitations':['StereoCamera・StereoEffectをRustで再現。フレーム毎のカメラ追従を60fps相当の時間基準で再現。性能の完全な同等性は未保証。']},
 'webgl_effects_anaglyph': {'example':204,'source':'src/browser/stereo_loaders.rs','test':'tests/browser/stereo-loaders.spec.js','limitations':['AnaglyphEffect（frameCorners・色行列合成）をRustで再現。フレーム毎のカメラ追従を60fps相当の時間基準で再現。8bit線形の目用ターゲットを色行列で増幅する差を別途上限で検証。性能の完全な同等性は未保証。']},
 'webgl_effects_parallaxbarrier': {'example':205,'source':'src/browser/stereo_loaders.rs','test':'tests/browser/stereo-loaders.spec.js','limitations':['ParallaxBarrierEffectをRustで再現。フレーム毎のカメラ追従を60fps相当の時間基準で再現。8bit線形の目用ターゲット由来の差を別途上限で検証。性能の完全な同等性は未保証。']},
 'webgl_loader_pcd': {'example':206,'source':'src/browser/stereo_loaders.rs','test':'tests/browser/stereo-loaders.spec.js','limitations':['PCDLoader（ascii・binary・LZF圧縮）をRustで再現。読み込んだ点群はGPUに常駐させ再選択時に再利用。WebGLの点ラスタライズ差は厳密な被覆計算との比較で別途検証。操作UI外観は未一致。OrbitControlsのキーボード操作は未移植。性能の完全な同等性は未保証。']},
 'webgl_loader_imagebitmap': {'example':207,'source':'src/browser/stereo_loaders.rs','test':'tests/browser/stereo-loaders.spec.js','limitations':['6個の立方体は1つのテクスチャを共有（原本は同じ画像を6回読み込み）。setTimeoutを例のクロックで再現。性能の完全な同等性は未保証。']},
 'webgl_multiple_views': {'example':198,'source':'src/browser/views_loaders.rs','test':'tests/browser/views-loaders.spec.js','limitations':['Stats表示は未移植。フレーム毎のカメラ移動を60fps相当の時間基準で再現。シザー付きクリアを各ビューの全画面背景三角形で再現。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_math_obb': {'example':199,'source':'src/browser/views_loaders.rs','test':'tests/browser/views-loaders.spec.js','limitations':['Stats表示は未移植。OBB衝突判定とレイ判定をRustへ移植。OrbitControlsのキーボード操作は未移植。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_custom_attributes_points2': {'example':200,'source':'src/browser/views_loaders.rs','test':'tests/browser/views-loaders.spec.js','limitations':['Stats表示は未移植。毎フレームのCPU深度ソート結果とサイズのみ常駐GPUバッファへ書き込み（原本と同じ）。精度依存の1状態は原本の1 ULP感度で比較。性能の完全な同等性は未保証。']},
 'webgl_loader_xyz': {'example':201,'source':'src/browser/views_loaders.rs','test':'tests/browser/views-loaders.spec.js','limitations':['XYZローダーをRustで再現。性能の完全な同等性は未保証。']},
 'webgl_loader_texture_tga': {'example':202,'source':'src/browser/views_loaders.rs','test':'tests/browser/views-loaders.spec.js','limitations':['TGALoaderのデコードをRustへ移植。OrbitControlsのキーボード操作は未移植。性能の完全な同等性は未保証。']},
 'webgl_interactive_voxelpainter': {'example':193,'source':'src/browser/interactive_scenes.rs','test':'tests/browser/interactive-scenes.spec.js','limitations':['Shiftキーはkeydown/keyupで追跡。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_interactive_buffergeometry': {'example':194,'source':'src/browser/interactive_scenes.rs','test':'tests/browser/interactive-scenes.spec.js','limitations':['Stats表示は未移植。ヒット時のみ4頂点の線ジオメトリを更新（原本と同じ）。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_clipping_intersection': {'example':195,'source':'src/browser/interactive_scenes.rs','test':'tests/browser/interactive-scenes.spec.js','limitations':['操作UI外観は未一致。OrbitControlsの回転・ズームをRustで再現（キーボード操作は未移植）。WebGLとのMSAA/alpha to coverage差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_multiple_scenes_comparison': {'example':196,'source':'src/browser/interactive_scenes.rs','test':'tests/browser/interactive-scenes.spec.js','limitations':['シザー付きクリアを右シーンの全画面背景三角形で再現。WebGLとのMSAA差を別途検証。OrbitControlsの回転・パン・ズームをRustで再現（キーボード操作は未移植）。性能の完全な同等性は未保証。']},
 'webgl_materials_blending_custom': {'example':197,'source':'src/browser/interactive_scenes.rs','test':'tests/browser/interactive-scenes.spec.js','limitations':['操作UI外観は未一致。min/maxのブレンド係数はWebGPU仕様によりOne（GLでは無視される）。性能の完全な同等性は未保証。']},
 'webgl_instancing_raycast': {'example':188,'source':'src/browser/interactive_objects.rs','test':'tests/browser/interactive-objects.spec.js','limitations':['Stats・操作UI外観は未一致。OrbitControlsのドラッグ回転・減衰をRustで再現（キーボード操作は未移植）。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_math_orientation_transform': {'example':189,'source':'src/browser/interactive_objects.rs','test':'tests/browser/interactive-objects.spec.js','limitations':['操作UI外観は未一致。WebGLとのワイヤーフレームMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_panorama_cube': {'example':190,'source':'src/browser/interactive_objects.rs','test':'tests/browser/interactive-objects.spec.js','limitations':['OrbitControlsのドラッグ回転・減衰をRustで再現（キーボード操作は未移植）。性能の完全な同等性は未保証。']},
 'webgl_materials_texture_canvas': {'example':191,'source':'src/browser/interactive_objects.rs','test':'tests/browser/interactive-objects.spec.js','limitations':['フレーム毎の回転を60fps相当の時間基準で再現。性能の完全な同等性は未保証。']},
 'webgl_raycaster_sprite': {'example':192,'source':'src/browser/interactive_objects.rs','test':'tests/browser/interactive-objects.spec.js','limitations':['OrbitControlsの回転・パン・ホイールズームをRustで再現（キーボード・タッチ操作の完全一致は未検証）。性能の完全な同等性は未保証。']},
 'webgl_shader': {'example':183,'source':'src/browser/interactive_shaders.rs','test':'tests/browser/interactive-shaders.spec.js','limitations':['GLSLフラグメントをWGSLへ逐語移植。性能の完全な同等性は未保証。']},
 'webgl_postprocessing_procedural': {'example':184,'source':'src/browser/interactive_shaders.rs','test':'tests/browser/interactive-shaders.spec.js','limitations':['Stats・操作UI外観は未一致。非2冪の高さへのリサイズ時はハッシュ雑音を1 ULP実験と分布で比較。性能の完全な同等性は未保証。']},
 'webgl_interactive_cubes': {'example':185,'source':'src/browser/interactive_shaders.rs','test':'tests/browser/interactive-shaders.spec.js','limitations':['Stats表示は未移植。フレーム毎0.1°のカメラ回転を60fps相当の時間基準で再現。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_interactive_cubes_ortho': {'example':186,'source':'src/browser/interactive_shaders.rs','test':'tests/browser/interactive-shaders.spec.js','limitations':['Stats表示は未移植。フレーム毎0.1°のカメラ回転を60fps相当の時間基準で再現。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_interactive_points': {'example':187,'source':'src/browser/interactive_shaders.rs','test':'tests/browser/interactive-shaders.spec.js','limitations':['Stats表示は未移植。点はGPUインスタンシングで展開し、レイキャストはCPU上の常駐位置で実行。フレーム毎の回転を60fps相当の時間基準で再現。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_attributes_none': {'example':168,'source':'src/browser/shader_geometry.rs','test':'tests/browser/shader-geometry.spec.js','limitations':['Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_attributes_integer': {'example':169,'source':'src/browser/shader_geometry.rs','test':'tests/browser/shader-geometry.spec.js','limitations':['Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_instancing': {'example':170,'source':'src/browser/shader_geometry.rs','test':'tests/browser/shader-geometry.spec.js','limitations':['Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_instancing_interleaved': {'example':171,'source':'src/browser/shader_geometry.rs','test':'tests/browser/shader-geometry.spec.js','limitations':['5000個の回転は共通クォータニオンと常駐行列を使いGPUで計算。汎用InterleavedBuffer APIの移植ではありません。Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。']},
 'webgl_materials_modified': {'example':172,'source':'src/browser/shader_geometry.rs','test':'tests/browser/shader-geometry.spec.js','limitations':['Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。']},
 'webgl_materials_wireframe': {'example':173,'source':'src/browser/geometry_materials.rs','test':'tests/browser/geometry-materials.spec.js','limitations':['Stats・操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_materials_texture_filters': {'example':174,'source':'src/browser/material_textures.rs','test':'tests/browser/geometry-materials.spec.js','limitations':['Stats・操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_geometry_shapes': {'example':175,'source':'src/browser/shapes.rs','test':'tests/browser/geometry-materials.spec.js','limitations':['固定Shape・Extrudeジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。Stats・操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_geometry_colors_lookuptable': {'example':176,'source':'src/browser/geometry_materials.rs','test':'tests/browser/geometry-materials.spec.js','limitations':['圧力モデルとLUTは事前生成。色選択はGPUで実行。Stats・操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_uint': {'example':177,'source':'src/browser/geometry_materials.rs','test':'tests/browser/geometry-materials.spec.js','limitations':['Stats・操作UI外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_materials_basic': {'example':178,'source':'src/browser/environment_materials.rs','test':'tests/browser/environment-materials.spec.js','limitations':['Stats・Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_materials_envmaps': {'example':179,'source':'src/browser/environment_materials.rs','test':'tests/browser/environment-materials.spec.js','limitations':['Stats・Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_materials_displacementmap': {'example':180,'source':'src/browser/environment_materials.rs','test':'tests/browser/environment-materials.spec.js','limitations':['固定OBJ形状は公式から事前生成。実行時のOBJLoader移植ではありません。Stats・Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_materials_bumpmap': {'example':181,'source':'src/browser/environment_materials.rs','test':'tests/browser/environment-materials.spec.js','limitations':['WebGLとのバンプ微分・フィルタリングの描画差を別途検証。Stats・Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_materials_blending': {'example':182,'source':'src/browser/environment_materials.rs','test':'tests/browser/environment-materials.spec.js','limitations':['Stats・Inspector外観は未一致。性能の完全な同等性は未保証。']},
 'webgl_points_billboards': {'example':163,'source':'src/browser/point_clouds.rs','test':'tests/browser/point-clouds.spec.js','limitations':['Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。']},
 'webgl_points_sprites': {'example':164,'source':'src/browser/point_clouds.rs','test':'tests/browser/point-clouds.spec.js','limitations':['WebGL点プリミティブとのラスタライズ差を別途検証。Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。']},
 'webgl_points_waves': {'example':165,'source':'src/browser/point_clouds.rs','test':'tests/browser/point-clouds.spec.js','limitations':['Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。']},
 'webgl_custom_attributes_points': {'example':166,'source':'src/browser/point_clouds.rs','test':'tests/browser/point-clouds.spec.js','limitations':['Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。']},
 'webgl_custom_attributes_points3': {'example':167,'source':'src/browser/point_clouds.rs','test':'tests/browser/point-clouds.spec.js','limitations':['固定BoxLineGeometryは公式から事前生成。Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。']},

 'webgl_buffergeometry_points': {'example':158,'source':'src/browser/buffer_particles.rs','test':'tests/browser/buffer-particles.spec.js','limitations':['Stats表示は未移植。点はGPUインスタンシングで展開。WebGL点ラスタライズ差を別途検証。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_points_interleaved': {'example':159,'source':'src/browser/buffer_particles.rs','test':'tests/browser/buffer-particles.spec.js','limitations':['Stats表示は未移植。圧縮色の16-byte GPUレコードを使用。汎用InterleavedBuffer APIの移植ではありません。WebGL点ラスタライズ差と性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_custom_attributes_particles': {'example':160,'source':'src/browser/buffer_particles.rs','test':'tests/browser/buffer-particles.spec.js','limitations':['Stats表示は未移植。点サイズのアニメーションをGPUへ移植。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_instancing_billboards': {'example':161,'source':'src/browser/buffer_particles.rs','test':'tests/browser/buffer-particles.spec.js','limitations':['Stats表示は未移植。75,000個の六角形をGPUインスタンシングで描画。性能の完全な同等性は未保証。']},
 'webgl_buffergeometry_selective_draw': {'example':162,'source':'src/browser/buffer_particles.rs','test':'tests/browser/buffer-particles.spec.js','limitations':['Stats・操作UI外観は未一致。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。']},

 'webgpu_furnace_test': {'example':153,'source':'src/browser/shapes.rs','test':'tests/browser/shapes.spec.js','limitations':['性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。']},
 'webgl_geometry_convex': {'example':154,'source':'src/browser/shapes.rs','test':'tests/browser/shapes.spec.js','limitations':['性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。']},
 'webgl_geometry_nurbs': {'example':155,'source':'src/browser/shapes.rs','test':'tests/browser/shapes.spec.js','limitations':['性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。']},
 'webgl_geometry_text_shapes': {'example':156,'source':'src/browser/shapes.rs','test':'tests/browser/shapes.spec.js','limitations':['性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。']},
 'webgl_geometry_text_stroke': {'example':157,'source':'src/browser/shapes.rs','test':'tests/browser/shapes.spec.js','limitations':['性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。']},

 'webgpu_materials_arrays': {'example':148,'source':'src/browser/material_textures.rs','test':'tests/browser/material-textures.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_clipping': {'example':149,'source':'src/browser/material_textures.rs','test':'tests/browser/material-textures.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials_texture_manualmipmap': {'example':150,'source':'src/browser/material_textures.rs','test':'tests/browser/material-textures.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_textures_anisotropy': {'example':151,'source':'src/browser/material_textures.rs','test':'tests/browser/material-textures.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_textures_partialupdate': {'example':152,'source':'src/browser/material_textures.rs','test':'tests/browser/material-textures.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},

 'webgpu_upscaling_taau': {'example':146,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_motion_blur': {'example':147,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_traa': {'example':145,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials_retroreflection': {'example':144,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_compute_particles_rain': {'example':143,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_compute_particles_snow': {'example':142,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_pixel': {'example':141,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。','公式と同じく描画解像度はDPR 1。']},
 'webgpu_reflection_blurred': {'example':140,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_compute_audio': {'example':139,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-audio.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_reflection': {'example':138,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_mirror': {'example':137,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_shadowmap': {'example':136,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_multiple_rendertargets_readback': {'example':135,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['非同期GPU readbackにより結果の表示まで数フレームかかる場合があります。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_tsl_procedural_terrain': {'example':134,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_tsl_angular_slicing': {'example':133,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_tsl_wood': {'example':132,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。','公式と同じく描画解像度はDPR 1。']},
 'webgpu_skinning_points': {'example':131,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_reflection_roughness': {'example':130,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materialx_noise': {'example':129,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_portal': {'example':128,'source':'src/browser/tsl_procedural.rs','test':'tests/browser/tsl-procedural.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials': {'example':123,'source':'src/browser/tsl_primitives.rs','test':'tests/browser/tsl-primitives.spec.js','limitations':['r186公式のLoop出力が黒になる挙動を維持。汎用Loopノードは未対応。','Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_sandbox': {'example':124,'source':'src/browser/tsl_primitives.rs','test':'tests/browser/tsl-primitives.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_shadow_contact': {'example':125,'source':'src/browser/tsl_primitives.rs','test':'tests/browser/tsl-primitives.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_lines_fat': {'example':126,'source':'src/browser/tsl_primitives.rs','test':'tests/browser/tsl-primitives.spec.js','limitations':['公式r186のdash offset更新不備は修正して比較。','Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_lines_fat_wireframe': {'example':127,'source':'src/browser/tsl_primitives.rs','test':'tests/browser/tsl-primitives.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},

 'webgpu_materials_sss': {'example':118,'source':'src/browser/tsl_materials.rs','test':'tests/browser/tsl-materials.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials_toon': {'example':119,'source':'src/browser/tsl_materials.rs','test':'tests/browser/tsl-materials.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_skinning_instancing': {'example':120,'source':'src/browser/tsl_materials.rs','test':'tests/browser/tsl-materials.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_oit': {'example':121,'source':'src/browser/tsl_materials.rs','test':'tests/browser/tsl-materials.spec.js','limitations':['最終合成とcanvas表示が別パス（公式より1パス多い）。','Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials_envmaps_groundprojected': {'example':122,'source':'src/browser/tsl_materials.rs','test':'tests/browser/tsl-materials.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_backdrop': {'example':113,'source':'src/browser/tsl_viewport.rs','test':'tests/browser/tsl-viewport.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_backdrop_area': {'example':114,'source':'src/browser/tsl_viewport.rs','test':'tests/browser/tsl-viewport.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_refraction': {'example':115,'source':'src/browser/tsl_viewport.rs','test':'tests/browser/tsl-viewport.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_particles_soft': {'example':116,'source':'src/browser/tsl_viewport.rs','test':'tests/browser/tsl-viewport.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_upscaling_fsr1': {'example':117,'source':'src/browser/tsl_viewport.rs','test':'tests/browser/tsl-viewport.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},

 'webgpu_pmrem_cubemap': {'example':108,'source':'src/browser/tsl_lighting.rs','test':'tests/browser/tsl-lighting.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_pmrem_scene': {'example':109,'source':'src/browser/tsl_lighting.rs','test':'tests/browser/tsl-lighting.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials_lightmap': {'example':110,'source':'src/browser/tsl_lighting.rs','test':'tests/browser/tsl-lighting.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_dof_basic': {'example':111,'source':'src/browser/tsl_lighting.rs','test':'tests/browser/tsl-lighting.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_lensflare': {'example':112,'source':'src/browser/tsl_lighting.rs','test':'tests/browser/tsl-lighting.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},

 'webgpu_postprocessing_anamorphic': {'example':87,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['GPUインスタンス変形、HDR高輝度抽出と横方向フィルタ、5段ブルーム。']},
 'webgpu_tsl_earth': {'example':88,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['昼夜テクスチャ、GPUバンプマップ、大気のフレネル表現。']},
 'webgpu_occlusion': {'example':89,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['GPUオクルージョンクエリの非同期結果をTSLの色uniformに反映。']},
 'webgpu_instance_uniform': {'example':90,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['共有ノードプログラムとオブジェクトごとのuniform、ネイティブGPUキューブマップ。']},
 'webgpu_postprocessing_dof': {'example':91,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['GPU上の近景・遠景CoC、64/16タップのボケと合成。CoCは同じ画素数・精度のRG16Fに格納。']},
 'webgpu_multiple_elements': {'example':92,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['単一GPUデバイス、40シーンのviewport/scissor描画。画面外は描画せず、部分表示はカメラのview offsetで切り取る。']},
 'webgpu_multiple_canvas': {'example':93,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['40個のWebGPU CanvasTargetが1個のGPUデバイスを共有。独立したOrbit操作。']},
 'webgpu_struct_drawindirect': {'example':94,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['GPU構造体のatomicStoreで間接描画コマンドを更新。頂点変形・色計算もGPUで実行。']},
 'webgpu_materials_cubemap_mipmaps': {'example':96,'source':'src/browser/tsl_extended/cube.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['Rust TSLでキューブマップ反射。GPU生成mipmapと公式の手動mipmapを比較。']},
 'webgpu_instance_path': {'example':98,'source':'src/browser/tsl_next.rs','test':'tests/browser/tsl-next.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_sobel': {'example':99,'source':'src/browser/tsl_next.rs','test':'tests/browser/tsl-next.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_smaa': {'example':100,'source':'src/browser/tsl_next.rs','test':'tests/browser/tsl-next.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_3dlut': {'example':101,'source':'src/browser/tsl_next.rs','test':'tests/browser/tsl-next.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_cubemap_mix': {'example':103,'source':'src/browser/tsl_environment.rs','test':'tests/browser/tsl-environment.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_cubemap_adjustments': {'example':104,'source':'src/browser/tsl_environment.rs','test':'tests/browser/tsl-environment.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials_envmaps_bpcem': {'example':105,'source':'src/browser/tsl_environment.rs','test':'tests/browser/tsl-environment.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_materials_alphahash': {'example':106,'source':'src/browser/tsl_environment.rs','test':'tests/browser/tsl-environment.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_ca': {'example':107,'source':'src/browser/tsl_environment.rs','test':'tests/browser/tsl-environment.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_parallax_uv': {'example':102,'source':'src/browser/tsl_next.rs','test':'tests/browser/tsl-next.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_rendertarget_2d-array_3d': {'example':97,'source':'src/browser/tsl_extended/layered.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['配列・3DテクスチャとレイヤーへのGPU描画を4ビューで比較。']},
 'webgpu_instance_points': {'example':95,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['GPU Computeでサイズ更新、ピクセル単位のインスタンス点群と共有ターゲットの拡大ビュー。']},
 'webgpu_texturegather': {'example':86,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['WebGPU側を全幅表示。色Gatherと深度比較GatherはGPU命令で実行。WebGL比較欄は対象外。']},
 'webgpu_centroid_sampling': {'example':85,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['左右のMSAA比較を1キャンバス上の独立したターゲットで表示。5種類のUV補間をGPUで実行。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_textures_2d-array_compressed': {'example':84,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['圧縮ブロックを維持したGPU配列テクスチャ。BC/ETC2/ASTC非対応環境では明示的にエラー。性能の完全な同等性は未保証。']},
 'webgpu_compute_texture_3d': {'example':83,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['200³ボクセルをGPU Computeで更新しGPUレイマーチング。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_volume_perlin': {'example':81,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['3DテクスチャをGPU上でレイマーチング。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_volume_cloud': {'example':82,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['3DテクスチャをGPU上でレイマーチング。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_textures_2d-array': {'example':80,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['109層をGPU配列テクスチャに保持してレイヤーをシェーダーで選択。性能の完全な同等性は未保証。']},
 'webgpu_multisampled_renderbuffers': {'example':78,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_layers': {'example':79,'source':'src/browser/tsl_extended.rs','test':'tests/browser/tsl-extended.spec.js','limitations':['Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_tsl_vfx_flames': {'example':66,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['公式の2種類のGPU炎シェーダーと水平Billboardを移植。性能の完全な同等性は未保証。']},
 'webgpu_custom_fog_background': {'example':67,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['MSAA深度からTSLの霧を計算し、HDRトーンマッピング後の色と合成。性能の完全な同等性は未保証。']},
 'webgpu_lights_selective': {'example':68,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['材質ごとのライト選択とTSL材質入力、公式Teapot形状を移植。性能の完全な同等性は未保証。']},
 'webgpu_lights_phong': {'example':69,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['材質ごとのライト選択とTSL材質入力、公式Teapot形状を移植。性能の完全な同等性は未保証。']},
 'webgpu_mrt': {'example':70,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['HDR色と8bit法線・拡散色・発光色の混合形式MRTを移植。性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_bloom': {'example':71,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['公式の5段階HDR BloomをGPUで移植。性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_bloom_emissive': {'example':72,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['公式の5段階HDR BloomをGPUで移植。性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_bloom_selective': {'example':73,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['MRTマスクとGPU Bloom、クリックによる対象選択を移植。比較用に固定乱数を使用。性能の完全な同等性は未保証。']},
 'webgpu_storage_buffer': {'example':74,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['WebGPU側のみ全幅表示。4種類のStorage BufferをGPU同期・反転。WebGL比較欄とタイムスタンプUIは対象外。']},
 'webgpu_compute_geometry': {'example':75,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['常駐位置と速度をGPU Computeで更新するJelly変形。性能の完全な同等性は未保証。']},
 'webgpu_tsl_vfx_tornado': {'example':76,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['GPU頂点変形・ノイズ・HDR Bloomによる竜巻を移植。性能の完全な同等性は未保証。']},
 'webgpu_mrt_mask': {'example':77,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['GPUスキニングと材質ごとのMRTマスク、2方向Gaussian Blurを移植。性能の完全な同等性は未保証。']},
 'webgpu_shadertoy': {'example':65,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['公式の2種類の固定シェーダーをRust TSL/WGSLで移植。任意GLSLを変換するTranspiler APIは未対応。']},
 'webgpu_lights_custom': {'example':64,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['50万点の常駐位置とTSLカスタム照明をGPUで計算。比較用に固定乱数を使用。性能の完全な同等性は未保証。']},
 'webgpu_multiple_rendertargets': {'example':63,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['TSLによる色・法線のMRT出力を1回の描画で生成し左右に表示。性能の完全な同等性は未保証。']},
 'webgpu_depth_texture': {'example':62,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['50オブジェクトの深度アタッチメントをGPU上で直接サンプリング。比較用に固定乱数を使用。性能の完全な同等性は未保証。']},
 'webgpu_skinning': {'example':61,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['公式Michelleモデル・GPUスキニング・TSL背景・Linear tone mappingを移植。性能の完全な同等性は未保証。']},
 'webgpu_tsl_halftone': {'example':60,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['公式モデルと画面座標・法線に基づくTSLハーフトーンと調整を移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_tsl_raging_sea': {'example':59,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['GPUの波変形・MaterialXノイズ・法線・発光と15個の調整項目を移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_lights_rectarealight': {'example':58,'source':'src/browser/tsl_surface.rs','test':'tests/browser/tsl-surface.spec.js','limitations':['TSL粗さノード・LTC面光源と回転・Orbit操作を移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_particles': {'example':53,'source':'src/browser/tsl_compute.rs','test':'tests/browser/tsl-compute.spec.js','limitations':['2000煙・1000炎のGPUビルボードと間接描画。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_instance_mesh': {'example':54,'source':'src/browser/tsl_compute.rs','test':'tests/browser/tsl-compute.spec.js','limitations':['1000 SuzanneインスタンスとTSLの色。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_compute_points': {'example':55,'source':'src/browser/tsl_compute.rs','test':'tests/browser/tsl-compute.spec.js','limitations':['30万点のGPUストレージ更新と点描画。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_compute_particles': {'example':56,'source':'src/browser/tsl_compute.rs','test':'tests/browser/tsl-compute.spec.js','limitations':['20万粒子のGPU物理更新・ポインター入力とAlpha to Coverage。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_compute_texture_pingpong': {'example':57,'source':'src/browser/tsl_compute.rs','test':'tests/browser/tsl-compute.spec.js','limitations':['512×512 HDRテクスチャのGPU交互更新。Inspector外観と性能の完全な同等性は未保証。']},

 'webgpu_fog_height': {'example':48,'source':'src/browser/tsl_particles.rs','test':'tests/browser/tsl-particles.spec.js','limitations':['TSL高さフォグと100インスタンス。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_sprites': {'example':49,'source':'src/browser/tsl_particles.rs','test':'tests/browser/tsl-particles.spec.js','limitations':['200スプライトのGPUビルボード・個別回転・フォグ。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_instance_sprites': {'example':50,'source':'src/browser/tsl_particles.rs','test':'tests/browser/tsl-particles.spec.js','limitations':['10000スプライトのGPU回転・インスタンス属性。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_tsl_galaxy': {'example':51,'source':'src/browser/tsl_particles.rs','test':'tests/browser/tsl-particles.spec.js','limitations':['20000粒子のGPU位置・色・拡縮。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_afterimage': {'example':52,'source':'src/browser/tsl_particles.rs','test':'tests/browser/tsl-particles.spec.js','limitations':['50000粒子のGPUアニメーションと履歴テクスチャ。Inspector外観と性能の完全な同等性は未保証。']},

 'webgpu_postprocessing_direct': {'example':43,'source':'src/browser/tsl_filters.rs','test':'tests/browser/tsl-filters.spec.js','limitations':['描画シェーダー内のTSL saturationとNeutral tone mappingを移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_radial_blur': {'example':44,'source':'src/browser/tsl_filters.rs','test':'tests/browser/tsl-filters.spec.js','limitations':['100インスタンスとGPU可変サンプル数Radial Blurを移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_fxaa': {'example':45,'source':'src/browser/tsl_filters.rs','test':'tests/browser/tsl-filters.spec.js','limitations':['100インスタンスとsRGB空間の適応的FXAAを移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_ssaa': {'example':46,'source':'src/browser/tsl_filters.rs','test':'tests/browser/tsl-filters.spec.js','limitations':['120インスタンスと1〜32回のジッター付きGPU描画・加算を移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_transition': {'example':47,'source':'src/browser/tsl_filters.rs','test':'tests/browser/tsl-filters.spec.js','limitations':['2シーン・各500インスタンスと6種の遷移マスクを移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_compute_texture': {'example':38,'source':'src/browser/tsl_passes.rs','test':'tests/browser/tsl-passes.spec.js','limitations':['Rust TSLのcompute・textureStoreでGPUテクスチャを生成。描画は変更時のみ。情報表示の外観と性能の完全な同等性は未保証。']},
 'webgpu_rtt': {'example':39,'source':'src/browser/tsl_passes.rs','test':'tests/browser/tsl-passes.spec.js','limitations':['TSLのrender-to-texture・saturation・hueとマウス操作を移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing': {'example':40,'source':'src/browser/tsl_passes.rs','test':'tests/browser/tsl-passes.spec.js','limitations':['100個のPhong球とDot Screen・RGB ShiftをTSLで移植。比較可能な固定乱数を使用。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_difference': {'example':41,'source':'src/browser/tsl_passes.rs','test':'tests/browser/tsl-passes.spec.js','limitations':['前フレームのGPUテクスチャを参照するTSL合成・速度調整・Orbit操作を移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_postprocessing_masking': {'example':42,'source':'src/browser/tsl_passes.rs','test':'tests/browser/tsl-passes.spec.js','limitations':['3シーンのGPU描画・アルファマスク・TSL画像合成を移植。Inspector外観と性能の完全な同等性は未保証。']},
 'webgpu_tsl_interoperability': {'example':35,'source':'src/browser/tsl_examples.rs','test':'tests/browser/tsl.spec.js','limitations':['RustのTSLノードAPIとWGSL関数連携でCRTを移植。Inspector UIの外観と性能の完全な同等性は未保証。']},
 'webgpu_texturegrad': {'example':36,'source':'src/browser/tsl_examples.rs','test':'tests/browser/tsl.spec.js','limitations':['公式のWebGPU側を単独表示。WebGL比較パネルは対象外。RustのTSLによる勾配付きGPUテクスチャサンプリング。性能の完全な同等性は未保証。']},
 'webgpu_procedural_texture': {'example':37,'source':'src/browser/tsl_examples.rs','test':'tests/browser/tsl.spec.js','limitations':['RustのTSLからGPUテクスチャ生成・2段Gaussian blurと調整を移植。Inspector UIの外観と性能の完全な同等性は未保証。']},
 'webgpu_loader_gltf_dispersion': {'example':33,'source':'src/browser/gltf_examples.rs','test':'tests/browser/gltf-physical.spec.js','limitations':['公式DispersionTest・色分散・HDR・Orbit操作を移植。情報表示の外観は未一致。性能の完全な同等性は未保証。']},
 'webgpu_loader_gltf_compressed': {'example':34,'source':'src/browser/gltf_examples.rs','test':'tests/browser/gltf-physical.spec.js','limitations':['公式coffeemat・Meshopt・KTX2/BasisのGPU圧縮テクスチャ・Orbit操作を移植。対応GPU圧縮形式が必要。情報表示と性能の完全な同等性は未保証。']},
 'webgl_loader_gltf_instancing': {'example':16,'source':'src/browser/expanded.rs','test':'tests/browser/gltf-instancing.spec.js','limitations':['試作版：公式モデルのGPUインスタンシングとOrbit操作を実装。公式WebGL版とのMSAA・金属反射の描画差と、初期化ごとの描画変動は調査中。外観・性能の同等性は未確認。']},
 'webgpu_loader_gltf_anisotropy': {'example':30,'source':'src/browser/gltf_examples.rs','test':'tests/browser/gltf-physical.spec.js','limitations':['公式の異方性反射・クリアコート・透過材質とOrbit操作を移植。情報表示とInspector UIは未一致。']},
 'webgpu_loader_gltf_sheen': {'example':31,'source':'src/browser/gltf_examples.rs','test':'tests/browser/gltf-physical.spec.js','limitations':['公式SheenChair・Sheen調整・減衰付きOrbit操作を移植。調整UIの外観と情報表示は未一致。']},
 'webgpu_loader_gltf_transmission': {'example':32,'source':'src/browser/gltf_examples.rs','test':'tests/browser/gltf-physical.spec.js','limitations':['公式の透過・玉虫色材質・蓋のアニメーション・自動回転とOrbit操作を移植。情報表示とInspector UIは未一致。']},
 'webgpu_loader_gltf_iridescence': {'example':29,'source':'src/browser/gltf_examples.rs','test':'tests/browser/gltf-iridescence.spec.js','limitations':['公式IridescenceLamp・HDR環境・自動回転・Orbit操作を移植。情報表示は未一致。一般の屈折・拡張材質と性能の完全互換は未保証。']},
 'webgl_loader_gltf_avif': {'example':28,'source':'src/browser/expanded.rs','test':'tests/browser/gltf-avif.spec.js','limitations':['公式Forest HouseのDraco形状とAVIFテクスチャを読み込み、Orbit・パン・ズームと変更時のみの描画を移植。MSAAの輪郭と情報オーバーレイの外観は未一致。性能同等性は未保証。']},
 'webgl_buffergeometry': {'example':26,'source':'src/browser/expanded_triangles.rs','test':'tests/browser/expanded.spec.js','limitations':['公式の16万三角形・頂点RGBA・Phong照明・霧・回転を移植。比較用の固定乱数を使用。Stats表示は未移植。汎用頂点形式によるGPUメモリ増加が残り、完全な性能同等性は未達。']},
 'webgl_buffergeometry_rawshader': {'example':27,'source':'src/browser/expanded_triangles.rs','test':'tests/browser/expanded.spec.js','limitations':['公式の200三角形・頂点RGBA・色アニメーションをWGSLへ移植。比較用の固定乱数を使用。Stats表示は未移植。汎用頂点形式のメモリ使用量は公式より大きい。']},
 'webgl_geometry_colors': {'example': 21, 'source': 'src/browser/expanded_geometry_colors.rs', 'test': 'tests/browser/expanded.spec.js', 'limitations': ['公式3個の頂点色Icosahedron・ワイヤー・影画像・ポインターカメラを移植。']},
 'webgl_buffergeometry_indexed': {'example': 22, 'source': 'src/browser/expanded_indexed.rs', 'test': 'tests/browser/expanded.spec.js', 'limitations': ['公式のインデックス付き色グリッド・回転・ワイヤーフレーム切替を移植。GUI外観とStatsは未一致。']},
 'webgl_lines_colors': {'example': 23, 'source': 'src/browser/expanded_lines.rs', 'test': 'tests/browser/expanded.spec.js', 'limitations': ['公式の6色線・Hilbert/Catmull-Rom・回転・ポインターカメラを移植。']},
 'webgl_morphtargets_horse': {'example': 24, 'source': 'src/browser/expanded_morph_models.rs', 'test': 'tests/browser/expanded.spec.js', 'limitations': ['公式Horse・GPUモーフ・変形後の面法線・1秒ループ・周回カメラを移植。Stats表示は未移植。']},
 'webgl_morphtargets_sphere': {'example': 25, 'source': 'src/browser/expanded_morph_models.rs', 'test': 'tests/browser/expanded.spec.js', 'limitations': ['公式AnimatedMorphSphere・GPUモーフ・点スプライト・Orbit操作を移植。WebGPUでは点をGPUで展開した四角形として描画。']},
 'webgpu_morphtargets': {'example': 18, 'source': 'src/browser/expanded.rs', 'test': 'tests/browser/expanded.spec.js', 'limitations': ['公式WebGPU版のGPUモーフシーンと操作を移植。Inspector全体のUIは未移植。']},
 'webgl_lines_dashed': {'example': 19, 'source': 'src/browser/expanded_lines.rs', 'test': 'tests/browser/expanded.spec.js', 'limitations': ['公式Hilbert曲線・Catmull-Rom補間・破線・霧・回転を移植。1ピクセル線はGPU線プリミティブ。Stats表示は未移植。']},
 'webgl_geometries': {'example':17,'source':'src/browser/expanded.rs','test':'tests/browser/expanded.spec.js','limitations':['公式16形状・Phong材質・カメラと物体の回転を移植。3時刻で画像比較、GPU保持と負荷を検証。Stats表示は未移植。']},
 'webgl_animation_skinning_morph': {'example': 15, 'source': 'src/browser/robot.rs', 'test': 'tests/browser/core-robot.spec.js', 'limitations': ['公式RobotExpressive・14クリップ・スキニング・モーフ・照明・床・FogをRustへ移植。クリップ切替とOrbit操作を追加。公式GUIの表情スライダー・一時動作の復帰・Statsは未移植。']},
 'webgpu_pmrem_test': {'example': 14, 'source': 'src/browser/gallery_scenes.rs', 'test': 'tests/browser/gallery-scenes.spec.js', 'limitations': ['公式WebGPU版の33球の光量比較シーンを移植。InspectorとOrbit制限は未一致。']},
 'webgpu_equirectangular': {'example': 13, 'source': 'src/browser/gallery_scenes.rs', 'test': 'tests/browser/gallery-scenes.spec.js', 'limitations': ['公式画像を直接サンプリングする背景、自動回転と明るさ調整を移植。Inspector・Orbitの操作感は未一致。']},
 'webgl_materials_texture_rotation': {'example': 11, 'source': 'src/browser/gallery_scenes.rs', 'test': 'tests/browser/gallery-scenes.spec.js', 'limitations': ['UVのoffset・repeat・rotation・centerとドラッグを移植。異方性フィルタリング対応。Orbit制限とGUI外観は未一致。']},
 'webgl_buffergeometry_lines': {'example': 12, 'source': 'src/browser/gallery_scenes.rs', 'test': 'tests/browser/gallery-scenes.spec.js', 'limitations': ['10000頂点の線モーフをGPU頂点シェーダーで処理。回転・モーフ重みのみCPUで更新。乱数は固定seed、Statsは未移植。']},
 'webgl_buffergeometry_lines_indexed': {'example': 7, 'source': 'src/browser/gallery_scenes.rs', 'test': 'tests/browser/gallery-scenes.spec.js', 'limitations': ['雪片の再帰生成・頂点色・回転を移植。乱数は比較可能な固定seed。Stats表示は未移植。']},
 'webgl_interactive_lines': {'example': 9, 'source': 'src/browser/gallery_scenes.rs', 'test': 'tests/browser/gallery-scenes.spec.js', 'limitations': ['50個の線、カメラ回転、線レイキャストと交点マーカーを移植。頂点と変換を公式と比較。WebGLの線ラスタライズ・Statsは未一致。乱数は固定seed。']},
 'webgl_interactive_raycasting_points': {'example': 10, 'source': 'src/browser/gallery_scenes.rs', 'test': 'tests/browser/gallery-scenes.spec.js', 'limitations': ['3種類の点群、回転カメラ、点レイキャスト、40個の縮小マーカーを移植。頂点と変換を公式と比較。WebGLの点ラスタライズ・Statsは未一致。']},
 'webgpu_pmrem_equirectangular': {'example': 6, 'source': 'src/browser/gallery.rs', 'test': 'tests/browser/gallery.spec.js', 'limitations': ['30個の球・材質・HDR背景は固定視点で公式と画像比較済み。Orbitの慣性・パン・ズーム制限とInspectorは未再現。']},
 'webgl_geometry_cube': {'example': 2, 'source': 'src/browser.rs', 'test': 'tests/browser/migration.spec.js', 'limitations': ['既存移植はMSAA・ミップマップなし。公式の全設定との一致は未達。']},
 'webgpu_loader_gltf': {'example': 4, 'source': 'src/browser/gltf_viewer.rs', 'test': 'tests/browser/gltf-pbr.spec.js', 'limitations': ['DamagedHelmet / BoomBoxはPBR比較済み。公式の全モデルカタログ、アニメーション、圧縮・拡張材質は未対応。']},
 'webgpu_lights_pointlights': {'example': 3, 'source': 'src/browser/point_lights.rs', 'test': 'tests/browser/point-lights.spec.js', 'limitations': ['変形はGPUのWGSL頂点シェーダーで実行。公式PhongをStandardへ変更しており、完全一致ではない。']},
}
UI_FILES = ['main.css', 'RobotoMono-Regular.woff2', 'RobotoMono-Medium.woff2', 'favicon.ico', 'favicon_white.ico', 'ic_close_black_24dp.svg', 'ic_menu_black_24dp.svg', 'ic_search_black_24dp.svg', 'ic_code_black_24dp.svg', 'ic_arrow_drop_down_black_24dp.svg']

def write(path, data):
 path.parent.mkdir(parents=True, exist_ok=True)
 payload = data if isinstance(data, bytes) else data.encode()
 if '--check' in sys.argv and not path.is_relative_to(ROOT/'.cache'):
  if not path.exists() or path.read_bytes()!=payload:
   raise SystemExit(f'Generated gallery drift: {path.relative_to(ROOT)}; run tools/gallery/build.py')
 else: path.write_bytes(payload)

def main():
 with tarfile.open(ARCHIVE) as tar:
  bundled = {}
  for entry in tar:
   path = entry.name.split('/', 1)[-1]
   if entry.isfile() and (path in ('examples/textures/equirectangular/spot1Lux.hdr','examples/jsm/loaders/HDRLoader.js','examples/textures/2294472375_24a3b8ef46_o.jpg','examples/textures/uv_grid_opengl.jpg','examples/jsm/controls/OrbitControls.js') or path.startswith('files/') or (path.startswith('examples/') and (path.endswith('.html') or path.startswith('examples/screenshots/') or path in ('examples/files.json', 'examples/tags.json', 'examples/files/thumbnails.svg')))):
    bundled[path] = tar.extractfile(entry).read()
  def read(path): return bundled[path]
  # Original shell for independent visual comparison; runtime never imports it.
  for path in ['examples/index.html', 'examples/files.json', 'examples/tags.json', 'examples/files/thumbnails.svg', *['files/'+name for name in UI_FILES]]:
   write(ROOT/'.cache/three-r186'/path, read(path))
  write(OUT/'assets/panorama.jpg', read('examples/textures/2294472375_24a3b8ef46_o.jpg'))
  write(OUT/'assets/uv-grid.jpg', read('examples/textures/uv_grid_opengl.jpg'))
  write(ROOT/'.cache/three-r186/examples/jsm/controls/OrbitControls.js',read('examples/jsm/controls/OrbitControls.js'))
  write(OUT/'assets/spot1Lux.hdr', read('examples/textures/equirectangular/spot1Lux.hdr'))
  write(ROOT/'.cache/three-r186/examples/jsm/loaders/HDRLoader.js',read('examples/jsm/loaders/HDRLoader.js'))
  files = json.loads(read('examples/files.json'))
  tags = json.loads(read('examples/tags.json'))
  entries = []
  included = {}
  for category, names in files.items():
   for name in names:
    source = read(f'examples/{name}.html').decode()
    write(ROOT/'.cache/three-r186/examples'/f'{name}.html', source)
    evidence = {}
    for feature, (_, pattern) in FEATURES.items():
     matches = [{'line': i, 'text': line.strip()[:180]} for i, line in enumerate(source.splitlines(), 1) if re.search(pattern, line)]
     if matches: evidence[feature] = matches[:4]
    # Renderer names, GLSL, glTF EXT_* extensions and backend capability checks
    # are NOT grounds for exclusion: depth/compressed-texture demos are portable.
    gl = re.search(r'\bGLBufferAttribute\b|renderer\.getContext\s*\(|WEBGL_clip_cull_distance', source) if name.startswith('webgl') else None
    excluded = f'WebGL-specific API: {gl.group()}' if gl else None
    preferred = WEBGPU_EQUIVALENTS.get(name)
    if preferred:
     excluded = f'Equivalent WebGPU example preferred: {preferred}'
    port = PORTS.get(name)
    status = 'excluded' if excluded else 'partial' if port else 'not-ported'
    entry = {'id': name, 'category': category, 'status': status, 'excluded_reason': excluded,
             'source': f'https://github.com/mrdoob/three.js/blob/{REV}/examples/{name}.html',
             'source_sha256': hashlib.sha256(source.encode()).hexdigest(),
             'requirements': evidence, 'port': port,
             'imports': sorted(set(re.findall(r"from\s*['\"]([^'\"]+)['\"]", source)))}
    if preferred: entry['preferred_example'] = preferred
    entries.append(entry)
    if port and not excluded:
     included.setdefault(category, []).append(name)
    if not excluded:
     write(OUT / 'screenshots' / f'{name}.jpg', read(f'examples/screenshots/{name}.jpg'))
  for name in UI_FILES: write(ROOT/'web/files'/name, read(f'files/{name}'))
  write(OUT/'files/thumbnails.svg', read('examples/files/thumbnails.svg'))
  index = read('examples/index.html').decode()
  index = index.replace('<title>three.js examples</title>', '<title>three-rs-wasm examples</title>')
  index = index.replace("file + '.html'", "'example.html?id=' + encodeURIComponent( file )")
  index = index.replace("'https://github.com/mrdoob/three.js/blob/master/examples/'", f"'https://github.com/mrdoob/three.js/blob/{REV}/examples/'")
  # Restore selection on browser back/forward as well as direct hash navigation.
  index = index.replace('// iOS iframe auto-resize workaround', """window.addEventListener( 'hashchange', () => {
                const file = window.location.hash.substring( 1 );
                if ( validRedirects.has( file ) && selected !== file ) {
                    selectFile( file ); viewer.src = validRedirects.get( file ); updateLinkScroll();
                }
            } );

            // iOS iframe auto-resize workaround""")
  write(OUT/'index.html', index)
  write(OUT/'files.json', json.dumps(included, indent=2)+'\n')
  write(OUT/'tags.json', json.dumps(tags, indent=2)+'\n')
  catalog = {'revision': REV, 'analysis': 'Source-level requirement detection, not proof of runtime compatibility. Unported examples are never substituted with a different scene.', 'features': {k:v[0] for k,v in FEATURES.items()}, 'examples': entries}
  write(OUT/'catalog.json', json.dumps(catalog, ensure_ascii=False, indent=2)+'\n')
  counts = Counter(e['status'] for e in entries)
  lines = ['# Three.js examples coverage', '', f'Pinned reference: `{REV}`. {len(entries)} examples inspected.', '', f"{counts['excluded']} excluded for explicit WebGL APIs or equivalent WebGPU examples; {len(entries)-counts['excluded']} retained. {counts['partial']} partial Rust ports; {counts['not-ported']} not yet ported. Only runnable ports appear in the gallery list. No complete-gallery reproduction claim.", '', 'All entries, source hashes, source-line evidence and exclusions: [`catalog.json`](../web/gallery/catalog.json).', '', 'See [example selection and comparison policy](example-policy.md).', '', 'See [performance acceptance and current audit](performance-parity.md). The counts below identify source usage, not proven independent blockers; do not add them together. TSL usage can be ported to Rust/WGSL without implementing TSL itself.', '', '| Required capability (source inventory, not current support status) | Examples mentioning it |', '| --- | ---: |']
  for key, n in Counter(k for e in entries if e['status']!='excluded' for k in e['requirements']).most_common(): lines.append(f'| {FEATURES[key][0]} | {n} |')
  lines += ['', '## Existing runnable ports', '', '| Example | Evidence | Remaining differences |', '| --- | --- | --- |']
  for name,p in PORTS.items(): lines.append(f"| {name} | `{p['test']}` | {' '.join(p['limitations'])} |")
  lines += ['', 'Detailed per-example port prerequisites and actual importer results: [port attempts](gallery-port-attempts.md).', '', 'Before the WebGPU preference policy, 605 upstream pages were also executed through a diagnostic bridge, with eligible captured scenes sent to the native Rust renderer: [runtime outcomes and limitations](gallery-runtime-results.md). These sampled frames are not behavioral ports and are not added to the gallery.', '', '## Priority', '', '1. Programmable materials and reusable WGSL effect interfaces, GPU compute/storage, and postprocessing targets/history unlock the largest group of WebGPU scenes.', '2. Animation/skinning/morph targets and compressed/additional asset loaders unlock animated models.', '3. Shadows, physical material extensions and advanced lights close appearance gaps.', '4. Instancing/batching and procedural geometry enable large scenes.', '5. WebXR, physics, spatial audio and DOM/SVG renderers require their own integrations and validation environments.', '', 'Gallery thumbnails are the original upstream previews, not screenshots of completed Rust ports. Unported entries show a diagnostic instead of executing Three.js or pretending another scene reproduces the example.', '']
  write(ROOT/'docs/examples-coverage.md', '\n'.join(lines))
  print(json.dumps({'total':len(entries), **counts}))

if __name__ == '__main__': main()

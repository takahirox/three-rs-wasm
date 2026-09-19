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
WEBGPU_EQUIVALENTS = {'webgl_loader_gltf': 'webgpu_loader_gltf', 'webgl_morphtargets': 'webgpu_morphtargets', 'webgl_pmrem_test': 'webgpu_pmrem_test', 'webgl_pmrem_equirectangular': 'webgpu_pmrem_equirectangular', 'webgl_panorama_equirectangular': 'webgpu_equirectangular', 'webgl_lights_rectarealight': 'webgpu_lights_rectarealight'}
PORTS = {
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

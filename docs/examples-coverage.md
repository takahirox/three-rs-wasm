# Three.js examples coverage

Pinned reference: `148ef33ecb6d2502ff796d4554abd1549c95d519`. 607 examples inspected.

2 excluded for explicit WebGL APIs; 605 retained. 26 partial Rust ports; 579 not yet ported. Only runnable ports appear in the gallery list. No complete-gallery reproduction claim.

All entries, source hashes, source-line evidence and exclusions: [`catalog.json`](../web/gallery/catalog.json).

See [performance acceptance and current audit](performance-parity.md). The counts below identify source usage, not proven independent blockers; do not add them together. TSL usage can be ported to Rust/WGSL without implementing TSL itself.

| Required capability (source inventory, not current support status) | Examples mentioning it |
| --- | ---: |
| Full camera controls: pan, touch, damping and control variants | 393 |
| Programmable materials / TSL equivalents | 219 |
| Phong, Lambert, normal, depth, toon and matcap materials | 181 |
| Additional procedural geometry builders | 176 |
| Inspector and per-example GUI parity | 175 |
| Shadow maps and shadow filtering | 131 |
| Hemisphere/spot/area lights, light probes and baking | 125 |
| Postprocessing passes and temporal history | 103 |
| Distance and height fog | 95 |
| Wireframe materials and scene helpers | 92 |
| Additional loaders and compressed assets | 86 |
| Instance transforms and batched drawing | 75 |
| Animation mixer and skeletal animation | 49 |
| Canvas, HTML, video and partial texture updates | 49 |
| Transmission, clearcoat, sheen, anisotropy and related PBR extensions | 48 |
| WebXR sessions, controllers and XR render targets | 32 |
| Configurable blend equations and factors | 30 |
| GPU compute and storage buffers | 30 |
| Volume rendering and layered textures | 18 |
| Physics integration | 14 |
| Curve interpolation and path builders | 13 |
| Morph target animation | 11 |
| CSS2D/CSS3D/SVG scene renderers | 11 |
| Wide / dashed line rendering | 8 |
| Clipping planes and stencil operations | 7 |
| Spatial audio and audio analysis | 6 |
| Stereo, anaglyph and parallax-barrier effects | 3 |

## Existing runnable ports

| Example | Evidence | Remaining differences |
| --- | --- | --- |
| webgl_lights_rectarealight | `tests/browser/expanded.spec.js` | 公式の3色の面光源・LTC・ヘルパー・粗さ床・TorusKnotとOrbit操作を移植。Stats表示は未移植。 |
| webgl_geometry_colors | `tests/browser/expanded.spec.js` | 公式3個の頂点色Icosahedron・ワイヤー・影画像・ポインターカメラを移植。 |
| webgl_buffergeometry_indexed | `tests/browser/expanded.spec.js` | 公式のインデックス付き色グリッド・回転・ワイヤーフレーム切替を移植。GUI外観とStatsは未一致。 |
| webgl_lines_colors | `tests/browser/expanded.spec.js` | 公式の6色線・Hilbert/Catmull-Rom・回転・ポインターカメラを移植。 |
| webgl_morphtargets_horse | `tests/browser/expanded.spec.js` | 公式Horse・GPUモーフ・変形後の面法線・1秒ループ・周回カメラを移植。Stats表示は未移植。 |
| webgl_morphtargets_sphere | `tests/browser/expanded.spec.js` | 公式AnimatedMorphSphere・GPUモーフ・点スプライト・Orbit操作を移植。WebGPUでは点をGPUで展開した四角形として描画。 |
| webgl_morphtargets | `tests/browser/expanded.spec.js` | 公式の球化・ねじり・混合モーフをGPUで処理。スライダー、回転・パン・ズーム無効化を比較済み。GUIの外観は未一致。 |
| webgpu_morphtargets | `tests/browser/expanded.spec.js` | WebGL版と同じGPUモーフシーンと操作を共有。Inspector全体のUIは未移植。 |
| webgl_lines_dashed | `tests/browser/expanded.spec.js` | 公式Hilbert曲線・Catmull-Rom補間・破線・霧・回転を移植。1ピクセル線はGPU線プリミティブ。Stats表示は未移植。 |
| webgl_geometries | `tests/browser/expanded.spec.js` | 公式16形状・Phong材質・カメラと物体の回転を移植。3時刻で画像比較、GPU保持と負荷を検証。Stats表示は未移植。 |
| webgl_animation_skinning_morph | `tests/browser/core-robot.spec.js` | 公式RobotExpressive・14クリップ・スキニング・モーフ・照明・床・FogをRustへ移植。クリップ切替とOrbit操作を追加。公式GUIの表情スライダー・一時動作の復帰・Statsは未移植。 |
| webgl_pmrem_test | `tests/browser/gallery-scenes.spec.js` | 公式spot1Lux HDR・33球・PMREMと直接光の切り替えを移植。Orbit制限とGUI外観は未一致。 |
| webgpu_pmrem_test | `tests/browser/gallery-scenes.spec.js` | WebGL版と同じ33球の光量比較シーンを共有。InspectorとOrbit制限は未一致。 |
| webgpu_equirectangular | `tests/browser/gallery-scenes.spec.js` | 公式画像を直接サンプリングする背景、自動回転と明るさ調整を移植。Inspector・Orbitの操作感は未一致。 |
| webgl_pmrem_equirectangular | `tests/browser/gallery-scenes.spec.js` | WebGPU版と同じ30球・HDRのシーンを共有。WebGLのPMREMとの描画差を比較ログに記録。Orbit制限は未一致。 |
| webgl_loader_gltf | `tests/browser/gltf-pbr.spec.js` | WebGPU版と共有するDamagedHelmet / BoomBoxの静的PBR移植。全モデルカタログ・アニメーション・拡張材質・GUIは未再現。 |
| webgl_materials_texture_rotation | `tests/browser/gallery-scenes.spec.js` | UVのoffset・repeat・rotation・centerとドラッグを移植。異方性フィルタリング対応。Orbit制限とGUI外観は未一致。 |
| webgl_buffergeometry_lines | `tests/browser/gallery-scenes.spec.js` | 10000頂点の線モーフをGPU頂点シェーダーで処理。回転・モーフ重みのみCPUで更新。乱数は固定seed、Statsは未移植。 |
| webgl_buffergeometry_lines_indexed | `tests/browser/gallery-scenes.spec.js` | 雪片の再帰生成・頂点色・回転を移植。乱数は比較可能な固定seed。Stats表示は未移植。 |
| webgl_panorama_equirectangular | `tests/browser/gallery-scenes.spec.js` | 公式写真・内向き球・自動回転・ドラッグ・FOVズームを移植。情報表示の見た目は未一致。 |
| webgl_interactive_lines | `tests/browser/gallery-scenes.spec.js` | 50個の線、カメラ回転、線レイキャストと交点マーカーを移植。頂点と変換を公式と比較。WebGLの線ラスタライズ・Statsは未一致。乱数は固定seed。 |
| webgl_interactive_raycasting_points | `tests/browser/gallery-scenes.spec.js` | 3種類の点群、回転カメラ、点レイキャスト、40個の縮小マーカーを移植。頂点と変換を公式と比較。WebGLの点ラスタライズ・Statsは未一致。 |
| webgpu_pmrem_equirectangular | `tests/browser/gallery.spec.js` | 30個の球・材質・HDR背景は固定視点で公式と画像比較済み。Orbitの慣性・パン・ズーム制限とInspectorは未再現。 |
| webgl_geometry_cube | `tests/browser/migration.spec.js` | 既存移植はMSAA・ミップマップなし。公式の全設定との一致は未達。 |
| webgpu_loader_gltf | `tests/browser/gltf-pbr.spec.js` | DamagedHelmet / BoomBoxはPBR比較済み。公式の全モデルカタログ、アニメーション、圧縮・拡張材質は未対応。 |
| webgpu_lights_pointlights | `tests/browser/point-lights.spec.js` | 変形はGPUのWGSL頂点シェーダーで実行。公式PhongをStandardへ変更しており、完全一致ではない。 |

Detailed per-example port prerequisites and actual importer results: [port attempts](gallery-port-attempts.md).

All 605 retained upstream pages were also executed through a diagnostic bridge, with eligible captured scenes sent to the native Rust renderer: [runtime outcomes and limitations](gallery-runtime-results.md). These sampled frames are not behavioral ports and are not added to the gallery.

## Priority

1. Programmable materials and reusable WGSL effect interfaces, GPU compute/storage, and postprocessing targets/history unlock the largest group of WebGPU scenes.
2. Animation/skinning/morph targets and compressed/additional asset loaders unlock animated models.
3. Shadows, physical material extensions and advanced lights close appearance gaps.
4. Instancing/batching and procedural geometry enable large scenes.
5. WebXR, physics, spatial audio and DOM/SVG renderers require their own integrations and validation environments.

Gallery thumbnails are the original upstream previews, not screenshots of completed Rust ports. Unported entries show a diagnostic instead of executing Three.js or pretending another scene reproduces the example.

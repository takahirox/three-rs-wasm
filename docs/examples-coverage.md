# Three.js examples coverage

Pinned reference: `148ef33ecb6d2502ff796d4554abd1549c95d519`. 607 examples inspected.

14 excluded for explicit WebGL APIs or equivalent WebGPU examples; 593 retained. 148 partial Rust ports; 445 not yet ported. Only runnable ports appear in the gallery list. No complete-gallery reproduction claim.

All entries, source hashes, source-line evidence and exclusions: [`catalog.json`](../web/gallery/catalog.json).

See [example selection and comparison policy](example-policy.md).

See [performance acceptance and current audit](performance-parity.md). The counts below identify source usage, not proven independent blockers; do not add them together. TSL usage can be ported to Rust/WGSL without implementing TSL itself.

| Required capability (source inventory, not current support status) | Examples mentioning it |
| --- | ---: |
| Full camera controls: pan, touch, damping and control variants | 385 |
| Programmable materials / TSL equivalents | 219 |
| Phong, Lambert, normal, depth, toon and matcap materials | 178 |
| Inspector and per-example GUI parity | 175 |
| Additional procedural geometry builders | 174 |
| Shadow maps and shadow filtering | 130 |
| Hemisphere/spot/area lights, light probes and baking | 123 |
| Postprocessing passes and temporal history | 103 |
| Distance and height fog | 93 |
| Wireframe materials and scene helpers | 91 |
| Additional loaders and compressed assets | 85 |
| Instance transforms and batched drawing | 75 |
| Animation mixer and skeletal animation | 48 |
| Transmission, clearcoat, sheen, anisotropy and related PBR extensions | 47 |
| Canvas, HTML, video and partial texture updates | 46 |
| WebXR sessions, controllers and XR render targets | 32 |
| Configurable blend equations and factors | 30 |
| GPU compute and storage buffers | 30 |
| Volume rendering and layered textures | 18 |
| Physics integration | 14 |
| Curve interpolation and path builders | 13 |
| CSS2D/CSS3D/SVG scene renderers | 11 |
| Morph target animation | 10 |
| Wide / dashed line rendering | 8 |
| Clipping planes and stencil operations | 6 |
| Spatial audio and audio analysis | 6 |
| Stereo, anaglyph and parallax-barrier effects | 3 |

## Existing runnable ports

| Example | Evidence | Remaining differences |
| --- | --- | --- |
| webgpu_materials_arrays | `tests/browser/material-textures.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_clipping | `tests/browser/material-textures.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_texture_manualmipmap | `tests/browser/material-textures.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_textures_anisotropy | `tests/browser/material-textures.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_textures_partialupdate | `tests/browser/material-textures.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_upscaling_taau | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_motion_blur | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_traa | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_retroreflection | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_compute_particles_rain | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_compute_particles_snow | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_pixel | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 公式と同じく描画解像度はDPR 1。 |
| webgpu_reflection_blurred | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_compute_audio | `tests/browser/tsl-audio.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_reflection | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_mirror | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_shadowmap | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_multiple_rendertargets_readback | `tests/browser/tsl-procedural.spec.js` | 非同期GPU readbackにより結果の表示まで数フレームかかる場合があります。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_tsl_procedural_terrain | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_tsl_angular_slicing | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_tsl_wood | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 公式と同じく描画解像度はDPR 1。 |
| webgpu_skinning_points | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_reflection_roughness | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materialx_noise | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_portal | `tests/browser/tsl-procedural.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials | `tests/browser/tsl-primitives.spec.js` | r186公式のLoop出力が黒になる挙動を維持。汎用Loopノードは未対応。 Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_sandbox | `tests/browser/tsl-primitives.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_shadow_contact | `tests/browser/tsl-primitives.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_lines_fat | `tests/browser/tsl-primitives.spec.js` | 公式r186のdash offset更新不備は修正して比較。 Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_lines_fat_wireframe | `tests/browser/tsl-primitives.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_sss | `tests/browser/tsl-materials.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_toon | `tests/browser/tsl-materials.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_skinning_instancing | `tests/browser/tsl-materials.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_oit | `tests/browser/tsl-materials.spec.js` | 最終合成とcanvas表示が別パス（公式より1パス多い）。 Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_envmaps_groundprojected | `tests/browser/tsl-materials.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_backdrop | `tests/browser/tsl-viewport.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_backdrop_area | `tests/browser/tsl-viewport.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_refraction | `tests/browser/tsl-viewport.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_particles_soft | `tests/browser/tsl-viewport.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_upscaling_fsr1 | `tests/browser/tsl-viewport.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_pmrem_cubemap | `tests/browser/tsl-lighting.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_pmrem_scene | `tests/browser/tsl-lighting.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_lightmap | `tests/browser/tsl-lighting.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_dof_basic | `tests/browser/tsl-lighting.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_lensflare | `tests/browser/tsl-lighting.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_anamorphic | `tests/browser/tsl-extended.spec.js` | GPUインスタンス変形、HDR高輝度抽出と横方向フィルタ、5段ブルーム。 |
| webgpu_tsl_earth | `tests/browser/tsl-extended.spec.js` | 昼夜テクスチャ、GPUバンプマップ、大気のフレネル表現。 |
| webgpu_occlusion | `tests/browser/tsl-extended.spec.js` | GPUオクルージョンクエリの非同期結果をTSLの色uniformに反映。 |
| webgpu_instance_uniform | `tests/browser/tsl-extended.spec.js` | 共有ノードプログラムとオブジェクトごとのuniform、ネイティブGPUキューブマップ。 |
| webgpu_postprocessing_dof | `tests/browser/tsl-extended.spec.js` | GPU上の近景・遠景CoC、64/16タップのボケと合成。CoCは同じ画素数・精度のRG16Fに格納。 |
| webgpu_multiple_elements | `tests/browser/tsl-extended.spec.js` | 単一GPUデバイス、40シーンのviewport/scissor描画。画面外は描画せず、部分表示はカメラのview offsetで切り取る。 |
| webgpu_multiple_canvas | `tests/browser/tsl-extended.spec.js` | 40個のWebGPU CanvasTargetが1個のGPUデバイスを共有。独立したOrbit操作。 |
| webgpu_struct_drawindirect | `tests/browser/tsl-extended.spec.js` | GPU構造体のatomicStoreで間接描画コマンドを更新。頂点変形・色計算もGPUで実行。 |
| webgpu_materials_cubemap_mipmaps | `tests/browser/tsl-extended.spec.js` | Rust TSLでキューブマップ反射。GPU生成mipmapと公式の手動mipmapを比較。 |
| webgpu_instance_path | `tests/browser/tsl-next.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_sobel | `tests/browser/tsl-next.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_smaa | `tests/browser/tsl-next.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_3dlut | `tests/browser/tsl-next.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_cubemap_mix | `tests/browser/tsl-environment.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_cubemap_adjustments | `tests/browser/tsl-environment.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_envmaps_bpcem | `tests/browser/tsl-environment.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_materials_alphahash | `tests/browser/tsl-environment.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_ca | `tests/browser/tsl-environment.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_parallax_uv | `tests/browser/tsl-next.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_rendertarget_2d-array_3d | `tests/browser/tsl-extended.spec.js` | 配列・3DテクスチャとレイヤーへのGPU描画を4ビューで比較。 |
| webgpu_instance_points | `tests/browser/tsl-extended.spec.js` | GPU Computeでサイズ更新、ピクセル単位のインスタンス点群と共有ターゲットの拡大ビュー。 |
| webgpu_texturegather | `tests/browser/tsl-extended.spec.js` | WebGPU側を全幅表示。色Gatherと深度比較GatherはGPU命令で実行。WebGL比較欄は対象外。 |
| webgpu_centroid_sampling | `tests/browser/tsl-extended.spec.js` | 左右のMSAA比較を1キャンバス上の独立したターゲットで表示。5種類のUV補間をGPUで実行。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_textures_2d-array_compressed | `tests/browser/tsl-extended.spec.js` | 圧縮ブロックを維持したGPU配列テクスチャ。BC/ETC2/ASTC非対応環境では明示的にエラー。性能の完全な同等性は未保証。 |
| webgpu_compute_texture_3d | `tests/browser/tsl-extended.spec.js` | 200³ボクセルをGPU Computeで更新しGPUレイマーチング。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_volume_perlin | `tests/browser/tsl-extended.spec.js` | 3DテクスチャをGPU上でレイマーチング。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_volume_cloud | `tests/browser/tsl-extended.spec.js` | 3DテクスチャをGPU上でレイマーチング。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_textures_2d-array | `tests/browser/tsl-extended.spec.js` | 109層をGPU配列テクスチャに保持してレイヤーをシェーダーで選択。性能の完全な同等性は未保証。 |
| webgpu_multisampled_renderbuffers | `tests/browser/tsl-extended.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_layers | `tests/browser/tsl-extended.spec.js` | Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_tsl_vfx_flames | `tests/browser/tsl-surface.spec.js` | 公式の2種類のGPU炎シェーダーと水平Billboardを移植。性能の完全な同等性は未保証。 |
| webgpu_custom_fog_background | `tests/browser/tsl-surface.spec.js` | MSAA深度からTSLの霧を計算し、HDRトーンマッピング後の色と合成。性能の完全な同等性は未保証。 |
| webgpu_lights_selective | `tests/browser/tsl-surface.spec.js` | 材質ごとのライト選択とTSL材質入力、公式Teapot形状を移植。性能の完全な同等性は未保証。 |
| webgpu_lights_phong | `tests/browser/tsl-surface.spec.js` | 材質ごとのライト選択とTSL材質入力、公式Teapot形状を移植。性能の完全な同等性は未保証。 |
| webgpu_mrt | `tests/browser/tsl-surface.spec.js` | HDR色と8bit法線・拡散色・発光色の混合形式MRTを移植。性能の完全な同等性は未保証。 |
| webgpu_postprocessing_bloom | `tests/browser/tsl-surface.spec.js` | 公式の5段階HDR BloomをGPUで移植。性能の完全な同等性は未保証。 |
| webgpu_postprocessing_bloom_emissive | `tests/browser/tsl-surface.spec.js` | 公式の5段階HDR BloomをGPUで移植。性能の完全な同等性は未保証。 |
| webgpu_postprocessing_bloom_selective | `tests/browser/tsl-surface.spec.js` | MRTマスクとGPU Bloom、クリックによる対象選択を移植。比較用に固定乱数を使用。性能の完全な同等性は未保証。 |
| webgpu_storage_buffer | `tests/browser/tsl-surface.spec.js` | WebGPU側のみ全幅表示。4種類のStorage BufferをGPU同期・反転。WebGL比較欄とタイムスタンプUIは対象外。 |
| webgpu_compute_geometry | `tests/browser/tsl-surface.spec.js` | 常駐位置と速度をGPU Computeで更新するJelly変形。性能の完全な同等性は未保証。 |
| webgpu_tsl_vfx_tornado | `tests/browser/tsl-surface.spec.js` | GPU頂点変形・ノイズ・HDR Bloomによる竜巻を移植。性能の完全な同等性は未保証。 |
| webgpu_mrt_mask | `tests/browser/tsl-surface.spec.js` | GPUスキニングと材質ごとのMRTマスク、2方向Gaussian Blurを移植。性能の完全な同等性は未保証。 |
| webgpu_shadertoy | `tests/browser/tsl-surface.spec.js` | 公式の2種類の固定シェーダーをRust TSL/WGSLで移植。任意GLSLを変換するTranspiler APIは未対応。 |
| webgpu_lights_custom | `tests/browser/tsl-surface.spec.js` | 50万点の常駐位置とTSLカスタム照明をGPUで計算。比較用に固定乱数を使用。性能の完全な同等性は未保証。 |
| webgpu_multiple_rendertargets | `tests/browser/tsl-surface.spec.js` | TSLによる色・法線のMRT出力を1回の描画で生成し左右に表示。性能の完全な同等性は未保証。 |
| webgpu_depth_texture | `tests/browser/tsl-surface.spec.js` | 50オブジェクトの深度アタッチメントをGPU上で直接サンプリング。比較用に固定乱数を使用。性能の完全な同等性は未保証。 |
| webgpu_skinning | `tests/browser/tsl-surface.spec.js` | 公式Michelleモデル・GPUスキニング・TSL背景・Linear tone mappingを移植。性能の完全な同等性は未保証。 |
| webgpu_tsl_halftone | `tests/browser/tsl-surface.spec.js` | 公式モデルと画面座標・法線に基づくTSLハーフトーンと調整を移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_tsl_raging_sea | `tests/browser/tsl-surface.spec.js` | GPUの波変形・MaterialXノイズ・法線・発光と15個の調整項目を移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_lights_rectarealight | `tests/browser/tsl-surface.spec.js` | TSL粗さノード・LTC面光源と回転・Orbit操作を移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_particles | `tests/browser/tsl-compute.spec.js` | 2000煙・1000炎のGPUビルボードと間接描画。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_instance_mesh | `tests/browser/tsl-compute.spec.js` | 1000 SuzanneインスタンスとTSLの色。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_compute_points | `tests/browser/tsl-compute.spec.js` | 30万点のGPUストレージ更新と点描画。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_compute_particles | `tests/browser/tsl-compute.spec.js` | 20万粒子のGPU物理更新・ポインター入力とAlpha to Coverage。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_compute_texture_pingpong | `tests/browser/tsl-compute.spec.js` | 512×512 HDRテクスチャのGPU交互更新。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_fog_height | `tests/browser/tsl-particles.spec.js` | TSL高さフォグと100インスタンス。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_sprites | `tests/browser/tsl-particles.spec.js` | 200スプライトのGPUビルボード・個別回転・フォグ。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_instance_sprites | `tests/browser/tsl-particles.spec.js` | 10000スプライトのGPU回転・インスタンス属性。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_tsl_galaxy | `tests/browser/tsl-particles.spec.js` | 20000粒子のGPU位置・色・拡縮。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_afterimage | `tests/browser/tsl-particles.spec.js` | 50000粒子のGPUアニメーションと履歴テクスチャ。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_direct | `tests/browser/tsl-filters.spec.js` | 描画シェーダー内のTSL saturationとNeutral tone mappingを移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_radial_blur | `tests/browser/tsl-filters.spec.js` | 100インスタンスとGPU可変サンプル数Radial Blurを移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_fxaa | `tests/browser/tsl-filters.spec.js` | 100インスタンスとsRGB空間の適応的FXAAを移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_ssaa | `tests/browser/tsl-filters.spec.js` | 120インスタンスと1〜32回のジッター付きGPU描画・加算を移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_transition | `tests/browser/tsl-filters.spec.js` | 2シーン・各500インスタンスと6種の遷移マスクを移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_compute_texture | `tests/browser/tsl-passes.spec.js` | Rust TSLのcompute・textureStoreでGPUテクスチャを生成。描画は変更時のみ。情報表示の外観と性能の完全な同等性は未保証。 |
| webgpu_rtt | `tests/browser/tsl-passes.spec.js` | TSLのrender-to-texture・saturation・hueとマウス操作を移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing | `tests/browser/tsl-passes.spec.js` | 100個のPhong球とDot Screen・RGB ShiftをTSLで移植。比較可能な固定乱数を使用。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_difference | `tests/browser/tsl-passes.spec.js` | 前フレームのGPUテクスチャを参照するTSL合成・速度調整・Orbit操作を移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_postprocessing_masking | `tests/browser/tsl-passes.spec.js` | 3シーンのGPU描画・アルファマスク・TSL画像合成を移植。Inspector外観と性能の完全な同等性は未保証。 |
| webgpu_tsl_interoperability | `tests/browser/tsl.spec.js` | RustのTSLノードAPIとWGSL関数連携でCRTを移植。Inspector UIの外観と性能の完全な同等性は未保証。 |
| webgpu_texturegrad | `tests/browser/tsl.spec.js` | 公式のWebGPU側を単独表示。WebGL比較パネルは対象外。RustのTSLによる勾配付きGPUテクスチャサンプリング。性能の完全な同等性は未保証。 |
| webgpu_procedural_texture | `tests/browser/tsl.spec.js` | RustのTSLからGPUテクスチャ生成・2段Gaussian blurと調整を移植。Inspector UIの外観と性能の完全な同等性は未保証。 |
| webgpu_loader_gltf_dispersion | `tests/browser/gltf-physical.spec.js` | 公式DispersionTest・色分散・HDR・Orbit操作を移植。情報表示の外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_loader_gltf_compressed | `tests/browser/gltf-physical.spec.js` | 公式coffeemat・Meshopt・KTX2/BasisのGPU圧縮テクスチャ・Orbit操作を移植。対応GPU圧縮形式が必要。情報表示と性能の完全な同等性は未保証。 |
| webgl_loader_gltf_instancing | `tests/browser/gltf-instancing.spec.js` | 試作版：公式モデルのGPUインスタンシングとOrbit操作を実装。公式WebGL版とのMSAA・金属反射の描画差と、初期化ごとの描画変動は調査中。外観・性能の同等性は未確認。 |
| webgpu_loader_gltf_anisotropy | `tests/browser/gltf-physical.spec.js` | 公式の異方性反射・クリアコート・透過材質とOrbit操作を移植。情報表示とInspector UIは未一致。 |
| webgpu_loader_gltf_sheen | `tests/browser/gltf-physical.spec.js` | 公式SheenChair・Sheen調整・減衰付きOrbit操作を移植。調整UIの外観と情報表示は未一致。 |
| webgpu_loader_gltf_transmission | `tests/browser/gltf-physical.spec.js` | 公式の透過・玉虫色材質・蓋のアニメーション・自動回転とOrbit操作を移植。情報表示とInspector UIは未一致。 |
| webgpu_loader_gltf_iridescence | `tests/browser/gltf-iridescence.spec.js` | 公式IridescenceLamp・HDR環境・自動回転・Orbit操作を移植。情報表示は未一致。一般の屈折・拡張材質と性能の完全互換は未保証。 |
| webgl_loader_gltf_avif | `tests/browser/gltf-avif.spec.js` | 公式Forest HouseのDraco形状とAVIFテクスチャを読み込み、Orbit・パン・ズームと変更時のみの描画を移植。MSAAの輪郭と情報オーバーレイの外観は未一致。性能同等性は未保証。 |
| webgl_buffergeometry | `tests/browser/expanded.spec.js` | 公式の16万三角形・頂点RGBA・Phong照明・霧・回転を移植。比較用の固定乱数を使用。Stats表示は未移植。汎用頂点形式によるGPUメモリ増加が残り、完全な性能同等性は未達。 |
| webgl_buffergeometry_rawshader | `tests/browser/expanded.spec.js` | 公式の200三角形・頂点RGBA・色アニメーションをWGSLへ移植。比較用の固定乱数を使用。Stats表示は未移植。汎用頂点形式のメモリ使用量は公式より大きい。 |
| webgl_geometry_colors | `tests/browser/expanded.spec.js` | 公式3個の頂点色Icosahedron・ワイヤー・影画像・ポインターカメラを移植。 |
| webgl_buffergeometry_indexed | `tests/browser/expanded.spec.js` | 公式のインデックス付き色グリッド・回転・ワイヤーフレーム切替を移植。GUI外観とStatsは未一致。 |
| webgl_lines_colors | `tests/browser/expanded.spec.js` | 公式の6色線・Hilbert/Catmull-Rom・回転・ポインターカメラを移植。 |
| webgl_morphtargets_horse | `tests/browser/expanded.spec.js` | 公式Horse・GPUモーフ・変形後の面法線・1秒ループ・周回カメラを移植。Stats表示は未移植。 |
| webgl_morphtargets_sphere | `tests/browser/expanded.spec.js` | 公式AnimatedMorphSphere・GPUモーフ・点スプライト・Orbit操作を移植。WebGPUでは点をGPUで展開した四角形として描画。 |
| webgpu_morphtargets | `tests/browser/expanded.spec.js` | 公式WebGPU版のGPUモーフシーンと操作を移植。Inspector全体のUIは未移植。 |
| webgl_lines_dashed | `tests/browser/expanded.spec.js` | 公式Hilbert曲線・Catmull-Rom補間・破線・霧・回転を移植。1ピクセル線はGPU線プリミティブ。Stats表示は未移植。 |
| webgl_geometries | `tests/browser/expanded.spec.js` | 公式16形状・Phong材質・カメラと物体の回転を移植。3時刻で画像比較、GPU保持と負荷を検証。Stats表示は未移植。 |
| webgl_animation_skinning_morph | `tests/browser/core-robot.spec.js` | 公式RobotExpressive・14クリップ・スキニング・モーフ・照明・床・FogをRustへ移植。クリップ切替とOrbit操作を追加。公式GUIの表情スライダー・一時動作の復帰・Statsは未移植。 |
| webgpu_pmrem_test | `tests/browser/gallery-scenes.spec.js` | 公式WebGPU版の33球の光量比較シーンを移植。InspectorとOrbit制限は未一致。 |
| webgpu_equirectangular | `tests/browser/gallery-scenes.spec.js` | 公式画像を直接サンプリングする背景、自動回転と明るさ調整を移植。Inspector・Orbitの操作感は未一致。 |
| webgl_materials_texture_rotation | `tests/browser/gallery-scenes.spec.js` | UVのoffset・repeat・rotation・centerとドラッグを移植。異方性フィルタリング対応。Orbit制限とGUI外観は未一致。 |
| webgl_buffergeometry_lines | `tests/browser/gallery-scenes.spec.js` | 10000頂点の線モーフをGPU頂点シェーダーで処理。回転・モーフ重みのみCPUで更新。乱数は固定seed、Statsは未移植。 |
| webgl_buffergeometry_lines_indexed | `tests/browser/gallery-scenes.spec.js` | 雪片の再帰生成・頂点色・回転を移植。乱数は比較可能な固定seed。Stats表示は未移植。 |
| webgl_interactive_lines | `tests/browser/gallery-scenes.spec.js` | 50個の線、カメラ回転、線レイキャストと交点マーカーを移植。頂点と変換を公式と比較。WebGLの線ラスタライズ・Statsは未一致。乱数は固定seed。 |
| webgl_interactive_raycasting_points | `tests/browser/gallery-scenes.spec.js` | 3種類の点群、回転カメラ、点レイキャスト、40個の縮小マーカーを移植。頂点と変換を公式と比較。WebGLの点ラスタライズ・Statsは未一致。 |
| webgpu_pmrem_equirectangular | `tests/browser/gallery.spec.js` | 30個の球・材質・HDR背景は固定視点で公式と画像比較済み。Orbitの慣性・パン・ズーム制限とInspectorは未再現。 |
| webgl_geometry_cube | `tests/browser/migration.spec.js` | 既存移植はMSAA・ミップマップなし。公式の全設定との一致は未達。 |
| webgpu_loader_gltf | `tests/browser/gltf-pbr.spec.js` | DamagedHelmet / BoomBoxはPBR比較済み。公式の全モデルカタログ、アニメーション、圧縮・拡張材質は未対応。 |
| webgpu_lights_pointlights | `tests/browser/point-lights.spec.js` | 変形はGPUのWGSL頂点シェーダーで実行。公式PhongをStandardへ変更しており、完全一致ではない。 |

Detailed per-example port prerequisites and actual importer results: [port attempts](gallery-port-attempts.md).

Before the WebGPU preference policy, 605 upstream pages were also executed through a diagnostic bridge, with eligible captured scenes sent to the native Rust renderer: [runtime outcomes and limitations](gallery-runtime-results.md). These sampled frames are not behavioral ports and are not added to the gallery.

## Priority

1. Programmable materials and reusable WGSL effect interfaces, GPU compute/storage, and postprocessing targets/history unlock the largest group of WebGPU scenes.
2. Animation/skinning/morph targets and compressed/additional asset loaders unlock animated models.
3. Shadows, physical material extensions and advanced lights close appearance gaps.
4. Instancing/batching and procedural geometry enable large scenes.
5. WebXR, physics, spatial audio and DOM/SVG renderers require their own integrations and validation environments.

Gallery thumbnails are the original upstream previews, not screenshots of completed Rust ports. Unported entries show a diagnostic instead of executing Three.js or pretending another scene reproduces the example.

# Three.js examples coverage

Pinned reference: `148ef33ecb6d2502ff796d4554abd1549c95d519`. 607 examples inspected.

32 excluded for explicit WebGL APIs or equivalent WebGPU examples; 575 retained. 298 partial Rust ports; 277 not yet ported. Only runnable ports appear in the gallery list. No complete-gallery reproduction claim.

All entries, source hashes, source-line evidence and exclusions: [`catalog.json`](../web/gallery/catalog.json).

See [example selection and comparison policy](example-policy.md).

See [performance acceptance and current audit](performance-parity.md). The counts below identify source usage, not proven independent blockers; do not add them together. TSL usage can be ported to Rust/WGSL without implementing TSL itself.

| Required capability (source inventory, not current support status) | Examples mentioning it |
| --- | ---: |
| Full camera controls: pan, touch, damping and control variants | 370 |
| Programmable materials / TSL equivalents | 219 |
| Inspector and per-example GUI parity | 175 |
| Phong, Lambert, normal, depth, toon and matcap materials | 173 |
| Additional procedural geometry builders | 171 |
| Shadow maps and shadow filtering | 126 |
| Hemisphere/spot/area lights, light probes and baking | 120 |
| Postprocessing passes and temporal history | 101 |
| Wireframe materials and scene helpers | 91 |
| Distance and height fog | 91 |
| Additional loaders and compressed assets | 81 |
| Instance transforms and batched drawing | 74 |
| Animation mixer and skeletal animation | 47 |
| Transmission, clearcoat, sheen, anisotropy and related PBR extensions | 45 |
| Canvas, HTML, video and partial texture updates | 42 |
| WebXR sessions, controllers and XR render targets | 32 |
| Configurable blend equations and factors | 30 |
| GPU compute and storage buffers | 30 |
| Volume rendering and layered textures | 18 |
| Physics integration | 14 |
| Curve interpolation and path builders | 13 |
| CSS2D/CSS3D/SVG scene renderers | 11 |
| Morph target animation | 9 |
| Wide / dashed line rendering | 8 |
| Clipping planes and stencil operations | 6 |
| Spatial audio and audio analysis | 6 |
| Stereo, anaglyph and parallax-barrier effects | 3 |

## Existing runnable ports

| Example | Evidence | Remaining differences |
| --- | --- | --- |
| webgl_raycaster_texture | `tests/browser/texture-flares.spec.js` | ブラウザのCanvas 2Dで描く格子画像と黄色の十字（GPUコピーとミップマップ再生成で3つのテクスチャに反映）、ポインタのレイキャストとtransformUv、立方体・平面・円のUV、円テクスチャのラップ・offset・repeat・rotationの操作をRustで再現。ラップの切り替えはテクスチャを共有するサンプラー別のプログラムで行い、原本の再アップロードは行わない。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgl_texture3d_partialupdate | `tests/browser/texture-flares.spec.js` | 128³のData3DTexture、1.5秒ごとにCPUのImprovedNoiseで生成する30³ブロックの部分書き込み（原本と同じCPU生成）、フレーム番号とgl_FragCoordによるジッター付きレイマーチ、キャンバスのグラデーション空、OrbitControls、全4項目の操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgl_materials_cubemap_render_to_mipmaps | `tests/browser/texture-flares.spec.js` | 半精度キューブレンダーターゲットのレベル0〜8への面ごとの描画（レベル別の色付け、ソースキューブの暗黙LODサンプリング）、頂点ごとの反射ベクトルによる2つの環境マップ球（CubeTextureのx反転）、極角制限付きOrbitControlsをRustで再現。性能の完全な同等性は未保証。 |
| webgpu_lensflares | `tests/browser/texture-flares.spec.js` | 3000個のPhongの箱、3つの点光源、LensflareMesh（16×16のフレームバッファ退避、マゼンタの深度テストによる遮蔽マップ、復元、頂点で9テクセルを読む加算要素、光源色のsRGB→線形の一度きりの変換）、フォグ、FlyControls（ポインタとキー）をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgl_materials_car | `tests/browser/texture-flares.spec.js` | Draco圧縮のferrari glTF、クリアコートの車体・金属の細部・透過ガラス、HDR環境、フォグ、ACES（露出0.85）、半透明のGridHelperの移動と車輪の回転、乗算合成のAO平面、OrbitControls、3色の入力をRustで再現。Stats外観は未一致。性能の完全な同等性は未保証。 |
| webgl_ubo_arrays | `tests/browser/draco-variants.spec.js` | 300個の点光源の位置と色（LightingDataブロック、毎フレームCPUで位置を更新して書き込み、原本と同じ）、ワールド位置と距離減衰の光ループ、平面と100個の球、パンなしのOrbitControls、countの操作をRustで再現。ブロックはWebGPUのストレージバッファとして束縛。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgl_loader_draco | `tests/browser/draco-variants.spec.js` | DRACOLoaderによるbunny.drcのデコード（位置・法線・色・UV、面インデックス）とcomputeVertexNormals、半球光とPCFの影付きスポットライト、フォグ、Date.nowで回るカメラをRustで再現。性能の完全な同等性は未保証。 |
| webgl_animation_keyframes | `tests/browser/draco-variants.spec.js` | Skyとその一度だけのPMREM（fromScene）による環境、Draco圧縮のLittlestTokyo glTFとキーフレームアニメーション、ACES、減衰付きOrbitControlsをRustで再現。Stats外観は未一致。性能の完全な同等性は未保証。 |
| webgl_loader_gltf_variants | `tests/browser/draco-variants.spec.js` | HDRの環境と背景、ACES、KHR_materials_variantsの靴（バリアントごとのマテリアル、元のマテリアルへの復帰）、変更時のみの描画、OrbitControls、Variantの操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_morphtargets_face | `tests/browser/draco-variants.spec.js` | RoomEnvironmentのPMREM（sigma 0.04）、ACES、meshoptとKTX2のfacecap glTF、52個のモーフターゲットのアニメーション、方位角と距離の制限付き減衰OrbitControlsをRustで再現。ミキサーが毎フレーム上書きするlisten表示のモーフスライダーとInspector外観は未一致。性能の完全な同等性は未保証。 |
| webgl_postprocessing_backgrounds | `tests/browser/passes-decals.spec.js` | EffectComposerのClearPass（色とアルファ）、TexturePass（木目テクスチャと不透明度）、CubeTexturePass（NoColorSpaceのpisaキューブ、10単位の裏面ボックス、カメラの回転と投影を共有）、RenderPass（3つの点光源とStandardの球）、OutputPass、ズームなしのOrbitControls、全8項目の操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgl_shader_lava | `tests/browser/passes-decals.spec.js` | 溶岩のGLSLシェーダー（雲と溶岩テクスチャ、uvScale、gl_FragCoord.z/wによるフォグ）を持つトーラス、BloomPass（25タップのガウス畳み込みを横と縦、加算合成）、OutputPass、delta×5の時間と回転をRustで再現。性能の完全な同等性は未保証。 |
| webgl_ubo | `tests/browser/passes-decals.spec.js` | ViewDataとLightingDataのユニフォームブロックを共有する2つのRawShaderMaterial（視点空間のPhong、sRGB出力）、200個の四面体と木箱、毎フレームの回転をRustで再現。ViewDataはフレーム共通のカメラユニフォーム、変化しないLightingDataは両プログラムの定数として実装。性能の完全な同等性は未保証。 |
| webgl_postprocessing_rgb_halftone | `tests/browser/passes-decals.spec.js` | 50個の法線とUVのシェーダー立方体、Phongの床と回転する点光源、HalftonePass（全5形状、チャンネルごとの角度、散乱、ブレンドモード、グレースケール、無効化）、OrbitControls、全10項目の操作をRustで再現。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgl_decals | `tests/browser/passes-decals.spec.js` | LeePerrySmithの頭部（Phong、カラー・スペキュラー・法線マップ）、ポインタのレイキャストと法線ライン、クリックで生成するDecalGeometry（6平面のクリッピング、ランダムな回転・大きさ・色、polygonOffset）、Clearボタン、OrbitControlsをRustで再現。デカールの生成は原本と同じくクリック時のCPU処理。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_lightprobe_cubecamera | `tests/browser/probes-hdr.spec.js` | CubeCameraで捉えたpisa背景からのLightProbeGenerator.fromCubeRenderTarget（8bit線形のキューブと同じ値からCPUでSH係数を一度だけ計算）、LightProbeHelper、背景キューブ、変更時のみの描画をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgl_materials_envmaps_hdr | `tests/browser/probes-hdr.spec.js` | Generated（DebugEnvironmentのPMREM）・LDR・HDRキューブの環境マップ切り替え、生キューブ背景とPMREM背景、デバッグ用のPMREMアトラス表示、粗さ・金属度・露出、ACESをRustで再現。Stats表示とlil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgl_loader_texture_ultrahdr | `tests/browser/probes-hdr.spec.js` | UltraHDRLoader（MPFとXMPの解析、ブラウザでのJPEGデコードとゲインマップ拡大、復元式とtoHalfFloat）、環境と背景、自動回転、解像度の切り替え（非同期再読み込み）をRustで再現。FloatType選択時も半精度で保持。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_materials_transmission | `tests/browser/probes-hdr.spec.js` | UltraHDRの環境と背景、透過（transmission）・IOR・厚み・スペキュラーを持つ両面の物理マテリアル、アルファマップの縞、全12項目の操作をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_performance | `tests/browser/probes-hdr.spec.js` | UltraHDRの環境、798メッシュのダンジョンglTF（WebP）、ACES、OrbitControls、static切り替えをRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgl_shadowmesh | `tests/browser/shadow-rtt.spec.js` | ShadowMesh（光源の同次座標による平面投影、ステンシルで二重描画を防止、前フレームのワールド行列を使用）、5つの物体の回転と移動、ArrowHelper、平行光源と点光源の切り替えボタンをRustで再現。性能の完全な同等性は未保証。 |
| webgl_instancing_dynamic | `tests/browser/shadow-rtt.spec.js` | 10,000個のInstancedMesh（毎フレームの行列更新とTWEENによる色の切り替え、原本と同じCPU更新）、RoomEnvironmentの環境、Neutralトーンマッピング、カメラの軌道とup変化をRustで再現。性能の完全な同等性は未保証。 |
| webgl_depth_texture | `tests/browser/shadow-rtt.spec.js` | DepthTexture付きのレンダーターゲット、深度の線形化ポストパス、50個のトーラスノットのInstancedMesh、減衰付きOrbitControls（描画後の更新）をRustで再現。深度の形式と型は表示に影響しない。WebGPUのMSAAは4サンプルのみのため、0以外のサンプル数は4として扱う。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_rtt | `tests/browser/shadow-rtt.spec.js` | レンダーターゲットへの描画（UVとtimeのシェーダー、2つのPhongトーラス）、画面クアッドと25個の球へのテクスチャ、autoClearなしの2段描画、マウス追従カメラをRustで再現。原本はリサイズに対応しないため、リサイズ後の表示は未比較。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_materials_normalmap | `tests/browser/shadow-rtt.spec.js` | LeePerrySmithの頭部（Phong、カラー・スペキュラー・法線マップ）、EffectComposer（BleachBypass、ColorCorrection、OutputPass、FXAA）、減衰付きOrbitControls、法線マップの切り替えと強度をRustで再現。devicePixelRatioを使わない原本と同じく1倍で描画。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| misc_exporter_exr | `tests/browser/exporters-video.spec.js` | EXRExporter（ZIP・ZIPS・無圧縮、Half/Float）、PMREMの背景とデータテクスチャ、減衰付きOrbitControls（rotateSpeed −0.25）をRustで再現。PMREMの書き出しはGPUからの読み戻し。zlib圧縮後のバイト列は実装の違いで異なり、展開後の画素で比較。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| misc_exporter_ktx2 | `tests/browser/exporters-video.spec.js` | KTX2Exporter（ktx-parseのwrite、無圧縮のRGBA Float/Half）、PMREMの背景（AgX）とデータテクスチャ、減衰付きOrbitControlsをRustで再現。PMREMの書き出しはGPUからの読み戻し。lil-gui外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_materials_video | `tests/browser/exporters-video.spec.js` | VideoTexture（新しいフレームのみ転送）を貼った200個のPhongキューブ（UVで分割）、色相の時間変化、1000フレーム周期の移動と反転、マウス追従カメラをRustで再現。性能の完全な同等性は未保証。 |
| webgpu_compile_async | `tests/browser/exporters-video.spec.js` | MaterialXノイズ（Perlin・Worley・Cell・Fractal）とhashによる256種の固有マテリアルを表示前にビルドし、1秒後に追加。法線マテリアルの球の往復、リサイズ時の12単位のフラスタムをRustで再現。モード切替ボタン（ページ再読み込み）と計測表示は未移植。性能の完全な同等性は未保証。 |
| webgl_materials_normalmap_object_space | `tests/browser/exporters-video.spec.js` | オブジェクト空間法線マップ（法線属性を削除、両面、裏面で反転）、カメラに付けた点光源、Nefertitiの縮小と中心合わせ、変更時のみの描画をRustで再現。devicePixelRatioを使わない原本と同じく1倍で描画。性能の完全な同等性は未保証。 |
| webgl_materials_cubemap | `tests/browser/shapes-lights.spec.js` | OBJLoaderの頭部とLambertのenvmap（反射・屈折、Multiply・Mix合成）、背景キューブ、極角制限付きOrbitControlsをRustで再現。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_loader_stl | `tests/browser/shapes-lights.spec.js` | STLLoader（ASCII・バイナリ・COLOR=ヘッダの頂点色）、フォグ、半球光と2灯の影をRustで再現。影のPCFの回転ノイズはWebGLと画面座標の上下が逆のため縁が異なる。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_geometry_extrude_shapes | `tests/browser/shapes-lights.spec.js` | ExtrudeGeometry（パス押し出し・ベベル）、Earcut（穴なし単純多角形）、CatmullRomCurve3の等弧長点とFrenetフレームをRustで再現し、原本の頂点と一致を確認。TrackballControlsは60fps相当の時間ステップ。UV属性は未生成（材質が不使用）。性能の完全な同等性は未保証。 |
| webgl_lights_spotlights | `tests/browser/shapes-lights.spec.js` | TWEEN（Quadratic.Out）による3灯のスポットライトの角度・半影・位置の補間、5秒ごとの再設定、スポットライトの影とSpotLightHelperをRustで再現。性能の完全な同等性は未保証。 |
| webgpu_clearcoat | `tests/browser/lights-probes.spec.js` | クリアコート付きMeshPhysicalMaterial（法線マップ、クリアコート法線マップ、FlakesTextureのキャンバス描画）、HDRキューブ環境、点光源をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| misc_controls_fly | `tests/browser/lights-probes.spec.js` | FlyControls（ポインタ位置によるヨー・ピッチ、キー移動・ロール）、法線・スペキュラーマップ付きの地球と雲・月、星のPoints（r186のWebGPUと同じネイティブ1ピクセル点）、FogExp2、FilmNodeのグレインをRustで再現。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgpu_shadowmap_pointlight | `tests/browser/lights-probes.spec.js` | 2つの点光源のキューブ影（半径10のPCF、bias）、alphaMap＋alphaTestで穴の空いた球（影にも反映）、BackSideの部屋をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_lightprobe | `tests/browser/lights-probes.spec.js` | LightProbeGenerator.fromCubeTexture（CPUでSH係数を計算）、LightProbeの照度、キューブ環境マップの球、LightProbeHelperをRustで再現。r186はライト強度の変更をメッシュの次回リフレッシュまで反映しないため、比較では参照側を強制リフレッシュ（移植版は即時反映）。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_tonemapping | `tests/browser/lights-probes.spec.js` | 7種のトーンマッピング（None、Linear、Reinhard、Cineon、ACESFilmic、AgX、Neutral）、露出、背景のぼかしと強度、Dracoのglb、ダンピング付きOrbitControlsをRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_sky | `tests/browser/sky-water.spec.js` | SkyMesh（Preetham大気散乱とr186の雲層）とCubeCameraによる毎フレームの6面キャプチャ、反射球、ACESトーンマッピングをRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_lights_sunlight | `tests/browser/sky-water.spec.js` | SunLight（2カスケード影）、SkyMeshから生成するPMREM環境、フォグ色と太陽色の補間、InstancedMeshの柱と塔、FirstPersonControlsをRustで再現。カスケード色分け表示（show cascades）は未移植。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| misc_controls_pointerlock | `tests/browser/sky-water.spec.js` | PointerLockControls（視点回転・前後左右移動）、重力とジャンプ、下向きレイによる箱への着地、ジッター付き床と500個の箱をRustで再現。ポインタロックはギャラリーのクリックで取得。性能の完全な同等性は未保証。 |
| webgpu_video_panorama | `tests/browser/sky-water.spec.js` | VideoTexture（新しい動画フレームごとにcopyExternalImageToTextureで転送、flipY）と内向き球、ドラッグによる経度・緯度の視点操作をRustで再現。性能の完全な同等性は未保証。 |
| webgpu_ocean | `tests/browser/sky-water.spec.js` | WaterMesh（法線テクスチャのノイズ、半解像度のReflector、フレネル）、SkyMesh（雲）、空から生成するPMREM環境、ブルーム後処理をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| misc_exporter_stl | `tests/browser/exporters-matcap.spec.js` | STLExporter（ASCII・バイナリ）をRustで再現し、書き出したファイルが原本とバイト単位で一致することを確認。シーン（影・フォグ・グリッド）も再現。性能の完全な同等性は未保証。 |
| misc_exporter_ply | `tests/browser/exporters-matcap.spec.js` | PLYExporter（ASCII・バイナリBE/LE、法線・UV・Uint8頂点色）をRustで再現し、書き出したファイルが原本とバイト単位で一致することを確認。性能の完全な同等性は未保証。 |
| misc_exporter_obj | `tests/browser/exporters-matcap.spec.js` | OBJExporter（メッシュ・点群、変換済み複数オブジェクト）と6種のジオメトリ切替をRustで再現し、書き出したファイルが原本とバイト単位で一致することを確認。性能の完全な同等性は未保証。 |
| webgpu_materials_matcap | `tests/browser/exporters-matcap.spec.js` | EXRLoader（ZIP圧縮・FLOATチャネルのHalf変換）、matcapUVによるEXRマットキャップ、法線マップ、ACESトーンマッピングをRustで再現。ドラッグ＆ドロップによるマットキャップ差し替えは未移植。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_lights_physical | `tests/browser/exporters-matcap.spec.js` | 光束（lm）・照度（lx）による点光源と半球光、点光源の影、バンプ・ラフネス・メタルネスマップ付きStandard材質、Reinhardトーンマッピングと露出をRustで再現。Inspector外観は未一致。性能の完全な同等性は未保証。 |
| misc_animation_groups | `tests/browser/selection-views.spec.js` | AnimationObjectGroupで25個の箱が共有するクォータニオン・離散カラー・不透明度のキーフレームをRustで再現（3秒ループ）。Stats表示は未移植。性能の完全な同等性は未保証。 |
| misc_animation_keys | `tests/browser/selection-views.spec.js` | 位置・スケール・クォータニオン・離散カラー・不透明度のキーフレームトラックとAxesHelperをRustで再現。Stats表示は未移植。性能の完全な同等性は未保証。 |
| misc_controls_drag | `tests/browser/selection-views.spec.js` | DragControls（左ドラッグで移動、右ドラッグで回転）、Shift+クリックのグループ選択とemissive表示、オンデマンド描画をRustで再現。ホバー時のカーソル変更とタッチ操作（Mキー）は未移植。性能の完全な同等性は未保証。 |
| misc_boxselection | `tests/browser/selection-views.spec.js` | SelectionBoxの視錐台選択（NDC始点・終点、emissive表示）をRustで再現し、SelectionHelperの矩形はDOMで表示。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgpu_camera_array | `tests/browser/selection-views.spec.js` | ArrayCameraの6×6サブカメラを上端基準ビューポートで描画し、影マップは1回だけ描画（shadow autoUpdate相当）。性能の完全な同等性は未保証。 |
| webgl_clipping_advanced | `tests/browser/text-clipping.spec.js` | 四面体のローカルクリッピング平面（毎フレーム変換）、回転する円筒状のグローバル平面、clipShadows付きInstancedMeshとスポット光・平行光源の影、平面の可視化をRustで再現。GUIはチェックボックスで表現（Visualizeのlisten表示更新は未移植）。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_geometry_extrude_splines | `tests/browser/text-clipping.spec.js` | CurveExtrasの14曲線とCatmullRom曲線、TubeGeometry（Frenetフレーム）、ワイヤーフレーム、スプラインカメラ・CameraHelper・lookAheadをRustで再現。パラメータ変更時のチューブ再生成は原本と同じくCPUで行う。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_geometry_text | `tests/browser/text-clipping.spec.js` | FontLoaderの書体JSON、ShapePath.toShapes、穴付きEarcut、ベベル付きExtrudeGeometry（TextGeometry）をRustで再現し、原本の頂点と一致を確認。10種の書体は起動時に一括取得（原本は選択時に取得）。キー入力（keydown/keypress）とドラッグ回転、4つのボタンを移植。性能の完全な同等性は未保証。 |
| webgl_modifier_tessellation | `tests/browser/text-clipping.spec.js` | TextGeometryのcenter()、TessellateModifier、面ごとの乱数色と変位、生のShaderMaterial出力をRustで再現。変位は頂点属性として常駐。TrackballControlsは60fps相当の時間ステップ。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_custom_attributes_lines | `tests/browser/text-clipping.spec.js` | TextGeometryの頂点列をLINE_STRIPで描画し、加算合成・深度テストなしの生ShaderMaterialを再現。変位属性の乱歩は原本と同じく毎フレームCPUで更新し全量を転送。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_lights_hemisphere | `tests/browser/shapes-lights.spec.js` | フラミンゴのモーフアニメーション、半球光と平行光源の影、各ヘルパー、空のグラデーションシェーダー、フォグをRustで再現。GUIのトグルはチェックボックスで表現。影の強さ（shadowIntensity）は未対応。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_materials_cubemap_refraction | `tests/browser/refraction-loaders.spec.js` | PLYLoader（バイナリ）とcomputeVertexNormals、Phongにenvmap_fragmentの屈折（CubeRefractionMapping・MultiplyOperation）と背景キューブを加えてRustで再現。マウス追従のカメラは60fps相当の時間ステップ。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_loader_ply | `tests/browser/refraction-loaders.spec.js` | PLYLoader（ASCII・バイナリ）、フラットシェーディング、半球光と2灯の平行光源の影、線形フォグをRustで再現。影のPCFはWebGLと同じVogel円盤とIGNだが、画面座標の上下が逆のため影の縁の回転が異なる。性能の完全な同等性は未保証。 |
| webgl_loader_kmz | `tests/browser/refraction-loaders.spec.js` | KMZLoaderのzip展開とdoc.kmlのモデル参照、ColladaLoaderの静的メッシュ・Phong材質・Z-up回転をRustで再現。変更時のみ描画。性能の完全な同等性は未保証。 |
| webgl_loader_collada | `tests/browser/refraction-loaders.spec.js` | ColladaLoaderの静的メッシュ（polylist・材質グループ・テクスチャ・ノード行列・Z-up）をRustで再現。スキン・アニメーション・キネマティクスは未対応（このアセットは不使用）。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_loader_texture_exr | `tests/browser/refraction-loaders.spec.js` | EXRLoaderのPIZ圧縮（ハフマン・ウェーブレット・LUT）をRustで再現し、原本の出力とバイト単位で一致を確認。Reinhardトーンマッピングとexposure操作。変更時のみ描画。性能の完全な同等性は未保証。 |
| webgl_loader_pdb | `tests/browser/helpers-formats.spec.js` | PDBLoaderの解析、原子・結合ごとのメッシュ、CSS2DRendererのラベル配置と重なり順をRust/DOMで再現。分子ごとのメッシュは一度だけ構築して常駐（原本は切替ごとに再構築）。ラベル層の書体は原本のmain.cssと同じ指定。性能の完全な同等性は未保証。 |
| webgl_helpers | `tests/browser/helpers-formats.spec.js` | 頂点法線・接線ヘルパー、BoxHelper、Wireframe/EdgesGeometry、PolarGrid、PointLightHelperをRustで再現。静止メッシュのヘルパー頂点は一度だけ計算して常駐（原本は毎フレーム同じ値を再計算・再送信）。性能の完全な同等性は未保証。 |
| webgl_modifier_simplifier | `tests/browser/helpers-formats.spec.js` | SimplifyModifierをmeshoptimizer 1.1のRust移植（optimesh）で再現し、原本のwasm版と同一の結果を確認。比率ごとの結果は一度だけ構築して常駐。変更時のみ描画。性能の完全な同等性は未保証。 |
| webgl_loader_amf | `tests/browser/helpers-formats.spec.js` | AMFLoaderのzip展開・XML解析・材質と色の規則をRustで再現。Z-upのOrbitControls。変更時のみ描画。性能の完全な同等性は未保証。 |
| webgl_loader_texture_tiff | `tests/browser/helpers-formats.spec.js` | TIFFLoader（UTIF）の無圧縮・LZW・JPEG（pdf.jsのベースラインデコーダ）をRustで再現し、原本とバイト単位で一致を確認。変更時のみ描画。性能の完全な同等性は未保証。 |
| webgl_loader_bvh | `tests/browser/picking-buffers.spec.js` | BVHLoaderの階層・モーション解析とAnimationMixerの線形補間／slerpFlat・ループをRustで再現。SkeletonHelperの頂点位置は原本同様に毎フレームCPUで書き込み。性能の完全な同等性は未保証。 |
| webgl_framebuffer_texture | `tests/browser/picking-buffers.spec.js` | Gosper曲線の毎フレーム色更新（原本同様CPU→GPU）とcopyFramebufferToTexture相当のテクスチャコピー、スプライトHUDをRustで再現。選択枠はDOMで表示。性能の完全な同等性は未保証。 |
| webgl_read_float_buffer | `tests/browser/picking-buffers.spec.js` | Float32レンダーターゲットへの描画と画面表示、マウス位置の値読み出し（非同期コピーで次フレーム以降に表示）をRustで再現。原本はリサイズ非対応のため、初期サイズを保持。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_interactive_cubes_gpu | `tests/browser/picking-buffers.spec.js` | 5000個の結合ボックスとGPUピッキング（1×1のビューオフセット描画と非同期読み出し）、TrackballControlsをRustで再現。IDは整数ターゲットの代わりにFloat32ターゲットへ書き込み（5000以下で厳密）。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_instancing_performance | `tests/browser/picking-buffers.spec.js` | INSTANCED・MERGED・NAIVEの3方式とcountをRustで再現し、原本同様に変更ごとに再構築。autoRotateは60fps相当の時間ステップ。GPUメモリ表示とStatsは未移植。性能の完全な同等性は未保証。 |
| webgl_loader_mdd | `tests/browser/models-modifiers.spec.js` | MDDLoaderのモーフターゲットとAnimationMixerの線形補間・ループをRustで再現し、GPUでモーフ合成。性能の完全な同等性は未保証。 |
| webgl_modifier_edgesplit | `tests/browser/models-modifiers.spec.js` | OBJLoader・mergeVertices・EdgeSplitModifierをRustで再現。パラメータの組ごとのジオメトリは一度だけ構築して常駐（原本は変更ごとに再構築）。変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_loader_3ds | `tests/browser/models-modifiers.spec.js` | TDSLoaderのチャンク解析とPhongマテリアル、法線マップ、TrackballControlsをRustで再現。更新は60fps相当の時間ステップ。性能の完全な同等性は未保証。 |
| webgl_geometry_teapot | `tests/browser/models-modifiers.spec.js` | TeapotGeometryのベジェ面分割と6種のシェーディング（反射はPhong結果に環境キューブを乗算）をRustで再現。組ごとのジオメトリは常駐。変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_instancing_scatter | `tests/browser/models-modifiers.spec.js` | MeshSurfaceSamplerと毎フレームのCPUインスタンス行列更新（原本と同じ）を60fps相当の時間基準で再現。resampleボタンは未移植。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_geometry_terrain | `tests/browser/terrain-loaders.spec.js` | ImprovedNoise地形と原本の正弦乱数、Canvas2Dでの陰影テクスチャ拡大をRustで再現。FirstPersonControlsを再現（長さ0のフレームでは更新しない）。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_geometry_terrain_raycast | `tests/browser/terrain-loaders.spec.js` | 地形とテクスチャ生成をRustで再現。ポインタ移動ごとのCPUレイキャスト（原本と同じ）で円錐を面法線へ向ける。OrbitControlsのキーボード操作は未移植。性能の完全な同等性は未保証。 |
| webgl_loader_gcode | `tests/browser/terrain-loaders.spec.js` | GCodeLoaderの解析をRustで再現し、各ファイルの線分は一度だけ構築して常駐（原本は切替ごとに再解析）。変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_loader_vox | `tests/browser/terrain-loaders.spec.js` | VOXLoaderのチャンク解析と貪欲メッシュ化、パレット色をRustで再現。性能の完全な同等性は未保証。 |
| webgl_loader_obj | `tests/browser/terrain-loaders.spec.js` | OBJLoader（オブジェクト・usemtlグループ）とMTLLoader（Kd・Ks・Ns・map_Kd）をRustで再現。同じ画像は1回だけ読み込み共有。性能の完全な同等性は未保証。 |
| webgl_loader_texture_hdr | `tests/browser/trackball-sprites.spec.js` | HDRLoaderのRGBEデコードと半精度変換（切り捨て）をRustで再現。Reinhardトーンマッピングと露出を再現し、変更時のみ描画。操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_geometry_minecraft | `tests/browser/trackball-sprites.spec.js` | ImprovedNoiseの地形と面の結合をRustで再現し、1つの常駐ジオメトリで描画（原本と同じ）。FirstPersonControlsを再現（長さ0のフレームでは更新しない）。Stats表示は未移植。性能の完全な同等性は未保証。 |
| misc_controls_trackball | `tests/browser/trackball-sprites.spec.js` | TrackballControls（回転・ズーム・パンの減衰、A/S/Dキー、正射影カメラ切替）をRustで再現。更新は60fps相当の時間ステップ。タッチ操作とmultiTouchRollは未検証。Stats表示は未移植。性能の完全な同等性は未保証。 |
| webgl_sprites | `tests/browser/trackball-sprites.spec.js` | SpriteMaterialの頂点処理（回転・中心・サイズ）とsRGB変換後のフォグをシェーダーで再現。フレーム毎の回転を60fps相当の時間基準で再現。性能の完全な同等性は未保証。 |
| webgl_lod | `tests/browser/trackball-sprites.spec.js` | LODの距離選択とFlyControls（マウス位置での旋回・ボタン前後移動・キー操作）をRustで再現。性能の完全な同等性は未保証。 |
| misc_controls_orbit | `tests/browser/controls-attributes.spec.js` | OrbitControls（減衰・極角制限・地面平面パン）をRustで再現。キーボード操作とカーソル形状は未移植。性能の完全な同等性は未保証。 |
| misc_controls_map | `tests/browser/controls-attributes.spec.js` | MapControls（左ドラッグのパン・右ドラッグの回転・zoomToCursor・screenSpacePanning）をRustで再現。タッチ操作は未検証。性能の完全な同等性は未保証。 |
| webgl_camera | `tests/browser/controls-attributes.spec.js` | Stats表示は未移植。CameraHelperの頂点はGPUで逆射影（原本はCPUで毎フレーム書き換え）。2つのビューポートとO/Pキー切替を再現。性能の完全な同等性は未保証。 |
| webgl_custom_attributes | `tests/browser/controls-attributes.spec.js` | Stats表示は未移植。毎フレームのCPUノイズ・HSL変化を60fps相当の時間基準で再現し、変位属性のみ常駐GPUバッファへ書き込み（原本と同じ）。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_drawrange | `tests/browser/controls-attributes.spec.js` | Stats表示は未移植。毎フレームの粒子移動と接続線の探索は原本と同じCPU処理で、60fps相当の時間基準で再現。更新分のみ常駐GPUバッファへ書き込み。性能の完全な同等性は未保証。 |
| webgl_effects_stereo | `tests/browser/stereo-loaders.spec.js` | StereoCamera・StereoEffectをRustで再現。フレーム毎のカメラ追従を60fps相当の時間基準で再現。性能の完全な同等性は未保証。 |
| webgl_effects_anaglyph | `tests/browser/stereo-loaders.spec.js` | AnaglyphEffect（frameCorners・色行列合成）をRustで再現。フレーム毎のカメラ追従を60fps相当の時間基準で再現。8bit線形の目用ターゲットを色行列で増幅する差を別途上限で検証。性能の完全な同等性は未保証。 |
| webgl_effects_parallaxbarrier | `tests/browser/stereo-loaders.spec.js` | ParallaxBarrierEffectをRustで再現。フレーム毎のカメラ追従を60fps相当の時間基準で再現。8bit線形の目用ターゲット由来の差を別途上限で検証。性能の完全な同等性は未保証。 |
| webgl_loader_pcd | `tests/browser/stereo-loaders.spec.js` | PCDLoader（ascii・binary・LZF圧縮）をRustで再現。読み込んだ点群はGPUに常駐させ再選択時に再利用。WebGLの点ラスタライズ差は厳密な被覆計算との比較で別途検証。操作UI外観は未一致。OrbitControlsのキーボード操作は未移植。性能の完全な同等性は未保証。 |
| webgl_loader_imagebitmap | `tests/browser/stereo-loaders.spec.js` | 6個の立方体は1つのテクスチャを共有（原本は同じ画像を6回読み込み）。setTimeoutを例のクロックで再現。性能の完全な同等性は未保証。 |
| webgl_multiple_views | `tests/browser/views-loaders.spec.js` | Stats表示は未移植。フレーム毎のカメラ移動を60fps相当の時間基準で再現。シザー付きクリアを各ビューの全画面背景三角形で再現。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_math_obb | `tests/browser/views-loaders.spec.js` | Stats表示は未移植。OBB衝突判定とレイ判定をRustへ移植。OrbitControlsのキーボード操作は未移植。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_custom_attributes_points2 | `tests/browser/views-loaders.spec.js` | Stats表示は未移植。毎フレームのCPU深度ソート結果とサイズのみ常駐GPUバッファへ書き込み（原本と同じ）。精度依存の1状態は原本の1 ULP感度で比較。性能の完全な同等性は未保証。 |
| webgl_loader_xyz | `tests/browser/views-loaders.spec.js` | XYZローダーをRustで再現。性能の完全な同等性は未保証。 |
| webgl_loader_texture_tga | `tests/browser/views-loaders.spec.js` | TGALoaderのデコードをRustへ移植。OrbitControlsのキーボード操作は未移植。性能の完全な同等性は未保証。 |
| webgl_interactive_voxelpainter | `tests/browser/interactive-scenes.spec.js` | Shiftキーはkeydown/keyupで追跡。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_interactive_buffergeometry | `tests/browser/interactive-scenes.spec.js` | Stats表示は未移植。ヒット時のみ4頂点の線ジオメトリを更新（原本と同じ）。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_clipping_intersection | `tests/browser/interactive-scenes.spec.js` | 操作UI外観は未一致。OrbitControlsの回転・ズームをRustで再現（キーボード操作は未移植）。WebGLとのMSAA/alpha to coverage差を別途検証。性能の完全な同等性は未保証。 |
| webgl_multiple_scenes_comparison | `tests/browser/interactive-scenes.spec.js` | シザー付きクリアを右シーンの全画面背景三角形で再現。WebGLとのMSAA差を別途検証。OrbitControlsの回転・パン・ズームをRustで再現（キーボード操作は未移植）。性能の完全な同等性は未保証。 |
| webgl_materials_blending_custom | `tests/browser/interactive-scenes.spec.js` | 操作UI外観は未一致。min/maxのブレンド係数はWebGPU仕様によりOne（GLでは無視される）。性能の完全な同等性は未保証。 |
| webgl_instancing_raycast | `tests/browser/interactive-objects.spec.js` | Stats・操作UI外観は未一致。OrbitControlsのドラッグ回転・減衰をRustで再現（キーボード操作は未移植）。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_math_orientation_transform | `tests/browser/interactive-objects.spec.js` | 操作UI外観は未一致。WebGLとのワイヤーフレームMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_panorama_cube | `tests/browser/interactive-objects.spec.js` | OrbitControlsのドラッグ回転・減衰をRustで再現（キーボード操作は未移植）。性能の完全な同等性は未保証。 |
| webgl_materials_texture_canvas | `tests/browser/interactive-objects.spec.js` | フレーム毎の回転を60fps相当の時間基準で再現。性能の完全な同等性は未保証。 |
| webgl_raycaster_sprite | `tests/browser/interactive-objects.spec.js` | OrbitControlsの回転・パン・ホイールズームをRustで再現（キーボード・タッチ操作の完全一致は未検証）。性能の完全な同等性は未保証。 |
| webgl_shader | `tests/browser/interactive-shaders.spec.js` | GLSLフラグメントをWGSLへ逐語移植。性能の完全な同等性は未保証。 |
| webgl_postprocessing_procedural | `tests/browser/interactive-shaders.spec.js` | Stats・操作UI外観は未一致。非2冪の高さへのリサイズ時はハッシュ雑音を1 ULP実験と分布で比較。性能の完全な同等性は未保証。 |
| webgl_interactive_cubes | `tests/browser/interactive-shaders.spec.js` | Stats表示は未移植。フレーム毎0.1°のカメラ回転を60fps相当の時間基準で再現。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_interactive_cubes_ortho | `tests/browser/interactive-shaders.spec.js` | Stats表示は未移植。フレーム毎0.1°のカメラ回転を60fps相当の時間基準で再現。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgl_interactive_points | `tests/browser/interactive-shaders.spec.js` | Stats表示は未移植。点はGPUインスタンシングで展開し、レイキャストはCPU上の常駐位置で実行。フレーム毎の回転を60fps相当の時間基準で再現。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_attributes_none | `tests/browser/shader-geometry.spec.js` | Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_attributes_integer | `tests/browser/shader-geometry.spec.js` | Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_instancing | `tests/browser/shader-geometry.spec.js` | Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_instancing_interleaved | `tests/browser/shader-geometry.spec.js` | 5000個の回転は共通クォータニオンと常駐行列を使いGPUで計算。汎用InterleavedBuffer APIの移植ではありません。Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。 |
| webgl_materials_modified | `tests/browser/shader-geometry.spec.js` | Stats・操作UI外観は未一致。静的属性はGPUに保持。性能の完全な同等性は未保証。 |
| webgl_materials_wireframe | `tests/browser/geometry-materials.spec.js` | Stats・操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_materials_texture_filters | `tests/browser/geometry-materials.spec.js` | Stats・操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_geometry_shapes | `tests/browser/geometry-materials.spec.js` | 固定Shape・Extrudeジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。Stats・操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_geometry_colors_lookuptable | `tests/browser/geometry-materials.spec.js` | 圧力モデルとLUTは事前生成。色選択はGPUで実行。Stats・操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_uint | `tests/browser/geometry-materials.spec.js` | Stats・操作UI外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_materials_basic | `tests/browser/environment-materials.spec.js` | Stats・Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_materials_envmaps | `tests/browser/environment-materials.spec.js` | Stats・Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgpu_materials_displacementmap | `tests/browser/environment-materials.spec.js` | 固定OBJ形状は公式から事前生成。実行時のOBJLoader移植ではありません。Stats・Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgl_materials_bumpmap | `tests/browser/environment-materials.spec.js` | WebGLとのバンプ微分・フィルタリングの描画差を別途検証。Stats・Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgl_materials_blending | `tests/browser/environment-materials.spec.js` | Stats・Inspector外観は未一致。性能の完全な同等性は未保証。 |
| webgl_points_billboards | `tests/browser/point-clouds.spec.js` | Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。 |
| webgl_points_sprites | `tests/browser/point-clouds.spec.js` | WebGL点プリミティブとのラスタライズ差を別途検証。Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。 |
| webgl_points_waves | `tests/browser/point-clouds.spec.js` | Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。 |
| webgl_custom_attributes_points | `tests/browser/point-clouds.spec.js` | Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。 |
| webgl_custom_attributes_points3 | `tests/browser/point-clouds.spec.js` | 固定BoxLineGeometryは公式から事前生成。Stats・操作UI外観は未一致。点の変形・サイズ更新はGPUで実行。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_points | `tests/browser/buffer-particles.spec.js` | Stats表示は未移植。点はGPUインスタンシングで展開。WebGL点ラスタライズ差を別途検証。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_points_interleaved | `tests/browser/buffer-particles.spec.js` | Stats表示は未移植。圧縮色の16-byte GPUレコードを使用。汎用InterleavedBuffer APIの移植ではありません。WebGL点ラスタライズ差と性能の完全な同等性は未保証。 |
| webgl_buffergeometry_custom_attributes_particles | `tests/browser/buffer-particles.spec.js` | Stats表示は未移植。点サイズのアニメーションをGPUへ移植。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_instancing_billboards | `tests/browser/buffer-particles.spec.js` | Stats表示は未移植。75,000個の六角形をGPUインスタンシングで描画。性能の完全な同等性は未保証。 |
| webgl_buffergeometry_selective_draw | `tests/browser/buffer-particles.spec.js` | Stats・操作UI外観は未一致。WebGLとのMSAA差を別途検証。性能の完全な同等性は未保証。 |
| webgpu_furnace_test | `tests/browser/shapes.spec.js` | 性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。 |
| webgl_geometry_convex | `tests/browser/shapes.spec.js` | 性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。 |
| webgl_geometry_nurbs | `tests/browser/shapes.spec.js` | 性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。 |
| webgl_geometry_text_shapes | `tests/browser/shapes.spec.js` | 性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。 |
| webgl_geometry_text_stroke | `tests/browser/shapes.spec.js` | 性能の完全な同等性は未保証。固定ジオメトリは公式から事前生成。関連Addon APIの完全移植ではありません。 |
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

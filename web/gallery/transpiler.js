// webgpu_tsl_transpiler: the page's two Monaco editors and dropdowns, as the
// original builds them; the transpiling itself runs in Rust
// ( src/transpiler/, the r186 GLSLDecoder, Linker, TSLEncoder and WGSLEncoder ).
const MONACO = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.55.1/min';

const STYLE = "\n\t\t\t* {\n\t\t\t\tbox-sizing: border-box;\n\t\t\t}\n\t\t\t#source {\n\t\t\t\tposition: absolute;\n\t\t\t\ttop: 50px;\n\t\t\t\tleft: 0;\n\t\t\t\twidth: calc(50% - 1px);\n\t\t\t\theight: calc(100% - 50px);\n\t\t\t}\n\t\t\t#result {\n\t\t\t\tposition: absolute;\n\t\t\t\ttop: 50px;\n\t\t\t\tright: 0;\n\t\t\t\twidth: 50%;\n\t\t\t\theight: calc(100% - 50px);\n\t\t\t}\n\t\t\t#source-dropdown {\n\t\t\t\tposition: absolute;\n\t\t\t\ttop: 0;\n\t\t\t\twidth: calc(50% - 1px);\n\t\t\t\tz-index: 100;\n\t\t\t\tpadding: 10px;\n\t\t\t\tbackground: #222;\n\t\t\t\ttext-align: center;\n\t\t\t}\n\t\t\t#result-dropdown {\n\t\t\t\tposition: absolute;\n\t\t\t\ttop: 0;\n\t\t\t\tleft: calc(50%);\n\t\t\t\twidth: 50%;\n\t\t\t\tpadding: 10px;\n\t\t\t\tz-index: 100;\n\t\t\t\tbackground: #222;\n\t\t\t\ttext-align: center;\n\t\t\t}\n\t\t\tselect {\n\t\t\t\tbackground: #333;\n\t\t\t\tcolor: white;\n\t\t\t\tborder: 1px solid #555;\n\t\t\t\tpadding: 5px 10px;\n\t\t\t}\n\t\t\tlabel {\n\t\t\t\tcolor: white;\n\t\t\t\tfont-family: Arial, sans-serif;\n\t\t\t\tfont-size: 14px;\n\t\t\t}\n\t\t";
const MARKUP = "<div id=\"source-dropdown\">\n\t\t\t<label for=\"decoder-select\">Encoder</label>\n\t\t\t<select id=\"decoder-select\">\n\t\t\t\t<option value=\"GLSL\">GLSL</option>\n\t\t\t</select>\n\t\t</div>\n\t\t<div id=\"result-dropdown\">\n\t\t\t<label for=\"encoder-select\">Decoder</label>\n\t\t\t<select id=\"encoder-select\">\n\t\t\t\t<option value=\"TSL\">TSL</option>\n\t\t\t\t<option value=\"WGSL\">WGSL</option>\n\t\t\t</select>\n\t\t</div>\n\t\t<div id=\"source\"></div>\n\t\t<div id=\"result\"></div>";
const GLSL = `/*
 * Perlin noise
 * https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
 */

const float PI = 3.141592653589793;

float rand(vec2 c){
	return fract(sin(dot(c.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

float noise(vec2 p, float freq ){
	float unit = 1./freq;
	vec2 ij = floor(p/unit);
	vec2 xy = mod(p,unit)/unit;
	//xy = 3.*xy*xy-2.*xy*xy*xy;
	xy = .5*(1.-cos(PI*xy));
	float a = rand((ij+vec2(0.,0.)));
	float b = rand((ij+vec2(1.,0.)));
	float c = rand((ij+vec2(0.,1.)));
	float d = rand((ij+vec2(1.,1.)));
	float x1 = mix(a, b, xy.x);
	float x2 = mix(c, d, xy.x);
	return mix(x1, x2, xy.y);
}

float pNoise(vec2 p, int res){
	float persistence = .5;
	float n = 0.;
	float normK = 0.;
	float f = 4.;
	float amp = 1.;
	int iCount = 0;
	for (int i = 0; i<50; i++){
		n+=amp*noise(p, f);
		f*=2.;
		normK+=amp;
		amp*=persistence;
		if (iCount == res) break;
		iCount++;
	}
	float nf = n/normK;
	return nf*nf*nf*nf;
}
`;

function loadMonaco() {
 return new Promise((resolve, reject) => {
  const script = document.createElement('script');
  script.src = `${MONACO}/vs/loader.js`;
  script.onload = () => {
   window.require.config({paths: {vs: `${MONACO}/vs`}});
   window.require(['vs/editor/editor.main'], () => resolve(window.monaco), reject);
  };
  script.onerror = () => reject(new Error('Monaco editor の読み込みに失敗しました。'));
  document.head.append(script);
 });
}

export async function startTranspiler(transpile, loading) {
 const style = document.createElement('style'); style.textContent = STYLE; document.head.append(style);
 const container = document.createElement('div'); container.innerHTML = MARKUP; document.body.append(...container.children);
 const monaco = await loadMonaco();
 loading.hidden = true;
 const options = {decoder: 'GLSL', encoder: 'TSL'};
 const encoderLanguages = {TSL: 'javascript', WGSL: 'wgsl'};
 let timeout = null;
 const editor = monaco.editor.create(document.getElementById('source'), {value: GLSL, language: 'c', theme: 'vs-dark', automaticLayout: true, wordWrap: 'on', minimap: {enabled: false}});
 const result = monaco.editor.create(document.getElementById('result'), {value: '', language: 'javascript', theme: 'vs-dark', automaticLayout: true, wordWrap: 'on', readOnly: true, minimap: {enabled: false}});
 const showCode = code => { result.setValue(code); result.revealLine(1); };
 const build = () => {
  // The Rust transpiler throws the original's error message.
  try { showCode(transpile(editor.getValue(), options.encoder)); } catch (e) { result.setValue('Error: ' + (e?.message ?? e)); }
  document.body.dataset.builds = String(Number(document.body.dataset.builds || 0) + 1);
 };
 build();
 editor.getModel().onDidChangeContent(() => { if (timeout) clearTimeout(timeout); timeout = setTimeout(build, 1000); });
 const decoderSelect = document.getElementById('decoder-select');
 const encoderSelect = document.getElementById('encoder-select');
 decoderSelect.addEventListener('change', () => { options.decoder = decoderSelect.value; build(); });
 encoderSelect.addEventListener('change', () => {
  options.encoder = encoderSelect.value;
  monaco.editor.setModelLanguage(result.getModel(), encoderLanguages[encoderSelect.value]);
  build();
 });
 window.transpilerEditors = {editor, result};
}

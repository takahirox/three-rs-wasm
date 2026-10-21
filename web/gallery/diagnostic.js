// An example page's diagnostic: why the example does not run here ( not
// ported, excluded, or its error ), the official preview and the features its
// source uses ( catalog.json ).
export function text(tag, value, parent) { const element = document.createElement(tag); element.textContent = value; parent.append(element); return element; }

// status: the page's data-status afterwards, the catalog's status by default.
export async function explain(id, error, status) {
 const loading = document.querySelector('#loading'), canvas = document.querySelector('canvas'), diagnostic = document.querySelector('#diagnostic');
 loading.hidden = true; canvas.hidden = true; diagnostic.hidden = false;
 const catalog = await (await fetch('catalog.json')).json();
 const entry = catalog.examples.find(example => example.id === id);
 text('h1', entry?.id.replaceAll('_', ' / ') || 'Unknown example', diagnostic);
 text('p', error || 'このexampleはまだRustへ移植されていません。', diagnostic);
 if (status) document.body.dataset.status = status;
 if (!entry) return;
 if (entry.preferred_example) {
  const preferred = catalog.examples.find(e => e.id === entry.preferred_example);
  const link = text('a', `WebGPU版: ${entry.preferred_example}${preferred?.port ? '' : '（移植待ち）'}`, diagnostic);
  link.href = `examples/${encodeURIComponent(entry.preferred_example)}.html`;
 }
 const preview = document.createElement('img'); preview.src = `screenshots/${entry.id}.jpg`; preview.alt = 'Three.js公式のプレビュー（Rustの実行結果ではありません）'; diagnostic.append(preview);
 text('p', preview.alt, diagnostic);
 const source = text('a', '固定したThree.jsのソースを見る ↗', diagnostic); source.href = entry.source; source.target = '_blank'; source.rel = 'noopener';
 text('h2', '必要機能の調査', diagnostic);
 text('p', '以下はソースから検出した機能です。実行検証済みの不足一覧ではなく、移植時に確認する対象を示します。', diagnostic);
 const list = document.createElement('ul'); diagnostic.append(list);
 for (const [feature, evidence] of Object.entries(entry.requirements)) {
  const li = text('li', catalog.features[feature], list);
  const details = document.createElement('details'); li.append(details); text('summary', '参照ソース', details);
  for (const item of evidence) { const link = text('a', `L${item.line}: ${item.text}`, text('p', '', details)); link.href = `${entry.source}#L${item.line}`; link.target = '_blank'; link.rel = 'noopener'; }
 }
 if (!list.children.length) text('p', '専用のシーン移植と比較テストが必要です。未対応機能なし、という判定ではありません。', diagnostic);
 document.body.dataset.status = status ?? entry.status;
}

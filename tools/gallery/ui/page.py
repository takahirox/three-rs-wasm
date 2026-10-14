"""The pinned examples' page chrome for the gallery's ports: the stylesheet a
page links ( main.css or example.css, copied to web/gallery/ui/css/ ), the
<style> blocks of its head and its #info element, read from the HTML as
published. Written to web/gallery/ui/pages.json.

Usage: python3 tools/gallery/ui/page.py [--check]
"""
import json
import re
import sys
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
EXAMPLES = ROOT / '.cache/three-r186/examples'
OUT = ROOT / 'web/gallery/ui/pages.json'


class Info(HTMLParser):
    """The outer HTML of the first element with id="info"."""

    VOID = {'br', 'img', 'input', 'hr', 'meta', 'link', 'source', 'wbr'}

    def __init__(self, html):
        super().__init__(convert_charrefs=False)
        self.html = html
        self.depth = 0
        self.start = None
        self.end = None
        self.feed(html)

    def position(self):
        line, column = self.getpos()
        lines = self.html.split('\n')
        return sum(len(l) + 1 for l in lines[:line - 1]) + column

    def handle_starttag(self, tag, attrs):
        if self.end is not None:
            return
        if self.start is None:
            if dict(attrs).get('id') == 'info':
                self.start = self.position()
                self.tag = tag
                self.depth = 1
            return
        if tag not in self.VOID:
            self.depth += 1

    def handle_endtag(self, tag):
        if self.start is None or self.end is not None or tag in self.VOID:
            return
        self.depth -= 1
        if self.depth == 0:
            rest = self.html[self.position():]
            self.end = self.position() + rest.index('>') + 1


def page(html):
    head = html.split('</head>')[0]
    css = 'example' if re.search(r'href="example\.css"', head) else 'main' if re.search(r'href="main\.css"', head) else None
    styles = [s.strip() for s in re.findall(r'<style[^>]*>([\s\S]*?)</style>', head)]
    # An #info left unclosed ends where the page's scripts begin.
    body = html.split('</head>', 1)[-1]
    html = html[:len(html) - len(body)] + body.split('<script', 1)[0]
    parser = Info(html)
    info = html[parser.start:parser.end].strip() if parser.start is not None else None
    return {'css': css, 'styles': styles, 'info': info}


def main():
    catalog = json.loads((ROOT / 'web/gallery/catalog.json').read_text())
    pages = {}
    for entry in catalog['examples']:
        path = EXAMPLES / f"{entry['id']}.html"
        if entry['port'] and path.exists():
            pages[entry['id']] = page(path.read_text())
    text = json.dumps(pages, indent='\t', ensure_ascii=False, sort_keys=True) + '\n'
    if '--check' in sys.argv:
        if OUT.read_text() != text:
            sys.exit('web/gallery/ui/pages.json is stale: run python3 tools/gallery/ui/page.py')
        return
    OUT.write_text(text)
    print(f'{len(pages)} pages')


if __name__ == '__main__':
    main()

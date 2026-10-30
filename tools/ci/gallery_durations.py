#!/usr/bin/env python3
"""The published-gallery check's durations on CI's software GPU
( tests/pages/durations.json ), from the JSON reports its shards upload
( gallery-report-<n>, .github/workflows/pages.yml ). tests/pages/gallery.spec.js
shares the examples between the shards by them.

  gh run download <run> -p 'gallery-report-*' -D <dir>
  gallery_durations.py <dir>
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
PREFIX = 'published gallery: '


def specs(suite):
    yield from suite.get('specs', [])
    for child in suite.get('suites', []):
        yield from specs(child)


def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    durations = {}
    for report in sorted(pathlib.Path(sys.argv[1]).rglob('*.json')):
        for suite in json.loads(report.read_text())['suites']:
            for spec in specs(suite):
                results = [r for t in spec['tests'] for r in t['results'] if r['status'] != 'skipped']
                if not results:
                    continue
                title = spec['title']
                # The spec names its site tests by their titles' starts.
                name = title[len(PREFIX):] if title.startswith(PREFIX) else next(
                    (n for n in ('site entry', 'official-style example URLs') if title.startswith(n)), None)
                if name:
                    # Seconds, of the run's last attempt.
                    durations[name] = round(results[-1]['duration'] / 1000, 1)
    if not durations:
        raise SystemExit(f'no gallery results under {sys.argv[1]}')
    out = ROOT / 'tests/pages/durations.json'
    out.write_text(json.dumps(dict(sorted(durations.items())), indent=1) + '\n')
    print(f'{out.relative_to(ROOT)}: {len(durations)} tests, {sum(durations.values()) / 60:.1f} minutes')


if __name__ == '__main__':
    main()

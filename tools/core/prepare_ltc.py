"""Convert the pinned Three.js LTC tables to little-endian half floats."""
import re
import struct
import sys
from pathlib import Path
root=Path(__file__).resolve().parents[2]
source=(root/'.cache/three-r186/examples/jsm/lights/RectAreaLightTexturesLib.js').read_text()
data=bytearray()
for table in (1,2):
    text=re.search(r'const LTC_MAT_'+str(table)+r' = \[([^\]]+)\]',source)[1]
    values=[float(v.strip().replace(' ','')) for v in text.split(',') if v.strip()]
    assert len(values)==64*64*4
    data.extend(b''.join(struct.pack('<e',value) for value in values))
target=root/'src/shaders/ltc-r186.bin'
if '--check' in sys.argv: assert target.read_bytes()==data
else: target.write_bytes(data)

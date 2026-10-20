"""A minimal WebAssembly binary reader and writer for the gallery's module
splitting: sections, function bodies, data segments, the linker's symbol
table and relocations ( wasm-ld --emit-relocs ), the name section."""


def leb(b, i):
    r = s = 0
    while True:
        x = b[i]
        i += 1
        r |= (x & 0x7f) << s
        s += 7
        if x < 0x80:
            return r, i


def sleb(b, i):
    r = s = 0
    while True:
        x = b[i]
        i += 1
        r |= (x & 0x7f) << s
        s += 7
        if x < 0x80:
            if x & 0x40:
                r -= 1 << s
            return r, i


def enc(n):
    out = bytearray()
    while True:
        x = n & 0x7f
        n >>= 7
        if n:
            out.append(x | 0x80)
        else:
            out.append(x)
            return bytes(out)


def enc_s(n):
    out = bytearray()
    while True:
        x = n & 0x7f
        n >>= 7
        if (n == 0 and not x & 0x40) or (n == -1 and x & 0x40):
            out.append(x)
            return bytes(out)
        out.append(x | 0x80)


def name(b, i):
    n, i = leb(b, i)
    return b[i:i + n].decode('utf-8', 'replace'), i + n


def sections(b):
    """[( id, custom name or None, payload start, payload end )]."""
    assert b[:4] == b'\0asm'
    out = []
    i = 8
    while i < len(b):
        sid = b[i]
        size, j = leb(b, i + 1)
        end = j + size
        cname = None
        if sid == 0:
            cname, k = name(b, j)
        out.append((sid, cname, j, end))
        i = end
    return out


def section(b, secs, sid, cname=None):
    for s in secs:
        if s[0] == sid and s[1] == cname:
            return s
    return None


def imported_functions(b, secs):
    s = section(b, secs, 2)
    if not s:
        return 0
    i = s[2]
    n, i = leb(b, i)
    count = 0
    for _ in range(n):
        _, i = name(b, i)
        _, i = name(b, i)
        kind = b[i]
        i += 1
        if kind == 0:
            _, i = leb(b, i)
            count += 1
        elif kind == 1:
            i += 1
            flags, i = leb(b, i)
            _, i = leb(b, i)
            if flags & 1:
                _, i = leb(b, i)
        elif kind == 2:
            flags, i = leb(b, i)
            _, i = leb(b, i)
            if flags & 1:
                _, i = leb(b, i)
        elif kind == 3:
            i += 1
            _, i = leb(b, i)  # mutability byte read as leb ( 0 / 1 )
        elif kind == 4:
            i += 1
            _, i = leb(b, i)
    return count


def bodies(b, secs):
    """Function bodies: [( start, end )] relative to the code section payload."""
    s = section(b, secs, 10)
    base = s[2]
    n, i = leb(b, base)
    out = []
    for _ in range(n):
        size, j = leb(b, i)
        out.append((i - base, j + size - base))
        i = j + size
    return out


def data_segments(b, secs):
    """[( payload offset of the bytes ( relative to the data section ), memory address, size )]."""
    s = section(b, secs, 11)
    base = s[2]
    n, i = leb(b, base)
    out = []
    for _ in range(n):
        flags, i = leb(b, i)
        addr = None
        if flags in (0, 2):
            if flags == 2:
                _, i = leb(b, i)
            assert b[i] == 0x41
            addr, i = sleb(b, i + 1)
            assert b[i] == 0x0b
            i += 1
        size, i = leb(b, i)
        out.append((i - base, addr, size))
        i += size
    return out


def function_names(b, secs):
    s = section(b, secs, 0, 'name')
    out = {}
    if not s:
        return out
    _, i = name(b, s[2])
    while i < s[3]:
        sub = b[i]
        size, i = leb(b, i + 1)
        end = i + size
        if sub == 1:
            n, j = leb(b, i)
            for _ in range(n):
                idx, j = leb(b, j)
                nm, j = name(b, j)
                out[idx] = nm
        i = end
    return out


ADDEND = {3, 4, 5, 8, 9, 11, 14, 15, 16, 17, 21, 22, 23, 25}


def relocations(b, secs, cname):
    s = section(b, secs, 0, cname)
    out = []
    if not s:
        return out
    _, i = name(b, s[2])
    _, i = leb(b, i)
    n, i = leb(b, i)
    for _ in range(n):
        t = b[i]
        off, i = leb(b, i + 1)
        idx, i = leb(b, i)
        add = 0
        if t in ADDEND:
            add, i = sleb(b, i)
        out.append((t, off, idx, add))
    return out


def symbols(b, secs):
    """The linking section's symbol table: [( kind, flags, index or None, name,
    ( segment, offset, size ) or None )]."""
    s = section(b, secs, 0, 'linking')
    _, i = name(b, s[2])
    _, i = leb(b, i)  # version
    out = []
    while i < s[3]:
        sub = b[i]
        size, i = leb(b, i + 1)
        end = i + size
        if sub == 8:
            n, j = leb(b, i)
            for _ in range(n):
                kind = b[j]
                flags, j = leb(b, j + 1)
                index = nm = loc = None
                if kind in (0, 2, 4, 5):
                    index, j = leb(b, j)
                    if not flags & 0x10 or flags & 0x40:
                        nm, j = name(b, j)
                elif kind == 1:
                    nm, j = name(b, j)
                    if not flags & 0x10:
                        seg, j = leb(b, j)
                        off, j = leb(b, j)
                        sz, j = leb(b, j)
                        loc = (seg, off, sz)
                elif kind == 3:
                    index, j = leb(b, j)
                out.append((kind, flags, index, nm, loc))
        i = end
    return out


def section_bytes(sid, payload):
    return bytes([sid]) + enc(len(payload)) + payload


def without_custom(b, drop):
    """The module without the custom sections whose names are in `drop`."""
    out = bytearray(b[:8])
    for sid, cname, start, end in sections(b):
        if sid == 0 and cname in drop:
            continue
        out += bytes([sid]) + enc(end - start) + b[start:end]
    return bytes(out)


def replace_sections(b, new):
    """Rebuild the module with `new` ( {id: payload or None} ) replacing or
    removing sections, a new id inserted in the standard order."""
    order = [1, 2, 3, 4, 5, 13, 6, 7, 8, 9, 12, 10, 11]
    secs = sections(b)
    out = bytearray(b[:8])
    present = {s[0] for s in secs}
    pending = {k: v for k, v in new.items() if k not in present and v is not None}
    for sid, cname, start, end in secs:
        if sid != 0:
            for k in list(pending):
                if order.index(k) < order.index(sid):
                    out += section_bytes(k, pending.pop(k))
        if sid != 0 and sid in new:
            if new[sid] is not None:
                out += section_bytes(sid, new[sid])
            continue
        out += bytes([sid]) + enc(end - start) + b[start:end]
    for k in sorted(pending, key=order.index):
        out += section_bytes(k, pending.pop(k))
    return bytes(out)


def data_payload(segments):
    """An active data section for memory 0: [( address, bytes )]."""
    out = bytearray(enc(len(segments)))
    for addr, data in segments:
        out += b'\x00\x41' + enc_s(addr) + b'\x0b' + enc(len(data)) + data
    return bytes(out)


def has_memory_import(b, secs):
    s = section(b, secs, 2)
    if not s:
        return False
    i = s[2]
    n, i = leb(b, i)
    for _ in range(n):
        _, i = name(b, i)
        _, i = name(b, i)
        kind = b[i]
        i += 1
        if kind == 2:
            return True
        if kind == 0:
            _, i = leb(b, i)
        elif kind == 1:
            i += 1
            flags, i = leb(b, i)
            _, i = leb(b, i)
            if flags & 1:
                _, i = leb(b, i)
        elif kind == 3:
            i += 2
        elif kind == 4:
            i += 1
            _, i = leb(b, i)
    return False


def with_index_names(b):
    """The module with every function named F<index> ( unique, and unchanged by
    wasm-bindgen's demangling ) in place of its name section."""
    secs = sections(b)
    nimp = imported_functions(b, secs)
    count = nimp + len(bodies(b, secs))
    sub = bytearray(enc(count))
    for i in range(count):
        nm = f'F{i}'.encode()
        sub += enc(i) + enc(len(nm)) + nm
    payload = enc(4) + b'name' + bytes([1]) + enc(len(sub)) + sub
    out = bytearray(b[:8])
    for sid, cname, start, end in secs:
        if sid == 0 and cname == 'name':
            out += section_bytes(0, payload)
            continue
        out += bytes([sid]) + enc(end - start) + b[start:end]
    return bytes(out)

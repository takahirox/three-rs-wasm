//! The JS string and regular-expression behaviour the transpiler relies on,
//! written out: `\w`, `\s`, `\b`, `.`, `trim()`, `slice()` and the
//! replacement patterns of `String.prototype.replace`.

/// `\w`: ASCII word characters.
pub fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// `\s`: JS WhiteSpace and LineTerminator characters.
pub fn is_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
                | '\u{FEFF}'
    )
}

/// The characters `.` does not match.
pub fn is_line_terminator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// `String.prototype.trim()`.
pub fn trim(s: &str) -> &str {
    s.trim_matches(is_space)
}

/// `String.prototype.trimEnd()`.
pub fn trim_end(s: &str) -> &str {
    s.trim_end_matches(is_space)
}

/// `\b` between `chars[i - 1]` and `chars[i]`.
pub fn boundary(chars: &[char], i: usize) -> bool {
    let before = i > 0 && is_word(chars[i - 1]);
    let after = i < chars.len() && is_word(chars[i]);
    before != after
}

/// `Array.prototype.slice( start, end )` index resolution.
pub fn slice_range(len: usize, start: isize, end: Option<isize>) -> (usize, usize) {
    let resolve = |v: isize| -> usize {
        if v < 0 {
            (len as isize + v).max(0) as usize
        } else {
            (v as usize).min(len)
        }
    };
    let s = resolve(start);
    let e = end.map_or(len, resolve);
    (s, e.max(s))
}

pub fn slice<T>(items: &[T], start: isize, end: Option<isize>) -> &[T] {
    let (s, e) = slice_range(items.len(), start, end);
    &items[s..e]
}

/// The replacement string of `replace()`: `$$`, `$&`, `` $` `` and `$'`
/// ( the patterns here have no capture groups, so `$n` stays literal ).
pub fn expand_replacement(
    replacement: &str,
    haystack: &[char],
    start: usize,
    end: usize,
) -> String {
    let r: Vec<char> = replacement.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < r.len() {
        if r[i] == '$' && i + 1 < r.len() {
            match r[i + 1] {
                '$' => {
                    out.push('$');
                    i += 2;
                    continue;
                }
                '&' => {
                    out.extend(&haystack[start..end]);
                    i += 2;
                    continue;
                }
                '`' => {
                    out.extend(&haystack[..start]);
                    i += 2;
                    continue;
                }
                '\'' => {
                    out.extend(&haystack[end..]);
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }
        out.push(r[i]);
        i += 1;
    }
    out
}

/// `str.replace( /\bword\b/g, replacement )` for a `\w+` word.
pub fn replace_word(s: &str, word: &str, replacement: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let w: Vec<char> = word.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut last = 0;
    while i + w.len() <= chars.len() {
        if !w.is_empty()
            && chars[i..i + w.len()] == w[..]
            && boundary(&chars, i)
            && boundary(&chars, i + w.len())
        {
            out.extend(&chars[last..i]);
            out.push_str(&expand_replacement(replacement, &chars, i, i + w.len()));
            i += w.len();
            last = i;
        } else {
            i += 1;
        }
    }
    out.extend(&chars[last..]);
    out
}

/// `/\bword\b/.test( s )`.
pub fn has_word(s: &str, word: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let w: Vec<char> = word.chars().collect();
    !w.is_empty()
        && (0..=chars.len().saturating_sub(w.len())).any(|i| {
            i + w.len() <= chars.len()
                && chars[i..i + w.len()] == w[..]
                && boundary(&chars, i)
                && boundary(&chars, i + w.len())
        })
}

/// A JS error, as the example shows `'Error: ' + e.message`.
#[derive(Debug, Clone, PartialEq)]
pub struct JsError(pub String);

impl std::fmt::Display for JsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub type Result<T> = std::result::Result<T, JsError>;

pub fn error<T>(message: impl Into<String>) -> Result<T> {
    Err(JsError(message.into()))
}

/// V8's TypeError for a property read of `undefined`.
pub fn read_undefined(property: &str) -> JsError {
    JsError(format!(
        "Cannot read properties of undefined (reading '{property}')"
    ))
}

/// V8's TypeError for a property read of `null`.
pub fn read_null(property: &str) -> JsError {
    JsError(format!(
        "Cannot read properties of null (reading '{property}')"
    ))
}

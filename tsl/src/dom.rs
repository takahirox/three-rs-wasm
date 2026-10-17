//! The DOMParser the addons parse XML documents with ( MaterialXLoader's
//! `new DOMParser().parseFromString( text, 'application/xml' )` ): a
//! document whose `documentElement` and its descendants have `nodeName`,
//! `children` ( the element children ), `getAttribute( name )` ( `null`
//! when absent ) and `hasAttribute`, with the XML entities decoded.
use crate::rt::*;

/// `name="value"` pairs and the children of one element.
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

fn decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let Some(end) = rest.find(';') else {
            out.push_str(rest);
            return out;
        };
        let entity = &rest[1..end];
        let ch = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            e if e.starts_with("#x") => u32::from_str_radix(&e[2..], 16).ok().and_then(char::from_u32),
            e if e.starts_with('#') => e[1..].parse().ok().and_then(char::from_u32),
            _ => None,
        };
        match ch {
            Some(c) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

struct Parser<'a> {
    s: &'a str,
    i: usize,
}
impl Parser<'_> {
    fn rest(&self) -> &str {
        &self.s[self.i..]
    }
    fn skip_space(&mut self) {
        while let Some(c) = self.rest().chars().next() {
            if c.is_whitespace() {
                self.i += c.len_utf8();
            } else {
                break;
            }
        }
    }
    fn skip_until(&mut self, end: &str) -> Result<(), String> {
        match self.rest().find(end) {
            Some(n) => {
                self.i += n + end.len();
                Ok(())
            }
            None => Err(format!("unterminated {end}")),
        }
    }
    /// Comments, processing instructions, doctypes and character data.
    fn skip_misc(&mut self) -> Result<(), String> {
        loop {
            let r = self.rest();
            if r.starts_with("<!--") {
                self.skip_until("-->")?;
            } else if r.starts_with("<?") {
                self.skip_until("?>")?;
            } else if r.starts_with("<![CDATA[") {
                self.skip_until("]]>")?;
            } else if r.starts_with("<!") {
                self.skip_until(">")?;
            } else if r.starts_with('<') || r.is_empty() {
                return Ok(());
            } else {
                // Text content: the addons read elements only.
                let n = r.find('<').unwrap_or(r.len());
                self.i += n;
            }
        }
    }
    fn name(&mut self) -> String {
        let r = self.rest();
        let n = r
            .find(|c: char| c.is_whitespace() || c == '/' || c == '>' || c == '=')
            .unwrap_or(r.len());
        let name = r[..n].to_string();
        self.i += n;
        name
    }
    fn element(&mut self) -> Result<Element, String> {
        if !self.rest().starts_with('<') {
            return Err("expected an element".into());
        }
        self.i += 1;
        let name = self.name();
        let mut attributes = vec![];
        loop {
            self.skip_space();
            let r = self.rest();
            if r.starts_with("/>") {
                self.i += 2;
                return Ok(Element {
                    name,
                    attributes,
                    children: vec![],
                });
            }
            if r.starts_with('>') {
                self.i += 1;
                break;
            }
            if r.is_empty() {
                return Err("unterminated start tag".into());
            }
            let key = self.name();
            self.skip_space();
            if !self.rest().starts_with('=') {
                return Err(format!("attribute {key} without a value"));
            }
            self.i += 1;
            self.skip_space();
            let quote = self.rest().chars().next().ok_or("unterminated attribute")?;
            if quote != '"' && quote != '\'' {
                return Err(format!("attribute {key} without quotes"));
            }
            self.i += 1;
            let end = self.rest().find(quote).ok_or("unterminated attribute")?;
            let value = decode(&self.rest()[..end]);
            self.i += end + 1;
            if attributes.iter().any(|(k, _)| *k == key) {
                return Err(format!("duplicate attribute {key}"));
            }
            attributes.push((key, value));
        }
        let mut children = vec![];
        loop {
            self.skip_misc()?;
            if self.rest().starts_with("</") {
                self.i += 2;
                let close = self.name();
                if close != name {
                    return Err(format!("mismatched end tag {close} for {name}"));
                }
                self.skip_space();
                if !self.rest().starts_with('>') {
                    return Err("unterminated end tag".into());
                }
                self.i += 1;
                return Ok(Element {
                    name,
                    attributes,
                    children,
                });
            }
            if self.rest().is_empty() {
                return Err(format!("unterminated element {name}"));
            }
            children.push(self.element()?);
        }
    }
}

fn parse(text: &str) -> Result<Element, String> {
    let mut p = Parser { s: text, i: 0 };
    if p.rest().starts_with('\u{feff}') {
        p.i += 3;
    }
    p.skip_misc()?;
    let root = p.element()?;
    p.skip_misc()?;
    if !p.rest().trim().is_empty() {
        return Err("content after the document element".into());
    }
    Ok(root)
}

fn to_val(e: Element) -> Val {
    let attributes = std::rc::Rc::new(e.attributes);
    let a = attributes.clone();
    let get_attribute = func("getAttribute", 1, move |_t, args| {
        let name = display(&arg(args, 0));
        Ok(a.iter()
            .find(|(k, _)| *k == name)
            .map_or(Val::Null, |(_, v)| s(v)))
    });
    let a = attributes.clone();
    let has_attribute = func("hasAttribute", 1, move |_t, args| {
        let name = display(&arg(args, 0));
        Ok(Val::Bool(a.iter().any(|(k, _)| *k == name)))
    });
    let children: Vec<Val> = e.children.into_iter().map(to_val).collect();
    object(&[
        ("nodeName", s(&e.name)),
        ("tagName", s(&e.name)),
        ("children", array(children)),
        ("getAttribute", get_attribute),
        ("hasAttribute", has_attribute),
    ])
}

/// Define `DOMParser` globally ( unless the host has one ).
pub fn install() {
    let parse_from_string = func("parseFromString", 2, |_t, args| {
        let text = display(&arg(args, 0));
        // A malformed document parses to a <parsererror> root, as browsers
        // report it.
        let root = match parse(&text) {
            Ok(e) => to_val(e),
            Err(message) => to_val(Element {
                name: "parsererror".into(),
                attributes: vec![("message".into(), message)],
                children: vec![],
            }),
        };
        Ok(object(&[("documentElement", root)]))
    });
    let ctor = func_ctor("DOMParser", 0, move |this, _args| {
        set(this, "parseFromString", parse_from_string.clone())?;
        Ok(Val::Undef)
    });
    define_global("DOMParser", ctor);
}

/// Define `TextDecoder` ( UTF-8 ) globally, as the addons decode documents.
pub fn install_text_decoder() {
    let ctor = func_ctor("TextDecoder", 0, |this, _args| {
        let decode = func("decode", 1, |_t, args| {
            let bytes: Vec<u8> = numbers(&arg(args, 0))?.into_iter().map(|b| b as u8).collect();
            Ok(s(&String::from_utf8_lossy(&bytes)))
        });
        set(this, "decode", decode)?;
        Ok(Val::Undef)
    });
    define_global("TextDecoder", ctor);
}

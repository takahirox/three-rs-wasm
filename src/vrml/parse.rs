//! VRMLLoader's chevrotain lexer and CST parser, and its CST → AST visitor:
//! the token rules tried in the original's order ( the first pattern that
//! matches wins unless its `longer_alt` matches more ), the grammar's rules,
//! and each field's values grouped by token type as the visitor reads them.
use crate::{Error, Result};

fn bad(what: &str) -> Error {
    Error::Asset(format!("THREE.VRMLLoader: {what}"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    NodeName,
    Def,
    Use,
    Route,
    To,
    True,
    False,
    Null,
    Version,
    Identifier,
    RouteIdentifier,
    String,
    Hex,
    Number,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
}

#[derive(Clone, Debug)]
struct Token {
    kind: Kind,
    image: String,
}

/// The node types, in the NodeName pattern's alternation order.
const NODE_TYPES: [&str; 54] = [
    "Anchor",
    "Billboard",
    "Collision",
    "Group",
    "Transform",
    "Inline",
    "LOD",
    "Switch",
    "PerspectiveCamera",
    "OrthographicCamera",
    "AudioClip",
    "DirectionalLight",
    "PointLight",
    "Script",
    "Shape",
    "Sound",
    "SpotLight",
    "WorldInfo",
    "CylinderSensor",
    "PlaneSensor",
    "ProximitySensor",
    "SphereSensor",
    "TimeSensor",
    "TouchSensor",
    "VisibilitySensor",
    "Box",
    "Cone",
    "Cylinder",
    "ElevationGrid",
    "Extrusion",
    "IndexedFaceSet",
    "IndexedLineSet",
    "PointSet",
    "Sphere",
    "Color",
    "Coordinate",
    "Normal",
    "TextureCoordinate",
    "Appearance",
    "FontStyle",
    "ImageTexture",
    "Material",
    "MovieTexture",
    "PixelTexture",
    "TextureTransform",
    "ColorInterpolator",
    "CoordinateInterpolator",
    "NormalInterpolator",
    "OrientationInterpolator",
    "PositionInterpolator",
    "ScalarInterpolator",
    "Background",
    "Fog",
    "NavigationInfo",
];

/// JavaScript's `\s`.
fn js_space(c: char) -> bool {
    crate::fbx::js::is_js_space(c)
}
fn line_terminator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}
/// The characters `[^\x30-\x39\0-\x20"'#+,-.[]\{}]`: an identifier's first.
fn first(c: char) -> bool {
    !(c.is_ascii_digit()
        || (c as u32) <= 0x20
        || matches!(
            c,
            '"' | '\'' | '#' | '+' | ',' | '-' | '.' | '[' | ']' | '\\' | '{' | '}'
        ))
}
/// `[^\0-\x20"'#+,.[]\{}]`: an identifier's rest ( digits and `-` allowed ).
fn rest(c: char) -> bool {
    !((c as u32) <= 0x20
        || matches!(
            c,
            '"' | '\'' | '#' | '+' | ',' | '.' | '[' | ']' | '\\' | '{' | '}'
        ))
}
/// `[^\0-\x20"'#+,-.[]\{}]`: a route identifier part's rest ( digits allowed ).
fn route_rest(c: char) -> bool {
    !((c as u32) <= 0x20
        || matches!(
            c,
            '"' | '\'' | '#' | '+' | ',' | '-' | '.' | '[' | ']' | '\\' | '{' | '}'
        ))
}

/// The length in chars of each pattern's match at the start of `s`.
fn identifier(s: &[char]) -> usize {
    if s.first().is_some_and(|&c| first(c)) {
        1 + s[1..].iter().take_while(|&&c| rest(c)).count()
    } else {
        0
    }
}
fn route_identifier(s: &[char]) -> usize {
    let part = |s: &[char]| -> usize {
        if s.first().is_some_and(|&c| first(c)) {
            1 + s[1..].iter().take_while(|&&c| route_rest(c)).count()
        } else {
            0
        }
    };
    let a = part(s);
    if a == 0 || s.get(a) != Some(&'.') {
        return 0;
    }
    let b = part(&s[a + 1..]);
    if b == 0 { 0 } else { a + 1 + b }
}
fn literal(s: &[char], word: &str) -> usize {
    let n = word.chars().count();
    if s.len() >= n && s[..n].iter().copied().eq(word.chars()) {
        n
    } else {
        0
    }
}
fn string(s: &[char]) -> usize {
    if s.first() != Some(&'"') {
        return 0;
    }
    let mut i = 1;
    while i < s.len() {
        match s[i] {
            '"' => return i + 1,
            '\\' => match s.get(i + 1) {
                Some('b' | 'f' | 'n' | 'r' | 't' | 'v' | '"' | '\\' | '/') => i += 2,
                Some('u')
                    if s.len() > i + 5 && s[i + 2..i + 6].iter().all(|c| c.is_ascii_hexdigit()) =>
                {
                    i += 6
                }
                _ => return 0,
            },
            _ => i += 1,
        }
    }
    0
}
fn hex(s: &[char]) -> usize {
    if s.first() == Some(&'0') && matches!(s.get(1), Some('x' | 'X')) {
        let n = s[2..].iter().take_while(|c| c.is_ascii_hexdigit()).count();
        if n > 0 {
            return 2 + n;
        }
    }
    0
}
/// `[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?` with the regex's backtracking.
fn number(s: &[char]) -> usize {
    let mut i = 0;
    if matches!(s.first(), Some('-' | '+')) {
        i = 1;
    }
    let digits = |from: usize| {
        s[from.min(s.len())..]
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .count()
    };
    let int = digits(i);
    // [0-9]* \.? [0-9]+: a fraction after the point, else the integer's digits.
    let end = if s.get(i + int) == Some(&'.') && digits(i + int + 1) > 0 {
        i + int + 1 + digits(i + int + 1)
    } else if int > 0 {
        i + int
    } else {
        return 0;
    };
    if matches!(s.get(end), Some('e' | 'E')) {
        let mut j = end + 1;
        if matches!(s.get(j), Some('-' | '+')) {
            j += 1;
        }
        let e = digits(j);
        if e > 0 {
            return j + e;
        }
    }
    end
}

/// Lexer.tokenize(): skipped whitespace and comments, errors for the rest.
fn lex(text: &str) -> Result<Vec<Token>> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = vec![];
    let mut i = 0;
    while i < chars.len() {
        let s = &chars[i..];
        let c = s[0];
        // WhiteSpace: /[ ,\s]/
        if c == ' ' || c == ',' || js_space(c) {
            i += 1;
            continue;
        }
        let mut found: Option<(Kind, usize)> = None;
        let with_alt = |kind: Kind, n: usize| -> (Kind, usize) {
            // The longer alternative alone ( Identifier's own is not tried ).
            let alt = identifier(s);
            if alt > n {
                (Kind::Identifier, alt)
            } else {
                (kind, n)
            }
        };
        if let Some(name) = NODE_TYPES
            .iter()
            .chain(std::iter::once(&"Viewpoint"))
            .chain(std::iter::once(&"Text"))
            .find(|n| literal(s, n) > 0)
        {
            found = Some(with_alt(Kind::NodeName, name.chars().count()));
        }
        if found.is_none() {
            for (kind, word) in [
                (Kind::Def, "DEF"),
                (Kind::Use, "USE"),
                (Kind::Route, "ROUTE"),
                (Kind::To, "TO"),
            ] {
                if literal(s, word) > 0 {
                    found = Some(with_alt(kind, word.len()));
                    break;
                }
            }
        }
        if found.is_none() {
            for (kind, word) in [
                (Kind::True, "TRUE"),
                (Kind::False, "FALSE"),
                (Kind::Null, "NULL"),
            ] {
                if literal(s, word) > 0 {
                    found = Some((kind, word.len()));
                    break;
                }
            }
        }
        if found.is_none() && literal(s, "#VRML") > 0 {
            let n = s.iter().take_while(|&&c| !line_terminator(c)).count();
            found = Some(with_alt(Kind::Version, n));
        }
        if found.is_none() {
            let n = identifier(s);
            if n > 0 {
                let route = route_identifier(s);
                found = Some(if route > n {
                    (Kind::RouteIdentifier, route)
                } else {
                    (Kind::Identifier, n)
                });
            }
        }
        if found.is_none() {
            let n = route_identifier(s);
            if n > 0 {
                found = Some((Kind::RouteIdentifier, n));
            }
        }
        if found.is_none() {
            for (kind, n) in [
                (Kind::String, string(s)),
                (Kind::Hex, hex(s)),
                (Kind::Number, number(s)),
            ] {
                if n > 0 {
                    found = Some((kind, n));
                    break;
                }
            }
        }
        if found.is_none() {
            found = match c {
                '[' => Some((Kind::LSquare, 1)),
                ']' => Some((Kind::RSquare, 1)),
                '{' => Some((Kind::LCurly, 1)),
                '}' => Some((Kind::RCurly, 1)),
                _ => None,
            };
        }
        if found.is_none() && c == '#' {
            // Comment: /#.*/, skipped.
            i += s.iter().take_while(|&&c| !line_terminator(c)).count();
            continue;
        }
        let Some((kind, n)) = found else {
            return Err(Error::Asset(
                "THREE.VRMLLexer: Lexing errors detected.".into(),
            ));
        };
        tokens.push(Token {
            kind,
            image: chars[i..i + n].iter().collect(),
        });
        i += n;
    }
    Ok(tokens)
}

/// A field: its values as processField() collects them, grouped by token
/// type ( nodes, USEs, strings, numbers, hexes, TRUEs, FALSEs, NULLs ), and
/// the type of the last group present.
#[derive(Clone, Debug, Default)]
pub(crate) struct Field {
    pub name: String,
    pub kind: Option<&'static str>,
    pub values: Vec<Value>,
}

/// A parsed node ( the AST's `{ name, fields, DEF }` ).
#[derive(Debug)]
pub(crate) struct Node {
    pub name: String,
    pub fields: Vec<Field>,
    pub def: Option<String>,
}
pub(crate) type NodeRef = std::rc::Rc<Node>;

#[derive(Clone, Debug)]
pub(crate) enum Value {
    Node(NodeRef),
    Use(String),
    String(String),
    Number(f64),
    Hex(String),
    Bool(bool),
    Null,
}
impl Value {
    /// The value used arithmetically ( `Number( value )` ).
    pub(crate) fn number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Bool(b) => f64::from(u8::from(*b)),
            Value::Null => 0.,
            Value::String(s) | Value::Hex(s) => crate::fbx::js::string_to_number(s),
            _ => f64::NAN,
        }
    }
}
/// The values of one field value rule, by token type.
#[derive(Default)]
struct Groups {
    nodes: Vec<NodeRef>,
    uses: Vec<String>,
    strings: Vec<String>,
    numbers: Vec<f64>,
    hexes: Vec<String>,
    trues: usize,
    falses: usize,
    nulls: usize,
}

/// The AST: the nodes in order ( routes are not used ).
pub(crate) struct Tree {
    pub nodes: Vec<NodeRef>,
}

struct Parser {
    tokens: Vec<Token>,
    at: usize,
}
impl Parser {
    fn peek(&self) -> Option<Kind> {
        self.tokens.get(self.at).map(|t| t.kind)
    }
    fn consume(&mut self, kind: Kind) -> Result<String> {
        match self.tokens.get(self.at) {
            Some(t) if t.kind == kind => {
                self.at += 1;
                Ok(t.image.clone())
            }
            _ => Err(bad("Parsing errors detected.")),
        }
    }
    fn node(&mut self) -> Result<NodeRef> {
        let def = if self.peek() == Some(Kind::Def) {
            self.consume(Kind::Def)?;
            Some(match self.peek() {
                Some(Kind::Identifier) => self.consume(Kind::Identifier)?,
                _ => self.consume(Kind::NodeName)?,
            })
        } else {
            None
        };
        let name = self.consume(Kind::NodeName)?;
        self.consume(Kind::LCurly)?;
        let mut fields = vec![];
        while self.peek() == Some(Kind::Identifier) {
            fields.push(self.field()?);
        }
        self.consume(Kind::RCurly)?;
        Ok(std::rc::Rc::new(Node { name, fields, def }))
    }
    fn value(&mut self, field: &mut Groups, single: bool) -> Result<bool> {
        match self.peek() {
            Some(Kind::Def | Kind::NodeName) => field.nodes.push(self.node()?),
            Some(Kind::Use) => {
                self.consume(Kind::Use)?;
                field.uses.push(match self.peek() {
                    Some(Kind::Identifier) => self.consume(Kind::Identifier)?,
                    _ => self.consume(Kind::NodeName)?,
                });
            }
            Some(Kind::String) => field.strings.push(self.consume(Kind::String)?),
            Some(Kind::Hex) => field.hexes.push(self.consume(Kind::Hex)?),
            Some(Kind::Number) => {
                let image = self.consume(Kind::Number)?;
                field.numbers.push(crate::fbx::js::parse_float(&image));
            }
            Some(Kind::True) if single => {
                self.consume(Kind::True)?;
                field.trues += 1;
            }
            Some(Kind::False) if single => {
                self.consume(Kind::False)?;
                field.falses += 1;
            }
            Some(Kind::Null) => {
                self.consume(Kind::Null)?;
                field.nulls += 1;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }
    fn field(&mut self) -> Result<Field> {
        let name = self.consume(Kind::Identifier)?;
        let mut g = Groups::default();
        if self.peek() == Some(Kind::LSquare) {
            self.consume(Kind::LSquare)?;
            while self.value(&mut g, false)? {}
            self.consume(Kind::RSquare)?;
        } else {
            // AT_LEAST_ONE singleFieldValue alternative.
            if !self.value(&mut g, true)? {
                return Err(bad("Parsing errors detected."));
            }
            while self.value(&mut g, true)? {}
        }
        // processField(): each group in turn, the last present naming the
        // type; string literals lose their quotes.
        let mut field = Field {
            name,
            ..Default::default()
        };
        let mut group = |present: bool, kind: &'static str, values: Vec<Value>| {
            if present {
                field.kind = Some(kind);
                field.values.extend(values);
            }
        };
        group(
            !g.nodes.is_empty(),
            "node",
            g.nodes.into_iter().map(Value::Node).collect(),
        );
        group(
            !g.uses.is_empty(),
            "use",
            g.uses.into_iter().map(Value::Use).collect(),
        );
        group(
            !g.strings.is_empty(),
            "string",
            g.strings
                .into_iter()
                .map(|s| Value::String(s.replace(['\'', '"'], "")))
                .collect(),
        );
        group(
            !g.numbers.is_empty(),
            "number",
            g.numbers.into_iter().map(Value::Number).collect(),
        );
        group(
            !g.hexes.is_empty(),
            "hex",
            g.hexes.into_iter().map(Value::Hex).collect(),
        );
        group(g.trues > 0, "boolean", vec![Value::Bool(true); g.trues]);
        group(g.falses > 0, "boolean", vec![Value::Bool(false); g.falses]);
        group(g.nulls > 0, "null", vec![Value::Null; g.nulls]);
        Ok(field)
    }
}

/// generateVRMLTree( data ).
pub(crate) fn tree(text: &str) -> Result<Tree> {
    if !text.contains("#VRML V2.0") {
        return Err(Error::Asset(
            "THREE.VRMLLexer: Version of VRML asset not supported.".into(),
        ));
    }
    let tokens = lex(text)?;
    let mut p = Parser { tokens, at: 0 };
    p.consume(Kind::Version)?;
    let mut nodes = vec![p.node()?];
    while matches!(p.peek(), Some(Kind::Def | Kind::NodeName)) {
        nodes.push(p.node()?);
    }
    while p.peek() == Some(Kind::Route) {
        p.consume(Kind::Route)?;
        p.consume(Kind::RouteIdentifier)?;
        p.consume(Kind::To)?;
        p.consume(Kind::RouteIdentifier)?;
    }
    if p.at != p.tokens.len() {
        return Err(bad("Parsing errors detected."));
    }
    Ok(Tree { nodes })
}

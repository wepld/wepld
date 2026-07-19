//! Minimal, dependency-free, depth-limited JSON for the S0.5A prototype.
//! EXPERIMENTAL — NEVER MERGE. Not a general-purpose JSON library and
//! not product code. It exists so the prototype has ZERO third-party
//! dependencies and ZERO build scripts/proc-macros — the smallest
//! possible supply-chain surface for an evidence spike. Production
//! WePLD would use an audited serde stack (see DEPENDENCY_EVIDENCE.md).
#![forbid(unsafe_code)]

use std::collections::BTreeMap;

/// Maximum nesting depth accepted by the parser. Untrusted input is
/// bounded in both size (frame ceiling) and depth (here) so a hostile
/// payload cannot exhaust the stack.
pub const MAX_DEPTH: usize = 32;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(BTreeMap<String, Json>),
}

impl Json {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Json::Num(n) if *n >= 0.0 && n.fract() == 0.0 => Some(*n as u64),
            _ => None,
        }
    }
    pub fn get<'a>(&'a self, key: &str) -> Option<&'a Json> {
        match self {
            Json::Obj(m) => m.get(key),
            _ => None,
        }
    }
    pub fn obj_keys(&self) -> Option<Vec<&str>> {
        match self {
            Json::Obj(m) => Some(m.keys().map(String::as_str).collect()),
            _ => None,
        }
    }
}

// ---------- serialization ----------

pub fn to_string(v: &Json) -> String {
    let mut s = String::new();
    write_value(&mut s, v);
    s
}

fn write_value(out: &mut String, v: &Json) {
    match v {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Num(n) => {
            if n.fract() == 0.0 && n.is_finite() {
                out.push_str(&format!("{}", *n as i64));
            } else {
                out.push_str(&format!("{n}"));
            }
        }
        Json::Str(s) => write_string(out, s),
        Json::Arr(a) => {
            out.push('[');
            for (i, e) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(out, e);
            }
            out.push(']');
        }
        Json::Obj(m) => {
            out.push('{');
            for (i, (k, val)) in m.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(out, k);
                out.push(':');
                write_value(out, val);
            }
            out.push('}');
        }
    }
}

fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

// ---------- parsing ----------

pub fn parse(input: &str) -> Result<Json, String> {
    let mut p = Parser { b: input.as_bytes(), i: 0 };
    p.ws();
    let v = p.value(0)?;
    p.ws();
    if p.i != p.b.len() {
        return Err("trailing-bytes".into());
    }
    Ok(v)
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn value(&mut self, depth: usize) -> Result<Json, String> {
        if depth > MAX_DEPTH {
            return Err("max-depth-exceeded".into());
        }
        self.ws();
        match self.peek() {
            Some(b'{') => self.object(depth),
            Some(b'[') => self.array(depth),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') | Some(b'f') => self.boolean(),
            Some(b'n') => self.null(),
            Some(c) if c == b'-' || c.is_ascii_digit() => self.number(),
            _ => Err("unexpected-token".into()),
        }
    }
    fn object(&mut self, depth: usize) -> Result<Json, String> {
        self.i += 1; // {
        let mut m = BTreeMap::new();
        self.ws();
        if self.peek() == Some(b'}') {
            self.i += 1;
            return Ok(Json::Obj(m));
        }
        loop {
            self.ws();
            if self.peek() != Some(b'"') {
                return Err("expected-key".into());
            }
            let key = self.string()?;
            self.ws();
            if self.peek() != Some(b':') {
                return Err("expected-colon".into());
            }
            self.i += 1;
            let val = self.value(depth + 1)?;
            if m.insert(key, val).is_some() {
                return Err("duplicate-key".into());
            }
            self.ws();
            match self.peek() {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Json::Obj(m));
                }
                _ => return Err("expected-comma-or-close".into()),
            }
        }
    }
    fn array(&mut self, depth: usize) -> Result<Json, String> {
        self.i += 1; // [
        let mut a = Vec::new();
        self.ws();
        if self.peek() == Some(b']') {
            self.i += 1;
            return Ok(Json::Arr(a));
        }
        loop {
            let val = self.value(depth + 1)?;
            a.push(val);
            self.ws();
            match self.peek() {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b']') => {
                    self.i += 1;
                    return Ok(Json::Arr(a));
                }
                _ => return Err("expected-comma-or-close".into()),
            }
        }
    }
    fn string(&mut self) -> Result<String, String> {
        self.i += 1; // opening quote
        let mut s = String::new();
        loop {
            let c = self.peek().ok_or("unterminated-string")?;
            self.i += 1;
            match c {
                b'"' => return Ok(s),
                b'\\' => {
                    let e = self.peek().ok_or("unterminated-escape")?;
                    self.i += 1;
                    match e {
                        b'"' => s.push('"'),
                        b'\\' => s.push('\\'),
                        b'/' => s.push('/'),
                        b'n' => s.push('\n'),
                        b'r' => s.push('\r'),
                        b't' => s.push('\t'),
                        b'b' => s.push('\u{08}'),
                        b'f' => s.push('\u{0c}'),
                        b'u' => {
                            let cp = self.hex4()?;
                            // Basic multilingual plane only; surrogate
                            // pairs are rejected rather than mishandled.
                            let ch = char::from_u32(cp).ok_or("bad-unicode-escape")?;
                            s.push(ch);
                        }
                        _ => return Err("bad-escape".into()),
                    }
                }
                c if c < 0x20 => return Err("control-char-in-string".into()),
                c => {
                    // Collect a UTF-8 sequence starting at c.
                    let start = self.i - 1;
                    let extra = utf8_len(c) - 1;
                    if self.i + extra > self.b.len() {
                        return Err("bad-utf8".into());
                    }
                    self.i += extra;
                    let slice = &self.b[start..self.i];
                    s.push_str(std::str::from_utf8(slice).map_err(|_| "bad-utf8")?);
                }
            }
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        if self.i + 4 > self.b.len() {
            return Err("short-unicode-escape".into());
        }
        let hex = std::str::from_utf8(&self.b[self.i..self.i + 4]).map_err(|_| "bad-hex")?;
        let cp = u32::from_str_radix(hex, 16).map_err(|_| "bad-hex")?;
        self.i += 4;
        Ok(cp)
    }
    fn boolean(&mut self) -> Result<Json, String> {
        if self.b[self.i..].starts_with(b"true") {
            self.i += 4;
            Ok(Json::Bool(true))
        } else if self.b[self.i..].starts_with(b"false") {
            self.i += 5;
            Ok(Json::Bool(false))
        } else {
            Err("bad-literal".into())
        }
    }
    fn null(&mut self) -> Result<Json, String> {
        if self.b[self.i..].starts_with(b"null") {
            self.i += 4;
            Ok(Json::Null)
        } else {
            Err("bad-literal".into())
        }
    }
    fn number(&mut self) -> Result<Json, String> {
        let start = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || matches!(c, b'.' | b'e' | b'E' | b'+' | b'-') {
                self.i += 1;
            } else {
                break;
            }
        }
        let s = std::str::from_utf8(&self.b[start..self.i]).map_err(|_| "bad-number")?;
        s.parse::<f64>().map(Json::Num).map_err(|_| "bad-number".into())
    }
}

fn utf8_len(first: u8) -> usize {
    if first < 0x80 {
        1
    } else if first >> 5 == 0b110 {
        2
    } else if first >> 4 == 0b1110 {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_basic() {
        let v = parse(r#"{"a":1,"b":"x","c":[true,null,-2.5]}"#).unwrap();
        assert_eq!(v.get("a").unwrap().as_u64(), Some(1));
        assert_eq!(v.get("b").unwrap().as_str(), Some("x"));
        let s = to_string(&v);
        assert_eq!(parse(&s).unwrap(), v);
    }

    #[test]
    fn rejects_trailing_and_depth() {
        assert!(parse("{} junk").is_err());
        let deep = "[".repeat(40);
        assert!(parse(&deep).is_err());
    }

    #[test]
    fn rejects_duplicate_and_control() {
        assert!(parse(r#"{"a":1,"a":2}"#).is_err());
        assert!(parse("\"bad\u{01}\"").is_err());
    }

    #[test]
    fn escapes() {
        let v = parse(r#""line\nbreakA""#).unwrap();
        assert_eq!(v.as_str(), Some("line\nbreakA"));
    }
}

//! LSP-style framing: `Content-Length: N\r\n\r\n<json>` where the JSON body
//! is a JSON-RPC 2.0 envelope around a contracts `WwpMessage`.

use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use wepld_contracts::wwp::WwpMessage;

/// A single frame body may not exceed this size. Bounds the allocation a
/// misbehaving or malicious worker can force via `Content-Length` (a WWP peer
/// is untrusted — v2-03). Context packs and results are far smaller.
pub const MAX_CONTENT_LEN: usize = 64 * 1024 * 1024;
/// A frame's headers may not exceed this size (bounds a headerless byte
/// stream that would otherwise grow a line buffer without limit).
pub const MAX_HEADER_BYTES: usize = 16 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum WwpError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("frame missing Content-Length header")]
    MissingLength,
    #[error("invalid Content-Length: {0}")]
    BadLength(String),
    #[error("frame Content-Length {0} exceeds maximum {MAX_CONTENT_LEN}")]
    ContentTooLarge(usize),
    #[error("frame headers exceed maximum {MAX_HEADER_BYTES} bytes")]
    HeaderTooLarge,
    #[error("malformed message: {0}")]
    Malformed(#[from] serde_json::Error),
}

/// JSON-RPC 2.0 envelope. `id` is reserved for request/response pairs
/// (first used by `brain.request` on Day 6); notifications omit it.
#[derive(Debug, Serialize, Deserialize)]
pub struct FrameMsg {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(flatten)]
    pub msg: WwpMessage,
}

impl FrameMsg {
    pub fn notification(msg: WwpMessage) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id: None,
            msg,
        }
    }

    pub fn request(id: u64, msg: WwpMessage) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id: Some(id),
            msg,
        }
    }
}

/// JSON-RPC 2.0 response (Core → worker; first used for `brain.request`).
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMsg {
    pub jsonrpc: String,
    pub id: u64,
    pub result: serde_json::Value,
}

/// Anything a WWP peer may receive.
#[derive(Debug)]
pub enum Incoming {
    Message(FrameMsg),
    Response(ResponseMsg),
}

pub fn write_frame<W: Write>(w: &mut W, msg: &FrameMsg) -> Result<(), WwpError> {
    let body = serde_json::to_vec(msg)?;
    write!(w, "Content-Length: {}\r\n\r\n", body.len())?;
    w.write_all(&body)?;
    w.flush()?;
    Ok(())
}

pub fn write_response<W: Write>(w: &mut W, resp: &ResponseMsg) -> Result<(), WwpError> {
    let body = serde_json::to_vec(resp)?;
    write!(w, "Content-Length: {}\r\n\r\n", body.len())?;
    w.write_all(&body)?;
    w.flush()?;
    Ok(())
}

/// Read one frame of either kind. `Ok(None)` = clean EOF.
pub fn read_incoming<R: BufRead>(r: &mut R) -> Result<Option<Incoming>, WwpError> {
    let Some(raw) = read_raw(r)? else {
        return Ok(None);
    };
    let value: serde_json::Value = serde_json::from_slice(&raw)?;
    if value.get("method").is_some() {
        Ok(Some(Incoming::Message(serde_json::from_value(value)?)))
    } else if value.get("result").is_some() {
        Ok(Some(Incoming::Response(serde_json::from_value(value)?)))
    } else {
        Err(WwpError::Malformed(serde::de::Error::custom(
            "frame is neither request nor response",
        )))
    }
}

/// Read one request/notification frame. `Ok(None)` = clean EOF (peer closed
/// the pipe).
pub fn read_frame<R: BufRead>(r: &mut R) -> Result<Option<FrameMsg>, WwpError> {
    match read_raw(r)? {
        None => Ok(None),
        Some(buf) => Ok(Some(serde_json::from_slice(&buf)?)),
    }
}

fn read_raw<R: BufRead>(r: &mut R) -> Result<Option<Vec<u8>>, WwpError> {
    let mut len: Option<usize> = None;
    let mut header_budget = MAX_HEADER_BYTES;
    loop {
        let Some(line) = read_header_line(r, &mut header_budget)? else {
            // Clean EOF only if it happened before any header byte.
            return if header_budget == MAX_HEADER_BYTES {
                Ok(None)
            } else {
                Err(WwpError::Io(std::io::Error::from(
                    std::io::ErrorKind::UnexpectedEof,
                )))
            };
        };
        let t = line.trim();
        if t.is_empty() {
            break;
        }
        if let Some(v) = t.strip_prefix("Content-Length:") {
            len = Some(
                v.trim()
                    .parse()
                    .map_err(|_| WwpError::BadLength(v.trim().to_owned()))?,
            );
        }
    }
    let len = len.ok_or(WwpError::MissingLength)?;
    if len > MAX_CONTENT_LEN {
        return Err(WwpError::ContentTooLarge(len));
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)?;
    Ok(Some(buf))
}

/// Read one header line (up to the `\n`) without allocating beyond the shared
/// header budget. Returns `None` on EOF with no bytes read.
fn read_header_line<R: BufRead>(r: &mut R, budget: &mut usize) -> Result<Option<String>, WwpError> {
    let mut line: Vec<u8> = Vec::new();
    let mut saw_any = false;
    loop {
        let mut byte = [0u8; 1];
        let n = r.read(&mut byte)?;
        if n == 0 {
            return if saw_any {
                Ok(Some(String::from_utf8_lossy(&line).into_owned()))
            } else {
                Ok(None)
            };
        }
        saw_any = true;
        if *budget == 0 {
            return Err(WwpError::HeaderTooLarge);
        }
        *budget -= 1;
        if byte[0] == b'\n' {
            return Ok(Some(String::from_utf8_lossy(&line).into_owned()));
        }
        line.push(byte[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use wepld_contracts::wwp::{Heartbeat, WwpMessage};

    fn hb(progress: &str) -> FrameMsg {
        FrameMsg::notification(WwpMessage::Heartbeat(Heartbeat {
            attempt_id: "att_1".into(),
            progress: progress.into(),
        }))
    }

    #[test]
    fn roundtrip_single_and_multiple_frames() {
        let mut buf = Vec::new();
        write_frame(&mut buf, &hb("one")).unwrap();
        write_frame(&mut buf, &hb("two")).unwrap();

        let mut r = Cursor::new(buf);
        let a = read_frame(&mut r).unwrap().unwrap();
        let b = read_frame(&mut r).unwrap().unwrap();
        assert!(read_frame(&mut r).unwrap().is_none(), "then clean EOF");
        match (a.msg, b.msg) {
            (WwpMessage::Heartbeat(x), WwpMessage::Heartbeat(y)) => {
                assert_eq!(x.progress, "one");
                assert_eq!(y.progress, "two");
            }
            other => panic!("wrong messages: {other:?}"),
        }
    }

    #[test]
    fn missing_length_and_bad_json_are_errors() {
        let mut r = Cursor::new(b"X-Header: 1\r\n\r\n".to_vec());
        assert!(matches!(read_frame(&mut r), Err(WwpError::MissingLength)));

        let mut buf = b"Content-Length: 9\r\n\r\nnot-json!".to_vec();
        let mut r = Cursor::new(&mut buf);
        assert!(matches!(read_frame(&mut r), Err(WwpError::Malformed(_))));
    }

    #[test]
    fn huge_content_length_is_refused_without_allocating() {
        // A malicious worker claims a terabyte body. Must error, not OOM.
        let mut r = Cursor::new(b"Content-Length: 1099511627776\r\n\r\n".to_vec());
        assert!(matches!(
            read_frame(&mut r),
            Err(WwpError::ContentTooLarge(1099511627776))
        ));
    }

    #[test]
    fn headerless_byte_stream_is_bounded() {
        // No newline ever: the header budget stops it instead of growing forever.
        let flood = vec![b'A'; MAX_HEADER_BYTES + 1024];
        let mut r = Cursor::new(flood);
        assert!(matches!(read_frame(&mut r), Err(WwpError::HeaderTooLarge)));
    }

    #[test]
    fn truncated_body_is_an_error_not_a_hang() {
        // Content-Length promises 100 bytes; only 3 arrive, then EOF.
        let mut r = Cursor::new(b"Content-Length: 100\r\n\r\nabc".to_vec());
        assert!(matches!(read_frame(&mut r), Err(WwpError::Io(_))));
    }

    #[test]
    fn max_valid_frame_still_parses() {
        // A large-but-legal body round-trips.
        let mut buf = Vec::new();
        write_frame(&mut buf, &hb(&"x".repeat(100_000))).unwrap();
        let mut r = Cursor::new(buf);
        assert!(read_frame(&mut r).unwrap().is_some());
    }
}

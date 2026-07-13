//! LSP-style framing: `Content-Length: N\r\n\r\n<json>` where the JSON body
//! is a JSON-RPC 2.0 envelope around a contracts `WwpMessage`.

use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use wepld_contracts::wwp::WwpMessage;

#[derive(Debug, thiserror::Error)]
pub enum WwpError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("frame missing Content-Length header")]
    MissingLength,
    #[error("invalid Content-Length: {0}")]
    BadLength(String),
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
}

pub fn write_frame<W: Write>(w: &mut W, msg: &FrameMsg) -> Result<(), WwpError> {
    let body = serde_json::to_vec(msg)?;
    write!(w, "Content-Length: {}\r\n\r\n", body.len())?;
    w.write_all(&body)?;
    w.flush()?;
    Ok(())
}

/// Read one frame. `Ok(None)` = clean EOF (peer closed the pipe).
pub fn read_frame<R: BufRead>(r: &mut R) -> Result<Option<FrameMsg>, WwpError> {
    let mut line = String::new();
    let mut len: Option<usize> = None;
    loop {
        line.clear();
        if r.read_line(&mut line)? == 0 {
            return Ok(None);
        }
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
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)?;
    Ok(Some(serde_json::from_slice(&buf)?))
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
}

//! Worker side: read frames from stdin, send frames to stdout. Each frame is
//! written under one stdout lock acquisition, so concurrent threads (main +
//! heartbeat) can never interleave partial frames.

use crate::frame::{read_incoming, write_frame, FrameMsg, Incoming, WwpError};
use std::io::StdinLock;
use wepld_contracts::wwp::WwpMessage;

pub fn send_to_core(msg: WwpMessage) -> Result<(), WwpError> {
    let mut out = std::io::stdout().lock();
    write_frame(&mut out, &FrameMsg::notification(msg))
}

/// Send a request that expects a response (e.g. `brain.request`). The caller
/// correlates the reply by `id` via [`worker_read_incoming`].
pub fn send_request_to_core(msg: WwpMessage, id: u64) -> Result<(), WwpError> {
    let mut out = std::io::stdout().lock();
    write_frame(&mut out, &FrameMsg::request(id, msg))
}

pub fn worker_read_incoming(stdin: &mut StdinLock<'static>) -> Result<Option<Incoming>, WwpError> {
    read_incoming(stdin)
}

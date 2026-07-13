//! Worker side: read frames from stdin, send frames to stdout. Each frame is
//! written under one stdout lock acquisition, so concurrent threads (main +
//! heartbeat) can never interleave partial frames.

use crate::frame::{read_frame, write_frame, FrameMsg, WwpError};
use std::io::StdinLock;
use wepld_contracts::wwp::WwpMessage;

pub fn send_to_core(msg: WwpMessage) -> Result<(), WwpError> {
    let mut out = std::io::stdout().lock();
    write_frame(&mut out, &FrameMsg::notification(msg))
}

pub fn worker_read_frame(stdin: &mut StdinLock<'static>) -> Result<Option<FrameMsg>, WwpError> {
    read_frame(stdin)
}

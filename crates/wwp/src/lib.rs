//! wepld-wwp — the WePLD Worker Protocol (WWP v0).
//!
//! The worker boundary is a protocol, not a function call (ADR-0005).
//! Message *shapes* live in `wepld-contracts::wwp`; this crate owns framing
//! (JSON-RPC 2.0 envelope, LSP-style Content-Length frames over stdio), the
//! host side (spawn, event stream, heartbeat watchdog, cancel/kill), and the
//! worker side (read loop + frame sender).
//!
//! Protocol invariants enforced here by construction: no message addresses a
//! human; no message mutates state — workers report, the Runtime decides.

mod frame;
mod host;
mod worker;

pub use frame::{
    read_frame, read_incoming, write_frame, write_response, FrameMsg, Incoming, ResponseMsg,
    WwpError,
};
pub use host::{spawn_worker, WorkerEvent, WorkerHandle};
pub use worker::{send_request_to_core, send_to_core, worker_read_incoming};

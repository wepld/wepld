//! Host (Core) side: spawn a worker process, deliver `attempt.start`, stream
//! its messages as events, watch heartbeats, cancel or kill.

use crate::frame::{
    read_incoming, write_frame, write_response, FrameMsg, Incoming, ResponseMsg, WwpError,
};
use std::io::BufReader;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use wepld_contracts::wwp::{AttemptCancel, AttemptStart, WwpMessage};

#[derive(Debug)]
pub enum WorkerEvent {
    /// A message from the worker (any message counts as liveness). Carries
    /// the JSON-RPC id so the Runtime can respond to requests.
    Message(FrameMsg),
    /// The worker's stdout closed (process ending). The Runtime must probe
    /// exit status and decide — EOF is never assumed to be success.
    Eof,
    /// No message within the heartbeat timeout. The Runtime decides.
    HeartbeatTimeout,
    /// The worker violated the protocol.
    Malformed(String),
}

pub struct WorkerHandle {
    child: Child,
    stdin: ChildStdin,
    pub events: Receiver<WorkerEvent>,
    stopped: Arc<AtomicBool>,
}

/// Spawn a WWP worker, send `attempt.start`, start reader + watchdog threads.
pub fn spawn_worker(
    cmd: &[String],
    start: &AttemptStart,
    heartbeat_timeout: Duration,
) -> Result<WorkerHandle, WwpError> {
    let mut child = Command::new(&cmd[0])
        .args(&cmd[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let mut stdin = child.stdin.take().expect("piped stdin");
    let stdout = child.stdout.take().expect("piped stdout");

    write_frame(
        &mut stdin,
        &FrameMsg::notification(WwpMessage::AttemptStart(Box::new(start.clone()))),
    )?;

    let (tx, rx) = std::sync::mpsc::channel::<WorkerEvent>();
    let last_seen = Arc::new(Mutex::new(Instant::now()));
    let stopped = Arc::new(AtomicBool::new(false));

    spawn_reader(stdout, tx.clone(), last_seen.clone(), stopped.clone());
    spawn_watchdog(tx, last_seen, stopped.clone(), heartbeat_timeout);

    Ok(WorkerHandle {
        child,
        stdin,
        events: rx,
        stopped,
    })
}

impl WorkerHandle {
    /// Cooperative cancellation. The Runtime kills after a grace period
    /// regardless — cancel is a courtesy, not a dependency.
    pub fn cancel(&mut self, attempt_id: &str) -> Result<(), WwpError> {
        write_frame(
            &mut self.stdin,
            &FrameMsg::notification(WwpMessage::AttemptCancel(AttemptCancel {
                attempt_id: attempt_id.to_owned(),
            })),
        )
    }

    /// Answer a worker request (e.g. a `BrainResult` for `brain.request`).
    pub fn respond(&mut self, id: u64, result: serde_json::Value) -> Result<(), WwpError> {
        write_response(
            &mut self.stdin,
            &ResponseMsg {
                jsonrpc: "2.0".to_owned(),
                id,
                result,
            },
        )
    }

    pub fn kill(&mut self) {
        self.stopped.store(true, Ordering::Relaxed);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }

    /// Reap the process and return its exit code (None = killed by signal).
    pub fn wait_exit(&mut self) -> std::io::Result<Option<i32>> {
        self.stopped.store(true, Ordering::Relaxed);
        Ok(self.child.wait()?.code())
    }
}

impl Drop for WorkerHandle {
    fn drop(&mut self) {
        self.stopped.store(true, Ordering::Relaxed);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn spawn_reader(
    stdout: std::process::ChildStdout,
    tx: Sender<WorkerEvent>,
    last_seen: Arc<Mutex<Instant>>,
    stopped: Arc<AtomicBool>,
) {
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            match read_incoming(&mut reader) {
                Ok(Some(Incoming::Message(frame))) => {
                    *last_seen.lock().expect("liveness lock") = Instant::now();
                    if tx.send(WorkerEvent::Message(frame)).is_err() {
                        break;
                    }
                }
                Ok(Some(Incoming::Response(_))) => {
                    stopped.store(true, Ordering::Relaxed);
                    let _ = tx.send(WorkerEvent::Malformed(
                        "worker sent a response frame".to_owned(),
                    ));
                    break;
                }
                Ok(None) => {
                    stopped.store(true, Ordering::Relaxed);
                    let _ = tx.send(WorkerEvent::Eof);
                    break;
                }
                Err(e) => {
                    stopped.store(true, Ordering::Relaxed);
                    let _ = tx.send(WorkerEvent::Malformed(e.to_string()));
                    break;
                }
            }
        }
    });
}

fn spawn_watchdog(
    tx: Sender<WorkerEvent>,
    last_seen: Arc<Mutex<Instant>>,
    stopped: Arc<AtomicBool>,
    timeout: Duration,
) {
    std::thread::spawn(move || {
        let tick = (timeout / 4).max(Duration::from_millis(25));
        loop {
            std::thread::sleep(tick);
            if stopped.load(Ordering::Relaxed) {
                break;
            }
            let elapsed = last_seen.lock().expect("liveness lock").elapsed();
            if elapsed > timeout {
                let _ = tx.send(WorkerEvent::HeartbeatTimeout);
                break;
            }
        }
    });
}

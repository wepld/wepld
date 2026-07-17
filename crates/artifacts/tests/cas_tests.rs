use wepld_artifacts::{Cas, CasError};

fn open() -> (tempfile::TempDir, Cas) {
    let dir = tempfile::tempdir().unwrap();
    let cas = Cas::open(dir.path()).unwrap();
    (dir, cas)
}

#[test]
fn roundtrip_and_dedup() {
    let (_dir, cas) = open();
    let a = cas.put(b"evidence body").unwrap();
    assert!(a.newly_written);
    let b = cas.put(b"evidence body").unwrap();
    assert!(!b.newly_written, "identical content must deduplicate");
    assert_eq!(a.hash, b.hash);
    assert_eq!(cas.get(&a.hash).unwrap(), b"evidence body");
    assert!(cas.verify(&a.hash).unwrap());
}

#[test]
fn corruption_is_detected_on_read() {
    let (dir, cas) = open();
    let r = cas.put(b"important test log").unwrap();
    // Corrupt the body out-of-band.
    let path = dir
        .path()
        .join("objects")
        .join(&r.hash[..2])
        .join(&r.hash[2..]);
    std::fs::write(&path, b"forged content").unwrap();

    assert!(matches!(cas.get(&r.hash), Err(CasError::Corrupt { .. })));
    assert!(!cas.verify(&r.hash).unwrap());
}

#[test]
fn tombstone_removes_body_and_preserves_reason() {
    let (_dir, cas) = open();
    let r = cas.put(b"classified diff").unwrap();
    cas.tombstone(&r.hash, "retention: user deletion request")
        .unwrap();

    match cas.get(&r.hash) {
        Err(CasError::Tombstoned { reason, .. }) => {
            assert!(reason.contains("deletion request"));
        }
        other => panic!("expected tombstoned, got {other:?}"),
    }
    assert_eq!(
        cas.tombstone_reason(&r.hash).unwrap().unwrap(),
        "retention: user deletion request"
    );
}

#[test]
fn missing_and_invalid_hashes_error_cleanly() {
    let (_dir, cas) = open();
    let missing = "a".repeat(64);
    assert!(matches!(cas.get(&missing), Err(CasError::NotFound(_))));
    assert!(matches!(
        cas.get("not-a-hash"),
        Err(CasError::InvalidHash(_))
    ));
    assert!(matches!(
        cas.tombstone(&missing, "x"),
        Err(CasError::NotFound(_))
    ));
}

// EXPERIMENTAL — NEVER MERGE. Real-core bridge-contract test: spawns the
// ACTUAL Rust core, speaks the real framed-stdio protocol, and feeds the
// REAL success/denial/error envelopes through the SAME bridge normalizer
// the UI uses — proving real core output becomes the canonical frontend
// contract (everything except the WebView/Tauri invoke layer). Node
// built-ins only; NO new dependency. SKIPS unless S05A_CORE_BIN is set.
import { test } from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { mkdtempSync, writeFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { normalizeBridgeResponse, formatBridgeResult } from "../test-build/bridge.js";

const CORE = process.env.S05A_CORE_BIN;
const skip = CORE && existsSync(CORE) ? false : "S05A_CORE_BIN not set";

class FramedClient {
  constructor(bin, fixtures, output) {
    this.buf = Buffer.alloc(0);
    this.waiters = [];
    this.session = "";
    this.child = spawn(bin, ["--fixtures", fixtures, "--output", output], {
      stdio: ["pipe", "pipe", "ignore"],
    });
    this.child.stdout.on("data", (d) => {
      this.buf = Buffer.concat([this.buf, d]);
      while (this.buf.length >= 4) {
        const n = this.buf.readUInt32BE(0);
        if (this.buf.length < 4 + n) break;
        const body = this.buf.subarray(4, 4 + n).toString("utf8");
        this.buf = this.buf.subarray(4 + n);
        const w = this.waiters.shift();
        if (w) w(body);
      }
    });
  }
  recv() {
    return new Promise((res) => this.waiters.push(res));
  }
  send(obj) {
    const b = Buffer.from(JSON.stringify(obj), "utf8");
    const p = Buffer.alloc(4);
    p.writeUInt32BE(b.length, 0);
    this.child.stdin.write(Buffer.concat([p, b]));
  }
  async start() {
    const hello = JSON.parse(await this.recv());
    this.session = hello.session;
  }
  close() {
    this.child.stdin.end();
    this.child.kill();
  }
}

test("real core envelopes become the canonical bridge contract", { skip }, async () => {
  const fixtures = mkdtempSync(join(tmpdir(), "s05a-fx-"));
  const output = mkdtempSync(join(tmpdir(), "s05a-out-"));
  writeFileSync(join(fixtures, "hello.txt"), "S0.5A fixture: the only file the read capability may reach.\n");

  const c = new FramedClient(CORE, fixtures, output);
  await c.start();
  assert.ok(c.session.length > 0, "handshake session");

  const cases = [
    { label: "Core health", op: "health", cap: null, params: {}, expect: "ok" },
    { label: "Read scoped fixture (allowed)", op: "read_fixture", cap: "cap-read-fixture-001", params: { path: "hello.txt" }, expect: "ok" },
    { label: "Read traversal (must deny)", op: "read_fixture", cap: "cap-read-fixture-001", params: { path: "../../secret.txt" }, expect: "denied" },
    { label: "Write scoped output (allowed)", op: "write_output", cap: "cap-write-output-001", params: { path: "run/out.txt", content: "ok" }, expect: "ok" },
    { label: "Write to .git (must deny)", op: "write_output", cap: "cap-write-output-001", params: { path: ".git/config", content: "x" }, expect: "denied" },
    { label: "Unknown operation (must deny)", op: "shell_exec", cap: null, params: {}, expect: "denied" },
  ];

  let id = 1;
  for (const tc of cases) {
    c.send({ v: "s05a/1", id: id++, session: c.session, capability: tc.cap, op: tc.op, params: tc.params });
    const raw = await c.recv();
    const r = normalizeBridgeResponse(raw);
    assert.equal(r.status, tc.expect, `${tc.label}: expected ${tc.expect}, got ${r.status} (${r.code}) from ${raw}`);
    const line = formatBridgeResult(tc.label, r);
    for (const bad of ["undefined", "null", "[object Object]"]) {
      assert.ok(!line.includes(bad), `${tc.label}: line contains ${bad}: ${line}`);
    }
  }

  // Stale session must be denied by the real core (session semantics
  // unchanged by this fix).
  c.send({ v: "s05a/1", id: id++, session: "forged-0000", op: "health", params: {} });
  const stale = normalizeBridgeResponse(await c.recv());
  assert.equal(stale.status, "denied");
  assert.equal(stale.code, "stale-or-unknown-session");

  c.close();
});

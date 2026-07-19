// EXPERIMENTAL — NEVER MERGE. Deterministic unit tests for the bridge
// response contract. Node built-in test runner (node:test) + node:assert
// — NO new dependency. Imports the compiled pure module. Run: `npm test`.
import { test } from "node:test";
import assert from "node:assert/strict";
import {
  normalizeBridgeResponse,
  bridgeErrorFromRejection,
  formatBridgeResult,
} from "../test-build/bridge.js";

const okString = JSON.stringify({ id: 1, kind: "Ok", result: { protocol: "s05a/1", session: "sX" } });
const deniedString = JSON.stringify({ id: 2, kind: "Denied", reason: "traversal-or-prefix-rejected", resource: "../x" });
const errorString = JSON.stringify({ id: 3, kind: "Error", reason: "malformed-params" });

function assertNoBadTokens(s) {
  for (const bad of ["undefined", "null", "[object Object]"]) {
    assert.ok(!s.includes(bad), `result must not contain ${bad}: ${s}`);
  }
}

// 1. structured success (string form, as the real host returns)
test("1 success normalizes to ok", () => {
  const r = normalizeBridgeResponse(okString);
  assert.equal(r.status, "ok");
  assert.equal(r.code, "ok");
  assert.equal(r.sessionId, "sX");
  assert.equal(r.requestId, "1");
});
test("1b success as object normalizes to ok", () => {
  assert.equal(normalizeBridgeResponse({ id: 9, kind: "Ok", result: { bytes: 60 } }).status, "ok");
});

// 2. structured policy denial
test("2 denial normalizes to denied with reason as code", () => {
  const r = normalizeBridgeResponse(deniedString);
  assert.equal(r.status, "denied");
  assert.equal(r.code, "traversal-or-prefix-rejected");
});

// 3. structured core/bridge error
test("3 error normalizes to error", () => {
  const r = normalizeBridgeResponse(errorString);
  assert.equal(r.status, "error");
  assert.equal(r.code, "malformed-params");
});

// 4. malformed response (not valid JSON)
test("4 malformed JSON -> contract error", () => {
  const r = normalizeBridgeResponse("{ this is not json");
  assert.equal(r.status, "error");
  assert.equal(r.code, "response-contract-invalid");
});

// 5. missing required field
test("5a Ok without result -> contract error", () => {
  assert.equal(normalizeBridgeResponse(JSON.stringify({ id: 1, kind: "Ok" })).code, "response-contract-invalid");
});
test("5b Denied without reason -> contract error", () => {
  assert.equal(normalizeBridgeResponse(JSON.stringify({ id: 1, kind: "Denied" })).code, "response-contract-invalid");
});

// 6. unsupported kind
test("6 unsupported kind -> contract error", () => {
  const r = normalizeBridgeResponse(JSON.stringify({ id: 1, kind: "Weird", reason: "x" }));
  assert.equal(r.status, "error");
  assert.equal(r.code, "response-contract-invalid");
});

// 7. request-id correlation
test("7 request-id mismatch -> error", () => {
  const r = normalizeBridgeResponse(okString, "999");
  assert.equal(r.status, "error");
  assert.equal(r.code, "request-id-mismatch");
});
test("7b request-id match -> ok", () => {
  assert.equal(normalizeBridgeResponse(okString, "1").status, "ok");
});

// 8. empty reason -> contract error (never blank render)
test("8 empty reason -> contract error", () => {
  const r = normalizeBridgeResponse(JSON.stringify({ id: 1, kind: "Denied", reason: "" }));
  assert.equal(r.code, "response-contract-invalid");
  assert.ok(r.message.length > 0);
});

// 9. raw JSON string where an object is expected (THE fix)
test("9 raw JSON string is parsed once and normalized", () => {
  assert.equal(normalizeBridgeResponse(okString).status, "ok");
});

// 10. double-encoded JSON -> error (must NOT be parsed twice)
test("10 double-encoded JSON -> contract error", () => {
  const r = normalizeBridgeResponse(JSON.stringify(okString));
  assert.equal(r.status, "error");
  assert.equal(r.code, "response-contract-invalid");
});

// 11. invoke rejection mapping
test("11 invoke rejection -> bridge error", () => {
  const r = bridgeErrorFromRejection(new Error("core-unavailable"));
  assert.equal(r.status, "error");
  assert.equal(r.code, "bridge-invoke-rejected");
  assert.ok(r.message.includes("core-unavailable"));
});
test("11b non-object primitive -> contract error", () => {
  assert.equal(normalizeBridgeResponse(42).code, "response-contract-invalid");
  assert.equal(normalizeBridgeResponse(undefined).code, "response-contract-invalid");
});

// 12/13/15: six REAL captured core envelopes -> non-empty, classified,
// sanitized, no bad tokens.
const REAL = [
  { label: "Core health", raw: '{"id":1,"kind":"Ok","result":{"capability_engine":"static-prototype-table/2-entries","core_build":"s05a-core 0.0.1 (prototype)","protocol":"s05a/1","session":"s5a84-18c3c6070f46fed4"}}', expect: "ok" },
  { label: "Read scoped fixture (allowed)", raw: '{"id":2,"kind":"Ok","result":{"bytes":60,"content":"S0.5A fixture: the only file the read capability may reach.\\n","path":"hello.txt"}}', expect: "ok" },
  { label: "Read traversal (must deny)", raw: '{"capability":"cap-read-fixture-001","id":3,"kind":"Denied","reason":"traversal-or-prefix-rejected","resource":"../../secret.txt"}', expect: "denied" },
  { label: "Write scoped output (allowed)", raw: '{"id":4,"kind":"Ok","result":{"bytes":2,"path":"run/out.txt"}}', expect: "ok" },
  { label: "Write to .git (must deny)", raw: '{"capability":"cap-write-output-001","id":5,"kind":"Denied","reason":"git-metadata-access-rejected","resource":".git/config"}', expect: "denied" },
  { label: "Unknown operation (must deny)", raw: '{"capability":null,"id":6,"kind":"Denied","reason":"unknown-operation:shell_exec","resource":null}', expect: "denied" },
];
test("12/13/15 all six real envelopes render explicit, classified, sanitized", () => {
  for (const { label, raw, expect } of REAL) {
    const r = normalizeBridgeResponse(raw);
    assert.equal(r.status, expect, `${label} status`);
    const line = formatBridgeResult(label, r);
    assert.ok(line.length > label.length + 3, `${label} non-empty`);
    assertNoBadTokens(line);
    assert.ok(!line.includes("S0.5A fixture:"), `${label} must not echo file content`);
  }
});

// 14. classification words in the rendered line
test("14 classification words in the rendered line", () => {
  assert.match(formatBridgeResult("x", normalizeBridgeResponse(okString)), /: OK — /);
  assert.match(formatBridgeResult("x", normalizeBridgeResponse(deniedString)), /: DENIED — /);
  assert.match(formatBridgeResult("x", normalizeBridgeResponse("nope")), /: Bridge error — /);
});

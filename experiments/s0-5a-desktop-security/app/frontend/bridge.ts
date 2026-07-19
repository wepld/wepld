// EXPERIMENTAL — NEVER MERGE. Canonical bridge response contract for the
// S0.5A prototype UI. This is the single documented shape the UI renders.
//
// Why this exists: the Tauri host command `core_request` returns
// Result<String, String> — i.e. a JSON *string* (the core's response
// envelope). The pre-fix UI called invoke<CoreResponse>(...) and read
// `.kind` off that value; because the runtime value is a string, `.kind`
// was `undefined`, so every operation rendered "undefined"
// (S05A-RUNTIME-001). The fix: parse the string EXACTLY ONCE, validate
// it at runtime, and normalize the core envelope into this canonical
// typed contract before it ever reaches UI state. No core/host security
// semantics are changed by this file.
//
// Pure module: zero imports, fully unit-testable under `node --test`.

export type BridgeStatus = "ok" | "denied" | "error";

export interface BridgeResponse {
  requestId: string;
  status: BridgeStatus;
  code: string;
  message: string;
  data?: Record<string, unknown>;
  sessionId?: string;
}

const CONTRACT_INVALID = "response-contract-invalid";

function bridgeError(requestId: string, code: string, message: string): BridgeResponse {
  return { requestId, status: "error", code, message };
}

/**
 * Normalize a raw `invoke()` result into the canonical BridgeResponse.
 * - Parses a JSON string EXACTLY ONCE (never twice; a value that is
 *   still a string after one parse is a double-encoded contract error).
 * - Never throws; never returns undefined/null fields.
 * - `expectedRequestId`, when provided, enforces request/response
 *   correlation.
 */
export function normalizeBridgeResponse(
  raw: unknown,
  expectedRequestId?: string,
): BridgeResponse {
  const fallbackId = expectedRequestId ?? "unknown";

  let obj: unknown = raw;
  if (typeof raw === "string") {
    try {
      obj = JSON.parse(raw);
    } catch {
      return bridgeError(fallbackId, CONTRACT_INVALID, "core response was not valid JSON");
    }
  }

  // After at most one parse we must hold a plain object envelope. A
  // remaining string means the payload was double-encoded — we do NOT
  // parse again; we classify it as a contract error.
  if (obj === null || typeof obj !== "object" || Array.isArray(obj)) {
    return bridgeError(fallbackId, CONTRACT_INVALID, "core response was not a structured object");
  }

  const env = obj as Record<string, unknown>;
  const id = env["id"];
  const requestId =
    typeof id === "number" && Number.isFinite(id)
      ? String(id)
      : typeof id === "string" && id.length > 0
        ? id
        : fallbackId;

  if (expectedRequestId !== undefined && requestId !== expectedRequestId) {
    return bridgeError(requestId, "request-id-mismatch", "core response id did not match the request");
  }

  const kind = env["kind"];

  if (kind === "Ok") {
    const result = env["result"];
    if (result === null || typeof result !== "object" || Array.isArray(result)) {
      return bridgeError(requestId, CONTRACT_INVALID, "success response missing structured result");
    }
    const data = result as Record<string, unknown>;
    const session = data["session"];
    const base: BridgeResponse = {
      requestId,
      status: "ok",
      code: "ok",
      message: "operation completed within capability scope",
      data,
    };
    return typeof session === "string" && session.length > 0
      ? { ...base, sessionId: session }
      : base;
  }

  if (kind === "Denied" || kind === "Error") {
    const reason = env["reason"];
    if (typeof reason !== "string" || reason.length === 0) {
      return bridgeError(
        requestId,
        CONTRACT_INVALID,
        kind === "Denied" ? "denial missing reason" : "error missing reason",
      );
    }
    return kind === "Denied"
      ? { requestId, status: "denied", code: reason, message: reason }
      : { requestId, status: "error", code: reason, message: reason };
  }

  // "Hello" (handshake) or any other/unsupported kind is not a valid
  // response to an operation request.
  return bridgeError(requestId, CONTRACT_INVALID, "unexpected or unsupported response kind");
}

/**
 * Map a rejected invoke() (host-side guard Err, or the core becoming
 * unavailable) to a canonical bridge error — never a success or a
 * security denial. Sanitized and length-bounded.
 */
export function bridgeErrorFromRejection(e: unknown): BridgeResponse {
  const s = typeof e === "string" ? e : e instanceof Error ? e.message : "bridge error";
  const message = s.length > 120 ? `${s.slice(0, 117)}...` : s;
  return { requestId: "unknown", status: "error", code: "bridge-invoke-rejected", message };
}

/**
 * Deterministic, sanitized, human-readable single-line result for the
 * status region. Never yields undefined / null / [object Object].
 */
export function formatBridgeResult(label: string, r: BridgeResponse): string {
  switch (r.status) {
    case "ok":
      return `${label}: OK — ${okDetail(r)}`;
    case "denied":
      return `${label}: DENIED — ${r.code}`;
    case "error":
      return `${label}: Bridge error — ${r.code}`;
  }
}

function okDetail(r: BridgeResponse): string {
  const d = r.data ?? {};
  const protocol = d["protocol"];
  if (typeof protocol === "string") {
    return `protocol ${protocol}, session established`;
  }
  const bytes = d["bytes"];
  if (typeof bytes === "number") {
    return `completed within capability scope (${bytes} bytes)`;
  }
  return "completed within capability scope";
}

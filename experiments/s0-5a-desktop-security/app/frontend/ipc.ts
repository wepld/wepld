// EXPERIMENTAL — NEVER MERGE. Typed wrapper over the ONE Tauri command
// the host exposes. The UI cannot spawn processes, touch the filesystem,
// open sockets, or read secrets; it can only ask the host to forward a
// typed request to the separate Rust core. The host owns the core.
//
// The host command `core_request` returns a JSON *string* (the core
// envelope). We therefore treat invoke()'s result as `unknown`, hand it
// to the bridge normalizer (which parses exactly once and validates),
// and only ever expose the canonical typed BridgeResponse to the UI.
import { invoke } from "@tauri-apps/api/core";
import { normalizeBridgeResponse, bridgeErrorFromRejection, type BridgeResponse } from "./bridge.ts";

export type { BridgeResponse } from "./bridge.ts";

export async function coreRequest(
  op: string,
  capability: string | null,
  paramsJson: string,
): Promise<BridgeResponse> {
  try {
    const raw = await invoke<unknown>("core_request", { op, capability, paramsJson });
    return normalizeBridgeResponse(raw);
  } catch (e) {
    // A rejected invoke() means the host bridge itself failed (host-side
    // guard Err, or the core became unavailable) — classify as a bridge
    // error, never as a success or a security denial.
    return bridgeErrorFromRejection(e);
  }
}

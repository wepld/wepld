// EXPERIMENTAL — NEVER MERGE. Typed wrapper over the ONE Tauri command
// the host exposes. The UI cannot spawn processes, touch the
// filesystem, open sockets, or read secrets; it can only ask the host
// to forward a typed request to the separate Rust core. The host, not
// this code, owns the core process.
import { invoke } from "@tauri-apps/api/core";

export interface CoreResponse {
  kind: "Ok" | "Denied" | "Error";
  id?: number;
  result?: unknown;
  reason?: string;
  capability?: string | null;
  resource?: string | null;
}

// The single exposed command. `paramsJson` is a JSON string validated
// and size-bounded by the Rust core; the UI never constructs privileged
// arguments beyond these typed fields.
export async function coreRequest(
  op: string,
  capability: string | null,
  paramsJson: string,
): Promise<CoreResponse> {
  return invoke<CoreResponse>("core_request", { op, capability, paramsJson });
}

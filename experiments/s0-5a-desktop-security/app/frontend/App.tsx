// EXPERIMENTAL — NEVER MERGE. Untrusted presentation layer with ZERO
// authority: status display, buttons for the authorized prototype
// operations, capability-decision explanations, a bench trigger, and
// failure/restart status. Every effect is delegated to the Rust core
// through the single host command. Accessibility is a first-class
// requirement: semantic controls, labels, a live region, keyboard
// operation, and an RTL toggle for Arabic smoke testing.
import { useCallback, useState } from "react";
import { coreRequest, type BridgeResponse } from "./ipc.ts";
import { formatBridgeResult } from "./bridge.ts";

interface Row {
  label: string;
  op: string;
  capability: string | null;
  paramsJson: string;
  expect: "ok" | "denied";
}

// Representative operations the UI is allowed to request. "denied"
// rows demonstrate that the boundary rejects unauthorized requests.
const ROWS: Row[] = [
  { label: "Core health", op: "health", capability: null, paramsJson: "{}", expect: "ok" },
  { label: "Read scoped fixture (allowed)", op: "read_fixture", capability: "cap-read-fixture-001", paramsJson: '{"path":"hello.txt"}', expect: "ok" },
  { label: "Read traversal (must deny)", op: "read_fixture", capability: "cap-read-fixture-001", paramsJson: '{"path":"../../secret.txt"}', expect: "denied" },
  { label: "Write scoped output (allowed)", op: "write_output", capability: "cap-write-output-001", paramsJson: '{"path":"run/out.txt","content":"ok"}', expect: "ok" },
  { label: "Write to .git (must deny)", op: "write_output", capability: "cap-write-output-001", paramsJson: '{"path":".git/config","content":"x"}', expect: "denied" },
  { label: "Unknown operation (must deny)", op: "shell_exec", capability: null, paramsJson: "{}", expect: "denied" },
];

export function App(): JSX.Element {
  const [status, setStatus] = useState<string>("Idle. Activate an operation.");
  const [dir, setDir] = useState<"ltr" | "rtl">("ltr");
  const [rows, setRows] = useState<Record<string, BridgeResponse | undefined>>({});

  const run = useCallback(async (row: Row) => {
    setStatus(`Requesting: ${row.label}…`);
    // coreRequest never throws: it always resolves to a validated
    // BridgeResponse (ok / denied / error). The single-line result is
    // written into the semantic live region below so keyboard and
    // screen-reader users receive the same information.
    const resp = await coreRequest(row.op, row.capability, row.paramsJson);
    setRows((prev) => ({ ...prev, [row.label]: resp }));
    setStatus(formatBridgeResult(row.label, resp));
  }, []);

  return (
    <main dir={dir} lang={dir === "rtl" ? "ar" : "en"} style={styles.main}>
      <h1 style={styles.h1}>S0.5A Desktop Security Prototype</h1>
      <p style={styles.note}>
        EXPERIMENTAL — NEVER MERGE. The UI holds zero authority; every
        effect is delegated to the separate Rust core.
      </p>

      <div
        role="status"
        aria-live="polite"
        aria-atomic="true"
        style={styles.status}
      >
        {status}
      </div>

      <section aria-labelledby="ops-h">
        <h2 id="ops-h" style={styles.h2}>Operations</h2>
        <ul style={styles.list}>
          {ROWS.map((row) => {
            const resp = rows[row.label];
            return (
              <li key={row.label} style={styles.li}>
                <button type="button" onClick={() => void run(row)} style={styles.button}>
                  {row.label}
                </button>
                <span style={styles.expect}>expects: {row.expect}</span>
                {resp && (
                  <span role="note" style={styles.result}>
                    → {resp.status.toUpperCase()} — {resp.code}
                  </span>
                )}
              </li>
            );
          })}
        </ul>
      </section>

      <section aria-labelledby="a11y-h">
        <h2 id="a11y-h" style={styles.h2}>Accessibility controls</h2>
        <button
          type="button"
          onClick={() => setDir((d) => (d === "ltr" ? "rtl" : "ltr"))}
          aria-pressed={dir === "rtl"}
          style={styles.button}
        >
          Toggle RTL (Arabic) layout — currently {dir.toUpperCase()}
        </button>
      </section>
    </main>
  );
}

// Inline styles only — no remote stylesheet, honoring the strict CSP.
const styles: Record<string, React.CSSProperties> = {
  main: { fontFamily: "system-ui, sans-serif", padding: "1rem", maxWidth: 760, margin: "0 auto" },
  h1: { fontSize: "1.4rem" },
  h2: { fontSize: "1.1rem", marginTop: "1.25rem" },
  note: { color: "#a33", fontWeight: 600 },
  status: { border: "2px solid currentColor", padding: "0.5rem", borderRadius: 6, margin: "0.75rem 0" },
  list: { listStyle: "none", padding: 0, display: "flex", flexDirection: "column", gap: "0.5rem" },
  li: { display: "flex", flexWrap: "wrap", alignItems: "center", gap: "0.5rem" },
  button: { padding: "0.5rem 0.75rem", fontSize: "1rem", cursor: "pointer" },
  expect: { fontSize: "0.85rem", opacity: 0.75 },
  result: { fontFamily: "ui-monospace, monospace", fontSize: "0.85rem" },
};

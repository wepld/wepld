#!/usr/bin/env python3
"""Dependency-free validation for WePLD architecture documentation."""

from __future__ import annotations

import argparse
import html
import os
import re
import subprocess
import sys
import unicodedata
from collections import Counter
from pathlib import Path
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"
ERRORS: list[str] = []


def error(message: str) -> None:
    ERRORS.append(message)


def run_git(*args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=check, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )


def run_git_bytes(*args: str) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=False,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )


def read_utf8(path: Path) -> str:
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError as exc:
        error(f"{path.relative_to(ROOT)}: invalid UTF-8: {exc}")
        return ""
    except OSError as exc:
        try:
            label = path.relative_to(ROOT)
        except ValueError:
            label = path
        error(f"{label}: cannot read file: {exc}")
        return ""
    if "\ufffd" in text:
        error(f"{path.relative_to(ROOT)}: contains Unicode replacement character")
    return text


def required_text(texts: dict[Path, str], path: Path) -> str:
    resolved = path.resolve()
    if resolved not in texts:
        error(f"missing required documentation file: {path.relative_to(ROOT)}")
        return ""
    return texts[resolved]


def section_between(text: str, start: str, end: str, label: str) -> str:
    if start not in text:
        error(f"{label}: missing section marker: {start}")
        return ""
    tail = text.split(start, 1)[1]
    if end not in tail:
        error(f"{label}: missing section marker: {end}")
        return ""
    return tail.split(end, 1)[0]


def github_slug(value: str) -> str:
    value = re.sub(r"!?(\[([^]]*)\])\([^)]*\)", r"\2", value)
    value = re.sub(r"<[^>]+>", "", value)
    value = value.replace("`", "").strip().lower()
    chars: list[str] = []
    for char in value:
        category = unicodedata.category(char)
        if char in {" ", "-", "_"} or category[0] in {"L", "N"}:
            chars.append(char)
    return re.sub(r"\s+", "-", "".join(chars))


def markdown_anchors(text: str) -> set[str]:
    anchors: set[str] = set()
    counts: Counter[str] = Counter()
    in_fence = False
    fence_char = ""
    fence_len = 0
    for line in text.splitlines():
        marker = re.match(r"^\s*(`{3,}|~{3,})", line)
        if marker:
            token = marker.group(1)
            if not in_fence:
                in_fence, fence_char, fence_len = True, token[0], len(token)
            elif token[0] == fence_char and len(token) >= fence_len:
                in_fence = False
            continue
        if in_fence:
            continue
        heading = re.match(r"^ {0,3}#{1,6}\s+(.+?)\s*#*\s*$", line)
        if not heading:
            continue
        base = github_slug(heading.group(1))
        if not base:
            continue
        count = counts[base]
        counts[base] += 1
        anchors.add(base if count == 0 else f"{base}-{count}")
    return anchors


def validate_fences_and_tables(path: Path, text: str) -> None:
    open_fence: tuple[str, int, int, str] | None = None
    mermaid_lines = 0
    lines = text.splitlines()
    for lineno, line in enumerate(lines, 1):
        marker = re.match(r"^\s*(`{3,}|~{3,})([^`]*)$", line)
        if marker:
            token, suffix = marker.group(1), marker.group(2).strip()
            if open_fence is None:
                open_fence = (token[0], len(token), lineno, suffix.split()[0] if suffix else "")
                mermaid_lines = 0
            elif token[0] == open_fence[0] and len(token) >= open_fence[1]:
                if open_fence[3].lower() == "mermaid" and mermaid_lines == 0:
                    error(f"{path.relative_to(ROOT)}:{open_fence[2]}: empty Mermaid fence")
                open_fence = None
            elif open_fence[3].lower() == "mermaid" and line.strip():
                mermaid_lines += 1
            continue
        if open_fence and open_fence[3].lower() == "mermaid" and line.strip():
            mermaid_lines += 1
    if open_fence:
        error(f"{path.relative_to(ROOT)}:{open_fence[2]}: unclosed code fence")

    for index, line in enumerate(lines[:-1]):
        if not line.lstrip().startswith("|"):
            continue
        separator = lines[index + 1]
        if re.match(r"^\s*\|?(?:\s*:?-{3,}:?\s*\|)+\s*$", separator):
            header_cells = len(re.findall(r"(?<!\\)\|", line))
            separator_cells = len(re.findall(r"(?<!\\)\|", separator))
            if header_cells != separator_cells:
                error(f"{path.relative_to(ROOT)}:{index + 1}: table header/separator width mismatch")


def split_target(raw: str) -> tuple[str, str]:
    raw = html.unescape(raw.strip())
    if raw.startswith("<") and raw.endswith(">"):
        raw = raw[1:-1]
    raw = raw.split(' "', 1)[0].split(" '", 1)[0]
    parts = urlsplit(raw)
    return unquote(parts.path), unquote(parts.fragment)


def validate_markdown_links(path: Path, text: str, texts: dict[Path, str]) -> None:
    link_pattern = re.compile(r"(?<!!)\[[^]\n]+\]\(([^)\n]+)\)")
    for match in link_pattern.finditer(text):
        raw = match.group(1).strip()
        if re.match(r"^(?:https?://|mailto:|data:)", raw, re.I):
            continue
        target_path, fragment = split_target(raw)
        target = path if not target_path else (path.parent / target_path).resolve()
        try:
            target.relative_to(ROOT)
        except ValueError:
            error(f"{path.relative_to(ROOT)}: local link escapes repository: {raw}")
            continue
        if not target.exists():
            error(f"{path.relative_to(ROOT)}: missing link target: {raw}")
            continue
        if not fragment:
            continue
        target_text = texts.get(target)
        if target_text is None:
            target_text = read_utf8(target)
            texts[target] = target_text
        if target.suffix.lower() == ".md":
            anchors = markdown_anchors(target_text)
        elif target.suffix.lower() in {".html", ".htm"}:
            anchors = set(re.findall(r"\b(?:id|name)=[\"']([^\"']+)[\"']", target_text, re.I))
        else:
            continue
        if fragment not in anchors:
            error(f"{path.relative_to(ROOT)}: missing anchor '{fragment}' in {target.relative_to(ROOT)}")


def validate_html(path: Path, text: str, texts: dict[Path, str]) -> None:
    ids = re.findall(r"\bid=[\"']([^\"']+)[\"']", text, re.I)
    for duplicate, count in Counter(ids).items():
        if count > 1:
            error(f"{path.relative_to(ROOT)}: duplicate HTML id '{duplicate}'")
    for raw in re.findall(r"\bhref=[\"']([^\"']+)[\"']", text, re.I):
        if re.match(r"^(?:https?://|mailto:|data:|javascript:)", raw, re.I):
            continue
        target_path, fragment = split_target(raw)
        target = path if not target_path else (path.parent / target_path).resolve()
        try:
            target.relative_to(ROOT)
        except ValueError:
            error(f"{path.relative_to(ROOT)}: HTML link escapes repository: {raw}")
            continue
        if not target.exists():
            error(f"{path.relative_to(ROOT)}: missing HTML link target: {raw}")
            continue
        if fragment:
            if not target.is_file():
                error(f"{path.relative_to(ROOT)}: HTML anchor target is not a file: {raw}")
                continue
            target_text = texts.get(target)
            if target_text is None:
                target_text = read_utf8(target)
                texts[target] = target_text
            anchors = set(re.findall(r"\b(?:id|name)=[\"']([^\"']+)[\"']", target_text, re.I))
            if fragment not in anchors:
                error(f"{path.relative_to(ROOT)}: missing HTML anchor '{fragment}' in {target.relative_to(ROOT)}")


def validate_adrs(texts: dict[Path, str]) -> None:
    adr_files = sorted((DOCS / "adr").glob("ADR-[0-9][0-9][0-9][0-9]-*.md"))
    numbers = [re.match(r"ADR-(\d{4})-", path.name).group(1) for path in adr_files]  # type: ignore[union-attr]
    for number, count in Counter(numbers).items():
        if count > 1:
            error(f"duplicate ADR number: {number}")
    expected = {f"{number:04d}" for number in range(15, 27)}
    if set(numbers) != expected:
        error(f"ADR set mismatch: expected {sorted(expected)}, found {numbers}")
    index = required_text(texts, DOCS / "adr" / "README.md")
    for path in adr_files:
        text = required_text(texts, path)
        status = re.search(r"^\*\*Status:\*\*\s*(.+?)\s*$", text, re.M)
        if not status or status.group(1) != "Proposed":
            error(f"{path.relative_to(ROOT)}: status must be exactly Proposed")
        if index.count(path.name) != 1:
            error(f"docs/adr/README.md: {path.name} must appear exactly once in the index")
    indexed = set(re.findall(r"\((ADR-\d{4}-[^)]+\.md)\)", index))
    if indexed != {path.name for path in adr_files}:
        error("docs/adr/README.md: indexed ADR filenames do not match files")


def h_sections(text: str) -> list[str]:
    return re.findall(r"^## H([1-9])\b", text, re.M)


def validate_roadmap(texts: dict[Path, str]) -> None:
    expected = list("123456789")
    for name in ("19_Implementation_Roadmap.md", "21_Project_Backlog.md", "22_Milestones.md"):
        path = (DOCS / name).resolve()
        found = h_sections(required_text(texts, path))
        if found != expected:
            error(f"{path.relative_to(ROOT)}: H milestone order/uniqueness mismatch: {found}")
    milestone_text = required_text(texts, DOCS / "22_Milestones.md")
    sections = re.split(r"(?=^## H[1-9]\b)", milestone_text, flags=re.M)[1:]
    for number, section in enumerate(sections, 1):
        required = "Baseline" if number == 1 else f"H{number - 1}"
        if required not in section:
            error(f"docs/22_Milestones.md: H{number} does not name dependency {required}")


def table_rows_between(text: str, start: str, end: str) -> list[list[str]]:
    section = section_between(text, start, end, "docs/35")
    rows: list[list[str]] = []
    for line in section.splitlines():
        if not line.startswith("|") or re.match(r"^\|\s*:?-", line):
            continue
        rows.append([cell.strip() for cell in line.strip().strip("|").split("|")])
    return rows


def validate_reference_study(texts: dict[Path, str]) -> None:
    text = required_text(texts, DOCS / "35_Reference_Systems_and_Competitive_Architecture.md")
    required_systems = {"Pi", "Zed / ACP", "Warp", "GitHub Spec Kit", "Claude Code", "Codex", "Cursor", "OpenCode", "Aider", "OpenHands", "Atoms", "MetaGPT"}
    ledger_rows = table_rows_between(text, "## First-party source and license ledger", "## Uniform assessment dimensions")
    ledger_data = ledger_rows[1:] if ledger_rows else []
    ledger_systems = {row[0] for row in ledger_data if row}
    missing = required_systems - ledger_systems
    if missing:
        error(f"docs/35: source/license ledger missing systems: {sorted(missing)}")
    for row in ledger_data:
        if len(row) != 3 or any(not cell for cell in row):
            error(f"docs/35: source/license ledger row must have 3 nonempty columns: {row[:1]}")
            continue
        if "http" not in row[1] or not re.search(r"license|proprietary|terms|reuse|copy|MIT|Apache|GPL|AGPL", row[2], re.I):
            error(f"docs/35: source/license ledger row lacks evidence or reuse constraint: {row[0]}")
    allowed_domains = ("github.com", "pi.dev", "zed.dev", "agentclientprotocol.com", "warp.dev", "docs.warp.dev", "docs.github.com", "github.github.com", "code.claude.com", "docs.anthropic.com", "openai.com", "platform.openai.com", "learn.chatgpt.com", "cursor.com", "docs.cursor.com", "opencode.ai", "aider.chat", "docs.openhands.dev", "atoms.dev", "help.atoms.dev", "docs.deepwisdom.ai")
    ledger_section = section_between(text, "## First-party source and license ledger", "## Uniform assessment dimensions", "docs/35")
    for url in re.findall(r"https?://[^)\s]+", ledger_section):
        host = urlsplit(url).hostname or ""
        if not any(host == domain or host.endswith("." + domain) for domain in allowed_domains):
            error(f"docs/35: non-official domain in source ledger: {url}")

    expected_dimensions = [f"{number:02d}" for number in range(1, 17)]
    for system, next_heading in (("Atoms", "MetaGPT"), ("MetaGPT", "Cross-system architectural synthesis")):
        section = section_between(text, f"## {system}", f"## {next_heading}", "docs/35")
        dimension_rows = [line for line in section.splitlines() if re.match(r"^\| D\d{2} \|", line)]
        found_dimensions = [match.group(1) for line in dimension_rows if (match := re.match(r"^\| D(\d{2}) \|", line))]
        if found_dimensions != expected_dimensions:
            error(f"docs/35: {system} must define D01..D16 exactly once; found {found_dimensions}")
        for line in dimension_rows:
            if "**Observed:**" not in line or "**Inference:**" not in line:
                error(f"docs/35: {system} dimension row lacks Observed/Inference separation")

    matrix_rows = table_rows_between(text, "## Reference Systems Matrix", "## Controlled architecture spikes")
    if not matrix_rows or len(matrix_rows[0]) != 11:
        error("docs/35: reference matrix must have 11 columns")
    matrix_data = matrix_rows[1:] if matrix_rows else []
    for row in matrix_data:
        if len(row) != 11 or any(not cell for cell in row):
            error(f"docs/35: matrix row has {len(row)} columns, expected 11: {row[:2]}")
    matrix_systems = {row[0] for row in matrix_data if row}
    missing_matrix = required_systems - matrix_systems
    if missing_matrix:
        error(f"docs/35: reference matrix missing systems: {sorted(missing_matrix)}")

    detailed_rows = table_rows_between(text, "| ID and stated problem", "## Roadmap admission rule")
    detailed: list[str] = []
    for row in detailed_rows:
        if len(row) != 7 or any(not cell for cell in row):
            error(f"docs/35: detailed experiment row must have 7 nonempty fields: {row[:1]}")
            continue
        match = re.search(r"RS-(\d{2})", row[0])
        if not match:
            error(f"docs/35: detailed experiment row lacks an RS ID: {row[0][:40]}")
            continue
        experiment_id = match.group(1)
        detailed.append(experiment_id)
        if 21 <= int(experiment_id) <= 30:
            treatment = row[2].lower()
            rollback = row[5].lower()
            if "control" not in treatment or "compare" not in treatment:
                error(f"docs/35: RS-{experiment_id} must name its control and compared treatments")
            if "inject" not in treatment:
                error(f"docs/35: RS-{experiment_id} must name safety/failure injections")
            if not re.search(r"[0-9≥≤%]", row[1]) or not re.search(r"[0-9≥≤%]", row[4]):
                error(f"docs/35: RS-{experiment_id} must state measurable benefit and acceptance thresholds")
            if not any(word in rollback for word in ("reject", "disable", "remove", "fall back", "defer")):
                error(f"docs/35: RS-{experiment_id} must state a rejection/disable/rollback rule")
            if not re.search(r"(?:Baseline|H[1-9])", row[6]):
                error(f"docs/35: RS-{experiment_id} must state milestone placement")
    expected_ids = [f"{number:02d}" for number in range(31)]
    if sorted(detailed) != expected_ids:
        error(f"docs/35: detailed experiment IDs must be unique RS-00..RS-30; found {sorted(detailed)}")

    extension_ids = {f"RS-{number:02d}" for number in range(21, 31)}
    for name in ("19_Implementation_Roadmap.md", "21_Project_Backlog.md", "22_Milestones.md", "34_Harness_Evaluation_Protocol.md"):
        document = required_text(texts, DOCS / name)
        missing_ids = sorted(extension_ids - set(re.findall(r"RS-\d{2}", document)))
        if missing_ids:
            error(f"docs/{name}: missing extension experiment mappings: {missing_ids}")
    for name in ("23_Technology_Evaluation.md", "26_Testing_Strategy.md"):
        if "RS-00–RS-30" not in required_text(texts, DOCS / name):
            error(f"docs/{name}: must name the complete RS-00–RS-30 register")


def validate_committee(texts: dict[Path, str]) -> None:
    committee = required_text(texts, DOCS / "36_Engineering_Committee.md")
    evaluation = required_text(texts, DOCS / "37_Committee_Evaluation_Protocol.md")
    label = "docs/36_Engineering_Committee.md"
    # Docs hard-wrap prose, so multi-word requirements are matched on a
    # whitespace-normalized view (markdown emphasis stripped).
    committee_flat = " ".join(committee.replace("**", "").split())

    required_sentinels = {
        "Committee agreement is not engineering truth.": "missing Committee authority boundary",
        "The Committee is advisory": "missing advisory-standing statement",
        "outside the authority chain": "missing authority-chain exclusion",
        "before seeing any other member's opinion": "missing independent first round",
        "Minority reports are preserved verbatim": "missing minority-report preservation",
        "maximum challenge rounds": "missing finite round limit",
        "hard cost ceiling": "missing hard budget limit",
        "data-egress policy": "missing data-egress policy",
        (
            "must not capture browser cookies, automate consumer chat sessions, "
            "or circumvent provider usage restrictions"
        ): "missing consumer-subscription boundary",
        "37_Committee_Evaluation_Protocol.md": "missing Committee evaluation protocol link",
    }
    for needle, description in required_sentinels.items():
        if needle not in committee_flat:
            error(f"{label}: {description} (required text absent): {needle[:60]}")

    dispositions = (
        "ReportReady", "QuorumNotMet", "MoreEvidenceRequired", "MemberFailure",
        "BudgetExhausted", "DeadlineExceeded", "PolicyBlocked", "Cancelled",
        "NonConvergent",
    )
    for disposition in dispositions:
        if disposition not in committee:
            error(f"{label}: missing durable failure disposition: {disposition}")

    eval_label = "docs/37_Committee_Evaluation_Protocol.md"
    for arm in [f"EC-A{number}" for number in range(1, 9)]:
        if arm not in evaluation:
            error(f"{eval_label}: missing compared configuration: {arm}")
    if "## Rejection criteria" not in evaluation:
        error(f"{eval_label}: missing rejection criteria for the Committee feature")

    combined = "\n".join(texts.values())
    forbidden = {
        r"(?:majority|committee)\s+vote\s+(?:approves|accepts|authorizes|merges|"
        r"can\s+approve|may\s+approve)": "model voting treated as approval",
        r"committee\s+(?:directly\s+|automatically\s+|silently\s+)?"
        r"(?:updates|edits|mutates|rewrites|modifies)\s+the\s+"
        r"(?:approved\s+)?(?:delivery\s?plan|plan\b|specification|outcome\s+contract)":
            "automatic plan mutation by the Committee",
        r"(?:capture|reuse|harvest)\s+(?:browser|session)\s+cookies\s+to":
            "consumer-subscription workaround",
        r"automat\w*\s+(?:a|the)\s+consumer\s+chat\s+session":
            "consumer-subscription workaround",
        r"unlimited\s+(?:challenge\s+|deliberation\s+)?rounds": "unbounded deliberation",
    }
    for pattern, description in forbidden.items():
        match = re.search(pattern, combined, re.I)
        if match:
            error(f"forbidden Committee claim ({description}): {match.group(0)!r}")


def validate_stale_claims(texts: dict[Path, str]) -> None:
    combined = "\n".join(texts.values())
    stale = {
        "Proposed ADR-0015–ADR-0024": "stale ADR range",
        "ADR-0015 through ADR-0024": "stale ADR range",
        "Proposed ADR-0015–ADR-0025": "stale ADR range after ADR-0026",
        "ADR-0015 through ADR-0025": "stale ADR range after ADR-0026",
        "ADR-0015–ADR-0025": "stale ADR range after ADR-0026",
        "RS-00–RS-20": "stale experiment range",
        "RS-00–RS-26": "stale experiment range",
        "Build Feature Baseline Gate accepted": "acceptance-only H1 gate",
        "baseline accepted, relevant ADR": "acceptance-only baseline summary",
        "push a branch, open a pull request": "stale Draft PR state claim",
    }
    for needle, description in stale.items():
        if needle in combined:
            error(f"stale claim ({description}): {needle}")
    if re.search(r"\]\([^)]*ADR-0024-harness-evaluation-provider-certification\.md", combined):
        error("stale link to removed combined ADR-0024 filename")


def validate_changed_scope(base: str) -> None:
    if not base or base.startswith("-") or any(character in base for character in "\r\n\0"):
        error(f"invalid base ref: {base!r}")
        return
    verified = run_git("rev-parse", "--verify", f"{base}^{{commit}}", check=False)
    if verified.returncode:
        error(f"git could not resolve base ref {base!r}:\n{verified.stdout}{verified.stderr}")

    names: set[str] = set()
    discovery_commands = (
        ("diff", "--name-only", "-z", f"{base}...HEAD"),
        ("diff", "--name-only", "-z"),
        ("diff", "--cached", "--name-only", "-z"),
        ("ls-files", "--others", "--exclude-standard", "-z"),
    )
    for args in discovery_commands:
        result = run_git_bytes(*args)
        if result.returncode:
            error(f"git {' '.join(args)} failed:\n{os.fsdecode(result.stdout)}{os.fsdecode(result.stderr)}")
            continue
        names.update(os.fsdecode(item) for item in result.stdout.split(b"\0") if item)
    allowed_exact = {"README.md", ".github/workflows/docs-validation.yml", "scripts/validate_architecture_docs.py"}
    forbidden = [name for name in names if name not in allowed_exact and not name.startswith("docs/")]
    if forbidden:
        error(f"production/non-documentation paths changed: {sorted(forbidden)}")
    for args in (("diff", "--check"), ("diff", "--cached", "--check"), ("diff", "--check", f"{base}...HEAD")):
        result = run_git(*args, check=False)
        if result.returncode:
            error(f"git {' '.join(args)} failed:\n{result.stdout}{result.stderr}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", default="origin/main", help="Git base ref for scope/diff validation")
    args = parser.parse_args()

    paths = [ROOT / "README.md", *sorted(DOCS.rglob("*.md")), *sorted(DOCS.rglob("*.html"))]
    texts = {path.resolve(): read_utf8(path) for path in paths}
    for path in paths:
        text = texts[path.resolve()]
        if path.suffix.lower() == ".md":
            validate_fences_and_tables(path, text)
            validate_markdown_links(path, text, texts)
        elif path.suffix.lower() in {".html", ".htm"}:
            validate_html(path, text, texts)

    validate_adrs(texts)
    validate_roadmap(texts)
    validate_reference_study(texts)
    validate_committee(texts)
    validate_stale_claims(texts)
    validate_changed_scope(args.base)

    if ERRORS:
        print("Architecture documentation validation FAILED", file=sys.stderr)
        for item in ERRORS:
            print(f"- {item}", file=sys.stderr)
        return 1
    print(f"Architecture documentation validation PASS ({len(paths)} files; base {args.base})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

# smith

A terminal CLI for [Smith_Agentic](https://github.com/AddisonTech/Smith_Agentic). Start crew runs, stream output, manage files, and cancel jobs — all without leaving your terminal.

Requires Smith_Agentic to be running locally. The UI lives at `http://localhost:8765`.

---

## Installation

You need [Rust](https://rustup.rs) installed.

**From source:**
```sh
git clone https://github.com/AddisonTech/Smith_Agentic_CLI
cd Smith_Agentic_CLI
cargo install --path .
```

**Direct install (once published to crates.io):**
```sh
cargo install smith
```

After install, `smith` is available anywhere in your shell.

---

## Prerequisites

Smith_Agentic must be running before any command that hits the API:

```sh
cd Smith_Agentic
python ui/server.py
```

---

## Configuration

| Variable             | Default                  | Description                     |
|----------------------|--------------------------|---------------------------------|
| `SMITH_AGENTIC_URL`  | `http://localhost:8765`  | Base URL of the backend server  |

Every command prints the URL it is targeting in dim text so you always know which instance you are talking to.

**Example — point to a remote instance:**
```sh
export SMITH_AGENTIC_URL=http://192.168.1.50:8765
smith status
```

---

## Commands

### `smith status`

Check whether Smith_Agentic and Ollama are reachable.

```
$ smith status

  → http://localhost:8765

  Smith_Agentic      online
  Ollama             online
```

---

### `smith crews`

List available crews and their configured default models.

```
$ smith crews

  → http://localhost:8765

  CREW        MODEL
  ────────────────────────────────
  default     qwen2.5:7b
  ops         qwen2.5:7b
  plc         qwen2.5:14b
  react       qwen2.5:7b
  safety      qwen2.5:7b
  vision      qwen2.5:14b
```

---

### `smith run --crew <crew> --goal <goal> [--model <model>] [--chain]`

Start a new crew run. Returns a run ID immediately; the crew executes in the background.

**Options:**

| Flag        | Default     | Description                                      |
|-------------|-------------|--------------------------------------------------|
| `--crew`    | `default`   | Crew to run (`default`, `plc`, `react`, `vision`, `safety`, `ops`) |
| `--goal`    | *(required)*| What you want the crew to accomplish             |
| `--model`   | *(crew default)* | Override the Ollama model for this run      |
| `--chain`   | false       | Auto-run safety then ops crews after completion  |

**Examples:**

```sh
# Default crew
smith run --goal "Write a Python function to parse ISO 8601 timestamps"

# PLC crew with model override
smith run --crew plc --goal "Generate a conveyor start ladder routine" --model qwen2.5:14b

# React crew with auto-chain
smith run --crew react --goal "Build a sortable data table component" --chain
```

**Output:**
```
  → http://localhost:8765

  run_id     a3f8c1d2
  crew       plc

  Follow output with:
    smith watch a3f8c1d2
```

---

### `smith watch <run_id>`

Stream output from a run. Polls every 2 seconds and prints new lines until the run reaches a terminal state (completed, error, or cancelled). Works for runs that have already finished too — it prints all buffered output and exits immediately.

```sh
smith watch a3f8c1d2
```

**Output (live):**
```
  → http://localhost:8765

  → watching a3f8c1d2

[SmithAgentic] Starting crew='plc' model='qwen2.5:14b'
[SmithAgentic] Goal: Generate a conveyor start ladder routine
[Orchestrator] Task received. Breaking into sub-goals...
[PLC Builder] Writing Conveyor_Start routine...
...

  status     completed
  files      conveyor_start.l5x
```

---

### `smith cancel <run_id>`

Send a cancellation signal to an active run. The crew finishes its current step cleanly before stopping.

```sh
smith cancel a3f8c1d2
```

```
  → http://localhost:8765

  cancellation sent to run a3f8c1d2
  status: cancelling
```

---

### `smith outputs`

List all files in the `outputs/` directory with human-readable sizes.

```sh
smith outputs
```

```
  → http://localhost:8765

  PATH                              SIZE
  ──────────────────────────────────────────────
  conveyor_start.l5x                3.2 KB
  deliverable.md                   12.8 KB
  qa_report.md                      5.1 KB
  docs/deliverable_docs.md          4.5 KB

  4 files
```

---

### `smith runs`

Run history is tracked in the Smith_Agentic UI. This command prints a reminder with the URL.

```sh
smith runs
```

```
  Run history is not available via the CLI.
  View it in the Smith_Agentic UI at http://localhost:8765
```

---

## Exit codes

| Code | Meaning                             |
|------|-------------------------------------|
| `0`  | Success                             |
| `1`  | Error (backend unreachable, API error, bad run ID, etc.) |

Error messages are printed to stderr. All normal output goes to stdout.

---

## Related

- [Smith_Agentic](https://github.com/AddisonTech/Smith_Agentic) — multi-agent backend
- [Smith_Agentic_UI](https://github.com/AddisonTech/Smith_Agentic_UI) — React dashboard with live streaming
- [Smith_Agentic_MCP](https://github.com/AddisonTech/Smith_Agentic_MCP) — MCP server for Claude Code / Claude Desktop

---

## License

MIT — see [LICENSE](LICENSE)

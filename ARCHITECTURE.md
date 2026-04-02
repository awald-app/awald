# Architecture

## Overview

Awald is a three-layer system. Rust owns memory, execution order, and rendering. Python owns computation. The boundary between them is `pyo3`.

```
┌──────────────────────────────────────────────────────────────────┐
│  Layer 1: UI Shell  (SvelteKit in Tauri WebView)                │
│  Responsibility: rendering, user interaction, state display      │
├──────────────────────────────────────────────────────────────────┤
│  Layer 2: Rust Core  (Tauri 2 backend)                          │
│  Responsibility: session state, execution ordering,              │
│                  data buffer ownership, output capture           │
├──────────────────────────────────────────────────────────────────┤
│  Layer 3: Python Runtime  (pyo3 bridge)                         │
│  Responsibility: statistical computation only                    │
│  Libraries: pyfixest, statsmodels, tidyecon, svy                │
└──────────────────────────────────────────────────────────────────┘
```

---

## Component Map

### 1. UI Shell (`src/`)

Built with SvelteKit. Runs inside Tauri's WebView (WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux). Communicates with the Rust backend exclusively via Tauri's typed command interface — no direct system calls from the frontend.

**Key components:**

| Component | Responsibility |
|---|---|
| `DataEditor` | Virtual-scroll spreadsheet view over a Polars DataFrame |
| `ScriptEditor` | CodeMirror 6 editor — Python-like syntax, linear execution only |
| `ResultsPane` | Renders `tidyecon` HTML table output inline |
| `VariableBrowser` | Lists columns, types, and summary stats for the loaded dataset |
| `StatusBar` | Session state, execution status, Python interpreter info |

**Frontend state management:**

Svelte stores hold:
- `sessionStore` — loaded dataset metadata (name, nrows, ncols, schema)
- `scriptStore` — current script content and execution cursor
- `resultsStore` — ordered list of output blocks (tables, text, errors)

The UI never holds raw data. It holds *metadata* and requests slices from Rust on scroll.

---

### 2. Rust Core (`src-tauri/src/`)

The Rust backend owns all mutable state. It is the single source of truth.

#### `data/` — Data layer

Wraps a `polars::DataFrame` in an `Arc<RwLock<DataFrame>>`. The data editor requests row slices via Tauri commands; Rust returns serialized JSON for the visible viewport only. No full dataset copy ever crosses to the frontend.

```rust
// src-tauri/src/data/store.rs
pub struct DataStore {
    frame: Arc<RwLock<DataFrame>>,
    path:  Option<PathBuf>,
    dirty: bool,
}
```

Supported import formats: `.csv`, `.parquet`, `.dta` (Stata), `.xlsx`.
Polars handles all parsing natively. `.dta` requires the `polars-io` Stata reader.

#### `engine/` — Python executor

Manages the Python interpreter lifecycle via `pyo3`. The executor owns a single `Python` GIL token per session. Execution is strictly sequential — the executor holds a `Mutex` that prevents concurrent Python calls.

```rust
// src-tauri/src/engine/executor.rs
pub struct Executor {
    gil_guard: Mutex<()>,          // one Python call at a time
    globals:   Arc<Mutex<PyDict>>, // persistent Python namespace
    history:   Vec<ExecutionRecord>,
}
```

**Execution model:**
1. Frontend submits a script string via Tauri command
2. Rust splits script into logical blocks (statements separated by blank lines)
3. Each block is `exec()`'d sequentially via pyo3
4. stdout/stderr captured per block
5. If a block produces a `tidyecon.SummaryTable` object, it is serialized to HTML and emitted as a result block
6. If a block raises an exception, execution halts at that block — subsequent blocks do not run

This guarantees linear, reproducible execution. The mental model is identical to a Stata `.do` file.

#### `session/` — Session state

Coordinates between data layer and engine. Holds the mapping from Python variable names to Polars DataFrame handles, so that `df = aw.load("data.csv")` in the script editor registers the frame in the data store and makes it inspectable in the variable browser without copying data.

#### `commands/` — Tauri command handlers

All frontend ↔ backend communication goes through typed Tauri commands:

```rust
#[tauri::command]
async fn load_file(path: String, state: State<AppState>) -> Result<DatasetMeta, String>

#[tauri::command]
async fn get_rows(start: usize, end: usize, state: State<AppState>) -> Result<Vec<Row>, String>

#[tauri::command]
async fn execute_script(script: String, state: State<AppState>) -> Result<ExecutionResult, String>

#[tauri::command]
async fn get_variable_summary(name: String, state: State<AppState>) -> Result<VarSummary, String>
```

---

### 3. Python Runtime (`python/`)

A self-contained Python environment managed by `uv`. Bundled into the Tauri application at build time via `tauri-plugin-shell` or direct embedding.

**Core dependencies:**

```toml
# python/pyproject.toml
[project]
requires-python = ">=3.12"
dependencies = [
    "pyfixest>=0.30",
    "statsmodels>=0.14",
    "polars>=1.0",
    "tidyecon>=0.1",
    "svy>=0.13",
    "pandas>=2.1",
    "numpy>=1.26",
]
```

**`awald_runtime/`** — thin Python module that:
- Exposes an `aw` namespace (e.g. `aw.load()`, `aw.reg()`, `aw.table()`)
- Wraps `pyfixest` and `statsmodels` with Awald's output conventions
- Routes results through `tidyecon.modelsummary()` for rendering

The goal is that a researcher familiar with Stata can write:

```python
df = aw.load("nlsw88.csv")
aw.describe(df)

m1 = aw.reg("wage ~ hours + tenure",        data=df)
m2 = aw.reg("wage ~ hours + tenure | race", data=df)  # FE

aw.table({"(1)": m1, "(2)": m2},
         coef_map={"hours": "Hours worked", "tenure": "Job tenure"},
         output="results/table1.docx")
```

This is the API surface that replaces Stata commands.

---

## Data Flow: Loading a Dataset

```
User selects file
      ↓
Frontend → Tauri command: load_file(path)
      ↓
Rust: Polars reads file into DataFrame
      ↓
Rust: stores DataFrame in DataStore (Arc<RwLock>)
      ↓
Rust: returns DatasetMeta {nrows, ncols, schema} to frontend
      ↓
Frontend: renders VariableBrowser + DataEditor shell
      ↓
DataEditor: requests rows 0–50 (visible viewport)
      ↓
Rust: DataFrame.slice(0, 50) → JSON → frontend
      ↓
DataEditor: renders 50 rows, user scrolls
      ↓
DataEditor: requests rows 51–100 → repeat
```

The DataFrame never leaves Rust memory in full. Only viewport slices cross the boundary.

---

## Data Flow: Executing a Script

```
User writes script, hits Run
      ↓
Frontend → Tauri command: execute_script(script)
      ↓
Rust: acquires Executor Mutex
      ↓
Rust: splits script into blocks
      ↓
For each block:
  Rust: pyo3 exec(block, globals)
  Python: runs pyfixest / statsmodels / etc.
  Python: if result is SummaryTable → serialize to HTML
  Rust: captures output, emits ResultBlock to frontend
  Frontend: appends ResultBlock to ResultsPane
      ↓
Rust: releases Mutex
      ↓
Execution complete
```

If any block raises a Python exception, the loop breaks. The error is surfaced inline next to the offending block in the script editor, identical to how Stata surfaces `.do` file errors.

---

## Threading Model

```
Main thread (Rust/Tauri)
├── UI event loop          ← WebView events, Tauri commands
├── Data thread            ← Polars DataFrame operations
└── Execution thread       ← pyo3 Python calls (serialized via Mutex)
```

The Python GIL is held only during active computation. The UI thread never blocks on Python execution — commands are `async` and the frontend shows a spinner while the executor runs.

---

## Build System

| Tool | Purpose |
|---|---|
| `cargo` | Rust compilation |
| `pnpm` | Node/SvelteKit build |
| `uv` | Python environment management |
| `tauri-cli` | Orchestrates cargo + pnpm, produces platform bundles |
| `cargo-deny` | Rust dependency audit |
| `cargo-audit` | Security vulnerability scan |

### Platform targets

| Platform | Installer format | WebView |
|---|---|---|
| Windows 10+ | `.msi`, `.exe` (NSIS) | WebView2 (Chromium-based) |
| macOS 11+ | `.dmg`, `.app` | WKWebView (Safari-based) |
| Linux | `.deb`, `.AppImage`, `.rpm` | WebKitGTK 4.1 |

---

## Security Boundary

Tauri 2's capability system enforces what the WebView can access. The frontend is sandboxed:

- No filesystem access except via explicit Tauri commands
- No direct Python calls — all computation goes through typed Rust commands
- No `eval()` or dynamic code injection from the frontend side
- CSP enforced on all WebView content

This matters for Awald specifically: user scripts execute in the Python runtime (Layer 3), not in the WebView. A malicious script can only affect the Python process, not the OS or the UI.

---

## Key Design Decisions

### Why Tauri over Electron?

Electron bundles Chromium (~150MB). Tauri uses the system WebView (~3–8MB binary). For a statistical tool targeting researchers at universities with IT-locked machines, a small, audited binary is a meaningful advantage.

### Why Rust owns execution order?

Python cannot reliably supervise its own execution model. The GIL, `asyncio`, and Jupyter's kernel model all introduce ordering ambiguities. Rust holding a `Mutex` over the Python interpreter guarantees sequential execution without exception. This is the property that makes `.do` files trustworthy — and Awald must have it.

### Why Polars over pandas for the data layer?

Polars stores data in Apache Arrow columnar format. A Rust layer can slice it zero-copy — no serialization, no allocation, no GIL crossing for viewport rendering. Pandas does not support this. For datasets with millions of rows, the difference between a responsive data editor and a laggy one is entirely this boundary.

### Why tidyecon for output?

tidyecon provides the `broom`/`modelsummary` protocol that Python's econometrics ecosystem lacks. It is the output layer that Stata's `esttab` occupies. Building it separately (at `github.com/SHA888/tidyecon`) means it can be adopted independently by researchers who never use the Awald desktop app — seeding the ecosystem before the app is ready.

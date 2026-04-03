# TODO

Full development roadmap for Awald. Follows [SemVer 2.0.0](https://semver.org).
Status: `[ ]` open · `[x]` done · `[~]` in progress · `[-]` deferred

Pre-release versions (`0.x.y`) allow breaking changes between minors.
`1.0.0` is the first stable public release — API frozen, installer published.

---

## Current: Pre-Alpha (`0.0.x`)

Repository scaffolding. Nothing is runnable yet.

### Licensing — confirmed
- [x] License decision: **MIT across all repositories**
- [x] Rationale: academic trust requires auditable code; moat is ecosystem not source

### Infrastructure
- [x] Domain — `awald.app` registered
- [x] GitHub org — `github.com/awald-app` created
- [x] Create repos:
  - [x] `github.com/awald-app/awald` — desktop shell (this repo)
  - [ ] `github.com/awald-app/awald-core` — Rust data layer + executor
  - [ ] `github.com/awald-app/awald-docs` — documentation site
  - [ ] `github.com/SHA888/tidyecon` — Python library (already built, move here)
- [x] MIT `LICENSE` in all four repos
- [x] `SECURITY.md` — vulnerability reporting policy
- [ ] Branch protection on `main` in all repos — require PR + CI pass

### This repo (`awald`)
- [x] `README.md`
- [x] `ARCHITECTURE.md`
- [x] `TODO.md`
- [x] `CONTRIBUTING.md`
- [x] `CHANGELOG.md`
- [x] `LICENSE`
- [x] `.gitignore`
- [x] `SECURITY.md`
- [x] `.github/workflows/ci.yml`
- [x] Repository scaffold — `cargo init` + `pnpm create svelte` + Tauri 2 init
- [x] Dependabot — `cargo`, `npm`, `pip` auto-updates

---

## v0.1.0 — Runnable MVP

**Goal:** Load a CSV. Run a regression. See a result table. Nothing more.
**Target:** Internal only. Not distributed.

### 0.1.0 — Repository structure
- [x] Init Tauri 2 project with SvelteKit frontend
  - [x] `pnpm create svelte@latest src`
  - [x] `cargo tauri init`
  - [x] Verify `pnpm tauri dev` launches empty window on all three platforms
- [x] Init Python environment
  - [x] `python/pyproject.toml` with uv
  - [x] Dependencies: `pyfixest>=0.30`, `statsmodels>=0.14`, `polars>=1.0`, `tidyecon>=0.1`
  - [x] `uv sync` produces `.venv` in `python/`
- [x] pyo3 wiring
  - [x] Add `pyo3` to `src-tauri/Cargo.toml` with `auto-initialize` feature
  - [x] Verify Python interpreter boots from embedded venv on all platforms
  - [x] Simple smoke test: `Python::with_gil(|py| py.eval("1+1"))` returns `2`

### 0.1.0 — Data loading
- [x] `DataStore` struct — `Arc<RwLock<polars::DataFrame>>`
- [x] Tauri command: `load_file(path: String) -> Result<DatasetMeta>`
  - [x] Supports `.csv` via `CsvReadOptions`
  - [x] Returns `{nrows, ncols, schema: [{name, dtype}]}`
- [x] Frontend: file picker dialog (`@tauri-apps/plugin-dialog`)
- [x] Frontend: displays `{filename} — {nrows} rows × {ncols} cols` in status bar
- [x] pyo3 upgrade: v0.22 → v0.28 with modern `Python::attach()` API
  - [x] Updated dependencies: pyo3 0.28, polars 0.53
  - [x] Migrated smoke test to use `Python::attach()` and `CString`
  - [x] All tests pass, smoke test prints "Python smoke test result: 2"

### 0.1.0 — Minimal data view
- [x] Tauri command: `get_rows(start: usize, end: usize) -> Result<Vec<RowData>>`
  - [x] Returns JSON-serialized slice of DataFrame
  - [x] No pagination logic yet — just returns requested range
- [x] Frontend: static HTML table (no virtual scroll yet)
  - [x] Renders first 100 rows
  - [x] Column headers from schema
  - [x] Responsive table with sticky headers and hover effects

### 0.1.0 — Script execution
- [ ] `Executor` struct with `Mutex<()>` GIL guard
- [ ] Persistent Python globals dict across executions
- [ ] Tauri command: `execute_script(script: String) -> Result<ExecutionResult>`
  - [ ] `exec(script, globals)` via pyo3
  - [ ] Captures stdout/stderr
  - [ ] Returns `{output: String, error: Option<String>, duration_ms: u64}`
- [ ] Frontend: `<textarea>` for script input
- [ ] Frontend: Run button → calls `execute_script`
- [ ] Frontend: displays raw text output below editor

### 0.1.0 — First regression
- [ ] Python runtime module: `python/awald_runtime/__init__.py`
  - [ ] `load(path) -> polars.DataFrame`
  - [ ] `reg(formula, data) -> statsmodels result`
  - [ ] `table(models, **kwargs) -> str (HTML)`  via tidyecon
- [ ] End-to-end test: script loads CSV, runs `aw.reg()`, calls `aw.table()`, output renders in frontend
- [ ] Results pane: renders HTML string returned from `table()` via `{@html}` in Svelte

### 0.1.0 — CI
- [ ] GitHub Actions: `.github/workflows/ci.yml`
  - [ ] Matrix: `ubuntu-latest`, `macos-latest`, `windows-latest`
  - [ ] Steps: `pnpm install` → `uv sync` → `cargo test` → `pnpm tauri build`
  - [ ] Cache: cargo registry, pnpm store, uv cache
- [ ] `cargo test` — all Rust unit tests pass
- [ ] `cargo clippy -- -D warnings` — zero warnings
- [ ] `cargo audit` — zero known vulnerabilities

---

## v0.2.0 — Data Editor

**Goal:** The data editor works as a first-class spreadsheet view.
**Target:** Internal only.

### 0.2.0 — Virtual scroll table
- [ ] `VirtualTable` Svelte component
  - [ ] Renders only rows within visible viewport (±50px buffer)
  - [ ] Row height: fixed 28px (configurable later)
  - [ ] Scroll events trigger `get_rows(start, end)` Tauri command
  - [ ] Debounced — max one in-flight request at a time
- [ ] Performance target: scroll through 1M row dataset at 60fps on mid-range laptop
- [ ] Column width: auto-fit to content, resizable via drag

### 0.2.0 — Data formats
- [ ] `.parquet` support — `ParquetReadOptions` via Polars
- [ ] `.dta` (Stata) support — Polars `read_stata()`
  - [ ] Verify value labels preserved as string categories
  - [ ] Verify date/datetime columns parsed correctly
- [ ] `.xlsx` support — `polars-io` Excel reader
- [ ] Format auto-detection from file extension
- [ ] Error handling: unsupported format returns user-visible message

### 0.2.0 — Variable browser
- [ ] `VariableBrowser` Svelte component
  - [ ] Lists all columns: name, dtype, null count, sample values
  - [ ] Sortable by name, dtype
  - [ ] Click column → highlights in data editor

### 0.2.0 — Summary statistics
- [ ] Tauri command: `get_variable_summary(name: String) -> Result<VarSummary>`
  - [ ] Numeric: min, max, mean, std, p25, p50, p75, null count
  - [ ] String/categorical: value counts (top 10), null count
- [ ] Frontend: click variable in browser → shows summary panel

---

## v0.3.0 — Script Editor

**Goal:** The script editor is a first-class environment with linear execution and inline output.

### 0.3.0 — Editor
- [ ] Replace `<textarea>` with CodeMirror 6
  - [ ] Python syntax highlighting
  - [ ] Line numbers
  - [ ] Basic autocomplete (variable names in scope)
- [ ] Script persistence: auto-save to `~/.awald/sessions/{id}.py`
- [ ] Recent scripts: list in sidebar

### 0.3.0 — Block-based execution
- [ ] Script parser: splits on blank lines into logical blocks
- [ ] `execute_block(block_index: usize)` — run single block
- [ ] `execute_all()` — run all blocks sequentially, halt on error
- [ ] Execution state per block: `pending | running | success | error`
- [ ] Frontend: block status indicators inline in editor gutter
- [ ] Frontend: error messages inline below the offending block

### 0.3.0 — Inline output
- [ ] Results rendered inline below each block that produces output
- [ ] HTML tables (from `tidyecon`) rendered via `{@html}`
- [ ] Text output rendered in monospace block
- [ ] Errors rendered with red styling and traceback

### 0.3.0 — Session state
- [ ] Variables in Python namespace listed in sidebar
- [ ] DataFrames in namespace synced to DataStore (accessible in data editor)
- [ ] `clear()` command resets namespace and results

---

## v0.4.0 — Full Econometrics

**Goal:** All core Stata use cases covered.

### 0.4.0 — pyfixest integration
- [ ] `aw.reg()` routes to `pyfixest.feols()` when FE syntax detected
  - [ ] Formula: `"y ~ x | fe"` → fixed effects
  - [ ] Formula: `"y ~ 1 | fe | x ~ z"` → IV
  - [ ] `vcov` parameter: `"iid"`, `"HC1"`, `{"CRV1": "cluster_var"}`
- [ ] `aw.poisson()` → `pyfixest.fepois()`
- [ ] Multi-model estimation: `aw.reg(["m1_formula", "m2_formula"], data=df)`

### 0.4.0 — Survey estimation
- [ ] `aw.svydesign(data, strata, psu, weights)` → `svy.SurveyDesign`
- [ ] `aw.svyreg(formula, design)` → survey-weighted regression
- [ ] `aw.svymean(vars, design)` → design-based means with correct SEs

### 0.4.0 — Causal inference
- [ ] DiD: `aw.did(formula, data, time, unit, treat)` → `pyfixest` Callaway-Sant'Anna
- [ ] RDD: `aw.rdd(formula, data, cutoff, bandwidth)` → `rdrobust` wrapper
- [ ] Synthetic control: deferred to v0.5.0

### 0.4.0 — Postestimation
- [ ] `aw.test(model, "x1 = x2")` → Wald test via pyfixest
- [ ] `aw.margins(model)` → average marginal effects
- [ ] `aw.predict(model, data)` → out-of-sample predictions

---

## v0.5.0 — Output & Export

**Goal:** Every output format needed for journal submission.

### 0.5.0 — tidyecon integration
- [ ] `aw.table()` wraps `tidyecon.modelsummary()` with Awald defaults
  - [ ] Default star convention: `* p<0.1  ** p<0.05  *** p<0.01`
  - [ ] Default statistic: SE in parentheses
  - [ ] Default GOF rows: N, R², fixed effects, vcov type
- [ ] `output="html"` — renders inline in results pane
- [ ] `output="path/to/file.tex"` — writes LaTeX, shows preview
- [ ] `output="path/to/file.docx"` — writes Word, opens in system viewer
- [ ] `output="path/to/file.html"` — writes HTML, opens in browser

### 0.5.0 — Graph output
- [ ] `aw.plot(model)` → coefficient plot (plotly or vega-lite)
- [ ] `aw.scatter(x, y, data)` → scatter with optional regression line
- [ ] Graphs saved as SVG/PNG from results pane
- [ ] Deferred: full ggplot2-equivalent — that is a separate milestone

### 0.5.0 — Script export
- [ ] Export current session as standalone `.py` script
- [ ] Export current session as Quarto `.qmd` document
  - [ ] Embeds results inline as HTML
  - [ ] Renders to PDF/HTML via Quarto CLI if installed

---

## v0.6.0 — Polish & Performance

### 0.6.0 — Performance
- [ ] Profile data editor scroll performance on 10M row dataset
  - [ ] Target: <16ms per frame (60fps) on 8GB RAM laptop
- [ ] Profile Python executor warm-up time
  - [ ] Target: first execution <2s after cold start
  - [ ] Subsequent: <100ms for simple regressions
- [ ] Memory profiling: verify no full DataFrame copies in Rust↔Python boundary

### 0.6.0 — UX
- [ ] Keyboard shortcuts: `Cmd/Ctrl+Enter` to run block, `Shift+Enter` to run all
- [ ] Dark mode (follows system preference)
- [ ] Font size adjustable in preferences
- [ ] Column freeze in data editor
- [ ] Find & replace in script editor

### 0.6.0 — Error handling
- [ ] All Python errors surfaced with actionable messages
- [ ] Common Stata→Awald translation hints (e.g. `reghdfe` → `aw.reg(...|fe)`)
- [ ] File format errors with format suggestions
- [ ] Memory warnings for large datasets

---

## v0.7.0 — Packaging & Distribution

### 0.7.0 — Installers
- [ ] Windows: `.msi` via Tauri bundler, signed
- [ ] macOS: `.dmg` with notarization, universal binary (x86_64 + arm64)
- [ ] Linux: `.deb`, `.AppImage`, `.rpm`
- [ ] Auto-updater: `tauri-plugin-updater`, checks `awald.app/releases/latest`

### 0.7.0 — Python bundling
- [ ] Python runtime bundled with installer (no separate Python install required)
- [ ] Strategy: embed uv-managed `.venv` in Tauri resource directory
- [ ] Test: fresh Windows/macOS/Linux VM with no Python installed → app launches
- [ ] Incremental updates: only re-download changed Python packages

### 0.7.0 — Update infrastructure
- [ ] `awald.app/releases/latest.json` — Tauri updater endpoint
- [ ] GitHub Releases as source of truth
- [ ] Signature verification via Tauri's built-in signing

---

## v0.8.0 — Documentation & Onboarding

- [ ] In-app tutorial (first run): load sample dataset → run regression → see table
- [ ] Sample datasets bundled: `nlsw88.csv`, `auto.csv` (Stata classics)
- [ ] `awald.app` documentation site — MkDocs Material
  - [ ] Getting started guide
  - [ ] API reference (`aw.*` commands)
  - [ ] Migration guide from Stata
  - [ ] Comparison table: Stata command → Awald equivalent
- [ ] Video: 5-minute "first regression" screencast

---

## v0.9.0 — Beta

**Goal:** Feature-complete. Seeking feedback from PhD students and researchers.
**Distribution:** Public beta, opt-in auto-updates.

- [ ] Recruit 10–20 beta users from PhD programs
- [ ] Structured feedback form embedded in app
- [ ] Bug bash: 2-week focused bug fixing sprint
- [ ] Performance regression tests in CI
- [ ] Accessibility audit (keyboard navigation, screen reader)
- [ ] Security audit: pyo3 boundary, Tauri capabilities, Python script execution

---

## v1.0.0 — First Stable Release

**Criteria (all must be met):**
- [ ] All v0.1.0 through v0.9.0 milestones complete
- [ ] Zero open P0/P1 bugs
- [ ] Installers tested on clean VMs: Windows 10+, macOS 12+, Ubuntu 22.04+
- [ ] Auto-updater verified working
- [ ] Documentation site live at `awald.app/docs`
- [ ] PyPI: `pip install tidyecon` produces correct output for all backends
- [ ] Announcement: Hacker News, Twitter/X, Economics Twitter, Reddit r/econometrics
- [ ] Paper or preprint: "Awald: An Open Statistical Computing Environment for Applied Economics"

---

## Post-1.0 Backlog

### Product extensions
- [ ] **v1.1.0** — Panel data wizard (guided DiD setup)
- [ ] **v1.2.0** — Collaboration: share session as `.awald` bundle (data + script + results)
- [ ] **v1.3.0** — Plugin system: user-installable `aw.*` command extensions
- [ ] **v1.4.0** — LLM integration: natural language → `aw.*` command suggestion
- [ ] **v2.0.0** — Mobile companion (iOS/Android) for viewing results

### Hosted services (business model)
These are built on top of the open software. The code remains MIT.

- [ ] **Awald Cloud** — run scripts without local install
  - [ ] Web interface: upload data, write script, get results
  - [ ] Target: students on IT-locked university machines
  - [ ] Pricing: free tier (small datasets), paid for larger
- [ ] **Replication Archive** — permanent rendered replication packages
  - [ ] Upload: `.awald` bundle → rendered HTML results with permanent URL
  - [ ] DOI integration for journal citation
  - [ ] Journal submission workflow (AEA data editor compatible)
- [ ] **Enterprise support** — SLA-backed support for institutional deployments
  - [ ] World Bank, IMF, central banks, government statistical offices
  - [ ] Custom `aw.*` extensions for institutional data sources
  - [ ] On-premises deployment option

---

## Deferred (explicitly out of scope for v1.0.0)

- [-] Time series GUI (ARIMA, VAR) — use statsmodels directly in script
- [-] Bayesian estimation GUI — deferred, use PyMC/ArviZ in script
- [-] Real-time collaboration — deferred to post-1.0
- [-] Windows ARM native build — deferred, WebView2 x64 runs via emulation

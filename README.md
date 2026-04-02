# Awald

> Empirical research belongs to everyone.

**Awald** is open-source statistical computing for applied researchers — a transparent, reproducible environment where every result can be verified, every method can be inspected, and every researcher can
participate regardless of institution or budget.

Built on Python's econometrics ecosystem inside a native Rust shell. MIT licensed.

[![Status](https://img.shields.io/badge/status-pre--alpha-red)](https://github.com/awald-app/awald)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Open Source](https://img.shields.io/badge/open%20source-yes-brightgreen)](https://github.com/awald-app/awald)
[![Website](https://img.shields.io/badge/website-awald.app-black)](https://awald.app)

---

## Why Awald

Empirical research has long depended on proprietary tools: closed source, license-gated, expensive, and opaque at the computational level. Researchers cannot audit what the software does. Institutions in LMICs cannot afford the same methods as well-funded universities. Replication depends on who owns a license, not who has the data and the question.

The statistical methods themselves are not proprietary. The implementations have been open for years. What was missing was the environment — a fast data editor, a reproducible script runner, and output that looks like a paper.

Awald is that environment.

| Feature | Proprietary tools | Awald |
|---|---|---|
| Cost | $500–$1,600/yr | Free |
| Source code | Closed | Open (MIT) |
| Reproducibility | Installer-dependent | Standard Python packages |
| LMIC access | License-gated | Unrestricted |
| Auditability | Black box | Full source |
| Output | Proprietary formats | HTML / LaTeX / Word |

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│  Tauri 2 shell  (Rust)                              │
│                                                      │
│  ┌──────────────┐   ┌───────────────────────────┐  │
│  │ Data Editor  │   │ Script Editor             │  │
│  │              │   │ (CodeMirror in WebView)   │  │
│  │ Virtual      │   │         ↓                 │  │
│  │ scroll table │   │ Rust executor             │  │
│  │      ↑       │   │ (linear, ordered)         │  │
│  │ Polars slice │   │         ↓                 │  │
│  │ (zero-copy)  │   │ pyo3 → Python             │  │
│  └──────────────┘   │ pyfixest / statsmodels    │  │
│                      │ tidyecon                  │  │
│  ┌──────────────┐   │         ↓                 │  │
│  │ Results pane │←──│ HTML table output         │  │
│  │ (WebView)    │   └───────────────────────────┘  │
│  └──────────────┘                                   │
└─────────────────────────────────────────────────────┘
```

Full architecture details in [`ARCHITECTURE.md`](ARCHITECTURE.md).

---

## Tech Stack

| Layer | Technology | Version |
|---|---|---|
| Desktop shell | [Tauri 2](https://v2.tauri.app) | 2.10.x |
| UI framework | [SvelteKit](https://kit.svelte.dev) | 2.x |
| Core language | Rust | 1.82+ (stable) |
| Python bridge | [pyo3](https://pyo3.rs) | 0.24.x |
| Data engine | [Polars](https://pola.rs) | 1.x |
| Regression | [pyfixest](https://github.com/py-econometrics/pyfixest) | 0.30.x |
| Statistics | [statsmodels](https://www.statsmodels.org) | 0.14.x |
| Output tables | [tidyecon](https://github.com/SHA888/tidyecon) | 0.1.x |
| Python runtime | Python | 3.12+ |
| Python packaging | [uv](https://docs.astral.sh/uv/) | latest |
| JS packaging | [pnpm](https://pnpm.io) | 9.x |
| Rust toolchain | [rustup](https://rustup.rs) | stable |

---

## Development Setup

### Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# Node / pnpm
corepack enable
corepack prepare pnpm@latest --activate

# Python / uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Tauri CLI
cargo install tauri-cli --version "^2"

# System dependencies (Linux)
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev \
  librsvg2-dev patchelf libxdo-dev
```

### Clone and run

```bash
git clone https://github.com/awald-app/awald
cd awald

# Install JS dependencies
pnpm install

# Install Python dependencies
uv sync

# Run in development mode
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

Produces platform-native installers in `src-tauri/target/release/bundle/`.

---

## Repository Structure

```
awald/
├── src/                    # SvelteKit frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # Svelte stores (session state)
│   │   └── types/          # TypeScript interfaces
│   └── routes/             # SvelteKit pages
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/       # Tauri command handlers
│   │   ├── engine/         # Python executor (pyo3)
│   │   ├── data/           # Polars data layer
│   │   └── session/        # Session state management
│   ├── Cargo.toml
│   └── tauri.conf.json
├── python/                 # Bundled Python environment
│   ├── pyproject.toml      # uv-managed
│   └── awald_runtime/      # Python runtime module
├── scripts/                # Dev tooling
├── tests/
│   ├── e2e/                # End-to-end (WebdriverIO)
│   └── unit/               # Rust unit tests
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── TODO.md
└── LICENSE
```

---

## Versioning

Awald follows [Semantic Versioning 2.0.0](https://semver.org).

- `0.x.y` — pre-release, breaking changes allowed between minor versions
- `1.0.0` — first stable release, public API frozen
- Patch releases (`x.y.Z`) — bug fixes only, no new features
- Minor releases (`x.Y.0`) — new features, backward compatible within major
- Major releases (`X.0.0`) — breaking changes

Current status: **pre-alpha (`0.0.x`)** — not yet usable.
Roadmap to `1.0.0` tracked in [`TODO.md`](TODO.md).

---

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). The highest-value contributions right now are:

1. **Tauri command scaffolding** — Rust command handlers for data loading and script execution
2. **Virtual scroll table** — the data editor widget
3. **Python runtime bundling** — embedding a uv-managed Python environment in the Tauri bundle

---

## Ecosystem

Awald is one application in a broader open ecosystem:

```
tidyecon          Python library    pip install tidyecon
                  tidy() + glance() + modelsummary()
                  Works standalone — no desktop app required

awald-core        Rust crate        Polars data layer + pyo3 executor
                  Reusable by other statistical desktop apps

awald             Desktop app       The shell that connects everything
                  awald.app

awald-docs        Docs site         awald.app/docs
```

Researchers can adopt `tidyecon` independently — for use in scripts, Jupyter
notebooks, or Quarto documents — before ever installing the desktop app.
This seeds the ecosystem and builds institutional trust ahead of the app.

---

## Business model

The software is free. Compute and hosting are not.

Future hosted services built on Awald:
- **Awald Cloud** — run scripts without a local install (students on locked university machines)
- **Replication Archive** — publish and render replication packages with permanent URLs
- **Enterprise support** — for World Bank, IMF, central bank deployments

None of these require the code to be proprietary.
All of them require the code to be trusted.

---

## License

MIT — see [`LICENSE`](LICENSE).

Awald is fully open source. All repositories are MIT licensed.

| Repository | Description |
|---|---|
| [`awald-app/awald`](https://github.com/awald-app/awald) | This repo — desktop shell (Tauri + SvelteKit) |
| [`awald-app/awald-core`](https://github.com/awald-app/awald-core) | Rust data layer and Python executor |
| [`SHA888/tidyecon`](https://github.com/SHA888/tidyecon) | Python tidy/modelsummary library |
| [`awald-app/awald-docs`](https://github.com/awald-app/awald-docs) | Documentation site |

The code is free. Hosted compute and enterprise support are not.

Built by [Kresna Sucandra](https://kresnasucandra.com) and contributors.
Powered by [pyfixest](https://github.com/py-econometrics/pyfixest),
[Polars](https://pola.rs),
[Tauri](https://tauri.app), and
[tidyecon](https://github.com/SHA888/tidyecon).

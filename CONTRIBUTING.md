# Contributing to Awald

## Setup

```bash
git clone https://github.com/awald-app/awald
cd awald

# Prerequisites: rustup, pnpm, uv (see README.md)

pnpm install
uv sync --directory python
pnpm tauri dev
```

## Branch model

| Branch | Purpose |
|---|---|
| `main` | Stable. Protected. Requires PR + CI pass. |
| `dev` | Integration branch. PRs target here first. |
| `feat/*` | Feature branches off `dev` |
| `fix/*` | Bug fix branches |
| `chore/*` | Tooling, docs, CI |

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(data-editor): add virtual scroll for large datasets
fix(executor): release GIL on Python timeout
chore(ci): add Windows arm64 to build matrix
docs(architecture): update data flow diagram
```

Types: `feat` · `fix` · `docs` · `chore` · `test` · `perf` · `refactor`

Breaking changes: append `!` — `feat(api)!: rename execute_script command`

## PR checklist

- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `pnpm check` (Svelte type check) passes
- [ ] New features have tests
- [ ] `TODO.md` updated if milestone item completed
- [ ] `CHANGELOG.md` entry added under `[Unreleased]`

## Where to start

The highest-value open tasks are in `TODO.md` under **v0.1.0**.
Specifically:
1. Tauri 2 + SvelteKit scaffold with pyo3 smoke test
2. `DataStore` struct with `load_file` command
3. `Executor` struct with `execute_script` command

If you are a researcher (not a developer) the most valuable contribution is
opening issues describing Stata workflows you need Awald to support.

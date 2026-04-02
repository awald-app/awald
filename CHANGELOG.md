# Changelog

All notable changes to Awald are documented here.
Format: [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).
Versioning: [SemVer 2.0.0](https://semver.org).

---

## [Unreleased]

### Added
- `README.md` — project description, stack, setup instructions
- `ARCHITECTURE.md` — system design, component map, data flows
- `TODO.md` — full SDLC roadmap v0.0.x → v1.0.0
- `CONTRIBUTING.md` — branch model, commit conventions, PR checklist
- `CHANGELOG.md` — this file
- Domain `awald.app` registered
- GitHub org `github.com/awald-app` created

### Changed
- **pyo3**: upgraded from v0.22 to v0.28 with modern `Python::attach()` API
- **polars**: upgraded from v0.44 to v0.53 for pyo3 0.28 compatibility
- Updated smoke test to use `CString` for Python code evaluation
- Updated README.md tech stack table with current versions

### Fixed
- pyo3 smoke test now works with modern API patterns
- Dependabot config updated to allow pyo3 patch updates (0.28.x)

---

## [0.1.0] — TBD

First runnable version. See TODO.md for acceptance criteria.

---

## [1.0.0] — TBD

First stable public release. See TODO.md for release checklist.

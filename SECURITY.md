# Security Policy

## Supported versions

| Version | Supported |
|---|---|
| `1.x.x` | Yes |
| `0.x.x` | Best effort |

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Email: `security@awald.app`

Include:
- Description of the vulnerability
- Steps to reproduce
- Affected version(s)
- Potential impact

You will receive acknowledgement within 48 hours.
Critical vulnerabilities are patched and released within 7 days.

## Scope

In scope:
- Python script execution sandbox escape
- Tauri capability bypass (WebView accessing filesystem without command)
- pyo3 boundary vulnerabilities
- Malicious `.dta`/`.csv`/`.parquet` file processing

Out of scope:
- Social engineering
- Issues in upstream dependencies (report to respective maintainers)
- Vulnerabilities requiring physical access to the machine

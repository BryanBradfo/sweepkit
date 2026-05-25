# 🧹 Sweepkit — Unused Dev Dependencies Scanner & Cleaner

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/8fd9aef2-806f-43d2-93de-4017475ab361">
        <img src="https://github.com/user-attachments/assets/8fd9aef2-806f-43d2-93de-4017475ab361" alt="Sweepkit" width="500">
    </picture>
</p>

<p align="center">
  <a href="https://github.com/BryanBradfo/sweepkit/actions/workflows/ci.yml?branch=main"><img src="https://img.shields.io/github/actions/workflow/status/BryanBradfo/sweepkit/ci.yml?branch=main&style=for-the-badge" alt="CI status"></a>
  <a href="https://github.com/BryanBradfo/sweepkit/releases"><img src="https://img.shields.io/github/v/release/BryanBradfo/sweepkit?include_prereleases&style=for-the-badge" alt="GitHub release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge" alt="MIT License"></a>
</p>

**Blazing-fast CLI tool to scan and clean unused dev dependencies across all ecosystems. Built in Rust 🦀 for maximum performance.**

---

## The Problem

As developers, we accumulate **gigabytes** of dev dependencies over time:
- `node_modules` folders scattered everywhere
- Python `__pycache__`, `.venv`, `.tox` directories
- Rust `target` build folders
- Gradle `.gradle` and `build` directories
- ...and many more

**Sweepkit** finds them all, shows you how much space they're eating, and lets you clean them up — fast.

## Features

- **Multi-ecosystem scanning** — Node.js, Python, Rust, Java/Gradle, C++, and more
- **Per-language filtering** — Focus on specific ecosystems with `--language` flag
- **Size analysis** — See exactly how much space each directory is wasting
- **Interactive cleanup** — Select what to delete with checkboxes
- **Blazing fast** — Built in Rust for lightning-speed filesystem traversal
- **Cross-platform** — Works on Windows, macOS, and Linux
- **Safe by default** — Always asks for confirmation, supports dry-run mode
- **Extensible** — Easy to add new language modules

## Installation

### From source (requires Rust toolchain)
```bash
git clone https://github.com/BryanBradfo/sweepkit.git
cd sweepkit
cargo install --path .
```

### From crates.io (coming soon)
```bash
cargo install sweepkit
```

## Usage

### Scan the current directory
```bash
sweepkit scan
```

### Scan a specific directory
```bash
sweepkit scan --path ~/projects
```

### Filter by language/ecosystem
```bash
# Only scan for Python dependencies
sweepkit scan --language python

# Only scan for Node.js dependencies
sweepkit scan --language node

# Only scan for Rust dependencies
sweepkit scan --language rust
```

### Clean up (interactive)
```bash
sweepkit clean
```

### Clean with language filter
```bash
# Only clean Python dependencies
sweepkit clean --language python
```

### Clean with dry-run (preview only)
```bash
sweepkit clean --dry-run
```

### Clean everything without confirmation
```bash
sweepkit clean --all
```

## Supported Ecosystems

SweepKit uses a modular architecture with dedicated language modules for each ecosystem. Each module provides smart detection with context-aware scanning.

| Ecosystem | Icon | Directories Detected |
|-----------|------|---------------------|
| 🐍 Python | `python` | `__pycache__`, `.venv`, `venv`, `env`, `.tox`, `.pytest_cache`, `.mypy_cache`, `.ruff_cache`, `*.egg-info`, `dist` (with `setup.py`) |
| 🟢 Node.js | `node` | `node_modules`, `.next`, `.nuxt`, `.parcel-cache`, `dist` (with `package.json`), `bower_components` |
| 🦀 Rust | `rust` | `target` (with `Cargo.toml`) |
| ☕ Java | `java` | `.gradle`, `build` (with `build.gradle`, `build.gradle.kts`, or `pom.xml`) |
| ⚙️ C++ | `cpp` | `build` (with `CMakeLists.txt` or `Makefile`), `cmake-build-*`, `out` (with `CMakeLists.txt`) |

### Context-Aware Detection

Sweepkit is smart about what it detects:
- **Sibling file validation**: Only detects `target` when `Cargo.toml` is present, `build` when build files exist, etc.
- **No false positives**: Won't flag generic directory names without proper context
- **Glob patterns**: Supports patterns like `*.egg-info` for Python packages

## Architecture

Sweepkit features a **modular, extensible architecture**:

```
src/
├── languages/          # Language-specific modules
│   ├── mod.rs         # LanguageCleaner trait definition
│   ├── python.rs      # Python ecosystem
│   ├── node.rs        # Node.js ecosystem
│   ├── rust_lang.rs   # Rust ecosystem
│   ├── java.rs        # Java/Gradle ecosystem
│   └── cpp.rs         # C/C++ ecosystem
├── scanner.rs         # Filesystem scanning engine
├── cleaner.rs         # Interactive cleanup logic
├── utils.rs           # Display & formatting utilities
└── main.rs            # CLI interface
```

### Adding a New Language

To add support for a new language:

1. Create a new file in `src/languages/` (e.g., `go.rs`)
2. Implement the `LanguageCleaner` trait
3. Register it in `src/languages/mod.rs` in the `get_all_cleaners()` function

Example:
```rust
pub struct GoCleaner;

impl LanguageCleaner for GoCleaner {
    fn name(&self) -> &str { "Go" }
    fn icon(&self) -> &str { "🐹" }
    
    fn project_patterns(&self) -> Vec<DetectionPattern> {
        vec![
            DetectionPattern::DirectoryName("vendor".to_string()),
            DetectionPattern::DirectoryWithSibling {
                dir_name: "bin".to_string(),
                sibling: "go.mod".to_string(),
            },
        ]
    }
    
    fn global_cache_paths(&self) -> Vec<GlobalCachePath> {
        // Implementation for global Go cache
        vec![]
    }
}
```

## Roadmap

- [x] Core scanning engine
- [x] Interactive cleanup with confirmations
- [x] Per-language cleaner modules
- [x] Language filtering (`--language` flag)
- [ ] Global cache scanning (pip, npm, cargo, etc.)
- [ ] Orphaned package detection
- [ ] Docker image cleanup integration
- [ ] Scheduled scanning (cron/Task Scheduler)
- [ ] TUI dashboard with charts
- [ ] Config file (`.sweepkit.toml`) for custom ignore rules
- [ ] GitHub Actions integration for CI cache cleanup

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Star this repo!

[![Star History Chart](https://api.star-history.com/svg?repos=bryanbradfo/sweepkit&type=date&legend=top-left)](https://www.star-history.com/#bryanbradfo/sweepkit&type=date&legend=top-left)

If you find this useful, give it a ⭐ — it helps others discover the project!

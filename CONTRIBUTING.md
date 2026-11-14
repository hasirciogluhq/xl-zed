# Contributing

This repository is the Zed extension for **xlang** (`.xl`). It stands as the
`xl-zed/` git submodule of [hasirciogluhq/xlang](https://github.com/hasirciogluhq/xlang)
and references nothing of that repository's tree.

## Setup

- [Zed](https://zed.dev) with the extension dev tools, and the Rust toolchain
  (the extension API is `zed_extension_api`).
- A build check: `cargo check`.
- A manual check: install the extension as a dev extension from Zed
  (*extensions → install dev extension*) pointed at this directory.

## Rules

- Repo content is English (code, comments, docs, commits).
- The grammar (`languages/xlang/xlang.tmLanguage.json`) and the language config
  (`languages/xlang/config.toml`) stay in step with the language: a new keyword,
  builtin, attribute, role or diagnostic code updates the grammar here.
- Language documentation lives in [xlang-docs](https://github.com/hasirciogluhq/xlang-docs);
  this extension is documented only in this repository's `README.md`.
- **Never add `Co-Authored-By` trailers, "Authored by Claude" or any "Generated with Claude Code"**
  line to commits, PRs or files in this repository.
- A change here is pushed first; the main repo then commits the updated
  submodule pointer.

## License

By contributing you agree that your contribution is licensed under the
Apache License, Version 2.0 ([LICENSE](LICENSE)).

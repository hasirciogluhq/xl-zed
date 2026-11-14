# xl-zed — notes for Claude

## What this repo is
- The Zed extension for **xlang** (`.xl`). It stands as the `xl-zed/` git submodule of `hasirciogluhq/xlang` and references nothing of that repo's tree.
- **Skeleton state:** the extension manifest, the `zed::Extension` registration, the language declaration and the TextMate grammar exist. Diagnostics, formatting, tasks and a language server are not implemented.
- The grammar (`languages/xlang/xlang.tmLanguage.json`) is a copy of `xl-vscode/syntaxes/xlang.tmLanguage.json`; keep the two in step (scope `source.xlang`).

## Zed extension contract
- Rust + `zed_extension_api`. `extension.toml` holds `id`, `name`, `version`, `authors`, `repository`, `[grammars.<name>]` (repository/rev/path) and `[language_servers.<id>]`.
- `src/lib.rs` implements `zed::Extension` (`new`, `language_server_command`, and the other provided methods) and ends with `zed::register_extension!`.
- The language is declared in `languages/xlang/config.toml`: `name`, `grammar`, `path_suffixes`, comments, brackets, `tab_size`.
- A language server slot is registered in `extension.toml`; `language_server_command` currently returns an error because no xlang server exists.

## The compiler contract (later features)
- The compiler is **`xlc`**, one command: `xlc [options] main.xl [objects...] [-o out] [--emit=...]`. Type-check is `--check`; diagnostics are `--diag-format json --context 0`.
- Libraries: `-M NAME=DIR` and `--import SPEC`; stdlib `-M std=<dir> --import std -DfeatureDefaultAllocator=true`.
- The `xl` tool builds those flags; `xl env --json <file>` prints them. When behaviour is added, mirror `xl-vscode` (`environment.ts`, `project.ts`, `diagnostics.ts`, `runner.ts`) instead of inventing a second resolution.

## Layout
- `extension.toml`, `src/lib.rs`, `languages/xlang/{config.toml,xlang.tmLanguage.json}`, plus the usual OSS files.
- Docs of the language live in `xlang-docs/`; this extension is documented only in this repo's `README.md`, which must match `extension.toml` in the same change.

## Tooling
- Rust: `cargo check && cargo build`. A dev extension is installed from Zed's *Extensions → Install Dev Extension*; Zed compiles it itself.
- `target/` and `Cargo.lock` are not committed.

## Rules
- Repo content is English (code, comments, docs, commits). User chat is Turkish.
- Keep the grammar and the language config in step with the language: a new keyword, builtin, attribute, role or diagnostic code updates the grammar here.
- File edits use the built-in Read/Edit/Write tools.
- **Never add `Co-Authored-By` trailers, "Authored by Claude" or any "Generated with Claude Code"** line to commits, PRs or files in this repo.
- Commits here are pushed first; the main repo then commits the updated submodule pointer.

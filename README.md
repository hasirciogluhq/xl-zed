# xl-zed

The [Zed](https://zed.dev) extension for **xlang**, a low-level, LLVM-based
systems programming language. It is the `xl-zed/` submodule of
[hasirciogluhq/xlang](https://github.com/hasirciogluhq/xlang); the language
documentation lives in [xlang-docs](https://hasirciogluhq.github.io/xlang-docs/).

This repository is a **skeleton**: the extension is registered, the language is
declared and the grammar is wired in. The features listed as planned below are
not implemented yet.

## Status

- [x] Extension manifest (`extension.toml`) and `zed::Extension` registration
- [x] Language declaration for `.xl` files (`languages/xlang/config.toml`)
- [x] TextMate grammar, shared with [xl-vscode](https://github.com/hasirciogluhq/xl-vscode)
- [ ] Diagnostics from `xlc --check` (`--diag-format json`)
- [ ] Formatting
- [ ] Run / build / test tasks through the `xl` tool
- [ ] Go-to-definition and completion (needs a language server; `xl-vscode` parses source locally instead)

The compiler (`xlc`) and the tool (`xl`) are the authorities; the extension never
implements language semantics. See the compiler contract described in
[xl-vscode's notes](https://github.com/hasirciogluhq/xl-vscode).

## Layout

| Path | Content |
| --- | --- |
| `extension.toml` | Extension manifest: id, grammar and language-server slots |
| `src/lib.rs` | `zed::Extension` implementation (the extension entry point) |
| `languages/xlang/config.toml` | Language declaration: `.xl`, comments, brackets, indent |
| `languages/xlang/xlang.tmLanguage.json` | TextMate grammar (scope `source.xlang`) |

## Install (development)

Zed's extension marketplace does not carry `xl-zed` yet. Install it as a dev
extension: *Extensions → Install Dev Extension* and pick this directory.

## Build

```bash
cargo check      # type-check the extension
cargo build      # compile it to target/
```

Zed compiles a dev extension itself; a local `cargo build` is only a check.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).

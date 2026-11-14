//! zed::Extension implementation for xlang.
//!
//! The extension is a skeleton: it registers the language, its grammar and an
//! empty language-server slot. Behaviour (compiler discovery through `xl env
//! --json`, diagnostics, formatting, run/build/test tasks) is added later and
//! mirrors `xl-vscode` (https://github.com/hasirciogluhq/xl-vscode).

use zed_extension_api::{self as zed, Result};

/// State of the extension, kept across worktrees.
///
/// The compiler (`xlc`) and the toolchain (`xl`) are resolved per worktree, so
/// nothing discovered here outlives the project it belongs to yet.
pub struct XlangExtension {
    /// Whether the language server binary was located for the current worktree.
    pub server_found: bool,
}

impl zed::Extension for XlangExtension {
    fn new() -> Self {
        Self {
            server_found: false,
        }
    }

    /// A language server for xlang has not been written yet; `None` keeps Zed
    /// from asking for a command. When one exists it is found with
    /// `worktree.which("...")` or a release download, as in `xl-vscode`.
    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Err("no xlang language server is available yet".to_string())
    }
}

zed::register_extension!(XlangExtension);

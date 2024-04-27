use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

struct SwiftExtension {}

impl SwiftExtension {}

impl zed::Extension for SwiftExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/sourcekit-lsp".into(),
            args: vec!["--log-level".into(), "debug".into(), "--scratch-path".into(), worktree.root_path()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("sourcekit-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "sourcekit-lsp": settings
        })))
    }
}

zed::register_extension!(SwiftExtension);

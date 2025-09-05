use zed_extension_api::{
    register_extension, settings::LspSettings, Command, Extension, LanguageServerId, Result,
    Worktree,
};

struct ZubanExt;

impl Extension for ZubanExt {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let env = worktree.shell_env();

        let path = worktree
            .which("zuban")
            .ok_or_else(|| "zuban not found, must be installed and in $PATH".to_string())?;

        Ok(Command {
            command: path,
            args: vec!["server".to_string()],
            env,
        })
    }

    // ref https://github.com/zed-extensions/pyrefly/blob/main/src/pyrefly.rs
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(_language_server_id.as_ref(), _worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

register_extension!(ZubanExt);

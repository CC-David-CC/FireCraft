//! The hosted editor's file actions and project binding.

use super::{Editor, Shell, View};
use crate::menus::ShellAction;

pub(super) struct Workspace {
    identity: u64,
    request: Option<&'static str>,
    confirm_discard: fn() -> bool,
}

impl Workspace {
    pub(super) fn document_replaced(&mut self) {
        self.identity = self.identity.wrapping_add(1);
        self.request = None;
    }
}

impl Shell {
    /// Routes native File menu and shortcut actions through `WebOS` Files.
    pub fn enable_workspace_files(&mut self, confirm_discard: fn() -> bool) {
        self.workspace = Some(Workspace {
            identity: 0,
            request: None,
            confirm_discard,
        });
        self.editor.session.mark_clean();
    }

    pub(super) fn queue_workspace_action(&mut self, action: ShellAction) -> bool {
        let Some(workspace) = &mut self.workspace else {
            return false;
        };
        let request = match action {
            ShellAction::Open => "open",
            ShellAction::Import => "import",
            ShellAction::Save => "save",
            ShellAction::SaveAs => "save-as",
            _ => return false,
        };
        workspace.request = Some(request);
        true
    }

    /// Returns the workspace file operation requested by the editor.
    pub fn take_workspace_action(&mut self) -> Option<&'static str> {
        self.workspace.as_mut()?.request.take()
    }

    /// The current document's identity within this editor instance.
    #[must_use]
    pub fn workspace_identity(&self) -> Option<u64> {
        Some(self.workspace.as_ref()?.identity)
    }

    /// Asks before replacing unsaved work from inside the editor.
    #[must_use]
    pub fn workspace_allows_replace(&self) -> bool {
        self.workspace.as_ref().is_none_or(|workspace| {
            !self.editor.session.is_dirty() || (workspace.confirm_discard)()
        })
    }

    /// Opens an editable project. The raster fallback used by standalone
    /// Open would flatten a damaged project, so use the native reader.
    ///
    /// # Errors
    /// Returns the native reader error while keeping the current project.
    pub fn open_workspace_project(
        &mut self,
        name: &str,
        bytes: &[u8],
    ) -> Result<(), engine::EngineError> {
        let editor = Editor::open(bytes)?;
        self.replace_editor(editor);
        self.raster = None;
        self.raster_scale = 1.0;
        self.view = View::default();
        self.doc_name = name.to_owned();
        self.status = format!("opened {name}");
        Ok(())
    }

    /// Places a raster from Files on the active layer.
    ///
    /// # Errors
    /// Returns an import error if the editor cannot decode or place the image.
    pub fn import_workspace_image(&mut self, name: &str, bytes: &[u8]) -> Result<(), String> {
        let size = self
            .import_raster(bytes, name)
            .map_err(|error| format!("Import failed: {error}"))?;
        self.status = format!("Imported {name} ({size})");
        Ok(())
    }

    /// Applies a verified save receipt for the current document.
    pub fn confirm_workspace_save(&mut self, name: &str) {
        self.editor.session.mark_clean();
        self.doc_name = name.to_owned();
    }
}

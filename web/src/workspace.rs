//! Same-origin `WebOS` Files bridge. The host owns paths and writes.

use std::{cell::OnceCell, rc::Rc};

use app::shell::Shell;
use wasm_bindgen::prelude::*;

thread_local! {
    static RUNNER: OnceCell<Rc<eframe::WebRunner>> = const { OnceCell::new() };
}

pub(super) fn register(runner: Rc<eframe::WebRunner>) {
    RUNNER.with(|slot| {
        let _ = slot.set(runner);
    });
}

pub(super) fn confirm_discard() -> bool {
    web_sys::window().is_some_and(|window| {
        window
            .confirm_with_message("Discard unsaved changes to this FireCraft project?")
            .unwrap_or(false)
    })
}

pub(super) fn load_local_file(shell: &mut Shell, name: &str, bytes: &[u8]) {
    if name.to_ascii_lowercase().ends_with(".fw.png") {
        if !shell.workspace_allows_replace() {
            shell.status = "Kept the current project".to_owned();
        } else if let Err(error) = shell.open_workspace_project(name, bytes) {
            shell.status = format!("Open failed: {error}");
        }
    } else if let Err(error) = shell.import_workspace_image(name, bytes) {
        shell.status = error;
    }
}

fn with_shell<T>(f: impl FnOnce(&mut Shell) -> T) -> Option<T> {
    RUNNER.with(|slot| {
        let mut shell = slot.get()?.app_mut::<Shell>()?;
        Some(f(&mut shell))
    })
}

fn not_running() -> JsValue {
    JsValue::from_str("FireCraft is starting.")
}

fn revision(shell: &Shell) -> String {
    format!(
        "{}:{}",
        shell.workspace_identity().unwrap_or_default(),
        shell.editor.session.revision(),
    )
}

#[wasm_bindgen]
pub fn workspace_ready() -> bool {
    RUNNER.with(|slot| slot.get().is_some())
}

#[wasm_bindgen]
pub fn project_identity() -> String {
    with_shell(|shell| shell.workspace_identity())
        .flatten()
        .map_or_else(String::new, |identity| identity.to_string())
}

#[wasm_bindgen]
pub fn project_dirty() -> bool {
    with_shell(|shell| shell.editor.session.is_dirty()).unwrap_or(false)
}

#[wasm_bindgen]
pub fn take_file_request() -> Option<String> {
    with_shell(|shell| shell.take_workspace_action().map(str::to_owned)).flatten()
}

#[wasm_bindgen]
pub fn workspace_status(message: &str) {
    with_shell(|shell| {
        shell.status = message.to_owned();
        shell.request_repaint();
    });
}

#[wasm_bindgen]
pub fn open_project(name: &str, bytes: &[u8]) -> Result<bool, JsValue> {
    with_shell(|shell| {
        shell
            .open_workspace_project(name, bytes)
            .map_err(|error| JsValue::from_str(&format!("Open failed: {error}")))?;
        shell.request_repaint();
        Ok(true)
    })
    .ok_or_else(not_running)?
}

#[wasm_bindgen]
pub fn import_file(name: &str, bytes: &[u8]) -> Result<bool, JsValue> {
    with_shell(|shell| {
        shell
            .import_workspace_image(name, bytes)
            .map_err(|error| JsValue::from_str(&error))?;
        shell.request_repaint();
        Ok(true)
    })
    .ok_or_else(not_running)?
}

#[wasm_bindgen]
pub fn project_snapshot() -> Result<JsValue, JsValue> {
    let (saved_revision, bytes) =
        with_shell(|shell| shell.editor.save().map(|bytes| (revision(shell), bytes)))
            .ok_or_else(not_running)?
            .map_err(|error| JsValue::from_str(&format!("Save failed: {error}")))?;
    Ok(crate::reply_object(&[
        ("bytes", js_sys::Uint8Array::from(bytes.as_slice()).into()),
        ("revision", JsValue::from_str(&saved_revision)),
    ]))
}

#[wasm_bindgen]
pub fn project_saved(saved_revision: &str, name: &str) -> bool {
    with_shell(|shell| {
        if revision(shell) != saved_revision {
            return false;
        }
        shell.confirm_workspace_save(name);
        shell.request_repaint();
        true
    }) == Some(true)
}

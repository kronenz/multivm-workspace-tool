mod workset;

use workset::{CreateWorksetInput, UpdateWorksetInput, Workset, WorksetStore, WorksetSummary};

#[tauri::command]
fn list_worksets(store: tauri::State<'_, WorksetStore>) -> Vec<WorksetSummary> {
    store.list()
}

#[tauri::command]
fn get_workset(id: String, store: tauri::State<'_, WorksetStore>) -> Option<Workset> {
    store.get(&id)
}

#[tauri::command]
fn create_workset(
    input: CreateWorksetInput,
    store: tauri::State<'_, WorksetStore>,
) -> Result<Workset, String> {
    store.create_result(input).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_workset(
    id: String,
    input: UpdateWorksetInput,
    store: tauri::State<'_, WorksetStore>,
) -> Result<Workset, String> {
    match store.update(&id, input) {
        Some(workset) => Ok(workset),
        None => Err(format!("workset not found: {id}")),
    }
}

#[tauri::command]
fn delete_workset(id: String, store: tauri::State<'_, WorksetStore>) -> Result<bool, String> {
    Ok(store.delete(&id))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(WorksetStore::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_worksets,
            get_workset,
            create_workset,
            update_workset,
            delete_workset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

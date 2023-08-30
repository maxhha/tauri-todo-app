// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use interactors::ProjectInteractor;
use models::Project;

mod interactors;
mod models;
mod ports;
mod repositories;
mod utils;

type Result<T> = core::result::Result<T, serde_error::Error>;

#[derive(Debug)]
pub struct AppState {
    project_interactor: ProjectInteractor,
}

#[tauri::command]
async fn create_project(name: &str, state: tauri::State<'_, AppState>) -> Result<Project> {
    state
        .project_interactor
        .create(name)
        .await
        .map_err(|e| serde_error::Error::new(&*e))
}

#[tauri::command]
async fn get_all_projects(state: tauri::State<'_, AppState>) -> Result<Vec<Project>> {
    state
        .project_interactor
        .list()
        .await
        .map_err(|e| serde_error::Error::new(&*e))
}

fn main() {
    let project_repository = Arc::new(repositories::InMemoryProjectRepository::new());

    tauri::Builder::default()
        .manage(AppState {
            project_interactor: ProjectInteractor::new(project_repository),
        })
        .invoke_handler(tauri::generate_handler![create_project, get_all_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

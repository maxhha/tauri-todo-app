// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[macro_use(defer)]
extern crate scopeguard;

use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use interactors::{GroupInteractor, ProjectInteractor};
use models::{Group, Project};
use tauri::Manager;

mod interactors;
mod models;
mod ports;
mod repositories;
mod result;
mod utils;

use result::Result;

#[derive(Debug)]
pub struct AppState {
    project_interactor: ProjectInteractor,
    group_interactor: GroupInteractor,
}

#[tauri::command]
async fn create_project(name: &str, state: tauri::State<'_, AppState>) -> Result<Project> {
    state.project_interactor.create(name).await
}

#[tauri::command]
async fn get_all_projects(state: tauri::State<'_, AppState>) -> Result<Vec<Project>> {
    state.project_interactor.list().await
}

#[tauri::command]
async fn create_group(
    name: &str,
    project_id: u64,
    state: tauri::State<'_, AppState>,
) -> Result<Group> {
    state.group_interactor.create(name, project_id).await
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path_resolver()
                .app_data_dir()
                .unwrap_or_else(|| PathBuf::from(".").join("data"));

            std::fs::create_dir_all(&app_data_dir).context(format!(
                "Failed to create app_data_dir {}",
                app_data_dir.display()
            ))?;

            let project_repository = Arc::new(repositories::ProjectRepository::new(
                &app_data_dir.join("Projects.bson"),
            ));

            let group_repository = Arc::new(repositories::GroupRepository::new());

            app.manage(AppState {
                project_interactor: ProjectInteractor::new(project_repository),
                group_interactor: GroupInteractor::new(group_repository),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_project,
            get_all_projects,
            create_group
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

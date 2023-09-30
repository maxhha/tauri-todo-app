// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[macro_use(defer)]
extern crate scopeguard;

use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use interactors::ProjectInteractor;
use models::Project;
use serde::ser::SerializeMap;
use tauri::Manager;

mod interactors;
mod models;
mod ports;
mod repositories;
mod utils;

enum AppError {
    Validation(validator::ValidationErrors),
    Unknown(serde_error::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> AppError {
        if error.is::<validator::ValidationErrors>() {
            return AppError::Validation(error.downcast().expect("downcast error failed"));
        }

        AppError::Unknown(serde_error::Error::new(&*error))
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            AppError::Validation(errors) => {
                map.serialize_entry("validation", errors)?;
            }
            AppError::Unknown(error) => {
                map.serialize_entry("unknown", error)?;
            }
        };
        map.end()
    }
}

type Result<T> = core::result::Result<T, AppError>;

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
        .map_err(AppError::from)
}

#[tauri::command]
async fn get_all_projects(state: tauri::State<'_, AppState>) -> Result<Vec<Project>> {
    state
        .project_interactor
        .list()
        .await
        .map_err(AppError::from)
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

            let project_repository = Arc::new(repositories::project::ProjectRepository::new(
                &app_data_dir.join("Projects.bson"),
            ));

            app.manage(AppState {
                project_interactor: ProjectInteractor::new(project_repository),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_project, get_all_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

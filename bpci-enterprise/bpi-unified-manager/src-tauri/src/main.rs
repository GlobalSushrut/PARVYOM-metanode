// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::State;
use serde::{Deserialize, Serialize};

// Import our unified component manager
use pravyom_enterprise::unified_manager::{
    UnifiedComponentManager, ComponentStatus, ComponentMetrics, Status, HealthStatus
};
use pravyom_enterprise::bso_k8_orchestrator::BsoK8Orchestrator;

// Application state
struct AppState {
    component_manager: Arc<UnifiedComponentManager>,
}

// Tauri commands

#[tauri::command]
async fn start_all_components(state: State<'_, AppState>) -> Result<String, String> {
    state.component_manager.start_all()
        .await
        .map(|_| "All components started successfully".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_all_components(state: State<'_, AppState>) -> Result<String, String> {
    state.component_manager.stop_all()
        .await
        .map(|_| "All components stopped successfully".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn restart_component(
    component_id: String,
    state: State<'_, AppState>
) -> Result<String, String> {
    state.component_manager.restart_component(&component_id)
        .await
        .map(|_| format!("Component {} restarted successfully", component_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_component_status(
    state: State<'_, AppState>
) -> Result<Vec<ComponentStatus>, String> {
    Ok(state.component_manager.get_all_status().await)
}

#[tauri::command]
async fn get_component_logs(
    component_id: String,
    lines: usize,
    state: State<'_, AppState>
) -> Result<Vec<String>, String> {
    state.component_manager.get_component_logs(&component_id, lines)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_component_metrics(
    component_id: String,
    state: State<'_, AppState>
) -> Result<ComponentMetrics, String> {
    state.component_manager.get_component_metrics(&component_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_component(
    component_id: String,
    state: State<'_, AppState>
) -> Result<String, String> {
    state.component_manager.start_component(&component_id)
        .await
        .map(|_| format!("Component {} started successfully", component_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_component(
    component_id: String,
    state: State<'_, AppState>
) -> Result<String, String> {
    state.component_manager.stop_component(&component_id)
        .await
        .map(|_| format!("Component {} stopped successfully", component_id))
        .map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() {
    // Initialize BSO-K8 orchestrator
    let bso_k8 = Arc::new(
        BsoK8Orchestrator::new("unified-manager-orchestrator".to_string())
            .await
            .expect("Failed to create BSO-K8 orchestrator")
    );
    
    // Initialize unified component manager
    let component_manager = Arc::new(
        UnifiedComponentManager::new(bso_k8)
            .await
            .expect("Failed to create unified component manager")
    );
    
    // Create app state
    let app_state = AppState {
        component_manager,
    };
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            start_all_components,
            stop_all_components,
            restart_component,
            get_all_component_status,
            get_component_logs,
            get_component_metrics,
            start_component,
            stop_component,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

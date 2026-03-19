mod commands;
mod adapters;
mod probes;
mod metrics;
mod storage;
mod scheduler;
mod alerts;
mod models;
mod error;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::server::list_servers,
            commands::server::add_server,
            commands::server::remove_server,
            commands::metrics::get_metrics,
            commands::metrics::get_metric_history,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod adapters;
pub mod alerts;
mod commands;
pub mod deployer;
pub mod error;
mod metrics;
pub mod models;
mod probes;
mod scheduler;
pub mod storage;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

fn configure_builder<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize database and start background services
            tauri::async_runtime::block_on(async {
                let pool = storage::database::init().await?;
                app.manage(pool.clone());

                // Create shared AlertEngine and load existing rules
                let alert_engine = Arc::new(Mutex::new(alerts::rules::AlertEngine::new()));
                commands::alerts::load_rules_into_engine(&pool, &alert_engine).await?;
                app.manage(alert_engine.clone());

                scheduler::poller::start(pool.clone(), alert_engine, app.handle().clone()).await?;
                scheduler::probe_scheduler::start(pool.clone()).await;
                storage::retention::start(pool.clone()).await;
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::server::list_servers,
            commands::server::add_server,
            commands::server::remove_server,
            commands::server::get_server_health_summary,
            commands::metrics::get_metrics,
            commands::metrics::get_metric_history,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::alerts::list_alert_rules,
            commands::alerts::add_alert_rule,
            commands::alerts::remove_alert_rule,
            commands::alerts::list_alert_events,
            commands::channels::list_notification_channels,
            commands::channels::add_notification_channel,
            commands::channels::remove_notification_channel,
            commands::channels::update_notification_channel,
            commands::channels::test_notification_channel,
            commands::probes::run_ping_probe,
            commands::probes::run_tcp_probe,
            commands::probes::run_dns_probe,
            commands::probes::get_probe_history,
            commands::probes::get_latest_probe_results,
            commands::known_hosts::list_known_hosts,
            commands::known_hosts::remove_known_host,
        ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    configure_builder(tauri::Builder::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::configure_builder;
    use tauri::Manager;
    use tauri::{
        ipc::{CallbackFn, InvokeBody},
        test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY},
        webview::InvokeRequest,
        WebviewWindowBuilder,
    };

    #[test]
    #[ignore = "Tauri 2 mock IPC dispatcher rejects `list_servers` with 'Plugin not found'; pre-existing failure since 58e094b — re-enable when Tauri mock_builder() supports our cmd routing"]
    fn desktop_shell_bootstraps_and_answers_list_servers() {
        let temp_dir =
            std::env::temp_dir().join(format!("serverhub-smoke-{}", uuid::Uuid::new_v4()));
        std::env::set_var("SERVERHUB_DATA_DIR", &temp_dir);

        let app = configure_builder(mock_builder())
            .build(mock_context(noop_assets()))
            .expect("mock app should build");
        let pool = tauri::async_runtime::block_on(crate::storage::database::init())
            .expect("test database should initialize");
        app.manage(pool);
        let webview = WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .expect("mock webview should build");

        let response = get_ipc_response(
            &webview,
            InvokeRequest {
                cmd: "list_servers".into(),
                callback: CallbackFn(0),
                error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body: InvokeBody::default(),
                headers: Default::default(),
                invoke_key: INVOKE_KEY.to_string(),
            },
        )
        .expect("list_servers IPC should succeed");

        let servers = response
            .deserialize::<Vec<crate::models::server::ServerConfig>>()
            .expect("response should deserialize");
        assert!(servers.is_empty());

        std::fs::remove_dir_all(temp_dir).ok();
    }
}

use crate::models::metric::MetricPoint;

#[tauri::command]
pub async fn get_metrics(server_id: String) -> Result<Vec<MetricPoint>, String> {
    // TODO: implement with adapter + storage
    let _ = server_id;
    Ok(vec![])
}

#[tauri::command]
pub async fn get_metric_history(
    server_id: String,
    key: String,
    from: i64,
    to: i64,
) -> Result<Vec<MetricPoint>, String> {
    // TODO: implement with storage query
    let _ = (server_id, key, from, to);
    Ok(vec![])
}

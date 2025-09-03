// MarkFly - 跨平台 Markdown 编辑器
// 基于 Tauri + Vue + ByteMD 构建

use tauri::Manager;

// Tauri 命令：获取应用信息
#[tauri::command]
fn get_app_info() -> serde_json::Value {
    serde_json::json!({
        "name": "MarkFly",
        "version": "0.1.0",
        "description": "跨平台 Markdown 编辑器"
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_info])
        .setup(|app| {
            // 应用启动时的初始化逻辑
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
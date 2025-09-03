// MarkFly - 跨平台 Markdown 编辑器
// 基于 Tauri + Vue + ByteMD 构建

use tauri::{Manager, Emitter};
use tauri::menu::{MenuBuilder, SubmenuBuilder, PredefinedMenuItem};

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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_app_info])
        .setup(|app| {
            println!("开始创建菜单...");
            
            // 在 macOS 上，第一个菜单会自动成为应用菜单，所以我们创建一个空的应用菜单
            #[cfg(target_os = "macos")]
            let app_menu = SubmenuBuilder::new(app, "MarkFly")
                .item(&PredefinedMenuItem::about(app, Some("关于 MarkFly"), None)?)
                .separator()
                .item(&PredefinedMenuItem::services(app, Some("服务"))?)
                .separator()
                .item(&PredefinedMenuItem::hide(app, Some("隐藏 MarkFly"))?)
                .item(&PredefinedMenuItem::hide_others(app, Some("隐藏其他"))?)
                .item(&PredefinedMenuItem::show_all(app, Some("全部显示"))?)
                .separator()
                .item(&PredefinedMenuItem::quit(app, Some("退出"))?)
                .build()?;
            
            // 创建文件菜单
            let file_menu = SubmenuBuilder::new(app, "文件")
                .text("new-file", "新建文件")
                .text("open-file", "打开文件...")
                .text("save-file", "保存文件")
                .text("save-file-as", "另存为...")
                .separator()
                .text("exit", "退出")
                .build()?;
            println!("文件菜单创建完成");
            
            // 创建编辑菜单，使用预定义项
            let edit_menu = SubmenuBuilder::new(app, "编辑")
                .item(&PredefinedMenuItem::undo(app, Some("撤销"))?)
                .item(&PredefinedMenuItem::redo(app, Some("重做"))?)
                .separator()
                .item(&PredefinedMenuItem::cut(app, Some("剪切"))?)
                .item(&PredefinedMenuItem::copy(app, Some("复制"))?)
                .item(&PredefinedMenuItem::paste(app, Some("粘贴"))?)
                .build()?;
            println!("编辑菜单创建完成");
            
            let view_menu = SubmenuBuilder::new(app, "视图")
                .text("toggle-preview", "切换预览模式")
                .text("toggle-theme", "切换主题")
                .build()?;
            println!("视图菜单创建完成");
            
            let help_menu = SubmenuBuilder::new(app, "帮助")
                .text("about", "关于 MarkFly")
                .build()?;
            println!("帮助菜单创建完成");
            
            // 在 macOS 上使用 set_as_app_menu，在其他平台上使用 window.set_menu
            #[cfg(target_os = "macos")]
            {
                let menu = MenuBuilder::new(app)
                    .item(&app_menu)  // 应用菜单作为第一个菜单
                    .item(&file_menu)
                    .item(&edit_menu)
                    .item(&view_menu)
                    .item(&help_menu)
                    .build()?;
                menu.set_as_app_menu()?;
                println!("macOS 应用菜单设置完成");
            }
            
            #[cfg(not(target_os = "macos"))]
            {
                let menu = MenuBuilder::new(app)
                    .item(&file_menu)
                    .item(&edit_menu)
                    .item(&view_menu)
                    .item(&help_menu)
                    .build()?;
                let window = app.get_webview_window("main").unwrap();
                window.set_menu(menu)?;
                println!("窗口菜单设置完成");
            }
            
            // 监听菜单事件
            app.on_menu_event(move |app_handle, event| {
                let event_id = event.id().0.as_str();
                println!("收到菜单事件: {}", event_id);
                match event_id {
                    "new-file" | "open-file" | "save-file" | "save-file-as" => {
                        // 发送事件到前端
                        let _ = app_handle.emit("menu", event_id);
                        println!("发送菜单事件到前端: {}", event_id);
                    }
                    "exit" | "quit" => {
                        println!("退出应用");
                        app_handle.exit(0);
                    }
                    "about" => {
                        println!("关于 MarkFly");
                    }
                    _ => {
                        println!("未知菜单事件: {}", event_id);
                    }
                }
            });

            // 应用启动时的初始化逻辑
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            println!("应用初始化完成");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
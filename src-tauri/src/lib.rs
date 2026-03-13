use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem},
    AppHandle, Emitter, Manager, Runtime,
};

static IS_DRAGGABLE: AtomicBool = AtomicBool::new(true);
static IS_CLICK_THROUGH: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Clone)]
pub struct WidgetState {
    pub is_draggable: bool,
    pub is_click_through: bool,
}

#[tauri::command]
async fn get_widget_state() -> WidgetState {
    WidgetState {
        is_draggable: IS_DRAGGABLE.load(Ordering::SeqCst),
        is_click_through: IS_CLICK_THROUGH.load(Ordering::SeqCst),
    }
}

#[tauri::command]
async fn toggle_drag_mode(app: AppHandle) -> Result<WidgetState, String> {
    let new_value = !IS_DRAGGABLE.load(Ordering::SeqCst);
    IS_DRAGGABLE.store(new_value, Ordering::SeqCst);

    update_window_state(&app);

    Ok(WidgetState {
        is_draggable: new_value,
        is_click_through: IS_CLICK_THROUGH.load(Ordering::SeqCst),
    })
}

#[tauri::command]
async fn toggle_click_through(app: AppHandle) -> Result<WidgetState, String> {
    let new_value = !IS_CLICK_THROUGH.load(Ordering::SeqCst);
    IS_CLICK_THROUGH.store(new_value, Ordering::SeqCst);

    update_window_state(&app);

    Ok(WidgetState {
        is_draggable: IS_DRAGGABLE.load(Ordering::SeqCst),
        is_click_through: new_value,
    })
}

#[tauri::command]
async fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
async fn show_context_menu(app: AppHandle) -> Result<(), String> {
    let menu = get_context_menu(&app).map_err(|e| e.to_string())?;

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.popup_menu(&menu);
    }

    Ok(())
}

fn update_window_state(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let is_draggable = IS_DRAGGABLE.load(Ordering::SeqCst);
        let click_through = IS_CLICK_THROUGH.load(Ordering::SeqCst);
        // Only enable click-through when not draggable AND click-through is enabled
        if let Err(e) = window.set_ignore_cursor_events(!is_draggable && click_through) {
            eprintln!("Failed to set ignore cursor events: {}", e);
        }
    }
}

fn get_context_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, Box<dyn std::error::Error>> {
    let is_draggable = IS_DRAGGABLE.load(Ordering::SeqCst);
    let is_click_through = IS_CLICK_THROUGH.load(Ordering::SeqCst);

    let toggle_drag_text = if is_draggable {
        "✓ 允许拖动"
    } else {
        "  锁定位置"
    };

    let toggle_click_text = if is_click_through {
        "✓ 点击穿透"
    } else {
        "  点击穿透"
    };

    let toggle_drag = MenuItem::with_id(app, "toggle_drag", toggle_drag_text, true, None::<&str>)?;
    let toggle_click_through = MenuItem::with_id(app, "toggle_click_through", toggle_click_text, true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置...", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle_drag, &toggle_click_through, &settings, &quit])?;

    Ok(menu)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Set window background color to transparent for macOS
            #[cfg(target_os = "macos")]
            if let Some(window) = app.get_webview_window("main") {
                use objc2::msg_send;
                use objc2::runtime::AnyObject;
                use objc2_app_kit::NSColor;

                let ns_window = window.ns_window().unwrap() as *mut AnyObject;
                unsafe {
                    // Set the window to be non-opaque
                    let _: () = msg_send![ns_window, setOpaque: false];

                    // Set the window background to clear (transparent)
                    let clear_color = NSColor::clearColor();
                    let _: () = msg_send![ns_window, setBackgroundColor: &*clear_color];

                    // Ensure the content view supports transparency
                    let content_view: *mut AnyObject = msg_send![ns_window, contentView];
                    let _: () = msg_send![content_view, setWantsLayer: true];
                }
            }

            // Setup menu event handler
            app.on_menu_event(|app, event| {
                match event.id.as_ref() {
                    "toggle_drag" => {
                        let _ = tauri::async_runtime::block_on(async {
                            toggle_drag_mode(app.clone()).await
                        });
                        let _ = app.emit("widget-state-changed", ());
                    }
                    "toggle_click_through" => {
                        let _ = tauri::async_runtime::block_on(async {
                            toggle_click_through(app.clone()).await
                        });
                        let _ = app.emit("widget-state-changed", ());
                    }
                    "settings" => {
                        let _ = app.emit("show-toast", "设置功能即将推出");
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_widget_state,
            toggle_drag_mode,
            toggle_click_through,
            quit_app,
            show_context_menu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

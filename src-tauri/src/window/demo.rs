use tauri::{App, CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder};

fn demo_menu() -> Menu {
    let quit = CustomMenuItem::new("demo-quit".to_string(), "Demo-Quit");
    let close = CustomMenuItem::new("demo-close".to_string(), "Demo-Close");
    let submenu = Submenu::new("Demo", Menu::new()
        .add_item(quit).add_item(close));
    Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
}

pub fn demo_window(app: &mut App) {
    let menu = demo_menu();

    let window = WindowBuilder::new(
        app,
        "demo".to_string(),
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
        .menu(menu).build().unwrap();

    let window_ = window.clone();
    window.on_menu_event(move |event| {
        match event.menu_item_id() {
            "demo-quit" => {
                std::process::exit(0);
            }
            "demo-close" => {
                window_.close().unwrap();
            }
            _ => {}
        }
    });
}

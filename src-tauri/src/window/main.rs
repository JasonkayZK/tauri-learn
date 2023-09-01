use tauri::{App, CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder};

fn main_menu() -> Menu {
    let quit = CustomMenuItem::new("main-quit".to_string(), "Main-Quit");
    let close = CustomMenuItem::new("main-close".to_string(), "Main-Close");
    let submenu = Submenu::new("Main", Menu::new()
        .add_item(quit).add_item(close));
    Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
}

pub fn main_window(app: &mut App) {
    let menu = main_menu();

    let window = WindowBuilder::new(
        app,
        "main".to_string(),
        tauri::WindowUrl::App("index.html".parse().unwrap()),
    )
        .menu(menu).build().unwrap();

    let window_ = window.clone();
    window.on_menu_event(move |event| {
        match event.menu_item_id() {
            "main-close" => {
                window_.close().unwrap();
            }
            "main-quit" => {
                std::process::exit(0);
            }
            _ => {}
        }
    });
}

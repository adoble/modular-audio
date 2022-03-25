slint::include_modules!();

fn main() {
    let ui = AppWindow::new();

    let ui_handle = ui.as_weak();
    ui.on_select_source(move || {
        let _ui = ui_handle.unwrap();
        //ui.set_counter(ui.get_counter() + 1);
    });

    ui.run();
}

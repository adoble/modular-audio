use std::sync::mpsc::channel;

use timer::Timer;

slint::include_modules!();

fn main() {
    let ui = AppWindow::new();

    // Set up the bluetooth timer
    let bluetooth_timer = timer::Timer::new();
    //let (tx, rx) = channel();


    
    let _ui_handle = ui.as_weak();
    // ui.on_select_source(move || {
    //     let _ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
        
    // });
    
    ui.global::<Source>().on_select_source(move | id | {
            
     
            println!("Selected source: {}", id);
    });


    ui.global::<Bluetooth>().set_connecting(true);

    ui.run();
}

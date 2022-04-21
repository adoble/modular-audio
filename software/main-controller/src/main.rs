use slint::{Timer, TimerMode};

slint::include_modules!();

fn main() {
    let ui = AppWindow::new();

    // Initialise the timer to simulate the bluetooth connection. 
    // This has to be done outside of the callback code. 
    let bluetooth_timer = Timer::default();
    
    let ui_handle = ui.as_weak();

    ui.global::<Source>().on_select_source(move |source_id| {
        println!("Selected source: {}", source_id);
        let ui = ui_handle.unwrap();
        let selected_source = match source_id {
            -1 | 0 => None, 
            1..=8 => Some(source_id),
            _ => panic!("Unknown source {}", source_id),
        };
       
        
        match selected_source {
            Some(1) => { 
                println!("Bluetooth!");
                
                let status = ui.global::<Bluetooth>().get_connecting();
                
                ui.global::<Bluetooth>().set_state(status);

                bluetooth_timer.start(
                    TimerMode::SingleShot,
                    std::time::Duration::from_millis(2000),
                    move || {
                        ui.global::<Bluetooth>().set_state(ui.global::<Bluetooth>().get_connected());
                        println!("Connected");
                    },
                );
            },
            //_ => (),
            Some(2..=8) => println!("Source {}", selected_source.expect("Unknown source")),
            _ => println!("Not a source"),
        }
    });

    ui.run();
}

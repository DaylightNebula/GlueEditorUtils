
pub fn glue_app(app_name: &str, eframe_app: eframe::AppCreator) {
    // setup options
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: true,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(400.0, 100.0)),
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    // run native
    let result = eframe::run_native(
        app_name, 
        options, 
        eframe_app
    );

    // if an error was returned, print error
    if result.is_err() {
        let error = result.err().unwrap();
        println!("EFrame run native returned error {error}");
    }
}
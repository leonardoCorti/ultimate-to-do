slint::include_modules!();

#[unsafe(no_mangle)]
fn android_main(_app: slint::android::AndroidApp) {
    let ui = AppWindow::new().unwrap();

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_request_decrease_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() - 1);
        }
    });

    ui.run().unwrap();

}

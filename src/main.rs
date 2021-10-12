mod system_data;

use std::cell::Cell;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use std::rc::Rc;

fn main() {
 let app = Application::builder().application_id("com.leniorko").build();
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application){
    let window = ApplicationWindow::builder().application(app).title("My GTK app").build();

    let  click_counter = Rc::new(Cell::new(0));

    let button = Button::builder().label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    button.connect_clicked(move |button|{
        &click_counter.set(&click_counter.get() + 1);
        let new_label = &click_counter.get().to_string();
        button.set_label(&*new_label);
    });

    window.set_child(Some(&button));

    window.present();
}
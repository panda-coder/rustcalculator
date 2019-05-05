extern crate gtk;
extern crate gio;

use gio::prelude::*;
use gtk::prelude::*;



use std::env::args;

mod calculator;

use gtk::{
    Application
};


fn main() {
    let application = Application::new(
        "com.github.panda-coder.rustcalculator",
        Default::default()
    ).expect("Initialization failed...");

    application.connect_activate(|app| {
        calculator::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

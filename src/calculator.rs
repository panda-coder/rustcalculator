extern crate gtk;
extern crate gio;

use gtk::prelude::*;


use gtk::{
    Button, WindowPosition,ApplicationWindow,Application,
    Menu, MenuBar, MenuItem
};

use gtk::prelude::Inhibit;


// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}



pub fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("Calculator");
    window.set_position(WindowPosition::Center);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    window.set_default_size(400, 400);
    let button = Button::new_with_label("Click me!");

    let menu = Menu::new();
    let menu_bar = MenuBar::new();
    let quit = MenuItem::new_with_label("Quit");

    menu.append(&quit);

    let file = MenuItem::new_with_label("File");
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    v_box.pack_start(&menu_bar, false, false, 0);

    window.add(&v_box);
    window.show_all();

    let window_weak = window.downgrade();
    quit.connect_activate(move |_| {
        let window = upgrade_weak!(window_weak);
        window.destroy();
        Inhibit(false);
    });

    window.connect_delete_event(|_, _| {
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });
}
extern crate gtk;

use gtk::prelude::*;


use gtk::{Button, Window, WindowType, WindowPosition,
          Menu, MenuBar, MenuItem};

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

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
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
        gtk::main_quit();
        Inhibit(false);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::main();
}

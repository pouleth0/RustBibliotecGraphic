extern crate gtk;
use gtk::prelude::*;

pub fn show_menu_window() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Menu");
    window.set_default_size(400, 800);
    window.set_position(gtk::WindowPosition::Left);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    window.show_all();

    gtk::main();
}
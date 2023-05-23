extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, ButtonExt, Grid, GridExt, Label, LabelExt, Window, WindowPosition, WindowType};

fn show_menu_painel() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Menu");
    window.set_position(WindowPosition::Center);
    window.set_default_size(400, 300);

    let grid = Grid::new();
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);
    grid.set_row_spacing(10);
    grid.set_column_spacing(10);

    let button1 = Button::new();
    button1.set_label("Button 1");
    grid.attach(&button1, 0, 0, 1, 1);

    let button2 = Button::new();
    button2.set_label("Button 2");
    grid.attach(&button2, 1, 0, 1, 1);

    let button3 = Button::new();
    button3.set_label("Button 3");
    grid.attach(&button3, 2, 0, 1, 1);

    let button4 = Button::new();
    button4.set_label("Button 4");
    grid.attach(&button4, 0, 1, 1, 1);

    let button5 = Button::new();
    button5.set_label("Button 5");
    grid.attach(&button5, 1, 1, 1, 1);

    let button6 = Button::new();
    button6.set_label("Button 6");
    grid.attach(&button6, 2, 1, 1, 1);

    let button7 = Button::new();
    button7.set_label("Button 7");
    grid.attach(&button7, 0, 2, 1, 1);

    let button8 = Button::new();
    button8.set_label("Button 8");
    grid.attach(&button8, 1, 2, 1, 1);

    window.add(&grid);
    window.show_all();

    gtk::main();
}
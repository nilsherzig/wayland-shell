use std::{fs, thread, time::Duration};

use chrono::{DateTime, Local};
use gio::prelude::*;
use glib::{clone, Continue, MainContext, PRIORITY_DEFAULT};
use gtk::{
    gdk::Display, prelude::*, ApplicationWindow, Button, CssProvider, Orientation, StyleContext,
};

fn activate(application: &gtk::Application) {
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::End)
        .build();
    gtk_box.add_css_class("main_container");

    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .build();

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    button.connect_clicked(move |_| {
        let sender = sender.clone();
        thread::spawn(move || {
            let mut counter = 0;
            while counter < 1000 {
                thread::sleep(Duration::from_millis(100));
                counter += 1;
                sender
                    .send(1000 - counter)
                    .expect("Could not send through channel");
            }
        });
    });

    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |sender_message| {
                        button.set_label(&sender_message.to_string());
                        Continue(true)
                    }
        ),
    );

    let date_label = gtk::Label::builder().label("time").build();

    let (clock_sender, clock_receiver) = MainContext::channel(PRIORITY_DEFAULT);
    thread::spawn(move || loop {
        let now: DateTime<Local> = Local::now();
        clock_sender
            .send(format!("{}", now.format("%a %e %b %T")))
            .expect("Could not send through channel");
        thread::sleep(Duration::from_secs(1));
    });

    clock_receiver.attach(
        None,
        clone!(@weak date_label => @default-return Continue(false),
                    move |sender_message| {
                        date_label.set_label(&sender_message.to_string());
                        Continue(true)
                    }
        ),
    );

    let battery_label = gtk::Label::builder().label("time").build();

    battery_label.add_css_class("battery_label");

    let (battery_sender, battery_receiver) = MainContext::channel(PRIORITY_DEFAULT);
    thread::spawn(move || loop {
        let contents = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .expect("error reading file")
            .trim()
            .to_string();

        let battery_value = contents.parse::<i32>().unwrap();

        let icon = match battery_value {
            0..=20 => "",
            21..=40 => "",
            41..=60 => "",
            61..=80 => "",
            81..=100 => "",
            _ => panic!("Invalid battery level: {}", battery_value),
        };

        battery_sender
            .send(format!("{}", icon))
            .expect("Could not send through channel");
        thread::sleep(Duration::from_secs(60));
    });

    battery_receiver.attach(
        None,
        clone!(@weak battery_label => @default-return Continue(false),
                    move |sender_message| {
                        battery_label.set_label(&sender_message.to_string());
                        Continue(true)
                    }
        ),
    );

    gtk_box.append(&battery_label);
    gtk_box.append(&date_label);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .height_request(60)
        .child(&gtk_box)
        .build();

    gtk4_layer_shell::init_for_window(&window);

    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

    gtk4_layer_shell::auto_exclusive_zone_enable(&window);

    gtk4_layer_shell::set_namespace(&window, "wayland-shell");

    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);

    let anchors = [
        (gtk4_layer_shell::Edge::Left, true),
        (gtk4_layer_shell::Edge::Right, true),
        (gtk4_layer_shell::Edge::Top, false),
        (gtk4_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
    }

    window.show()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let application =
        gtk::Application::new(Some("com.nilsherzig.wayland-shell"), Default::default());

    application.connect_startup(|_| load_css());
    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}

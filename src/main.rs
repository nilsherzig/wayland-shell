use std::{fs, thread, time::Duration};

use chrono::{DateTime, Local};
use gio::prelude::*;
use glib::{clone, Continue, MainContext, PRIORITY_DEFAULT};
use gtk::{
    gdk::Display, prelude::*, ApplicationWindow, Button, CssProvider, Orientation, StyleContext,
};

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    // let window = gtk::ApplicationWindow::new(application);

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(gtk::Align::End)
        .build();
    gtk_box.add_css_class("main_container");

    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        // .margin_start(12)
        // .margin_end(12)
        .build();

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            // Deactivate the button until the operation is done
            // sender.send(10).expect("Could not send through channel");
            // let ten_seconds = Duration::from_secs(10);

            let mut counter = 0;
            while counter < 1000 {
                thread::sleep(Duration::from_millis(100));
                counter += 1;
                sender
                    .send(1000 - counter)
                    .expect("Could not send through channel");
            }
            // Activate the button again
        });
    });

    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |sender_message| {
                        // button.set_sensitive(enable_button);
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
                        // button.set_sensitive(enable_button);
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
                        // button.set_sensitive(enable_button);
                        battery_label.set_label(&sender_message.to_string());
                        Continue(true)
                    }
        ),
    );

    // gtk_box.append(&button);
    gtk_box.append(&battery_label);
    gtk_box.append(&date_label);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .height_request(60)
        .child(&gtk_box)
        .build();

    // Before the window is first realized, set it up to be a layer surface
    gtk4_layer_shell::init_for_window(&window);

    // Display above normal windows
    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

    // Push other windows out of the way
    gtk4_layer_shell::auto_exclusive_zone_enable(&window);

    gtk4_layer_shell::set_namespace(&window, "wayland-shell");

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk4_layer_shell::Edge::Left, true),
        (gtk4_layer_shell::Edge::Right, true),
        (gtk4_layer_shell::Edge::Top, false),
        (gtk4_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
    }

    // Set up a widget
    // let label = gtk::Label::new(Some(""));
    // label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    // window.set_child(Some(&label));

    // let test = gtk::Label::new(Some("asdasd"));
    // window.set_child(Some(&test));

    window.show()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
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

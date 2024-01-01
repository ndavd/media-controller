use gtk::glib::{Propagation, ControlFlow};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use crate::controller::{Color, MediaController};

fn set_visual(window: &ApplicationWindow, _screen: Option<&gtk::gdk::Screen>) {
    if let Some(screen) = GtkWindowExt::screen(window) {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}

fn draw(_win: &ApplicationWindow, ctx: &gtk::cairo::Context, bg: Color) -> Propagation {
    ctx.set_source_rgba(bg.r as f64, bg.g as f64, bg.b as f64, bg.a as f64);
    ctx.paint().unwrap();
    Propagation::Proceed
}

fn realize(win: &ApplicationWindow, width: i32, bottom: i32) {
    let gdk_win = win.window().unwrap();
    gdk_win.set_override_redirect(true);

    let monitor_geometry = gdk_win
        .screen()
        .display()
        .primary_monitor()
        .unwrap()
        .geometry();

    win.move_(
        monitor_geometry.width() / 2 - width / 2,
        monitor_geometry.height() - bottom,
    );
    win.show_all();
}

fn build_ui(
    app: &Application,
    controller: &MediaController,
    shared: std::sync::Arc<std::sync::Mutex<String>>,
) {
    let w = controller.width as i32;
    let h = controller.height as i32;
    let b = controller.bottom as i32;

    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(w)
        .default_height(h)
        .build();

    set_visual(&win, None);
    win.connect_screen_changed(set_visual);
    win.set_app_paintable(true);
    win.connect_realize(move |win| realize(win, w, b));
    let color = controller.color;
    win.connect_draw(move |win, ctx| draw(win, ctx, color));

    let label = gtk::Label::new(None);
    label.set_text(&shared.lock().unwrap());
    label.set_yalign(0.0);
    let attr = gtk::pango::AttrList::new();
    attr.insert(gtk::pango::AttrFontDesc::new(
        &gtk::pango::FontDescription::from_string(&controller.font_description),
    ));
    label.set_attributes(Some(&attr));
    win.add(&label);

    gtk::glib::timeout_add_seconds(controller.duration, move || {
        println!("Closing...");
        std::process::exit(0);
    });

    win.show_all();
}

pub fn spawn_window(controller: MediaController, shared: std::sync::Arc<std::sync::Mutex<String>>) {
    gtk::init().unwrap();
    let app = Application::builder().build();
    app.connect_activate(move |app| build_ui(app, &controller, shared.clone()));
    app.run_with_args::<&str>(&[]);
}

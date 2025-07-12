use gtk4::gdk::Display;
use gtk4::glib::ControlFlow;
use gtk4::{prelude::*, CssProvider};
use gtk4::{Application, ApplicationWindow};

use gtk4_layer_shell::LayerShell;

use crate::MediaController;

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

    win.init_layer_shell();
    win.set_layer(gtk4_layer_shell::Layer::Overlay);
    win.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
    win.set_margin(gtk4_layer_shell::Edge::Bottom, b);

    let css = format!(
        ".media-controller-window {{ background-color: {} }}",
        controller.color
    );
    let css_provider = CssProvider::new();
    css_provider.load_from_data(css.as_str());
    let display = Display::default().unwrap();
    gtk4::style_context_add_provider_for_display(
        &display,
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let label = gtk4::Label::new(None);

    label.set_text(&shared.lock().unwrap());
    let attr = gtk4::pango::AttrList::new();
    attr.insert(gtk4::pango::AttrFontDesc::new(
        &gtk4::pango::FontDescription::from_string(&controller.font_description),
    ));
    label.set_attributes(Some(&attr));
    win.set_child(Some(&label));
    win.add_css_class("media-controller-window");

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(10), move || {
        if let Ok(shared) = shared.lock() {
            if label.text().as_str() != shared.as_str() {
                label.set_text(&shared);
            }
        }
        ControlFlow::Continue
    });

    win.present();
}

pub fn spawn_wl_window(
    controller: MediaController,
    shared: std::sync::Arc<std::sync::Mutex<String>>,
) {
    gtk4::init().unwrap();
    let app = Application::builder().build();
    app.connect_activate(move |app| build_ui(app, &controller, shared.clone()));
    app.run_with_args::<&str>(&[]);
}

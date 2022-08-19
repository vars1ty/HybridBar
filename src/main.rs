mod config;
mod debug;
mod r#loop;
mod proc;
mod ui;

use gtk::gdk::*;
use gtk::prelude::*;
use gtk::*;
use json::JsonValue;

/// Prints a message with the [Hybrid] prefix.
fn prefix_print(msg: &str) {
    println!("[Hybrid] {msg}")
}

/// Initializes the status bar.
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);

    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    // Initialize layer shell before the window has been fully initialized.
    gtk_layer_shell::init_for_window(&window);

    // Order below normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Bottom);

    // Push other windows out of the way
    // Toggling this off may help some if they are in applications that have weird unicode text, which may mess with the bars scaling.
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    // Scale and set the bar to the top.
    let anchors = [
        (gtk_layer_shell::Edge::Left, true),
        (gtk_layer_shell::Edge::Right, true),
        (gtk_layer_shell::Edge::Top, true),
        (gtk_layer_shell::Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        gtk_layer_shell::set_anchor(&window, anchor, state);
    }

    // For transparency to work.
    window.set_app_paintable(true);

    // Build all the widgets.
    ui::build_widgets(&window);
    prefix_print("Ready!")
}

/// Loads the CSS
#[allow(unused_must_use)] // Only for this because load_from_data is a special snowflake.
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("[ERROR] Could not connect to a display, fix your PC.\n"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Called upon application startup.
fn main() {
    prefix_print("Building application...");
    let application = gtk::Application::new(None, Default::default());
    prefix_print("Loading CSS...");
    application.connect_startup(|_| load_css());
    prefix_print("Creating viewport...");
    // Activate the layer shell.
    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}

/// Applies custom visuals.
fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

/// Converts the value of a child inside "background" to a `f64`.
fn get_background_float(cfg: &JsonValue, identifier: &str) -> f64 {
    cfg["background"][identifier]
        .as_f64()
        .expect("[ERROR] Failed converting background:{identifier} to f64!\n")
        / 255.0
    // Divide by 255 so that RGB values apply and users don't have to use 0-1 values.
}

/// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    let cfg = config::read_config();
    let b_cfg = &cfg;
    // Fetch config for the values.
    let r: f64 = get_background_float(b_cfg, "r");
    let g: f64 = get_background_float(b_cfg, "g");
    let b: f64 = get_background_float(b_cfg, "b");
    let a: f64 = get_background_float(b_cfg, "a");
    // Apply
    ctx.set_source_rgba(r, g, b, a);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("[ERROR] Failed painting!\n");
    Inhibit(false)
}

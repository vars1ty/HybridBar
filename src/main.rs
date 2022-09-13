mod config;
mod debug;
mod environment;
mod r#loop;
mod proc;
mod ui;
mod widget_builder;

use std::path::Path;

use debug::log;
use gtk::gdk::*;
use gtk::prelude::*;
use gtk::*;
use gtk_layer_shell::Edge;
use json::JsonValue;

/// Gets the anchors.
fn get_anchors() -> [(gtk_layer_shell::Edge, bool); 4] {
    let pos = environment::try_get_var("HYBRID_POS");
    if pos != "TOP" && pos != "BOTTOM" {
        panic!("[ERROR] Invalid position! Values: [ TOP, BOTTOM ]\n")
    }

    // If the position was valid, return the result.
    [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, pos == "TOP"),
        (Edge::Bottom, pos == "BOTTOM"),
    ]
}

/// Initializes the status bar.
fn activate(application: &Application) {
    // Create a normal GTK window however you like
    let window = ApplicationWindow::new(application);

    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    // Initialize layer shell before the window has been fully initialized.
    gtk_layer_shell::init_for_window(&window);

    // Order below normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Bottom);

    // Push other windows out of the way
    // Toggling this off may help some if they are in applications that have weird unicode text, which may mess with the bars scaling.
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    for (anchor, state) in get_anchors() {
        gtk_layer_shell::set_anchor(&window, anchor, state);
    }

    // For transparency to work.
    window.set_app_paintable(true);

    // Build all the widgets.
    ui::build_widgets(&window);
    log("Ready!")
}

/// Loads the CSS
#[allow(unused_must_use)]
fn load_css() {
    let provider = CssProvider::new();
    let mut css_path = config::get_path();
    css_path.push_str("style.css");
    if !Path::new(&css_path).is_file() {
        log("No style.css file was found, falling back to default GTK settings!")
    }

    provider.load_from_path(&css_path);

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("[ERROR] Could not connect to a display, fix your PC.\n"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Called upon application startup.
fn main() {
    log("Building application...");
    let application = Application::new(None, Default::default());
    log("Loading CSS...");
    application.connect_startup(|_| load_css());
    log("Creating viewport...");
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

/// Converts the value of a child inside `background` to a `f64`.
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
    // Fetch config for the values.
    let r = get_background_float(&cfg, "r");
    let g = get_background_float(&cfg, "g");
    let b = get_background_float(&cfg, "b");
    let a = get_background_float(&cfg, "a");
    // Apply
    ctx.set_source_rgba(r, g, b, a);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("[ERROR] Failed painting!\n");
    Inhibit(false)
}

#![no_main]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

#[path = "utils/aliases.rs"]
mod aliases;
#[path = "utils/cava.rs"]
mod cava;
mod config;
mod constants;
mod environment;
mod r#loop;
#[path = "utils/math.rs"]
mod math;
mod structures;
mod ui;
mod widget;
mod widgets;

use constants::*;
use gtk::gdk::*;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::*;
use gtk_layer_shell::Edge;
use json::JsonValue;
use std::path::Path;

/// Gets the anchors.
fn get_anchors() -> [(gtk_layer_shell::Edge, bool); 4] {
    let expand_left = conf_bool!(HYBRID_ROOT_JSON, "expand_left", true);
    let expand_right = conf_bool!(HYBRID_ROOT_JSON, "expand_right", true);

    let pos = conf!(HYBRID_ROOT_JSON, "position", true, false)
        .string
        .unwrap_or_else(|| "Top".to_owned());

    if !pos.eq_ignore_ascii_case("Top") && !pos.eq_ignore_ascii_case("Bottom") && !pos.is_empty() {
        panic!("{}", ERR_INVALID_POS)
    }

    // If the position was valid, return the result.
    [
        (Edge::Left, expand_left),
        (Edge::Right, expand_right),
        (Edge::Top, pos.eq_ignore_ascii_case("Top") || pos.is_empty()),
        (Edge::Bottom, pos.eq_ignore_ascii_case("Bottom")),
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

    // Order above normal windows
    // Prior to 0.2.9, this was set to Bottom but it caused issues with tooltips being shown below
    // windows.
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);

    // Push other windows out of the way
    // Toggling this off may help some if they are in applications that have weird unicode text, which may mess with the bars scaling.
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    for (anchor, state) in get_anchors() {
        gtk_layer_shell::set_anchor(&window, anchor, state);
    }

    // Allows for specifing the namespace of the layer.
    // The default is "gtk-layer-shell" to not break existing configs.
    let namespace = conf!(HYBRID_ROOT_JSON, "namespace", true, false)
        .string
        .unwrap_or_else(|| "gtk-layer-shell".to_owned());

    gtk_layer_shell::set_namespace(&window, &namespace);

    // Initialize gdk::Display by default value, which is decided by the compositor.
    let display = Display::default().expect(ERR_GET_DISPLAY);

    // Loads the monitor variable from config, default is 0.
    let config_monitor = conf!(HYBRID_ROOT_JSON, "monitor", false, false)
        .number
        .unwrap_or_default();

    // Gets the actual gdk::Monitor from configured number.
    let monitor = display.monitor(config_monitor).expect(ERR_GET_MONITOR);

    // Sets which monitor should be used for the bar.
    gtk_layer_shell::set_monitor(&window, &monitor);

    // For transparency to work.
    window.set_app_paintable(true);

    // Build all the widgets.
    ui::build_widgets(&window);
    log!("Ready!");
}

/// Loads the CSS
pub fn load_css() {
    let provider = CssProvider::new();
    // 0.2.8: Allow for defining the name of the stylesheet to look up
    let css_file = conf!(HYBRID_ROOT_JSON, "stylesheet", true, false)
        .string
        .unwrap_or_else(|| DEFAULT_CSS.to_owned());

    let mut css_path = config::get_path();
    css_path.push_str(&css_file);

    if Path::new(&css_path).is_file() {
        provider
            .load_from_path(&css_path)
            .unwrap_or_else(|_| panic!("[ERROR] Failed loading CSS from '{css_file}'!"))
    } else {
        provider
            .load_from_data(include_bytes!("../examples/style.css"))
            .expect(ERR_LOAD_SAMPLE_CSS);
        log!("No custom stylesheet was found, using ../examples/style.css")
    }

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &Screen::default().expect(ERR_SCREEN_DEFAULT),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Called upon application startup.
#[no_mangle]
#[tokio::main]
async fn main() {
    log!("Building application...");
    let application = Application::new(None, ApplicationFlags::default());
    log!("Loading CSS...");
    application.connect_startup(|_| load_css());
    log!("Creating viewport...");
    // Activate the layer shell.
    application.connect_activate(|app| {
        activate(app);
    });

    tracing_subscriber::fmt::init();
    application.run();
}

/// Applies custom visuals.
fn set_visual(window: &ApplicationWindow, screen: Option<&Screen>) {
    if let Some(screen) = screen {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // Needed for transparency, not available in GTK 4+ so
                                             // F.
        }
    }
}

/// Converts the value of a child inside `background` to a `f64`.
fn get_background_float(cfg: &JsonValue, identifier: &str, from_255: bool) -> f64 {
    let mut res = cfg[HYBRID_ROOT_JSON][identifier]
        .as_f64()
        .unwrap_or_else(|| panic!("[ERROR] Failed converting hybrid:{identifier} to f64!"));

    // Only divide by 255 if explicitly told to.
    if from_255 {
        res /= 255.0;
    }

    // Return the result.
    res
}

/// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    let cfg = config::CONFIG.read().unwrap();

    // Fetch config for the values.
    let r = get_background_float(&cfg, "r", true);
    let g = get_background_float(&cfg, "g", true);
    let b = get_background_float(&cfg, "b", true);
    let a = get_background_float(&cfg, "a", false);

    // Apply
    ctx.set_source_rgba(r, g, b, a);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect(ERR_CUSTOM_DRAW);
    Inhibit(false)
}

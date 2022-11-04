#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

#[path = "widgets/box_widget.rs"]
mod box_widget;
#[path = "widgets/button_widget.rs"]
mod button_widget;
#[path = "utils/cava.rs"]
mod cava;
#[path = "widgets/cava_widget.rs"]
mod cava_widget;
#[path = "widgets/cmd_widget.rs"]
mod cmd_widget;
mod config;
mod environment;
#[path = "widgets/label_widget.rs"]
mod label_widget;
mod r#loop;
#[path = "utils/math.rs"]
mod math;
#[path = "widgets/spacing_widget.rs"]
mod spacing_widget;
mod structures;
mod ui;
mod widget;

use gtk::gdk::*;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::*;
use gtk_layer_shell::Edge;
use json::JsonValue;
use label_widget::LabelWidget;
use widget::HWidget;

/// Gets the anchors.
fn get_anchors() -> [(gtk_layer_shell::Edge, bool); 4] {
    const ROOT: &str = "hybrid";
    let mut expand_right = true;
    let mut expand_left = true;
    let mut pos = String::from("Top");

    // Check if there's any user-defined values for expand l-r/pos, if there are then sync them.
    if let Some(c_expand_right) = config::try_get(ROOT, "expand_right", true, false) {
        expand_right = c_expand_right.0 == "true";
    }

    if let Some(c_expand_left) = config::try_get(ROOT, "expand_left", true, false) {
        expand_left = c_expand_left.0 == "true";
    }

    if let Some(c_pos) = config::try_get(ROOT, "position", true, false) {
        pos = c_pos.0;
    }

    if !pos.eq_ignore_ascii_case("Top") && !pos.eq_ignore_ascii_case("Bottom") && !pos.is_empty() {
        panic!("[ERROR] Invalid position! Values: [ TOP, BOTTOM ] - casing doesn't matter.\n")
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

    // Allows for writing in input fields if the value is true.
    // This is false by default since it's stealing focus until you focus a different application,
    // which may trigger some users.
    if let Some(c_allow_keyboard) = config::try_get("hybrid", "allow_keyboard", true, false) {
        gtk_layer_shell::set_keyboard_interactivity(&window, c_allow_keyboard.0 == "true");
    }

    // Initialize gdk::Display by default value, which is decided by the compositor.
    let display = Display::default().expect("[ERROR] Could not get default display, is your compositor doing okay?\n");

    // Loads the monitor variable from config, default is 0.
    let config_monitor = config::try_get("hybrid", "monitor", false, false)
        .unwrap_or_else(|| (String::default(), 0));

    // Gets the actual gdk::Monitor from configured number.
    let monitor = display
        .monitor(config_monitor.1)
        .expect("[ERROR] Could not find monitor.\n");

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
    let mut css_file = String::from("style.css");
    if let Some(c_css_file) = config::try_get("hybrid", "stylesheet", true, false) {
        css_file = c_css_file.0
    }

    let mut css_path = config::get_path();
    css_path.push_str(&css_file);

    provider
        .load_from_path(&css_path)
        .unwrap_or_else(|_| panic!("[ERROR] Failed loading CSS from '{css_file}'!\n"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("[ERROR] Couldn't find any valid displays!\n"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Called upon application startup.
#[tokio::main]
async fn main() {
    log!("Caching config...");
    config::cache();
    log!("Building application...");
    let application = Application::new(None, ApplicationFlags::default());
    log!("Loading CSS...");
    application.connect_startup(|_| load_css());
    log!("Creating viewport...");
    // Activate the layer shell.
    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}

/// Applies custom visuals.
fn set_visual(window: &ApplicationWindow, screen: Option<&Screen>) {
    if let Some(screen) = screen {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

/// Converts the value of a child inside `background` to a `f64`.
fn get_background_float(cfg: &JsonValue, identifier: &str, from_255: bool) -> f64 {
    let mut res = cfg["hybrid"][identifier]
        .as_f64()
        .unwrap_or_else(|| panic!("[ERROR] Failed converting hybrid:{identifier} to f64!\n"));

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
    ctx.paint().expect("[ERROR] Failed painting!\n");
    Inhibit(false)
}

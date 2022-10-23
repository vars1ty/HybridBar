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
use std::path::Path;
use widget::HWidget;

/// Gets the anchors.
fn get_anchors() -> [(gtk_layer_shell::Edge, bool); 4] {
    let pos = environment::try_get_var("HYBRID_POS", "TOP");
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
    log!("Ready!");
}

/// Loads the CSS
pub fn load_css() {
    let provider = CssProvider::new();
    let mut css_path = config::get_path();
    css_path.push_str("style.css");
    if !Path::new(&css_path).is_file() {
        log!("No style.css file was found, falling back to default GTK settings!")
    }

    provider
        .load_from_path(&css_path)
        .expect("[ERROR] Failed loading CSS!\n");

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
fn set_visual(window: &ApplicationWindow, screen: Option<&gdk::Screen>) {
    if screen.is_some() {
        let u_screen = screen.unwrap();
        if let Some(ref visual) = u_screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

/// Converts the value of a child inside `background` to a `f64`.
fn get_background_float(cfg: &JsonValue, identifier: &str, from_255: bool) -> f64 {
    let mut res = cfg["hybrid"][identifier]
        .as_f64()
        .expect(format!("[ERROR] Failed converting hybrid:{identifier} to f64!\n").as_str());

    // Only divide by 255 if explicitly told to.
    if from_255 {
        res = res / 255.0;
    }

    // Return the result.
    res
}

/// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    let cfg = config::read_config();
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

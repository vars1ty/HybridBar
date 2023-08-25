use crate::{
    r#loop::update,
    structures::{BaseKeys, RevealerExtensions, WidgetHolders},
    utils::cava::{self, HAS_CAVA_STARTED},
    *,
};
use crate::{
    widget::{Align, HWidget},
    widgets::{
        box_widget::BoxWidget, button_widget::ButtonWidget, cava_widget::CavaWidget,
        label_widget::LabelWidget, spacing_widget::SpacingWidget,
    },
};
use gtk::traits::*;
use parking_lot::RwLock;
use smallvec::SmallVec;

/// Hybrid Bar UI
pub struct UI {
    /// Holds the Left, Centered and Right box widgets.
    widget_holders: RwLock<Option<WidgetHolders>>,

    /// Config instance.
    config: &'static Config,
}

impl UI {
    pub fn init(config: &'static Config) -> Self {
        Self {
            widget_holders: Default::default(),
            config,
        }
    }

    /// Add and align a widget.
    pub fn add_and_align(&self, widget: &impl IsA<Widget>, align: Align, box_holder: Option<&Box>) {
        let holders = self.widget_holders.read();
        let holders = holders
            .as_ref()
            .expect("[ERROR] Couldn't call read() on widget_holders!");
        if let Some(r#box) = box_holder {
            r#box.add(widget);
        } else {
            match align {
                Align::Left => holders.left.add(widget),
                Align::Centered => holders.centered.add(widget),
                Align::Right => holders.right.add(widget),
            }
        }
    }

    /// Builds all of the widgets.
    pub fn build_widgets(
        &self,
        window: &ApplicationWindow,
        vm: Option<Vm>,
        config: &'static Config,
    ) {
        // Create box widgets, which we'll be using to draw the content onto.
        let root = Box::new(Orientation::Horizontal, 0);
        let left = Box::new(Orientation::Horizontal, 0);
        let centered = Box::new(Orientation::Horizontal, 0);
        let right = Box::new(Orientation::Horizontal, 0);

        // 0.2.5: Root expands across the entire bar, previously "left" would do this but it isn't
        //   ideal when customizing, since borders would draw on the entire bar rather than just on the
        //   left portion of the bar.
        root.set_widget_name("root");

        // 0.2.5: Allow for customizing left, centered and right.
        left.set_widget_name("left");
        centered.set_widget_name("centered");
        right.set_widget_name("right");

        root.set_center_widget(Some(&centered));
        root.pack_end(&right, false, true, 0);
        root.add(&left);
        window.add(&root);

        *self.widget_holders.write() = Some(WidgetHolders {
            root,
            left,
            centered,
            right,
        });

        // Prepare and show all of the widgets.
        self.create_components(config);
        window.show_all();

        // Update dynamic content.
        update(vm, config);
    }

    /// Gets the base key values.
    pub fn get_base_keys(
        &self,
        root: &JsonValue,
        config: &'static Config,
    ) -> (String, String, u64, String, String) {
        let custom_variables = &config.get_custom_variables();
        let text = config.with_variables(
            root["text"].as_str().unwrap_or_default().to_owned(),
            custom_variables,
        );
        let command = config.with_variables(
            root["command"].as_str().unwrap_or_default().to_owned(),
            custom_variables,
        );
        let update_rate: u64 = root["update_rate"]
            .as_i32()
            .unwrap_or(100)
            .try_into()
            .unwrap_or_else(|_| {
                panic!("[ERROR] Couldn't convert update_rate to u64! Source: {root}")
            });
        let tooltip = config.with_variables(
            root["tooltip"].as_str().unwrap_or_default().to_owned(),
            custom_variables,
        );
        let tooltip_command = config.with_variables(
            root["tooltip_command"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            custom_variables,
        );
        (text, command, update_rate, tooltip, tooltip_command)
    }

    /// Creates all of the widgets.
    fn create_components(&self, config: &'static Config) {
        // Add all of the widgets defined from the config.
        const ALIGNMENT: char = '-';
        const SEPARATOR: &str = "_";
        let relevant = config
            .read_config_raw()
            .entries()
            .filter(|(key, _)| key.contains(ALIGNMENT) && key.contains(SEPARATOR));

        for (key, json) in relevant {
            // Gets the widget identifiers.
            let identifiers: SmallVec<[&str; 4]> = key.split(SEPARATOR).collect();

            // Identifier example: `left-label_ABC` <= `left-label` is the IDENTIFIER, `ABC` is the NAME.
            let identifier = identifiers[0];

            // Grabs widget alignment and widget type from the identifier separated by '-'.
            let (widget_alignment, widget_type) = identifier
                .split_once(ALIGNMENT)
                .expect(ERR_INVALID_WIDGET_FORMAT);

            // Base keys, all being optional.
            let (text, command, update_rate, tooltip, tooltip_command) =
                self.get_base_keys(json, config);
            let base_keys = BaseKeys {
                text,
                command,
                update_rate,
                tooltip,
                tooltip_command,
                alignment: Align::from_str(widget_alignment).expect(ERR_INVALID_ALIGNMENT),
            };

            // Gets every element after the widget identifier, then appends '_' in between.
            let widget_name = identifiers[1..].join(SEPARATOR);

            if widget_name.is_empty() {
                panic!("{}", ERR_EMPTY_NAME)
            }

            log!(format!(
                "Adding widget '{identifier}' with alignment '{widget_alignment}'!",
            ));

            // Add the widget.
            self.add_widget(
                json,
                (widget_type, &widget_name),
                base_keys,
                identifier,
                None,
                config,
            )
        }
    }

    /// Add a new widget of specified identifier.
    pub fn add_widget(
        &self,
        key: &JsonValue,
        widget_pkg: (&str, &str),
        base_keys: BaseKeys,
        identifier: &str,
        box_holder: Option<&Box>,
        config: &'static Config,
    ) {
        // Extract name and type.
        let (widget_type, widget_name) = widget_pkg;

        // Extract data from the base keys.
        let text = base_keys.text;
        let command = base_keys.command;
        let update_rate = base_keys.update_rate;
        let tooltip = base_keys.tooltip;
        let tooltip_command = base_keys.tooltip_command;
        let alignment = base_keys.alignment;

        match widget_type {
            "label" => {
                let label = LabelWidget {
                    tooltip,
                    tooltip_command,
                    text,
                    command,
                    update_rate,
                    label: Label::new(None),
                    listen: key["listen"].as_bool().unwrap_or_default(),
                    revealer: Revealer::new(),
                    update_anim: RevealerTransitionType::from_str(
                        key["update_anim"].as_str().unwrap_or("crossfade"),
                    ),
                    anim_duration: key["anim_duration"].as_u32().unwrap_or(250),
                    config,
                };

                label.add(self, widget_name, alignment, box_holder)
            }
            "button" => {
                let button = ButtonWidget {
                    tooltip,
                    tooltip_command,
                    command,
                    button: Button::with_label(&text),
                    config,
                };

                button.add(self, widget_name, alignment, box_holder)
            }
            "spacing" => {
                let spacing = SpacingWidget {
                    spacing_start: key["spacing_start"].as_i32().unwrap_or_default(),
                    spacing_end: key["spacing_end"].as_i32().unwrap_or_default(),
                };

                spacing.add(self, widget_name, alignment, box_holder)
            }
            "box" => {
                let box_widget = BoxWidget {
                    width: key["width"].as_i32().unwrap_or_default(),
                    widgets: key["widgets"].to_owned(),
                    config,
                };

                box_widget.add(self, widget_name, alignment, box_holder)
            }
            "cava" => {
                let cava = CavaWidget {
                    label: Label::new(None),
                };

                if let Ok(mut has_cava_started) = HAS_CAVA_STARTED.lock() {
                    if !*has_cava_started {
                        cava::update_bars(config);
                        // Ensure it only calls update_bars once.
                        *has_cava_started = true
                    }
                }

                cava.add(self, widget_name, alignment, box_holder)
            }
            _ => {
                panic!("[ERROR] There is no widget type defined as '{identifier}'!")
            }
        }
    }

    /// Gets the config.
    pub fn get_config(&self) -> &'static Config {
        self.config
    }
}

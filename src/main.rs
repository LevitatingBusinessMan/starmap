#![feature(lazy_cell)]
use generator::generate_stars_with_seed;
use gtk::gdk;
use gtk::pango;
use gtk::prelude::*;
use gtk::FontLevel;
use relm4::prelude::*;
use relm4::abstractions::drawing::*;
mod draw;
mod generator;

use generator::Star;

#[derive(PartialEq, Clone, Debug)]
struct Colors {
    starnames: (f64,f64,f64),
    wall: (f64,f64,f64),
    starcolor: Option<(f64,f64,f64)>,
    jumplines: (f64,f64,f64),
}

static DARK_COLORS: Colors = Colors {
    starnames: (1.0,1.0,1.0),
    wall: (0.0,0.0,0.0),
    starcolor: None,
    jumplines: (0.5,0.5,0.5),
};

static LIGHT_COLORS: Colors = Colors {
    starnames: (0.0,0.0,0.0),
    wall: (1.0,1.0,1.0),
    starcolor: None,
    jumplines: (0.7,0.7,0.7),
};

struct App {
    stars: Vec<Star>,
    font_desc: pango::FontDescription,
    starcount: u32,
    draw_handler: DrawHandler,
    seed: u64,
    jumplines: bool,
    jumpdistance: f64,
    colors: Colors,
    scale: f64,
    display_class: bool,
}

#[derive(Debug)]
enum Msg {
    FontSelected(pango::FontDescription),
    Resize(i32,i32),
    StarCountChanged(u32),
    RegenerateSeed,
    EditedSeed(String),
    DarkSelected,
    LightSelected,
    JumpDistance(f64),
    JumpLines(bool),
    DisplayClass(bool),
    Print,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("CMDR Levitating's starmap generator"),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                gtk::Box {
                    set_width_request: 250,
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 20,
                    set_spacing: 5,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        set_label: "Star count"
                    },

                    gtk::SpinButton {
                        set_adjustment: &gtk::Adjustment::new(model.starcount as f64,0.0,generator::AMOUNT as f64,1.0,1.0,1.0),
                        connect_value_changed[sender] => move |b| { sender.input(Msg::StarCountChanged(b.value() as u32)) },
                    },

                    gtk::Label {
                        set_label: "Font",
                    },

                    gtk::FontDialogButton {
                        set_dialog: &gtk::FontDialog::new(),
                        set_level: FontLevel::Features,
                        set_use_size: false,
                        set_use_font: true,
                        set_font_features: None,
                        set_font_desc: &model.font_desc,
                        
                        connect_font_desc_notify[sender] => move |fdb| {
                            sender.input(Msg::FontSelected(fdb.font_desc().unwrap()));
                        },
                    },

                    gtk::Label {
                        set_label: "Color Preset",
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        append: light_preset = &gtk::ToggleButton {
                            set_label: "Light",
                            set_active: model.colors == LIGHT_COLORS,
                            connect_toggled => Msg::LightSelected,
                        },
                        gtk::ToggleButton {
                            set_label: "Dark",
                            set_active: model.colors == DARK_COLORS,
                            set_group: Some(&light_preset),
                            connect_toggled => Msg::DarkSelected,
                        },
                    },

                    gtk::Label {
                        set_label: "Jumpline distance (ly)",
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        set_spacing: 10,

                        gtk::SpinButton {
                            set_adjustment: &gtk::Adjustment::new(model.jumpdistance, 0.0, 100.0, 0.2, 0.1, 0.0),
                            set_digits: 2,
                            set_width_request: 150,
                            connect_value_changed[sender] => move |b| { sender.input(Msg::JumpDistance(b.value())) },
                        },
                        gtk::Switch {
                            #[watch]
                            set_active: model.jumplines,
                            connect_active_notify[sender] => move |s| { sender.input(Msg::JumpLines(s.is_active())) },
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        set_spacing: 10,

                        gtk::Label {
                            set_label: "Display star class",
                        },
                        gtk::Switch {
                            #[watch]
                            set_active: model.display_class,
                            connect_active_notify[sender] => move |s| { sender.input(Msg::DisplayClass(s.is_active())) },
                        },
                    },

                    gtk::Button {
                        set_label: "Print",
                        connect_clicked => Msg::Print,
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::End,
                        set_vexpand: true,

                        gtk::Label {
                            set_label: "Seed",
                        },

                        gtk::Entry {
                            gtk::prelude::EditableExt::set_alignment: 0.5,
                            // https://stackoverflow.com/a/68107365/8935250
                            #[watch]
                            set_buffer: &gtk::EntryBuffer::builder().text(format!("{:#x}", model.seed)).build(),
                            connect_activate[sender] => move |buf| { sender.input(Msg::EditedSeed(buf.text().to_string())) },
                        },

                        gtk::Button {
                            set_label: "Regenerate",
                            connect_clicked => Msg::RegenerateSeed,
                        },
                    },
                },

                gtk::Separator,

                #[local_ref]
                draw_area -> gtk::DrawingArea {
                    set_expand: true,
                    set_width_request: 400,
                    set_height_request: 400,
                    set_margin_all: 10,
                    set_cursor: gdk::Cursor::from_name("cell", None).as_ref(),
                    connect_resize[sender] => move |_,x,y| {sender.input(Msg::Resize(x,y))},
                },
            }
        }
    }

    // Initialize the component.
    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        let draw_handler = DrawHandler::new();

        let (stars, seed) = generator::generate_stars();

        let model = App {
            stars,
            font_desc: pango::FontDescription::from_string("Monospace Bold 12"),
            draw_handler,
            seed,
            starcount: 32,
            jumplines: true,
            jumpdistance: 10.0,
            colors: DARK_COLORS.clone(),
            scale: 50.0,
            display_class: false,
        };

        let draw_area = model.draw_handler.drawing_area();

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        println!("{:?}", msg);
        match msg {
            Msg::FontSelected(desc) => {
                println!("Font chosen: {:?}", desc.family().unwrap_or("unknown".into()));
                self.font_desc = desc;
            },
            Msg::StarCountChanged(count) => {
                self.starcount = count;
            },
            Msg::RegenerateSeed => {
                let (stars, seed) = generator::generate_stars();
                self.stars = stars;
                self.seed = seed;
            },
            Msg::EditedSeed(newseed) => {
                match u64::from_str_radix(newseed.strip_prefix("0x").unwrap_or(&newseed), 16) {
                    Ok(seed) => {
                        self.seed = seed;
                        self.stars = generate_stars_with_seed(seed);
                    },
                    Err(_) => {
                        let alert = gtk::AlertDialog::builder()
                        .detail("The seed you entered is invalid. Seeds should be presented as an hexidecimal.")
                        .message(format!("Invalid Seed: {}", newseed))
                        .build();
                        alert.show(relm4::main_application().active_window().as_ref());
                    },
                }
            },
            Msg::LightSelected => {
                self.colors = LIGHT_COLORS.clone()
            },
            Msg::DarkSelected => {
                self.colors = DARK_COLORS.clone()
            },
            Msg::JumpDistance(dist) => {
                self.jumpdistance = dist;
            },
            Msg::JumpLines(state) => {
                self.jumplines = state;
            },
            Msg::DisplayClass(state) => {
                self.display_class = state;
            },
            Msg::Print => {
                let printop = gtk::PrintOperation::new();
                let res = printop.run(gtk::PrintOperationAction::PrintDialog, relm4::main_application().active_window().as_ref()).unwrap();
                println!("{:?}", res);
            },
            _ => {}
        }
        draw::draw(self);
    }
}

fn main() {
    let app = RelmApp::new("levitating.StarMap");
    app.run::<App>(());
}

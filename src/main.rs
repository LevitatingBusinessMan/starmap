#![feature(lazy_cell)]
use gtk::gdk;
use gtk::pango;
use gtk::prelude::*;
use gtk::FontLevel;
use relm4::prelude::*;
use relm4::abstractions::drawing::*;

mod draw;
mod generator;

use generator::Star;

struct App {
    stars: Vec<Star>,
    font_desc: pango::FontDescription,
    starcount: u32,
    draw_handler: DrawHandler,
    seed: u64,
}

#[derive(Debug)]
enum Msg {
    FontSelected(pango::FontDescription),
    Resize(i32,i32),
    StarCountChanged(u32),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Starmap"),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                gtk::Box {
                    set_width_request: 250,
                    set_expand: true,
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Label {
                        set_label: "Stars"
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
                        set_level: FontLevel::Family,
                        set_use_size: false,
                        set_use_font: true,
                        set_font_features: None,
                        set_font_desc: &model.font_desc,
                        
                        connect_font_desc_notify[sender] => move |fdb| {
                            sender.input(Msg::FontSelected(fdb.font_desc().unwrap()));
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::End,

                        gtk::Entry {
                            set_buffer: &gtk::EntryBuffer::builder().text(format!("{}", model.seed)).build(),
                        },

                        gtk::Button {
                            set_label: "Regenerate",
                        },
                    }
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

        for s in &stars {
            println!("{} {} {} {:?}", s.name, s.class, s.planets, s.cords);
        }

        let model = App {
            stars,
            font_desc: pango::FontDescription::from_string("Sans"),
            draw_handler,
            seed,
            starcount: 32,
        };

        let draw_area = model.draw_handler.drawing_area();

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::FontSelected(desc) => {
                println!("Font chosen: {:?}", desc.family().unwrap_or("unknown".into()));
            },
            Msg::Resize(x,y) => {
                println!("resized {} {}", x, y);
            },
            Msg::StarCountChanged(count) => {
                self.starcount = count;
            }
        }
        draw::draw(self);
    }
}

fn main() {
    let app = RelmApp::new("levitating.StarMap");
    app.run::<App>(());
}

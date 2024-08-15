use gtk::gdk;
use gtk::pango;
use gtk::prelude::*;
use gtk::cairo;
use gtk::FontLevel;
use relm4::prelude::*;
use relm4::abstractions::drawing::*;

use std::f64::consts::PI;

struct App {
    font_dialog_button: gtk::FontDialogButton,
    draw_handler: DrawHandler,
}

#[derive(Debug)]
enum Msg {
    FontSelected,
    Resize(i32,i32),
}

impl App {
    fn draw(&mut self) {
        let cx: DrawContext = self.draw_handler.get_context();
        let width = self.draw_handler.width();
        let height = self.draw_handler.height();

        cx.set_source_rgb(0.0, 0.0, 0.0);
        cx.paint().unwrap();
        cx.set_source_rgb(1.0, 1.0, 1.0);
        cx.arc(width as f64 / 2.0, height as f64 / 2.0, height.min(width) as f64 / 2.0, 0.0, 2.0 * PI);
        cx.fill().unwrap();
        cx.set_font_size(40.0);
        let family = self.font_dialog_button.font_desc().unwrap().family().unwrap();
        cx.select_font_face(family.as_str(), cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        cx.move_to(width as f64 / 2.0, 50.0);
        cx.show_text("HELLO WORLD").unwrap();
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            //set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,

                gtk::Box {
                    set_width_request: 250,
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Label {
                        set_label: "Stars"
                    },

                    gtk::SpinButton {
                        set_adjustment: &gtk::Adjustment::new(0.0,0.0,f64::MAX,1.0,1.0,1.0),
                    },

                    gtk::Label {
                        set_label: "Font",
                    },

                    #[local_ref]
                    font_dialog_button -> gtk::FontDialogButton {
                        set_level: FontLevel::Family,
                        set_use_size: false,
                        set_use_font: true,
                        set_font_features: None,
                        set_font_desc: &pango::FontDescription::from_string("Sans"),
                        
                        connect_font_desc_notify => Msg::FontSelected,
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
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        let font_dialog_button = gtk::FontDialogButton::new(Some(gtk::FontDialog::new()));

        let draw_handler = DrawHandler::new();

        let mut model = App {
            font_dialog_button: font_dialog_button.clone(),
            draw_handler,
        };

        let draw_area = model.draw_handler.drawing_area();

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::FontSelected => {
                if let Some(desc) = self.font_dialog_button.font_desc() {
                    println!("Font chosen: {:?}", desc.family().unwrap_or("unknown".into()));
                }
            },
            Msg::Resize(x,y) => {
                println!("resized {} {}", x, y);
            }
        }
        self.draw();
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<App>(0);
}
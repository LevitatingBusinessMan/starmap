use gtk::pango::{ffi::PANGO_SCALE, FontDescription};
use rand_distr::num_traits::Pow;
use relm4::abstractions::DrawContext;

use std::f64::consts::PI;

use crate::{generator::Star, App};

pub fn draw(app: &mut crate::App) {
    let cx: DrawContext = app.draw_handler.get_context();
    let app: &App = &*app;

    let width = app.draw_handler.width();
    let height = app.draw_handler.height();

    cx.set_source_rgb(app.colors.wall.0, app.colors.wall.1, app.colors.wall.2);
    cx.paint().unwrap();
    
    if app.jumplines {
        draw_jumplines(&cx, width, height, app);
    }

    for star in &app.stars[0..app.starcount as usize] {
        draw_star(&cx, width, height, star, app);
    }

}

fn draw_jumplines(cx: &DrawContext, width: i32, height: i32, app: &App) {
    for star in &app.stars[0..app.starcount as usize] {
        cx.set_source_rgb(app.colors.jumplines.0, app.colors.jumplines.1, app.colors.jumplines.2);
        cx.set_line_width(3.0);
        for jstar in &app.stars[0..app.starcount as usize] {
            let distance: f64 = (((star.cords.0 - jstar.cords.0).abs().pow(2) + (star.cords.1- jstar.cords.1).abs().pow(2)) as f64).sqrt();
            if distance * app.scale < app.jumpdistance {
                cx.move_to(star.cords.0 * width as f64, star.cords.1 * height as f64);
                cx.line_to(jstar.cords.0 * width as f64, jstar.cords.1 * height as f64);
                cx.stroke().unwrap();
            }
        }
    }
}

fn draw_star(cx: &DrawContext, width: i32, height: i32, star: &Star, app: &App) {
    // star shape
    if let Some(starcolor) = app.colors.starcolor {
        cx.set_source_rgb(starcolor.0, starcolor.1, starcolor.2);

    } else {
        let color = starclass2color(star.class);
        cx.set_source_rgb(color.0, color.1, color.2);
    }
    cx.arc(star.cords.0 * width as f64, star.cords.1 * height as f64, 4.0, 0.0, 2.0 * PI);
    cx.fill().unwrap();

    // star name
    let layout = pangocairo::functions::create_layout(&cx);
    layout.set_font_description(Some(&app.font_desc));
    //layout.set_font_description(Some(&FontDescription::from_string("Sans Normal 15")));
    println!("{:?} from {:?}", layout.font_description().unwrap().family(), app.font_desc.family());
    if app.display_class {
        layout.set_text(&format!("{} [{}]", star.name, star.class));
    } else {
        layout.set_text(&star.name);
    }
    cx.set_source_rgb(app.colors.starnames.0, app.colors.starnames.1, app.colors.starnames.2);
    cx.move_to(star.cords.0 * width as f64 + 6.0, star.cords.1 * height as f64 - (layout.size().1 / PANGO_SCALE) as f64);
    pangocairo::functions::show_layout(&cx, &layout);
}

/*
def hex_to_rgb(hex_color)
    r = hex_color[0..1].to_i(16) / 255.0
    g = hex_color[2..3].to_i(16) / 255.0
    b = hex_color[4..5].to_i(16) / 255.0
    [r.round(2), g.round(2), b.round(2)]
end
*/

fn starclass2color(class: char) -> (f64,f64,f64) {
    match class {
        'O' => (0.61, 0.69, 1.0),
        'B' => (0.64, 0.75, 1.0),
        'A' => (0.84, 0.88, 1.0),
        'F' => (0.98, 0.96, 1.0),
        'G' => (1.0, 0.93, 0.89),
        'K' => (1.0, 0.85, 0.71),
        'M' => (1.0, 0.71, 0.42),
        _ => unreachable!(),
    }
}

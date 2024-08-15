use gtk::pango::ffi::PANGO_SCALE;
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
        cx.set_source_rgb(1.0, 0.5, 0.5);
    }
    cx.arc(star.cords.0 * width as f64, star.cords.1 * height as f64, 4.0, 0.0, 2.0 * PI);
    cx.fill().unwrap();

    // star name
    let layout = pangocairo::functions::create_layout(&cx);
    layout.set_font_description(Some(&app.font_desc));
    layout.set_text(&star.name);
    cx.set_source_rgb(app.colors.starnames.0, app.colors.starnames.1, app.colors.starnames.2);
    cx.move_to(star.cords.0 * width as f64 + 6.0, star.cords.1 * height as f64 - (layout.size().1 / PANGO_SCALE) as f64);
    pangocairo::functions::show_layout(&cx, &layout);
}
